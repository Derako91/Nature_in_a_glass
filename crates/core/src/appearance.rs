//! Vom Simulationszustand zum Bild – ohne Grafik.
//!
//! Hier steht, welches Asset ein Feld bekommt, wie viele Bäume es zeigt und
//! wo sie stehen (siehe `docs/gestaltung.md`, Abschnitt 7). Die Regeln sind
//! rein rechnerisch, damit Beobachter und Tests dieselbe Wahrheit verwenden.
//! Positionen und Varianten sind fest aus Koordinate und Seed abgeleitet:
//! Dasselbe Feld sieht immer gleich aus.

use crate::math::clamp01;
use crate::rng::hash2;
use crate::sim::{RILL_FLOW, STREAM_FLOW};
use crate::world::{POND_MM, Terrain, Tile, World};

/// Grundboden der Raute.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ground {
    Soil,
    Sand,
    Rock,
    ShallowWater,
    DeepWater,
    Pond,
}

impl Ground {
    pub fn asset_id(self) -> &'static str {
        match self {
            Ground::Soil => "ground_soil",
            Ground::Sand => "ground_sand",
            Ground::Rock => "ground_rock",
            Ground::ShallowWater => "water_shallow",
            Ground::DeepWater => "water_deep",
            Ground::Pond => "water_pond",
        }
    }

    pub fn wall_id(self) -> &'static str {
        match self {
            Ground::Sand => "wall_sand",
            Ground::Rock => "wall_rock",
            _ => "wall_soil",
        }
    }

    pub fn is_water(self) -> bool {
        matches!(
            self,
            Ground::ShallowWater | Ground::DeepWater | Ground::Pond
        )
    }
}

/// Überlagerung auf dem Grundboden mit Deckkraft 0..1.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Overlay {
    pub id: &'static str,
    pub alpha: f32,
}

/// Wachstumsstufe eines Baums.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TreeStage {
    Young,
    Mid,
    Old,
}

impl TreeStage {
    pub fn from_age(age: f32) -> Self {
        if age < 15.0 {
            TreeStage::Young
        } else if age < 60.0 {
            TreeStage::Mid
        } else {
            TreeStage::Old
        }
    }

    pub fn asset_id(self) -> &'static str {
        match self {
            TreeStage::Young => "tree_young",
            TreeStage::Mid => "tree_mid",
            TreeStage::Old => "tree_old",
        }
    }
}

/// Ein Sprite auf einem Feld.
#[derive(Debug, Clone, PartialEq)]
pub struct Sprite {
    pub id: &'static str,
    pub state: &'static str,
    /// Fester Variantenwert; der Beobachter rechnet `variant_seed % varianten`.
    pub variant_seed: u32,
    /// Fußpunkt in der Raute: −1..1 je Achse, |u| + |v| ≤ 1 (u nach rechts, v nach unten).
    pub u: f32,
    pub v: f32,
}

/// Wie stark ein Wasserlauf auf dem Feld ist.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stream {
    None,
    Rill,
    Creek,
}

/// Anzahl Bäume nach Waldbedeckung (Tabelle 7.2).
pub fn tree_count(forest: f32) -> u32 {
    match forest {
        f if f < 0.05 => 0,
        f if f < 0.2 => 1,
        f if f < 0.4 => 2,
        f if f < 0.6 => 3,
        f if f < 0.7 => 4,
        f if f < 0.8 => 5,
        _ => 6,
    }
}

/// Anzahl Büsche nach Buschbedeckung (Tabelle 7.2).
pub fn shrub_count(shrub: f32) -> u32 {
    match shrub {
        s if s < 0.1 => 0,
        s if s < 0.3 => 1,
        s if s < 0.6 => 2,
        s if s < 0.8 => 3,
        _ => 4,
    }
}

/// Stehende Stämme nach Totholz.
pub fn snag_count(deadwood: f32) -> u32 {
    if deadwood <= 0.1 {
        0
    } else {
        ((deadwood * 4.0) as u32).clamp(1, 4)
    }
}

/// Liegende Stämme nach Totholz.
pub fn log_count(deadwood: f32) -> u32 {
    if deadwood <= 0.6 {
        0
    } else if deadwood <= 1.0 {
        1
    } else {
        2
    }
}

