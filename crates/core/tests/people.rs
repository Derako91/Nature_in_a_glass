//! v0.6: Menschen – Ankunft, Bedürfnisse, Wissen, Biografie, Tod.

use nig_core::people::{Cause, PlaceKind};
use nig_core::{EventKind, Intervention, Simulation};

fn arrive(sim: &mut Simulation) {
    sim.apply(Intervention::PeopleArrive { at: None });
}

#[test]
fn new_island_has_no_people() {
    let mut sim = Simulation::new(42);
    sim.advance_years(3);
    assert_eq!(sim.people.alive(), 0);
}

#[test]
fn boat_brings_five_people_to_the_coast() {
    let mut sim = Simulation::new(42);
    sim.advance_years(10);
    arrive(&mut sim);
    assert_eq!(sim.people.alive(), 5);
    let tile = sim.people.list[0].tile as usize;
    assert!(
        sim.world
            .neighbors(tile)
            .any(|j| sim.world.tiles[j].is_ocean()),
        "Boot landet nicht an der Küste"
    );
    let ev = sim
        .journal
        .events
        .iter()
        .find(|e| e.kind == EventKind::Intervention && e.message.contains("Boot"))
        .expect("Ankunft in der Chronik");
    for p in &sim.people.list {
        assert!(ev.message.contains(&p.name), "{} fehlt", p.name);
        assert!(!p.bio.is_empty());
    }
    // Fünf verschiedene Aussehen, Frauen und Männer gemischt
    let mut looks: Vec<u8> = sim.people.list.iter().map(|p| p.look).collect();
    looks.sort();
    assert_eq!(looks, vec![0, 1, 2, 3, 4]);
}

/// Auf einer gewöhnlichen Insel überleben die meisten die ersten Jahre:
/// Sie finden Wasser, sammeln Nahrung und überstehen den Winter.
#[test]
fn people_find_water_and_survive_the_first_years() {
    let mut sim = Simulation::new(42);
    sim.advance_years(10);
    arrive(&mut sim);
    sim.advance_years(5);
    assert!(
        sim.people.alive() >= 3,
        "nur {} leben noch",
        sim.people.alive()
    );
    // Nur die mit dem Boot Gekommenen (Kinder können noch jung sein)
    for p in sim.people.list.iter().filter(|p| p.mother == 0) {
        assert!(
            p.memory.iter().any(|m| m.kind == PlaceKind::Water),
            "{} kennt kein Wasser",
            p.name
        );
        assert!(
            p.bio.iter().any(|b| b.text.contains("Winter")),
            "{}: kein Winter in der Biografie",
            p.name
        );
        assert!(p.hunger < 0.9 && p.thirst < 1.0);
    }
}

/// Ohne Regen und Quellen versiegt alles Wasser – dann verdursten sie.
/// Tote bleiben mit Biografie in Erinnerung und liegen einige Tage da.
#[test]
fn without_water_people_die_of_thirst() {
    let mut sim = Simulation::new(5);
    sim.advance_years(2);
    sim.apply(Intervention::SetAnnualRain { mm: 0.0 });
    for k in 0..sim.world.springs.len() {
        sim.apply(Intervention::SetSpringFactor {
            spring: k,
            factor: 0.0,
        });
    }
    sim.advance_years(3);
    arrive(&mut sim);
    let mut saw_body = false;
    for _ in 0..120 {
        sim.advance_days(1);
        saw_body |= !sim.people.bodies.is_empty();
        if sim.people.alive() == 0 {
            break;
        }
    }
    assert_eq!(sim.people.alive(), 0, "Menschen überleben ohne Wasser");
    assert!(saw_body);
    assert_eq!(sim.people.departed.len(), 5);
    assert!(
        sim.people.departed.iter().any(|d| d.cause == Cause::Thirst),
        "niemand verdurstet: {:?}",
        sim.people
            .departed
            .iter()
            .map(|d| d.cause)
            .collect::<Vec<_>>()
    );
    for d in &sim.people.departed {
        assert!(d.person.bio.last().unwrap().text.contains("Jahren"));
    }
    assert!(
        sim.journal
            .events
            .iter()
            .any(|e| e.kind == EventKind::PeopleGone)
    );
    assert!(EventKind::PersonDied.is_important());
    sim.advance_days(20);
    assert!(sim.people.bodies.is_empty(), "Tote verschwinden nicht");
}

