//! v0.6: Menschen – Ankunft, Bedürfnisse, Wissen, Biografie, Tod.

use nig_core::people::{Cause, PlaceKind};
use nig_core::{EventKind, Intervention, Simulation};

fn arrive(sim: &mut Simulation) {
    sim.apply(Intervention::PeopleArrive { at: None });
}

#[test]
fn new_island_has_no_people() {
    let mut sim = Simulation::new(42);
    sim.advance_years(3);
    assert_eq!(sim.people.alive(), 0);
}

#[test]
fn boat_brings_five_people_to_the_coast() {
    let mut sim = Simulation::new(42);
    sim.advance_years(10);
    arrive(&mut sim);
    assert_eq!(sim.people.alive(), 5);
    let tile = sim.people.list[0].tile as usize;
    assert!(
        sim.world
            .neighbors(tile)
            .any(|j| sim.world.tiles[j].is_ocean()),
        "Boot landet nicht an der Küste"
    );
    let ev = sim
        .journal
        .events
        .iter()
        .find(|e| e.kind == EventKind::Intervention && e.message.contains("Boot"))
        .expect("Ankunft in der Chronik");
    for p in &sim.people.list {
        assert!(ev.message.contains(&p.name), "{} fehlt", p.name);
        assert!(!p.bio.is_empty());
    }
    // Fünf verschiedene Aussehen, Frauen und Männer gemischt
    let mut looks: Vec<u8> = sim.people.list.iter().map(|p| p.look).collect();
    looks.sort();
    assert_eq!(looks, vec![0, 1, 2, 3, 4]);
}

/// Auf einer gewöhnlichen Insel überleben die meisten die ersten Jahre:
/// Sie finden Wasser, sammeln Nahrung und überstehen den Winter.
#[test]
fn people_find_water_and_survive_the_first_years() {
    let mut sim = Simulation::new(42);
    sim.advance_years(10);
    arrive(&mut sim);
    sim.advance_years(5);
    assert!(
        sim.people.alive() >= 3,
        "nur {} leben noch",
        sim.people.alive()
    );
    // Nur die mit dem Boot Gekommenen (Kinder können noch jung sein)
    for p in sim.people.list.iter().filter(|p| p.mother == 0) {
        assert!(
            p.memory.iter().any(|m| m.kind == PlaceKind::Water),
            "{} kennt kein Wasser",
            p.name
        );
        assert!(
            p.bio.iter().any(|b| b.text.contains("Winter")),
            "{}: kein Winter in der Biografie",
            p.name
        );
        assert!(p.hunger < 0.9 && p.thirst < 1.0);
    }
}

/// Ohne Regen und Quellen versiegt alles Wasser – dann verdursten sie.
/// Tote bleiben mit Biografie in Erinnerung und liegen einige Tage da.
#[test]
fn without_water_people_die_of_thirst() {
    let mut sim = Simulation::new(5);
    sim.advance_years(2);
    sim.apply(Intervention::SetAnnualRain { mm: 0.0 });
    for k in 0..sim.world.springs.len() {
        sim.apply(Intervention::SetSpringFactor {
            spring: k,
            factor: 0.0,
        });
    }
    sim.advance_years(3);
    arrive(&mut sim);
    let mut saw_body = false;
    for _ in 0..120 {
        sim.advance_days(1);
        saw_body |= !sim.people.bodies.is_empty();
        if sim.people.alive() == 0 {
            break;
        }
    }
    assert_eq!(sim.people.alive(), 0, "Menschen überleben ohne Wasser");
    assert!(saw_body);
    assert_eq!(sim.people.departed.len(), 5);
    assert!(
        sim.people.departed.iter().any(|d| d.cause == Cause::Thirst),
        "niemand verdurstet: {:?}",
        sim.people
            .departed
            .iter()
            .map(|d| d.cause)
            .collect::<Vec<_>>()
    );
    for d in &sim.people.departed {
        assert!(d.person.bio.last().unwrap().text.contains("Jahren"));
    }
    assert!(
        sim.journal
            .events
            .iter()
            .any(|e| e.kind == EventKind::PeopleGone)
    );
    assert!(EventKind::PersonDied.is_important());
    sim.advance_days(20);
    assert!(sim.people.bodies.is_empty(), "Tote verschwinden nicht");
}

#[test]
fn people_are_deterministic_and_survive_save_load() {
    let mut a = Simulation::new(7);
    a.advance_years(8);
    arrive(&mut a);
    a.advance_days(200);
    let json = a.to_json();
    let mut b = Simulation::from_json(&json).unwrap();
    a.advance_days(400);
    b.advance_days(400);
    assert_eq!(a.state_hash(), b.state_hash());
    assert_eq!(a.people, b.people);
}

