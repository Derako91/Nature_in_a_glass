//! Lagerplätze (v0.8): der erste Schritt zur Sesshaftigkeit.
//!
//! Niemand bekommt einen Bauauftrag (Prinzip 1). Wer viele Nächte an
//! derselben verlässlichen Wasserstelle verbringt, legt dort eine
//! Feuerstelle mit Windschutz an. Wer sich dort aufhält und nichts Dringendes
//! zu tun hat, holt Holz aus dem Wald in der Nähe – das Holz fehlt danach im
//! Wald. Reicht das Holz und sind es mehr Menschen, als Platz haben, entsteht
//! eine Hütte. Überschüssige Nahrung wird im Lager abgelegt und verdirbt
//! langsam. Kommt niemand mehr, verfällt das Lager.
//!
//! Folgen für die Menschen: Nachts schützen Windschutz, Hütte und Feuer vor
//! Kälte; tagsüber kehrt man ohne Not zum Lager zurück und sammelt lieber in
//! seiner Nähe. Folgen für die Natur: Holzschlag lichtet den Wald rund um das
//! Lager, und in Trockenzeiten können Funken vom Lagerfeuer einen Brand
//! auslösen.

use serde::{Deserialize, Serialize};

use super::{NewsKind, People, PeopleNews, Person, cheb, drinkable, water_reliability};
use crate::climate::Weather;
use crate::fire::flammability;
use crate::math::clamp01;
use crate::rng::Rng;
use crate::world::World;

/// So viele Nächte in derselben Gegend, bevor jemand ein Lager anlegt.
pub const STAY_NIGHTS: u16 = 25;
/// Als „dieselbe Gegend“ gilt dieser Abstand zum Ort der ersten Nacht (Felder).
const STAY_RADIUS: i32 = 3;
/// Mindestabstand zwischen zwei Lagerplätzen (Felder).
pub const CAMP_SPACING: i32 = 5;
/// Mindestverlässlichkeit der Wasserstelle am Lager.
const RELIABLE_WATER: f32 = 0.6;
/// Holz für eine Hütte (Tageslasten).
pub const HUT_WOOD: f32 = 8.0;
/// So viele Menschen finden in einer Hütte Platz.
pub const HUT_PEOPLE: usize = 5;
/// Höchstens so viele Hütten je Lager.
pub const MAX_HUTS: u8 = 4;
/// Brennholz, das man als Vorrat hält (Tageslasten).
const FIRE_RESERVE: f32 = 4.0;
/// Eine Tageslast Holz entspricht diesem Anteil eines vollen Waldfeldes.
pub const WOOD_PER_LOAD: f32 = 0.004;
/// Holz sucht man bis zu so weit vom Lager (Felder) …
const WOOD_RANGE: i32 = 3;
/// … und wenn dort nichts mehr ist, bis so weit.
const WOOD_RANGE_FAR: i32 = 6;
/// Höchstens so viel Nahrung lagert man (Rationen).
pub const STORE_MAX: f32 = 150.0;
/// Ungenutzt gilt ein Lager nach so vielen Tagen als verlassen.
const ABANDON_DAYS: u64 = 2 * 365;
/// So schnell verfällt ein ungenutztes Lager (Anteil je Tag).
const DECAY_PER_DAY: f32 = 1.0 / (4.0 * 365.0);

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CampStage {
    /// Feuerstelle und Windschutz aus Ästen.
    Hearth,
    /// Mindestens eine Hütte.
    Huts,
}

/// Ein Lagerplatz.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Camp {
    pub id: u32,
    pub tile: u32,
    pub founded_hours: u64,
    /// Name der Person, die das Lager angelegt hat.
    pub founder: String,
    pub huts: u8,
    /// Holzvorrat (Tageslasten).
    pub wood: f32,
    /// Abgelegte Nahrung (Rationen).
    pub store: f32,
    /// 1 = gut erhalten, 0 = zerfallen.
    pub condition: f32,
    pub last_used_day: u64,
    pub abandoned: bool,
    /// Ob heute ein Feuer brennt (für die Darstellung).
    pub fire_today: bool,
}

impl Camp {
    pub fn stage(&self) -> CampStage {
        if self.huts > 0 {
            CampStage::Huts
        } else {
            CampStage::Hearth
        }
    }