#[test]
fn people_are_deterministic_and_survive_save_load() {
    let mut a = Simulation::new(7);
    a.advance_years(8);
    arrive(&mut a);
    a.advance_days(200);
    let json = a.to_json();
    let mut b = Simulation::from_json(&json).unwrap();
    a.advance_days(400);
    b.advance_days(400);
    assert_eq!(a.state_hash(), b.state_hash());
    assert_eq!(a.people, b.people);
}

/// v0.7: Paare finden sich, Kinder kommen zur Welt, wachsen bei der Mutter
/// auf und kennen ihre Familie.
#[test]
fn couples_form_and_children_are_born() {
    let mut sim = Simulation::new(42);
    sim.advance_years(10);
    arrive(&mut sim);
    sim.advance_years(12);
    let pe = &sim.people;
    assert!(pe.born >= 1, "kein Kind in 12 Jahren");
    let all: Vec<&nig_core::people::Person> = pe
        .list
        .iter()
        .chain(pe.departed.iter().map(|d| &d.person))
        .collect();
    let couples = all.iter().filter(|p| p.partner != 0).count()
        + pe.departed
            .iter()
            .filter(|d| !d.person.children.is_empty())
            .count();
    assert!(couples >= 1, "kein Paar");
    let child = all
        .iter()
        .find(|p| p.mother != 0)
        .expect("ein Kind mit Mutter");
    let mother = all.iter().find(|p| p.id == child.mother).unwrap();
    assert!(mother.children.contains(&child.id));
    assert!(child.bio[0].text.contains("geboren"));
    assert!(child.bio[0].text.contains(&mother.name));
    assert!(
        sim.journal
            .events
            .iter()
            .any(|e| e.kind == EventKind::FirstBorn)
    );
    // Kleinkinder sind bei einem Erwachsenen
    for c in pe.list.iter().filter(|p| p.is_small_child()) {
        assert!(
            pe.list.iter().any(|a| !a.is_child() && a.tile == c.tile),
            "{} ist allein",
            c.name
        );
    }
}

// ---------- v0.8: Lagerplätze ----------

fn settled(seed: u64, years: u64) -> Simulation {
    let mut sim = Simulation::new(seed);
    sim.advance_years(10);
    arrive(&mut sim);
    sim.advance_years(years);
    sim
}

/// Wer viele Nächte an verlässlichem Wasser verbringt, legt dort ein Lager
/// an. Sind die Menschen zahlreich genug, entstehen Hütten.
#[test]
fn people_settle_at_reliable_water_and_build_huts() {
    let sim = settled(2024, 5);
    let pe = &sim.people;
    assert!(!pe.camps.is_empty(), "kein Lagerplatz in 5 Jahren");
    for c in pe.camps.iter().filter(|c| !c.abandoned) {
        assert!(
            nig_core::people::drinkable(&sim.world, c.tile as usize),
            "Lager ohne Wasser"
        );
    }
    assert!(pe.huts_built >= 1, "keine Hütte");
    assert!(
        pe.list.iter().any(|p| p.camp != 0),
        "niemand gehört zum Lager"
    );
    let ev = &sim.journal.events;
    assert!(ev.iter().any(|e| e.kind == EventKind::CampFounded));
    assert!(ev.iter().any(|e| e.kind == EventKind::HutBuilt));
    // Lager liegen nicht dicht nebeneinander
    for a in pe.camps.iter().filter(|a| !a.abandoned) {
        for b in pe.camps.iter().filter(|b| b.id < a.id && !b.abandoned) {
            let (ax, ay) = sim.world.xy(a.tile as usize);
            let (bx, by) = sim.world.xy(b.tile as usize);
            let d = (ax as i32 - bx as i32)
                .abs()
                .max((ay as i32 - by as i32).abs());
            assert!(d >= nig_core::people::camps_spacing(), "Lager zu dicht");
        }
    }
}