/// v0.7: Paare finden sich, Kinder kommen zur Welt, wachsen bei der Mutter
/// auf und kennen ihre Familie.
#[test]
fn couples_form_and_children_are_born() {
    let mut sim = Simulation::new(42);
    sim.advance_years(10);
    arrive(&mut sim);
    sim.advance_years(12);
    let pe = &sim.people;
    assert!(pe.born >= 1, "kein Kind in 12 Jahren");
    let all: Vec<&nig_core::people::Person> = pe
        .list
        .iter()
        .chain(pe.departed.iter().map(|d| &d.person))
        .collect();
    let couples = all.iter().filter(|p| p.partner != 0).count()
        + pe.departed
            .iter()
            .filter(|d| !d.person.children.is_empty())
            .count();
    assert!(couples >= 1, "kein Paar");
    let child = all
        .iter()
        .find(|p| p.mother != 0)
        .expect("ein Kind mit Mutter");
    let mother = all.iter().find(|p| p.id == child.mother).unwrap();
    assert!(mother.children.contains(&child.id));
    assert!(child.bio[0].text.contains("geboren"));
    assert!(child.bio[0].text.contains(&mother.name));
    assert!(
        sim.journal
            .events
            .iter()
            .any(|e| e.kind == EventKind::FirstBorn)
    );
    // Kleinkinder sind bei einem Erwachsenen
    for c in pe.list.iter().filter(|p| p.is_small_child()) {
        assert!(
            pe.list.iter().any(|a| !a.is_child() && a.tile == c.tile),
            "{} ist allein",
            c.name
        );
    }
}

// ---------- v0.8: Lagerplätze ----------

fn settled(seed: u64, years: u64) -> Simulation {
    let mut sim = Simulation::new(seed);
    sim.advance_years(10);
    arrive(&mut sim);
    sim.advance_years(years);
    sim
}

/// Wer viele Nächte an verlässlichem Wasser verbringt, legt dort ein Lager
/// an. Sind die Menschen zahlreich genug, entstehen Hütten.
#[test]
fn people_settle_at_reliable_water_and_build_huts() {
    let sim = settled(2024, 5);
    let pe = &sim.people;
    assert!(!pe.camps.is_empty(), "kein Lagerplatz in 5 Jahren");
    for c in pe.camps.iter().filter(|c| !c.abandoned) {
        assert!(
            nig_core::people::drinkable(&sim.world, c.tile as usize),
            "Lager ohne Wasser"
        );
    }
    assert!(pe.huts_built >= 1, "keine Hütte");
    assert!(
        pe.list.iter().any(|p| p.camp != 0),
        "niemand gehört zum Lager"
    );
    let ev = &sim.journal.events;
    assert!(ev.iter().any(|e| e.kind == EventKind::CampFounded));
    assert!(ev.iter().any(|e| e.kind == EventKind::HutBuilt));
    // Lager liegen nicht dicht nebeneinander
    for a in pe.camps.iter().filter(|a| !a.abandoned) {
        for b in pe.camps.iter().filter(|b| b.id < a.id && !b.abandoned) {
            let (ax, ay) = sim.world.xy(a.tile as usize);
            let (bx, by) = sim.world.xy(b.tile as usize);
            let d = (ax as i32 - bx as i32)
                .abs()
                .max((ay as i32 - by as i32).abs());
            assert!(d >= nig_core::people::camps_spacing(), "Lager zu dicht");
        }
    }
}

/// Holz für Feuer und Hütten fehlt danach im Wald rund um das Lager.
#[test]
fn camp_wood_thins_the_forest_nearby() {
    let with = settled(2024, 12);
    let mut without = Simulation::new(2024);
    without.advance_years(22);
    let camp = with.people.camps.first().expect("ein Lager").tile as usize;
    let wood = |sim: &Simulation| -> f32 {
        let (cx, cy) = sim.world.xy(camp);
        let mut sum = 0.0;
        for y in cy.saturating_sub(3)..=(cy + 3).min(sim.world.height - 1) {
            for x in cx.saturating_sub(3)..=(cx + 3).min(sim.world.width - 1) {
                let t = sim.world.tile(x, y);
                sum += t.forest + t.deadwood;
            }
        }
        sum
    };
    assert!(
        wood(&with) < wood(&without) - 0.3,
        "mit Lager {:.2}, ohne {:.2}",
        wood(&with),
        wood(&without)
    );
}