    /// Schutz in der Nacht (0..1): Windschutz, Hütte, Feuer.
    pub fn shelter(&self) -> f32 {
        let base: f32 = if self.huts > 0 { 0.85 } else { 0.5 };
        let fire = if self.wood >= 0.2 { 0.3 } else { 0.0 };
        (base + fire).min(0.95) * (0.4 + 0.6 * self.condition)
    }

    /// Kurzbeschreibung für die Oberfläche.
    pub fn describe(&self) -> String {
        let what = match self.huts {
            0 => "Feuerstelle und Windschutz".to_string(),
            1 => "eine Hütte".to_string(),
            n => format!("{n} Hütten"),
        };
        if self.abandoned {
            format!("verlassen ({what})")
        } else {
            what
        }
    }
}

/// Was das Lagerleben in der Welt verändert (wird nach dem Tag angewendet).
#[derive(Debug, Default)]
pub struct Effects {
    /// Holz: Feld, Totholz, Wald (Anteile).
    cuts: Vec<(u32, f32, f32)>,
    /// Felder, die durch Funken Feuer fangen.
    ignitions: Vec<u32>,
    /// Felder, die für einen Garten gerodet werden.
    pub(super) clears: Vec<u32>,
}

impl Effects {
    pub fn apply(self, world: &mut World) {
        for (i, dead, forest) in self.cuts {
            let t = &mut world.tiles[i as usize];
            t.deadwood = (t.deadwood - dead).max(0.0);
            t.forest = (t.forest - forest).max(0.0);
            if t.forest < 0.005 {
                t.forest = 0.0;
                t.forest_age = 0.0;
            }
        }
        for i in self.clears {
            let t = &mut world.tiles[i as usize];
            t.forest = 0.0;
            t.forest_age = 0.0;
            t.deadwood = 0.0;
            t.shrub *= 0.2;
            t.grass *= 0.3;
        }
        for i in self.ignitions {
            let t = &mut world.tiles[i as usize];
            if t.fire <= 0.0 {
                t.fire = 0.8;
            }
        }
    }
}

pub(super) fn adult(p: &Person) -> bool {
    !p.is_child()
}

/// Nicht in Not: Man hat Zeit für Holz und Hütte.
pub(super) fn free(p: &Person) -> bool {
    adult(p) && p.hunger < 0.4 && p.thirst < 0.3 && p.injury < 0.2 && p.health > 0.6
}

pub(super) fn xy_text(world: &World, tile: u32) -> String {
    let (x, y) = world.xy(tile as usize);
    format!("({x}, {y})")
}

