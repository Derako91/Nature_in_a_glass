//! Regressionstests aus der Roadmap (Nummern wie in docs/ROADMAP.md).

use nig_core::fire::{flammability, ignite_for_test};
use nig_core::hydro::total_water;
use nig_core::{EventKind, Intervention, SeedKind, Simulation};

fn set_springs(sim: &mut Simulation, factor: f32) {
    for k in 0..sim.world.springs.len() {
        sim.apply(Intervention::SetSpringFactor { spring: k, factor });
    }
}

fn has_event(sim: &Simulation, kind: EventKind) -> bool {
    sim.journal.events.iter().any(|e| e.kind == kind)
}

/// Nr. 1 und Dennis' Austrocknungs-Experiment: Paradies, dann alles aus.
/// Der Wald stirbt – aber langsamer als in v0.1 (dort 50 % nach 210 Tagen),
/// hinterlässt Totholz und die Chronik berichtet davon.
#[test]
fn forest_dies_in_extreme_drought_slowly_and_leaves_deadwood() {
    let mut sim = Simulation::new(42);
    sim.apply(Intervention::SetAnnualRain { mm: 3000.0 });
    sim.apply(Intervention::SetMeanTemp { celsius: 25.0 });
    set_springs(&mut sim, 2.0);
    sim.advance_years(60);
    let start = sim.snapshot().forest_pct;
    assert!(start > 50.0, "Paradies ohne Wald: {start:.1} %");

    sim.apply(Intervention::SetAnnualRain { mm: 0.0 });
    set_springs(&mut sim, 0.0);
    sim.advance_days(250);
    let s = sim.snapshot();
    assert!(
        s.forest_pct > start * 0.5,
        "Wald stirbt zu schnell: {:.1} % von {start:.1} % nach 250 Tagen",
        s.forest_pct
    );

    sim.advance_years(6);
    let s = sim.snapshot();
    assert!(
        s.forest_pct < 1.0,
        "Wald überlebt völlige Dürre: {:.1} %",
        s.forest_pct
    );
    assert!(
        s.deadwood_pct > 15.0,
        "Kein Totholz: {:.1} %",
        s.deadwood_pct
    );
    assert!(has_event(&sim, EventKind::ForestDieback));
}

/// Nr. 1: Ohne Samenquelle kommt der Wald nicht zurück – auch nicht nach Regen.
#[test]
fn dead_forest_does_not_return_without_seeds() {
    let mut sim = Simulation::new(3);
    for t in sim.world.tiles.iter_mut() {
        t.forest = 0.0;
        t.forest_age = 0.0;
    }
    sim.advance_years(15);
    assert_eq!(sim.snapshot().forest_pct, 0.0);
}

/// Nr. 2: Gras kehrt nach dem Regen zurück (seine Samen überdauern im Boden).
#[test]
fn grass_returns_after_rain() {
    let mut sim = Simulation::new(5);
    sim.apply(Intervention::SetAnnualRain { mm: 0.0 });
    set_springs(&mut sim, 0.0);
    sim.advance_years(3);
    let dry = sim.snapshot().grass_pct;
    assert!(
        dry < 10.0,
        "Gras überlebt drei Jahre ohne Regen: {dry:.1} %"
    );

    sim.apply(Intervention::SetAnnualRain { mm: 950.0 });
    set_springs(&mut sim, 1.0);
    sim.advance_years(5);
    let wet = sim.snapshot().grass_pct;
    assert!(wet > 20.0, "Gras kehrt nicht zurück: {wet:.1} %");
}