/// Holz für Feuer und Hütten fehlt danach im Wald rund um das Lager.
#[test]
fn camp_wood_thins_the_forest_nearby() {
    let with = settled(2024, 12);
    let mut without = Simulation::new(2024);
    without.advance_years(22);
    let camp = with.people.camps.first().expect("ein Lager").tile as usize;
    let wood = |sim: &Simulation| -> f32 {
        let (cx, cy) = sim.world.xy(camp);
        let mut sum = 0.0;
        for y in cy.saturating_sub(3)..=(cy + 3).min(sim.world.height - 1) {
            for x in cx.saturating_sub(3)..=(cx + 3).min(sim.world.width - 1) {
                let t = sim.world.tile(x, y);
                sum += t.forest + t.deadwood;
            }
        }
        sum
    };
    assert!(
        wood(&with) < wood(&without) - 0.3,
        "mit Lager {:.2}, ohne {:.2}",
        wood(&with),
        wood(&without)
    );
}

/// Kommt niemand mehr, wird das Lager verlassen und verfällt.
#[test]
fn abandoned_camps_decay_and_disappear() {
    let mut sim = settled(2024, 3);
    assert!(!sim.people.camps.is_empty());
    // Alle verlassen die Insel (nur im Test)
    sim.people.list.clear();
    sim.advance_years(3);
    assert!(sim.people.camps.iter().all(|c| c.abandoned));
    assert!(
        sim.journal
            .events
            .iter()
            .any(|e| e.kind == EventKind::CampAbandoned)
    );
    sim.advance_years(3);
    assert!(sim.people.camps.is_empty(), "Lager verfällt nicht");
    assert!(
        sim.journal
            .events
            .iter()
            .any(|e| e.kind == EventKind::CampGone)
    );
}

#[test]
fn camps_survive_save_load() {
    let mut a = settled(2024, 2);
    assert!(!a.people.camps.is_empty());
    let mut b = Simulation::from_json(&a.to_json()).unwrap();
    assert_eq!(a.people.camps, b.people.camps);
    a.advance_days(300);
    b.advance_days(300);
    assert_eq!(a.state_hash(), b.state_hash());
}

// ---------- v0.9: Fischfang, Wasser wiederfinden ----------

/// Wer am Wasser lebt, lernt zu fischen; das Können geht an Kinder weiter.
#[test]
fn people_learn_to_fish_and_teach_children() {
    let sim = settled(5, 20);
    let pe = &sim.people;
    assert!(pe.first_fisher, "niemand ist geschickt im Fischen");
    assert!(
        sim.journal
            .events
            .iter()
            .any(|e| e.kind == EventKind::FirstFisher)
    );
    // Auf der Insel geborene Erwachsene haben das Fischen von anderen gelernt.
    let born_adults: Vec<_> = pe
        .list
        .iter()
        .filter(|p| p.mother != 0 && !p.is_child())
        .collect();
    assert!(
        born_adults.is_empty() || born_adults.iter().any(|p| p.fishing > 0.2),
        "Können geht nicht weiter"
    );
}

/// Ein ausgetrockneter Teich hält niemanden fest: Wer dort Durst hat, sucht
/// weiter und findet anderes Wasser.
#[test]
fn thirsty_people_leave_a_dried_up_water_place() {
    let mut sim = settled(5, 2);
    let w = &sim.world;
    // Eine Stelle ohne Wasser, die alle für eine sichere Wasserstelle halten
    let dry = (0..w.tiles.len())
        .find(|&i| {
            let t = &w.tiles[i];
            !t.is_ocean()
                && t.surface_water < 1.0
                && !nig_core::people::drinkable(w, i)
                && w.neighbors(i).all(|j| !w.tiles[j].is_ocean())
        })
        .unwrap() as u32;
    for p in &mut sim.people.list {
        p.tile = dry;
        p.thirst = 0.3;
        p.memory.push(nig_core::people::Place {
            tile: dry,
            kind: PlaceKind::Water,
            value: 1.0,
            seen_day: 0,
        });
    }
    sim.advance_days(1);
    assert!(
        sim.people.list.iter().filter(|p| !p.is_child()).all(|p| !p
            .memory
            .iter()
            .any(|m| m.tile == dry && m.kind == PlaceKind::Water)),
        "trockene Stelle nicht vergessen"
    );
}

