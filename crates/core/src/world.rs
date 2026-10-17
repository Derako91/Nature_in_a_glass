//! Insel, Kacheln und Gelände.

use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

use serde::{Deserialize, Serialize};

use crate::math::{clamp01, fbm};
use crate::rng::Rng;

/// Kantenlänge einer Kachel in Metern. 1 mm Wasser auf einer Kachel = 2,5 m³.
pub const TILE_SIZE_M: f32 = 50.0;

/// Ab dieser Wassertiefe (mm) gilt eine Kachel als stehendes Gewässer.
pub const POND_MM: f32 = 150.0;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Tile {
    /// Höhe über dem Meeresspiegel in Metern. ≤ 0 ist Meer.
    pub elevation: f32,
    /// Wasser, das der Boden halten kann (mm).
    pub soil_capacity: f32,
    /// Stehendes oder fließendes Oberflächenwasser (mm).
    pub surface_water: f32,
    /// Bodenfeuchte (mm, 0..soil_capacity).
    pub soil_moisture: f32,
    /// Geglätteter täglicher Durchfluss (mm/Tag) – zeigt Bäche und Flüsse.
    pub flow: f32,
    /// Grasbedeckung 0..1.
    pub grass: f32,
    /// Waldbedeckung 0..1.
    pub forest: f32,
    /// Aufeinanderfolgende Trockentage (für Dürreschäden am Wald).
    pub drought_days: u16,
}

impl Tile {
    pub fn is_ocean(&self) -> bool {
        self.elevation <= 0.0
    }

