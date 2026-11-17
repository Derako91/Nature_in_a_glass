// Regulären Hirsch-Prüfstand erzeugen; keine direkten Personen-/Kadaveränderungen.
use nig_core::{Simulation, Intervention, appearance};
use nig_core::animals::Species;
fn main() {
    let mut sim = Simulation::with_size(5, 128, 128);
    sim.advance_hours(10);
    sim.advance_years(10);
    sim.apply(Intervention::ReleaseAnimals { species: Species::Deer, at: None });
    sim.apply(Intervention::PeopleArrive { at: None });
    sim.advance_years(30);
    for _ in 0..5 {
        sim.apply(Intervention::ReleaseAnimals { species: Species::Deer, at: Some((44,77)) });
    }
    for day in 1..=365 {
        sim.advance_days(1);
        if let Some((tile,_)) = appearance::carcasses(&sim.animals).iter().find(|(_,id)| *id=="deer_hunted") {
            println!("deer extra_days={day} tile={:?}", sim.world.xy(*tile as usize));
            let output = std::env::args().nth(1).unwrap_or("/tmp/canoe22-deer-save.json".into());
            std::fs::write(output, sim.to_json()).unwrap();
            break;
        }
    }
}
