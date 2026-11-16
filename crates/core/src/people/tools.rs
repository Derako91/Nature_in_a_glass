//! Werkzeuge (v0.13): Netz, Korb, Steinbeil.
//!
//! Werkzeuge werden nicht verordnet (Prinzip 1), sie entstehen aus dem Tun:
//! Wer oft und geschickt fischt, knüpft irgendwann ein Netz; wer oft mit
//! vollen Händen vom Sammeln kommt, flicht einen Korb; wer oft Holz holt,
//! schlägt sich ein Steinbeil. Das Wissen ist persönlich (Prinzip 6): Wer mit
//! Kundigen am selben Ort ist, lernt es mit der Zeit, Kinder schneller.
//! Sterben alle, die es konnten, ist es verloren – bis es jemand neu erfindet.
//!
//! Wirkung: Netz – mehr Fisch; Korb – mehr Gesammeltes und mehr tragen;
//! Steinbeil – doppelt so viel Holz am Tag.

use super::{Activity, NewsKind, People, PeopleNews, Person, Sex};
use crate::rng::Rng;
use crate::world::World;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tool {
    Net,
    Basket,
    Axe,
    /// Tongefäß (seit v0.15): Vorräte verderben langsamer.
    Pot,
}

pub const TOOLS: [Tool; 4] = [Tool::Net, Tool::Basket, Tool::Axe, Tool::Pot];

impl Tool {
    pub fn bit(self) -> u8 {
        match self {
            Tool::Net => 1,
            Tool::Basket => 2,
            Tool::Axe => 4,
            Tool::Pot => 8,
        }
    }
    pub fn name(self) -> &'static str {
        match self {
            Tool::Net => "Netz",
            Tool::Basket => "Korb",
            Tool::Axe => "Steinbeil",
            Tool::Pot => "Tongefäß",
        }
    }
    /// „ein Netz“, „einen Korb“, „ein Steinbeil“
    pub fn with_article(self) -> &'static str {
        match self {
            Tool::Net => "ein Netz",
            Tool::Basket => "einen Korb",
            Tool::Axe => "ein Steinbeil",
            Tool::Pot => "ein Tongefäß",
        }
    }
    /// „Netze knüpfen“, „Körbe flechten“, „Steinbeile schlagen“
    pub fn craft(self) -> &'static str {
        match self {
            Tool::Net => "Netze zu knüpfen",
            Tool::Basket => "Körbe zu flechten",
            Tool::Axe => "Steinbeile zu schlagen",
            Tool::Pot => "Tongefäße zu brennen",
        }
    }
}

impl Person {
    pub fn knows(&self, t: Tool) -> bool {
        self.tools & t.bit() != 0
    }
}

/// Ein Tongefäß zu erfinden liegt nahe, wer am eigenen Lager steht, das einen
/// Garten und gut gefüllten Vorrat hat, und Wasser (Lehm) daneben.
fn pot_place(people: &People, world: &World, p: &Person) -> bool {
    p.camp != 0
        && people.gardens.iter().any(|g| g.camp == p.camp)
        && people.camps.iter().any(|c| {
            c.id == p.camp
                && !c.abandoned
                && c.store >= 50.0
                && super::cheb(world, c.tile, p.tile) <= 1
        })
        && super::drinkable(world, p.tile as usize)
}

/// In Tongefäßen verderben Vorräte so viel langsamer (Faktor).
pub const POT_SPOIL: f32 = 0.5;

/// Mehr Fisch mit Netz.
pub const NET_FACTOR: f32 = 1.6;
/// Mehr Gesammeltes mit Korb.
pub const BASKET_FACTOR: f32 = 1.3;
/// Mit Korb trägt man so viel mehr (Rationen).
pub const BASKET_PACK: f32 = 1.5;

