//! Vom Simulationszustand zum Bild – ohne Grafik.
//!
//! Hier steht, welches Asset ein Feld bekommt, wie viele Bäume es zeigt und
//! wo sie stehen (siehe `docs/gestaltung.md`, Abschnitt 7). Die Regeln sind
//! rein rechnerisch, damit Beobachter und Tests dieselbe Wahrheit verwenden.
//! Positionen und Varianten sind fest aus Koordinate und Seed abgeleitet:
//! Dasselbe Feld sieht immer gleich aus.

use crate::animals::{Activity, Animals, Species};
use crate::math::clamp01;
use crate::people::People;
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

// ---------- Tiere (gestaltung.md, Abschnitt 15) ----------

/// Ein sichtbares Tier: welches Sprite, wo, wie geschwächt.
#[derive(Debug, Clone, PartialEq)]
pub struct AnimalSprite {
    /// Asset-ID: `deer`, `deer_young`, `wolf`, `fox`, `rabbit`.
    pub id: &'static str,
    /// Zustand: `graze`, `walk`, `run`, `rest`.
    pub state: &'static str,
    /// Blickrichtung: `l` oder `r`.
    pub dir: &'static str,
    pub variant: u8,
    /// Feld heute und gestern (für gleitende Bewegung bei 1 Std/s).
    pub tile: u32,
    pub prev_tile: u32,
    /// Fußpunkt in der Raute (−1…1 wie bei [`positions`]).
    pub pos: (f32, f32),
    /// Verkleinerung: junge Wölfe und Füchse 0,75, sonst 1.
    pub scale: f32,
    /// Schwäche 0…1 (Renderer entsättigt um bis zu 25 %).
    pub weakness: f32,
}

fn activity_id(a: Activity) -> &'static str {
    match a {
        Activity::Graze => "graze",
        Activity::Walk => "walk",
        Activity::Run => "run",
        Activity::Rest => "rest",
    }
}

/// Kaninchen-Sprites je Feld nach Bestand (Tabelle 15.2).
pub fn rabbit_count(rabbits: f32) -> u32 {
    if rabbits < 1.0 {
        0
    } else if rabbits < 5.0 {
        1
    } else if rabbits < 15.0 {
        2
    } else {
        3
    }
}

/// Fußpunkt eines Tieres: fest aus Kennung und Feld, damit es auf einem Feld
/// nicht springt.
fn animal_pos(id: u32, tile: u32, seed: u64) -> (f32, f32) {
    let h = hash2(id as i32, tile as i32, seed ^ 0xa11a_a1a5);
    let a = unit(h) * 2.0 - 1.0;
    let b = unit(h.rotate_left(24)) * 2.0 - 1.0;
    ((a + b) * 0.35, (b - a) * 0.35)
}

/// Alle Hirsche, Wölfe und Füchse sowie die Kaninchen-Sprites.
pub fn animal_sprites(animals: &Animals, world: &World) -> Vec<AnimalSprite> {
    let mut out = Vec::new();
    for x in &animals.list {
        let young = x.is_young();
        let id = match x.species {
            Species::Deer if young => "deer_young",
            Species::Deer => "deer",
            Species::Wolf => "wolf",
            Species::Fox => "fox",
            Species::Rabbit => "rabbit",
        };
        let scale = if young && matches!(x.species, Species::Wolf | Species::Fox) {
            0.75
        } else {
            1.0
        };
        out.push(AnimalSprite {
            id,
            state: activity_id(x.activity),
            dir: if x.facing_right { "r" } else { "l" },
            variant: x.variant,
            tile: x.tile,
            prev_tile: x.prev_tile,
            pos: animal_pos(x.id, x.tile, world.seed),
            scale,
            weakness: clamp01((0.4 - x.energy) / 0.4),
        });
    }
    for (i, &r) in animals.rabbits.iter().enumerate() {
        let n = rabbit_count(r);
        if n == 0 {
            continue;
        }
        let (x, y) = world.xy(i);
        for (k, pos) in positions(x, y, world.seed, 7, n).into_iter().enumerate() {
            let h = fixed(x, y, world.seed, 7_000 + k as u64);
            out.push(AnimalSprite {
                id: "rabbit",
                state: if h.is_multiple_of(3) { "rest" } else { "graze" },
                dir: if h.is_multiple_of(2) { "l" } else { "r" },
                variant: (h % 2) as u8,
                tile: i as u32,
                prev_tile: i as u32,
                pos,
                scale: 1.0,
                weakness: 0.0,
            });
        }
    }
    out
}

/// Kadaver: Feld und Asset-ID (`carcass_large` oder `carcass_small`).
pub fn carcasses(animals: &Animals) -> Vec<(u32, &'static str)> {
    animals
        .carcasses
        .iter()
        .map(|c| {
            let id = match c.species {
                Species::Deer | Species::Wolf => "carcass_large",
                _ => "carcass_small",
            };
            (c.tile, id)
        })
        .collect()
}

/// Spuren-Deckkraft (0…1) einer Art auf einem Feld – für die Darstellung
/// ab 1 Monat/s statt einzelner Tiere (`tracks_<art>`).
pub fn track_density(animals: &Animals, tile: u32, species: Species) -> f32 {
    match species {
        Species::Rabbit => {
            clamp01(animals.rabbits.get(tile as usize).copied().unwrap_or(0.0) / 15.0)
        }
        _ => clamp01(animals.count_at(tile, species) as f32 / 4.0),
    }
}