/// Ein Tag Lagerleben, nachdem alle unterwegs waren.
#[allow(clippy::too_many_arguments)]
pub(super) fn camp_day(
    people: &mut People,
    world: &World,
    weather: &Weather,
    now: u64,
    rng: &mut Rng,
    news: &mut Vec<PeopleNews>,
    effects: &mut Effects,
) {
    let day = now / 24;

    // --- Wo man die Nächte verbringt ---
    for p in &mut people.list {
        if p.flags.stay_days > 0 && cheb(world, p.tile, p.flags.stay_tile) <= STAY_RADIUS {
            p.flags.stay_days = p.flags.stay_days.saturating_add(1);
        } else {
            p.flags.stay_tile = p.tile;
            p.flags.stay_days = 1;
        }
    }

    // --- Zugehörigkeit: dazukommen, umziehen, fortbleiben ---
    for k in 0..people.list.len() {
        let (tile, camp, stay_tile, stay_days) = {
            let p = &people.list[k];
            (p.tile, p.camp, p.flags.stay_tile, p.flags.stay_days)
        };
        let own = people.camps.iter().find(|c| c.id == camp && !c.abandoned);
        // Wer lange anderswo übernachtet, kehrt nicht mehr zurück.
        if let Some(c) = own
            && stay_days >= 60
            && cheb(world, stay_tile, c.tile) > CAMP_SPACING
        {
            let at = xy_text(world, c.tile);
            let p = &mut people.list[k];
            p.camp = 0;
            if adult(p) {
                p.note(
                    now,
                    format!("Kehrt nicht mehr zum Lagerplatz bei {at} zurück."),
                );
            }
            continue;
        }
        let Some(here) = people
            .camps
            .iter()
            .find(|c| !c.abandoned && cheb(world, c.tile, tile) <= 1)
        else {
            continue;
        };
        if here.id == camp {
            continue;
        }
        let join = own.is_none() || stay_days >= 40;
        if !join {
            continue;
        }
        let (id, at, founder) = (here.id, xy_text(world, here.tile), here.founder.clone());
        let p = &mut people.list[k];
        let moved = own.is_some();
        p.camp = id;
        if adult(p) && p.name != founder {
            p.note(
                now,
                if moved {
                    format!("Zieht zum Lagerplatz bei {at}.")
                } else {
                    format!("Bleibt beim Lagerplatz bei {at}, den {founder} angelegt hat.")
                },
            );
        }
    }

    // --- Neue Lager: viele Nächte an verlässlichem Wasser ---
    for k in 0..people.list.len() {
        let p = &people.list[k];
        if !adult(p) || p.flags.stay_days < STAY_NIGHTS {
            continue;
        }
        if people.camps.iter().any(|c| c.id == p.camp && !c.abandoned) {
            continue;
        }
        let tile = p.tile;
        let i = tile as usize;
        if !super::standable(world, i)
            || !drinkable(world, i)
            || water_reliability(world, i) < RELIABLE_WATER
            || world.tiles[i].fire > 0.0
        {
            continue;
        }
        if people
            .camps
            .iter()
            .any(|c| !c.abandoned && cheb(world, c.tile, tile) < CAMP_SPACING)
        {
            continue;
        }
        if !rng.chance(0.1) {
            continue;
        }
        let name = p.name.clone();
        let at = xy_text(world, tile);
        // Ein verlassenes Lager in der Nähe wird wieder bezogen.
        let old = people
            .camps
            .iter()
            .position(|c| c.abandoned && cheb(world, c.tile, tile) < CAMP_SPACING);
        let (id, text, bio) = if let Some(o) = old {
            let c = &mut people.camps[o];
            c.abandoned = false;
            c.last_used_day = day;
            c.condition = c.condition.max(0.5);
            let at = xy_text(world, c.tile);
            (
                c.id,
                format!("{name} bezieht den verlassenen Lagerplatz bei {at} wieder."),
                format!("Bezieht den verlassenen Lagerplatz bei {at} wieder."),
            )
        } else {
            people.next_camp += 1;
            let id = people.next_camp;
            let first = people.camps.is_empty() && id == 1;
            people.camps.push(Camp {
                id,
                tile,
                founded_hours: now,
                founder: name.clone(),
                huts: 0,
                wood: 1.0,
                store: 0.0,
                condition: 1.0,
                last_used_day: day,
                abandoned: false,
                fire_today: false,
            });
            let text = if first {
                format!(
                    "Der erste Lagerplatz der Insel: {name} legt bei {at} eine Feuerstelle und einen Windschutz aus Ästen an."
                )
            } else {
                format!(
                    "{name} legt bei {at} einen Lagerplatz an: eine Feuerstelle und einen Windschutz aus Ästen."
                )
            };
            (
                id,
                text,
                format!("Legt bei {at} einen Lagerplatz an, nahe am Wasser."),
            )
        };
        let p = &mut people.list[k];
        p.camp = id;
        p.note(now, bio);
        let tile = people.camps.iter().find(|c| c.id == id).unwrap().tile;
        news.push(PeopleNews {
            kind: NewsKind::CampFounded,
            tile,
            text,
        });
    }

    // --- Leben am Lager ---
    for ci in 0..people.camps.len() {
        if people.camps[ci].abandoned {
            continue;
        }
        let (cid, ctile) = (people.camps[ci].id, people.camps[ci].tile);
        let present: Vec<usize> = (0..people.list.len())
            .filter(|&k| cheb(world, people.list[k].tile, ctile) <= 1)
            .collect();
        let members = people.list.iter().filter(|p| p.camp == cid).count();
        let temp = weather.temp_at(world.tiles[ctile as usize].elevation);
        let camp = &mut people.camps[ci];
        camp.fire_today = false;
        if present.is_empty() {
            continue;
        }
        camp.last_used_day = day;
        camp.condition = (camp.condition + 0.02).min(1.0);

        // Feuer: zum Kochen, bei Kälte auch zum Wärmen
        let burn = 0.1 + if temp < 6.0 { 0.3 } else { 0.0 };
        if camp.wood >= burn {
            camp.wood -= burn;
            camp.fire_today = true;
        } else {
            camp.wood = 0.0;
        }

        // Holz holen, wenn es fehlt
        let hut_needed = members > camp.huts as usize * HUT_PEOPLE && camp.huts < MAX_HUTS;
        let target = FIRE_RESERVE + if hut_needed { HUT_WOOD } else { 0.0 };
        for &k in &present {
            if people.camps[ci].wood >= target {
                break;
            }
            let p = &people.list[k];
            if !free(p) || p.activity == super::Activity::Run {
                continue;
            }
            let Some(src) = wood_source(world, ctile, effects) else {
                break;
            };
            let t = &world.tiles[src as usize];
            let already: (f32, f32) = effects
                .cuts
                .iter()
                .filter(|c| c.0 == src)
                .fold((0.0, 0.0), |a, c| (a.0 + c.1, a.1 + c.2));
            let dead = (t.deadwood - already.0).clamp(0.0, WOOD_PER_LOAD);
            let forest = (WOOD_PER_LOAD - dead).min((t.forest - already.1).max(0.0));
            effects.cuts.push((src, dead, forest));
            people.camps[ci].wood += (dead + forest) / WOOD_PER_LOAD;
            let p = &mut people.list[k];
            p.activity = super::Activity::Build;
            p.hunger = clamp01(p.hunger + 0.01);
            p.fatigue = clamp01(p.fatigue + 0.1);
        }

        // Hütte bauen
        let camp = &mut people.camps[ci];
        if hut_needed && camp.wood >= FIRE_RESERVE * 0.5 + HUT_WOOD {
            camp.wood -= HUT_WOOD;
            camp.huts += 1;
            people.huts_built += 1;
            let at = xy_text(world, ctile);
            let builders: Vec<String> = present
                .iter()
                .filter(|&&k| free(&people.list[k]))
                .map(|&k| people.list[k].name.clone())
                .collect();
            let who = match builders.len() {
                0 => String::new(),
                1 => format!(" {} hat sie gebaut.", builders[0]),
                _ => format!(
                    " {} und {} haben sie gebaut.",
                    builders[..builders.len() - 1].join(", "),
                    builders[builders.len() - 1]
                ),
            };
            let n = people.camps[ci].huts;
            let text = if people.huts_built == 1 {
                format!(
                    "Die erste Hütte der Insel steht am Lagerplatz bei {at}, aus Stangen, Ästen und Rinde.{who}"
                )
            } else if n == 1 {
                format!("Am Lagerplatz bei {at} steht nun eine Hütte.{who}")
            } else {
                format!("Am Lagerplatz bei {at} stehen nun {n} Hütten.{who}")
            };
            for &k in &present {
                if free(&people.list[k]) {
                    people.list[k].note(now, format!("Baut am Lagerplatz bei {at} eine Hütte."));
                }
            }
            news.push(PeopleNews {
                kind: NewsKind::HutBuilt,
                tile: ctile,
                text,
            });
        }

        // Vorräte: Überschuss ablegen, bei Hunger nehmen
        for &k in &present {
            let p = &mut people.list[k];
            if !adult(p) {
                continue;
            }
            let camp = &mut people.camps[ci];
            if p.hunger < 0.2 && p.pack > super::PACK_MAX - 0.3 {
                let put = (p.pack - 1.5).min(STORE_MAX - camp.store).max(0.0);
                p.pack -= put;
                camp.store += put;
            } else if p.hunger > 0.25 && p.pack < 1.0 && camp.store > 0.0 {
                let take = camp.store.min(1.5);
                camp.store -= take;
                p.pack += take;
            }
        }

        // Funken: in Trockenzeiten und bei Wind kann das Feuer überspringen.
        let camp = &people.camps[ci];
        if camp.fire_today {
            let (target, fl) = world
                .neighbors(ctile as usize)
                .map(|j| (j, flammability(&world.tiles[j])))
                .fold((usize::MAX, 0.0f32), |a, b| if b.1 > a.1 { b } else { a });
            let roll = rng.next_f32();
            if target != usize::MAX
                && fl > 0.2
                && roll < 0.0008 * fl * (0.3 + weather.wind_speed)
                && world.tiles[target].fire <= 0.0
            {
                effects.ignitions.push(target as u32);
                news.push(PeopleNews {
                    kind: NewsKind::CampfireSpread,
                    tile: target as u32,
                    text: format!(
                        "Funken vom Lagerfeuer bei {} setzen trockene Pflanzen in Brand.",
                        xy_text(world, ctile)
                    ),
                });
            }
        }
    }

    // --- Vorräte verderben ---
    for c in &mut people.camps {
        let temp = weather.temp_at(world.tiles[c.tile as usize].elevation);
        let spoil = if temp < 0.0 { 0.004 } else { 0.01 };
        c.store = (c.store * (1.0 - spoil)).max(0.0);
        if c.store < 0.05 {
            c.store = 0.0;
        }
    }

    decay_unused(people, world, day, news);
}