// ---------- v0.10: Gärten ----------

/// Wer lange an einem Lager lebt, legt irgendwann einen Garten an; die Ernte
/// kommt in den Vorrat. Ohne Menschen verwildert er.
#[test]
fn gardens_grow_are_harvested_and_go_wild() {
    let mut sim = settled(300, 20);
    let pe = &sim.people;
    assert!(!pe.gardens.is_empty(), "kein Garten in 20 Jahren");
    assert!(pe.harvests >= 1, "keine Ernte");
    assert!(
        pe.list.iter().any(|p| p.farming > 0.2),
        "niemand kann gärtnern"
    );
    let ev = &sim.journal.events;
    assert!(ev.iter().any(|e| e.kind == EventKind::GardenPlanted));
    assert!(ev.iter().any(|e| e.kind == EventKind::Harvest));
    // Gärten liegen neben einem Lager
    for g in &pe.gardens {
        let c = pe.camps.iter().find(|c| c.id == g.camp).expect("Lager");
        let (gx, gy) = sim.world.xy(g.tile as usize);
        let (cx, cy) = sim.world.xy(c.tile as usize);
        assert!(
            (gx as i32 - cx as i32)
                .abs()
                .max((gy as i32 - cy as i32).abs())
                <= 2
        );
    }
    // Speichern und Laden
    let b = Simulation::from_json(&sim.to_json()).unwrap();
    assert_eq!(b.people.gardens, sim.people.gardens);
    // Alle verlassen die Insel: die Gärten verwildern
    sim.people.list.clear();
    sim.advance_years(4);
    assert!(sim.people.gardens.is_empty(), "Garten verwildert nicht");
    assert!(
        sim.journal
            .events
            .iter()
            .any(|e| e.kind == EventKind::GardenLost)
    );
}

/// Wer vor allem gefischt hat, erscheint mit der Tätigkeit „fischt“ (Bild `_fish`).
#[test]
fn fishing_shows_as_its_own_activity() {
    use nig_core::people::Activity;
    let mut sim = settled(5, 5);
    let mut seen = false;
    for _ in 0..365 {
        sim.advance_days(1);
        if sim
            .people
            .list
            .iter()
            .any(|p| !p.is_child() && p.activity == Activity::Fish)
        {
            seen = true;
            break;
        }
    }
    assert!(seen, "niemand fischt sichtbar");
    assert_eq!(Activity::Fish.id(), "fish");
}

// ---------- v0.11: Lager teilen sich ----------

/// Ist ein Lager zu eng, bricht ein junges Paar auf und legt anderswo ein
/// neues Lager an.
#[test]
fn crowded_camps_split_and_a_new_camp_appears() {
    use nig_core::people::Sex;
    let mut sim = settled(42, 40);
    let camp = sim
        .people
        .camps
        .iter()
        .find(|c| !c.abandoned)
        .expect("Lager")
        .clone();
    // Das Lager ist voll: vier Hütten und viele junge Paare (nur im Test)
    sim.people
        .camps
        .iter_mut()
        .find(|c| c.id == camp.id)
        .unwrap()
        .huts = 4;
    let model = sim
        .people
        .list
        .iter()
        .find(|p| !p.is_child() && p.camp == camp.id)
        .expect("Bewohner")
        .clone();
    let mut next = sim.people.list.iter().map(|p| p.id).max().unwrap() + 1000;
    for k in 0..12 {
        let mut p = model.clone();
        p.id = next;
        next += 1;
        p.name = format!("Test {k}");
        p.sex = if k % 2 == 0 { Sex::Female } else { Sex::Male };
        p.age_days = 25 * 365;
        p.partner = if k % 2 == 0 { next } else { next - 2 };
        p.mother = 0;
        p.father = 0;
        p.children.clear();
        p.hunger = 0.0;
        p.thirst = 0.0;
        p.health = 1.0;
        p.injury = 0.0;
        p.tile = camp.tile;
        sim.people.list.push(p);
    }
    sim.people.list.sort_by_key(|p| p.id);
    let before = sim.people.camps.iter().filter(|c| !c.abandoned).count();
    sim.advance_years(2);
    assert!(
        sim.journal
            .events
            .iter()
            .any(|e| e.kind == EventKind::CampSplit),
        "niemand bricht auf"
    );
    let after = sim.people.camps.iter().filter(|c| !c.abandoned).count();
    assert!(after > before, "kein neues Lager ({before} → {after})");
}

