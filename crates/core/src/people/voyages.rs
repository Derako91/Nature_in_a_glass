//! Überfahrten (v0.20, ADR 0012 Etappe 2): mit dem Einbaum zu anderen Inseln.
//!
//! Niemand schickt Menschen über das Meer (Prinzip 1). Wer einen Einbaum
//! bauen kann und am Ufer steht, sieht bei klarem Wetter manchmal Land am
//! Horizont – eine andere Insel. Das Wissen ist persönlich (Prinzip 6): Nur
//! wer sie gesehen hat (oder gerade daneben steht), kennt sie.
//!
//! Wer eine andere Insel kennt und Grund hat, fortzugehen – das Lager ist zu
//! eng, Holz fehlt, man hungert –, fährt bei ruhigem Wetter mit Partner und
//! Kindern hinüber. Ohne Not geschieht das selten, aus Neugier. Auf der neuen
//! Insel kennt man nichts: keine Wasserstelle, keinen Futterplatz. Die Fahrt
//! selbst ist gefährlich, je weiter, desto mehr.

use super::{Activity, Cause, NewsKind, People, PeopleNews, Person, PlaceKind, Sex, Tool};
use crate::climate::Weather;
use crate::rng::Rng;
use crate::world::World;

/// So weit sieht man vom Ufer aus Land (Felder).
pub const VIEW: i32 = 30;
/// Inseln ab dieser Größe sieht man und fährt man an (Felder).
pub const MIN_ISLAND: u32 = 40;
/// Bei mehr Wind als diesem fährt niemand los.
const CALM: f32 = 0.4;
/// Chance je Tag, Land zu entdecken (klare Sicht, am Ufer).
const SEE_CHANCE: f32 = 0.05;
/// Chance je Tag aufzubrechen: mit Grund …
const GO_PRESSED: f32 = 0.01;
/// … und aus reiner Neugier.
const GO_CURIOUS: f32 = 0.002;
/// So viele Menschen bleiben mindestens auf der Heimatinsel zurück.
const STAY_BEHIND: usize = 8;
/// Gefahr zu kentern je Feld offenen Meeres.
const RISK_PER_TILE: f32 = 0.001;

/// Inseln, die groß genug zum Ansteuern sind (feste Reihenfolge).
fn big_islands(world: &World) -> usize {
    world
        .islands
        .iter()
        .filter(|i| i.tiles >= MIN_ISLAND)
        .count()
}

fn island_id(world: &World, i: usize) -> Option<u32> {
    world
        .island_at(i)
        .filter(|is| is.tiles >= MIN_ISLAND)
        .map(|is| is.id)
}

fn bit(id: u32) -> u32 {
    if id < 32 { 1 << id } else { 0 }
}

/// Wie weit es von jedem Feld übers Meer bis zu jeder großen Insel ist und
/// wo man dort zuerst an Land käme. Einmal je Welt berechnet.
#[derive(Debug, Clone, Default)]
pub struct Sight {
    key: (usize, usize),
    ids: Vec<u32>,
    dist: Vec<Vec<u16>>,
    landing: Vec<Vec<u32>>,
}

impl Sight {
    fn prepare(&mut self, world: &World) {
        let key = (world.tiles.len(), world.islands.len());
        if self.key == key && !self.ids.is_empty() {
            return;
        }
        self.key = key;
        self.ids = world
            .islands
            .iter()
            .filter(|i| i.tiles >= MIN_ISLAND)
            .map(|i| i.id)
            .collect();
        let n = world.tiles.len();
        self.dist.clear();
        self.landing.clear();
        for &id in &self.ids {
            // Breitensuche übers Meer, ausgehend von allem Land der Insel
            let mut dist = vec![u16::MAX; n];
            let mut landing = vec![u32::MAX; n];
            let mut queue: std::collections::VecDeque<usize> = Default::default();
            for i in 0..n {
                if super::standable(world, i) && island_id(world, i) == Some(id) {
                    dist[i] = 0;
                    landing[i] = i as u32;
                    queue.push_back(i);
                }
            }
            while let Some(i) = queue.pop_front() {
                let d = dist[i];
                if d as i32 >= VIEW {
                    continue;
                }
                for j in world.neighbors(i) {
                    if dist[j] != u16::MAX {
                        continue;
                    }
                    dist[j] = d + 1;
                    landing[j] = landing[i];
                    // Weiter geht es nur übers Meer; fremdes Ufer ist Ziel.
                    if world.tiles[j].is_ocean() {
                        queue.push_back(j);
                    }
                }
            }
            self.dist.push(dist);
            self.landing.push(landing);
        }
    }

