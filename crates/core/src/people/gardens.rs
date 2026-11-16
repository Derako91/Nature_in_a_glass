//! Gärten (v0.10): der Anfang des Anbaus.
//!
//! Niemand bekommt einen Auftrag (Prinzip 1). Wer schon länger an einem
//! Lager lebt, steckt im Frühjahr mit etwas Zufall zum ersten Mal Knollen und
//! Samen in den Boden – daraus wird ein Garten neben dem Lager. Er wächst über
//! den Sommer, wenn der Boden feucht und es warm genug ist und jemand jätet.
//! Im Herbst wird geerntet; die Ernte kommt in den Vorrat des Lagers und hilft
//! durch den Winter. Gärtnern ist Können, das mit Übung wächst und abgeschaut
//! wird. Wird ein Garten zwei Jahre nicht gepflegt, verwildert er.
//!
//! Folgen für die Natur: Für einen Garten wird ein Feld gerodet und bei jeder
//! Aussaat wieder freigemacht; verwildert er, wächst dort wieder, was die
//! Natur bringt.

use serde::{Deserialize, Serialize};

use super::camps::{Effects, free, xy_text};
use super::{NewsKind, People, PeopleNews, Sex, cheb, standable};
use crate::climate::Weather;
use crate::math::clamp01;
use crate::rng::Rng;
use crate::world::World;

/// Aussaat im Frühjahr, Wachstum im Sommer, Ernte im Herbst (Tag im Jahr).
pub const SOW_FROM: u32 = 80;
pub const SOW_TO: u32 = 140;
pub const RIPE_DAY: u32 = 240;
pub const HARVEST_TO: u32 = 320;
/// Ernte eines Gartens im besten Jahr bei vollem Können (Rationen).
pub const GARDEN_YIELD: f32 = 150.0;
/// So viel trägt ein Erwachsener an einem Erntetag ins Lager.
const HARVEST_PER_DAY: f32 = 8.0;
/// Ein Garten je so viele Menschen am Lager.
const PEOPLE_PER_GARDEN: usize = 4;
/// Höchstens so viele Gärten je Lager.
const MAX_GARDENS: usize = 6;
/// Ohne Pflege verwildert ein Garten nach so vielen Tagen.
const WILD_DAYS: u64 = 2 * 365;
/// Wahrscheinlichkeit je Frühlingstag, dass jemand das Säen entdeckt.
const DISCOVER_CHANCE: f32 = 0.002;

/// Ein Garten neben einem Lager.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Garden {
    pub id: u32,
    pub tile: u32,
    pub camp: u32,
    pub founded_hours: u64,
    /// In diesem Jahr gesät.
    pub sown: bool,
    /// Wachstum in diesem Jahr 0..1.
    pub growth: f32,
    /// Reife Ernte, die noch auf dem Feld steht (Rationen).
    pub ripe: f32,
    /// Letzter Tag, an dem jemand gesät, gejätet oder geerntet hat.
    pub tended_day: u64,
}

impl Garden {
    /// Bild-Zustand (gestaltung.md 17.9).
    pub fn stage(&self, doy: u32) -> &'static str {
        if self.ripe > 0.5 {
            "ripe"
        } else if self.sown && self.growth >= 0.5 {
            "growing"
        } else if self.sown {
            "sprout"
        } else if (SOW_FROM..=HARVEST_TO).contains(&doy) || self.growth > 0.0 {
            "stubble"
        } else {
            "bare"
        }
    }
}

/// Bester Platz für einen neuen Garten: nahe am Lager, feuchter, tiefer
/// Boden, kein Wasser, kein Feuer, noch kein Garten.
fn garden_spot(world: &World, people: &People, camp_tile: u32) -> Option<u32> {
    let (cx, cy) = world.xy(camp_tile as usize);
    let mut best: Option<(u32, f32)> = None;
    for dy in -2i32..=2 {
        for dx in -2i32..=2 {
            let (x, y) = (cx as i32 + dx, cy as i32 + dy);
            if x < 0 || y < 0 || x >= world.width as i32 || y >= world.height as i32 {
                continue;
            }
            let i = world.idx(x as u32, y as u32);
            if i as u32 == camp_tile || !standable(world, i) {
                continue;
            }
            let t = &world.tiles[i];
            if t.fire > 0.0
                || t.soil_depth < 0.15
                || people.gardens.iter().any(|g| g.tile == i as u32)
                || people.camps.iter().any(|c| c.tile == i as u32)
            {
                continue;
            }
            let near = 1.0 - 0.15 * dx.abs().max(dy.abs()) as f32;
            let score = (t.moisture_year.max(t.moisture_fraction()) + t.soil_depth.min(1.0)) * near;
            if best.is_none_or(|b| score > b.1) {
                best = Some((i as u32, score));
            }
        }
    }
    best.map(|b| b.0)
}

