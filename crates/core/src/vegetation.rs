//! Pflanzenwachstum und -verfall: Gras und Wald. Ein Aufruf entspricht einem Tag.
//!
//! Gras wächst schnell und überall, wo es feucht und warm genug ist.
//! Wald wächst langsam, braucht mehr Wasser und breitet sich nur von
//! bestehendem Wald aus (Samen). Ein abgestorbener Wald kehrt also nur
//! von seinen Rändern her zurück.

use crate::climate::Weather;
use crate::math::clamp01;
use crate::rng::Rng;
use crate::world::{POND_MM, World};

pub fn step_day(world: &mut World, weather: &Weather, rng: &mut Rng, forest_prev: &mut Vec<f32>) {
    forest_prev.clear();
    forest_prev.extend(world.tiles.iter().map(|t| t.forest));
    let max_e = world.max_elevation;

    for i in 0..world.tiles.len() {
        if world.tiles[i].is_ocean() {
            continue;
        }
        // Samenquelle: stärkster Wald in der Nachbarschaft (Stand vom Vortag)
        let seed_source = world
            .neighbors(i)
            .map(|n| forest_prev[n])
            .fold(0.0f32, f32::max);
        let roll = rng.next_f32();

        let t = &mut world.tiles[i];
        let temp = weather.temp_at(t.elevation);
        let w = t.moisture_fraction();
        let rel = t.elevation / max_e;
        let warmth = clamp01((temp - 4.0) / 12.0) * (1.0 - clamp01((temp - 32.0) / 8.0));
        let coast: f32 = if t.elevation < 3.0 { 0.35 } else { 1.0 };
        let treeline = 1.0 - clamp01((rel - 0.72) / 0.18);
        let flooded = t.surface_water > POND_MM;

        // --- Gras ---
        let g_water = clamp01((w - 0.15) / 0.45);
        let g_max = (1.0 - 0.7 * t.forest) * coast.max(0.6);
        let mut g = t.grass;
        if g < 0.02 && g_water > 0.3 && warmth > 0.2 {
            g += 0.002; // angewehte Samen
        }
        g += 0.045 * warmth * g_water * g * (1.0 - g / g_max.max(0.05));
        if w < 0.08 {
            g -= 0.02 * g; // vertrocknet
        }
        if temp < 2.0 {
            g -= 0.002 * g; // Winterruhe
        }
        if flooded {
            g -= 0.03 * g;
        }
        t.grass = clamp01(g);

        // --- Wald ---
        let f_water = clamp01((w - 0.25) / 0.45);
        if w < 0.12 {
            t.drought_days = t.drought_days.saturating_add(1);
        } else {
            t.drought_days = t.drought_days.saturating_sub(2);
        }
        let mut f = t.forest;
        if f < 0.01 && seed_source > 0.4 && f_water > 0.5 && treeline > 0.3 && !flooded {
            // Keimung neuer Bäume am Waldrand
            if roll < 0.002 * coast {
                f = 0.02;
            }
        }
        f += 0.0012 * warmth * f_water * treeline * coast * f * (1.0 - f);
        if t.drought_days > 40 {
            f -= 0.004 * f; // Dürreschäden
        }
        if flooded {
            f -= 0.01 * f; // Staunässe
        }
        f -= 0.00002 * f; // natürliches Absterben alter Bäume
        if f < 0.005 {
            f = 0.0;
        }
        t.forest = clamp01(f);
    }
}
