//! v0.3: Wasser folgt Gelände und Wasserspiegel – Dämme, Durchbrüche,
//! Überschwemmungen. Regen wird abgeschaltet, wo nur Quellwasser zählen soll.

use nig_core::hydro::total_water;
use nig_core::world::POND_MM;
use nig_core::{EventKind, Intervention, Simulation};

fn path_of(sim: &Simulation, spring: usize) -> Vec<usize> {
    let s = &sim.world.springs[spring];
    let mut i = sim.world.idx(s.x, s.y);
    let mut path = vec![];
    while !sim.world.tiles[i].is_ocean() && !path.contains(&i) {
        path.push(i);
        match sim.world.downstream[i] {
            Some(d) => i = d as usize,
            None => break,
        }
    }
    path
}

/// Ein Bachstück außerhalb von Seen: (Feld oberhalb, Dammfeld).
fn creek_segment(sim: &Simulation) -> (usize, usize) {
    for k in 0..sim.world.springs.len() {
        let p = path_of(sim, k);
        for w in p.windows(2) {
            let (a, b) = (&sim.world.tiles[w[0]], &sim.world.tiles[w[1]]);
            if a.surface_water < POND_MM && b.surface_water < POND_MM && b.flow > 30.0 {
                return (w[0], w[1]);
            }
        }
    }
    panic!("Kein Bachstück außerhalb von Seen gefunden");
}

/// So viel aufschütten, dass das Dammfeld 5 m über dem Wasserspiegel oberhalb liegt.
fn dam_height(sim: &Simulation, above: usize, dam: usize) -> f32 {
    nig_core::hydro::head(&sim.world.tiles[above]) - sim.world.tiles[dam].elevation + 5.0
}

/// Insel mit laufender Quelle, ohne Regen.
fn spring_island() -> Simulation {
    let mut sim = Simulation::new(42);
    sim.advance_years(20);
    sim.apply(Intervention::SetAnnualRain { mm: 0.0 });
    sim.advance_days(60);
    sim
}

/// Gesamte Wassermenge auf und in der Insel bleibt erklärbar.
fn balance_error(sim: &mut Simulation, days: u32) -> f64 {
    let (mut error, mut throughput) = (0.0f64, 1.0f64);
    for _ in 0..days {
        let before = total_water(&sim.world, sim.aquifer);
        sim.advance_days(1);
        let after = total_water(&sim.world, sim.aquifer);
        let d = sim.last_water;
        let input = d.rain_total as f64;
        let output =
            (d.to_sea + d.evaporation + d.transpiration + d.deep_loss + d.aquifer_leak) as f64;
        error += (after - before - (input - output)).abs();
        throughput += input + output;
    }
    error / throughput
}

#[test]
fn dam_blocks_creek_and_breach_restores_it() {
    let mut sim = spring_island();
    let (above, dam) = creek_segment(&sim);
    let (dx, dy) = sim.world.xy(dam);
    let flow_before = sim.world.tiles[dam].flow;
    let height = dam_height(&sim, above, dam);

    // Damm aufschütten
    sim.apply(Intervention::ChangeTerrain {
        x: dx,
        y: dy,
        meters: height,
    });
    let err = balance_error(&mut sim, 60);
    assert!(err < 1e-3, "Wasserbilanz mit Damm offen: {err:.2e}");
    // Der Bach erreicht das Dammfeld nicht mehr; es bekommt nur noch Wasser,
    // das seitlich vom Hang zuläuft.
    assert_ne!(
        sim.world.downstream[above],
        Some(dam as u32),
        "Der Bach fließt weiter in den Damm"
    );
    let flow_dammed = sim.world.tiles[dam].flow;
    assert!(
        flow_dammed < flow_before * 0.5,
        "Damm hält kaum: {flow_dammed:.1} von {flow_before:.1} mm/Tag"
    );
    // Das Wasser staut sich vor dem Damm oder sucht sich einen anderen Weg
    let ponded = sim.world.tiles[above].surface_water;
    let bypass: f32 = sim
        .world
        .neighbors(above)
        .filter(|&n| n != dam)
        .map(|n| sim.world.tiles[n].flow)
        .fold(0.0, f32::max);
    assert!(
        ponded > POND_MM || bypass > 15.0,
        "Wasser verschwindet: Stau {ponded:.0} mm, Umweg {bypass:.0} mm/Tag"
    );

    // Durchbruch: Damm wieder abtragen
    sim.apply(Intervention::ChangeTerrain {
        x: dx,
        y: dy,
        meters: -height,
    });
    sim.advance_days(60);
    let flow_after = sim.world.tiles[dam].flow;
    assert!(
        flow_after > flow_before * 0.5,
        "Bach kehrt nach Durchbruch nicht zurück: {flow_after:.1} von {flow_before:.1}"
    );
    let interventions = sim
        .journal
        .events
        .iter()
        .filter(|e| e.kind == EventKind::Intervention && e.location == Some((dx, dy)))
        .count();
    assert_eq!(
        interventions, 2,
        "Damm und Durchbruch stehen nicht in der Chronik"
    );
}

