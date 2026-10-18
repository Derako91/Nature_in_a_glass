//! v0.4: Tiere – Aussetzen, Fressen, Jagd, Fortpflanzung, Aussterben.

use nig_core::animals::{Cause, Species};
use nig_core::{EventKind, Intervention, Simulation};

fn release(sim: &mut Simulation, species: Species) {
    sim.apply(Intervention::ReleaseAnimals { species, at: None });
}

#[test]
fn new_island_has_no_animals() {
    let mut sim = Simulation::new(42);
    sim.advance_years(3);
    for s in Species::ALL {
        assert_eq!(sim.animals.total(s), 0, "{s:?}");
    }
}

#[test]
fn released_deer_multiply_and_graze() {
    let mut sim = Simulation::new(42);
    sim.advance_years(15);
    let mut control = sim.clone();
    release(&mut sim, Species::Deer);
    assert_eq!(sim.animals.total(Species::Deer), 6);
    assert!(
        sim.journal
            .events
            .iter()
            .any(|e| e.kind == EventKind::Intervention && e.message.contains("Hirsche"))
    );
    sim.advance_years(18);
    control.advance_years(18);
    let deer = sim.animals.total(Species::Deer);
    assert!(deer > 50, "nur {deer} Hirsche nach 18 Jahren");
    let (a, b) = (sim.snapshot(), control.snapshot());
    assert!(
        a.grass_pct + a.shrub_pct < b.grass_pct + b.shrub_pct - 5.0,
        "Hirsche fressen kaum: {:.0}+{:.0} gegen {:.0}+{:.0}",
        a.grass_pct,
        a.shrub_pct,
        b.grass_pct,
        b.shrub_pct
    );
}

/// Ohne Futter verhungern sie – und kommen nicht von selbst zurück.
#[test]
fn starving_deer_die_out_and_stay_gone() {
    let mut sim = Simulation::new(5);
    sim.advance_years(10);
    release(&mut sim, Species::Deer);
    sim.apply(Intervention::SetAnnualRain { mm: 0.0 });
    for k in 0..sim.world.springs.len() {
        sim.apply(Intervention::SetSpringFactor {
            spring: k,
            factor: 0.0,
        });
    }
    sim.advance_years(4);
    assert_eq!(sim.animals.total(Species::Deer), 0);
    let gone = sim
        .journal
        .events
        .iter()
        .find(|e| e.kind == EventKind::AnimalsExtinct)
        .expect("Aussterben nicht gemeldet");
    assert!(gone.message.contains("Hirsche"));
    assert!(gone.causes.iter().any(|c| c.contains("verhungert")));
    // Regen zurück: Gras wächst wieder, Hirsche nicht.
    sim.apply(Intervention::SetAnnualRain { mm: 950.0 });
    sim.advance_years(15);
    assert_eq!(sim.animals.total(Species::Deer), 0);
}

#[test]
fn wolves_hunt_deer() {
    let mut sim = Simulation::new(42);
    sim.advance_years(10);
    release(&mut sim, Species::Deer);
    sim.advance_years(12);
    let mut control = sim.clone();
    release(&mut sim, Species::Wolf);
    let mut killed = 0;
    for _ in 0..3 {
        sim.advance_days(364);
        killed += sim.animals.deaths.get(Species::Deer, Cause::Killed);
        sim.advance_days(1);
        control.advance_years(1);
    }
    assert!(killed >= 5, "nur {killed} Hirsche gerissen");
    assert!(
        sim.animals.total(Species::Deer) < control.animals.total(Species::Deer),
        "Wölfe ohne Wirkung auf die Hirsche"
    );
}

#[test]
fn foxes_eat_rabbits() {
    let mut sim = Simulation::new(42);
    sim.advance_years(10);
    release(&mut sim, Species::Rabbit);
    sim.advance_years(4);
    let r0 = sim.animals.total(Species::Rabbit);
    assert!(r0 > 100, "Kaninchen vermehren sich nicht: {r0}");
    let mut control = sim.clone();
    release(&mut sim, Species::Fox);
    sim.advance_years(2);
    control.advance_years(2);
    assert!(
        sim.animals.total(Species::Rabbit) < control.animals.total(Species::Rabbit),
        "Füchse ohne Wirkung"
    );
}

#[test]
fn animals_are_reproducible_and_survive_save_load() {
    let run = || {
        let mut sim = Simulation::new(9);
        sim.advance_years(8);
        for s in Species::ALL {
            release(&mut sim, s);
        }
        sim.advance_years(3);
        sim
    };
    let a = run();
    let b = run();
    assert_eq!(a.state_hash(), b.state_hash());

    let mut straight = a.clone();
    straight.advance_years(2);
    let mut resumed = Simulation::from_json(&a.to_json()).unwrap();
    resumed.advance_years(2);
    assert_eq!(straight.state_hash(), resumed.state_hash());
    assert_eq!(straight.animals, resumed.animals);
}

#[test]
fn cannot_release_into_water() {
    let mut sim = Simulation::new(42);
    let before = sim.journal.events.len();
    sim.apply(Intervention::ReleaseAnimals {
        species: Species::Deer,
        at: Some((0, 0)),
    });
    assert_eq!(sim.animals.total(Species::Deer), 0);
    assert_eq!(sim.journal.events.len(), before);
}

/// Dennis' Beobachtung in 0.4.0: Auf einer alten, dicht bewaldeten Insel
/// schrumpften ausgesetzte Kaninchen ohne Fressfeinde von 20 auf 4.
#[test]
fn rabbits_spread_on_a_forested_island() {
    let mut sim = Simulation::new(42);
    sim.advance_years(150);
    assert!(sim.snapshot().forest_pct > 60.0);
    release(&mut sim, Species::Rabbit);
    sim.advance_years(8);
    let n = sim.animals.total(Species::Rabbit);
    assert!(n > 300, "nur {n} Kaninchen nach 8 Jahren");
}
