//! Dörfer (v0.12): Lager, die nah beieinander liegen.
//!
//! Ein Dorf wird nicht gegründet, es ergibt sich (Prinzip 1): Liegen
//! mindestens drei bewohnte Lager mit zusammen zehn Menschen so nah beieinander, dass man sich
//! gegenseitig besucht (Kette mit höchstens 8 Feldern Abstand), gelten sie als
//! Dorf. Es bekommt einen Namen aus seiner Umgebung (Küste, See, Wald,
//! Wiese, Berg). Im Dorf hilft man sich: Hungert ein Lager, bringen Lager mit
//! vollem Vorrat etwas vorbei. Und junge Leute finden auch im Nachbarlager
//! einen Partner.
//!
//! Zerfällt ein Dorf (weniger als drei Lager), verschwindet es; der Name
//! bleibt in der Chronik.

use serde::{Deserialize, Serialize};

use super::camps::xy_text;
use super::{NewsKind, People, PeopleNews, cheb};
use crate::rng::Rng;
use crate::world::{POND_MM, World};

/// So nah müssen Lager beieinander liegen, um ein Dorf zu bilden (Felder).
pub const VILLAGE_LINK: i32 = 8;
/// So viele bewohnte Lager braucht ein Dorf …
pub const VILLAGE_MIN_CAMPS: usize = 3;
/// … und so viele Menschen zusammen.
pub const VILLAGE_MIN_PEOPLE: usize = 10;
/// So nah an einem zerfallenen Dorf bekommt ein neues dessen Namen (Felder).
const VILLAGE_REVIVE: i32 = 12;
/// Ein bestehendes Dorf hält sich mit zwei bewohnten Lagern und so vielen Menschen.
pub const VILLAGE_KEEP_PEOPLE: usize = 6;
/// Hilfe: so viel bringt man an einem Tag (Rationen).
const HELP_PER_DAY: f32 = 6.0;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Village {
    pub id: u32,
    pub name: String,
    /// Lager (Ids), nach Id sortiert.
    pub camps: Vec<u32>,
    pub founded_hours: u64,
    /// Feld in der Mitte (für den Namen auf der Karte).
    pub center: u32,
}

/// Ein Name aus der Umgebung der Lager: was am häufigsten um sie herum liegt.
fn village_name(world: &World, tiles: &[u32], rng: &mut Rng, used: &[String]) -> String {
    let (mut coast, mut lake, mut forest, mut meadow, mut high) = (0, 0, 0, 0, 0);
    for &t in tiles {
        let (cx, cy) = world.xy(t as usize);
        for dy in -2i32..=2 {
            for dx in -2i32..=2 {
                let (x, y) = (cx as i32 + dx, cy as i32 + dy);
                if x < 0 || y < 0 || x >= world.width as i32 || y >= world.height as i32 {
                    continue;
                }
                let tile = world.tile(x as u32, y as u32);
                if tile.is_ocean() {
                    coast += 1;
                } else if tile.surface_water > 3.0 * POND_MM {
                    lake += 1;
                } else if tile.forest > 0.4 {
                    forest += 1;
                } else if tile.elevation > 150.0 {
                    high += 1;
                } else {
                    meadow += 1;
                }
            }
        }
    }
    let kind = [
        (coast * 2, 0),
        (lake * 3, 1),
        (forest, 2),
        (high, 3),
        (meadow, 4),
    ]
    .into_iter()
    .max_by_key(|&(n, k)| (n, std::cmp::Reverse(k)))
    .map(|x| x.1)
    .unwrap_or(4);
    let (first, last): (&[&str], &[&str]) = match kind {
        0 => (
            &["Möwen", "Muschel", "Tang", "Robben", "Kiesel", "Salz"],
            &["bucht", "strand", "ufer", "hafen"],
        ),
        1 => (
            &["Reiher", "Schilf", "Binsen", "Otter", "Kiesel", "Spiegel"],
            &["see", "ufer", "weiher", "teich"],
        ),
        2 => (
            &["Erlen", "Birken", "Eschen", "Kiefern", "Moos", "Hasel"],
            &["grund", "hain", "wald", "lichtung"],
        ),
        3 => (
            &["Fels", "Adler", "Stein", "Wind", "Grat", "Geröll"],
            &["höhe", "eck", "stieg", "kamm"],
        ),
        _ => (
            &["Wiesen", "Klee", "Heide", "Lerchen", "Hasen", "Sonnen"],
            &["au", "feld", "anger", "matt"],
        ),
    };
    for _ in 0..12 {
        let a = first[rng.below(first.len() as u32) as usize];
        let b = last[rng.below(last.len() as u32) as usize];
        let name = format!("{a}{b}");
        if !used.contains(&name) {
            return name;
        }
    }
    format!("{}{} {}", first[0], last[0], used.len() + 1)
}