/// Ein Tag in den Gärten.
#[allow(clippy::too_many_arguments)]
pub(super) fn garden_day(
    people: &mut People,
    world: &World,
    weather: &Weather,
    doy: u32,
    now: u64,
    rng: &mut Rng,
    news: &mut Vec<PeopleNews>,
    effects: &mut Effects,
) {
    let day = now / 24;

    // --- Neue Gärten: im Frühjahr, an bewohnten Lagern ---
    if (SOW_FROM..=SOW_TO).contains(&doy) {
        for ci in 0..people.camps.len() {
            let c = &people.camps[ci];
            if c.abandoned || now < c.founded_hours + 365 * 24 {
                continue;
            }
            let (cid, ctile) = (c.id, c.tile);
            let members = people.list.iter().filter(|p| p.camp == cid).count();
            let have = people.gardens.iter().filter(|g| g.camp == cid).count();
            if have >= MAX_GARDENS || have * PEOPLE_PER_GARDEN >= members.max(1) {
                continue;
            }
            // Wer ist am Lager und könnte säen?
            let Some(k) = (0..people.list.len())
                .filter(|&k| {
                    let p = &people.list[k];
                    p.camp == cid && free(p) && cheb(world, p.tile, ctile) <= 1
                })
                .max_by(|&a, &b| {
                    let (pa, pb) = (&people.list[a], &people.list[b]);
                    pa.farming.total_cmp(&pb.farming).then(pb.id.cmp(&pa.id))
                })
            else {
                continue;
            };
            let skill = people.list[k].farming;
            // Ohne Vorbild ist es eine seltene Entdeckung, mit Können naheliegend.
            let chance = if skill >= 0.2 {
                0.03
            } else if skill > 0.0 {
                0.01
            } else {
                DISCOVER_CHANCE
            };
            if !rng.chance(chance) {
                continue;
            }
            let Some(tile) = garden_spot(world, people, ctile) else {
                continue;
            };
            people.next_garden += 1;
            let first = people.gardens.is_empty() && people.next_garden == 1;
            people.gardens.push(Garden {
                id: people.next_garden,
                tile,
                camp: cid,
                founded_hours: now,
                sown: true,
                growth: 0.0,
                ripe: 0.0,
                tended_day: day,
            });
            effects.clears.push(tile);
            let at = xy_text(world, tile);
            let p = &mut people.list[k];
            if p.farming < 0.1 {
                p.farming = 0.1;
            }
            p.activity = super::Activity::Gather;
            p.note(
                now,
                if first {
                    format!("Steckt bei {at} Knollen und Samen in den Boden – der erste Garten.")
                } else {
                    format!("Legt bei {at} einen Garten an.")
                },
            );
            let name = p.name.clone();
            news.push(PeopleNews {
                kind: NewsKind::GardenPlanted,
                tile,
                text: if first {
                    format!(
                        "Der erste Garten der Insel: {name} rodet bei {at} ein Stück Land und steckt Knollen und Samen in den Boden."
                    )
                } else {
                    format!("{name} legt bei {at} einen weiteren Garten an.")
                },
            });
        }
    }

    // --- Säen, jäten, ernten ---
    for gi in 0..people.gardens.len() {
        let (gtile, gcamp) = (people.gardens[gi].tile, people.gardens[gi].camp);
        let Some(camp_idx) = people
            .camps
            .iter()
            .position(|c| c.id == gcamp && !c.abandoned)
        else {
            continue;
        };
        let ctile = people.camps[camp_idx].tile;
        // Wer kann heute im Garten arbeiten? Freie Erwachsene am Lager.
        let workers: Vec<usize> = (0..people.list.len())
            .filter(|&k| {
                let p = &people.list[k];
                p.camp == gcamp && free(p) && cheb(world, p.tile, ctile) <= 1
            })
            .collect();
        let skill = workers
            .iter()
            .map(|&k| people.list[k].farming)
            .fold(0.0f32, f32::max);
        let g = &mut people.gardens[gi];
        let mut worked = false;
        if (SOW_FROM..=SOW_TO).contains(&doy) && !g.sown && !workers.is_empty() {
            // Säen: das Feld wieder freimachen
            g.sown = true;
            g.growth = 0.0;
            g.tended_day = day;
            effects.clears.push(gtile);
            worked = true;
        } else if g.sown && doy > SOW_TO && doy < RIPE_DAY {
            let t = &world.tiles[gtile as usize];
            let temp = weather.temp_at(t.elevation);
            let water = clamp01((t.moisture_fraction() - 0.1) / 0.3);
            let warm = if temp > 6.0 { 1.0 } else { 0.2 };
            // Jäten: alle paar Tage jemand
            if !workers.is_empty() && day >= g.tended_day + 4 {
                g.tended_day = day;
                worked = true;
            }
            let tended = if day < g.tended_day + 20 { 1.0 } else { 0.4 };
            // Eine Gärtnerin am Lager pflegt besser.
            let tended = tended
                * if workers
                    .iter()
                    .any(|&k| people.list[k].role() == Some(super::Role::Gardener))
                {
                    1.2
                } else {
                    1.0
                };
            g.growth = (g.growth + 0.012 * water * warm * tended).min(1.0);
        } else if g.sown && doy == RIPE_DAY {
            g.ripe = g.growth * GARDEN_YIELD * (0.4 + 0.6 * skill.max(0.1));
            g.sown = false;
        }
        if g.ripe > 0.0 && (RIPE_DAY..=HARVEST_TO).contains(&doy) && !workers.is_empty() {
            let camp = &mut people.camps[camp_idx];
            let take = (workers.len() as f32 * HARVEST_PER_DAY)
                .min(g.ripe)
                .min(camp.store_cap() - camp.store)
                .max(0.0);
            g.ripe -= take;
            camp.store += take;
            g.tended_day = day;
            worked = true;
            if take > 0.0 && g.ripe <= 0.5 {
                people.harvests += 1;
                let total = g.growth * GARDEN_YIELD * (0.4 + 0.6 * skill.max(0.1));
                if people.harvests == 1 {
                    news.push(PeopleNews {
                        kind: NewsKind::Harvest,
                        tile: gtile,
                        text: format!(
                            "Die erste Ernte der Insel: Am Lagerplatz bei {} kommen Knollen und Samen für {:.0} Tage in den Vorrat.",
                            xy_text(world, ctile),
                            total
                        ),
                    });
                }
            }
        }
        // Nicht geerntetes verdirbt auf dem Feld
        let g = &mut people.gardens[gi];
        if doy > HARVEST_TO {
            g.ripe = 0.0;
        }
        if worked {
            for &k in &workers {
                let p = &mut people.list[k];
                p.activity = super::Activity::Gather;
                let before = p.farming;
                p.farming = (p.farming + 0.01 * (1.0 - p.farming)).min(1.0);
                if before < 0.5 && p.farming >= 0.5 {
                    let who = if p.sex == Sex::Female {
                        "eine erfahrene Gärtnerin"
                    } else {
                        "ein erfahrener Gärtner"
                    };
                    p.note(now, format!("Ist {who} geworden."));
                }
            }
        }
    }

    go_wild(people, world, day, news);
}

/// Gärten ohne Pflege verwildern (auch, wenn niemand mehr auf der Insel lebt).
pub(super) fn go_wild(people: &mut People, world: &World, day: u64, news: &mut Vec<PeopleNews>) {
    let mut k = 0;
    while k < people.gardens.len() {
        let g = &people.gardens[k];
        let camp_alive = people.camps.iter().any(|c| c.id == g.camp && !c.abandoned);
        if day >= g.tended_day + WILD_DAYS || !camp_alive && day >= g.tended_day + 365 {
            let at = xy_text(world, g.tile);
            news.push(PeopleNews {
                kind: NewsKind::GardenLost,
                tile: g.tile,
                text: format!("Der Garten bei {at} verwildert: Niemand pflegt ihn mehr."),
            });
            people.gardens.remove(k);
        } else {
            k += 1;
        }
    }
}