// ---------- v0.12: Dörfer ----------

/// Mehrere bewohnte Lager nah beieinander bilden ein Dorf mit Namen; die
/// Chronik meldet es. Ohne Menschen zerfällt es.
#[test]
fn camps_close_together_become_a_named_village() {
    let mut sim = settled(42, 90);
    let pe = &sim.people;
    assert!(!pe.villages.is_empty(), "kein Dorf");
    let v = &pe.villages[0];
    assert!(v.camps.len() >= 2);
    assert!(!v.name.is_empty());
    assert!(
        sim.journal
            .events
            .iter()
            .any(|e| e.kind == EventKind::VillageFounded && e.message.contains(&v.name))
    );
    for id in &v.camps {
        assert_eq!(
            nig_core::people::village_of(pe, *id).map(|x| x.id),
            Some(v.id)
        );
    }
    let b = Simulation::from_json(&sim.to_json()).unwrap();
    assert_eq!(b.people.villages, sim.people.villages);
    sim.people.list.clear();
    sim.advance_days(60);
    assert!(sim.people.villages.is_empty(), "Dorf ohne Menschen");
}

// ---------- v0.13: Umzug und Werkzeuge ----------

/// Fehlt einer kleinen Gruppe lange Holz (oder versiegt ihr Wasser), zieht
/// das ganze Lager um: Auf Insel 99 gibt es um das erste Lager kaum Wald.
#[test]
fn a_camp_without_wood_moves_away() {
    let sim = settled(99, 2);
    let ev = sim
        .journal
        .events
        .iter()
        .find(|e| e.kind == EventKind::CampMoved)
        .expect("Lager zieht nicht um");
    assert!(ev.message.contains("aufgegeben"));
    assert!(sim.people.camps.iter().any(|c| c.abandoned));
    // Am Ziel entsteht ein neues Lager
    assert!(sim.people.camps.iter().any(|c| !c.abandoned));
}

/// Werkzeuge entstehen aus dem Tun und gehen auf andere über.
#[test]
fn tools_are_invented_and_spread() {
    use nig_core::people::TOOLS;
    let sim = settled(5, 30);
    let ev = &sim.journal.events;
    assert!(
        ev.iter().any(|e| e.kind == EventKind::ToolInvented),
        "nichts erfunden"
    );
    let best = TOOLS
        .iter()
        .map(|t| sim.people.list.iter().filter(|p| p.knows(*t)).count())
        .max()
        .unwrap_or(0);
    assert!(best >= 2, "Wissen geht nicht weiter");
}

// ---------- v0.14: Jagd ----------

/// Menschen jagen Hirsche und Kaninchen. (Nach 30 Jahren gibt es mit Jägern
/// oft sogar mehr Hirsche: Ohne Jagd vermehren sie sich, bis viele
/// verhungern. Deshalb prüft der Test nur, dass gejagt wird.)
#[test]
fn people_hunt_deer_and_rabbits() {
    use nig_core::animals::Species;
    let mut with = Simulation::new(2024);
    with.advance_years(5);
    with.apply(Intervention::ReleaseAnimals {
        species: Species::Deer,
        at: None,
    });
    with.apply(Intervention::ReleaseAnimals {
        species: Species::Rabbit,
        at: None,
    });
    with.advance_years(5);
    arrive(&mut with);
    with.advance_years(30);
    assert!(
        with.journal
            .events
            .iter()
            .any(|e| e.kind == EventKind::FirstHunt),
        "keine Jagd"
    );
    assert!(with.animals.hunted[Species::Deer.index()] > 0);
    assert!(with.animals.hunted[Species::Rabbit.index()] > 0);
    assert!(with.people.list.iter().any(|p| p.hunting > 0.3));
}

// ---------- v0.15: Tongefäß und Rollen ----------

