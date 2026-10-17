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
    /// Aufeinanderfolgende Tage mit Wassermangel der Bäume.
    pub drought_days: u16,

    // --- seit Spielstand-Format 2 (v0.2) ---
    /// Buschland 0..1.
    #[serde(default)]
    pub shrub: f32,
    /// Alter des Baumbestands in Jahren (0 ohne Wald).
    #[serde(default)]
    pub forest_age: f32,
    /// Stehendes und liegendes Totholz 0..1.5 (in Einheiten von „voller Wald“).
    #[serde(default)]
    pub deadwood: f32,
    /// Wasser im tieferen Boden, erreichbar nur für tiefe Wurzeln (mm).
    #[serde(default)]
    pub deep_moisture: f32,
    /// Wie gut die Bäume heute mit Wasser versorgt sind (0..1).
    #[serde(default = "one")]
    pub tree_supply: f32,
    /// Brandintensität heute (0 = kein Feuer).
    #[serde(default)]
    pub fire: f32,
    /// Sichtbare Brandspur, verblasst über Jahre (0..1).
    #[serde(default)]
    pub burn_scar: f32,

    // --- seit Spielstand-Format 3 (v0.3) ---
    /// Mittlerer Durchfluss über etwa ein Jahr (mm/Tag).
    #[serde(default)]
    pub flow_year: f32,
    /// Anteil der Tage im letzten Jahr, an denen ein Rinnsal floss (0..1).
    #[serde(default)]
    pub flow_permanence: f32,
    /// Mittlere Bodenfeuchte über etwa ein Jahr (0..1), z. B. für Biome.
    #[serde(default)]
    pub moisture_year: f32,

    // --- seit Spielstand-Format 5 (v0.3.0) ---
    /// Dicke der beweglichen Bodenschicht über dem Gestein (m).
    #[serde(default)]
    pub soil_depth: f32,
    /// Noch nicht ins Gelände übernommene Änderung der Bodenschicht (m,
    /// positiv = Ablagerung). Kleine tägliche Beträge werden hier gesammelt,
    /// bis sie groß genug sind, um die Höhe genau zu verändern.
    #[serde(default)]
    pub soil_change: f32,
    /// Noch nicht übernommener Abtrag des Gesteins (m, ≤ 0).
    #[serde(default)]
    pub rock_change: f32,
    /// Geglättete Höhenänderung durch Abtrag und Ablagerung (m pro Jahr).
    #[serde(default)]
    pub erosion_rate: f32,
}

/// Wasser, das ein Meter Boden halten kann (mm): Sand an der Küste hält wenig.
pub fn water_per_soil_meter(elevation: f32) -> f32 {
    if elevation < 3.0 { 70.0 } else { 180.0 }
}

/// Gedächtnis der Jahresmittel: Gewicht des heutigen Tages (≈ 1/365).
pub const YEAR_WEIGHT: f32 = 1.0 / 365.0;

fn one() -> f32 {
    1.0
}

/// Tiefer Boden hält etwa doppelt so viel Wasser wie der Wurzelraum darüber –
/// wo der Boden dünn ist, gibt es also auch kaum Tiefenwasser.
pub const DEEP_FACTOR: f32 = 2.0;

impl Tile {
    pub fn deep_capacity(&self) -> f32 {
        self.soil_capacity * DEEP_FACTOR
    }

    pub fn deep_fraction(&self) -> f32 {
        let c = self.deep_capacity();
        if c <= 0.0 {
            0.0
        } else {
            self.deep_moisture / c
        }
    }

