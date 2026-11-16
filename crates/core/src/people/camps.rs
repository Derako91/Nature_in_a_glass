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

use super::{NewsKind, People, PeopleNews, Person, Sex, cheb, drinkable, water_reliability};
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
/// Vorrat am Lager: so viel ohne Hütte, dazu je Hütte (Rationen).
pub const STORE_BASE: f32 = 100.0;
pub const STORE_PER_HUT: f32 = 100.0;
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
    /// Tage in Folge ohne erreichbares Holz (seit Format 12).
    #[serde(default)]
    pub no_wood_days: u16,
    /// Nahrung, die Nachbarn aus dem Dorf gebracht haben (seit Format 13).
    #[serde(default)]
    pub help_received: f32,
    /// Tage in Folge, an denen am Lager kein Wasser zu finden ist (seit Format 14).
    #[serde(default)]
    pub dry_days: u16,
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

    /// So viel Vorrat fasst das Lager (Rationen).
    pub fn store_cap(&self) -> f32 {
        STORE_BASE + STORE_PER_HUT * self.huts as f32
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
            if p.settle != 0 {
                continue;
            }
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
        // Wer zu einem neuen Lager aufgebrochen ist, legt es am Ziel gleich an.
        let arrived = p.settle != 0 && cheb(world, p.tile, p.settle - 1) <= 1;
        if !adult(p) || (p.flags.stay_days < STAY_NIGHTS && !arrived) {
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
        if !arrived && !rng.chance(0.1) {
            continue;
        }
        let name = p.name.clone();
        let settle = p.settle;
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
                no_wood_days: 0,
                help_received: 0.0,
                dry_days: 0,
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
        p.settle = 0;
        p.note(now, bio);
        // Wer mit aufgebrochen ist, gehört gleich dazu.
        if settle != 0 {
            for q in &mut people.list {
                if q.settle == settle {
                    q.settle = 0;
                    q.camp = id;
                }
            }
        }
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
        // Wasser am Lager: auf dem Feld oder gleich daneben
        if drinkable(world, ctile as usize) {
            camp.dry_days = 0;
        } else {
            camp.dry_days = camp.dry_days.saturating_add(1);
        }

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
        // Mit Steinbeil schafft man an einem Tag zwei Lasten.
        let workers: Vec<usize> = present
            .iter()
            .flat_map(|&k| {
                let n = if people.list[k].knows(super::Tool::Axe) {
                    2
                } else {
                    1
                };
                std::iter::repeat_n(k, n)
            })
            .collect();
        for &k in &workers {
            if people.camps[ci].wood >= target {
                break;
            }
            let p = &people.list[k];
            if !free(p) || p.activity == super::Activity::Run {
                continue;
            }
            let Some(src) = wood_source(world, ctile, effects) else {
                people.camps[ci].no_wood_days = people.camps[ci].no_wood_days.saturating_add(1);
                break;
            };
            people.camps[ci].no_wood_days = 0;
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
            if p.hunger < 0.2 && p.pack > p.pack_max() - 0.3 {
                // Gesammeltes nur bis zur Hälfte: Der Rest bleibt für die Ernte.
                let put = (p.pack - 1.5)
                    .min(camp.store_cap() * 0.5 - camp.store)
                    .max(0.0);
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

    split_camps(people, world, now, rng, news);
    move_camps(people, world, now, rng, news);

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

/// Ein Lager wird zu eng (alle Hütten voll und mehr Menschen, als darin
/// Platz haben) oder es gibt seit Monaten kein Holz mehr: Dann bricht mit
/// etwas Zufall ein junges Paar mit seinen Kindern zu einer guten
/// Wasserstelle auf, die die Frau kennt – nicht zu nah und nicht zu weit.
/// Dort legt es ein neues Lager an. Findet es nicht hin, gibt es auf.
fn split_camps(
    people: &mut People,
    world: &World,
    now: u64,
    rng: &mut Rng,
    news: &mut Vec<PeopleNews>,
) {
    let day = now / 24;
    // Aufgeben nach langer Suche
    for p in &mut people.list {
        if p.settle != 0 && day > p.flags.settle_day + 150 {
            p.settle = 0;
            if adult(p) {
                p.note(
                    now,
                    "Findet keinen guten Platz für ein neues Lager.".to_string(),
                );
            }
        }
    }
    for ci in 0..people.camps.len() {
        let c = &people.camps[ci];
        if c.abandoned {
            continue;
        }
        let (cid, ctile) = (c.id, c.tile);
        let members = people.list.iter().filter(|p| p.camp == cid).count();
        let crowded = c.huts >= MAX_HUTS && members > MAX_HUTS as usize * HUT_PEOPLE;
        // Kleine Gruppen teilen sich wegen Holz nicht: Allein wären sie verloren.
        let no_wood = c.no_wood_days >= 90 && members >= 16;
        if !(crowded || no_wood) || !rng.chance(0.01) {
            continue;
        }
        // Ein junges Paar aus diesem Lager
        let Some(w) = (0..people.list.len()).find(|&k| {
            let p = &people.list[k];
            p.sex == Sex::Female
                && p.camp == cid
                && p.settle == 0
                && p.partner != 0
                && free(p)
                && p.age_years() < 35
                && people
                    .list
                    .iter()
                    .any(|q| q.id == p.partner && q.camp == cid && free(q))
        }) else {
            continue;
        };
        // Ziel: eine verlässliche Wasserstelle, die sie kennt
        let others: Vec<u32> = people
            .camps
            .iter()
            .filter(|c| !c.abandoned)
            .map(|c| c.tile)
            .collect();
        let target = people.list[w]
            .memory
            .iter()
            .filter(|m| m.kind == super::PlaceKind::Water && m.value >= RELIABLE_WATER)
            .filter(|m| {
                let d = cheb(world, m.tile, ctile);
                (CAMP_SPACING..=16).contains(&d)
                    && others
                        .iter()
                        .all(|&o| cheb(world, o, m.tile) >= CAMP_SPACING)
                    && super::standable(world, m.tile as usize)
                    && drinkable(world, m.tile as usize)
            })
            .max_by(|a, b| {
                let sa = a.value - 0.02 * cheb(world, a.tile, ctile) as f32;
                let sb = b.value - 0.02 * cheb(world, b.tile, ctile) as f32;
                sa.total_cmp(&sb).then(b.tile.cmp(&a.tile))
            })
            .map(|m| m.tile);
        let Some(target) = target else {
            continue;
        };
        let (wid, pid) = (people.list[w].id, people.list[w].partner);
        let (wname, pname) = (
            people.list[w].name.clone(),
            people
                .list
                .iter()
                .find(|q| q.id == pid)
                .map(|q| q.name.clone())
                .unwrap_or_default(),
        );
        let from = xy_text(world, ctile);
        let to = xy_text(world, target);
        let why = if crowded {
            "es ist zu eng geworden"
        } else {
            "in der Nähe gibt es kein Holz mehr"
        };
        for p in &mut people.list {
            // Das Paar und seine Kinder, die noch keine eigenen Wege gehen
            let family = p.id == wid
                || p.id == pid
                || (p.is_child() && (p.mother == wid || p.father == pid));
            if family && p.camp == cid {
                p.camp = 0;
                p.settle = target + 1;
                p.flags.settle_day = day;
                if adult(p) {
                    p.note(
                        now,
                        format!("Bricht vom Lager bei {from} auf, {why}, und zieht nach {to}."),
                    );
                }
            }
        }
        news.push(PeopleNews {
            kind: NewsKind::CampSplit,
            tile: ctile,
            text: format!(
                "{wname} und {pname} verlassen mit ihren Kindern das Lager bei {from} – {why} – und ziehen nach {to}, um dort ein eigenes Lager anzulegen."
            ),
        });
    }
}

/// Das ganze Lager zieht um: Die Wasserstelle ist seit zwei Monaten trocken,
/// oder einer kleinen Gruppe (die sich nicht teilt) fehlt seit einem halben
/// Jahr Holz. Ziel ist die beste verlässliche Wasserstelle, die jemand im
/// Lager kennt (5–20 Felder entfernt, nicht zu nah an anderen Lagern). Das
/// alte Lager wird aufgegeben und verfällt.
fn move_camps(
    people: &mut People,
    world: &World,
    now: u64,
    rng: &mut Rng,
    news: &mut Vec<PeopleNews>,
) {
    let day = now / 24;
    for ci in 0..people.camps.len() {
        let c = &people.camps[ci];
        if c.abandoned {
            continue;
        }
        let (cid, ctile) = (c.id, c.tile);
        let members: Vec<usize> = (0..people.list.len())
            .filter(|&k| people.list[k].camp == cid)
            .collect();
        if members.is_empty() {
            continue;
        }
        let dry = c.dry_days >= 60;
        let no_wood = c.no_wood_days >= 180 && members.len() < 16;
        if !(dry || no_wood) || !rng.chance(0.02) {
            continue;
        }
        let others: Vec<u32> = people
            .camps
            .iter()
            .filter(|o| !o.abandoned && o.id != cid)
            .map(|o| o.tile)
            .collect();
        // Was alle Erwachsenen des Lagers zusammen an Wasserstellen kennen
        let target = members
            .iter()
            .filter(|&&k| adult(&people.list[k]))
            .flat_map(|&k| people.list[k].memory.iter())
            .filter(|m| m.kind == super::PlaceKind::Water && m.value >= RELIABLE_WATER)
            .filter(|m| {
                let d = cheb(world, m.tile, ctile);
                (CAMP_SPACING..=20).contains(&d)
                    && others
                        .iter()
                        .all(|&o| cheb(world, o, m.tile) >= CAMP_SPACING)
                    && super::standable(world, m.tile as usize)
                    && drinkable(world, m.tile as usize)
            })
            .max_by(|a, b| {
                let sa = a.value - 0.02 * cheb(world, a.tile, ctile) as f32;
                let sb = b.value - 0.02 * cheb(world, b.tile, ctile) as f32;
                sa.total_cmp(&sb).then(b.tile.cmp(&a.tile))
            })
            .map(|m| m.tile);
        let Some(target) = target else {
            continue;
        };
        let from = xy_text(world, ctile);
        let to = xy_text(world, target);
        let why = if dry {
            "die Wasserstelle ist versiegt"
        } else {
            "in der Gegend gibt es kein Holz mehr"
        };
        for &k in &members {
            let p = &mut people.list[k];
            p.camp = 0;
            p.settle = target + 1;
            p.flags.settle_day = day;
            if adult(p) {
                p.note(
                    now,
                    format!("Verlässt mit allen anderen das Lager bei {from}, {why}, und zieht nach {to}."),
                );
            }
        }
        let c = &mut people.camps[ci];
        c.abandoned = true;
        c.last_used_day = day;
        news.push(PeopleNews {
            kind: NewsKind::CampMoved,
            tile: ctile,
            text: format!(
                "Das Lager bei {from} wird aufgegeben – {why}. {} Menschen ziehen nach {to}.",
                members.len()
            ),
        });
    }
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
            no_wood_days: 0,
            help_received: 0.0,
            dry_days: 0,
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