/// Wer etwas am besten kann, gilt im Lager als Fischerin, Gärtner … –
/// höchstens eine Person je Rolle und Lager, jede mit höchstens einer Rolle.
#[test]
fn roles_follow_skill_and_pots_are_invented() {
    use nig_core::people::Tool;
    let sim = settled(5, 40);
    let pe = &sim.people;
    let with_role: Vec<_> = pe.list.iter().filter(|p| p.role().is_some()).collect();
    assert!(!with_role.is_empty(), "keine Rollen");
    for p in &with_role {
        assert!(!p.is_child());
        assert!(p.role_label().is_some());
        // Niemand im selben Lager hat dieselbe Rolle.
        assert_eq!(
            pe.list
                .iter()
                .filter(|q| q.camp == p.camp && q.role() == p.role())
                .count(),
            1
        );
    }
    assert!(
        sim.journal
            .events
            .iter()
            .any(|e| e.kind == EventKind::ToolInvented && e.message.contains("Tongefäß")),
        "kein Tongefäß in 40 Jahren"
    );
    assert!(pe.list.iter().any(|p| p.knows(Tool::Pot)));
}

// ---------- v0.16: Einbaum ----------

/// Wer geschickt fischt und Netz und Beil kennt, baut irgendwann einen
/// Einbaum; das Wissen geht weiter.
#[test]
fn skilled_fishers_build_a_dugout() {
    use nig_core::people::Tool;
    let sim = settled(7, 40);
    assert!(
        sim.journal
            .events
            .iter()
            .any(|e| e.kind == EventKind::ToolInvented && e.message.contains("Einbaum")),
        "kein Einbaum in 40 Jahren"
    );
    assert!(
        sim.people
            .list
            .iter()
            .filter(|p| p.knows(Tool::Boat))
            .count()
            >= 2
    );
}

// ---------- v0.20: Überfahrten ----------

/// Wer einen Einbaum bauen kann, sieht von der Küste andere Inseln und fährt
/// irgendwann mit Partner hinüber – auf einer einzelnen Insel nie.
#[test]
fn boat_people_cross_to_another_island() {
    use nig_core::people::Sex;
    use nig_core::world::Shape;
    let mut sim = Simulation::with_shape(2024, 160, 160, Shape::Archipelago);
    sim.advance_years(2);
    // Küstenfeld der größten Insel, das einer anderen Insel am nächsten liegt
    let w = &sim.world;
    let home = w.islands.iter().max_by_key(|i| i.tiles).unwrap().id;
    let other: Vec<(i32, i32)> = (0..w.tiles.len())
        .filter(|&i| {
            !w.tiles[i].is_ocean()
                && w.island_at(i)
                    .is_some_and(|x| x.id != home && x.tiles >= 40)
        })
        .map(|i| {
            let (x, y) = w.xy(i);
            (x as i32, y as i32)
        })
        .collect();
    let coast = (0..w.tiles.len())
        .filter(|&i| {
            !w.tiles[i].is_ocean()
                && w.tiles[i].surface_water <= 5.0
                && w.island_at(i).map(|x| x.id) == Some(home)
                && w.neighbors(i).any(|j| w.tiles[j].is_ocean())
        })
        .min_by_key(|&i| {
            let (x, y) = w.xy(i);
            other
                .iter()
                .map(|o| (o.0 - x as i32).abs().max((o.1 - y as i32).abs()))
                .min()
                .unwrap()
        })
        .unwrap();
    let (x, y) = w.xy(coast);
    for _ in 0..2 {
        sim.apply(Intervention::PeopleArrive { at: Some((x, y)) });
    }
    // Alle können Einbäume bauen; Frauen und Männer sind Paare.
    let women: Vec<u32> = sim
        .people
        .list
        .iter()
        .filter(|p| p.sex == Sex::Female)
        .map(|p| p.id)
        .collect();
    let men: Vec<u32> = sim
        .people
        .list
        .iter()
        .filter(|p| p.sex == Sex::Male)
        .map(|p| p.id)
        .collect();
    for (w, m) in women.iter().zip(&men) {
        for p in &mut sim.people.list {
            if p.id == *w {
                p.partner = *m;
            } else if p.id == *m {
                p.partner = *w;
            }
        }
    }
    for p in &mut sim.people.list {
        p.tools = 31;
        p.fishing = 0.8;
        p.age_days = p.age_days.min(30 * 365);
    }
    let mut crossed = false;
    for _ in 0..10 {
        sim.advance_years(1);
        if sim.people.crossings > 0 {
            crossed = true;
            break;
        }
    }
    assert!(crossed, "niemand ist in zehn Jahren übergesetzt");
    let ev = sim
        .journal
        .events
        .iter()
        .find(|e| e.kind == EventKind::Crossing)
        .expect("keine Überfahrt in der Chronik");
    assert!(ev.message.contains("Einbaum"));
    assert!(sim.people.list.iter().any(|p| p.seen != 0));
    // Wer übergesetzt ist, steht auf einer anderen Insel.
    assert!(
        sim.people.list.iter().any(|p| sim
            .world
            .island_at(p.tile as usize)
            .is_some_and(|i| i.id != home))
            || !sim.people.departed.is_empty()
    );
}

