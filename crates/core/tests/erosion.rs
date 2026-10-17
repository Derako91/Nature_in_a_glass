//! v0.3: Bodenabtrag, Ablagerung, Bodenbildung und benannte Seen.

use nig_core::erosion::{self, M3_PER_TILE_M};
use nig_core::world::water_per_soil_meter;
use nig_core::{EventKind, Intervention, Simulation};

fn steep_land(sim: &Simulation) -> Vec<usize> {
    let w = &sim.world;
    (0..w.tiles.len())
        .filter(|&i| {
            let t = &w.tiles[i];
            if t.is_ocean() || t.surface_water > 150.0 {
                return false;
            }
            let drop = w
                .neighbors(i)
                .map(|n| t.elevation - w.tiles[n].elevation)
                .fold(0.0f32, f32::max);
            drop / 50.0 > 0.12
        })
        .collect()
}

fn soil(sim: &Simulation, i: usize) -> f32 {
    let t = &sim.world.tiles[i];
    t.soil_depth + t.soil_change
}

/// Ein kahler Hang verliert bei Regen viel mehr Boden als ein bewachsener.
#[test]
fn bare_slopes_lose_soil_vegetation_protects() {
    let mut sim = Simulation::new(42);
    sim.advance_years(20);
    let steep = steep_land(&sim);
    assert!(steep.len() > 100);
    let (bare, covered): (Vec<usize>, Vec<usize>) = steep.iter().partition(|&&i| i % 2 == 0);
    for &i in &bare {
        let t = &mut sim.world.tiles[i];
        t.forest = 0.0;
        t.shrub = 0.0;
        t.grass = 0.0;
        t.deadwood = 0.0;
        t.burn_scar = 1.0;
    }
    let before: Vec<f32> = (0..sim.world.tiles.len()).map(|i| soil(&sim, i)).collect();
    sim.apply(Intervention::SetAnnualRain { mm: 1800.0 });
    sim.advance_years(1);
    let mean_loss =
        |v: &[usize]| v.iter().map(|&i| before[i] - soil(&sim, i)).sum::<f32>() / v.len() as f32;
    let lost_bare = mean_loss(&bare);
    let lost_covered = mean_loss(&covered);
    assert!(
        lost_bare > 0.002,
        "kahler Hang verliert nur {:.2} mm",
        lost_bare * 1000.0
    );
    assert!(
        lost_bare > 5.0 * lost_covered.max(0.0),
        "kahl {:.2} mm, bewachsen {:.2} mm",
        lost_bare * 1000.0,
        lost_covered * 1000.0
    );
    assert!(
        sim.journal
            .events
            .iter()
            .any(|e| e.kind == EventKind::SoilErosion),
        "Abspülung nicht in der Chronik"
    );
}

/// Was oben fehlt, liegt unten oder im Meer: Die Summe aus Gelände und
/// noch nicht übernommenen Änderungen bleibt gleich.
#[test]
fn sediment_balance_is_closed() {
    let mut sim = Simulation::new(7);
    let start = erosion::total_ground(&sim.world);
    let mut moved_m3 = 0.0f64;
    for _ in 0..60 {
        sim.advance_years(1);
        moved_m3 += sim.history.last().unwrap().eroded_m3 as f64;
    }
    let moved = moved_m3 / M3_PER_TILE_M as f64;
    let diff = (erosion::total_ground(&sim.world) - start).abs();
    assert!(moved > 1.0, "kaum Abtrag: {moved_m3:.0} m³");
    assert!(
        diff < 0.01 * moved,
        "Bilanz offen: {diff:.3} m bei {moved:.1} m bewegtem Boden"
    );
    let to_sea: f32 = sim.history.iter().map(|y| y.sediment_to_sea_m3).sum();
    assert!(to_sea > 0.0, "kein Boden erreicht das Meer");
}

