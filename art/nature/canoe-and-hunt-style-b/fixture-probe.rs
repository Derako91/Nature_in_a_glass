use nig_core::{Simulation,Intervention,appearance};
use nig_core::animals::Species;
fn main() {
    let mut sim=Simulation::with_size(5,128,128);
    sim.advance_years(10);
    sim.apply(Intervention::ReleaseAnimals{species:Species::Deer,at:None});
    sim.apply(Intervention::PeopleArrive{at:None});
    sim.advance_years(30);
    println!("world={}x{} people={}",sim.world.width,sim.world.height,sim.people.list.len());
    for day in 0..61 {
        if day>0 {sim.advance_days(1);}
        for p in appearance::person_sprites(&sim.people,&sim.world) {
            if p.tool==Some("boat") || day==0 {
                let (x,y)=sim.world.xy(p.tile as usize);
                let sea:Vec<_>=sim.world.neighbors(p.tile as usize).filter(|&j|sim.world.tiles[j].is_ocean()).map(|j|sim.world.xy(j)).collect();
                println!("day={day} id={} look={} activity={} tool={:?} tile={x},{y} sea={sea:?}",p.id,p.look,p.activity,p.tool);
            }
        }
    }
}
