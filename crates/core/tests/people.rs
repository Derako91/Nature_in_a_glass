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
    for p in &sim.people.list {
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