    pub fn moisture_fraction(&self) -> f32 {
        if self.soil_capacity <= 0.0 {
            0.0
        } else {
            self.soil_moisture / self.soil_capacity
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Terrain {
    DeepOcean,
    ShallowOcean,
    Beach,
    Lowland,
    Hills,
    Mountain,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Spring {
    pub x: u32,
    pub y: u32,
    /// Grundschüttung in mm auf der Kachel pro Tag bei vollem Grundwasser.
    pub base_rate: f32,
    /// Faktor durch Eingriffe des Beobachters (1.0 = natürlich).
    pub factor: f32,
    /// Heutige Schüttung (mm/Tag), nur zur Anzeige.
    pub current: f32,
    /// Ob die Quelle gerade als versiegt gilt.
    pub dry: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct World {
    pub width: u32,
    pub height: u32,
    pub seed: u64,
    pub max_elevation: f32,
    pub tiles: Vec<Tile>,
    pub springs: Vec<Spring>,

    // Abgeleitete Daten, werden nach dem Laden neu berechnet.
    /// Abflussziel jeder Kachel (Index), `None` für Meer.
    #[serde(skip)]
    pub downstream: Vec<Option<u32>>,
    /// Kacheln in Reihenfolge von oben nach unten (bergab).
    #[serde(skip)]
    pub drain_order: Vec<u32>,
    /// Wie viel Wasser (mm) eine Senke halten kann, bevor sie überläuft.
    #[serde(skip)]
    pub basin_capacity: Vec<f32>,
}

pub const NEIGHBORS: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

impl World {
    pub fn idx(&self, x: u32, y: u32) -> usize {
        (y * self.width + x) as usize
    }

    pub fn xy(&self, i: usize) -> (u32, u32) {
        (i as u32 % self.width, i as u32 / self.width)
    }

    pub fn tile(&self, x: u32, y: u32) -> &Tile {
        &self.tiles[self.idx(x, y)]
    }

    pub fn neighbors(&self, i: usize) -> impl Iterator<Item = usize> + '_ {
        let (x, y) = self.xy(i);
        NEIGHBORS.iter().filter_map(move |(dx, dy)| {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0 && ny >= 0 && (nx as u32) < self.width && (ny as u32) < self.height {
                Some((ny as u32 * self.width + nx as u32) as usize)
            } else {
                None
            }
        })
    }

    pub fn terrain(&self, i: usize) -> Terrain {
        let e = self.tiles[i].elevation;
        let rel = e / self.max_elevation;
        if e <= -15.0 {
            Terrain::DeepOcean
        } else if e <= 0.0 {
            Terrain::ShallowOcean
        } else if e < 3.0 {
            Terrain::Beach
        } else if rel < 0.35 {
            Terrain::Lowland
        } else if rel < 0.7 {
            Terrain::Hills
        } else {
            Terrain::Mountain
        }
    }

    pub fn land_count(&self) -> usize {
        self.tiles.iter().filter(|t| !t.is_ocean()).count()
    }

    /// Erzeugt eine neue Insel aus einem Seed.
    pub fn generate(seed: u64, width: u32, height: u32) -> Self {
        let mut tiles = Vec::with_capacity((width * height) as usize);
        let shape_seed = seed ^ 0x51a9_d1ce;
        let soil_seed = seed ^ 0x0050_11d0;
        let veg_seed = seed ^ 0x0f0e_57ee;
        let peak = 320.0;

        for y in 0..height {
            for x in 0..width {
                let nx = (x as f32 + 0.5) / width as f32 * 2.0 - 1.0;
                let ny = (y as f32 + 0.5) / height as f32 * 2.0 - 1.0;
                let d = (nx * nx + ny * ny).sqrt();
                let n = fbm(x as f32 / 14.0, y as f32 / 14.0, shape_seed, 5);
                // Grundform: Rauschen minus Abstand zur Mitte → Insel
                let shape = n * 1.25 - d * 1.05 + 0.05;
                // Steiler Abfall zum Kartenrand, damit überall Meer drumherum ist
                let edge = clamp01((d - 0.82) / 0.18);
                let h = shape - edge * 0.6;
                let elevation = if h > 0.0 {
                    // Höhere Lagen werden überproportional steiler (Bergkern)
                    let s = h * 1.6;
                    s * s * peak
                } else {
                    h * 120.0
                };
                tiles.push(Tile {
                    elevation,
                    soil_capacity: 0.0,
                    surface_water: 0.0,
                    soil_moisture: 0.0,
                    flow: 0.0,
                    grass: 0.0,
                    forest: 0.0,
                    drought_days: 0,
                });
            }
        }

        let max_elevation = tiles.iter().map(|t| t.elevation).fold(1.0f32, f32::max);
        let mut world = World {
            width,
            height,
            seed,
            max_elevation,
            tiles,
            springs: Vec::new(),
            downstream: Vec::new(),
            drain_order: Vec::new(),
            basin_capacity: Vec::new(),
        };

        // Boden: tief in flachen Niederungen, dünn an steilen Hängen und Gipfeln,
        // sandig an der Küste.
        for i in 0..world.tiles.len() {
            if world.tiles[i].is_ocean() {
                continue;
            }
            let e = world.tiles[i].elevation;
            let slope = world
                .neighbors(i)
                .map(|n| (world.tiles[n].elevation - e).abs())
                .fold(0.0f32, f32::max)
                / TILE_SIZE_M;
            let (x, y) = world.xy(i);
            let var = fbm(x as f32 / 6.0, y as f32 / 6.0, soil_seed, 3);
            let rel = e / max_elevation;
            let depth = clamp01(1.3 - slope * 3.0 - rel * 0.8 + (var - 0.5) * 0.6) * 1.5 + 0.1;
            let per_m = if e < 3.0 { 70.0 } else { 180.0 };
            let t = &mut world.tiles[i];
            t.soil_capacity = depth * per_m;
            t.soil_moisture = t.soil_capacity * 0.5;
            // Startvegetation: Grasland fast überall, Waldinseln in feuchten Lagen
            let v = fbm(x as f32 / 8.0, y as f32 / 8.0, veg_seed, 4);
            t.grass = if e < 3.0 { 0.1 } else { clamp01(0.3 + v * 0.4) };
            t.forest = if e >= 3.0 && rel < 0.75 && v > 0.55 {
                clamp01((v - 0.55) * 4.0)
            } else {
                0.0
            };
        }

        world.rebuild_drainage();
        world.place_springs(seed);
        world
    }

    fn place_springs(&mut self, seed: u64) {
        let mut rng = Rng::stream(seed, "quellen");
        let mut candidates: Vec<usize> = (0..self.tiles.len())
            .filter(|&i| {
                let rel = self.tiles[i].elevation / self.max_elevation;
                (0.3..0.65).contains(&rel)
            })
            .collect();
        let wanted = 2;
        while self.springs.len() < wanted && !candidates.is_empty() {
            let k = rng.below(candidates.len() as u32) as usize;
            let i = candidates.swap_remove(k);
            let (x, y) = self.xy(i);
            let far = self.springs.iter().all(|s| {
                let dx = s.x as i32 - x as i32;
                let dy = s.y as i32 - y as i32;
                dx * dx + dy * dy > 15 * 15
            });
            if far {
                self.springs.push(Spring {
                    x,
                    y,
                    base_rate: 250.0,
                    factor: 1.0,
                    current: 0.0,
                    dry: false,
                });
            }
        }
    }

    /// Berechnet Fließrichtungen und Senken (Priority-Flood).
    ///
    /// Vom Meer aus wird das Gelände „geflutet“. Jede Landkachel fließt zu der
    /// Kachel, von der aus sie erreicht wurde. Senken werden dabei bis zu ihrem
    /// Überlauf aufgefüllt – dort können sich Seen bilden.
    pub fn rebuild_drainage(&mut self) {
        let n = self.tiles.len();
        let mut filled = vec![f32::INFINITY; n];
        let mut downstream = vec![None; n];
        let mut visited = vec![false; n];
        let mut pop_order = Vec::with_capacity(n);
        let mut heap = BinaryHeap::new();
        let mut counter: u64 = 0;

        for i in 0..n {
            if self.tiles[i].is_ocean() {
                filled[i] = self.tiles[i].elevation;
                visited[i] = true;
                heap.push(Reverse(Entry(filled[i], counter, i)));
                counter += 1;
            }
        }
        while let Some(Reverse(Entry(level, _, i))) = heap.pop() {
            pop_order.push(i as u32);
            let ns: Vec<usize> = self.neighbors(i).collect();
            for nb in ns {
                if visited[nb] {
                    continue;
                }
                visited[nb] = true;
                let fl = self.tiles[nb].elevation.max(level + 0.001);
                filled[nb] = fl;
                downstream[nb] = Some(i as u32);
                heap.push(Reverse(Entry(fl, counter, nb)));
                counter += 1;
            }
        }

        self.basin_capacity = (0..n)
            .map(|i| {
                let depth_m = filled[i] - self.tiles[i].elevation;
                if depth_m > 0.02 {
                    depth_m * 1000.0
                } else {
                    0.0
                }
            })
            .collect();
        pop_order.reverse();
        self.drain_order = pop_order;
        self.downstream = downstream;
    }
}

#[derive(Debug, Clone, Copy)]
struct Entry(f32, u64, usize);

impl PartialEq for Entry {
    fn eq(&self, o: &Self) -> bool {
        self.cmp(o) == Ordering::Equal
    }
}
impl Eq for Entry {}
impl PartialOrd for Entry {
    fn partial_cmp(&self, o: &Self) -> Option<Ordering> {
        Some(self.cmp(o))
    }
}
impl Ord for Entry {
    fn cmp(&self, o: &Self) -> Ordering {
        self.0.total_cmp(&o.0).then(self.1.cmp(&o.1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn island_has_ocean_border_and_land() {
        let w = World::generate(1, 64, 64);
        for x in 0..64 {
            assert!(w.tile(x, 0).is_ocean());
            assert!(w.tile(x, 63).is_ocean());
        }
        let land = w.land_count() as f32 / w.tiles.len() as f32;
        assert!(land > 0.2 && land < 0.75, "Landanteil {land}");
    }

    #[test]
    fn every_land_tile_drains_to_sea() {
        let w = World::generate(3, 64, 64);
        for i in 0..w.tiles.len() {
            let mut cur = i;
            let mut steps = 0;
            while !w.tiles[cur].is_ocean() {
                cur = w.downstream[cur].expect("Landkachel ohne Abfluss") as usize;
                steps += 1;
                assert!(steps < 10_000);
            }
        }
    }

    #[test]
    fn generation_is_deterministic() {
        let a = World::generate(99, 64, 64);
        let b = World::generate(99, 64, 64);
        assert_eq!(a.tiles, b.tiles);
        assert_eq!(a.springs, b.springs);
    }
}
