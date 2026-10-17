//! Bodenabtrag, Transport und Ablagerung durch fließendes Wasser, dazu die
//! langsame Bodenbildung aus dem Gestein.
//!
//! Ablauf je Tag (nach dem Wasserkreislauf, in derselben Reihenfolge vom
//! höchsten zum tiefsten Wasserspiegel):
//! - Abfließendes Wasser löst Boden, je mehr Wasser, je steiler und je
//!   kahler der Boden. Pflanzen schützen stark, Brandflächen kaum.
//! - Wo die Bodenschicht fehlt, schneidet sich ein Bach langsam ins Gestein.
//! - Das Wasser trägt höchstens so viel, wie sein Gefälle erlaubt. Wird es
//!   langsamer (flaches Gelände, Seen), lagert es den Rest ab.
//! - Was das Meer erreicht, lagert sich auf dem Meeresboden vor der Küste ab.
//!
//! Die täglichen Beträge sind winzig. Sie werden je Feld gesammelt und erst
//! ins Gelände übernommen, wenn sie mindestens [`APPLY_STEP_M`] erreichen –
//! sonst würden sie bei großen Höhenwerten in der Rechengenauigkeit
//! verloren gehen. Bilanz: Was oben fehlt, liegt unten oder im Meer.

use serde::{Deserialize, Serialize};

use crate::climate::Weather;
use crate::hydro::head;
use crate::math::clamp01;
use crate::sim::{MIN_LAND_ELEVATION, STREAM_FLOW};
use crate::world::{POND_MM, TILE_SIZE_M, World, water_per_soil_meter};

/// Abtrag durch fließendes Wasser je (mm Durchfluss × Gefälle) auf kahlem Boden (m).
pub const DETACH_K: f32 = 6.0e-7;
/// Abspülung durch Regentropfen und flächigen Abfluss am Hang, je
/// (mm Regen)² × Gefälle auf kahlem Boden (m). Wirkt nur, wenn Wasser abfließt.
pub const SHEET_K: f32 = 3.0e-6;
/// Transportvermögen je (mm Durchfluss × Gefälle) (m Boden über ein Feld).
pub const CAPACITY_K: f32 = 1.0e-3;
/// Gestein ist viel härter als Boden.
pub const ROCK_FACTOR: f32 = 0.05;
/// Bodenbildung aus Gestein bei fehlender Bodenschicht (m pro Jahr).
pub const SOIL_PRODUCTION_M: f32 = 0.00015;
/// Ab dieser Dicke bildet sich kein neuer Boden mehr (m).
pub const SOIL_PRODUCTION_LIMIT: f32 = 1.0;
/// So hoch muss abgelagerter Boden vor der Küste reichen, bevor er als
/// neues Land auftaucht (m über dem Meeresspiegel).
pub const NEW_LAND_M: f32 = 0.3;
/// Gesammelte Änderungen werden ab diesem Betrag ins Gelände übernommen (m).
pub const APPLY_STEP_M: f32 = 0.001;
/// Ab diesem Verlust an einem Tag zählt ein Feld als stark abgespült (m).
pub const HEAVY_LOSS_M: f32 = 0.002;
/// Bodenmenge in m³, die eine Schicht von 1 m auf einem Feld hat.
pub const M3_PER_TILE_M: f32 = TILE_SIZE_M * TILE_SIZE_M;

/// Abtragsbilanz eines Tages (m Schicht, summiert über alle Felder).
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq)]
pub struct ErosionDay {
    /// Gelöster Boden und Gestein.
    pub eroded: f32,
    /// Auf Land abgelagert.
    pub deposited: f32,
    /// Ins Meer getragen (lagert sich vor der Küste ab).
    pub to_sea: f32,
    /// Felder, die heute mehr als 2 mm Boden verloren haben.
    pub heavy_tiles: u32,
    /// Davon auf Brandflächen.
    pub heavy_burned_tiles: u32,
    /// Feld mit dem stärksten Abtrag heute.
    pub worst_tile: Option<u32>,
}