// ---------- Menschen (gestaltung.md 17) ----------

/// Was der Renderer für einen Menschen zeichnet.
#[derive(Debug, Clone, PartialEq)]
pub struct PersonSprite {
    pub id: u32,
    pub name: String,
    /// Aussehen 0–4 (17.4).
    pub look: u8,
    /// Was die Person heute vor allem tut (`idle`, `walk`, `gather` …). Die
    /// Darstellung zur Tageszeit liefert [`person_state_at`].
    pub activity: &'static str,
    pub dir: &'static str,
    pub tile: u32,
    pub prev_tile: u32,
    /// Fußpunkt in der Raute (−1…1).
    pub pos: (f32, f32),
    pub weakness: f32,
}

/// Alle lebenden Menschen.
pub fn person_sprites(people: &People, world: &World) -> Vec<PersonSprite> {
    people
        .list
        .iter()
        .map(|p| PersonSprite {
            id: p.id,
            name: p.name.clone(),
            look: p.look,
            activity: p.activity.id(),
            dir: if p.facing_right { "r" } else { "l" },
            tile: p.tile,
            prev_tile: p.prev_tile,
            pos: person_pos(p.id, world.seed),
            weakness: p.weakness(),
        })
        .collect()
}

/// Fester Platz je Person, damit mehrere auf einem Feld nebeneinander stehen.
fn person_pos(id: u32, seed: u64) -> (f32, f32) {
    let h = hash2(id as i32, 77, seed ^ 0x9e05_1e5a);
    let a = unit(h) * 2.0 - 1.0;
    let b = unit(h.rotate_left(24)) * 2.0 - 1.0;
    ((a + b) * 0.3, (b - a) * 0.3)
}

/// Bild zur Tageszeit (nur bei 1 Std/s sichtbar, 17.5): nachts schlafen,
/// morgens unterwegs (wenn die Person das Feld gewechselt hat), sonst die
/// Tätigkeit des Tages. Gibt den Zustand und den Weganteil 0…1 zurück
/// (0 = noch am alten Feld, 1 = angekommen).
pub fn person_state_at(s: &PersonSprite, hour: f32) -> (&'static str, f32) {
    let moved = s.tile != s.prev_tile;
    let travel = matches!(s.activity, "walk" | "run" | "hurt");
    if s.activity == "run" && moved {
        // Flucht: sofort und schnell
        let t = clamp01((hour - 6.0) / 2.0);
        return (if t < 1.0 { "run" } else { "idle" }, t);
    }
    if !(6.0..21.0).contains(&hour) {
        return ("sleep", if hour >= 21.0 { 1.0 } else { 0.0 });
    }
    if moved {
        let t = clamp01((hour - 6.0) / 4.0);
        if t < 1.0 {
            return (if s.activity == "hurt" { "hurt" } else { "walk" }, t);
        }
    }
    let state = match s.activity {
        "walk" | "run" if travel => "idle",
        a => a,
    };
    // Abends wird gegessen, wer tagsüber gesammelt hat
    if state == "gather" && hour >= 18.0 {
        return ("eat", 1.0);
    }
    (state, 1.0)
}

/// Tote Menschen: Feld, Aussehen, Blickrichtung (`person_dead_<aussehen>_<richtung>`).
pub fn person_bodies(people: &People) -> Vec<(u32, u8, &'static str)> {
    people
        .bodies
        .iter()
        .map(|b| (b.tile, b.look, if b.facing_right { "r" } else { "l" }))
        .collect()
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
    fn animals_have_sprites_and_stable_positions() {
        use crate::{Intervention, Simulation};
        assert_eq!(rabbit_count(0.5), 0);
        assert_eq!(rabbit_count(3.0), 1);
        assert_eq!(rabbit_count(10.0), 2);
        assert_eq!(rabbit_count(40.0), 3);
        let mut sim = Simulation::new(42);
        sim.apply(Intervention::ReleaseAnimals {
            species: Species::Deer,
            at: None,
        });
        let a = animal_sprites(&sim.animals, &sim.world);
        assert_eq!(a.len(), 6);
        assert!(
            a.iter()
                .all(|s| s.id == "deer" && s.pos.0.abs() + s.pos.1.abs() <= 0.86)
        );
        assert_eq!(a, animal_sprites(&sim.animals, &sim.world));
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

    #[test]
    fn people_sleep_at_night_and_walk_in_the_morning() {
        let s = PersonSprite {
            id: 1,
            name: "Mira".into(),
            look: 0,
            activity: "gather",
            dir: "l",
            tile: 5,
            prev_tile: 4,
            pos: (0.0, 0.0),
            weakness: 0.0,
        };
        assert_eq!(person_state_at(&s, 2.0).0, "sleep");
        assert_eq!(person_state_at(&s, 23.0).0, "sleep");
        let (st, t) = person_state_at(&s, 7.0);
        assert_eq!(st, "walk");
        assert!(t > 0.0 && t < 1.0);
        assert_eq!(person_state_at(&s, 12.0), ("gather", 1.0));
        assert_eq!(person_state_at(&s, 19.0).0, "eat");
    }
}
