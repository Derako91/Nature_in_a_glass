//! Quellen: Aus- und Wiedereinschalten muss im Wasserfluss nachvollziehbar sein.
//!
//! Damit Regen das Ergebnis nicht überdeckt, wird er für diese Tests
//! abgeschaltet – gemessen wird nur das Quellwasser.

use nig_core::{Intervention, Simulation};

/// Felder von der Quelle bergab bis zum Meer.
fn path_of(sim: &Simulation, spring: usize) -> Vec<usize> {
    let s = &sim.world.springs[spring];
    let mut i = sim.world.idx(s.x, s.y);
    let mut path = vec![];
    while !sim.world.tiles[i].is_ocean() {
        path.push(i);
        i = sim.world.downstream[i].expect("Landfeld ohne Abfluss") as usize;
    }
    path
}

/// Mittlerer Durchfluss auf den ersten Feldern unterhalb der Quelle.
fn creek_flow(sim: &Simulation, path: &[usize]) -> f32 {
    let n = path.len().min(5);
    path[..n].iter().map(|&i| sim.world.tiles[i].flow).sum::<f32>() / n as f32
}

fn set_springs(sim: &mut Simulation, factor: f32) {
    for k in 0..sim.world.springs.len() {
        sim.apply(Intervention::SetSpringFactor { spring: k, factor });
    }
}

/// Insel nach 20 Jahren, Regen abgeschaltet, Quellen normal.
fn dry_island() -> Simulation {
    let mut sim = Simulation::new(42);
    sim.advance_years(20);
    sim.apply(Intervention::SetAnnualRain { mm: 0.0 });
    sim.advance_days(30); // Regenwasser abfließen lassen
    sim
}

#[test]
fn spring_off_dries_creek_and_on_restores_it() {
    let mut sim = dry_island();
    let path = path_of(&sim, 0);
    assert!(path.len() >= 3, "Quelle zu nah am Meer für den Test");

    sim.advance_days(30);
    let on = creek_flow(&sim, &path);
    assert!(on > 20.0, "Bach unter laufender Quelle fehlt: {on:.1} mm/Tag");

    // Ausschalten: sofort keine Schüttung, Bach trocknet allmählich aus
    set_springs(&mut sim, 0.0);
    sim.advance_days(1);
    assert_eq!(sim.world.springs[0].current, 0.0);
    sim.advance_days(7);
    let after_week = creek_flow(&sim, &path);
    assert!(after_week < on, "Bach wird nach dem Abschalten nicht schwächer");
    sim.advance_days(60);
    let off = creek_flow(&sim, &path);
    assert!(off < on * 0.05, "Bach trocknet nicht aus: {off:.1} von {on:.1} mm/Tag");

    // Wieder einschalten: Bach kehrt zurück
    set_springs(&mut sim, 1.0);
    sim.advance_days(30);
    let again = creek_flow(&sim, &path);
    assert!(again > on * 0.5, "Bach kehrt nicht zurück: {again:.1} von {on:.1} mm/Tag");
}

#[test]
fn spring_output_scales_with_factor() {
    let base = dry_island();
    let mut out = vec![];
    for factor in [0.2f32, 1.0, 3.0] {
        let mut sim = base.clone();
        set_springs(&mut sim, factor);
        sim.advance_days(1);
        out.push(sim.world.springs[0].current);
    }
    // Am selben Ausgangszustand verhält sich die Schüttung wie der Regler
    assert!((out[0] / out[1] - 0.2).abs() < 0.01, "20 %: {:?}", out);
    assert!((out[2] / out[1] - 3.0).abs() < 0.03, "300 %: {:?}", out);

    // Mehr Quellwasser → mehr Wasser im Bach
    let mut flows = vec![];
    for factor in [0.2f32, 1.0, 3.0] {
        let mut sim = base.clone();
        set_springs(&mut sim, factor);
        sim.advance_days(20);
        flows.push(creek_flow(&sim, &path_of(&sim, 0)));
    }
    assert!(flows[0] < flows[1] && flows[1] < flows[2], "Bach folgt dem Regler nicht: {flows:?}");
}

#[test]
fn spring_on_off_is_reproducible() {
    let run = || {
        let mut sim = dry_island();
        set_springs(&mut sim, 0.0);
        sim.advance_days(40);
        set_springs(&mut sim, 3.0);
        sim.advance_days(40);
        sim.state_hash()
    };
    assert_eq!(run(), run());
}