/// Wie ungeschützt der Boden ist (0 = dicht bewachsen, 1 = kahl).
/// Schon mäßiger Bewuchs schützt viel, deshalb kubisch.
pub fn exposure(t: &crate::world::Tile) -> f32 {
    let cover = clamp01(1.1 * t.forest + 0.8 * t.shrub + 0.85 * t.grass + 0.15 * t.deadwood);
    let bare = 1.0 - cover;
    (bare * bare * bare).max(0.002)
}

#[derive(Debug, Clone, Default)]
pub struct Scratch {
    load: Vec<f32>,
    done: Vec<bool>,
}

/// Ein Tag Abtrag und Ablagerung. `through`: Wasser, das heute jedes Feld
/// verlassen hat (mm); `order`: Reihenfolge des Wasserkreislaufs.
pub fn step_day(
    world: &mut World,
    weather: &Weather,
    through: &[f32],
    order: &[u32],
    yearly_change: &mut [f32],
    s: &mut Scratch,
) -> ErosionDay {
    let n = world.tiles.len();
    if s.load.len() != n {
        s.load = vec![0.0; n];
        s.done = vec![false; n];
    }
    s.load.iter_mut().for_each(|v| *v = 0.0);
    s.done.iter_mut().for_each(|v| *v = false);
    let mut day = ErosionDay::default();
    let mut worst = 0.0f32;

    for &i in order {
        let i = i as usize;
        s.done[i] = true;
        let incoming = s.load[i];
        let q = through[i];
        let recv = world.downstream[i].map(|r| r as usize);
        let t = &world.tiles[i];

        // Gefälle des Wasserspiegels zum Hauptabfluss
        let slope = match recv {
            Some(r) if q > 0.0 => {
                let tr = &world.tiles[r];
                let h_r = if tr.is_ocean() { 0.0 } else { head(tr) };
                let (x, y) = world.xy(i);
                let (rx, ry) = world.xy(r);
                let dist = if rx != x && ry != y {
                    TILE_SIZE_M * std::f32::consts::SQRT_2
                } else {
                    TILE_SIZE_M
                };
                ((head(t) - h_r) / dist).clamp(0.0, 1.0)
            }
            _ => 0.0,
        };
        let power = q * slope;
        let capacity = CAPACITY_K * power;

        // Abtrag: nicht unter stehendem Wasser, nicht unter die Mindesthöhe
        let mut soil_out = 0.0;
        let mut rock_out = 0.0;
        if power > 0.0 && t.surface_water <= POND_MM {
            // Im Bachbett wächst nichts: viel Wasser legt den Grund frei.
            let channel = clamp01(q / STREAM_FLOW);
            let bare = exposure(t);
            let exp = bare + (1.0 - bare) * 0.5 * channel;
            let (x, y) = world.xy(i);
            let rain = weather.rain_at(x, y, t.elevation / world.max_elevation, world.width);
            let channel_wanted = DETACH_K * power * exp;
            let wanted = channel_wanted + SHEET_K * rain * rain * slope * bare;
            let soil = (t.soil_depth + t.soil_change).max(0.0);
            let room = (t.elevation + t.soil_change + t.rock_change - MIN_LAND_ELEVATION).max(0.0);
            soil_out = wanted.min(soil).min(room);
            // Fehlt der Boden, greift das Wasser – viel langsamer – das Gestein an.
            // Nur fließendes Wasser im Bett, nicht der Regen am Hang.
            if soil_out < wanted {
                let rest = (channel_wanted - soil_out).max(0.0) * ROCK_FACTOR;
                rock_out = rest.min(room - soil_out).max(0.0);
            }
        }
        let mut carry = incoming + soil_out + rock_out;
        let mut deposit = 0.0;
        if carry > capacity || recv.is_none() {
            let keep = if recv.is_none() { 0.0 } else { capacity };
            deposit = carry - keep;
            carry = keep;
        }
        {
            let t = &mut world.tiles[i];
            t.soil_change += deposit - soil_out;
            t.rock_change -= rock_out;
        }
        day.eroded += soil_out + rock_out;
        day.deposited += deposit;
        let loss = soil_out + rock_out - deposit;
        yearly_change[i] -= loss;
        if loss > HEAVY_LOSS_M {
            day.heavy_tiles += 1;
            if world.tiles[i].burn_scar > 0.3 {
                day.heavy_burned_tiles += 1;
            }
        }
        if loss > worst {
            worst = loss;
            day.worst_tile = Some(i as u32);
        }
        if carry > 0.0
            && let Some(r) = recv
        {
            if world.tiles[r].is_ocean() {
                // Im Meer verteilt sich die Fracht vor der Mündung.
                let mut spots = [0u32; 9];
                let mut k = 0;
                spots[k] = r as u32;
                k += 1;
                for nb in world.neighbors(r) {
                    if world.tiles[nb].is_ocean() {
                        spots[k] = nb as u32;
                        k += 1;
                    }
                }
                let part = carry / k as f32;
                for &o in &spots[..k] {
                    world.tiles[o as usize].soil_change += part;
                    yearly_change[o as usize] += part;
                }
                day.to_sea += carry;
            } else if s.done[r] {
                yearly_change[r] += carry;
                // Empfänger schon berechnet (Wasserspiegel hat sich im
                // Lauf des Tages verschoben): dort ablagern.
                world.tiles[r].soil_change += carry;
                day.deposited += carry;
            } else {
                s.load[r] += carry;
            }
        }
    }
    day
}

