//! Rollen im Lager (v0.15): wer etwas am besten kann.
//!
//! Rollen werden nicht vergeben (Prinzip 1, keine festen Rollen nach
//! Prinzip 4): Einmal im Monat schaut jedes Lager, wer unter seinen
//! Erwachsenen am besten fischt, gärtnert oder jagt. Wer darin geübt ist
//! (ab 70 %) und es am besten kann, gilt als Fischerin, Gärtner, Jägerin …
//! Jeder hat höchstens eine Rolle; ist jemand in allem der Beste, bekommt die
//! nächste Rolle der oder die Nächstbeste.
//! Wer eine Rolle hat, tut es auch mit mehr Hingabe: Fischer fangen mehr,
//! Gärtnerinnen pflegen besser, Jäger ziehen schon bei kleinem Hunger los.
//! Stirbt jemand oder überholt ihn ein anderer, wechselt die Rolle.

use super::{People, Person, Sex};

/// Ab diesem Können kommt eine Rolle in Frage.
pub const ROLE_SKILL: f32 = 0.7;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Fisher = 1,
    Gardener = 2,
    Hunter = 3,
}

impl Role {
    fn from_u8(v: u8) -> Option<Role> {
        match v {
            1 => Some(Role::Fisher),
            2 => Some(Role::Gardener),
            3 => Some(Role::Hunter),
            _ => None,
        }
    }
    fn skill(self, p: &Person) -> f32 {
        match self {
            Role::Fisher => p.fishing,
            Role::Gardener => p.farming,
            Role::Hunter => p.hunting,
        }
    }
    pub fn label(self, sex: Sex) -> &'static str {
        match (self, sex) {
            (Role::Fisher, Sex::Female) => "Fischerin",
            (Role::Fisher, Sex::Male) => "Fischer",
            (Role::Gardener, Sex::Female) => "Gärtnerin",
            (Role::Gardener, Sex::Male) => "Gärtner",
            (Role::Hunter, Sex::Female) => "Jägerin",
            (Role::Hunter, Sex::Male) => "Jäger",
        }
    }
}

impl Person {
    pub fn role(&self) -> Option<Role> {
        Role::from_u8(self.role)
    }
    /// „Fischerin“, „Jäger“ … oder nichts.
    pub fn role_label(&self) -> Option<&'static str> {
        self.role().map(|r| r.label(self.sex))
    }
}

/// Einmal im Monat die Rollen je Lager bestimmen.
pub(super) fn roles_day(people: &mut People, now: u64) {
    if !(now / 24).is_multiple_of(30) {
        return;
    }
    let mut next = vec![0u8; people.list.len()];
    let camps: Vec<u32> = people
        .camps
        .iter()
        .filter(|c| !c.abandoned)
        .map(|c| c.id)
        .collect();
    for cid in camps {
        let members: Vec<usize> = (0..people.list.len())
            .filter(|&k| people.list[k].camp == cid && !people.list[k].is_child())
            .collect();
        for role in [Role::Fisher, Role::Gardener, Role::Hunter] {
            // Der oder die Beste, ab 70 %; bei Gleichstand der Ältere
            // Je Rolle die beste Person, die noch keine andere Rolle hat
            let best = members
                .iter()
                .copied()
                .filter(|&k| next[k] == 0 && role.skill(&people.list[k]) >= ROLE_SKILL)
                .max_by(|&a, &b| {
                    let (pa, pb) = (&people.list[a], &people.list[b]);
                    role.skill(pa)
                        .total_cmp(&role.skill(pb))
                        .then(pa.age_days.cmp(&pb.age_days))
                        .then(pb.id.cmp(&pa.id))
                });
            if let Some(k) = best {
                next[k] = role as u8;
            }
        }
    }
    for (k, p) in people.list.iter_mut().enumerate() {
        if p.role != next[k] {
            let old = p.role();
            p.role = next[k];
            match (old, p.role()) {
                (_, Some(r)) => {
                    let label = r.label(p.sex);
                    let art = if p.sex == Sex::Female {
                        "die beste"
                    } else {
                        "der beste"
                    };
                    p.note(now, format!("Gilt nun als {art} {label} am Lager."));
                }
                (Some(r), None) => {
                    let label = r.label(p.sex);
                    p.note(now, format!("Ist nicht mehr die erste Wahl als {label}."));
                }
                _ => {}
            }
        }
    }
}