/// Dörfer neu bestimmen (einmal im Monat), Hilfe zwischen Lagern (täglich).
pub(super) fn village_day(
    people: &mut People,
    world: &World,
    now: u64,
    rng: &mut Rng,
    news: &mut Vec<PeopleNews>,
) {
    let day = now / 24;
    if day.is_multiple_of(30) {
        regroup(people, world, now, rng, news);
    }
    help(people);
}

/// Lager zu Dörfern zusammenfassen (Ketten mit höchstens `VILLAGE_LINK`).
fn regroup(
    people: &mut People,
    world: &World,
    now: u64,
    rng: &mut Rng,
    news: &mut Vec<PeopleNews>,
) {
    // Nur Lager, in denen jemand lebt
    let camps: Vec<(u32, u32, usize)> = people
        .camps
        .iter()
        .filter(|c| !c.abandoned)
        .map(|c| {
            (
                c.id,
                c.tile,
                people.list.iter().filter(|p| p.camp == c.id).count(),
            )
        })
        .filter(|c| c.2 > 0)
        .collect();
    // Gruppen über Nachbarschaft (feste Reihenfolge)
    let mut group = vec![usize::MAX; camps.len()];
    let mut groups: Vec<Vec<usize>> = Vec::new();
    for s in 0..camps.len() {
        if group[s] != usize::MAX {
            continue;
        }
        let g = groups.len();
        let mut stack = vec![s];
        let mut members = Vec::new();
        group[s] = g;
        while let Some(a) = stack.pop() {
            members.push(a);
            for b in 0..camps.len() {
                if group[b] == usize::MAX && cheb(world, camps[a].1, camps[b].1) <= VILLAGE_LINK {
                    group[b] = g;
                    stack.push(b);
                }
            }
        }
        members.sort_unstable();
        groups.push(members);
    }
    let mut next: Vec<Village> = Vec::new();
    for g in groups.iter().filter(|g| {
        let size = g.iter().map(|&k| camps[k].2).sum::<usize>();
        // Ein bestehendes Dorf hält sich länger, als es zum Entstehen braucht.
        let existing = people
            .villages
            .iter()
            .any(|v| g.iter().any(|&k| v.camps.contains(&camps[k].0)));
        (g.len() >= VILLAGE_MIN_CAMPS && size >= VILLAGE_MIN_PEOPLE)
            || (existing && g.len() >= 2 && size >= VILLAGE_KEEP_PEOPLE)
    }) {
        let ids: Vec<u32> = g.iter().map(|&k| camps[k].0).collect();
        let tiles: Vec<u32> = g.iter().map(|&k| camps[k].1).collect();
        let (sx, sy) = tiles.iter().fold((0u32, 0u32), |a, &t| {
            let (x, y) = world.xy(t as usize);
            (a.0 + x, a.1 + y)
        });
        let n = tiles.len() as u32;
        let center = world.idx(sx / n, sy / n) as u32;
        // Ein bestehendes Dorf, das die meisten dieser Lager hatte, lebt weiter.
        let old = people
            .villages
            .iter()
            .filter(|v| !next.iter().any(|w| w.id == v.id))
            .max_by_key(|v| {
                (
                    v.camps.iter().filter(|c| ids.contains(c)).count(),
                    std::cmp::Reverse(v.id),
                )
            })
            // … oder ein Dorf ganz in der Nähe, das sich nur verlagert hat
            .filter(|v| {
                v.camps.iter().any(|c| ids.contains(c))
                    || cheb(world, v.center, center) <= VILLAGE_LINK
            });
        match old {
            Some(v) => next.push(Village {
                camps: ids,
                center,
                ..v.clone()
            }),
            None => {
                people.next_village += 1;
                // Am Ort eines zerfallenen Dorfes lebt der alte Name wieder auf.
                let former = people
                    .former_villages
                    .iter()
                    .position(|f| cheb(world, f.1, center) <= VILLAGE_REVIVE);
                let (name, revived) = if let Some(k) = former {
                    (people.former_villages.remove(k).0, true)
                } else {
                    let used: Vec<String> = people
                        .villages
                        .iter()
                        .map(|v| v.name.clone())
                        .chain(people.village_names.iter().cloned())
                        .collect();
                    let name = village_name(world, &tiles, rng, &used);
                    people.village_names.push(name.clone());
                    (name, false)
                };
                let first = people.next_village == 1;
                news.push(PeopleNews {
                    kind: NewsKind::VillageFounded,
                    tile: center,
                    text: if revived {
                        format!(
                            "{name} ist wieder ein Dorf: {} Lager liegen bei {} nah beieinander.",
                            ids.len(),
                            xy_text(world, center)
                        )
                    } else if first {
                        format!(
                            "Das erste Dorf der Insel: {} Lager liegen bei {} nah beieinander – man nennt den Ort {name}.",
                            ids.len(),
                            xy_text(world, center)
                        )
                    } else {
                        format!(
                            "Ein neues Dorf: {} Lager bei {} – man nennt den Ort {name}.",
                            ids.len(),
                            xy_text(world, center)
                        )
                    },
                });
                next.push(Village {
                    id: people.next_village,
                    name,
                    camps: ids,
                    founded_hours: now,
                    center,
                });
            }
        }
    }
    // Verschwundene Dörfer
    for v in &people.villages {
        if !next.iter().any(|w| w.id == v.id) {
            people.former_villages.push((v.name.clone(), v.center));
            news.push(PeopleNews {
                kind: NewsKind::VillageGone,
                tile: v.center,
                text: format!(
                    "{} ist kein Dorf mehr: Zu wenige Lager sind geblieben.",
                    v.name
                ),
            });
        }
    }
    next.sort_by_key(|v| v.id);
    people.villages = next;
}