/// Übernimmt gesammelte Änderungen ins Gelände, bildet neuen Boden und passt
/// das Wasserhaltevermögen an. Einmal im Jahr. Gibt Felder zurück, die aus
/// dem Meer aufgetaucht sind (Schwemmland).
pub fn apply_year(world: &mut World, yearly_change: &mut [f32]) -> Vec<u32> {
    let mut new_land = Vec::new();
    for (i, t) in world.tiles.iter_mut().enumerate() {
        let was_ocean = t.is_ocean();
        let net = yearly_change[i];
        t.erosion_rate = t.erosion_rate * 0.9 + net * 0.1;
        yearly_change[i] = 0.0;

        // Bodenbildung: aus dem Gestein, schneller unter Wald, langsamer
        // je dicker die Schicht schon ist. Die Höhe bleibt dabei gleich.
        if !was_ocean {
            let room = clamp01(1.0 - t.soil_depth / SOIL_PRODUCTION_LIMIT);
            t.soil_depth += SOIL_PRODUCTION_M * room * (0.5 + 0.5 * t.forest);
        }

        let pending = t.soil_change + t.rock_change;
        // Vor der Küste wächst eine Sandbank unter Wasser heran; sie taucht
        // erst als Land auf, wenn sie deutlich über den Meeresspiegel reicht.
        let shoal = was_ocean && {
            let after = t.elevation + pending;
            after > -0.01 && after < NEW_LAND_M
        };
        if pending.abs() >= APPLY_STEP_M && !shoal {
            t.elevation += pending;
            t.soil_depth = (t.soil_depth + t.soil_change).max(0.0);
            t.soil_change = 0.0;
            t.rock_change = 0.0;
        }
        if t.is_ocean() {
            continue;
        }
        if was_ocean {
            // Aus dem Meer aufgetaucht: Schwemmland aus abgelagertem Boden
            t.soil_moisture = 0.0;
            t.deep_moisture = 0.0;
            t.surface_water = 0.0;
            new_land.push(i as u32);
        }
        let cap = t.soil_depth * water_per_soil_meter(t.elevation);
        t.soil_capacity = cap;
        // Wasser, das der dünnere Boden nicht mehr hält, tritt an die Oberfläche.
        if t.soil_moisture > cap {
            t.surface_water += t.soil_moisture - cap;
            t.soil_moisture = cap;
        }
        let deep_cap = t.deep_capacity();
        if t.deep_moisture > deep_cap {
            t.surface_water += t.deep_moisture - deep_cap;
            t.deep_moisture = deep_cap;
        }
    }
    new_land
}

/// Summe aus Gelände und noch nicht übernommenen Änderungen über alle
/// Felder (m). Bleibt durch Abtrag und Ablagerung unverändert.
pub fn total_ground(world: &World) -> f64 {
    world
        .tiles
        .iter()
        .map(|t| t.elevation as f64 + t.soil_change as f64 + t.rock_change as f64)
        .sum()
}
