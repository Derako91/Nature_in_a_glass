//! Tiere: Hirsche, Wölfe und Füchse als einzelne Tiere, Kaninchen als
//! Bestand je Feld (dafür sind es zu viele). Ein Aufruf entspricht einem Tag.
//!
//! Nahrungsnetz:
//! - Kaninchen fressen Gras, vermehren sich im Frühjahr und Sommer schnell und
//!   wandern in benachbarte Felder ab, wenn es eng wird.
//! - Hirsche fressen Gras, Busch und junge Bäume. Sie ziehen dorthin, wo es
//!   Futter gibt, bleiben gern beieinander und fliehen vor Wölfen.
//! - Wölfe jagen vor allem Hirsche, schwache und junge leichter, zu mehreren
//!   erfolgreicher. Hungrig nehmen sie auch Kaninchen.
//! - Füchse jagen Kaninchen und fressen an Kadavern.
//!
//! Jedes Tier hat Energiereserven. Fehlt Futter, zehrt es davon und
//! verhungert schließlich. Junge kommen einmal im Jahr zur Welt, wenn die
//! Mutter gut genährt ist und ein Partner in der Nähe lebt. Nichts wird
//! künstlich stabil gehalten: Eine ausgestorbene Art kehrt nur zurück, wenn
//! der Beobachter neue Tiere aussetzt.

use serde::{Deserialize, Serialize};

use crate::climate::Weather;
use crate::math::clamp01;
use crate::rng::Rng;
use crate::world::{POND_MM, World};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Species {
    Deer,
    Wolf,
    Fox,
    Rabbit,
}

impl Species {
    pub const ALL: [Species; 4] = [Species::Deer, Species::Rabbit, Species::Wolf, Species::Fox];

    pub fn index(self) -> usize {
        match self {
            Species::Deer => 0,
            Species::Wolf => 1,
            Species::Fox => 2,
            Species::Rabbit => 3,
        }
    }

    /// Einzahl, Mehrzahl.
    pub fn names(self) -> (&'static str, &'static str) {
        match self {
            Species::Deer => ("Hirsch", "Hirsche"),
            Species::Wolf => ("Wolf", "Wölfe"),
            Species::Fox => ("Fuchs", "Füchse"),
            Species::Rabbit => ("Kaninchen", "Kaninchen"),
        }
    }