/// Auf einer einzelnen Insel gibt es nichts zu sehen und keine Überfahrt.
#[test]
fn no_crossings_on_a_single_island() {
    let sim = settled(7, 40);
    assert_eq!(sim.people.crossings, 0);
    assert!(sim.people.list.iter().all(|p| p.seen == 0));
}

// ---------- v0.21: angekündigtes Boot ----------

/// Ein angekündigtes Boot landet genau zur geplanten Zeit von selbst – auch
/// nach Speichern und Laden; in der Inselwelt an der größten Insel.
#[test]
fn announced_boat_lands_on_time() {
    use nig_core::world::Shape;
    let mut sim = Simulation::new(2024);
    sim.plan_boat(2);
    let at = sim.boat_at.unwrap();
    sim.advance_years(1);
    assert_eq!(sim.people.alive(), 0);
    let mut sim = Simulation::from_json(&sim.to_json()).unwrap();
    assert_eq!(sim.boat_at, Some(at));
    sim.skip_to_boat();
    assert_eq!(sim.time_hours, at);
    assert_eq!(sim.people.alive(), 5);
    assert_eq!(sim.boat_at, None);
    assert!(
        sim.journal
            .events
            .iter()
            .any(|e| e.message.contains("Ein Boot erreicht die Küste"))
    );

    let mut arch = Simulation::with_shape(2024, 160, 160, Shape::Archipelago);
    arch.plan_boat(0);
    arch.advance_days(1);
    let big = arch
        .world
        .islands
        .iter()
        .max_by_key(|i| i.tiles)
        .unwrap()
        .id;
    assert!(
        arch.people
            .list
            .iter()
            .all(|p| arch.world.island_at(p.tile as usize).map(|i| i.id) == Some(big))
    );
}

// ---------- v0.22: Darstellung von Jagd und Einbaum ----------

/// Wer jagt, wird als Jäger gezeigt; erlegte Hirsche sind als solche zu
/// erkennen; Einbaum-Fischer bekommen ein Meerfeld.
#[test]
fn hunters_boats_and_hunted_deer_are_shown() {
    use nig_core::animals::Species;
    use nig_core::appearance::{carcasses, person_sprites};
    let mut sim = Simulation::new(2024);
    sim.advance_years(5);
    for species in [Species::Deer, Species::Rabbit] {
        sim.apply(Intervention::ReleaseAnimals { species, at: None });
    }
    sim.advance_years(5);
    arrive(&mut sim);
    let (mut hunt, mut deer) = (false, false);
    for _ in 0..40 * 365 {
        sim.advance_days(1);
        hunt |= person_sprites(&sim.people, &sim.world)
            .iter()
            .any(|p| p.activity == "hunt");
        deer |= carcasses(&sim.animals).iter().any(|c| c.1 == "deer_hunted");
        if hunt && deer {
            break;
        }
    }
    assert!(hunt, "niemand wird als Jäger gezeigt");
    assert!(deer, "kein erlegter Hirsch zu sehen");
    // Einbaum: Meerfeld neben der Küste, nur für Boot-Fischer
    for p in person_sprites(&sim.people, &sim.world) {
        if let Some(b) = p.boat_tile {
            assert_eq!(p.tool, Some("boat"));
            assert!(sim.world.tiles[b as usize].is_ocean());
        }
    }
}
