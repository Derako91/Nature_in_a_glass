//! Menschen (v0.6): die ersten Bewohner der Insel.
//!
//! Menschen kommen nur durch den Beobachter auf die Insel (ein Boot landet).
//! Danach handelt jeder für sich, angetrieben von seinen Bedürfnissen
//! (Prinzip 5): Durst, Hunger, Müdigkeit, Kälte. Niemand wird gesteuert.
//!
//! Ein Aufruf entspricht einem Tag:
//! - Jeder schaut sich um (Sichtweite) und merkt sich Wasser- und Futterstellen.
//!   Wissen ist persönlich (Prinzip 6): Wer eine Quelle nie gesehen hat, kennt
//!   sie nicht. Erinnerte Stellen können inzwischen versiegt sein.
//! - Das dringendste Bedürfnis bestimmt das Ziel: trinken, sammeln oder bei
//!   den anderen bleiben. Der Weg meidet steile Stufen – wer sie trotzdem
//!   hinabsteigt, kann stürzen.
//! - Nahrung: Beeren im Busch (Sommer), Nüsse und Pilze im Wald (Herbst),
//!   Wurzeln und Samen im Gras, Muscheln und Fisch an der Küste, Fleisch von
//!   Kadavern. Abgesammelte Stellen erholen sich langsam.
//! - Gefahren: Verdursten, Verhungern, Kälte ohne Wald als Schutz, Stürze,
//!   hungrige Wölfe bei Menschen allein, Feuer, Alter.
//! - Jeder hat eine Lebensgeschichte (Biografie). Tote bleiben in Erinnerung
//!   (Prinzip 3) und liegen einige Tage dort, wo sie gestorben sind.
//!
//! Nachwuchs gibt es in v0.6 noch nicht.

use std::cmp::Reverse;
use std::collections::BinaryHeap;

use serde::{Deserialize, Serialize};

use crate::animals::{Animals, Species};
use crate::climate::Weather;
use crate::math::clamp01;
use crate::rng::Rng;
use crate::sim::RILL_FLOW;
use crate::world::{POND_MM, World};

/// Menschen je Boot.
pub const BOAT_SIZE: usize = 5;
/// So weit sieht ein Mensch (Felder).
pub const VISION: i32 = 6;
/// Seen und Teiche sieht man von weiter weg.
pub const VISION_LAKE: i32 = 12;
/// Wegbudget eines gesunden Menschen an einem Tag (1 = ein Feld ebenaus).
pub const DAY_BUDGET: f32 = 16.0;
/// Tagesbedarf: eine Ration. Hunger steigt ohne Essen um so viel am Tag.
const HUNGER_PER_DAY: f32 = 0.035;
/// Wie stark eine gegessene Ration den Hunger senkt.
const HUNGER_PER_RATION: f32 = 0.06;
/// Ohne Trinken ist man nach knapp drei Tagen am Ende.
const THIRST_PER_DAY: f32 = 0.25;
/// Höchstens so viele Rationen trägt man bei sich.
pub const PACK_MAX: f32 = 3.0;
/// So viel Ausbeute erholt sich je Tag (Anteil).
const REGROW: f32 = 0.03;
/// Höhenunterschied, ab dem ein Schritt nicht mehr möglich ist (m).
const STEP_MAX_M: f32 = 25.0;
/// Ab diesem Abstieg kann man stürzen (m).
const FALL_FROM_M: f32 = 8.0;
/// Gedächtnis: so viele Orte merkt sich ein Mensch.
const MEMORY: usize = 16;
/// Einträge in einer Biografie (die ersten bleiben, danach die neuesten).
const BIO_MAX: usize = 300;
/// Bis zu diesem Alter wird ein Kind getragen und versorgt (Tage).
pub const SMALL_CHILD_DAYS: u32 = 8 * 365;
/// Ab diesem Alter ist man erwachsen (Tage).
pub const ADULT_DAYS: u32 = 14 * 365;
/// Dauer einer Schwangerschaft (Tage).
pub const PREGNANCY_DAYS: u16 = 270;
/// Wahrscheinlichkeit je Tag, dass sich zwei, die viel beieinander sind, als Paar finden.
const PAIR_CHANCE: f32 = 0.004;
/// Wahrscheinlichkeit je Tag für eine Empfängnis bei gut genährter Frau.
const CONCEIVE_CHANCE: f32 = 0.012;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Sex {
    Female,
    Male,
}

/// Was ein Mensch heute vor allem getan hat (bestimmt das Bild, 17.4).
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Activity {
    Idle,
    Walk,
    Run,
    Gather,
    Eat,
    Drink,
    Sleep,
    Hurt,
}