/// Kommt niemand mehr, wird das Lager verlassen und verfällt.
#[test]
fn abandoned_camps_decay_and_disappear() {
    let mut sim = settled(2024, 3);
    assert!(!sim.people.camps.is_empty());
    // Alle verlassen die Insel (nur im Test)
    sim.people.list.clear();
    sim.advance_years(3);
    assert!(sim.people.camps.iter().all(|c| c.abandoned));
    assert!(
        sim.journal
            .events
            .iter()
            .any(|e| e.kind == EventKind::CampAbandoned)
    );
    sim.advance_years(3);
    assert!(sim.people.camps.is_empty(), "Lager verfällt nicht");
    assert!(
        sim.journal
            .events
            .iter()
            .any(|e| e.kind == EventKind::CampGone)
    );
}

#[test]
fn camps_survive_save_load() {
    let mut a = settled(2024, 2);
    assert!(!a.people.camps.is_empty());
    let mut b = Simulation::from_json(&a.to_json()).unwrap();
    assert_eq!(a.people.camps, b.people.camps);
    a.advance_days(300);
    b.advance_days(300);
    assert_eq!(a.state_hash(), b.state_hash());
}

// ---------- v0.9: Fischfang, Wasser wiederfinden ----------

/// Wer am Wasser lebt, lernt zu fischen; das Können geht an Kinder weiter.
#[test]
fn people_learn_to_fish_and_teach_children() {
    let sim = settled(5, 20);
    let pe = &sim.people;
    assert!(pe.first_fisher, "niemand ist geschickt im Fischen");
    assert!(
        sim.journal
            .events
            .iter()
            .any(|e| e.kind == EventKind::FirstFisher)
    );
    // Auf der Insel geborene Erwachsene haben das Fischen von anderen gelernt.
    let born_adults: Vec<_> = pe
        .list
        .iter()
        .filter(|p| p.mother != 0 && !p.is_child())
        .collect();
    assert!(
        born_adults.is_empty() || born_adults.iter().any(|p| p.fishing > 0.2),
        "Können geht nicht weiter"
    );
}

/// Ein ausgetrockneter Teich hält niemanden fest: Wer dort Durst hat, sucht
/// weiter und findet anderes Wasser.
#[test]
fn thirsty_people_leave_a_dried_up_water_place() {
    let mut sim = settled(5, 2);
    let w = &sim.world;
    // Eine Stelle ohne Wasser, die alle für eine sichere Wasserstelle halten
    let dry = (0..w.tiles.len())
        .find(|&i| {
            let t = &w.tiles[i];
            !t.is_ocean()
                && t.surface_water < 1.0
                && !nig_core::people::drinkable(w, i)
                && w.neighbors(i).all(|j| !w.tiles[j].is_ocean())
        })
        .unwrap() as u32;
    for p in &mut sim.people.list {
        p.tile = dry;
        p.thirst = 0.3;
        p.memory.push(nig_core::people::Place {
            tile: dry,
            kind: PlaceKind::Water,
            value: 1.0,
            seen_day: 0,
        });
    }
    sim.advance_days(1);
    assert!(
        sim.people.list.iter().filter(|p| !p.is_child()).all(|p| !p
            .memory
            .iter()
            .any(|m| m.tile == dry && m.kind == PlaceKind::Water)),
        "trockene Stelle nicht vergessen"
    );
}

// ---------- v0.10: Gärten ----------

/// Wer lange an einem Lager lebt, legt irgendwann einen Garten an; die Ernte
/// kommt in den Vorrat. Ohne Menschen verwildert er.
#[test]
fn gardens_grow_are_harvested_and_go_wild() {
    let mut sim = settled(300, 20);
    let pe = &sim.people;
    assert!(!pe.gardens.is_empty(), "kein Garten in 20 Jahren");
    assert!(pe.harvests >= 1, "keine Ernte");
    assert!(
        pe.list.iter().any(|p| p.farming > 0.2),
        "niemand kann gärtnern"
    );
    let ev = &sim.journal.events;
    assert!(ev.iter().any(|e| e.kind == EventKind::GardenPlanted));
    assert!(ev.iter().any(|e| e.kind == EventKind::Harvest));
    // Gärten liegen neben einem Lager
    for g in &pe.gardens {
        let c = pe.camps.iter().find(|c| c.id == g.camp).expect("Lager");
        let (gx, gy) = sim.world.xy(g.tile as usize);
        let (cx, cy) = sim.world.xy(c.tile as usize);
        assert!(
            (gx as i32 - cx as i32)
                .abs()
                .max((gy as i32 - cy as i32).abs())
                <= 2
        );
    }
    // Speichern und Laden
    let b = Simulation::from_json(&sim.to_json()).unwrap();
    assert_eq!(b.people.gardens, sim.people.gardens);
    // Alle verlassen die Insel: die Gärten verwildern
    sim.people.list.clear();
    sim.advance_years(4);
    assert!(sim.people.gardens.is_empty(), "Garten verwildert nicht");
    assert!(
        sim.journal
            .events
            .iter()
            .any(|e| e.kind == EventKind::GardenLost)
    );
}
