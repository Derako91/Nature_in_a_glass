//! Feuer: Blitzschlag, Zündung, Ausbreitung, Abbrand. Ein Aufruf = ein Tag.
//!
//! Ein Feuer braucht eine Ursache (Blitz) und Brennmaterial, das trocken
//! genug ist. Es breitet sich mit dem Wind und bergauf schneller aus,
//! Bäche, Seen, Fels und nasse Böden bremsen es, Regen löscht es.

use crate::climate::Weather;
use crate::math::clamp01;
use crate::rng::Rng;
use crate::sim::STREAM_FLOW;
use crate::world::{POND_MM, Tile, World};

/// Brennmaterial eines Feldes (0 … etwa 1,5).
pub fn fuel(t: &Tile) -> f32 {
    t.grass * 0.35 + t.shrub * 0.7 + t.forest * 0.6 + t.deadwood * 1.2
}

/// Wie trocken das Brennmaterial ist (0 = nass, 1 = staubtrocken).
pub fn dryness(t: &Tile) -> f32 {
    if t.surface_water > 5.0 {
        return 0.0;
    }
    clamp01((0.35 - t.moisture_fraction()) / 0.3)
}

/// Brennbarkeit = Brennmaterial × Trockenheit.
pub fn flammability(t: &Tile) -> f32 {
    if t.is_ocean() {
        return 0.0;
    }
    fuel(t) * dryness(t)
}

/// Wo ein Blitz eingeschlagen hat und ob er gezündet hat.
#[derive(Debug, Clone)]
pub struct Strike {
    pub tile: usize,
    pub ignited: bool,
    pub fuel: f32,
    pub dryness: f32,
}

#[derive(Debug, Clone, Default)]
pub struct FireDay {
    pub strikes: Vec<Strike>,
    /// Felder, die heute neu Feuer gefangen haben.
    pub newly_burning: u32,
    /// Felder, die heute brennen.
    pub burning: u32,
    /// Ob Regen heute Feuer gelöscht hat.
    pub rain_doused: bool,
}

#[derive(Debug, Default, Clone)]
pub struct Scratch {
    next: Vec<f32>,
}

/// Blitzeinschläge: höher gelegene Felder werden häufiger getroffen.
fn lightning(world: &mut World, weather: &Weather, rng: &mut Rng, day: &mut FireDay) {
    let n = world.tiles.len() as u32;
    for _ in 0..weather.lightning_strikes {
        // Zwei Kandidaten, der höhere zieht den Blitz an
        let a = rng.below(n) as usize;
        let b = rng.below(n) as usize;
        let roll = rng.next_f32();
        let i = if world.tiles[a].elevation >= world.tiles[b].elevation {
            a
        } else {
            b
        };
        let t = &world.tiles[i];
        if t.is_ocean() {
            continue;
        }
        let (x, y) = world.xy(i);
        let rain_here = weather.rain_at(x, y, t.elevation / world.max_elevation, world.width);
        let wet_factor = if rain_here > 6.0 { 0.1 } else { 1.0 };
        let p = clamp01(flammability(t) * 0.3 * wet_factor);
        let ignited = roll < p && t.fire <= 0.0;
        day.strikes.push(Strike {
            tile: i,
            ignited,
            fuel: fuel(t),
            dryness: dryness(t),
        });
        if ignited {
            world.tiles[i].fire = 1.0;
            day.newly_burning += 1;
        }
    }
}

pub fn step_day(
    world: &mut World,
    weather: &Weather,
    rng: &mut Rng,
    scratch: &mut Scratch,
) -> FireDay {
    let mut day = FireDay::default();
    lightning(world, weather, rng, &mut day);

    let n = world.tiles.len();
    scratch.next.clear();
    scratch.next.resize(n, 0.0);
    let (wx, wy) = weather.wind_vector();
    let wind = weather.wind_speed;
    let (w, max_e) = (world.width, world.max_elevation);

    for i in 0..n {
        let intensity = world.tiles[i].fire;
        if intensity <= 0.0 {
            continue;
        }
        day.burning += 1;
        let (x, y) = world.xy(i);
        let e = world.tiles[i].elevation;
        let rain_here = weather.rain_at(x, y, e / max_e, w);
        let doused = rain_here > 8.0;
        if doused {
            day.rain_doused = true;
        }

        // Ausbreitung auf Nachbarn (feste Reihenfolge → reproduzierbar)
        if !doused {
            let ns: Vec<usize> = world.neighbors(i).collect();
            for nb in ns {
                let roll = rng.next_f32();
                let tn = &world.tiles[nb];
                if tn.is_ocean() || tn.fire > 0.0 || scratch.next[nb] > 0.0 {
                    continue;
                }
                let (nx, ny) = world.xy(nb);
                let dx = nx as f32 - x as f32;
                let dy = ny as f32 - y as f32;
                let len = (dx * dx + dy * dy).sqrt();
                let along = (dx * wx + dy * wy) / len; // −1 gegen den Wind … +1 mit dem Wind
                let wind_factor = (1.0 + 1.6 * wind * along).clamp(0.15, 3.0);
                let slope_factor = (1.0 + (tn.elevation - e) / 40.0).clamp(0.5, 2.0);
                let mut p = 0.22 * intensity * flammability(tn) * wind_factor * slope_factor;
                if tn.flow > STREAM_FLOW || tn.surface_water > POND_MM {
                    p *= 0.15; // Bäche und Teiche als Brandschneise
                }
                if roll < p.clamp(0.0, 0.95) {
                    scratch.next[nb] = 0.9;
                    day.newly_burning += 1;
                }
            }
        }

        // Abbrand
        let t = &mut world.tiles[i];
        let c = 0.5 + 0.4 * dryness(t);
        t.deadwood -= t.deadwood * c;
        let forest_loss = t.forest * c * 0.8;
        t.forest -= forest_loss;
        t.deadwood += forest_loss * 0.35; // verkohlte Stämme bleiben stehen
        t.shrub -= t.shrub * c;
        t.grass -= t.grass * 0.95;
        if t.forest < 0.005 {
            t.forest = 0.0;
            t.forest_age = 0.0;
        }
        t.burn_scar = 1.0;
        let remaining = fuel(t);
        let next = if doused || remaining < 0.08 {
            0.0
        } else {
            (remaining * 0.9).min(1.0)
        };
        scratch.next[i] = scratch.next[i].max(next);
    }

    for i in 0..n {
        world.tiles[i].fire = scratch.next[i];
    }
    day
}

/// Nur für Tests und Experimente: ein Feld direkt entzünden.
#[doc(hidden)]
pub fn ignite_for_test(world: &mut World, i: usize) {
    world.tiles[i].fire = 1.0;
}