/// Nr. 3: Ein einzelner Samen kann bei geeigneten Bedingungen einen neuen
/// Wald begründen – der Ort wird erst durch die Keimung bekannt.
#[test]
fn seed_can_found_new_forest() {
    let mut sim = Simulation::new(3);
    for t in sim.world.tiles.iter_mut() {
        t.forest = 0.0;
        t.forest_age = 0.0;
    }
    sim.apply(Intervention::ScatterSeed {
        kind: SeedKind::Tree,
    });
    let scatter = sim.journal.events.last().unwrap();
    assert_eq!(scatter.kind, EventKind::Intervention);
    assert!(scatter.location.is_none(), "Ort des Samens wird verraten");

    // Manche Samen landen ungünstig – der Beobachter versucht es mehrmals.
    let mut germinated = false;
    for _ in 0..10 {
        sim.advance_years(3);
        if has_event(&sim, EventKind::Germination) {
            germinated = true;
            break;
        }
        sim.apply(Intervention::ScatterSeed {
            kind: SeedKind::Tree,
        });
    }
    assert!(germinated, "Kein Samen ist in 30 Jahren aufgegangen");
    let g = sim
        .journal
        .events
        .iter()
        .find(|e| e.kind == EventKind::Germination)
        .unwrap();
    assert!(g.location.is_some());
    assert!(has_event(&sim, EventKind::TreesReturn));

    sim.advance_years(60);
    let s = sim.snapshot();
    assert!(
        s.tree_tiles > 3,
        "Aus dem Samen ist kein Bestand geworden: {} Felder",
        s.tree_tiles
    );
}

/// Nr. 3 (Gegenprobe): In der Wüste geht kein Samen auf.
#[test]
fn seed_fails_without_water() {
    let mut sim = Simulation::new(3);
    sim.apply(Intervention::SetAnnualRain { mm: 0.0 });
    set_springs(&mut sim, 0.0);
    sim.advance_years(1);
    sim.apply(Intervention::ScatterSeed {
        kind: SeedKind::Shrub,
    });
    sim.advance_years(4);
    assert!(!has_event(&sim, EventKind::Germination));
    assert!(has_event(&sim, EventKind::SeedFailed));
    assert!(sim.seeds.is_empty());
}

/// Nr. 4: Wasserbilanz geschlossen – auch bei Starkregen entsteht kein Wasser
/// aus dem Nichts, und Starkregen fließt nachvollziehbar ab.
#[test]
fn water_balance_is_closed() {
    let mut sim = Simulation::new(11);
    sim.apply(Intervention::SetAnnualRain { mm: 2500.0 });
    sim.advance_days(30);
    let mut max_runoff = 0.0f32;
    let mut throughput = 0.0f64;
    let mut error = 0.0f64;
    for _ in 0..730 {
        let before = total_water(&sim.world, sim.aquifer);
        sim.advance_days(1);
        let after = total_water(&sim.world, sim.aquifer);
        let d = sim.last_water;
        let input = d.rain_total as f64;
        let output =
            (d.to_sea + d.evaporation + d.transpiration + d.deep_loss + d.aquifer_leak) as f64;
        error += (after - before - (input - output)).abs();
        throughput += input + output;
        max_runoff = max_runoff.max(d.to_sea);
    }
    let rel = error / throughput;
    assert!(rel < 1e-3, "Wasserbilanz offen: relativer Fehler {rel:.2e}");
    assert!(max_runoff > 0.0, "Kein Abfluss ins Meer");
}

