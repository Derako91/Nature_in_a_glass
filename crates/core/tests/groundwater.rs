//! v0.3: Grundwasser als echter Füllstand mit Ober- und Untergrenze.

use nig_core::hydro::{SPRING_MIN_LEVEL, spring_pressure, total_water};
use nig_core::{EventKind, Intervention, Simulation};

fn set_springs(sim: &mut Simulation, factor: f32) {
    for k in 0..sim.world.springs.len() {
        sim.apply(Intervention::SetSpringFactor { spring: k, factor });
    }
}

/// Füllstand bleibt immer zwischen 0 und 100 % – auch bei Extremregen.
#[test]
fn level_stays_within_bounds_and_overflows_when_full() {
    let mut sim = Simulation::new(7);
    sim.apply(Intervention::SetAnnualRain { mm: 5000.0 });
    let mut overflowed = 0.0f64;
    let (mut error, mut throughput) = (0.0f64, 1.0f64);
    for _ in 0..(3 * 365) {
        let before = total_water(&sim.world, sim.aquifer);
        sim.advance_days(1);
        let after = total_water(&sim.world, sim.aquifer);
        let level = sim.groundwater_level();
        assert!((0.0..=1.0).contains(&level), "Füllstand {level}");
        assert!(sim.aquifer <= sim.aquifer_capacity + 0.01);
        let d = sim.last_water;
        overflowed += d.aquifer_overflow as f64;
        let input = d.rain_total as f64;
        let output =
            (d.to_sea + d.evaporation + d.transpiration + d.deep_loss + d.aquifer_leak) as f64;
        error += (after - before - (input - output)).abs();
        throughput += input + output;
    }
    assert!(overflowed > 0.0, "Voller Grundwasserkörper läuft nie über");
    assert!(
        sim.groundwater_level() > 0.99,
        "Extremregen füllt das Grundwasser nicht"
    );
    assert!(error / throughput < 1e-3, "Wasserbilanz mit Überlauf offen");
}

/// Quellen versiegen, bevor das Grundwasser leer ist – sie liegen höher.
#[test]
fn springs_run_dry_above_empty() {
    assert_eq!(spring_pressure(SPRING_MIN_LEVEL), 0.0);
    assert_eq!(spring_pressure(0.0), 0.0);
    assert!(spring_pressure(0.5) > 0.0);

    // Ohne Regen werden die Quellen zum dünnen Rinnsal (aus dem tiefen Boden
    // sickert noch lange etwas nach) und gelten als versiegt, während im
    // Grundwasserkörper unterhalb der Austritte noch Wasser steht.
    let mut sim = Simulation::new(42);
    sim.advance_years(5);
    sim.apply(Intervention::SetAnnualRain { mm: 0.0 });
    sim.advance_years(6);
    for s in &sim.world.springs {
        assert!(
            s.dry,
            "Quelle bei ({}, {}) gilt nicht als versiegt",
            s.x, s.y
        );
        assert!(
            s.current < 0.1 * s.base_rate,
            "Quelle schüttet noch {:.1}",
            s.current
        );
    }
    assert!(sim.aquifer > 0.0, "Grundwasser ganz leer");
    assert!(
        sim.groundwater_level() < 0.1,
        "Füllstand {:.1} %",
        sim.groundwater_level() * 100.0
    );
    assert!(
        sim.journal
            .events
            .iter()
            .any(|e| e.kind == EventKind::SpringDry)
    );
}

/// Tiefstand wird gemeldet und bei Erholung ebenso.
#[test]
fn low_groundwater_is_reported_and_recovery_too() {
    let mut sim = Simulation::new(42);
    sim.advance_years(5);
    sim.apply(Intervention::SetAnnualRain { mm: 0.0 });
    sim.advance_years(4);
    let low = sim
        .journal
        .events
        .iter()
        .find(|e| e.kind == EventKind::GroundwaterLow)
        .expect("Tiefstand nicht gemeldet");
    assert!(!low.causes.is_empty());

    sim.apply(Intervention::SetAnnualRain { mm: 2500.0 });
    set_springs(&mut sim, 0.0); // nichts entnehmen, damit es sich füllt
    sim.advance_years(6);
    assert!(
        sim.journal
            .events
            .iter()
            .any(|e| e.kind == EventKind::GroundwaterRecovered),
        "Erholung nicht gemeldet (Füllstand {:.0} %)",
        sim.groundwater_level() * 100.0
    );
}

/// Spielstände aus Format 3 bekommen ein Fassungsvermögen.
#[test]
fn old_savegames_get_capacity() {
    let json = include_str!("fixtures/v2_seed7_3y.json");
    let sim = Simulation::from_json(json).unwrap();
    assert!(sim.aquifer_capacity > 0.0);
    assert!(sim.aquifer <= sim.aquifer_capacity);
    assert!((0.0..=1.0).contains(&sim.groundwater_level()));
}