    /// Wie weit die Wurzeln in die Tiefe reichen (0,2 junger Wald … 1,0 Altwald).
    pub fn root_reach(&self) -> f32 {
        0.2 + 0.8 * clamp01(self.forest_age / 60.0)
    }

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
    /// Hauptfließrichtung jeder Kachel (Index des Nachbarn, der am meisten
    /// Wasser bekommt). Wird jeden Tag aus dem Wasserspiegel neu bestimmt;
    /// `None` für Meer und für Senken, in denen sich Wasser staut.
    #[serde(skip)]
    pub downstream: Vec<Option<u32>>,
    /// Nachbarn jedes Feldes (bis zu 8), einmal berechnet. Reihenfolge wie
    /// [`NEIGHBORS`], damit Zufallszüge reproduzierbar bleiben.
    #[serde(skip)]
    neighbor_table: Vec<([u32; 8], u8)>,
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
        let (list, n) = &self.neighbor_table[i];
        list[..*n as usize].iter().map(|&j| j as usize)
    }

    fn build_neighbor_table(&mut self) {
        let (w, h) = (self.width, self.height);
        self.neighbor_table = (0..self.tiles.len())
            .map(|i| {
                let (x, y) = (i as u32 % w, i as u32 / w);
                let mut list = [0u32; 8];
                let mut n = 0u8;
                for (dx, dy) in NEIGHBORS {
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;
                    if nx >= 0 && ny >= 0 && (nx as u32) < w && (ny as u32) < h {
                        list[n as usize] = ny as u32 * w + nx as u32;
                        n += 1;
                    }
                }
                (list, n)
            })
            .collect();
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

    /// Setzt winzige Restwerte auf 0. Werte, die Tag für Tag schrumpfen
    /// (z. B. Gras auf nacktem Fels), bleiben sonst im Bereich „subnormaler“
    /// Gleitkommazahlen hängen – das Ergebnis ändert sich dadurch nicht
    /// sichtbar, aber jede Rechnung damit ist auf manchen Prozessoren
    /// vielfach langsamer.
    pub fn flush_tiny(&mut self) {
        const TINY: f32 = 1e-30;
        let f = |v: &mut f32| {
            if v.abs() < TINY {
                *v = 0.0;
            }
        };
        for t in self.tiles.iter_mut() {
            f(&mut t.surface_water);
            f(&mut t.soil_moisture);
            f(&mut t.flow);
            f(&mut t.grass);
            f(&mut t.forest);
            f(&mut t.shrub);
            f(&mut t.forest_age);
            f(&mut t.deadwood);
            f(&mut t.deep_moisture);
            f(&mut t.tree_supply);
            f(&mut t.fire);
            f(&mut t.burn_scar);
            f(&mut t.flow_year);
            f(&mut t.flow_permanence);
            f(&mut t.moisture_year);
            f(&mut t.soil_change);
            f(&mut t.rock_change);
            f(&mut t.erosion_rate);
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
                    shrub: 0.0,
                    forest_age: 0.0,
                    deadwood: 0.0,
                    deep_moisture: 0.0,
                    tree_supply: 1.0,
                    fire: 0.0,
                    burn_scar: 0.0,
                    flow_year: 0.0,
                    flow_permanence: 0.0,
                    moisture_year: 0.5,
                    soil_depth: 0.0,
                    soil_change: 0.0,
                    rock_change: 0.0,
                    erosion_rate: 0.0,
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
            neighbor_table: Vec::new(),
        };
        world.build_neighbor_table();

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
            let t = &mut world.tiles[i];
            t.soil_depth = depth;
            t.soil_capacity = depth * water_per_soil_meter(e);
            t.soil_moisture = t.soil_capacity * 0.5;
            // Startvegetation: Grasland fast überall, Waldinseln in feuchten Lagen
            let v = fbm(x as f32 / 8.0, y as f32 / 8.0, veg_seed, 4);
            t.grass = if e < 3.0 { 0.1 } else { clamp01(0.3 + v * 0.4) };
            t.forest = if e >= 3.0 && rel < 0.75 && v > 0.55 {
                clamp01((v - 0.55) * 4.0)
            } else {
                0.0
            };
            // Ältere Bestände im Kern der Waldinseln, jüngere am Rand
            t.forest_age = if t.forest > 0.0 {
                10.0 + t.forest * 50.0
            } else {
                0.0
            };
            // Buschland an den Waldrändern
            t.shrub = if e >= 3.0 && (0.45..0.62).contains(&v) {
                0.35
            } else {
                0.0
            };
            t.deep_moisture = t.deep_capacity() * 0.6;
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

    /// Anfängliche Fließrichtungen aus dem Gelände (Priority-Flood).
    ///
    /// Vom Meer aus wird das Gelände „geflutet“; jede Landkachel zeigt zu der
    /// Kachel, von der aus sie erreicht wurde – so führt von überall ein Weg
    /// zum Meer, auch aus Senken. Ab dem ersten Tag bestimmt der Wasserkreislauf
    /// die Richtungen neu aus dem Wasserspiegel (`hydro.rs`).
    pub fn rebuild_drainage(&mut self) {
        if self.neighbor_table.len() != self.tiles.len() {
            self.build_neighbor_table();
        }
        let n = self.tiles.len();
        let mut downstream = vec![None; n];
        let mut visited = vec![false; n];
        let mut heap = BinaryHeap::new();
        let mut counter: u64 = 0;

        for (i, t) in self.tiles.iter().enumerate() {
            if t.is_ocean() {
                visited[i] = true;
                heap.push(Reverse(Entry(t.elevation, counter, i)));
                counter += 1;
            }
        }
        while let Some(Reverse(Entry(level, _, i))) = heap.pop() {
            let ns: Vec<usize> = self.neighbors(i).collect();
            for nb in ns {
                if visited[nb] {
                    continue;
                }
                visited[nb] = true;
                let fl = self.tiles[nb].elevation.max(level + 0.001);
                downstream[nb] = Some(i as u32);
                heap.push(Reverse(Entry(fl, counter, nb)));
                counter += 1;
            }
        }
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