/// Nr. 5: Feuer hängt von den Bedingungen ab – derselbe Wald brennt nass
/// kaum, ausgedörrt mit Totholz großflächig. Die Bedingungen werden gezielt
/// hergestellt, damit der Test die Feuerregeln prüft und nicht die Vorgeschichte.
#[test]
fn fire_depends_on_conditions() {
    let mut base = Simulation::new(42);
    base.advance_years(1);
    base.apply(Intervention::SetAnnualRain { mm: 0.0 }); // kein Regen löscht
    for t in base.world.tiles.iter_mut().filter(|t| !t.is_ocean()) {
        t.forest = 0.8;
        t.forest_age = 80.0;
        t.deadwood = 0.5;
        t.surface_water = 0.0;
    }
    let w = &base.world;
    let start = (0..w.tiles.len())
        .filter(|&i| !w.tiles[i].is_ocean())
        .min_by_key(|&i| {
            let (x, y) = w.xy(i);
            (x as i32 - 32).abs() + (y as i32 - 32).abs()
        })
        .unwrap();

    let burned = |mut sim: Simulation, moisture: f32| {
        for t in sim.world.tiles.iter_mut() {
            t.soil_moisture = t.soil_capacity * moisture;
        }
        ignite_for_test(&mut sim.world, start);
        sim.advance_days(1);
        let first_day = flammability(&sim.world.tiles[start]);
        sim.advance_days(30);
        (
            sim.world.tiles.iter().filter(|t| t.burn_scar > 0.9).count(),
            first_day,
        )
    };

    let (wet_burned, wet_flamm) = burned(base.clone(), 1.0);
    let (dry_burned, _) = burned(base.clone(), 0.0);

    assert!(
        wet_flamm < 0.2,
        "Nasser Wald ist zu brennbar: {wet_flamm:.2}"
    );
    assert!(
        wet_burned <= 5,
        "Nasser Wald brennt großflächig: {wet_burned} Felder"
    );
    assert!(
        dry_burned >= 100,
        "Trockener Wald brennt kaum: {dry_burned} Felder"
    );
}

/// Nr. 5 (Gegenprobe): Ein Blitz in feuchter Vegetation zündet nicht.
#[test]
fn wet_vegetation_does_not_ignite() {
    let mut sim = Simulation::new(42);
    sim.advance_years(1);
    for t in sim.world.tiles.iter_mut().filter(|t| !t.is_ocean()) {
        t.soil_moisture = t.soil_capacity * 0.6;
        assert_eq!(flammability(t), 0.0);
    }
}

/// Nr. 6: Jahrhundertelanger Zeitraffer bleibt stabil.
#[test]
fn five_hundred_years_stay_stable() {
    let mut sim = Simulation::new(1234);
    sim.advance_years(500);
    for t in &sim.world.tiles {
        for v in [
            t.surface_water,
            t.soil_moisture,
            t.deep_moisture,
            t.grass,
            t.shrub,
            t.forest,
            t.deadwood,
            t.fire,
        ] {
            assert!(v.is_finite() && v >= -0.001, "ungültiger Wert {v}");
        }
        assert!(t.forest <= 1.0 && t.shrub <= 1.0 && t.grass <= 1.0);
        assert!(t.deep_moisture <= t.deep_capacity() + 0.01);
    }
    assert!(sim.aquifer.is_finite() && sim.aquifer >= 0.0);
    let last = sim.history.last().unwrap();
    assert!(
        last.forest_pct + last.shrub_pct + last.grass_pct > 20.0,
        "Insel verödet"
    );
    assert!(
        sim.history.iter().any(|y| y.fires > 0),
        "In 500 Jahren kein einziger Brand"
    );
}

/// Spielstände aus v0.1 lassen sich weiterhin laden und weiterrechnen.
#[test]
fn loads_version_1_savegame() {
    let json = include_str!("fixtures/v1_seed7_3y.json");
    let mut sim = Simulation::from_json(json).expect("v0.1-Spielstand lesbar");
    assert_eq!(sim.year(), 4);
    let forested = sim.world.tiles.iter().filter(|t| t.forest > 0.0).count();
    assert!(forested > 0);
    assert!(
        sim.world
            .tiles
            .iter()
            .filter(|t| t.forest > 0.0)
            .all(|t| t.forest_age > 0.0)
    );
    let events_before = sim.journal.events.len();
    sim.advance_years(2);
    assert!(sim.journal.events.len() > events_before);
    assert!(
        !sim.journal.events[events_before..]
            .iter()
            .any(|e| e.kind == EventKind::TreesReturn),
        "Migration meldet fälschlich zurückkehrende Bäume"
    );
    // Und im neuen Format speichern und wieder laden
    let again = Simulation::from_json(&sim.to_json()).unwrap();
    assert_eq!(again.state_hash(), sim.state_hash());
}
