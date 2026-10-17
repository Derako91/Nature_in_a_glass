//! v0.3: Dauerhafte Bäche, zeitweise Bäche und Regenrinnen unterscheiden;
//! Wassermengen in echten Einheiten.

use nig_core::hydro::{CourseKind, M3_PER_TILE_MM, liters_per_second, watercourse};
use nig_core::{Intervention, Simulation};

fn set_springs(sim: &mut Simulation, factor: f32) {
    for k in 0..sim.world.springs.len() {
        sim.apply(Intervention::SetSpringFactor { spring: k, factor });
    }
}

fn spring_tile(sim: &Simulation, k: usize) -> usize {
    let s = &sim.world.springs[k];
    sim.world.idx(s.x, s.y)
}

/// Unter einer Quelle entsteht ohne Regen ein dauerhafter Bach.
#[test]
fn spring_creek_becomes_permanent_without_rain() {
    let mut sim = Simulation::new(42);
    sim.advance_years(5);
    sim.apply(Intervention::SetAnnualRain { mm: 0.0 });
    sim.advance_years(3);
    let below = sim.world.downstream[spring_tile(&sim, 0)].expect("Quelle ohne Abfluss") as usize;
    let wc = watercourse(&sim.world.tiles[below]);
    assert_eq!(wc.kind, CourseKind::Permanent, "Quellbach nicht dauerhaft");
    assert!(sim.snapshot().permanent_tiles > 0);
    // Ohne Regen gibt es keine Regenrinnen
    assert_eq!(sim.snapshot().rain_channel_tiles, 0);
}

/// Ohne Quellen und ohne Regen trocknen auch dauerhafte Bäche aus.
#[test]
fn permanent_creeks_fade_without_water() {
    let mut sim = Simulation::new(42);
    sim.advance_years(5);
    sim.apply(Intervention::SetAnnualRain { mm: 0.0 });
    set_springs(&mut sim, 0.0);
    sim.advance_years(3);
    let s = sim.snapshot();
    assert_eq!(s.permanent_tiles, 0, "Bäche ohne Wasserquelle dauerhaft");
    assert_eq!(s.seasonal_tiles, 0, "fließende Bäche ohne Wasser");
}

/// Im normalen Klima gibt es alle drei Arten: Quellbäche fließen das ganze
/// Jahr, Winterbäche nur zeitweise, Regenrinnen nur nach Regen.
#[test]
fn normal_climate_has_all_kinds() {
    let mut sim = Simulation::new(42);
    sim.advance_years(30);
    let (mut permanent, mut seasonal, mut rain) = (0, 0, 0);
    for _ in 0..365 {
        sim.advance_days(1);
        let s = sim.snapshot();
        permanent = permanent.max(s.permanent_tiles);
        seasonal = seasonal.max(s.seasonal_tiles + s.seasonal_dry_tiles);
        rain = rain.max(s.rain_channel_tiles);
    }
    assert!(permanent > 0, "keine dauerhaften Bäche");
    assert!(seasonal > 0, "keine zeitweisen Bäche");
    assert!(rain > 0, "keine Regenrinnen");
}

/// Einheiten: 1 mm auf einem Feld sind 2,5 m³; Abfluss wird in m³ gezählt.
#[test]
fn volumes_use_real_units() {
    assert!((M3_PER_TILE_MM - 2.5).abs() < 1e-6);
    // 86,4 m³ am Tag sind 1 Liter pro Sekunde
    assert!((liters_per_second(86.4 / 2.5) - 1.0).abs() < 1e-4);

    let mut sim = Simulation::new(11);
    sim.advance_years(1);
    let mut sum = 0.0f64;
    for _ in 0..365 {
        sim.advance_days(1);
        sum += (sim.last_water.to_sea * M3_PER_TILE_MM) as f64;
    }
    let year = sim.history.last().unwrap().discharge_m3 as f64;
    assert!(
        (year - sum).abs() / sum.max(1.0) < 0.01,
        "Jahresabfluss {year:.0} m³ ≠ Summe {sum:.0} m³"
    );
}

/// Spielstände aus v0.2 lassen sich laden; das Jahresgedächtnis wird geschätzt.
#[test]
fn loads_version_2_savegame() {
    let json = include_str!("fixtures/v2_seed7_3y.json");
    let mut sim = Simulation::from_json(json).expect("v0.2-Spielstand lesbar");
    assert_eq!(sim.year(), 4);
    assert!(sim.world.tiles.iter().any(|t| t.flow_permanence > 0.0));
    sim.advance_years(1);
    let again = Simulation::from_json(&sim.to_json()).unwrap();
    assert_eq!(again.state_hash(), sim.state_hash());
}