/// Über Jahrhunderte: Land bleibt Land, das Wasserhaltevermögen folgt der
/// Bodendicke, Senken und Seen füllen sich auf.
#[test]
fn centuries_of_erosion_stay_consistent() {
    let mut sim = Simulation::new(42);
    let start: Vec<f32> = sim.world.tiles.iter().map(|t| t.elevation).collect();
    let was_land: Vec<bool> = sim.world.tiles.iter().map(|t| !t.is_ocean()).collect();
    sim.advance_years(3);
    let lake_tiles: Vec<usize> = sim
        .lakes
        .current()
        .flat_map(|l| l.tiles.iter().map(|&t| t as usize))
        .collect();
    assert!(!lake_tiles.is_empty(), "keine Seen gefunden");
    sim.advance_years(297);

    for (i, t) in sim.world.tiles.iter().enumerate() {
        if was_land[i] {
            assert!(!t.is_ocean(), "Feld {i} ist zu Meer geworden");
        }
        assert!(t.soil_depth >= 0.0);
        if !t.is_ocean() {
            let expected = t.soil_depth * water_per_soil_meter(t.elevation);
            assert!((t.soil_capacity - expected).abs() < 0.01, "Feld {i}");
            assert!(t.soil_moisture <= t.soil_capacity + 0.001);
        }
    }
    let rise: f32 = lake_tiles
        .iter()
        .map(|&i| sim.world.tiles[i].elevation - start[i])
        .sum::<f32>()
        / lake_tiles.len() as f32;
    assert!(rise > 0.01, "Seeboden hebt sich nicht ({rise:.3} m)");
    // Plausibles Tempo: wenige Zentimeter je Jahrhundert im Mittel der Insel
    let land: Vec<usize> = (0..start.len()).filter(|&i| was_land[i]).collect();
    let mean_abs = land
        .iter()
        .map(|&i| (sim.world.tiles[i].elevation - start[i]).abs())
        .sum::<f32>()
        / land.len() as f32;
    assert!(
        mean_abs < 0.5,
        "Gelände ändert sich zu schnell: {mean_abs:.2} m"
    );
}

/// Seen bekommen Namen, behalten sie und tragen sie eindeutig.
#[test]
fn lakes_are_named_and_keep_their_names() {
    let mut sim = Simulation::new(42);
    sim.advance_years(3);
    let names: Vec<String> = sim.lakes.current().map(|l| l.name.clone()).collect();
    assert!(!names.is_empty());
    let mut sorted = names.clone();
    sorted.sort();
    sorted.dedup();
    assert_eq!(sorted.len(), names.len(), "doppelte Namen: {names:?}");
    assert!(
        sim.journal
            .events
            .iter()
            .any(|e| e.kind == EventKind::LakeFormed)
    );
    let biggest = sim
        .lakes
        .current()
        .max_by_key(|l| l.tiles.len())
        .unwrap()
        .clone();
    sim.advance_years(10);
    let tile = biggest.tiles[0];
    assert_eq!(sim.lakes.name_at(tile), Some(biggest.name.as_str()));

    // Gleicher Seed, gleiche Namen
    let mut other = Simulation::new(42);
    other.advance_years(3);
    let again: Vec<String> = other.lakes.current().map(|l| l.name.clone()).collect();
    assert_eq!(names, again);
}

/// Ein ausgelaufener See wird gemeldet (Durchbruch des Beobachters).
#[test]
fn drained_lake_is_reported() {
    let mut sim = Simulation::new(42);
    sim.advance_years(3);
    sim.apply(Intervention::SetAnnualRain { mm: 0.0 });
    for k in 0..sim.world.springs.len() {
        sim.apply(Intervention::SetSpringFactor {
            spring: k,
            factor: 0.0,
        });
    }
    sim.advance_years(8);
    assert!(
        sim.journal
            .events
            .iter()
            .any(|e| e.kind == EventKind::LakeGone),
        "verschwundener See nicht gemeldet"
    );
}

/// Spielstände ohne Bodenschicht bekommen sie aus dem Wasserhaltevermögen,
/// bestehende Seen werden erkannt.
#[test]
fn old_savegames_get_soil_depth_and_lake_names() {
    for json in [
        include_str!("fixtures/v2_seed7_3y.json"),
        include_str!("fixtures/v4_seed7_3y.json"),
    ] {
        check_old_savegame(json);
    }
}

fn check_old_savegame(json: &str) {
    let mut sim = Simulation::from_json(json).unwrap();
    for t in &sim.world.tiles {
        if !t.is_ocean() {
            let expected = t.soil_capacity / water_per_soil_meter(t.elevation);
            assert!((t.soil_depth - expected).abs() < 1e-4);
        }
    }
    sim.advance_years(4);
    assert!(sim.mean_soil_depth() > 0.05);
    assert!(sim.lakes.current().count() > 0, "keine Seen benannt");
    // Weiterlaufen nach Speichern und Laden bleibt identisch
    let again = Simulation::from_json(&sim.to_json()).unwrap();
    assert_eq!(again.state_hash(), sim.state_hash());
}