#[test]
fn breach_drains_a_dammed_pond() {
    let mut sim = spring_island();
    let (above, dam) = creek_segment(&sim);
    let (dx, dy) = sim.world.xy(dam);
    let height = dam_height(&sim, above, dam);
    sim.apply(Intervention::ChangeTerrain {
        x: dx,
        y: dy,
        meters: height,
    });
    sim.advance_days(90);
    let stored = sim.world.tiles[above].surface_water;
    sim.apply(Intervention::ChangeTerrain {
        x: dx,
        y: dy,
        meters: -height - 2.0,
    });
    sim.advance_days(20);
    let left = sim.world.tiles[above].surface_water;
    assert!(
        left < stored * 0.5 || stored < POND_MM,
        "Stau läuft nach Durchbruch nicht ab: {stored:.0} → {left:.0} mm"
    );
}

#[test]
fn heavy_rain_floods_temporarily() {
    let mut sim = Simulation::new(7);
    sim.advance_years(10);
    sim.apply(Intervention::SetAnnualRain { mm: 0.0 });
    sim.advance_days(90);
    let dry = sim.snapshot().pond_tiles;

    sim.apply(Intervention::SetAnnualRain { mm: 5000.0 });
    let mut peak = 0;
    for _ in 0..30 {
        sim.advance_days(1);
        peak = peak.max(sim.snapshot().pond_tiles);
    }
    assert!(
        peak > dry + 10,
        "Starkregen überschwemmt nichts: {dry} → {peak} Felder"
    );

    sim.apply(Intervention::SetAnnualRain { mm: 0.0 });
    sim.advance_days(120);
    let after = sim.snapshot().pond_tiles;
    assert!(
        after < peak / 2,
        "Überschwemmung läuft nicht ab: {peak} → {after} Felder"
    );
}

#[test]
fn terrain_changes_are_limited_and_keep_land() {
    let mut sim = Simulation::new(3);
    let i = (0..sim.world.tiles.len())
        .find(|&i| !sim.world.tiles[i].is_ocean() && sim.world.tiles[i].elevation < 5.0)
        .unwrap();
    let (x, y) = sim.world.xy(i);
    sim.apply(Intervention::ChangeTerrain {
        x,
        y,
        meters: -100.0,
    });
    assert!(!sim.world.tiles[i].is_ocean(), "Graben erzeugt Meer");
    // Meer lässt sich nicht aufschütten
    let sea = (0..sim.world.tiles.len())
        .find(|&i| sim.world.tiles[i].is_ocean())
        .unwrap();
    let (sx, sy) = sim.world.xy(sea);
    let before = sim.world.tiles[sea].elevation;
    sim.apply(Intervention::ChangeTerrain {
        x: sx,
        y: sy,
        meters: 10.0,
    });
    assert_eq!(sim.world.tiles[sea].elevation, before);
}
