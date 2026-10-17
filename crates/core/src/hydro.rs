//! Wasserkreislauf: Regen, Quellen, Versickern, Abfluss, Seen, Verdunstung,
//! Grundwasser. Ein Aufruf entspricht einem Tag.

use crate::climate::{potential_evaporation, Weather};
use crate::world::World;

/// Höchstens so viel Wasser (mm) versickert pro Tag in einer Kachel.
const INFILTRATION_MM: f32 = 40.0;
/// Dünner Wasserfilm, der nicht abfließt (mm).
const FILM_MM: f32 = 2.0;
/// Anteil des Sickerwassers, der das Grundwasser der Quellen erreicht.
const RECHARGE_FRACTION: f32 = 0.6;
/// Tägliche Entleerung des Grundwasserkörpers zum Meer hin.
const AQUIFER_LEAK: f32 = 0.0015;

#[derive(Debug, Clone, Copy, Default)]
pub struct HydroDay {
    /// Mittlerer Niederschlag je Landkachel (mm).
    pub rain_mm: f32,
    /// Wasser, das heute ins Meer floss (Summe mm über Kacheln).
    pub to_sea: f32,
    /// Grundwasserneubildung (Summe mm über Kacheln).
    pub recharge: f32,
}

pub fn step_day(
    world: &mut World,
    weather: &Weather,
    aquifer: &mut f32,
    aquifer_ref: f32,
    through: &mut [f32],
) -> HydroDay {
    let mut day = HydroDay::default();
    let (w, max_e) = (world.width, world.max_elevation);
    let mut land = 0u32;

    // 1. Niederschlag
    for i in 0..world.tiles.len() {
        if world.tiles[i].is_ocean() {
            continue;
        }
        let (x, y) = world.xy(i);
        let rel = world.tiles[i].elevation / max_e;
        let r = weather.rain_at(x, y, rel, w);
        world.tiles[i].surface_water += r;
        day.rain_mm += r;
        land += 1;
    }
    if land > 0 {
        day.rain_mm /= land as f32;
    }

    // 2. Quellen speisen sich aus dem Grundwasser
    let fill = (*aquifer / aquifer_ref).clamp(0.0, 1.5);
    for s in world.springs.iter_mut() {
        let out = (s.base_rate * s.factor * fill).min(*aquifer).max(0.0);
        *aquifer -= out;
        s.current = out;
        let i = (s.y * w + s.x) as usize;
        world.tiles[i].surface_water += out;
    }

    // 3. Versickern und Abfluss bergab (oben → unten, ein Durchgang pro Tag)
    for v in through.iter_mut() {
        *v = 0.0;
    }
    for k in 0..world.drain_order.len() {
        let i = world.drain_order[k] as usize;
        if world.tiles[i].is_ocean() {
            continue;
        }
        let basin = world.basin_capacity[i];
        let t = &mut world.tiles[i];
        let room = (t.soil_capacity - t.soil_moisture).max(0.0);
        let inf = t.surface_water.min(INFILTRATION_MM).min(room);
        t.surface_water -= inf;
        t.soil_moisture += inf;

        let excess = t.surface_water - basin - FILM_MM;
        if excess > 0.0 {
            t.surface_water -= excess;
            through[i] = excess;
            match world.downstream[i] {
                Some(d) if !world.tiles[d as usize].is_ocean() => {
                    world.tiles[d as usize].surface_water += excess;
                }
                _ => day.to_sea += excess,
            }
        }
    }

    // 4. Verdunstung, Pflanzen, Sickerwasser
    let mut recharge = 0.0;
    for i in 0..world.tiles.len() {
        let t = &mut world.tiles[i];
        if t.is_ocean() {
            continue;
        }
        t.flow = t.flow * 0.85 + through[i] * 0.15;
        let pet = potential_evaporation(weather.temp_at(t.elevation));
        let evap = t.surface_water.min(pet * 1.1);
        t.surface_water -= evap;

        let plants = 0.25 + 0.45 * t.grass + 0.8 * t.forest;
        let et = (pet * plants * t.moisture_fraction()).min(t.soil_moisture);
        t.soil_moisture -= et;

        let wet = t.soil_capacity * 0.6;
        if t.soil_moisture > wet {
            let perc = (t.soil_moisture - wet) * 0.12;
            t.soil_moisture -= perc;
            recharge += perc * RECHARGE_FRACTION;
        }
    }
    *aquifer += recharge;
    *aquifer -= *aquifer * AQUIFER_LEAK;
    day.recharge = recharge;
    day
}