/// Nachbarschaftshilfe: Lager mit vollem Vorrat bringen hungernden Lagern
/// desselben Dorfes etwas vorbei.
fn help(people: &mut People) {
    for v in 0..people.villages.len() {
        let ids = people.villages[v].camps.clone();
        // Bedarf je Lager: Menschen und Vorrat
        let need: Vec<(usize, f32, usize)> = ids
            .iter()
            .filter_map(|id| {
                let ci = people.camps.iter().position(|c| c.id == *id)?;
                let members = people.list.iter().filter(|p| p.camp == *id).count();
                Some((ci, people.camps[ci].store, members))
            })
            .collect();
        let Some(&(poor, poor_store, poor_n)) = need
            .iter()
            .filter(|(_, s, n)| *n > 0 && *s < *n as f32 * 2.0)
            .min_by(|a, b| (a.1 / a.2 as f32).total_cmp(&(b.1 / b.2 as f32)))
        else {
            continue;
        };
        let Some(&(rich, rich_store, rich_n)) = need
            .iter()
            .filter(|(ci, s, n)| *ci != poor && *s > (*n as f32 * 15.0).max(20.0))
            .max_by(|a, b| a.1.total_cmp(&b.1))
        else {
            continue;
        };
        let give = HELP_PER_DAY
            .min(rich_store - (rich_n as f32 * 15.0).max(20.0))
            .min(people.camps[poor].store_cap() - poor_store)
            .max(0.0);
        if give <= 0.0 || poor_n == 0 {
            continue;
        }
        people.camps[rich].store -= give;
        people.camps[poor].store += give;
        people.camps[poor].help_received += give;
    }
}

/// Dorf, zu dem ein Lager gehört.
pub fn village_of(people: &People, camp: u32) -> Option<&Village> {
    if camp == 0 {
        return None;
    }
    people.villages.iter().find(|v| v.camps.contains(&camp))
}