    /// Andere Inseln in Sichtweite: (Insel, Landeplatz, Abstand), nach Insel.
    fn in_view(&self, from: u32, home: u32) -> Vec<(u32, u32, i32)> {
        let f = from as usize;
        self.ids
            .iter()
            .enumerate()
            .filter(|&(_, &id)| id != home)
            .filter_map(|(k, &id)| {
                let d = *self.dist[k].get(f)?;
                (d != u16::MAX && (d as i32) <= VIEW).then(|| (id, self.landing[k][f], d as i32))
            })
            .collect()
    }
}

fn island_name(world: &World, id: u32) -> String {
    world
        .islands
        .get(id as usize)
        .map(|i| i.name.clone())
        .unwrap_or_else(|| "eine andere Insel".to_string())
}

fn names(list: &[String]) -> String {
    match list.len() {
        0 => String::new(),
        1 => list[0].clone(),
        n => format!("{} und {}", list[..n - 1].join(", "), list[n - 1]),
    }
}

/// Ein Tag: Land sehen, aufbrechen, übersetzen.
pub(super) fn voyages_day(
    people: &mut People,
    world: &World,
    weather: &Weather,
    now: u64,
    rng: &mut Rng,
    sight: &mut Sight,
    news: &mut Vec<PeopleNews>,
) {
    // Auf einer einzelnen Insel gibt es nichts zu sehen – und keinen Zufall
    // zu verbrauchen.
    if big_islands(world) < 2 {
        return;
    }
    // Wer schon Menschen auf einer Insel gesehen hat, gilt dort als angekommen.
    for p in &people.list {
        if let Some(id) = island_id(world, p.tile as usize) {
            people.reached |= bit(id);
        }
    }
    if weather.wind_speed >= CALM {
        return;
    }
    sight.prepare(world);

    // --- Land am Horizont ---
    for k in 0..people.list.len() {
        let p = &people.list[k];
        if p.is_child() || !p.knows(Tool::Boat) || !super::coastal(world, p.tile as usize) {
            continue;
        }
        let Some(home) = island_id(world, p.tile as usize) else {
            continue;
        };
        let tile = p.tile;
        let new: Vec<(u32, u32, i32)> = sight
            .in_view(tile, home)
            .into_iter()
            .filter(|v| people.list[k].seen & bit(v.0) == 0)
            .collect();
        let Some(&(id, _, d)) = new.first() else {
            continue;
        };
        if !rng.chance(SEE_CHANCE) {
            continue;
        }
        let name = island_name(world, id);
        let (x, y) = world.xy(tile as usize);
        for q in &mut people.list {
            if q.tile == tile && q.seen & bit(id) == 0 && !q.is_child() {
                q.seen |= bit(id);
                q.note(
                    now,
                    format!(
                        "Sieht vom Ufer bei ({x}, {y}) Land am Horizont, etwa {d} Felder übers Meer: {name}."
                    ),
                );
            }
        }
    }

    // --- Aufbruch ---
    let mut gone: Vec<u32> = Vec::new();
    for k in 0..people.list.len() {
        let p = &people.list[k];
        if gone.contains(&p.id)
            || p.is_child()
            || p.seen == 0
            || !p.knows(Tool::Boat)
            || p.settle != 0
            || p.partner == 0
            || p.injury >= 0.2
            || p.health <= 0.6
            || p.age_years() >= 40
            || !super::coastal(world, p.tile as usize)
        {
            continue;
        }
        let Some(home) = island_id(world, p.tile as usize) else {
            continue;
        };
        let partner = p.partner;
        let tile = p.tile;
        // Der Partner muss in der Nähe sein.
        let Some(pk) = people
            .list
            .iter()
            .position(|q| q.id == partner && super::cheb(world, q.tile, tile) <= 3)
        else {
            continue;
        };
        // Grund zum Fortgehen?
        let camp = people
            .camps
            .iter()
            .find(|c| c.id == p.camp && !c.abandoned);
        let pressed = match camp {
            Some(c) => {
                let members = people.list.iter().filter(|q| q.camp == c.id).count();
                members > (c.huts.max(1) as usize) * super::camps::HUT_PEOPLE
                    || c.no_wood_days >= 90
                    || (c.store < 5.0 && p.hunger > 0.4)
            }
            None => p.hunger > 0.4,
        };
        // Eine kleine Gruppe lässt man nicht zurück: Es müssen genug bleiben.
        let staying = people
            .list
            .iter()
            .filter(|q| island_id(world, q.tile as usize) == Some(home))
            .count();
        if staying < STAY_BEHIND + 2 {
            continue;
        }
        let chance = if pressed { GO_PRESSED } else { GO_CURIOUS };
        if !rng.chance(chance) {
            continue;
        }
        // Ziel: die gesehene Insel mit den wenigsten Menschen.
        let view = sight.in_view(tile, home);
        let target = view
            .iter()
            .filter(|v| people.list[k].seen & bit(v.0) != 0)
            .min_by_key(|v| {
                let crowd = people
                    .list
                    .iter()
                    .filter(|q| island_id(world, q.tile as usize) == Some(v.0))
                    .count();
                (crowd, v.2, v.0)
            })
            .copied();
        let Some((to_id, landing, dist)) = target else {
            continue;
        };
        let (wid, mid) = if people.list[k].sex == Sex::Female {
            (people.list[k].id, partner)
        } else {
            (partner, people.list[k].id)
        };
        // Mit: das Paar und Kinder, die noch keine eigenen Wege gehen und in
        // der Nähe sind.
        let crew: Vec<usize> = (0..people.list.len())
            .filter(|&j| {
                let q = &people.list[j];
                j == k
                    || j == pk
                    || (q.is_child()
                        && (q.mother == wid || q.father == mid)
                        && super::cheb(world, q.tile, tile) <= 2)
            })
            .collect();
        let crew_names: Vec<String> = crew
            .iter()
            .filter(|&&j| !people.list[j].is_child())
            .map(|&j| people.list[j].name.clone())
            .collect();
        let kids = crew.len() - crew_names.len();
        let from_name = island_name(world, home);
        let to_name = island_name(world, to_id);
        let first = people.reached & bit(to_id) == 0;
        let who = names(&crew_names);
        let with_kids = match kids {
            0 => String::new(),
            1 => " mit ihrem Kind".to_string(),
            n => format!(" mit ihren {n} Kindern"),
        };
        for &j in &crew {
            gone.push(people.list[j].id);
        }
        // Die Fahrt: Je weiter, desto gefährlicher.
        if rng.chance(RISK_PER_TILE * dist as f32) {
            for &j in &crew {
                let q = &mut people.list[j];
                q.health = 0.0;
                q.harm = Some(Cause::Drowned);
                q.note(
                    now,
                    format!("Kentert auf der Überfahrt von {from_name} nach {to_name}."),
                );
            }
            news.push(PeopleNews {
                kind: NewsKind::Crossing,
                tile,
                text: format!(
                    "{who}{with_kids} wollen mit dem Einbaum nach {to_name} übersetzen – das Boot kentert auf offenem Meer, niemand kommt an."
                ),
            });
            continue;
        }
        let (x, y) = world.xy(landing as usize);
        for &j in &crew {
            let q = &mut people.list[j];
            land(q, world, landing, to_id, now / 24);
            if !q.is_child() {
                q.note(
                    now,
                    format!(
                        "Fährt mit dem Einbaum von {from_name} hinüber nach {to_name} und landet bei ({x}, {y})."
                    ),
                );
            }
        }
        let text = if first {
            format!(
                "Zum ersten Mal betreten Menschen {to_name}: {who}{with_kids} setzen mit dem Einbaum von {from_name} über und landen bei ({x}, {y})."
            )
        } else {
            format!(
                "{who}{with_kids} fahren mit dem Einbaum von {from_name} nach {to_name} hinüber und landen bei ({x}, {y})."
            )
        };
        people.reached |= bit(to_id);
        people.crossings += 1;
        news.push(PeopleNews {
            kind: NewsKind::Crossing,
            tile: landing,
            text,
        });
    }
}

/// An Land gehen: Die neue Insel ist unbekannt.
fn land(q: &mut Person, world: &World, landing: u32, island: u32, day: u64) {
    q.tile = landing;
    q.prev_tile = landing;
    q.camp = 0;
    q.settle = 0;
    q.role = 0;
    q.activity = Activity::Idle;
    q.seen |= bit(island);
    q.memory
        .retain(|m| island_id(world, m.tile as usize) == Some(island));
    // Das Ufer kennt man: Dort gibt es Muscheln und Fisch.
    q.memory.push(super::Place {
        tile: landing,
        kind: PlaceKind::Shore,
        value: 1.0,
        seen_day: day,
    });
    q.flags.stay_tile = landing;
    q.flags.stay_days = 0;
    q.flags.knows_water = false;
}