/// Ein Tag: erfinden, abschauen, vergessen.
pub(super) fn tools_day(
    people: &mut People,
    world: &World,
    now: u64,
    rng: &mut Rng,
    news: &mut Vec<PeopleNews>,
) {
    // --- Erfinden aus dem Tun ---
    for k in 0..people.list.len() {
        let p = &people.list[k];
        if p.is_child() {
            continue;
        }
        let chance = |t: Tool| -> f32 {
            if p.knows(t) {
                return 0.0;
            }
            match t {
                Tool::Net if p.activity == Activity::Fish && p.fishing >= 0.5 => 0.003,
                Tool::Basket
                    if matches!(p.activity, Activity::Gather | Activity::Eat)
                        && p.pack >= super::PACK_MAX - 0.2 =>
                {
                    0.0004
                }
                Tool::Axe if p.activity == Activity::Build => 0.003,
                // Am Lager mit Garten und gefülltem Vorrat, Lehm vom Ufer
                Tool::Pot if pot_place(people, world, p) => 0.0005,
                _ => 0.0,
            }
        };
        let found: Vec<Tool> = TOOLS
            .iter()
            .copied()
            .filter(|&t| {
                let c = chance(t);
                c > 0.0 && rng.chance(c)
            })
            .collect();
        for t in found {
            let p = &mut people.list[k];
            p.tools |= t.bit();
            let (x, y) = world.xy(p.tile as usize);
            let first = people.tools_ever & t.bit() == 0;
            let again = !first;
            people.tools_ever |= t.bit();
            let text = match (t, first) {
                (Tool::Net, true) => format!(
                    "{} knüpft bei ({x}, {y}) aus Pflanzenfasern das erste Netz der Insel – damit fängt man viel mehr Fisch.",
                    p.name
                ),
                (Tool::Basket, true) => format!(
                    "{} flicht bei ({x}, {y}) aus Weiden und Gras den ersten Korb der Insel – man kann mehr sammeln und tragen.",
                    p.name
                ),
                (Tool::Axe, true) => format!(
                    "{} schlägt bei ({x}, {y}) aus einem Feuerstein das erste Steinbeil der Insel – Holz zu holen geht nun viel schneller.",
                    p.name
                ),
                (Tool::Pot, true) => format!(
                    "{} formt bei ({x}, {y}) aus Lehm vom Ufer das erste Tongefäß der Insel und brennt es im Feuer – Vorräte halten darin viel länger.",
                    p.name
                ),
                (t, _) => format!(
                    "{} findet wieder heraus, wie man {} – das Wissen war verloren gegangen.",
                    p.name,
                    t.craft()
                ),
            };
            let who = if p.sex == Sex::Female { "Sie" } else { "Er" };
            p.note(
                now,
                if again {
                    format!("Findet heraus, wie man {}.", t.craft())
                } else {
                    format!(
                        "Erfindet {}. {who} ist der erste Mensch auf der Insel, der das kann.",
                        t.with_article()
                    )
                },
            );
            news.push(PeopleNews {
                kind: NewsKind::ToolInvented,
                tile: p.tile,
                text,
            });
        }
    }

    // --- Abschauen bei Kundigen am selben Ort ---
    let knowers: Vec<(u32, u8, String)> = people
        .list
        .iter()
        .filter(|p| !p.is_child() && p.tools != 0)
        .map(|p| (p.tile, p.tools, p.name.clone()))
        .collect();
    for p in &mut people.list {
        if p.age_days < 6 * 365 {
            continue;
        }
        for t in TOOLS {
            if p.knows(t) {
                continue;
            }
            let Some(teacher) = knowers
                .iter()
                .find(|kn| kn.0 == p.tile && kn.1 & t.bit() != 0 && kn.2 != p.name)
            else {
                continue;
            };
            let chance = if p.is_child() { 0.02 } else { 0.01 };
            if rng.chance(chance) {
                p.tools |= t.bit();
                let teacher = teacher.2.clone();
                p.note(now, format!("Lernt von {teacher}, {}.", t.craft()));
            }
        }
    }

    // --- Verloren: niemand kann es mehr ---
    let now_known = people.list.iter().fold(0u8, |a, p| a | p.tools);
    for t in TOOLS {
        if people.tools_alive & t.bit() != 0 && now_known & t.bit() == 0 {
            let tile = people.departed.last().map(|d| d.person.tile).unwrap_or(0);
            news.push(PeopleNews {
                kind: NewsKind::ToolLost,
                tile,
                text: format!(
                    "Niemand auf der Insel weiß mehr, wie man {}: Das Wissen ist mit den Toten verloren gegangen.",
                    t.craft()
                ),
            });
        }
    }
    people.tools_alive = now_known;
}
