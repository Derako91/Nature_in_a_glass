//! Wasserkreislauf: Regen, Quellen, Versickern, Abfluss, Seen, Verdunstung,
//! Pflanzenwasser, Tiefenwasser, Grundwasser. Ein Aufruf entspricht einem Tag.
//!
//! Wasserspeicher je Feld: Oberfläche → Wurzelraum (Bodenfeuchte) →
//! tiefer Boden (nur für tiefe Wurzeln) → gemeinsames Grundwasser der Quellen.

use crate::climate::{Weather, potential_evaporation};
use crate::math::clamp01;
use crate::world::World;

/// Höchstens so viel Wasser (mm) versickert pro Tag in einer Kachel.
const INFILTRATION_MM: f32 = 40.0;
/// Dünner Wasserfilm, der nicht abfließt (mm).
const FILM_MM: f32 = 2.0;
/// Anteil des Sickerwassers unter dem tiefen Boden, der das Grundwasser der
/// Quellen erreicht (der Rest versickert unerreichbar in die Tiefe).
const RECHARGE_FRACTION: f32 = 0.8;
/// Tägliche Entleerung des Grundwasserkörpers zum Meer hin.
const AQUIFER_LEAK: f32 = 0.0015;
/// Tägliches Absickern aus dem tiefen Boden ins Grundwasser.
const DEEP_DRAIN: f32 = 0.001;
/// Wie viel des Tiefenwassers Wurzeln pro Tag höchstens erschließen.
const DEEP_UPTAKE: f32 = 0.03;

/// Wasserbilanz eines Tages. Alle Mengen als Summe in mm über alle Felder.
#[derive(Debug, Clone, Copy, Default)]
pub struct HydroDay {
    /// Mittlerer Niederschlag je Landkachel (mm).
    pub rain_mm: f32,
    pub rain_total: f32,
    /// Wasser, das ins Meer floss.
    pub to_sea: f32,
    /// Verdunstung von offenen Wasserflächen.
    pub evaporation: f32,
    /// Verdunstung über Boden und Pflanzen.
    pub transpiration: f32,
    /// Versickerung in unerreichbare Tiefe.
    pub deep_loss: f32,
    /// Abfluss des Grundwassers unterirdisch ins Meer.
    pub aquifer_leak: f32,
    /// Neubildung des Grundwassers.
    pub recharge: f32,
}

/// Wasserbedarf der Bäume relativ zur Verdunstung: dichter, alter Wald
/// braucht mehr als lichter Jungwuchs.
pub fn forest_demand_factor(forest: f32, age: f32) -> f32 {
    0.8 * forest * (0.6 + 0.4 * clamp01(age / 60.0))
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
        day.rain_total += r;
        land += 1;
    }
    if land > 0 {
        day.rain_mm = day.rain_total / land as f32;
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

    // 4. Verdunstung, Pflanzenwasser, Sickerwasser
    let mut recharge = 0.0;
    for (t, &passed) in world.tiles.iter_mut().zip(through.iter()) {
        if t.is_ocean() {
            continue;
        }
        t.flow = t.flow * 0.85 + passed * 0.15;
        let pet = potential_evaporation(weather.temp_at(t.elevation));
        let evap = t.surface_water.min(pet * 1.1);
        t.surface_water -= evap;
        day.evaporation += evap;

        // Bedarf: offener Boden, Gras, Busch und Bäume
        let w_frac = t.moisture_fraction();
        let tree_need = forest_demand_factor(t.forest, t.forest_age);
        let need = 0.25 + 0.45 * t.grass + 0.55 * t.shrub + tree_need;
        let et = (pet * need * w_frac).min(t.soil_moisture);
        t.soil_moisture -= et;
        day.transpiration += et;

        // Bäume holen den Rest – soweit ihre Wurzeln reichen – aus der Tiefe
        let reach = t.root_reach();
        if tree_need > 0.0 {
            let wanted = pet * tree_need;
            let from_soil = wanted * w_frac;
            let deficit = (wanted - from_soil).max(0.0);
            let take = deficit
                .min(t.deep_moisture * reach * DEEP_UPTAKE)
                .min(t.deep_moisture);
            t.deep_moisture -= take;
            day.transpiration += take;
            t.tree_supply = clamp01((from_soil + take) / wanted.max(1e-6));
        } else {
            // Ohne Wald: Aussicht für Sämlinge (flache Wurzeln)
            t.tree_supply = clamp01(w_frac + 0.5 * reach * t.deep_fraction());
        }

        // Sickerwasser füllt zuerst den tiefen Boden, dann das Grundwasser
        let wet = t.soil_capacity * 0.6;
        if t.soil_moisture > wet {
            let perc = (t.soil_moisture - wet) * 0.12;
            t.soil_moisture -= perc;
            let room = (t.deep_capacity() - t.deep_moisture).max(0.0);
            let to_deep = perc.min(room);
            t.deep_moisture += to_deep;
            let below = perc - to_deep;
            recharge += below * RECHARGE_FRACTION;
            day.deep_loss += below * (1.0 - RECHARGE_FRACTION);
        }
        // Der tiefe Boden gibt langsam an das Grundwasser ab
        let drain = t.deep_moisture * DEEP_DRAIN;
        t.deep_moisture -= drain;
        recharge += drain;
    }
    *aquifer += recharge;
    let leak = *aquifer * AQUIFER_LEAK;
    *aquifer -= leak;
    day.aquifer_leak = leak;
    day.recharge = recharge;
    day
}

/// Gesamtes Wasser auf und in der Insel (mm-Summe über alle Felder).
pub fn total_water(world: &World, aquifer: f32) -> f64 {
    let mut sum = aquifer as f64;
    for t in &world.tiles {
        if !t.is_ocean() {
            sum += (t.surface_water + t.soil_moisture + t.deep_moisture) as f64;
        }
    }
    sum
}
