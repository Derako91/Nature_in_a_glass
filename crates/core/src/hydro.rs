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
const AQUIFER_LEAK: f32 = 0.0006;
/// Tägliches Absickern aus dem tiefen Boden ins Grundwasser.
const DEEP_DRAIN: f32 = 0.001;
/// Wie viel des Tiefenwassers Wurzeln pro Tag höchstens erschließen.
const DEEP_UPTAKE: f32 = 0.03;
const SQRT2: f32 = std::f32::consts::SQRT_2;

/// 1 mm Wasser auf einem Feld (50 × 50 m) sind 2,5 m³.
pub const M3_PER_TILE_MM: f32 = crate::world::TILE_SIZE_M * crate::world::TILE_SIZE_M / 1000.0;

/// Durchfluss in Litern pro Sekunde aus mm/Tag auf einem Feld.
pub fn liters_per_second(mm_per_day: f32) -> f32 {
    mm_per_day * M3_PER_TILE_MM * 1000.0 / 86_400.0
}

/// Art eines Wasserlaufs, aus dem Jahresgedächtnis des Feldes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CourseKind {
    /// Hier fließt kaum je Wasser.
    None,
    /// Fließt nur nach Regen, sonst trocken (unter einem Viertel der Tage).
    RainChannel,
    /// Fließt einen Teil des Jahres (ein Viertel bis gut vier Fünftel der Tage).
    Seasonal,
    /// Fließt fast das ganze Jahr.
    Permanent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Watercourse {
    pub kind: CourseKind,
    /// Fließt gerade Wasser? (Ein zeitweiser Bach kann auch ein trockenes Bett sein.)
    pub flowing: bool,
}

/// Ab diesem Anteil fließender Tage gilt ein Lauf als dauerhaft.
pub const PERMANENT_SHARE: f32 = 0.85;
/// Ab diesem Anteil als zeitweise.
pub const SEASONAL_SHARE: f32 = 0.25;

pub fn watercourse(t: &crate::world::Tile) -> Watercourse {
    let flowing = t.flow >= crate::sim::RILL_FLOW;
    let p = t.flow_permanence;
    let kind = if t.is_ocean() {
        CourseKind::None
    } else if p >= PERMANENT_SHARE {
        CourseKind::Permanent
    } else if p >= SEASONAL_SHARE {
        CourseKind::Seasonal
    } else if flowing {
        CourseKind::RainChannel
    } else {
        CourseKind::None
    };
    Watercourse { kind, flowing }
}

/// Wasserspiegel eines Feldes in Metern: Gelände plus stehendes Wasser.
pub fn head(t: &crate::world::Tile) -> f32 {
    t.elevation + t.surface_water * 0.001
}

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

    // 3. Versickern und Abfluss nach Wasserspiegel.
    //
    // Die Felder werden vom höchsten zum tiefsten Wasserspiegel (Gelände +
    // Wasserstand) durchgegangen. Jedes Feld gibt sein überschüssiges Wasser an
    // alle tieferen Nachbarn ab – steilere bekommen deutlich mehr. Höchstens so
    // viel, dass der Nachbar nicht über das eigene Niveau steigt; der Rest
    // staut sich. Ohne tieferen Nachbarn bleibt das Wasser stehen (Teich, See,
    // Überschwemmung), bis es überläuft. So folgt Wasser Gelände UND Wasserstand:
    // Dämme stauen, Durchbrüche lassen Seen auslaufen, Starkregen tritt über Ufer.
    for v in through.iter_mut() {
        *v = 0.0;
    }
    // Sortierschlüssel einmal berechnen; Gleichstand nach Index → reproduzierbar
    let mut keyed: Vec<(f32, u32)> = world
        .tiles
        .iter()
        .enumerate()
        .filter(|(_, t)| !t.is_ocean())
        .map(|(i, t)| (head(t), i as u32))
        .collect();
    keyed.sort_unstable_by(|a, b| b.0.total_cmp(&a.0).then(a.1.cmp(&b.1)));
    let order: Vec<u32> = keyed.into_iter().map(|(_, i)| i).collect();
    let mut receivers: Vec<(usize, f32, f32)> = Vec::with_capacity(8);
    for &i in &order {
        let i = i as usize;
        {
            let t = &mut world.tiles[i];
            let room = (t.soil_capacity - t.soil_moisture).max(0.0);
            let inf = t.surface_water.min(INFILTRATION_MM).min(room);
            t.surface_water -= inf;
            t.soil_moisture += inf;
        }
        let t = &world.tiles[i];
        let excess = t.surface_water - FILM_MM;
        if excess <= 0.0 {
            world.downstream[i] = None;
            continue;
        }
        let h_i = head(t);
        let (x, y) = world.xy(i);
        receivers.clear();
        let mut total_weight = 0.0;
        let mut best = None;
        let mut best_weight = 0.0;
        for nb in world.neighbors(i) {
            let tn = &world.tiles[nb];
            let h_n = if tn.is_ocean() { 0.0 } else { head(tn) };
            if h_n >= h_i {
                continue;
            }
            let (nx, ny) = world.xy(nb);
            let diagonal = nx != x && ny != y;
            let drop = h_i - h_n;
            let slope = if diagonal { drop / SQRT2 } else { drop };
            let weight = slope * slope * slope; // stark bündelnd: Wasser sammelt sich in Rinnen
            // Höchstens die Hälfte des Höhenunterschieds herüberschieben (mm)
            let cap = if tn.is_ocean() {
                f32::INFINITY
            } else {
                drop * 500.0
            };
            receivers.push((nb, weight, cap));
            total_weight += weight;
            if weight > best_weight {
                best_weight = weight;
                best = Some(nb as u32);
            }
        }
        world.downstream[i] = best;
        if receivers.is_empty() || total_weight <= 0.0 {
            continue; // Senke: Wasser staut sich
        }
        let mut sent = 0.0;
        for &(nb, weight, cap) in &receivers {
            let share = (excess * weight / total_weight).min(cap);
            if share <= 0.0 {
                continue;
            }
            sent += share;
            if world.tiles[nb].is_ocean() {
                day.to_sea += share;
            } else {
                world.tiles[nb].surface_water += share;
            }
        }
        world.tiles[i].surface_water -= sent;
        through[i] = sent;
    }

    // 4. Verdunstung, Pflanzenwasser, Sickerwasser
    let mut recharge = 0.0;
    for (t, &passed) in world.tiles.iter_mut().zip(through.iter()) {
        if t.is_ocean() {
            continue;
        }
        t.flow = t.flow * 0.85 + passed * 0.15;
        // Jahresgedächtnis: wie viel und wie beständig fließt hier Wasser?
        let k = crate::world::YEAR_WEIGHT;
        t.flow_year += (passed - t.flow_year) * k;
        let flowing = if passed >= crate::sim::RILL_FLOW {
            1.0
        } else {
            0.0
        };
        t.flow_permanence += (flowing - t.flow_permanence) * k;
        t.moisture_year += (t.moisture_fraction() - t.moisture_year) * k;
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