pub fn ground(world: &World, i: usize) -> Ground {
    let t = &world.tiles[i];
    match world.terrain(i) {
        Terrain::DeepOcean => Ground::DeepWater,
        Terrain::ShallowOcean => Ground::ShallowWater,
        _ if t.surface_water > POND_MM => Ground::Pond,
        Terrain::Beach => Ground::Sand,
        Terrain::Mountain => Ground::Rock,
        // Abgespült bis aufs Gestein: nackter Fels auch im Tiefland
        _ if t.soil_depth + t.soil_change < BARE_ROCK_SOIL_M => Ground::Rock,
        _ => Ground::Soil,
    }
}

/// Unter dieser Bodendicke (m) zeigt ein Feld nackten Fels.
pub const BARE_ROCK_SOIL_M: f32 = 0.05;
/// Ab dieser Ablagerungsrate (m pro Jahr) ist frisch angeschwemmter Boden voll sichtbar.
pub const SILT_FULL_RATE: f32 = 0.002;

/// Überlagerungen in Zeichenreihenfolge.
pub fn overlays(t: &Tile) -> Vec<Overlay> {
    if t.is_ocean() || t.surface_water > POND_MM {
        return Vec::new();
    }
    let wet = clamp01((t.moisture_fraction() - 0.15) / 0.45);
    let mut out = Vec::with_capacity(5);
    let mut push = |id, alpha: f32| {
        if alpha > 0.03 {
            out.push(Overlay {
                id,
                alpha: clamp01(alpha),
            });
        }
    };
    push("ground_grass_dry", t.grass * (1.0 - wet));
    push("ground_grass_wet", t.grass * wet);
    push("ground_shrub_floor", t.shrub * 0.8);
    push("ground_forest_floor", t.forest * 0.9);
    push("ground_burnt", t.burn_scar * (1.0 - t.grass));
    // Frisch angeschwemmter Boden (Bachufer, Schwemmfächer, verlandende Seeränder)
    push(
        "ground_silt",
        clamp01(t.erosion_rate / SILT_FULL_RATE) * (1.0 - t.forest),
    );
    out
}

pub fn stream(t: &Tile) -> Stream {
    if t.flow >= STREAM_FLOW {
        Stream::Creek
    } else if t.flow >= RILL_FLOW {
        Stream::Rill
    } else {
        Stream::None
    }
}

/// Feste Pseudozufallszahl für Feld, Seed und laufende Nummer.
fn fixed(x: u32, y: u32, seed: u64, salt: u64) -> u64 {
    hash2(
        x as i32,
        y as i32,
        seed ^ salt.wrapping_mul(0x9e37_79b9_7f4a_7c15),
    )
}

fn unit(h: u64) -> f32 {
    (h >> 40) as f32 / (1u64 << 24) as f32
}

/// Fußpunkte in der Raute. Punkt k hängt nur von k ab (nicht von der Anzahl),
/// damit beim Wachsen eines Waldes die vorhandenen Bäume stehen bleiben.
/// Mindestabstand und Randabstand nach `gestaltung.md` 7.3 (bei 64 × 32 px).
pub fn positions(x: u32, y: u32, seed: u64, kind: u64, count: u32) -> Vec<(f32, f32)> {
    // 6 px Abstand bei halber Rautenbreite 32 px → 0,19; 4 px Rand → Rautenmaß 0,85
    const MIN_DIST: f32 = 0.19;
    const INNER: f32 = 0.85;
    let mut out: Vec<(f32, f32)> = Vec::with_capacity(count as usize);
    for k in 0..count as u64 {
        let mut chosen = None;
        let mut fallback = (0.0, 0.0);
        for attempt in 0..12u64 {
            let h = fixed(x, y, seed, kind * 1000 + k * 16 + attempt);
            let a = unit(h) * 2.0 - 1.0;
            let b = unit(h.rotate_left(24)) * 2.0 - 1.0;
            // Quadrat → Raute (um 45° gedreht), dann auf Innenmaß schrumpfen
            let u = (a + b) * 0.5 * INNER;
            let v = (b - a) * 0.5 * INNER;
            if attempt == 0 {
                fallback = (u, v);
            }
            let free = out.iter().all(|&(pu, pv)| {
                // Abstand in Bildpunkten: v zählt doppelt (Raute ist halb so hoch)
                let du = u - pu;
                let dv = (v - pv) * 0.5;
                (du * du + dv * dv).sqrt() >= MIN_DIST
            });
            if free {
                chosen = Some((u, v));
                break;
            }
        }
        out.push(chosen.unwrap_or(fallback));
    }
    out
}