    /// Mehrzahl im Dativ („mit den …“).
    pub fn dative_plural(self) -> &'static str {
        match self {
            Species::Deer => "Hirschen",
            Species::Wolf => "Wölfen",
            Species::Fox => "Füchsen",
            Species::Rabbit => "Kaninchen",
        }
    }

    /// Höchstalter in Tagen.
    fn max_age(self) -> u32 {
        match self {
            Species::Deer => 18 * 365,
            Species::Wolf => 13 * 365,
            Species::Fox => 9 * 365,
            Species::Rabbit => 0,
        }
    }

    /// Täglicher Energieverbrauch (Anteil der vollen Reserve).
    fn metabolism(self) -> f32 {
        match self {
            Species::Deer => 0.02,
            Species::Wolf => 0.02,
            Species::Fox => 0.035,
            Species::Rabbit => 0.0,
        }
    }

    /// Ab diesem Alter (Tage) gilt ein Tier als erwachsen und kann Junge haben.
    fn adult_age(self) -> u32 {
        match self {
            Species::Deer | Species::Wolf => 2 * 365,
            _ => 365,
        }
    }

    /// So lange (Tage) folgen Junge ihrer Mutter.
    fn follow_days(self) -> u32 {
        match self {
            Species::Deer => 200,
            Species::Wolf => 150,
            _ => 100,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Sex {
    Female,
    Male,
}

/// Was ein Tier heute getan hat (bestimmt das Sprite).
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Activity {
    Graze,
    Walk,
    Run,
    Rest,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Cause {
    Starved,
    Killed,
    Old,
    Fire,
    Other,
}

impl Cause {
    fn index(self) -> usize {
        match self {
            Cause::Starved => 0,
            Cause::Killed => 1,
            Cause::Old => 2,
            Cause::Fire => 3,
            Cause::Other => 4,
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            Cause::Starved => "verhungert",
            Cause::Killed => "gerissen",
            Cause::Old => "an Altersschwäche gestorben",
            Cause::Fire => "im Feuer umgekommen",
            Cause::Other => "auf andere Weise umgekommen",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Animal {
    pub id: u32,
    pub species: Species,
    pub sex: Sex,
    pub tile: u32,
    /// Feld vom Vortag (für gleitende Bewegung in der Darstellung).
    pub prev_tile: u32,
    pub age_days: u32,
    /// Energiereserve 0..1.
    pub energy: f32,
    pub activity: Activity,
    /// Blickrichtung auf dem Bildschirm: nach rechts unten.
    pub facing_right: bool,
    /// Fellvariante (fest).
    pub variant: u8,
    /// Mutter (0 = keine bekannt).
    pub mother: u32,
}

impl Animal {
    pub fn is_young(&self) -> bool {
        self.age_days < self.species.adult_age().min(365)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Carcass {
    pub tile: u32,
    pub species: Species,
    pub days_left: u16,
    /// Noch fressbares Fleisch für Wölfe (Energieeinheiten).
    #[serde(default)]
    pub meat: f32,
}

/// Todesfälle je Art und Ursache (laufendes Jahr).
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct DeathLog {
    pub counts: [[u32; 5]; 4],
}

impl DeathLog {
    pub fn add(&mut self, s: Species, c: Cause) {
        self.counts[s.index()][c.index()] += 1;
    }
    pub fn get(&self, s: Species, c: Cause) -> u32 {
        self.counts[s.index()][c.index()]
    }
    pub fn total(&self, s: Species) -> u32 {
        self.counts[s.index()].iter().sum()
    }
    /// Häufigste Todesursache einer Art.
    pub fn main_cause(&self, s: Species) -> Option<Cause> {
        let all = [
            Cause::Starved,
            Cause::Killed,
            Cause::Old,
            Cause::Fire,
            Cause::Other,
        ];
        let mut best: Option<(Cause, u32)> = None;
        for c in all {
            let n = self.get(s, c);
            if n > 0 && best.is_none_or(|(_, b)| n > b) {
                best = Some((c, n));
            }
        }
        best.map(|(c, _)| c)
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Animals {
    /// Einzeltiere, nach `id` sortiert.
    pub list: Vec<Animal>,
    /// Kaninchen je Feld.
    pub rabbits: Vec<f32>,
    pub carcasses: Vec<Carcass>,
    next_id: u32,
    /// Ob eine Art schon einmal auf der Insel gelebt hat.
    pub ever: [bool; 4],
    /// Todesfälle im laufenden Jahr.
    pub deaths: DeathLog,
    /// Todesfälle seit der letzten Aussterbe-Meldung je Art.
    pub deaths_since: DeathLog,
    /// Geburten im laufenden Jahr je Art.
    pub births: [u32; 4],
}

/// Was an einem Tag geschehen ist.
#[derive(Debug, Clone, Default)]
pub struct AnimalDay {
    /// Arten, deren letztes Tier heute gestorben ist.
    pub extinct: Vec<Species>,
}

// --- Kaninchen ---
/// Kaninchen je Feld bei vollem Gras ohne Wald.
const RABBIT_K_GRASS: f32 = 12.0;
const RABBIT_K_SHRUB: f32 = 4.0;
/// Grasverbrauch je Kaninchen und Tag (Anteil voller Grasdecke).
const RABBIT_EAT: f32 = 0.00008;
const RABBIT_GROWTH: f32 = 0.015;
/// Anteil, der bei Gedränge täglich in Nachbarfelder abwandert.
const RABBIT_DISPERSAL: f32 = 0.04;

// --- Hirsche ---
/// Futterbedarf je Hirsch und Tag (Anteil voller Grasdecke eines Feldes).
const DEER_NEED: f32 = 0.006;
/// Höchstens dieser Anteil des Futters auf einem Feld je Tier und Tag.
const BITE: f32 = 0.1;

/// Unter so viel Futter sucht ein Hirsch weiter entfernt (Felder).
const DEER_SEARCH_BELOW: f32 = 0.15;
const DEER_SEARCH: i32 = 5;
const DEER_SEARCH_HUNGRY: i32 = 10;

// --- Füchse ---
const FOX_SEARCH: i32 = 20;
/// Bei so vielen Kaninchen im Revier (Feld und Nachbarn) gelingt jede zweite Jagd.
const FOX_CATCH_HALF: f32 = 16.0;
/// Nebenbei gefundene Nahrung je Tag (Mäuse, Insekten, Beeren); im Winter die Hälfte.
const FOX_SIDE_FOOD: f32 = 0.025;

// --- Wölfe ---
/// Bei so vielen Hirschen auf der Insel gelingt nur noch die Hälfte der Jagden.
const PREY_HALF: f32 = 40.0;
/// Reviere: So viele Würfe im Jahr bietet die kleine Insel Platz – ein
/// Wolfsrudel, einige Fuchsfamilien. Weitere Paare bekommen keine Jungen.
pub const WOLF_LITTERS_PER_YEAR: u32 = 1;
pub const FOX_LITTERS_PER_YEAR: u32 = 3;
/// So viel Vorrat fressen sich Wölfe am Riss an (über die volle Reserve 1 hinaus).
const WOLF_GORGE: f32 = 1.6;
/// Erst unter dieser Energie gehen Wölfe auf die Jagd.
const WOLF_HUNGRY: f32 = 0.75;
/// Fleisch eines gerissenen Hirsches (Energieeinheiten für Wölfe).
const DEER_MEAT: f32 = 4.0;
/// So weit wittern Wölfe Beute (Felder).
const WOLF_SEARCH: i32 = 40;

impl Animals {
    pub fn total(&self, s: Species) -> u32 {
        match s {
            Species::Rabbit => self.rabbit_total().round() as u32,
            _ => self.list.iter().filter(|a| a.species == s).count() as u32,
        }
    }

    pub fn rabbit_total(&self) -> f32 {
        self.rabbits.iter().sum()
    }

    /// Tiere auf einem Feld.
    pub fn count_at(&self, tile: u32, s: Species) -> u32 {
        match s {
            Species::Rabbit => self
                .rabbits
                .get(tile as usize)
                .map(|r| r.round() as u32)
                .unwrap_or(0),
            _ => self
                .list
                .iter()
                .filter(|a| a.species == s && a.tile == tile)
                .count() as u32,
        }
    }

    fn ensure(&mut self, n: usize) {
        if self.rabbits.len() != n {
            self.rabbits = vec![0.0; n];
        }
    }

    /// Beute einer Art (für die Wahl des Aussetzortes).
    pub fn prey_tiles(&self, s: Species) -> Vec<u32> {
        match s {
            Species::Wolf => self
                .list
                .iter()
                .filter(|x| x.species == Species::Deer)
                .map(|x| x.tile)
                .collect(),
            Species::Fox => (0..self.rabbits.len() as u32)
                .filter(|&i| self.rabbits[i as usize] >= 1.0)
                .collect(),
            _ => Vec::new(),
        }
    }

    /// Setzt eine kleine Gruppe aus. Gibt die Zahl der Tiere zurück.
    pub fn release(&mut self, world: &World, s: Species, tile: u32, rng: &mut Rng) -> u32 {
        self.ensure(world.tiles.len());
        self.ever[s.index()] = true;
        if s == Species::Rabbit {
            let n = 20.0;
            self.rabbits[tile as usize] += n;
            return n as u32;
        }
        let (males, females) = match s {
            Species::Deer => (2, 4),
            Species::Wolf => (2, 2),
            _ => (2, 2),
        };
        for k in 0..(males + females) {
            let sex = if k < males { Sex::Male } else { Sex::Female };
            let age = s.adult_age() + rng.below(3 * 365);
            self.list.push(Animal {
                id: self.next_id + 1,
                species: s,
                sex,
                tile,
                prev_tile: tile,
                age_days: age,
                energy: 1.0,
                activity: Activity::Rest,
                facing_right: rng.chance(0.5),
                variant: rng.below(2) as u8,
                mother: 0,
            });
            self.next_id += 1;
        }
        males + females
    }
}

fn walkable(world: &World, i: usize) -> bool {
    let t = &world.tiles[i];
    !t.is_ocean() && t.surface_water <= POND_MM
}

fn cheb(world: &World, a: u32, b: u32) -> i32 {
    let (ax, ay) = world.xy(a as usize);
    let (bx, by) = world.xy(b as usize);
    (ax as i32 - bx as i32)
        .abs()
        .max((ay as i32 - by as i32).abs())
}

/// Futter für Hirsche auf einem Feld (in Einheiten voller Grasdecke).
pub fn deer_forage(t: &crate::world::Tile) -> f32 {
    let young_trees = if t.forest_age < 15.0 { t.forest } else { 0.0 };
    t.grass + 0.8 * t.shrub + 0.6 * young_trees
}

/// Hirsch frisst auf seinem Feld; gibt den gedeckten Anteil des Bedarfs zurück.
fn deer_eat(t: &mut crate::world::Tile) -> f32 {
    let mut need = DEER_NEED;
    let g = (t.grass * BITE).min(need);
    t.grass -= g;
    need -= g;
    let s = (t.shrub * BITE).min(need / 0.8) * 0.8;
    t.shrub -= s / 0.8;
    need -= s;
    // Junge Bäume werden verbissen: Das bremst die Verjüngung des Waldes.
    // Altbäume sind zu hoch; dort zählt nur der Unterwuchs (Gras, Busch).
    if t.forest_age < 15.0 {
        let f = (t.forest * BITE).min(need / 0.6) * 0.6;
        t.forest -= f / 0.6;
        need -= f;
    }
    clamp01(1.0 - need / DEER_NEED)
}

/// Schritt zum Ziel: Nachbar, der am nächsten heranführt (gangbar).
fn step_towards(world: &World, from: u32, to: u32) -> u32 {
    let mut best = from;
    let mut best_d = cheb(world, from, to);
    for nb in world.neighbors(from as usize) {
        if !walkable(world, nb) {
            continue;
        }
        let d = cheb(world, nb as u32, to);
        if d < best_d {
            best_d = d;
            best = nb as u32;
        }
    }
    best
}

/// Kürzester Weg über gangbare Felder zum nächsten Feld, das `goal` erfüllt
/// (Breitensuche, feste Nachbarreihenfolge). Gibt die ersten beiden Schritte
/// und die Weglänge zurück.
fn path_to(
    world: &World,
    from: u32,
    max_steps: u32,
    goal: impl Fn(usize) -> bool,
) -> Option<(u32, u32, u32)> {
    let n = world.tiles.len();
    let mut parent = vec![u32::MAX; n];
    let mut dist = vec![0u32; n];
    let mut queue = std::collections::VecDeque::new();
    parent[from as usize] = from;
    queue.push_back(from as usize);
    while let Some(i) = queue.pop_front() {
        if i != from as usize && goal(i) {
            // Weg zurückverfolgen
            let mut path = vec![i as u32];
            let mut c = i;
            while parent[c] as usize != from as usize {
                c = parent[c] as usize;
                path.push(c as u32);
            }
            path.reverse();
            let first = path[0];
            let second = *path.get(1).unwrap_or(&first);
            return Some((first, second, dist[i]));
        }
        if dist[i] >= max_steps {
            continue;
        }
        for nb in world.neighbors(i) {
            if parent[nb] == u32::MAX && walkable(world, nb) {
                parent[nb] = i as u32;
                dist[nb] = dist[i] + 1;
                queue.push_back(nb);
            }
        }
    }
    None
}

/// Zufälliger Schritt auf ein gangbares Nachbarfeld.
fn random_step(world: &World, from: u32, rng: &mut Rng) -> u32 {
    let nb: Vec<usize> = world
        .neighbors(from as usize)
        .filter(|&j| walkable(world, j))
        .collect();
    if nb.is_empty() {
        from
    } else {
        nb[rng.below(nb.len() as u32) as usize] as u32
    }
}

/// Schritt weg von einer Gefahr.
fn step_away(world: &World, from: u32, danger: u32) -> u32 {
    let mut best = from;
    let mut best_d = cheb(world, from, danger);
    for nb in world.neighbors(from as usize) {
        if !walkable(world, nb) || world.tiles[nb].fire > 0.0 {
            continue;
        }
        let d = cheb(world, nb as u32, danger);
        if d > best_d {
            best_d = d;
            best = nb as u32;
        }
    }
    best
}

fn facing(world: &World, from: u32, to: u32, old: bool) -> bool {
    let (fx, fy) = world.xy(from as usize);
    let (tx, ty) = world.xy(to as usize);
    // Bildschirm-x in der Isometrie: x − y
    let dx = (tx as i32 - ty as i32) - (fx as i32 - fy as i32);
    if dx > 0 {
        true
    } else if dx < 0 {
        false
    } else {
        old
    }
}

pub fn step_day(
    a: &mut Animals,
    world: &mut World,
    weather: &Weather,
    day_of_year: u32,
    rng: &mut Rng,
) -> AnimalDay {
    let n = world.tiles.len();
    a.ensure(n);
    let mut day = AnimalDay::default();
    let alive_before: [bool; 4] = {
        let mut v = [false; 4];
        for s in Species::ALL {
            v[s.index()] = match s {
                Species::Rabbit => a.rabbit_total() > 0.0,
                _ => a.list.iter().any(|x| x.species == s),
            };
        }
        v
    };

    step_rabbits(a, world, weather, day_of_year);
    if !a.list.is_empty() {
        step_individuals(a, world, weather, rng);
        wolves_feed(a, world);
        births(a, world, day_of_year, rng);
    }

    // Kadaver zerfallen
    for c in a.carcasses.iter_mut() {
        c.days_left = c.days_left.saturating_sub(1);
    }
    a.carcasses.retain(|c| c.days_left > 0);

    for s in Species::ALL {
        let alive = match s {
            Species::Rabbit => a.rabbit_total() > 0.0,
            _ => a.list.iter().any(|x| x.species == s),
        };
        if alive_before[s.index()] && !alive {
            day.extinct.push(s);
        }
    }
    day
}

/// Wie viele Kaninchen ein Feld ernährt: viel auf Wiesen, etwas im Gebüsch,
/// wenig im Wald (Waldränder, Lichtungen).
pub fn rabbit_capacity(t: &crate::world::Tile) -> f32 {
    if t.is_ocean() || t.surface_water > POND_MM {
        return 0.0;
    }
    RABBIT_K_GRASS * t.grass * (1.0 - 0.4 * t.forest) + RABBIT_K_SHRUB * t.shrub + 0.5
}

fn step_rabbits(a: &mut Animals, world: &mut World, weather: &Weather, doy: u32) {
    if a.rabbits.iter().all(|&r| r <= 0.0) {
        return;
    }
    let n = world.tiles.len();
    let breeding = (60..240).contains(&doy);
    let mut moved = vec![0.0f32; n];
    for i in 0..n {
        let r = a.rabbits[i];
        if r <= 0.0 {
            continue;
        }
        if !walkable(world, i) {
            // Überschwemmt: Die meisten flüchten auf trockene Nachbarfelder.
            let n_ok = world.neighbors(i).filter(|&j| walkable(world, j)).count();
            if n_ok > 0 {
                let share = r * 0.8 / n_ok as f32;
                for j in world.neighbors(i) {
                    if walkable(world, j) {
                        moved[j] += share;
                    }
                }
            }
            a.rabbits[i] = 0.0;
            continue;
        }
        let t = &mut world.tiles[i];
        let cap = rabbit_capacity(t);
        let temp = weather.temp_at(t.elevation);
        let mut r2 = r;
        // Fressen
        let eat = (r * RABBIT_EAT).min(t.grass * 0.3);
        t.grass -= eat;
        // Vermehrung (wo es mehr als vereinzelte Tiere gibt), Sterben, Hunger
        if breeding && temp > 5.0 && r >= 0.5 && cap > r {
            r2 += RABBIT_GROWTH * r * (1.0 - r / cap);
        }
        r2 -= 0.001 * r;
        if temp < 2.0 {
            r2 -= 0.002 * r;
        }
        if r > cap {
            r2 -= 0.01 * (r - cap);
        }
        if t.fire > 0.0 {
            r2 *= 0.3;
        }
        // Abwandern, sobald es eng wird – auch wenige Tiere besiedeln so
        // nach und nach die ganze Insel
        if r2 > 1.0 && r2 > 0.5 * cap {
            // Junge Tiere ziehen zum Nachbarfeld mit dem meisten freien Platz.
            let mut best: Option<(f32, usize)> = None;
            for j in world.neighbors(i) {
                if !walkable(world, j) {
                    continue;
                }
                let room = rabbit_capacity(&world.tiles[j]) - a.rabbits[j] - moved[j];
                if best.is_none_or(|(b, _)| room > b) {
                    best = Some((room, j));
                }
            }
            if let Some((room, j)) = best
                && room > 0.0
            {
                let out = (RABBIT_DISPERSAL * r2).min(room);
                moved[j] += out;
                r2 -= out;
            }
        }
        a.rabbits[i] = r2.max(0.0);
    }
    let mut total = 0.0;
    for (r, m) in a.rabbits.iter_mut().zip(&moved) {
        *r += m;
        // Verstreute Einzeltiere ohne Partner verschwinden.
        if *r < 0.05 {
            *r = 0.0;
        }
        total += *r;
    }
    if total < 1.0 {
        a.rabbits.iter_mut().for_each(|r| *r = 0.0);
    }
}

fn step_individuals(a: &mut Animals, world: &mut World, weather: &Weather, rng: &mut Rng) {
    // Lage zu Tagesbeginn
    let wolves: Vec<u32> = a
        .list
        .iter()
        .filter(|x| x.species == Species::Wolf)
        .map(|x| x.tile)
        .collect();
    let deer_total = a.list.iter().filter(|x| x.species == Species::Deer).count() as f32;
    let mut deer_on = vec![0u16; world.tiles.len()];
    for x in a.list.iter().filter(|x| x.species == Species::Deer) {
        deer_on[x.tile as usize] += 1;
    }
    let mut dead: Vec<Option<Cause>> = vec![None; a.list.len()];

    for k in 0..a.list.len() {
        if dead[k].is_some() {
            continue;
        }
        let me = a.list[k].clone();
        let s = me.species;
        let mut tile = me.tile;
        // Junge brauchen weniger, solange sie bei der Mutter sind; danach
        // sind Jährlinge und Alte empfindlicher. Hirsche sparen im Winter.
        let mut metab = s.metabolism();
        if me.age_days < s.follow_days() {
            metab *= 0.5;
        } else if me.age_days < 365 || me.age_days * 10 > s.max_age() * 7 {
            metab *= 1.3;
        }
        if s == Species::Deer && weather.temp_at(world.tiles[tile as usize].elevation) < 5.0 {
            metab *= 0.7;
        }
        let mut energy = me.energy - metab;
        let mut activity = Activity::Rest;

        // Junge folgen der Mutter
        // Die Liste ist nach `id` sortiert: Mutter per Binärsuche finden.
        let mother_tile = if me.mother != 0 && me.age_days < s.follow_days() {
            a.list
                .binary_search_by_key(&me.mother, |m| m.id)
                .ok()
                .filter(|&j| dead[j].is_none())
                .map(|j| a.list[j].tile)
        } else {
            None
        };

        match s {
            Species::Deer => {
                let nearest_wolf = wolves
                    .iter()
                    .copied()
                    .min_by_key(|&w| (cheb(world, w, tile), w));
                // Nicht jeder Wolf wird rechtzeitig bemerkt.
                let threat = nearest_wolf
                    .filter(|&w| cheb(world, w, tile) <= 1)
                    .filter(|_| rng.chance(0.7));
                if let Some(w) = threat {
                    // Flucht: zwei Schritte weg
                    let t1 = step_away(world, tile, w);
                    let t2 = step_away(world, t1, w);
                    tile = t2;
                    activity = Activity::Run;
                    energy -= 0.01;
                } else if let Some(mt) = mother_tile {
                    if mt != tile {
                        tile = step_towards(world, tile, mt);
                        activity = Activity::Walk;
                    }
                } else {
                    // Futter suchen, beieinander bleiben, Wölfe meiden
                    let score = |j: usize, deer_on: &[u16]| -> f32 {
                        let t = &world.tiles[j];
                        let mut v = deer_forage(t);
                        if deer_on[j] > 0 && deer_on[j] < 10 {
                            v += 0.15;
                        }
                        if let Some(w) = nearest_wolf
                            && cheb(world, w, j as u32) <= 2
                        {
                            v -= 0.5;
                        }
                        if t.fire > 0.0 {
                            v -= 5.0;
                        }
                        v
                    };
                    let here_food = deer_forage(&world.tiles[tile as usize]);
                    if here_food < DEER_SEARCH_BELOW {
                        // Hier ist kaum noch etwas: weiter umsehen
                        let (cx, cy) = world.xy(tile as usize);
                        let mut target = None;
                        let mut best_v = here_food + 0.05;
                        // Hungrige Tiere suchen weiter
                        let reach = if energy < 0.5 {
                            DEER_SEARCH_HUNGRY
                        } else {
                            DEER_SEARCH
                        };
                        for dy in -reach..=reach {
                            for dx in -reach..=reach {
                                let (x, y) = (cx as i32 + dx, cy as i32 + dy);
                                if x < 0
                                    || y < 0
                                    || x >= world.width as i32
                                    || y >= world.height as i32
                                {
                                    continue;
                                }
                                let j = world.idx(x as u32, y as u32);
                                if !walkable(world, j) {
                                    continue;
                                }
                                let v = score(j, &deer_on) - 0.02 * dx.abs().max(dy.abs()) as f32;
                                if v > best_v {
                                    best_v = v;
                                    target = Some(j as u32);
                                }
                            }
                        }
                        let t1 = match target {
                            Some(tg) => step_towards(world, tile, tg),
                            None => {
                                // Nichts in Sicht: aufs Geratewohl weiterziehen
                                let nb: Vec<usize> = world
                                    .neighbors(tile as usize)
                                    .filter(|&j| walkable(world, j))
                                    .collect();
                                if nb.is_empty() {
                                    tile
                                } else {
                                    nb[rng.below(nb.len() as u32) as usize] as u32
                                }
                            }
                        };
                        if t1 != tile {
                            deer_on[tile as usize] = deer_on[tile as usize].saturating_sub(1);
                            deer_on[t1 as usize] += 1;
                            tile = t1;
                            activity = Activity::Walk;
                        }
                    }
                    let here = score(tile as usize, &deer_on) + 0.2;
                    let mut best = (here + rng.next_f32() * 0.1, tile);
                    for nb in world.neighbors(tile as usize) {
                        if !walkable(world, nb) {
                            continue;
                        }
                        let v = score(nb, &deer_on) + rng.next_f32() * 0.1;
                        if v > best.0 {
                            best = (v, nb as u32);
                        }
                    }
                    if activity == Activity::Rest && best.1 != tile {
                        deer_on[tile as usize] = deer_on[tile as usize].saturating_sub(1);
                        deer_on[best.1 as usize] += 1;
                        tile = best.1;
                        activity = Activity::Walk;
                    }
                }
                let fed = deer_eat(&mut world.tiles[tile as usize]);
                energy += 0.04 * fed;
                if activity == Activity::Rest && fed > 0.2 {
                    activity = Activity::Graze;
                }
            }
            Species::Wolf => {
                let hungry = energy < WOLF_HUNGRY;
                // Beute auf dem eigenen oder einem Nachbarfeld?
                let mut victim = if hungry {
                    find_victim(&a.list, &dead, world, tile)
                } else {
                    None
                };
                if victim.is_none() {
                    // Kaninchen nebenbei, wenn hungrig
                    let r = a.rabbits[tile as usize];
                    if energy < 0.7 && r >= 1.0 && rng.chance((r / 8.0).min(0.5)) {
                        a.rabbits[tile as usize] -= 1.0;
                        energy += 0.12;
                    }
                    if let Some(mt) = mother_tile {
                        if mt != tile {
                            tile = step_towards(world, tile, mt);
                            activity = Activity::Walk;
                        }
                    } else if hungry {
                        // Nächsten Hirsch auf dem Landweg suchen (um Seen herum)
                        let target = path_to(world, tile, WOLF_SEARCH as u32, |j| deer_on[j] > 0);
                        if let Some((s1, s2, d)) = target {
                            // Aus der Ferne anschleichen, aus der Nähe hetzen
                            let chase = d <= 4;
                            tile = if chase || d > 10 { s2 } else { s1 };
                            activity = if chase { Activity::Run } else { Activity::Walk };
                            victim = find_victim(&a.list, &dead, world, tile);
                        } else {
                            // Umherstreifen, beim Rudel bleiben
                            let other = wolves
                                .iter()
                                .copied()
                                .filter(|&w| w != me.tile)
                                .min_by_key(|&w| (cheb(world, w, tile), w));
                            let nb: Vec<usize> = world
                                .neighbors(tile as usize)
                                .filter(|&j| walkable(world, j))
                                .collect();
                            if let Some(o) = other.filter(|&o| cheb(world, o, tile) > 3) {
                                tile = step_towards(world, tile, o);
                            } else if !nb.is_empty() {
                                tile = nb[rng.below(nb.len() as u32) as usize] as u32;
                            }
                            activity = Activity::Walk;
                        }
                    }
                }
                if let Some((j, ve, young, vt)) = victim {
                    let pack = wolves.iter().filter(|&&w| cheb(world, w, vt) <= 1).count() as f32;
                    tile = vt;
                    let p = (0.15
                        + 0.3 * (1.0 - ve)
                        + if young { 0.2 } else { 0.0 }
                        + 0.08 * (pack - 1.0).clamp(0.0, 3.0))
                    .min(0.8);
                    // Wenige Hirsche sind wachsam und verstecken sich gut:
                    // Je seltener die Beute, desto öfter misslingt die Jagd.
                    let p = p * deer_total / (deer_total + PREY_HALF);
                    activity = Activity::Run;
                    energy -= 0.01;
                    if rng.chance(p) {
                        dead[j] = Some(Cause::Killed);
                        a.carcasses.push(Carcass {
                            tile,
                            species: Species::Deer,
                            days_left: 20,
                            meat: DEER_MEAT,
                        });
                    }
                }
            }
            Species::Fox => {
                // Füchse fressen auch Mäuse, Insekten und Beeren – das reicht
                // zum Überleben im Sommer, aber nicht für Junge im Winter.
                let t = &world.tiles[tile as usize];
                let warm = if weather.temp_at(t.elevation) > 8.0 {
                    1.0
                } else {
                    0.6
                };
                energy += FOX_SIDE_FOOD * warm * clamp01(t.grass + t.shrub);
                let r = a.rabbits[tile as usize];
                // Jagdrevier: das eigene Feld und die Nachbarfelder
                let mut best_nb = (r, tile);
                let mut around = r;
                for j in world.neighbors(tile as usize) {
                    around += a.rabbits[j];
                    if a.rabbits[j] > best_nb.0 {
                        best_nb = (a.rabbits[j], j as u32);
                    }
                }
                let p = (around / FOX_CATCH_HALF).min(0.7);
                if best_nb.0 >= 1.0 && energy < 0.9 && rng.chance(p) {
                    a.rabbits[best_nb.1 as usize] -= 1.0;
                    tile = best_nb.1;
                    energy += 0.3;
                    activity = Activity::Run;
                } else if let Some(c) = a
                    .carcasses
                    .iter_mut()
                    .find(|c| c.tile == tile && c.days_left > 2)
                    && energy < 0.8
                {
                    c.days_left -= 2;
                    energy += 0.2;
                    activity = Activity::Graze;
                } else if let Some(mt) = mother_tile {
                    if mt != tile {
                        tile = step_towards(world, tile, mt);
                        activity = Activity::Walk;
                    }
                } else if energy < 0.9 {
                    // Zum Feld mit den meisten Kaninchen – in der Nähe oder,
                    // wenn hier keine sind, weiter weg
                    // Lohnt das Revier nicht mehr (wenig Beute), weiterziehen
                    if around < 4.0 {
                        let (cx, cy) = world.xy(tile as usize);
                        let mut target = None;
                        let mut best_v = 1.0f32;
                        for dy in -FOX_SEARCH..=FOX_SEARCH {
                            for dx in -FOX_SEARCH..=FOX_SEARCH {
                                let (x, y) = (cx as i32 + dx, cy as i32 + dy);
                                if x < 0
                                    || y < 0
                                    || x >= world.width as i32
                                    || y >= world.height as i32
                                {
                                    continue;
                                }
                                let j = world.idx(x as u32, y as u32);
                                let v = a.rabbits[j] - 0.1 * dx.abs().max(dy.abs()) as f32;
                                if v > best_v {
                                    best_v = v;
                                    target = Some(j as u32);
                                }
                            }
                        }
                        let t1 = match target {
                            Some(tg) => step_towards(world, tile, tg),
                            None => random_step(world, tile, rng),
                        };
                        let t2 = match target {
                            Some(tg) => step_towards(world, t1, tg),
                            None => t1,
                        };
                        tile = t2;
                        activity = Activity::Walk;
                    }
                    let mut best = (a.rabbits[tile as usize] + rng.next_f32() * 0.5, tile);
                    for nb in world.neighbors(tile as usize) {
                        if !walkable(world, nb) {
                            continue;
                        }
                        let v = a.rabbits[nb] + rng.next_f32() * 0.5;
                        if v > best.0 {
                            best = (v, nb as u32);
                        }
                    }
                    if best.1 != tile {
                        tile = best.1;
                        activity = Activity::Walk;
                    }
                }
            }
            Species::Rabbit => {}
        }

        // Feuer: Flucht, sonst Tod
        if world.tiles[tile as usize].fire > 0.0 {
            let away = world
                .neighbors(tile as usize)
                .find(|&j| walkable(world, j) && world.tiles[j].fire <= 0.0);
            match away {
                Some(j) => {
                    tile = j as u32;
                    activity = Activity::Run;
                }
                None => {
                    if rng.chance(0.5) {
                        dead[k] = Some(Cause::Fire);
                    }
                }
            }
        }

        let x = &mut a.list[k];
        x.facing_right = facing(world, x.tile, tile, x.facing_right);
        x.prev_tile = x.tile;
        x.tile = tile;
        // Wölfe schlingen am Riss mehr, als sie täglich brauchen (Vorrat).
        x.energy = energy.min(if s == Species::Wolf { WOLF_GORGE } else { 1.0 });
        x.activity = activity;
        x.age_days += 1;

        if dead[k].is_none() {
            let max = s.max_age();
            let old = x.age_days * 10 > max * 7;
            if x.energy <= 0.0 {
                dead[k] = Some(Cause::Starved);
            } else if x.age_days >= max || (old && rng.chance(0.002)) {
                dead[k] = Some(Cause::Old);
            } else if rng.chance(if x.is_young() { 0.0005 } else { 0.0001 }) {
                dead[k] = Some(Cause::Other);
            }
        }
    }

    // Tote entfernen (Reihenfolge bleibt erhalten)
    let mut idx = 0;
    let (deaths, since, carcasses) = (&mut a.deaths, &mut a.deaths_since, &mut a.carcasses);
    a.list.retain(|x| {
        let d = dead[idx];
        idx += 1;
        if let Some(c) = d {
            deaths.add(x.species, c);
            since.add(x.species, c);
            if c != Cause::Killed {
                carcasses.push(Carcass {
                    tile: x.tile,
                    species: x.species,
                    days_left: 12,
                    meat: if x.species == Species::Deer {
                        DEER_MEAT * 0.5
                    } else {
                        0.0
                    },
                });
            }
            false
        } else {
            true
        }
    });
}

/// Wölfe fressen an Rissen in ihrer Nähe – das ganze Rudel, nicht nur der
/// Jäger.
fn wolves_feed(a: &mut Animals, world: &World) {
    let (list, carcasses) = (&mut a.list, &mut a.carcasses);
    for w in list.iter_mut().filter(|x| x.species == Species::Wolf) {
        if w.energy >= WOLF_GORGE {
            continue;
        }
        // Welpen werden vom Rudel mitversorgt.
        let reach = if w.age_days < Species::Wolf.follow_days() {
            4
        } else {
            1
        };
        if let Some(c) = carcasses
            .iter_mut()
            .find(|c| c.meat > 0.0 && cheb(world, c.tile, w.tile) <= reach)
        {
            let take = (WOLF_GORGE - w.energy).min(0.5).min(c.meat);
            c.meat -= take;
            w.energy += take;
        }
    }
}

/// Schwächster Hirsch auf dem Feld oder daneben (Junge und Entkräftete zuerst).
fn find_victim(
    list: &[Animal],
    dead: &[Option<Cause>],
    world: &World,
    tile: u32,
) -> Option<(usize, f32, bool, u32)> {
    list.iter()
        .enumerate()
        .filter(|(j, x)| {
            x.species == Species::Deer && cheb(world, x.tile, tile) <= 1 && dead[*j].is_none()
        })
        .min_by(|(_, p), (_, q)| {
            let wp = p.energy - if p.is_young() { 0.5 } else { 0.0 };
            let wq = q.energy - if q.is_young() { 0.5 } else { 0.0 };
            wp.total_cmp(&wq).then(p.id.cmp(&q.id))
        })
        .map(|(j, x)| (j, x.energy, x.is_young(), x.tile))
}

fn births(a: &mut Animals, world: &World, doy: u32, rng: &mut Rng) {
    let season = |s: Species| match s {
        Species::Deer => 140,
        Species::Wolf => 110,
        Species::Fox => 90,
        Species::Rabbit => u32::MAX,
    };
    let mut newborn = Vec::new();
    for s in [Species::Deer, Species::Wolf, Species::Fox] {
        if doy != season(s) {
            continue;
        }
        let range = match s {
            Species::Deer => 12,
            Species::Wolf => 4,
            _ => 8,
        };
        let mut litters = 0u32;
        let adults: Vec<&Animal> = a
            .list
            .iter()
            .filter(|x| x.species == s && x.age_days >= s.adult_age())
            .collect();
        for mother in adults.iter().filter(|x| x.sex == Sex::Female) {
            let partner = adults
                .iter()
                .any(|m| m.sex == Sex::Male && cheb(world, m.tile, mother.tile) <= range);
            if !partner {
                continue;
            }
            // Im Wolfsrudel bekommt nur die älteste Wölfin Junge.
            if s == Species::Wolf
                && adults.iter().any(|o| {
                    o.sex == Sex::Female
                        && o.id != mother.id
                        && cheb(world, o.tile, mother.tile) <= range
                        && (o.age_days, o.id) > (mother.age_days, mother.id)
                })
            {
                continue;
            }
            let max_litters = match s {
                Species::Wolf => WOLF_LITTERS_PER_YEAR,
                Species::Fox => FOX_LITTERS_PER_YEAR,
                _ => u32::MAX,
            };
            if litters >= max_litters {
                continue;
            }
            let (min_energy, litter) = match s {
                Species::Deer => (0.35, 1),
                Species::Wolf => (0.6, 2 + rng.below(3)),
                _ => (0.4, 2 + rng.below(3)),
            };
            if mother.energy < min_energy || !rng.chance(0.85) {
                continue;
            }
            litters += 1;
            for _ in 0..litter {
                newborn.push((s, mother.id, mother.tile, mother.variant));
            }
        }
    }
    for (s, mother, tile, variant) in newborn {
        a.next_id += 1;
        a.births[s.index()] += 1;
        let v = if rng.chance(0.7) {
            variant
        } else {
            rng.below(2) as u8
        };
        let sex = if rng.chance(0.5) {
            Sex::Female
        } else {
            Sex::Male
        };
        a.list.push(Animal {
            id: a.next_id,
            species: s,
            sex,
            tile,
            prev_tile: tile,
            age_days: 0,
            energy: 0.6,
            activity: Activity::Rest,
            facing_right: rng.chance(0.5),
            variant: v,
            mother,
        });
    }
}
