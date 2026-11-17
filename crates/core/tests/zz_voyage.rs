//! Messung (ignoriert): Überfahrten in der Inselwelt.
use nig_core::events::EventKind;
use nig_core::people::Tool;
use nig_core::sim::{Intervention, Simulation};
use nig_core::world::Shape;

#[test]
#[ignore]
fn zz_voyage_probe() {
    let seeds: Vec<u64> = std::env::var("SEEDS")
        .unwrap_or("2024".into())
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    let years: u64 = std::env::var("YEARS")
        .ok()
        .and_then(|y| y.parse().ok())
        .unwrap_or(60);
    for seed in seeds {
        let mut sim = Simulation::with_shape(seed, 160, 160, Shape::Archipelago);
        sim.advance_years(10);
        // Auf der größten Insel an der Küste landen
        let big = sim.world.islands.iter().max_by_key(|i| i.tiles).unwrap().id;
        let w = &sim.world;
        let coast = (0..w.tiles.len())
            .find(|&i| {
                !w.tiles[i].is_ocean()
                    && w.tiles[i].surface_water <= 5.0
                    && w.island_at(i).map(|x| x.id) == Some(big)
                    && w.neighbors(i).any(|j| w.tiles[j].is_ocean())
            })
            .unwrap();
        let (x, y) = w.xy(coast);
        sim.apply(Intervention::PeopleArrive { at: Some((x, y)) });
        for decade in 0..years / 10 {
            sim.advance_years(10);
            let pe = &sim.people;
            let mut per: Vec<(String, usize)> = Vec::new();
            for p in &pe.list {
                let n = sim
                    .world
                    .island_at(p.tile as usize)
                    .map(|i| i.name.clone())
                    .unwrap_or("?".into());
                match per.iter_mut().find(|e| e.0 == n) {
                    Some(e) => e.1 += 1,
                    None => per.push((n, 1)),
                }
            }
            println!(
                "seed {seed} jahr {}: leben {} boot {} gesehen {} ueberfahrten {} inseln {:?}",
                (decade + 1) * 10,
                pe.alive(),
                pe.list.iter().filter(|p| p.knows(Tool::Boat)).count(),
                pe.list.iter().filter(|p| p.seen != 0).count(),
                pe.crossings,
                per
            );
        }
        for e in sim
            .journal
            .events
            .iter()
            .filter(|e| e.kind == EventKind::Crossing)
        {
            println!("  {}", e.message);
        }
    }
}

#[test]
#[ignore]
fn zz_island_gaps() {
    for seed in [1u64, 2, 3, 4, 5, 6, 7, 2024] {
        let sim = Simulation::with_shape(seed, 160, 160, Shape::Archipelago);
        let w = &sim.world;
        let big: Vec<_> = w.islands.iter().filter(|i| i.tiles >= 40).collect();
        let land: Vec<(u32, i32, i32)> = (0..w.tiles.len())
            .filter(|&i| !w.tiles[i].is_ocean())
            .filter_map(|i| {
                let id = w.island_at(i)?.id;
                let (x, y) = w.xy(i);
                Some((id, x as i32, y as i32))
            })
            .collect();
        let mut out = Vec::new();
        for a in &big {
            // Abstand zur nächsten anderen großen Insel
            let mut best = i32::MAX;
            for p in land.iter().filter(|l| l.0 == a.id) {
                for q in land
                    .iter()
                    .filter(|l| l.0 != a.id && big.iter().any(|b| b.id == l.0))
                {
                    best = best.min((p.1 - q.1).abs().max((p.2 - q.2).abs()));
                }
            }
            out.push(format!("{}({}):{}", a.name, a.tiles, best));
        }
        println!("seed {seed}: {}", out.join("  "));
    }
}
