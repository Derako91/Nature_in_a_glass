//! Simulationstests über lange Zeiträume.

use nig_core::{EventKind, Intervention, Simulation};

#[test]
fn same_seed_same_world_after_years() {
    let mut a = Simulation::new(42);
    let mut b = Simulation::new(42);
    a.advance_years(5);
    b.advance_years(5);
    assert_eq!(a.state_hash(), b.state_hash());
}

#[test]
fn different_seeds_differ() {
    let a = Simulation::new(1);
    let b = Simulation::new(2);
    assert_ne!(a.state_hash(), b.state_hash());
}

#[test]
fn stepping_granularity_does_not_matter() {
    // Stundenweise oder in einem Rutsch: gleiches Ergebnis.
    let mut a = Simulation::new(7);
    let mut b = Simulation::new(7);
    for _ in 0..(400 * 24) {
        a.advance_hours(1);
    }
    b.advance_days(400);
    assert_eq!(a.state_hash(), b.state_hash());
}

#[test]
fn save_and_load_continue_identically() {
    let mut straight = Simulation::new(11);
    straight.advance_years(6);

    let mut first = Simulation::new(11);
    first.advance_years(3);
    let json = first.to_json();
    let mut resumed = Simulation::from_json(&json).expect("Spielstand lesbar");
    resumed.advance_years(3);

    assert_eq!(straight.state_hash(), resumed.state_hash());
}

#[test]
fn rejects_unknown_save_format() {
    let json =
        Simulation::new(1)
            .to_json()
            .replacen("\"format_version\":1", "\"format_version\":999", 1);
    assert!(Simulation::from_json(&json).is_err());
}

#[test]
fn hundred_years_stay_plausible() {
    let mut sim = Simulation::new(42);
    sim.advance_years(100);
    assert_eq!(sim.year(), 101);
    assert_eq!(sim.history.len(), 100);

    for t in &sim.world.tiles {
        assert!(t.surface_water.is_finite() && t.surface_water >= 0.0);
        assert!(t.soil_moisture >= -0.001 && t.soil_moisture <= t.soil_capacity + 0.001);
        assert!((0.0..=1.0).contains(&t.grass));
        assert!((0.0..=1.0).contains(&t.forest));
    }
    for y in &sim.history {
        assert!(
            y.rain_mm > 300.0 && y.rain_mm < 2500.0,
            "Jahr {}: {} mm",
            y.year,
            y.rain_mm
        );
    }
    let avg_rain: f32 = sim.history.iter().map(|y| y.rain_mm).sum::<f32>() / 100.0;
    assert!(
        (800.0..1100.0).contains(&avg_rain),
        "Mittlerer Regen {avg_rain}"
    );

    // Die Insel lebt: Vegetation verschwindet nicht und Bäche existieren.
    let last = sim.history.last().unwrap();
    assert!(last.forest_pct > 5.0);
    assert!(last.grass_pct > 5.0);
    assert!(sim.history.iter().any(|y| y.stream_tiles > 10));
}

#[test]
fn no_rain_turns_island_brown() {
    let mut sim = Simulation::new(5);
    sim.apply(Intervention::SetAnnualRain { mm: 0.0 });
    for k in 0..sim.world.springs.len() {
        sim.apply(Intervention::SetSpringFactor {
            spring: k,
            factor: 0.0,
        });
    }
    let before = sim.snapshot();
    sim.advance_years(3);
    let after = sim.snapshot();
    assert!(after.grass_pct < before.grass_pct * 0.5);
    assert!(
        sim.journal
            .events
            .iter()
            .any(|e| e.kind == EventKind::DroughtStart)
    );
}

#[test]
fn closing_a_spring_is_recorded_with_cause() {
    let mut sim = Simulation::new(9);
    assert!(!sim.world.springs.is_empty());
    sim.apply(Intervention::SetSpringFactor {
        spring: 0,
        factor: 0.0,
    });
    sim.advance_days(2);
    let dry = sim
        .journal
        .events
        .iter()
        .find(|e| e.kind == EventKind::SpringDry)
        .expect("Versiegen wird protokolliert");
    assert!(dry.causes.iter().any(|c| c.contains("Eingriff")));
    assert!(
        sim.journal
            .events
            .iter()
            .any(|e| e.kind == EventKind::Intervention)
    );
}

#[test]
fn cleared_forest_only_returns_from_edges() {
    // Permanenz: Ohne Samen in der Nähe wächst kein Wald.
    let mut sim = Simulation::new(3);
    for t in sim.world.tiles.iter_mut() {
        t.forest = 0.0;
    }
    sim.advance_years(10);
    assert_eq!(sim.snapshot().forest_pct, 0.0);
}

/// Referenzwert: Seed 42 nach 20 Jahren. Die CI prüft diesen Test unter
/// Windows, macOS und Linux – so fällt auf, wenn eine Plattform anders rechnet.
///
/// Ändert sich die Simulation absichtlich, muss dieser Wert neu gesetzt werden
/// (`nig-sim --seed 42 --years 20` zeigt die neue Prüfsumme).
const REFERENCE_HASH_SEED42_20Y: u64 = 0xa10e_29e9_2cc2_7dd8;

#[test]
fn cross_platform_reference_hash() {
    let mut sim = Simulation::new(42);
    sim.advance_years(20);
    assert_eq!(
        sim.state_hash(),
        REFERENCE_HASH_SEED42_20Y,
        "Prüfsumme {:016x} weicht vom Referenzwert ab",
        sim.state_hash()
    );
}