/// Ungenutzte Lager verfallen; verlassene verschwinden irgendwann.
pub(super) fn decay_unused(
    people: &mut People,
    world: &World,
    day: u64,
    news: &mut Vec<PeopleNews>,
) {
    for c in &mut people.camps {
        if c.last_used_day == day {
            continue;
        }
        c.condition = (c.condition - DECAY_PER_DAY).max(0.0);
        c.fire_today = false;
        if c.condition < 0.5 {
            c.wood = 0.0;
        }
        if !c.abandoned && day >= c.last_used_day + ABANDON_DAYS {
            c.abandoned = true;
            let what = match c.huts {
                0 => "",
                1 => " Die Hütte verfällt.",
                _ => " Die Hütten verfallen.",
            };
            news.push(PeopleNews {
                kind: NewsKind::CampAbandoned,
                tile: c.tile,
                text: format!(
                    "Seit zwei Jahren kommt niemand mehr zum Lagerplatz bei {}.{what}",
                    xy_text(world, c.tile)
                ),
            });
        }
    }
    let gone: Vec<u32> = people
        .camps
        .iter()
        .filter(|c| c.abandoned && c.condition <= 0.0)
        .map(|c| c.tile)
        .collect();
    for tile in gone {
        news.push(PeopleNews {
            kind: NewsKind::CampGone,
            tile,
            text: format!(
                "Vom alten Lagerplatz bei {} ist nichts mehr zu sehen.",
                xy_text(world, tile)
            ),
        });
    }
    people
        .camps
        .retain(|c| !(c.abandoned && c.condition <= 0.0));
    let ids: Vec<u32> = people.camps.iter().map(|c| c.id).collect();
    for p in &mut people.list {
        if p.camp != 0 && !ids.contains(&p.camp) {
            p.camp = 0;
        }
    }
}