/// Alle Sprites eines Feldes, nach Fußpunkt von hinten nach vorne sortiert.
pub fn sprites(world: &World, i: usize) -> Vec<Sprite> {
    let t = &world.tiles[i];
    if t.is_ocean() || t.surface_water > POND_MM {
        return Vec::new();
    }
    let (x, y) = world.xy(i);
    let seed = world.seed;
    let mut out = Vec::new();
    let mut add = |kind: u64, n: u32, id: &'static str, state: &'static str| {
        for (k, (u, v)) in positions(x, y, seed, kind, n).into_iter().enumerate() {
            let variant_seed = (fixed(x, y, seed, 7000 + kind * 100 + k as u64) >> 33) as u32;
            out.push(Sprite {
                id,
                state,
                variant_seed,
                u,
                v,
            });
        }
    };

    let tree_state = if t.tree_supply < 0.3 {
        "stressed"
    } else {
        "normal"
    };
    let shrub_state = if t.moisture_fraction() < 0.08 {
        "stressed"
    } else {
        "normal"
    };
    let charred = if t.burn_scar > 0.3 {
        "charred"
    } else {
        "normal"
    };
    add(
        1,
        tree_count(t.forest),
        TreeStage::from_age(t.forest_age).asset_id(),
        tree_state,
    );
    add(2, shrub_count(t.shrub), "shrub", shrub_state);
    add(3, snag_count(t.deadwood), "snag_standing", charred);
    add(4, log_count(t.deadwood), "log_fallen", charred);
    if t.fire > 0.0 {
        let (id, n) = if t.forest > 0.2 || t.deadwood > 0.3 {
            ("fire_flame_large", 1 + (t.fire * 2.0) as u32)
        } else {
            ("fire_flame_small", 1 + (t.fire * 2.0) as u32)
        };
        add(5, n, id, "normal");
    }
    out.sort_by(|a, b| a.v.total_cmp(&b.v).then(a.u.total_cmp(&b.u)));
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positions_are_stable_when_forest_grows() {
        let three = positions(10, 20, 42, 1, 3);
        let four = positions(10, 20, 42, 1, 4);
        assert_eq!(&four[..3], &three[..]);
    }

    #[test]
    fn positions_stay_inside_diamond() {
        for x in 0..20 {
            for n in [1, 4, 6] {
                for (u, v) in positions(x, 7, 1, 1, n) {
                    assert!(u.abs() + v.abs() <= 0.86, "({u}, {v}) außerhalb");
                }
            }
        }
    }

    #[test]
    fn positions_keep_distance_mostly() {
        let mut close = 0;
        let mut total = 0;
        for x in 0..50 {
            let p = positions(x, 3, 9, 1, 6);
            for a in 0..p.len() {
                for b in 0..a {
                    total += 1;
                    let du = p[a].0 - p[b].0;
                    let dv = (p[a].1 - p[b].1) * 0.5;
                    if (du * du + dv * dv).sqrt() < 0.19 {
                        close += 1;
                    }
                }
            }
        }
        assert!(close * 20 < total, "{close} von {total} Paaren zu nah");
    }

    #[test]
    fn stripped_soil_shows_rock_and_fresh_silt_shows() {
        let mut w = World::generate(42, 64, 64);
        let i = (0..w.tiles.len())
            .find(|&i| matches!(w.terrain(i), Terrain::Lowland | Terrain::Hills))
            .unwrap();
        w.tiles[i].surface_water = 0.0;
        w.tiles[i].soil_depth = 0.01;
        w.tiles[i].soil_change = 0.0;
        assert_eq!(ground(&w, i), Ground::Rock);
        w.tiles[i].soil_depth = 0.5;
        assert_eq!(ground(&w, i), Ground::Soil);
        w.tiles[i].forest = 0.0;
        w.tiles[i].erosion_rate = SILT_FULL_RATE;
        assert!(overlays(&w.tiles[i]).iter().any(|o| o.id == "ground_silt"));
    }

    #[test]
    fn counts_follow_table() {
        assert_eq!(tree_count(0.0), 0);
        assert_eq!(tree_count(0.1), 1);
        assert_eq!(tree_count(0.5), 3);
        assert_eq!(tree_count(0.95), 6);
        assert_eq!(shrub_count(0.9), 4);
        assert_eq!(TreeStage::from_age(5.0), TreeStage::Young);
        assert_eq!(TreeStage::from_age(30.0), TreeStage::Mid);
        assert_eq!(TreeStage::from_age(80.0), TreeStage::Old);
    }
}
