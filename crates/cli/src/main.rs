//! `nig-sim` – Nature in a Glass ohne Fenster.
//!
//! Beispiele:
//!   nig-sim --seed 42 --years 100
//!   nig-sim --seed 42 --years 100 --map --save welt.json
//!   nig-sim --load welt.json --years 10

use std::process::ExitCode;
use std::time::Instant;

use nig_core::world::POND_MM;
use nig_core::{Simulation, Terrain};

struct Args {
    seed: u64,
    years: u64,
    map: bool,
    events: bool,
    save: Option<String>,
    load: Option<String>,
}

fn parse() -> Result<Args, String> {
    let mut a = Args { seed: 42, years: 10, map: false, events: false, save: None, load: None };
    let mut it = std::env::args().skip(1);
    while let Some(arg) = it.next() {
        let mut val = |name: &str| it.next().ok_or(format!("{name} braucht einen Wert"));
        match arg.as_str() {
            "--seed" => a.seed = val("--seed")?.parse().map_err(|_| "Seed muss eine Zahl sein")?,
            "--years" => a.years = val("--years")?.parse().map_err(|_| "Jahre müssen eine Zahl sein")?,
            "--map" => a.map = true,
            "--events" => a.events = true,
            "--save" => a.save = Some(val("--save")?),
            "--load" => a.load = Some(val("--load")?),
            "-h" | "--help" => {
                println!(
                    "nig-sim [--seed N] [--years N] [--map] [--events] [--save DATEI] [--load DATEI]"
                );
                std::process::exit(0);
            }
            other => return Err(format!("Unbekannte Option: {other}")),
        }
    }
    Ok(a)
}

fn print_map(sim: &Simulation) {
    let w = &sim.world;
    for y in 0..w.height {
        let mut line = String::new();
        for x in 0..w.width {
            let i = w.idx(x, y);
            let t = &w.tiles[i];
            let c = match w.terrain(i) {
                Terrain::DeepOcean => '~',
                Terrain::ShallowOcean => '-',
                _ if w.springs.iter().any(|s| s.x == x && s.y == y) => 'Q',
                _ if t.surface_water > POND_MM => 'O',
                _ if t.flow > nig_core::sim::STREAM_FLOW => '=',
                Terrain::Beach => '.',
                _ if t.forest > 0.5 => 'T',
                _ if t.forest > 0.15 => 't',
                Terrain::Mountain => '^',
                _ if t.grass > 0.5 => '"',
                _ if t.grass > 0.15 => ',',
                _ => ':',
            };
            line.push(c);
            line.push(c);
        }
        println!("{line}");
    }
    println!("Legende: ~ Tiefsee  - Flachwasser  . Strand  \" Gras  , lichtes Gras  : karg");
    println!("         t lichter Wald  T Wald  ^ Fels  = Bach  O Teich/See  Q Quelle");
}

fn main() -> ExitCode {
    let args = match parse() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("{e}");
            return ExitCode::FAILURE;
        }
    };

    let mut sim = match &args.load {
        Some(path) => {
            let json = match std::fs::read_to_string(path) {
                Ok(j) => j,
                Err(e) => {
                    eprintln!("Kann {path} nicht lesen: {e}");
                    return ExitCode::FAILURE;
                }
            };
            match Simulation::from_json(&json) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("{e}");
                    return ExitCode::FAILURE;
                }
            }
        }
        None => Simulation::new(args.seed),
    };

    println!(
        "Insel {} · {} Landfelder · höchster Punkt {:.0} m · Start in Jahr {}",
        sim.world.seed,
        sim.world.land_count(),
        sim.world.max_elevation,
        sim.year()
    );
    println!(
        "{:>5} {:>7} {:>7} {:>6} {:>7} {:>6} {:>5} {:>8} {:>6} {:>6}",
        "Jahr", "Regen", "Wald%", "Gras%", "Feucht%", "Bäche", "Seen", "Grundw%", "Unwet", "Dürre"
    );

    let start = Instant::now();
    let first = sim.history.len();
    sim.advance_years(args.years);
    let elapsed = start.elapsed();

    for s in &sim.history[first..] {
        println!(
            "{:>5} {:>7.0} {:>7.1} {:>6.1} {:>7.1} {:>6} {:>5} {:>8.0} {:>6} {:>6}",
            s.year,
            s.rain_mm,
            s.forest_pct,
            s.grass_pct,
            s.moisture_pct,
            s.stream_tiles,
            s.pond_tiles,
            s.aquifer_pct,
            s.storms,
            s.drought_days
        );
    }

    if args.events {
        println!("\nChronik (ohne Jahresbilanzen):");
        for e in sim.journal.events.iter().filter(|e| e.kind != nig_core::EventKind::YearSummary) {
            let day = e.time_hours / 24;
            println!(
                "  Jahr {:>4} Tag {:>3}  {}  [{}]",
                day / 365 + 1,
                day % 365 + 1,
                e.message,
                e.causes.join("; ")
            );
        }
    }

    if args.map {
        println!();
        print_map(&sim);
    }

    println!(
        "\n{} Jahre in {:.2} s simuliert · Prüfsumme {:016x}",
        args.years,
        elapsed.as_secs_f64(),
        sim.state_hash()
    );

    if let Some(path) = &args.save {
        if let Err(e) = std::fs::write(path, sim.to_json()) {
            eprintln!("Kann {path} nicht schreiben: {e}");
            return ExitCode::FAILURE;
        }
        println!("Gespeichert: {path}");
    }
    ExitCode::SUCCESS
}