/// Das Feld mit dem meisten Holz in der Nähe (Totholz zählt doppelt, es
/// ist leichter zu holen). Feste Reihenfolge bei Gleichstand.
fn wood_source(world: &World, camp: u32, effects: &Effects) -> Option<u32> {
    // Erst in der Nähe suchen; ist dort alles abgeholzt, weiter weg.
    wood_source_within(world, camp, effects, WOOD_RANGE)
        .or_else(|| wood_source_within(world, camp, effects, WOOD_RANGE_FAR))
}

fn wood_source_within(world: &World, camp: u32, effects: &Effects, range: i32) -> Option<u32> {
    let (cx, cy) = world.xy(camp as usize);
    let mut best: Option<(u32, f32)> = None;
    for dy in -range..=range {
        for dx in -range..=range {
            let (x, y) = (cx as i32 + dx, cy as i32 + dy);
            if x < 0 || y < 0 || x >= world.width as i32 || y >= world.height as i32 {
                continue;
            }
            let i = world.idx(x as u32, y as u32);
            let t = &world.tiles[i];
            if t.is_ocean() || t.fire > 0.0 {
                continue;
            }
            let (dead, forest) = effects
                .cuts
                .iter()
                .filter(|c| c.0 == i as u32)
                .fold((t.deadwood, t.forest), |a, c| (a.0 - c.1, a.1 - c.2));
            if dead + forest < WOOD_PER_LOAD + 0.02 {
                continue;
            }
            // Weiter weg ist mühsamer
            let d = dx.abs().max(dy.abs()) as f32;
            let score = (dead * 2.0 + forest) / (1.0 + 0.3 * d);
            if best.is_none_or(|b| score > b.1) {
                best = Some((i as u32, score));
            }
        }
    }
    best.map(|b| b.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn camp(huts: u8, wood: f32, condition: f32) -> Camp {
        Camp {
            id: 1,
            tile: 0,
            founded_hours: 0,
            founder: "Mira".into(),
            huts,
            wood,
            store: 0.0,
            condition,
            last_used_day: 0,
            abandoned: false,
            fire_today: false,
        }
    }

    #[test]
    fn huts_and_fire_shelter_more_than_a_windbreak() {
        let windbreak = camp(0, 0.0, 1.0).shelter();
        let fire = camp(0, 2.0, 1.0).shelter();
        let hut = camp(1, 2.0, 1.0).shelter();
        assert!(windbreak < fire && fire < hut, "{windbreak} {fire} {hut}");
        assert!(camp(1, 2.0, 0.2).shelter() < hut);
    }
}