impl Activity {
    pub fn id(self) -> &'static str {
        match self {
            Activity::Idle => "idle",
            Activity::Walk => "walk",
            Activity::Run => "run",
            Activity::Gather => "gather",
            Activity::Eat => "eat",
            Activity::Drink => "drink",
            Activity::Sleep => "sleep",
            Activity::Hurt => "hurt",
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            Activity::Idle => "ruht sich aus",
            Activity::Walk => "ist unterwegs",
            Activity::Run => "flieht",
            Activity::Gather => "sammelt Nahrung",
            Activity::Eat => "isst",
            Activity::Drink => "trinkt",
            Activity::Sleep => "schläft",
            Activity::Hurt => "schleppt sich verletzt dahin",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Cause {
    Thirst,
    Hunger,
    Cold,
    Fall,
    Wolves,
    Fire,
    Age,
    Illness,
    Childbirth,
}

impl Cause {
    pub fn label(self) -> &'static str {
        match self {
            Cause::Thirst => "verdurstet",
            Cause::Hunger => "verhungert",
            Cause::Cold => "erfroren",
            Cause::Fall => "an den Folgen eines Sturzes gestorben",
            Cause::Wolves => "von Wölfen getötet worden",
            Cause::Fire => "im Feuer umgekommen",
            Cause::Age => "an Altersschwäche gestorben",
            Cause::Illness => "an einer Krankheit gestorben",
            Cause::Childbirth => "bei einer Geburt gestorben",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PlaceKind {
    Water,
    Food,
    /// Küste: Muscheln und Fisch, das ganze Jahr verlässlich.
    Shore,
}

/// Ein Ort, den ein Mensch kennt.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Place {
    pub tile: u32,
    pub kind: PlaceKind,
    /// Erwartete Ausbeute (Rationen) bei Futter, 1 bei Wasser.
    pub value: f32,
    /// Tag, an dem der Ort zuletzt gesehen wurde.
    pub seen_day: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BioEntry {
    pub time_hours: u64,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Person {
    pub id: u32,
    pub name: String,
    pub sex: Sex,
    /// Aussehen 0–4 (gestaltung.md 17.4), fest.
    pub look: u8,
    pub age_days: u32,
    pub tile: u32,
    pub prev_tile: u32,
    /// 0 = satt, 1 = verhungert.
    pub hunger: f32,
    /// 0 = getrunken, 1 = verdurstet.
    pub thirst: f32,
    /// 0 = ausgeruht, 1 = erschöpft.
    pub fatigue: f32,
    /// 1 = gesund, 0 = tot.
    pub health: f32,
    /// Schwere einer Verletzung 0..1 (heilt langsam).
    pub injury: f32,
    /// Mitgetragene Nahrung (Rationen).
    pub pack: f32,
    pub activity: Activity,
    pub facing_right: bool,
    pub memory: Vec<Place>,
    pub bio: Vec<BioEntry>,
    /// Ankunft auf der Insel.
    pub arrived_hours: u64,
    /// Merker, damit die Biografie nicht jeden Tag dasselbe meldet.
    flags: Flags,
    /// Was heute am meisten geschadet hat (Todesursache).
    harm: Option<Cause>,
    // --- Familie, seit Format 8 ---
    /// Eltern (0 = unbekannt, z. B. mit dem Boot gekommen).
    #[serde(default)]
    pub mother: u32,
    #[serde(default)]
    pub father: u32,
    /// Lebenspartner (0 = keiner).
    #[serde(default)]
    pub partner: u32,
    #[serde(default)]
    pub children: Vec<u32>,
    /// Tage der Schwangerschaft (0 = nicht schwanger).
    #[serde(default)]
    pub pregnant_days: u16,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
struct Flags {
    hungry: bool,
    thirsty: bool,
    cold_year: u64,
    winters: u32,
    knows_water: bool,
    /// Wo und wann man sich zuletzt gründlich umgesehen hat.
    #[serde(default)]
    scan_tile: u32,
    #[serde(default)]
    scan_day: u64,
}

impl Person {
    pub fn age_years(&self) -> u32 {
        self.age_days / 365
    }
    /// Kleinkind: wird getragen und versorgt.
    pub fn is_small_child(&self) -> bool {
        self.age_days < SMALL_CHILD_DAYS
    }
    /// Kind (noch nicht erwachsen).
    pub fn is_child(&self) -> bool {
        self.age_days < ADULT_DAYS
    }
    /// Alter in Worten (Jahre, bei Säuglingen Monate).
    pub fn age_text(&self) -> String {
        match self.age_days {
            d if d < 60 => format!("{} Tagen", d.max(1)),
            d if d < 365 => format!("{} Monaten", d / 30),
            d => format!("{} Jahren", d / 365),
        }
    }
    /// Wie geschwächt die Person wirkt (0..1, für die Darstellung).
    pub fn weakness(&self) -> f32 {
        clamp01(
            (1.0 - self.health)
                .max(self.hunger - 0.3)
                .max(self.thirst - 0.3),
        )
    }
    fn note(&mut self, now: u64, text: String) {
        if self.bio.len() >= BIO_MAX {
            // Die ersten Einträge (Ankunft) bleiben, ältere mittlere fallen weg.
            self.bio.remove(10);
        }
        self.bio.push(BioEntry {
            time_hours: now,
            text,
        });
    }
}

/// Ein Toter, an den man sich erinnert.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Departed {
    pub person: Person,
    pub cause: Cause,
    pub died_hours: u64,
}

/// Leichnam auf einem Feld (verschwindet nach einigen Tagen).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Body {
    pub tile: u32,
    pub look: u8,
    pub facing_right: bool,
    pub days_left: u16,
}

/// Was an einem Tag in der Chronik erscheinen soll.
#[derive(Debug, Clone)]
pub struct PeopleNews {
    pub kind: NewsKind,
    pub tile: u32,
    pub text: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NewsKind {
    Injured,
    Died,
    AllGone,
    Born,
    /// Das erste Kind, das auf der Insel geboren wird.
    FirstBorn,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct People {
    /// Lebende, nach `id` sortiert.
    pub list: Vec<Person>,
    pub departed: Vec<Departed>,
    pub bodies: Vec<Body>,
    /// Abgesammelter Anteil je Feld (0..1), erholt sich langsam.
    pub taken: Vec<f32>,
    next_id: u32,
    /// Wie viele Boote schon angekommen sind.
    pub boats: u32,
    /// Auf der Insel geborene Kinder (insgesamt).
    #[serde(default)]
    pub born: u32,
}

const FEMALE_NAMES: [&str; 30] = [
    "Mira", "Ilka", "Runa", "Tala", "Edda", "Liv", "Asta", "Neva", "Saja", "Yrsa", "Ama", "Wenna",
    "Brin", "Kaja", "Lone", "Maren", "Nila", "Oda", "Rika", "Siv", "Tove", "Una", "Veda", "Alva",
    "Dagny", "Elin", "Frida", "Greta", "Hilde", "Isa",
];
const MALE_NAMES: [&str; 30] = [
    "Tovo", "Aren", "Bero", "Kael", "Odo", "Jorn", "Sten", "Ulf", "Halm", "Ivo", "Rasko", "Fenn",
    "Arvid", "Bjarn", "Dag", "Eik", "Falk", "Gorm", "Hakon", "Ingo", "Joki", "Knut", "Lasse",
    "Mads", "Nils", "Ole", "Pelle", "Ragn", "Sune", "Torben",
];
/// Ein Name, den noch niemand trägt oder getragen hat; notfalls mit Zählung.
fn pick_name(sex: Sex, used: &[String], rng: &mut Rng) -> String {
    let pool: &[&str] = match sex {
        Sex::Female => &FEMALE_NAMES,
        Sex::Male => &MALE_NAMES,
    };
    let free: Vec<&str> = pool
        .iter()
        .copied()
        .filter(|n| !used.iter().any(|u| u == n))
        .collect();
    if !free.is_empty() {
        return free[rng.below(free.len() as u32) as usize].to_string();
    }
    let base = pool[rng.below(pool.len() as u32) as usize];
    for k in 2.. {
        let n = format!("{base} {}", roman(k));
        if !used.contains(&n) {
            return n;
        }
    }
    unreachable!()
}

fn roman(mut n: u32) -> String {
    let table = [
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ];
    let mut out = String::new();
    for (v, r) in table {
        while n >= v {
            out.push_str(r);
            n -= v;
        }
    }
    out
}

/// Geschlecht je Aussehen (17.4): 0, 2, 4 Frauen; 1, 3 Männer.
pub fn look_sex(look: u8) -> Sex {
    if look.is_multiple_of(2) {
        Sex::Female
    } else {
        Sex::Male
    }
}

/// Darf man von `a` nach `b` gehen? Gibt die Kosten zurück.
fn edge_cost(world: &World, a: usize, b: usize) -> Option<f32> {
    let tb = &world.tiles[b];
    if tb.is_ocean() || tb.surface_water > POND_MM || tb.fire > 0.0 {
        return None;
    }
    let dh = tb.elevation - world.tiles[a].elevation;
    if dh.abs() > STEP_MAX_M {
        return None;
    }
    let (ax, ay) = world.xy(a);
    let (bx, by) = world.xy(b);
    let base = if ax != bx && ay != by { 1.4 } else { 1.0 };
    // Steile Stufen sind mühsam (bergauf) oder gefährlich (bergab).
    let steep = (dh.abs() - 3.0).max(0.0) * 0.12;
    let danger = (-dh - FALL_FROM_M).max(0.0) * 0.5;
    Some(base + steep + danger)
}

/// Erreichbarkeit: Kosten und Vorgänger je Feld (Dijkstra, feste Reihenfolge).
#[derive(Debug, Clone, Default)]
pub struct Scratch {
    cost: Vec<f32>,
    parent: Vec<u32>,
    touched: Vec<u32>,
    /// Tageswerte je Feld, einmal am Tag für alle berechnet.
    info: Vec<TileInfo>,
    /// Wegkosten zu den Nachbarn je Feld, einmal am Tag beim ersten Bedarf.
    edges: Vec<[(u32, f32); 8]>,
    edge_n: Vec<u8>,
    edge_stamp: Vec<u64>,
    last_reach: Option<(u64, u32, u32)>,
    /// Tag, für den `info` gilt (je Feld; nur angeschaute Felder werden berechnet).
    stamp: Vec<u64>,
    today: u64,
    doy: u32,
    temp_c: f32,
}

#[derive(Debug, Clone, Copy, Default)]
struct TileInfo {
    drink: bool,
    reliable: f32,
    food: f32,
    coast: bool,
}

impl Scratch {
    fn prepare_day(&mut self, world: &World, weather: &Weather, doy: u32, day: u64) {
        let n = world.tiles.len();
        if self.info.len() != n {
            self.info = vec![TileInfo::default(); n];
            self.stamp = vec![u64::MAX; n];
        }
        if self.edges.len() != n {
            self.edges = vec![[(0, 0.0); 8]; n];
            self.edge_n = vec![0; n];
            self.edge_stamp = vec![u64::MAX; n];
        }
        self.today = day;
        self.doy = doy;
        self.temp_c = weather.temp_c;
    }

    /// Tageswerte eines Feldes, beim ersten Blick am Tag berechnet.
    fn at(&mut self, world: &World, i: usize) -> TileInfo {
        if self.stamp[i] != self.today {
            self.stamp[i] = self.today;
            let land = !world.tiles[i].is_ocean();
            let d = land && drinkable(world, i);
            let temp = self.temp_c - world.tiles[i].elevation.max(0.0) * 0.0065;
            self.info[i] = TileInfo {
                drink: d,
                reliable: if d { water_reliability(world, i) } else { 0.0 },
                food: if land {
                    forage_yield(world, i, self.doy, temp)
                } else {
                    0.0
                },
                coast: land && coastal(world, i),
            };
        }
        self.info[i]
    }
}

impl Scratch {
    fn reach(&mut self, world: &World, from: u32, max_cost: f32) {
        let n = world.tiles.len();
        if self.cost.len() != n {
            self.cost = vec![f32::INFINITY; n];
            self.parent = vec![u32::MAX; n];
        }
        // Gleicher Ausgangspunkt am selben Tag (Familie, Gruppe): Ergebnis bleibt.
        if self.last_reach == Some((self.today, from, max_cost.to_bits())) {
            return;
        }
        self.last_reach = Some((self.today, from, max_cost.to_bits()));
        for &t in &self.touched {
            self.cost[t as usize] = f32::INFINITY;
            self.parent[t as usize] = u32::MAX;
        }
        self.touched.clear();
        // Kosten in Hundertstel für eine ganzzahlige, eindeutige Ordnung
        let mut heap = BinaryHeap::new();
        self.cost[from as usize] = 0.0;
        self.parent[from as usize] = from;
        self.touched.push(from);
        heap.push(Reverse((0u32, from)));
        while let Some(Reverse((c, i))) = heap.pop() {
            let ci = self.cost[i as usize];
            if (c as f32) > ci * 100.0 + 0.5 {
                continue;
            }
            let iu = i as usize;
            if self.edge_stamp[iu] != self.today {
                self.edge_stamp[iu] = self.today;
                let mut k = 0;
                for nb in world.neighbors(iu) {
                    if let Some(e) = edge_cost(world, iu, nb) {
                        self.edges[iu][k] = (nb as u32, e);
                        k += 1;
                    }
                }
                self.edge_n[iu] = k as u8;
            }
            for k in 0..self.edge_n[iu] as usize {
                let (nb, e) = self.edges[iu][k];
                let nb = nb as usize;
                let nc = ci + e;
                if nc <= max_cost && nc < self.cost[nb] {
                    if self.cost[nb] == f32::INFINITY {
                        self.touched.push(nb as u32);
                    }
                    self.cost[nb] = nc;
                    self.parent[nb] = i;
                    heap.push(Reverse(((nc * 100.0) as u32, nb as u32)));
                }
            }
        }
        // `touched` steht in der (festen) Reihenfolge, in der die Felder
        // erreicht wurden – das genügt für eine eindeutige Auswertung.
    }

    fn path(&self, from: u32, to: u32) -> Vec<u32> {
        let mut p = vec![to];
        let mut c = to;
        while c != from {
            c = self.parent[c as usize];
            if c == u32::MAX {
                return Vec::new();
            }
            p.push(c);
        }
        p.reverse();
        p
    }
}

/// Kann man hier trinken (auf dem Feld oder am Rand eines Gewässers)?
pub fn drinkable(world: &World, i: usize) -> bool {
    let wet = |j: usize| {
        let t = &world.tiles[j];
        !t.is_ocean() && (t.surface_water > POND_MM || t.flow >= RILL_FLOW)
    };
    wet(i) || world.neighbors(i).any(wet)
}

fn coastal(world: &World, i: usize) -> bool {
    world.neighbors(i).any(|j| world.tiles[j].is_ocean())
}

/// Nahrung, die ein Tag Sammeln auf einem Feld bringt (Rationen), ohne
/// Abzug für schon Abgesammeltes.
pub fn forage_yield(world: &World, i: usize, doy: u32, temp_c: f32) -> f32 {
    let t = &world.tiles[i];
    let berries = (150..270).contains(&doy);
    let autumn = (240..320).contains(&doy);
    // Außerhalb der Saison: Wurzeln, Knollen, Rinde, Vogeleier, Kleintiere
    let mut plants = t.shrub * if berries { 3.0 } else { 0.6 }
        + t.forest * if autumn { 2.5 } else { 0.6 }
        + t.grass * if (120..300).contains(&doy) { 0.8 } else { 0.35 };
    if temp_c < 0.0 {
        plants *= 0.3;
    }
    let shore = if coastal(world, i) { 1.7 } else { 0.0 };
    let fish = if world
        .neighbors(i)
        .any(|j| world.tiles[j].surface_water > 3.0 * POND_MM && !world.tiles[j].is_ocean())
    {
        0.7
    } else {
        0.0
    };
    plants + shore + fish
}

fn cheb(world: &World, a: u32, b: u32) -> i32 {
    let (ax, ay) = world.xy(a as usize);
    let (bx, by) = world.xy(b as usize);
    (ax as i32 - bx as i32)
        .abs()
        .max((ay as i32 - by as i32).abs())
}

fn facing(world: &World, from: u32, to: u32, old: bool) -> bool {
    let (fx, fy) = world.xy(from as usize);
    let (tx, ty) = world.xy(to as usize);
    let dx = (tx as i32 - ty as i32) - (fx as i32 - fy as i32);
    if dx > 0 {
        true
    } else if dx < 0 {
        false
    } else {
        old
    }
}

fn place_text(world: &World, lake: Option<&str>, i: u32) -> String {
    let (x, y) = world.xy(i as usize);
    match lake {
        Some(n) => format!("am {n} ({x}, {y})"),
        None => format!("bei ({x}, {y})"),
    }
}

impl People {
    pub fn alive(&self) -> usize {
        self.list.len()
    }

    pub fn count_at(&self, tile: u32) -> usize {
        self.list.iter().filter(|p| p.tile == tile).count()
    }

    fn ensure(&mut self, n: usize) {
        if self.taken.len() != n {
            self.taken = vec![0.0; n];
        }
    }

    /// Ein Boot landet an der Küste: fünf Menschen gehen an Land.
    /// Gibt die Namen zurück.
    pub fn arrive(&mut self, world: &World, tile: u32, now: u64, rng: &mut Rng) -> Vec<String> {
        self.ensure(world.tiles.len());
        self.boats += 1;
        let used: Vec<String> = self
            .list
            .iter()
            .map(|p| p.name.clone())
            .chain(self.departed.iter().map(|d| d.person.name.clone()))
            .collect();
        let mut names = Vec::new();
        let (x, y) = world.xy(tile as usize);
        for look in 0..BOAT_SIZE as u8 {
            let sex = look_sex(look);
            let mut taken: Vec<String> = used.clone();
            taken.extend(names.iter().cloned());
            let name = pick_name(sex, &taken, rng);
            let age_days = (16 + rng.below(19)) * 365 + rng.below(365);
            self.next_id += 1;
            let mut p = Person {
                id: self.next_id,
                name: name.clone(),
                sex,
                look,
                age_days,
                tile,
                prev_tile: tile,
                hunger: 0.1,
                thirst: 0.2,
                fatigue: 0.3,
                health: 1.0,
                injury: 0.0,
                pack: 2.0,
                activity: Activity::Idle,
                facing_right: rng.chance(0.5),
                memory: Vec::new(),
                bio: Vec::new(),
                arrived_hours: now,
                flags: Flags::default(),
                harm: None,
                mother: 0,
                father: 0,
                partner: 0,
                children: Vec::new(),
                pregnant_days: 0,
            };
            p.note(
                now,
                format!(
                    "Kommt mit einem Boot bei ({x}, {y}) auf die Insel, {} Jahre alt.",
                    p.age_years()
                ),
            );
            names.push(name);
            self.list.push(p);
        }
        // Gefährten kennen einander von Anfang an.
        let all = names.join(", ");
        for p in self.list.iter_mut().filter(|p| p.arrived_hours == now) {
            p.note(now, format!("Mit im Boot: {all}."));
        }
        names
    }
}

/// Ein Tag für alle Menschen.
#[allow(clippy::too_many_arguments)]
pub fn step_day(
    people: &mut People,
    world: &World,
    animals: &mut Animals,
    weather: &Weather,
    lakes: &crate::lakes::Lakes,
    doy: u32,
    now: u64,
    rng: &mut Rng,
    s: &mut Scratch,
) -> Vec<PeopleNews> {
    let mut news = Vec::new();
    for b in &mut people.bodies {
        b.days_left = b.days_left.saturating_sub(1);
    }
    people.bodies.retain(|b| b.days_left > 0);
    if people.list.is_empty() {
        return news;
    }
    people.ensure(world.tiles.len());
    let day = now / 24;
    s.prepare_day(world, weather, doy, day);
    for v in &mut people.taken {
        *v = (*v - REGROW).max(0.0);
    }
    let year = day / 365 + 1;
    let rained = weather.rain_mm >= 2.0;

    // Wo die anderen sind (Mitte der Gruppe, vom Vortag)
    let positions: Vec<(u32, u32)> = people.list.iter().map(|p| (p.id, p.tile)).collect();
    // Wer heute helfen kann: gesund, nicht durstig, mit Vorrat (Stand vom Vortag)
    let adults_exist = people.list.iter().any(|p| !p.is_child());
    let helpers: Vec<(u32, u32, String, bool, bool)> = people
        .list
        .iter()
        .filter(|p| !p.is_child())
        .map(|p| {
            (
                p.id,
                p.tile,
                p.name.clone(),
                p.thirst < 0.3 && p.injury < 0.2,
                p.pack >= 1.5 && p.hunger < 0.3,
            )
        })
        .collect();
    let mut gave_food: Vec<u32> = Vec::new();
    // Verletzte Gefährten (Stand vom Vortag): Gesunde bleiben bei ihnen.
    let hurt: Vec<(u32, u32)> = people
        .list
        .iter()
        .filter(|p| p.injury > 0.2 || p.health < 0.6)
        .map(|p| (p.id, p.tile))
        .collect();

    for k in 0..people.list.len() {
        let p = &mut people.list[k];
        p.harm = None;
        p.prev_tile = p.tile;
        p.age_days += 1;
        let start = p.tile;
        // Kinder ziehen mit ihrer Familie (unten, nach den Erwachsenen).
        if p.is_child() && adults_exist {
            continue;
        }

        // --- Umschauen und merken (nach einem Ortswechsel, sonst alle 5 Tage) ---
        let (px, py) = world.xy(start as usize);
        let scan = p.flags.scan_tile != start || day >= p.flags.scan_day + 5 || p.memory.is_empty();
        if scan {
            p.flags.scan_tile = start;
            p.flags.scan_day = day;
        }
        for dy in -VISION..=VISION {
            if !scan {
                break;
            }
            for dx in -VISION..=VISION {
                let (x, y) = (px as i32 + dx, py as i32 + dy);
                if x < 0 || y < 0 || x >= world.width as i32 || y >= world.height as i32 {
                    continue;
                }
                let i = world.idx(x as u32, y as u32);
                if world.tiles[i].is_ocean() {
                    continue;
                }
                let info = s.at(world, i);
                let water = info.drink;
                let food = info.food * (1.0 - people.taken[i]);
                // Trockene Stellen vergisst man erst, wenn man davorsteht:
                // Ein Bach kann nach dem nächsten Regen wieder fließen.
                if water && standable(world, i) {
                    remember_water(p, world, i as u32, info.reliable, day);
                } else if !water && dx.abs() <= 1 && dy.abs() <= 1 {
                    forget_dry(p, i as u32);
                }
                remember(p, i as u32, PlaceKind::Food, food, day);
                if info.coast {
                    remember(p, i as u32, PlaceKind::Shore, 1.3, day);
                }
                if water && !p.flags.knows_water {
                    p.flags.knows_water = true;
                    let lake = lakes.name_at(i as u32);
                    p.note(
                        now,
                        format!("Findet Wasser {}.", place_text(world, lake, i as u32)),
                    );
                }
            }
        }

        for dy in -VISION_LAKE..=VISION_LAKE {
            if !scan {
                break;
            }
            for dx in -VISION_LAKE..=VISION_LAKE {
                if dx.abs().max(dy.abs()) <= VISION {
                    continue;
                }
                let (x, y) = (px as i32 + dx, py as i32 + dy);
                if x < 0 || y < 0 || x >= world.width as i32 || y >= world.height as i32 {
                    continue;
                }
                let i = world.idx(x as u32, y as u32);
                let t = &world.tiles[i];
                if t.is_ocean() || t.surface_water <= 3.0 * POND_MM {
                    continue;
                }
                // Ein Ufer merken, auf dem man stehen kann
                if let Some(j) = world.neighbors(i).find(|&j| standable(world, j)) {
                    remember_water(p, world, j as u32, 1.0, day);
                }
            }
        }

        // --- Ziel wählen ---
        let budget = DAY_BUDGET
            * if p.injury > 0.2 { 0.6 } else { 1.0 }
            * if p.is_child() { 0.6 } else { 1.0 }
            * if p.fatigue > 0.9 { 0.5 } else { 1.0 };
        let fire_near = world.tiles[start as usize].fire > 0.0
            || world
                .neighbors(start as usize)
                .any(|j| world.tiles[j].fire > 0.0);
        // Zufrieden: satt, Wasser hier, bei den anderen, niemand braucht
        // Hilfe – dann bleibt man und sammelt, ohne neue Wege zu planen.
        let content = {
            let (cx, cy, others) = group_center(world, &positions, p.id);
            let near_group = others == 0 || cheb(world, start, world.idx(cx, cy) as u32) <= 6;
            let someone_hurt = hurt.iter().any(|h| h.0 != p.id && h.1 != start);
            let reliable_known = p
                .memory
                .iter()
                .any(|m| m.kind == PlaceKind::Water && m.value >= 0.6);
            !fire_near
                && p.thirst < 0.2
                && s.at(world, start as usize).drink
                && p.hunger < 0.15
                && p.pack >= PACK_MAX * 0.5
                && p.injury < 0.2
                && near_group
                && !someone_hurt
                && reliable_known
                && !p.is_child()
                && s.at(world, start as usize).food * (1.0 - people.taken[start as usize]) > 0.5
        };
        if !content {
            // Wegsuche im Umkreis von zwei Tagesmärschen, durstig weiter.
            let range = if p.thirst >= 0.2 || p.hunger > 0.35 {
                3.0
            } else {
                1.2
            };
            s.reach(world, start, budget * range);
        }
        // Verletzte und Schwache bekommen von Gefährten am selben Ort Wasser.
        let needy = p.injury > 0.2 || p.health < 0.6;
        let water_helper = if needy && p.thirst >= 0.25 {
            helpers
                .iter()
                .find(|h| h.0 != p.id && h.1 == start && h.3)
                .map(|h| h.2.clone())
        } else {
            None
        };
        if let Some(h) = &water_helper {
            if p.thirst > 0.45 {
                p.note(now, format!("{h} bringt Wasser."));
            }
            p.thirst = 0.0;
        }
        if p.hunger > 0.5
            && let Some(h) = helpers
                .iter()
                .find(|h| h.0 != p.id && h.1 == start && h.4 && !gave_food.contains(&h.0))
        {
            gave_food.push(h.0);
            p.pack += 1.0;
            p.note(now, format!("{} teilt Essen.", h.2));
        }
        let can_drink_here = s.at(world, start as usize).drink || rained;

        let mut activity;
        let target: u32 = if content {
            activity = Activity::Gather;
            start
        } else if fire_near {
            activity = Activity::Run;
            // Das erreichbare Feld, das am weitesten vom Feuer weg ist
            let mut best = start;
            let mut best_d = -1;
            for &t in &s.touched {
                if s.cost[t as usize] > budget {
                    continue;
                }
                let d = nearest_fire(world, t, 8);
                if d > best_d {
                    best_d = d;
                    best = t;
                }
            }
            best
        } else if p.thirst >= 0.2 && !can_drink_here {
            activity = Activity::Walk;
            // Bekannte Wasserstelle, sonst bergab suchen
            // Verlässliche Stellen zuerst: Ein Rinnsal ist nach ein paar
            // trockenen Tagen versiegt, ein See nicht.
            let score = |m: &Place| s.cost[m.tile as usize] - 30.0 * m.value;
            let known = p
                .memory
                .iter()
                .filter(|m| m.kind == PlaceKind::Water && s.cost[m.tile as usize].is_finite())
                .min_by(|a, b| score(a).total_cmp(&score(b)).then(a.tile.cmp(&b.tile)))
                .map(|m| m.tile);
            match known {
                Some(t) => t,
                None => explore(world, s, start, budget, rng),
            }
        } else {
            // Sammeln: beste bekannte Stelle, abzüglich Weg
            let need = p.hunger >= 0.15 || p.pack < PACK_MAX * 0.5;
            let here = s.at(world, start as usize).food * (1.0 - people.taken[start as usize]);
            let water_gap = |p: &Person, t: u32| -> f32 {
                p.memory
                    .iter()
                    .filter(|m| m.kind == PlaceKind::Water)
                    .map(|m| cheb(world, m.tile, t) as f32 + 12.0 * (1.0 - m.value))
                    .fold(30.0, f32::min)
            };
            // Hunger macht weite Wege und Abstand zum Wasser erträglicher.
            let gap_w = 0.08 * (1.0 - p.hunger).max(0.2);
            let walk_w = 0.06 * (1.0 - 0.6 * p.hunger);
            let mut best = start;
            let mut best_score = here - gap_w * water_gap(p, start);
            for m in p.memory.iter().filter(|m| m.kind != PlaceKind::Water) {
                let c = s.cost[m.tile as usize];
                if !c.is_finite() {
                    continue;
                }
                // Alte Eindrücke verblassen (die Jahreszeit ändert sich).
                let age = (day - m.seen_day) as f32;
                let value = if m.kind == PlaceKind::Shore {
                    m.value
                } else {
                    (m.value - 0.02 * age).max(0.0)
                };
                let score = value - walk_w * c - gap_w * water_gap(p, m.tile);
                if score > best_score + 0.05 {
                    best_score = score;
                    best = m.tile;
                }
            }
            // Großer Hunger und nichts Gutes in Sicht: zur Küste hinunter
            if p.hunger > 0.35 && best_score < 0.6 && best == start {
                best = explore(world, s, start, budget, rng);
            }
            // Noch keine verlässliche Wasserstelle bekannt: erkunden
            let reliable = p
                .memory
                .iter()
                .any(|m| m.kind == PlaceKind::Water && m.value >= 0.6);
            let scouting = !reliable && p.thirst < 0.2 && p.hunger < 0.4 && rng.chance(0.5);
            if scouting {
                best = scout(world, s, start, budget, rng);
            }
            // Wer gesund ist und nicht hungert, kümmert sich um Verletzte.
            let fine = p.injury < 0.2 && p.health > 0.7 && p.hunger < 0.4 && p.thirst < 0.3;
            let carer = fine
                .then(|| {
                    hurt.iter()
                        .filter(|h| h.0 != p.id)
                        .min_by_key(|h| (cheb(world, h.1, start), h.0))
                        .copied()
                })
                .flatten();
            if let Some((_, t)) = carer
                && s.cost[t as usize].is_finite()
            {
                best = t;
            }
            // Paare bleiben beieinander, wenn nichts drängt.
            if fine
                && p.partner != 0
                && let Some(&(_, pt)) = positions.iter().find(|(id, _)| *id == p.partner)
                && cheb(world, start, pt) > 1
                && cheb(world, best, pt) > 1
                && s.cost[pt as usize].is_finite()
            {
                best = pt;
            }
            // Kinder bleiben in der Nähe der Mutter.
            if p.is_child()
                && let Some(&(_, mt)) = positions.iter().find(|(id, _)| *id == p.mother)
                && cheb(world, start, mt) > 1
                && s.cost[mt as usize].is_finite()
            {
                best = mt;
            }
            // Ohne Not bei den anderen bleiben
            let (cx, cy, others) = group_center(world, &positions, p.id);
            if others > 0 {
                let center = world.idx(cx, cy) as u32;
                let far = cheb(world, best, center);
                if !need && cheb(world, start, center) > 6 {
                    best = nearest_reachable(world, s, center).unwrap_or(best);
                } else if need && far > 10 && here > 0.6 {
                    best = start;
                }
            }
            activity = if scouting {
                Activity::Walk
            } else if need || best != start {
                Activity::Gather
            } else {
                Activity::Idle
            };
            best
        };

        // --- Gehen ---
        let mut path = if target == start {
            vec![start]
        } else {
            s.path(start, target)
        };
        if path.is_empty() {
            path = vec![start];
        }
        let mut spent = 0.0;
        let mut at = start;
        let mut drank = can_drink_here;
        for w in path.windows(2) {
            let (a, b) = (w[0], w[1]);
            let Some(e) = edge_cost(world, a as usize, b as usize) else {
                break;
            };
            if spent + e > budget {
                break;
            }
            spent += e;
            // Abstieg über eine hohe Stufe: Sturzgefahr
            let drop = world.tiles[a as usize].elevation - world.tiles[b as usize].elevation;
            if drop > FALL_FROM_M {
                let risk = ((drop - FALL_FROM_M) * 0.001).min(0.02);
                if rng.chance(risk) {
                    let sev = 0.15 + 0.4 * rng.next_f32();
                    p.injury = clamp01(p.injury + sev);
                    p.health -= sev * 0.6;
                    p.harm = Some(Cause::Fall);
                    let (x, y) = world.xy(b as usize);
                    let how = if sev > 0.4 { "schwer" } else { "leicht" };
                    p.note(
                        now,
                        format!("Stürzt an einem Abhang bei ({x}, {y}) und verletzt sich {how}."),
                    );
                    news.push(PeopleNews {
                        kind: NewsKind::Injured,
                        tile: b,
                        text: format!("{} stürzt an einem Abhang und verletzt sich {how}.", p.name),
                    });
                    p.facing_right = facing(world, a, b, p.facing_right);
                    at = b;
                    break;
                }
            }
            p.facing_right = facing(world, a, b, p.facing_right);
            at = b;
            if s.at(world, at as usize).drink {
                drank = true;
            }
        }
        p.tile = at;
        p.fatigue = clamp01(p.fatigue + 0.25 + spent * 0.02);
        let moved_far = spent > budget * 0.5;

        // --- Trinken, Sammeln, Essen ---
        if drank {
            if p.thirst >= 0.3 && !moved_far && activity != Activity::Run {
                activity = Activity::Drink;
            }
            p.thirst = 0.0;
            p.flags.thirsty = false;
        } else {
            // Tau, Beeren und feuchte Pflanzen lindern ein wenig.
            let juicy = if (150..270).contains(&doy) {
                world.tiles[at as usize].shrub * 0.08
            } else {
                0.0
            };
            p.thirst += (THIRST_PER_DAY - juicy).max(0.12);
        }
        let time_left = clamp01(1.0 - spent / DAY_BUDGET);
        let mut food = p.pack;
        if activity != Activity::Run {
            let i = at as usize;
            let food_i = s.at(world, i).food;
            let y = food_i * (1.0 - people.taken[i]);
            let work = (0.3 + 0.7 * time_left)
                * if p.injury > 0.2 { 0.5 } else { 1.0 }
                * if p.is_child() { 0.5 } else { 1.0 };
            let got = (y * work).min(2.5);
            // Abgesammelt: Je mehr man holt, desto weniger bleibt.
            let cap = food_i.max(0.5) * 8.0;
            people.taken[i] = clamp01(people.taken[i] + got / cap);
            food += got;
            // Fleisch von einem Kadaver
            if let Some(c) = animals
                .carcasses
                .iter_mut()
                .find(|c| c.tile == at && c.meat > 0.05)
            {
                let m = c.meat.min(0.8);
                c.meat -= m;
                food += m * 2.0;
                if p.hunger > 0.2 {
                    p.note(now, "Findet ein totes Tier und isst davon.".to_string());
                }
            }
        }
        let want = if p.hunger > 0.3 { 1.5 } else { 1.0 };
        let eat = food.min(want);
        food -= eat;
        p.pack = food.min(PACK_MAX);
        p.hunger = clamp01(p.hunger + HUNGER_PER_DAY - HUNGER_PER_RATION * eat).max(0.0);
        if activity == Activity::Gather && eat > 0.0 && !moved_far && time_left > 0.8 {
            activity = Activity::Eat;
        }

        // --- Nacht: Kälte, Schlaf ---
        let tt = &world.tiles[at as usize];
        let temp = weather.temp_at(tt.elevation);
        let shelter = clamp01(tt.forest * 1.5 + tt.shrub * 0.4);
        let mut cold = 0.0;
        if temp < 2.0 {
            cold = (2.0 - temp) * 0.004 * (1.0 - shelter);
            if p.hunger > 0.5 {
                cold *= 1.5;
            }
        }
        if cold > 0.0 {
            p.health -= cold;
            if cold > 0.01 {
                p.harm = Some(p.harm.unwrap_or(Cause::Cold));
                if p.flags.cold_year != year {
                    p.flags.cold_year = year;
                    p.note(now, format!("Friert bei {temp:.0} °C ohne Schutz."));
                }
            }
        }
        let rest = if cold > 0.01 || p.hunger > 0.6 {
            0.35
        } else {
            0.6
        };
        p.fatigue = clamp01(p.fatigue - rest);

        // --- Hunger, Durst, Heilung ---
        if p.hunger > 0.75 {
            p.health -= 0.02;
            p.harm = Some(Cause::Hunger);
        }
        if p.thirst > 0.7 {
            p.health -= 0.08;
            p.harm = Some(Cause::Thirst);
        }
        if p.hunger < 0.5 && p.thirst < 0.5 && cold < 0.005 {
            p.health = (p.health + 0.01).min(1.0);
            if p.injury > 0.0 {
                p.injury = (p.injury - 0.015).max(0.0);
                if p.injury == 0.0 {
                    p.note(now, "Ist wieder genesen.".to_string());
                }
            }
        }
        if p.hunger > 0.5 && !p.flags.hungry {
            p.flags.hungry = true;
            p.note(now, "Leidet Hunger.".to_string());
        } else if p.hunger < 0.2 && p.flags.hungry {
            p.flags.hungry = false;
            p.note(now, "Hat wieder genug zu essen.".to_string());
        }
        if p.thirst > 0.6 && !p.flags.thirsty {
            p.flags.thirsty = true;
            p.note(now, "Findet kein Wasser und leidet Durst.".to_string());
        }
        if doy == 80 && p.age_days > 300 && day * 24 > p.arrived_hours + 24 * 100 {
            p.flags.winters += 1;
            let n = p.flags.winters;
            p.note(
                now,
                if n == 1 {
                    "Übersteht den ersten Winter auf der Insel.".to_string()
                } else {
                    format!("Übersteht den {n}. Winter auf der Insel.")
                },
            );
        }
        if p.injury > 0.2 && activity != Activity::Run {
            activity = Activity::Hurt;
        }
        if activity == Activity::Walk && !moved_far && spent == 0.0 {
            activity = Activity::Idle;
        }
        p.activity = activity;
    }

    for id in gave_food {
        if let Some(h) = people.list.iter_mut().find(|q| q.id == id) {
            h.pack = (h.pack - 1.0).max(0.0);
        }
    }

    // --- Kleinkinder: mit der Mutter (oder einem anderen Erwachsenen) ---
    if adults_exist {
        care_for_children(people, world, weather, doy, now);
    }

    // --- Ältere Kinder: Erwachsene am selben Ort teilen mit ihnen ---
    share_with_children(people, now);

    // --- Paare, Schwangerschaft, Geburt ---
    family_day(people, world, now, rng, &mut news);

    // --- Gefahren durch andere: Wölfe, Feuer ---
    for k in 0..people.list.len() {
        let tile = people.list[k].tile;
        let company = people.list.iter().filter(|q| q.tile == tile).count();
        let hungry_wolves = animals
            .list
            .iter()
            .filter(|a| {
                a.species == Species::Wolf && a.energy < 0.5 && cheb(world, a.tile, tile) <= 1
            })
            .count();
        let p = &mut people.list[k];
        if hungry_wolves > 0 && company == 1 && rng.chance(0.03 * hungry_wolves as f32) {
            let sev = 0.3 + 0.4 * rng.next_f32();
            p.injury = clamp01(p.injury + sev);
            p.health -= sev * 0.8;
            p.harm = Some(Cause::Wolves);
            p.activity = Activity::Run;
            p.note(
                now,
                "Wird allein von hungrigen Wölfen angegriffen.".to_string(),
            );
            news.push(PeopleNews {
                kind: NewsKind::Injured,
                tile,
                text: format!("Hungrige Wölfe greifen {} an.", p.name),
            });
        }
        if world.tiles[tile as usize].fire > 0.0 {
            p.health -= 0.4;
            p.harm = Some(Cause::Fire);
        }
    }

    // --- Tod ---
    let mut k = 0;
    while k < people.list.len() {
        let p = &people.list[k];
        let years = p.age_days as f32 / 365.0;
        let old = if years > 45.0 {
            let x = (years - 45.0) / 40.0;
            (x * x / 365.0).min(0.05)
        } else {
            0.0
        };
        // Krankheiten: für Säuglinge und Kleinkinder viel gefährlicher
        let ill = match p.age_days {
            d if d < 365 => 0.0002,
            d if d < SMALL_CHILD_DAYS => 0.00003,
            _ => 0.00002,
        };
        let cause = if p.thirst >= 1.0 {
            Some(Cause::Thirst)
        } else if p.hunger >= 1.0 {
            Some(Cause::Hunger)
        } else if p.health <= 0.0 {
            Some(p.harm.unwrap_or(Cause::Hunger))
        } else if rng.chance(old) {
            Some(Cause::Age)
        } else if rng.chance(ill) {
            Some(Cause::Illness)
        } else {
            None
        };
        let Some(cause) = cause else {
            k += 1;
            continue;
        };
        let mut p = people.list.remove(k);
        p.health = 0.0;
        let age = p.age_years();
        p.note(now, format!("Ist mit {} {}.", p.age_text(), cause.label()));
        let partner = p.partner;
        if let Some(q) = people.list.iter_mut().find(|q| q.id == partner) {
            q.partner = 0;
        }
        let (x, y) = world.xy(p.tile as usize);
        news.push(PeopleNews {
            kind: NewsKind::Died,
            tile: p.tile,
            text: if p.is_child() {
                format!(
                    "{} ist {}, erst {} alt.",
                    p.name,
                    cause.label(),
                    short_age(p.age_days)
                )
            } else {
                format!("{} ist {} ({age} Jahre).", p.name, cause.label())
            },
        });
        people.bodies.push(Body {
            tile: p.tile,
            look: p.look,
            facing_right: p.facing_right,
            days_left: 12,
        });
        for q in &mut people.list {
            q.note(now, format!("Verliert {} ({x}, {y}).", p.name));
        }
        people.departed.push(Departed {
            person: p,
            cause,
            died_hours: now,
        });
        if people.list.is_empty() {
            news.push(PeopleNews {
                kind: NewsKind::AllGone,
                tile: people.departed.last().map(|d| d.person.tile).unwrap_or(0),
                text: "Kein Mensch lebt mehr auf der Insel.".to_string(),
            });
        }
    }
    news
}

fn short_age(days: u32) -> String {
    match days {
        d if d < 60 => format!("{} Tage", d.max(1)),
        d if d < 365 => format!("{} Monate", d / 30),
        d if d < 730 => "1 Jahr".to_string(),
        d => format!("{} Jahre", d / 365),
    }
}

/// Kleinkinder sind dort, wo ihre Mutter ist (sonst der Vater oder der
/// nächste Erwachsene). Sie trinken, wenn die Betreuerin getrunken hat, und
/// essen aus den Vorräten der Erwachsenen am selben Ort; Säuglinge trinken
/// bei der Mutter, solange sie nicht selbst hungert.
fn care_for_children(people: &mut People, world: &World, weather: &Weather, doy: u32, now: u64) {
    for k in 0..people.list.len() {
        if !people.list[k].is_child() {
            continue;
        }
        let small = people.list[k].is_small_child();
        let (mother, father, here) = {
            let c = &people.list[k];
            (c.mother, c.father, c.tile)
        };
        let adult = |p: &Person| !p.is_child();
        let carer = people
            .list
            .iter()
            .position(|p| p.id == mother && adult(p))
            .or_else(|| people.list.iter().position(|p| p.id == father && adult(p)))
            .or_else(|| {
                (0..people.list.len())
                    .filter(|&j| adult(&people.list[j]))
                    .min_by_key(|&j| (cheb(world, people.list[j].tile, here), people.list[j].id))
            });
        let Some(ci) = carer else { continue };
        let (ct, cprev, cface, cthirst, cid) = {
            let c = &people.list[ci];
            (c.tile, c.prev_tile, c.facing_right, c.thirst, c.id)
        };
        // Ältere Kinder sammeln am Ort mit, so gut sie es schon können.
        let mut found = 0.0f32;
        if !small {
            let i = ct as usize;
            let temp = weather.temp_at(world.tiles[i].elevation);
            let y = forage_yield(world, i, doy, temp) * (1.0 - people.taken[i]);
            found = (y * 0.5).min(1.2);
            let cap = y.max(0.5) * 8.0;
            people.taken[i] = clamp01(people.taken[i] + found / cap);
            // Wissen geht weiter (Prinzip 6): Das Kind lernt die Wasserstellen
            // und Küstenplätze kennen, die die Betreuerin kennt.
            if people.list[k].age_days.is_multiple_of(30) {
                let known: Vec<Place> = people.list[ci]
                    .memory
                    .iter()
                    .filter(|m| m.kind != PlaceKind::Food)
                    .cloned()
                    .collect();
                let cname = people.list[ci].name.clone();
                let c = &mut people.list[k];
                let before = c
                    .memory
                    .iter()
                    .filter(|m| m.kind == PlaceKind::Water)
                    .count();
                for m in known {
                    if !c
                        .memory
                        .iter()
                        .any(|x| x.tile == m.tile && x.kind == m.kind)
                    {
                        remember(c, m.tile, m.kind, m.value, m.seen_day);
                    }
                }
                let after = c
                    .memory
                    .iter()
                    .filter(|m| m.kind == PlaceKind::Water)
                    .count();
                if before == 0 && after > 0 {
                    c.note(now, format!("Lernt von {cname}, wo es Wasser gibt."));
                }
            }
        }
        // Essen: Muttermilch, Selbstgesammeltes, Vorräte der Erwachsenen am Ort
        let need = if small { 0.5f32 } else { 0.8 };
        let mut eaten = found.min(need);
        people.list[k].pack = (people.list[k].pack + found - eaten).min(1.0);
        if eaten < need {
            let own = people.list[k].pack.min(need - eaten);
            people.list[k].pack -= own;
            eaten += own;
        }
        let nursing = people.list[k].age_days < 2 * 365 && cid == mother;
        if nursing && people.list[ci].hunger < 0.6 {
            eaten = need;
            people.list[ci].hunger = clamp01(people.list[ci].hunger + 0.012);
        } else if eaten < need {
            let order: Vec<usize> =
                std::iter::once(ci)
                    .chain((0..people.list.len()).filter(|&j| {
                        j != ci && adult(&people.list[j]) && people.list[j].tile == ct
                    }))
                    .collect();
            for j in order {
                let give = people.list[j].pack.min(need - eaten);
                if give > 0.0 {
                    people.list[j].pack -= give;
                    eaten += give;
                }
                if eaten >= need {
                    break;
                }
            }
            // Reicht der Vorrat nicht, teilen die Eltern ihr eigenes Essen:
            // Sie werden selbst hungriger, das Kind nicht.
            let short = need - eaten;
            if short > 0.0 && people.list[ci].hunger < 0.7 {
                people.list[ci].hunger =
                    clamp01(people.list[ci].hunger + short * HUNGER_PER_RATION);
                eaten = need;
            }
        }
        let tt = &world.tiles[ct as usize];
        let temp = weather.temp_at(tt.elevation);
        let shelter = clamp01(tt.forest * 1.5 + tt.shrub * 0.4);
        let c = &mut people.list[k];
        c.prev_tile = if c.tile == ct { ct } else { cprev };
        c.tile = ct;
        c.facing_right = cface;
        c.activity = if c.prev_tile != c.tile {
            Activity::Walk
        } else if found > 0.0 {
            Activity::Gather
        } else {
            Activity::Idle
        };
        if cthirst < 0.25 {
            c.thirst = 0.0;
        } else {
            c.thirst += THIRST_PER_DAY;
        }
        // Bezogen auf den kleineren Bedarf eines Kindes
        c.hunger = clamp01(c.hunger + HUNGER_PER_DAY - HUNGER_PER_RATION * eaten / need);
        // Getragene Kinder frieren weniger, aber Kälte trifft sie trotzdem.
        if temp < 2.0 {
            let cold = (2.0 - temp) * 0.003 * (1.0 - shelter);
            c.health -= cold;
            if cold > 0.01 {
                c.harm = Some(Cause::Cold);
            }
        }
        if c.hunger > 0.75 {
            c.health -= 0.03;
            c.harm = Some(Cause::Hunger);
        }
        if c.thirst > 0.7 {
            c.health -= 0.1;
            c.harm = Some(Cause::Thirst);
        }
        if c.hunger < 0.5 && c.thirst < 0.5 {
            c.health = (c.health + 0.01).min(1.0);
        }
        if c.age_days == SMALL_CHILD_DAYS - 1 {
            c.note(
                now,
                "Läuft nun selbst mit und hilft beim Sammeln.".to_string(),
            );
        }
        if c.age_days == ADULT_DAYS - 1 {
            c.note(now, "Ist nun erwachsen und geht eigene Wege.".to_string());
        }
    }
}

/// Ältere Kinder sammeln schon selbst, bekommen aber von Eltern (sonst
/// anderen Erwachsenen) am selben Ort ab, wenn sie hungern.
fn share_with_children(people: &mut People, now: u64) {
    for k in 0..people.list.len() {
        let c = &people.list[k];
        if !c.is_child() || c.is_small_child() || c.hunger < 0.3 {
            continue;
        }
        let (tile, mother, father) = (c.tile, c.mother, c.father);
        let giver = (0..people.list.len())
            .filter(|&j| {
                let a = &people.list[j];
                !a.is_child() && a.tile == tile && (a.pack >= 0.5 || a.hunger < 0.5)
            })
            .min_by_key(|&j| {
                let id = people.list[j].id;
                (id != mother && id != father, id)
            });
        let Some(g) = giver else { continue };
        let from_pack = people.list[g].pack.min(1.0);
        people.list[g].pack -= from_pack;
        if from_pack < 1.0 {
            people.list[g].hunger =
                clamp01(people.list[g].hunger + (1.0 - from_pack) * HUNGER_PER_RATION);
        }
        let gname = people.list[g].name.clone();
        let c = &mut people.list[k];
        c.hunger = (c.hunger - HUNGER_PER_RATION).max(0.0);
        if c.hunger > 0.5 && !c.flags.hungry {
            c.note(now, format!("{gname} teilt Essen."));
        }
    }
}

/// Paare finden sich, Frauen werden schwanger, Kinder kommen zur Welt.
fn family_day(
    people: &mut People,
    world: &World,
    now: u64,
    rng: &mut Rng,
    news: &mut Vec<PeopleNews>,
) {
    let n = people.list.len();
    // Paare: Unverbundene Erwachsene, die viel beieinander sind
    for a in 0..n {
        for b in 0..n {
            let (pa, pb) = (&people.list[a], &people.list[b]);
            if pa.sex != Sex::Female || pb.sex != Sex::Male {
                continue;
            }
            if pa.partner != 0 || pb.partner != 0 || pa.is_child() || pb.is_child() {
                continue;
            }
            if pa.age_days < 16 * 365 || pb.age_days < 16 * 365 {
                continue;
            }
            if cheb(world, pa.tile, pb.tile) > 1 {
                continue;
            }
            // Keine engen Verwandten
            let related = pa.mother == pb.id
                || pb.mother == pa.id
                || pa.father == pb.id
                || pb.father == pa.id
                || (pa.mother != 0 && pa.mother == pb.mother)
                || (pa.father != 0 && pa.father == pb.father);
            if related {
                continue;
            }
            let gap = (pa.age_days as i64 - pb.age_days as i64).unsigned_abs() as f32 / 365.0;
            let chance = PAIR_CHANCE * if gap > 15.0 { 0.25 } else { 1.0 };
            if !rng.chance(chance) {
                continue;
            }
            let (ida, idb) = (pa.id, pb.id);
            let (na, nb) = (pa.name.clone(), pb.name.clone());
            people.list[a].partner = idb;
            people.list[b].partner = ida;
            people.list[a].note(now, format!("Wird die Gefährtin von {nb}."));
            people.list[b].note(now, format!("Wird der Gefährte von {na}."));
        }
    }

    // Schwangerschaft und Geburt
    let mut births = Vec::new();
    for k in 0..n {
        let p = &people.list[k];
        if p.sex != Sex::Female {
            continue;
        }
        if p.pregnant_days > 0 {
            let starving = p.hunger > 0.7 || p.health < 0.4;
            let q = &mut people.list[k];
            if starving && rng.chance(0.01) {
                q.pregnant_days = 0;
                q.note(now, "Verliert ihr ungeborenes Kind.".to_string());
                continue;
            }
            q.pregnant_days += 1;
            if q.pregnant_days >= PREGNANCY_DAYS {
                q.pregnant_days = 0;
                births.push(k);
            }
            continue;
        }
        let years = p.age_days / 365;
        if p.partner == 0 || !(16..=42).contains(&years) {
            continue;
        }
        if p.hunger > 0.5 || p.health < 0.6 || p.injury > 0.3 {
            continue;
        }
        let fed = 1.0 - p.hunger * 1.6;
        let Some(partner) = people.list.iter().find(|q| q.id == p.partner) else {
            continue;
        };
        if cheb(world, partner.tile, p.tile) > 4 {
            continue;
        }
        // Das jüngste Kind wird noch gestillt
        let youngest = p
            .children
            .iter()
            .filter_map(|c| people.list.iter().find(|q| q.id == *c))
            .map(|c| c.age_days)
            .min();
        if youngest.is_some_and(|a| a < 2 * 365) {
            continue;
        }
        if rng.chance(CONCEIVE_CHANCE * fed) {
            people.list[k].pregnant_days = 1;
            people.list[k].note(now, "Erwartet ein Kind.".to_string());
        }
    }
    for k in births {
        let used: Vec<String> = people
            .list
            .iter()
            .map(|p| p.name.clone())
            .chain(people.departed.iter().map(|d| d.person.name.clone()))
            .collect();
        let sex = if rng.chance(0.5) {
            Sex::Female
        } else {
            Sex::Male
        };
        let look = match sex {
            Sex::Female => [0u8, 2, 4][rng.below(3) as usize],
            Sex::Male => [1u8, 3][rng.below(2) as usize],
        };
        let name = pick_name(sex, &used, rng);
        let mother = &people.list[k];
        let (mid, mname, tile, father) =
            (mother.id, mother.name.clone(), mother.tile, mother.partner);
        let memory = mother.memory.clone();
        let fname = people
            .list
            .iter()
            .find(|q| q.id == father)
            .map(|q| q.name.clone());
        let (x, y) = world.xy(tile as usize);
        let what = if sex == Sex::Female {
            "Tochter"
        } else {
            "Sohn"
        };
        // „eine Tochter“, „einen Sohn“
        let a_what = if sex == Sex::Female {
            "eine Tochter"
        } else {
            "einen Sohn"
        };
        // Eine Geburt ist gefährlich – für Kind und Mutter.
        let stillborn = rng.chance(0.03);
        let mother_dies = rng.chance(0.015);
        {
            let m = &mut people.list[k];
            m.health -= 0.15;
            if mother_dies {
                m.health = 0.0;
                m.harm = Some(Cause::Childbirth);
            }
        }
        if stillborn {
            people.list[k].note(now, format!("Bringt {a_what} tot zur Welt."));
            continue;
        }
        people.next_id += 1;
        let id = people.next_id;
        people.born += 1;
        let first = people.born == 1;
        let mut child = Person {
            id,
            name: name.clone(),
            sex,
            look,
            age_days: 0,
            tile,
            prev_tile: tile,
            hunger: 0.0,
            thirst: 0.0,
            fatigue: 0.0,
            health: 1.0,
            injury: 0.0,
            pack: 0.0,
            activity: Activity::Sleep,
            facing_right: rng.chance(0.5),
            memory,
            bio: Vec::new(),
            arrived_hours: now,
            flags: Flags::default(),
            harm: None,
            mother: mid,
            father,
            partner: 0,
            children: Vec::new(),
            pregnant_days: 0,
        };
        let parents = match &fname {
            Some(f) => format!("Mutter: {mname}, Vater: {f}."),
            None => format!("Mutter: {mname}."),
        };
        child.note(now, format!("Wird bei ({x}, {y}) geboren. {parents}"));
        people.list[k].children.push(id);
        people.list[k].note(now, format!("Bringt {a_what} zur Welt: {name}."));
        if let Some(f) = people.list.iter_mut().find(|q| q.id == father) {
            f.children.push(id);
            f.note(now, format!("Wird Vater: {name} ({what})."));
        }
        let text = if first {
            format!(
                "Das erste Kind der Insel kommt zur Welt: {mname} bringt {a_what} zur Welt, {name}."
            )
        } else {
            format!("{mname} bringt {a_what} zur Welt: {name}.")
        };
        news.push(PeopleNews {
            kind: if first {
                NewsKind::FirstBorn
            } else {
                NewsKind::Born
            },
            tile,
            text,
        });
        people.list.push(child);
    }
}

/// Merkt sich einen Ort (oder vergisst ihn, wenn er nichts mehr bietet).
/// Kann man auf dem Feld stehen (kein Meer, kein tiefes Wasser)?
fn standable(world: &World, i: usize) -> bool {
    let t = &world.tiles[i];
    !t.is_ocean() && t.surface_water <= POND_MM
}

/// Wie verlässlich eine Wasserstelle ist (0..1): Teiche und Seen ganz,
/// Bäche nach dem Anteil der Tage, an denen sie fließen.
pub fn water_reliability(world: &World, i: usize) -> f32 {
    let mut best = 0.0f32;
    for j in std::iter::once(i).chain(world.neighbors(i)) {
        let t = &world.tiles[j];
        if t.is_ocean() {
            continue;
        }
        if t.surface_water > POND_MM {
            best = 1.0;
        }
        best = best.max(t.flow_permanence);
    }
    clamp01(best)
}

/// Wasserstellen werden gebündelt gemerkt: eine je Gewässerstück, damit
/// man mehrere verschiedene Stellen kennt.
fn remember_water(p: &mut Person, world: &World, tile: u32, value: f32, day: u64) {
    if let Some(m) = p
        .memory
        .iter_mut()
        .find(|m| m.kind == PlaceKind::Water && cheb(world, m.tile, tile) <= 2)
    {
        if m.tile == tile || value > m.value + 0.1 {
            m.tile = tile;
            m.value = value;
        }
        m.seen_day = day;
        return;
    }
    remember(p, tile, PlaceKind::Water, value.max(0.05), day);
}

fn forget_dry(p: &mut Person, tile: u32) {
    p.memory
        .retain(|m| !(m.tile == tile && m.kind == PlaceKind::Water));
}

fn remember(p: &mut Person, tile: u32, kind: PlaceKind, value: f32, day: u64) {
    let worth = match kind {
        PlaceKind::Water | PlaceKind::Shore => value > 0.0,
        PlaceKind::Food => value >= 0.8,
    };
    if let Some(k) = p
        .memory
        .iter()
        .position(|m| m.tile == tile && m.kind == kind)
    {
        if worth {
            p.memory[k].value = value;
            p.memory[k].seen_day = day;
        } else {
            p.memory.remove(k);
        }
        return;
    }
    if !worth {
        return;
    }
    let same_count = p.memory.iter().filter(|m| m.kind == kind).count();
    let cap = match kind {
        PlaceKind::Shore => 4,
        _ => MEMORY / 2,
    };
    if same_count >= cap {
        // Den schwächsten bzw. ältesten Ort derselben Art vergessen
        let worst = (0..p.memory.len())
            .filter(|&k| p.memory[k].kind == kind)
            .min_by(|&a, &b| {
                let sa = p.memory[a].value - (day - p.memory[a].seen_day) as f32 * 0.01;
                let sb = p.memory[b].value - (day - p.memory[b].seen_day) as f32 * 0.01;
                sa.total_cmp(&sb).then(a.cmp(&b))
            })
            .unwrap();
        let w = &p.memory[worst];
        let sw = w.value - (day - w.seen_day) as f32 * 0.01;
        if sw >= value {
            return;
        }
        p.memory.remove(worst);
    }
    p.memory.push(Place {
        tile,
        kind,
        value,
        seen_day: day,
    });
}

/// Ohne bekanntes Wasser: bergab suchen (dort sammelt sich Wasser), mit
/// etwas Zufall. Mensch kann sich irren.
fn explore(world: &World, s: &Scratch, start: u32, budget: f32, rng: &mut Rng) -> u32 {
    let mut best = start;
    let mut best_score = f32::INFINITY;
    for &t in &s.touched {
        let c = s.cost[t as usize];
        if c > budget || c < budget * 0.4 {
            continue;
        }
        let score = world.tiles[t as usize].elevation + rng.next_f32() * 15.0;
        if score < best_score {
            best_score = score;
            best = t;
        }
    }
    best
}

/// Erkunden: ein weit entferntes erreichbares Feld in zufälliger Richtung.
fn scout(world: &World, s: &Scratch, start: u32, budget: f32, rng: &mut Rng) -> u32 {
    let far: Vec<u32> = s
        .touched
        .iter()
        .copied()
        .filter(|&t| {
            let c = s.cost[t as usize];
            c > budget * 0.7 && c <= budget
        })
        .collect();
    if far.is_empty() {
        return start;
    }
    let _ = world;
    far[rng.below(far.len() as u32) as usize]
}

fn nearest_fire(world: &World, t: u32, r: i32) -> i32 {
    let (x, y) = world.xy(t as usize);
    for d in 0..=r {
        for dy in -d..=d {
            for dx in -d..=d {
                if dx.abs().max(dy.abs()) != d {
                    continue;
                }
                let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                if nx < 0 || ny < 0 || nx >= world.width as i32 || ny >= world.height as i32 {
                    continue;
                }
                if world.tile(nx as u32, ny as u32).fire > 0.0 {
                    return d;
                }
            }
        }
    }
    r + 1
}

fn group_center(world: &World, pos: &[(u32, u32)], me: u32) -> (u32, u32, usize) {
    let (mut sx, mut sy, mut n) = (0u64, 0u64, 0usize);
    for &(id, t) in pos {
        if id == me {
            continue;
        }
        let (x, y) = world.xy(t as usize);
        sx += x as u64;
        sy += y as u64;
        n += 1;
    }
    if n == 0 {
        return (0, 0, 0);
    }
    ((sx / n as u64) as u32, (sy / n as u64) as u32, n)
}

fn nearest_reachable(world: &World, s: &Scratch, center: u32) -> Option<u32> {
    s.touched
        .iter()
        .copied()
        .min_by_key(|&t| (cheb(world, t, center), t))
}
