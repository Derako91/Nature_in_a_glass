//! Seen erkennen, benennen und über die Zeit verfolgen.
//!
//! Einmal im Jahr (zum Jahreswechsel, im nassen Winter) werden zusammen-
//! hängende Felder mit stehendem Wasser gesucht. Erst wenn eine Wasserfläche
//! drei Jahreswechsel hintereinander da ist, gilt sie als See und bekommt
//! einen Namen. Ein See behält seinen Namen,
//! solange er sich mit seiner früheren Fläche überschneidet. Fehlt er drei
//! Jahreswechsel hintereinander, gilt er als verschwunden – verlandet,
//! ausgetrocknet oder mit einem anderen See zusammengewachsen.

use serde::{Deserialize, Serialize};

use crate::rng::Rng;
use crate::world::{POND_MM, World};

/// So viele Felder braucht ein stehendes Gewässer, um als See zu gelten.
pub const MIN_LAKE_TILES: usize = 3;
/// So viele Jahreswechsel darf ein See fehlen, bevor er als verschwunden gilt.
pub const MISSING_YEARS: u8 = 3;
/// So viele Jahreswechsel hintereinander muss eine Wasserfläche da sein, um
/// als See zu gelten. Tümpel aus einem einzelnen nassen Winter zählen nicht.
pub const CONFIRM_YEARS: u8 = 3;

const NAMES: [&str; 40] = [
    "Erlen", "Birken", "Schilf", "Kranich", "Reiher", "Nebel", "Stern", "Mond", "Silber", "Moos",
    "Weiden", "Binsen", "Krähen", "Otter", "Biber", "Libellen", "Kiesel", "Morgen", "Abend",
    "Wolken", "Spiegel", "Dunkel", "Kalten", "Quell", "Lerchen", "Unken", "Frosch", "Eschen",
    "Linden", "Ahorn", "Tannen", "Hasel", "Wind", "Stillen", "Grün", "Lichten", "Fels", "Tiefen",
    "Rohr", "Nixen",
];

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Lake {
    pub id: u32,
    pub name: String,
    /// Felder bei der letzten Erkennung.
    pub tiles: Vec<u32>,
    pub born_year: u64,
    pub max_tiles: u32,
    /// Größte gemessene Wassertiefe (m).
    pub max_depth_m: f32,
    /// Mittlere Geländehöhe der Seefläche bei der Entstehung (m).
    pub floor_at_birth: f32,
    /// Wie oft der See zuletzt hintereinander gefehlt hat.
    pub missing: u8,
    /// Wie oft hintereinander gesehen (bis zur Bestätigung).
    #[serde(default)]
    pub seen: u8,
    /// Erst bestätigte Seen haben einen Namen und stehen in der Chronik.
    #[serde(default)]
    pub confirmed: bool,
}

