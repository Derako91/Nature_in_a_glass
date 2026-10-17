//! Pflanzenwachstum und -verfall: Gras, Buschland, Wald, Totholz.
//! Ein Aufruf entspricht einem Tag.
//!
//! - Gras wächst schnell überall, wo es feucht und warm genug ist. Seine
//!   Samen überdauern im Boden, deshalb kehrt es nach Regen zurück.
//! - Buschland wächst mittelschnell, verträgt Trockenheit besser als Wald und
//!   breitet sich von Büschen und Waldrändern aus.
//! - Wald wächst langsam und nur von bestehendem Wald aus (Samen). Junge Bäume
//!   keimen bevorzugt im Schutz von Büschen. Ob Bäume eine Dürre überstehen,
//!   hängt an ihrer Wasserversorgung (`tree_supply`): Alter → Wurzeltiefe,
//!   Boden → Tiefenwasser, Dichte → Bedarf. Siehe `hydro.rs`.
//! - Abgestorbene Bäume werden zu Totholz, das über Jahre zerfällt.

use crate::climate::Weather;
use crate::math::clamp01;
use crate::rng::Rng;
use crate::world::{POND_MM, World};

/// Ab so vielen Tagen Wassermangel beginnen Bäume abzusterben.
pub const TREE_STRESS_DAYS: u16 = 45;
/// Wasserversorgung, unter der Bäume als gestresst gelten.
pub const TREE_STRESS_SUPPLY: f32 = 0.3;

#[derive(Debug, Clone, Copy, Default)]
pub struct VegetationDay {
    /// Heute abgestorbene Waldfläche (Summe über Felder, 0..1 je Feld).
    pub forest_died: f32,
}

#[derive(Debug, Default, Clone)]
pub struct Scratch {
    forest: Vec<f32>,
    shrub: Vec<f32>,
}

pub fn step_day(
    world: &mut World,
    weather: &Weather,
    rng: &mut Rng,
    scratch: &mut Scratch,
) -> VegetationDay {
    let mut day = VegetationDay::default();
    scratch.forest.clear();
    scratch.forest.extend(world.tiles.iter().map(|t| t.forest));
    scratch.shrub.clear();
    scratch.shrub.extend(world.tiles.iter().map(|t| t.shrub));
    let max_e = world.max_elevation;

    for i in 0..world.tiles.len() {
        if world.tiles[i].is_ocean() {
            continue;
        }
        // Samenquellen in der Nachbarschaft (Stand vom Vortag)
        let mut tree_seeds = 0.0f32;
        let mut shrub_seeds = 0.0f32;
        for n in world.neighbors(i) {
            tree_seeds = tree_seeds.max(scratch.forest[n]);
            shrub_seeds = shrub_seeds
                .max(scratch.shrub[n])
                .max(scratch.forest[n] * 0.8);
        }
        let roll_tree = rng.next_f32();
        let roll_shrub = rng.next_f32();

        let t = &mut world.tiles[i];
        let temp = weather.temp_at(t.elevation);
        let w = t.moisture_fraction();
        let rel = t.elevation / max_e;
        let warmth = clamp01((temp - 4.0) / 12.0) * (1.0 - clamp01((temp - 32.0) / 8.0));
        let coast: f32 = if t.elevation < 3.0 { 0.35 } else { 1.0 };
        let treeline = 1.0 - clamp01((rel - 0.72) / 0.18);
        let flooded = t.surface_water > POND_MM;
        let ash_bonus = 1.0 + 0.6 * t.burn_scar;
        let burning = t.fire > 0.0;

        // --- Gras ---
        let g_water = clamp01((w - 0.15) / 0.45);
        let g_max = ((1.0 - 0.7 * t.forest - 0.3 * t.shrub) * coast.max(0.6)).max(0.05);
        let mut g = t.grass;
        if g < 0.02 && g_water > 0.3 && warmth > 0.2 && !burning {
            g += 0.002; // Samen aus dem Boden
        }
        g += 0.045 * warmth * g_water * ash_bonus * g * (1.0 - g / g_max);
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

        // --- Buschland ---
        let s_water = clamp01((w - 0.1) / 0.4);
        let s_max = ((1.0 - 0.85 * t.forest) * coast.max(0.5)).max(0.02);
        let mut s = t.shrub;
        if s < 0.01 && shrub_seeds > 0.3 && s_water > 0.4 && !flooded && !burning {
            if roll_shrub < 0.003 {
                s = 0.02; // Samen von Nachbarbüschen oder Vögeln vom Waldrand
            }
        }
        s += 0.004 * warmth * s_water * ash_bonus * s * (1.0 - s / s_max);
        let mut dead = 0.0;
        if w < 0.05 {
            let loss = 0.004 * s;
            s -= loss;
            dead += loss * 0.3;
        }
        if flooded {
            s -= 0.01 * s;
        }
        if s < 0.005 {
            s = 0.0;
        }
        t.shrub = clamp01(s);

        // --- Wald ---
        let f_water = clamp01((t.tree_supply - 0.3) / 0.5);
        let mut f = t.forest;
        if f < 0.01
            && tree_seeds > 0.4
            && f_water > 0.5
            && treeline > 0.3
            && !flooded
            && !burning
            && roll_tree < 0.002 * coast * (1.0 + 2.0 * t.shrub) * (1.0 - 0.5 * t.grass)
        {
            f = 0.02; // Keimung am Waldrand, im Schutz von Büschen leichter
            t.forest_age = 0.0;
        }
        f += 0.0012 * warmth * f_water * treeline * coast * f * (1.0 - f);

        if t.tree_supply < TREE_STRESS_SUPPLY && f > 0.0 {
            t.drought_days = t.drought_days.saturating_add(1);
        } else {
            t.drought_days = t.drought_days.saturating_sub(2);
        }
        let mut loss = 0.0;
        if t.drought_days > TREE_STRESS_DAYS {
            loss += 0.005 * f; // Bäume verdursten
        }
        if flooded {
            loss += 0.01 * f; // Staunässe
        }
        loss += 0.00002 * f; // natürliches Absterben alter Bäume
        f -= loss;
        if f < 0.005 {
            loss += f;
            f = 0.0;
        }
        dead += loss;
        day.forest_died += loss;
        t.forest = clamp01(f);
        if t.forest > 0.0 {
            t.forest_age += 1.0 / 365.0;
        } else {
            t.forest_age = 0.0;
            t.drought_days = 0;
        }

        // --- Totholz ---
        // Zerfällt langsam: trocken und kalt über Jahrzehnte, feucht und warm
        // in wenigen Jahren.
        let decay = 0.0002 + 0.0008 * warmth * clamp01(w * 1.5);
        t.deadwood = (t.deadwood * (1.0 - decay) + dead).min(1.5);
        if t.deadwood < 0.002 {
            t.deadwood = 0.0;
        }
        t.burn_scar *= 1.0 - 0.0006;
        if t.burn_scar < 0.01 {
            t.burn_scar = 0.0;
        }
    }
    day
}
