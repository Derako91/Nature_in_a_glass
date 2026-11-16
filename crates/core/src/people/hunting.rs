//! Jagd (v0.14): Menschen und Wild.
//!
//! Niemand befiehlt eine Jagd (Prinzip 1). Wer gesund ist, nicht verdurstet
//! und etwas Hunger hat, versucht es, wenn Wild in der Nähe ist: Hirsche
//! jagt man besser zu mehreren, Kaninchen allein. Jagen ist Können, das mit
//! Übung wächst und abgeschaut wird. Ein Hirsch bringt viel Fleisch – was die
//! Jäger nicht tragen, kommt in den Vorrat ihres Lagers oder bleibt liegen.
//!
//! Folgen für die Natur: Das Wild wird weniger; eine Insel kann ihre Hirsche
//! durch Überjagung verlieren, und dann hungern auch die Wölfe.

use super::camps::xy_text;
use super::{Activity, NewsKind, People, PeopleNews, cheb};
use crate::animals::{Animals, Carcass, Species};
use crate::math::clamp01;
use crate::rng::Rng;
use crate::world::World;

/// Fleisch eines Hirsches für Menschen (Rationen).
pub const DEER_RATIONS: f32 = 30.0;
/// So nah muss ein Hirsch sein (Felder).
const DEER_RANGE: i32 = 2;
/// Ab so vielen Kaninchen auf dem Feld lohnt die Jagd.
const RABBITS_MIN: f32 = 1.0;

/// Ein Jagdtag nach allen anderen Tätigkeiten.
pub(super) fn hunting_day(
    people: &mut People,
    animals: &mut Animals,
    world: &World,
    now: u64,
    rng: &mut Rng,
    news: &mut Vec<PeopleNews>,
) {
    let n = people.list.len();
    let mut done = vec![false; n];
    let ready = |p: &super::Person| {
        !p.is_child()
            && p.injury < 0.2
            && p.health > 0.6
            && p.thirst < 0.5
            // Gejagt wird bei echtem Hunger oder leerem Beutel – wer als Jäger
            // gilt, zieht schon früher los.
            && (p.hunger > if p.role() == Some(super::Role::Hunter) { 0.1 } else { 0.25 }
                || p.pack < 0.5)
            && p.settle == 0
            && p.activity != Activity::Run
    };
    for k in 0..n {
        if done[k] || !ready(&people.list[k]) {
            continue;
        }
        let here = people.list[k].tile;
        // --- Hirsch: nächster in Reichweite (feste Reihenfolge) ---
        let deer = animals
            .list
            .iter()
            .enumerate()
            .filter(|(_, a)| a.species == Species::Deer && cheb(world, a.tile, here) <= DEER_RANGE)
            .min_by_key(|(_, a)| (cheb(world, a.tile, here), a.id))
            .map(|(i, _)| i);
        if let Some(di) = deer {
            // Wer am selben Ort oder gleich daneben ist, jagt mit.
            let hunters: Vec<usize> = (0..n)
                .filter(|&j| {
                    !done[j]
                        && ready(&people.list[j])
                        && cheb(world, people.list[j].tile, here) <= 1
                })
                .collect();
            let skill =
                hunters.iter().map(|&j| people.list[j].hunting).sum::<f32>() / hunters.len() as f32;
            let chance = (0.03 * hunters.len() as f32 * (0.5 + skill)).min(0.4);
            for &j in &hunters {
                done[j] = true;
                let p = &mut people.list[j];
                p.hunting = (p.hunting + 0.01 * (1.0 - p.hunting)).min(1.0);
                p.hunger = clamp01(p.hunger + 0.005);
            }
            // Gefährlich: Ein Hirsch kann sich wehren.
            if rng.chance(0.001) {
                let j = hunters[rng.below(hunters.len() as u32) as usize];
                let p = &mut people.list[j];
                p.injury = clamp01(p.injury + 0.3);
                p.health -= 0.15;
                p.note(
                    now,
                    "Wird bei der Jagd von einem Hirsch verletzt.".to_string(),
                );
            }
            if !rng.chance(chance) {
                continue;
            }
            let deer = animals.list.remove(di);
            for &j in &hunters {
                people.list[j].activity = Activity::Run;
            }
            animals.hunted[Species::Deer.index()] += 1;
            animals.hunted_since[Species::Deer.index()] += 1;
            people.deer_hunted += 1;
            // Fleisch verteilen: erst die Jäger, dann das Lager
            let mut meat = DEER_RATIONS;
            for &j in &hunters {
                let p = &mut people.list[j];
                let take = (p.pack_max() - p.pack).clamp(0.0, meat);
                p.pack += take;
                meat -= take;
                p.hunting = (p.hunting + 0.05 * (1.0 - p.hunting)).min(1.0);
            }
            let camp = people.list[hunters[0]].camp;
            if meat > 0.0
                && let Some(c) = people
                    .camps
                    .iter_mut()
                    .find(|c| c.id == camp && !c.abandoned)
                && cheb(world, c.tile, here) <= 8
            {
                let put = meat.min(c.store_cap() - c.store).max(0.0);
                c.store += put;
                meat -= put;
            }
            if meat > 0.0 {
                // Der Rest bleibt liegen – für andere Menschen, Wölfe, Füchse.
                animals.carcasses.push(Carcass {
                    tile: deer.tile,
                    species: Species::Deer,
                    days_left: 10,
                    meat: (meat / 2.0).min(4.0),
                });
            }
            let names: Vec<String> = hunters
                .iter()
                .map(|&j| people.list[j].name.clone())
                .collect();
            let who = match names.len() {
                1 => names[0].clone(),
                _ => format!(
                    "{} und {}",
                    names[..names.len() - 1].join(", "),
                    names[names.len() - 1]
                ),
            };
            for &j in &hunters {
                people.list[j].note(now, "Erlegt mit anderen einen Hirsch.".to_string());
            }
            if people.deer_hunted == 1 {
                news.push(PeopleNews {
                    kind: NewsKind::FirstHunt,
                    tile: deer.tile,
                    text: format!(
                        "Zum ersten Mal erlegen Menschen einen Hirsch: {who} bei {} – das Fleisch reicht für Tage.",
                        xy_text(world, deer.tile)
                    ),
                });
            }
            continue;
        }
        // --- Kaninchen: allein, auf dem eigenen Feld (wenn kein Hirsch da ist) ---
        let i = here as usize;
        if animals.rabbits.get(i).copied().unwrap_or(0.0) >= RABBITS_MIN {
            done[k] = true;
            let p = &mut people.list[k];
            let chance = 0.08 * (0.5 + p.hunting);
            p.hunting = (p.hunting + 0.005 * (1.0 - p.hunting)).min(1.0);
            if rng.chance(chance) {
                animals.rabbits[i] = (animals.rabbits[i] - 1.0).max(0.0);
                animals.hunted[Species::Rabbit.index()] += 1;
                animals.hunted_since[Species::Rabbit.index()] += 1;
                p.pack = (p.pack + 1.0).min(p.pack_max());
            }
        }
    }
}