impl Lake {
    /// Mittelpunkt (Feld mit dem tiefsten Wasser bei der letzten Erkennung).
    pub fn anchor(&self) -> u32 {
        self.tiles.first().copied().unwrap_or(0)
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Lakes {
    pub lakes: Vec<Lake>,
    next_id: u32,
    used_names: Vec<u16>,
}

/// Was beim jährlichen Abgleich geschehen ist.
#[derive(Debug, Clone, PartialEq)]
pub enum LakeChange {
    Formed {
        lake: Lake,
    },
    /// Durch Ablagerung aufgefüllt.
    Silted {
        lake: Lake,
        years: u64,
    },
    /// Wasser ausgeblieben oder abgelaufen.
    DriedUp {
        lake: Lake,
        years: u64,
    },
    /// In einem anderen See aufgegangen.
    Merged {
        lake: Lake,
        into: String,
    },
}

/// Zusammenhängende Wasserflächen, je nach tiefstem Feld sortiert.
fn components(world: &World) -> Vec<Vec<u32>> {
    let n = world.tiles.len();
    let wet = |i: usize| !world.tiles[i].is_ocean() && world.tiles[i].surface_water > POND_MM;
    let mut seen = vec![false; n];
    let mut out = Vec::new();
    let mut stack = Vec::new();
    for start in 0..n {
        if seen[start] || !wet(start) {
            continue;
        }
        let mut comp = Vec::new();
        seen[start] = true;
        stack.push(start);
        while let Some(i) = stack.pop() {
            comp.push(i as u32);
            for nb in world.neighbors(i) {
                if !seen[nb] && wet(nb) {
                    seen[nb] = true;
                    stack.push(nb);
                }
            }
        }
        if comp.len() >= MIN_LAKE_TILES {
            // Tiefstes Wasser zuerst, Gleichstand nach Index
            comp.sort_by(|&a, &b| {
                let (wa, wb) = (
                    world.tiles[a as usize].surface_water,
                    world.tiles[b as usize].surface_water,
                );
                wb.total_cmp(&wa).then(a.cmp(&b))
            });
            out.push(comp);
        }
    }
    out
}

fn mean_floor(world: &World, tiles: &[u32]) -> f32 {
    let sum: f32 = tiles
        .iter()
        .map(|&i| world.tiles[i as usize].elevation)
        .sum();
    sum / tiles.len().max(1) as f32
}

impl Lakes {
    fn pick_name(&mut self, seed: u64, size: usize) -> String {
        let suffix = if size >= 12 {
            "see"
        } else if size >= 5 {
            "weiher"
        } else {
            "teich"
        };
        let mut rng = Rng::stream(seed ^ self.next_id as u64, "seen");
        let free: Vec<u16> = (0..NAMES.len() as u16)
            .filter(|k| !self.used_names.contains(k))
            .collect();
        let round = self.used_names.len() / NAMES.len();
        let k = if free.is_empty() {
            rng.below(NAMES.len() as u32) as u16
        } else {
            free[rng.below(free.len() as u32) as usize]
        };
        self.used_names.push(k);
        if free.is_empty() {
            format!("{}{} {}", NAMES[k as usize], suffix, round + 1)
        } else {
            format!("{}{}", NAMES[k as usize], suffix)
        }
    }

    /// Jährlicher Abgleich. Gibt Veränderungen für die Chronik zurück.
    pub fn update(&mut self, world: &World, year: u64) -> Vec<LakeChange> {
        let comps = components(world);
        let mut changes = Vec::new();
        let mut matched = vec![false; self.lakes.len()];
        let mut comp_taken = vec![false; comps.len()];

        // Jede Wasserfläche dem bekannten See mit der größten Überschneidung zuordnen
        for (c, comp) in comps.iter().enumerate() {
            let mut best: Option<(usize, usize)> = None;
            for (k, lake) in self.lakes.iter().enumerate() {
                if matched[k] {
                    continue;
                }
                let overlap = comp.iter().filter(|t| lake.tiles.contains(t)).count();
                if overlap > 0 && best.is_none_or(|(_, o)| overlap > o) {
                    best = Some((k, overlap));
                }
            }
            if let Some((k, _)) = best {
                matched[k] = true;
                comp_taken[c] = true;
                let lake = &mut self.lakes[k];
                lake.tiles = comp.clone();
                lake.missing = 0;
                lake.seen = lake.seen.saturating_add(1);
                lake.max_tiles = lake.max_tiles.max(comp.len() as u32);
                let depth = world.tiles[comp[0] as usize].surface_water / 1000.0;
                lake.max_depth_m = lake.max_depth_m.max(depth);
            }
        }

        // Nicht wiedergefundene Seen
        let mut k = 0;
        while k < self.lakes.len() {
            if matched[k] {
                k += 1;
                continue;
            }
            if !self.lakes[k].confirmed {
                // Unbestätigte Wasserfläche: still vergessen
                self.lakes.remove(k);
                matched.remove(k);
                continue;
            }
            self.lakes[k].missing += 1;
            if self.lakes[k].missing < MISSING_YEARS {
                k += 1;
                continue;
            }
            let lake = self.lakes.remove(k);
            matched.remove(k);
            let years = year.saturating_sub(lake.born_year);
            let into = self.lakes.iter().find(|o| {
                o.confirmed && o.missing == 0 && o.tiles.iter().any(|t| lake.tiles.contains(t))
            });
            let change = if let Some(o) = into {
                LakeChange::Merged {
                    into: o.name.clone(),
                    lake,
                }
            } else if mean_floor(world, &lake.tiles) - lake.floor_at_birth
                >= (0.5 * lake.max_depth_m).max(0.3)
            {
                LakeChange::Silted { lake, years }
            } else {
                LakeChange::DriedUp { lake, years }
            };
            changes.push(change);
        }

        // Neue Wasserflächen werden vorgemerkt
        for (c, comp) in comps.iter().enumerate() {
            if comp_taken[c] {
                continue;
            }
            self.lakes.push(Lake {
                id: 0,
                name: String::new(),
                tiles: comp.clone(),
                born_year: year,
                max_tiles: comp.len() as u32,
                max_depth_m: world.tiles[comp[0] as usize].surface_water / 1000.0,
                floor_at_birth: mean_floor(world, comp),
                missing: 0,
                seen: 1,
                confirmed: false,
            });
        }

        // Lange genug da: Der See bekommt seinen Namen
        for k in 0..self.lakes.len() {
            if self.lakes[k].confirmed || self.lakes[k].seen < CONFIRM_YEARS {
                continue;
            }
            let size = self.lakes[k].tiles.len();
            let name = self.pick_name(world.seed, size);
            let id = self.next_id;
            self.next_id += 1;
            let lake = &mut self.lakes[k];
            lake.id = id;
            lake.name = name;
            lake.confirmed = true;
            changes.push(LakeChange::Formed { lake: lake.clone() });
        }
        changes
    }

    /// Name des Sees, zu dem ein Feld gehört.
    pub fn name_at(&self, tile: u32) -> Option<&str> {
        self.lakes
            .iter()
            .find(|l| l.confirmed && l.missing == 0 && l.tiles.contains(&tile))
            .map(|l| l.name.as_str())
    }

    /// Bestehende Seen (bei der letzten Erkennung vorhanden).
    pub fn current(&self) -> impl Iterator<Item = &Lake> {
        self.lakes.iter().filter(|l| l.confirmed && l.missing == 0)
    }
}
