//! Simulationsablauf: Uhr, Scheduler, Eingriffe, Statistik, Speichern.

use serde::{Deserialize, Serialize};

use crate::climate::{Climate, Weather};
use crate::events::{EventKind, Journal};
use crate::hydro;
use crate::rng::Rng;
use crate::vegetation;
use crate::world::{World, POND_MM};

pub const HOURS_PER_DAY: u64 = 24;
pub const DAYS_PER_YEAR: u64 = 365;
pub const HOURS_PER_YEAR: u64 = HOURS_PER_DAY * DAYS_PER_YEAR;

/// Version des Spielstand-Formats. Erhöhen, wenn sich die Struktur ändert.
pub const SAVE_FORMAT_VERSION: u32 = 1;
pub const ENGINE_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const DEFAULT_SIZE: u32 = 64;

/// Ab diesem Durchfluss (mm/Tag) zählt eine Kachel als Bach.
pub const STREAM_FLOW: f32 = 60.0;

/// Kennzahlen eines vollständigen Jahres.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct YearStats {
    pub year: u64,
    pub rain_mm: f32,
    pub forest_pct: f32,
    pub grass_pct: f32,
    pub moisture_pct: f32,
    pub pond_tiles: u32,
    pub stream_tiles: u32,
    pub aquifer_pct: f32,
    pub storms: u32,
    pub drought_days: u32,
}

/// Momentaufnahme der ganzen Insel.
#[derive(Debug, Clone, Default)]
pub struct Snapshot {
    pub forest_pct: f32,
    pub grass_pct: f32,
    pub moisture_pct: f32,
    pub pond_tiles: u32,
    pub stream_tiles: u32,
}

/// Erlaubte Eingriffe des Beobachters: nur Bedingungen, nie Bewohner.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Intervention {
    /// Schüttung einer Quelle verändern (1.0 = natürlich).
    SetSpringFactor { spring: usize, factor: f32 },
    /// Jahresniederschlag des Klimas ändern (mm).
    SetAnnualRain { mm: f32 },
    /// Jahresmitteltemperatur ändern (°C).
    SetMeanTemp { celsius: f32 },
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
struct YearAccumulator {
    rain_mm: f32,
    storms: u32,
    drought_days: u32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
struct DroughtTracker {
    active: bool,
    since_hours: u64,
    /// Niederschlag der letzten 60 Tage (Ringpuffer).
    recent_rain: Vec<f32>,
    cursor: usize,
    /// Geglättete Bodenfeuchte der Insel (0..1).
    smoothed: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Simulation {
    pub world: World,
    pub climate: Climate,
    pub weather: Weather,
    /// Vergangene Simulationszeit in Stunden seit Erschaffung.
    pub time_hours: u64,
    /// Grundwasser der Quellen (Summe mm über Kacheln).
    pub aquifer: f32,
    pub aquifer_ref: f32,
    pub journal: Journal,
    pub history: Vec<YearStats>,
    rng_weather: Rng,
    rng_vegetation: Rng,
    year: YearAccumulator,
    drought: DroughtTracker,

    #[serde(skip)]
    scratch_through: Vec<f32>,
    #[serde(skip)]
    scratch_forest: Vec<f32>,
}

#[derive(Serialize, Deserialize)]
struct SaveFile {
    format_version: u32,
    engine_version: String,
    simulation: Simulation,
}

impl Simulation {
    pub fn new(seed: u64) -> Self {
        Self::with_size(seed, DEFAULT_SIZE, DEFAULT_SIZE)
    }

    pub fn with_size(seed: u64, width: u32, height: u32) -> Self {
        let world = World::generate(seed, width, height);
        let aquifer_ref: f32 = world.springs.iter().map(|s| s.base_rate).sum::<f32>() * 300.0;
        let mut sim = Simulation {
            world,
            climate: Climate::default(),
            weather: Weather::default(),
            time_hours: 0,
            aquifer: aquifer_ref,
            aquifer_ref: aquifer_ref.max(1.0),
            journal: Journal::default(),
            history: Vec::new(),
            rng_weather: Rng::stream(seed, "wetter"),
            rng_vegetation: Rng::stream(seed, "vegetation"),
            year: YearAccumulator::default(),
            drought: DroughtTracker {
                recent_rain: vec![2.5; 60],
                smoothed: 0.5,
                ..Default::default()
            },
            scratch_through: Vec::new(),
            scratch_forest: Vec::new(),
        };
        let land = sim.world.land_count();
        sim.journal.push(
            0,
            EventKind::WorldCreated,
            None,
            format!(
                "Eine Insel entsteht: {} Landfelder, höchster Punkt {:.0} m, {} Quellen.",
                land,
                sim.world.max_elevation,
                sim.world.springs.len()
            ),
            vec![format!("Seed {}", seed)],
        );
        sim.run_day();
        sim
    }

    // ---------- Zeit ----------

    pub fn day_index(&self) -> u64 {
        self.time_hours / HOURS_PER_DAY
    }
    /// Jahr, beginnend bei 1.
    pub fn year(&self) -> u64 {
        self.day_index() / DAYS_PER_YEAR + 1
    }
    /// Tag im Jahr, 0..365.
    pub fn day_of_year(&self) -> u32 {
        (self.day_index() % DAYS_PER_YEAR) as u32
    }
    pub fn hour_of_day(&self) -> u32 {
        (self.time_hours % HOURS_PER_DAY) as u32
    }

    /// Lässt die Zeit um `hours` Stunden fortschreiten.
    ///
    /// Scheduler: Tageprozesse (Wetter, Wasser, Vegetation) laufen bei jedem
    /// Tageswechsel, die Jahresbilanz bei jedem Jahreswechsel. Spätere
    /// Systeme (z. B. Bewohner im Minutentakt) hängen sich hier ein.
    pub fn advance_hours(&mut self, hours: u64) {
        let target = self.time_hours + hours;
        while self.time_hours < target {
            // Direkt zum nächsten Tageswechsel springen, wenn er im Ziel liegt
            let next_day = (self.time_hours / HOURS_PER_DAY + 1) * HOURS_PER_DAY;
            if next_day > target {
                self.time_hours = target;
                break;
            }
            self.time_hours = next_day;
            if self.day_of_year() == 0 {
                self.close_year();
            }
            self.run_day();
        }
    }

    pub fn advance_days(&mut self, days: u64) {
        self.advance_hours(days * HOURS_PER_DAY);
    }

    pub fn advance_years(&mut self, years: u64) {
        self.advance_hours(years * HOURS_PER_YEAR);
    }

    fn run_day(&mut self) {
        let (w, h) = (self.world.width, self.world.height);
        let doy = self.day_of_year();
        self.weather
            .roll(&self.climate, doy, w, h, &mut self.rng_weather);

        let n = self.world.tiles.len();
        if self.scratch_through.len() != n {
            self.scratch_through = vec![0.0; n];
        }
        let day = hydro::step_day(
            &mut self.world,
            &self.weather,
            &mut self.aquifer,
            self.aquifer_ref,
            &mut self.scratch_through,
        );
        vegetation::step_day(
            &mut self.world,
            &self.weather,
            &mut self.rng_vegetation,
            &mut self.scratch_forest,
        );

        self.year.rain_mm += day.rain_mm;
        self.detect_events(day.rain_mm);
    }

    fn detect_events(&mut self, rain_today: f32) {
        let now = self.time_hours;

        if self.weather.storm {
            self.year.storms += 1;
            self.journal.push(
                now,
                EventKind::HeavyRain,
                Some((self.weather.center_x as u32, self.weather.center_y as u32)),
                format!("Starkregen: im Mittel {:.0} mm an einem Tag.", rain_today),
                vec!["Unwetterlage über der Insel".into()],
            );
        }

        // Dürre: Bodenfeuchte der ganzen Insel
        let moisture = self.island_moisture();
        let doy_mid = (self.day_of_year() + 365 - 30) % 365;
        let seasonal = 1.0 - self.climate.summer_dryness * crate::math::season(doy_mid);
        let expected60 = self.climate.annual_rain_mm * 60.0 / 365.0 * seasonal;
        let d = &mut self.drought;
        if d.recent_rain.len() != 60 {
            d.recent_rain = vec![0.0; 60];
        }
        d.recent_rain[d.cursor] = rain_today;
        d.cursor = (d.cursor + 1) % 60;
        let rain60: f32 = d.recent_rain.iter().sum();
        d.smoothed = d.smoothed * 0.9 + moisture * 0.1;
        // Ein trockener Sommer ist normal. Dürre heißt: Böden ausgetrocknet
        // UND deutlich weniger Regen als üblich.
        let rain_deficit = rain60 <= expected60 * 0.4 || d.smoothed < 0.1;
        if d.active {
            self.year.drought_days += 1;
        }
        if !d.active && d.smoothed < 0.2 && rain_deficit {
            d.active = true;
            d.since_hours = now;
            self.journal.push(
                now,
                EventKind::DroughtStart,
                None,
                format!("Dürre: Die Böden sind ausgetrocknet ({:.0} % Feuchte).", d.smoothed * 100.0),
                vec![
                    format!(
                        "Niederschlag der letzten 60 Tage: {:.0} mm (üblich etwa {:.0} mm)",
                        rain60, expected60
                    ),
                    format!("Temperatur: {:.1} °C", self.weather.temp_c),
                ],
            );
        } else if d.active && d.smoothed > 0.35 {
            d.active = false;
            let days = (now - d.since_hours) / HOURS_PER_DAY;
            self.journal.push(
                now,
                EventKind::DroughtEnd,
                None,
                format!("Die Dürre endet nach {} Tagen.", days),
                vec![format!("Niederschlag der letzten 60 Tage: {:.0} mm", rain60)],
            );
        }

        // Quellen
        let fill = self.aquifer / self.aquifer_ref;
        for k in 0..self.world.springs.len() {
            let s = &mut self.world.springs[k];
            let ratio = if s.base_rate > 0.0 { s.current / s.base_rate } else { 0.0 };
            if !s.dry && ratio < 0.1 {
                s.dry = true;
                let cause = if s.factor < 0.15 {
                    "Zufluss durch Eingriff gedrosselt".to_string()
                } else {
                    format!("Grundwasser auf {:.0} % gesunken", fill * 100.0)
                };
                let (x, y) = (s.x, s.y);
                self.journal.push(
                    now,
                    EventKind::SpringDry,
                    Some((x, y)),
                    format!("Die Quelle bei ({}, {}) versiegt.", x, y),
                    vec![cause],
                );
            } else if s.dry && ratio > 0.25 {
                s.dry = false;
                let (x, y) = (s.x, s.y);
                self.journal.push(
                    now,
                    EventKind::SpringFlowing,
                    Some((x, y)),
                    format!("Die Quelle bei ({}, {}) fließt wieder.", x, y),
                    vec![format!("Grundwasser bei {:.0} %", fill * 100.0)],
                );
            }
        }
    }

    fn close_year(&mut self) {
        let snap = self.snapshot();
        let finished = self.year() - 1;
        let stats = YearStats {
            year: finished,
            rain_mm: self.year.rain_mm,
            forest_pct: snap.forest_pct,
            grass_pct: snap.grass_pct,
            moisture_pct: snap.moisture_pct,
            pond_tiles: snap.pond_tiles,
            stream_tiles: snap.stream_tiles,
            aquifer_pct: self.aquifer / self.aquifer_ref * 100.0,
            storms: self.year.storms,
            drought_days: self.year.drought_days,
        };
        let prev_forest = self.history.last().map(|s| s.forest_pct);
        let trend = match prev_forest {
            Some(p) => format!(" ({:+.1})", stats.forest_pct - p),
            None => String::new(),
        };
        self.journal.push(
            self.time_hours,
            EventKind::YearSummary,
            None,
            format!(
                "Jahr {}: {:.0} mm Regen, Wald {:.1} %{}, Gras {:.0} %, {} Bachfelder.",
                finished, stats.rain_mm, stats.forest_pct, trend, stats.grass_pct, stats.stream_tiles
            ),
            Vec::new(),
        );
        self.history.push(stats);
        self.year = YearAccumulator::default();
    }

    // ---------- Auswertung ----------

    fn island_moisture(&self) -> f32 {
        let (mut sum, mut n) = (0.0, 0u32);
        for t in &self.world.tiles {
            if !t.is_ocean() {
                sum += t.moisture_fraction();
                n += 1;
            }
        }
        if n == 0 {
            0.0
        } else {
            sum / n as f32
        }
    }

    pub fn snapshot(&self) -> Snapshot {
        let mut s = Snapshot::default();
        let mut n = 0u32;
        for t in &self.world.tiles {
            if t.is_ocean() {
                continue;
            }
            n += 1;
            s.forest_pct += t.forest;
            s.grass_pct += t.grass;
            s.moisture_pct += t.moisture_fraction();
            if t.surface_water > POND_MM {
                s.pond_tiles += 1;
            }
            if t.flow > STREAM_FLOW {
                s.stream_tiles += 1;
            }
        }
        let k = 100.0 / n.max(1) as f32;
        s.forest_pct *= k;
        s.grass_pct *= k;
        s.moisture_pct *= k;
        s
    }

    /// Prüfsumme über den gesamten Zustand (für Reproduzierbarkeitstests).
    pub fn state_hash(&self) -> u64 {
        let mut h: u64 = 0xcbf2_9ce4_8422_2325;
        let mut feed = |v: u64| {
            h ^= v;
            h = h.wrapping_mul(0x0000_0100_0000_01b3);
        };
        feed(self.time_hours);
        feed(self.aquifer.to_bits() as u64);
        for t in &self.world.tiles {
            feed(t.surface_water.to_bits() as u64);
            feed(t.soil_moisture.to_bits() as u64);
            feed(t.flow.to_bits() as u64);
            feed(t.grass.to_bits() as u64);
            feed(t.forest.to_bits() as u64);
            feed(t.drought_days as u64);
        }
        feed(self.journal.events.len() as u64);
        h
    }

    // ---------- Eingriffe ----------

    pub fn apply(&mut self, intervention: Intervention) {
        let now = self.time_hours;
        let (loc, msg) = match &intervention {
            Intervention::SetSpringFactor { spring, factor } => {
                let Some(s) = self.world.springs.get_mut(*spring) else {
                    return;
                };
                let f = factor.clamp(0.0, 5.0);
                let old = s.factor;
                s.factor = f;
                (
                    Some((s.x, s.y)),
                    format!(
                        "Der Beobachter verändert die Quelle bei ({}, {}): {:.0} % → {:.0} %.",
                        s.x,
                        s.y,
                        old * 100.0,
                        f * 100.0
                    ),
                )
            }
            Intervention::SetAnnualRain { mm } => {
                let old = self.climate.annual_rain_mm;
                self.climate.annual_rain_mm = mm.clamp(0.0, 5000.0);
                (
                    None,
                    format!(
                        "Der Beobachter verändert das Klima: Jahresniederschlag {:.0} → {:.0} mm.",
                        old, self.climate.annual_rain_mm
                    ),
                )
            }
            Intervention::SetMeanTemp { celsius } => {
                let old = self.climate.mean_temp_c;
                self.climate.mean_temp_c = celsius.clamp(-10.0, 35.0);
                (
                    None,
                    format!(
                        "Der Beobachter verändert das Klima: Jahresmittel {:.1} → {:.1} °C.",
                        old, self.climate.mean_temp_c
                    ),
                )
            }
        };
        self.journal
            .push(now, EventKind::Intervention, loc, msg, vec!["Eingriff des Beobachters".into()]);
    }

    // ---------- Speichern / Laden ----------

    pub fn to_json(&self) -> String {
        let file = SaveFile {
            format_version: SAVE_FORMAT_VERSION,
            engine_version: ENGINE_VERSION.to_string(),
            simulation: self.clone(),
        };
        serde_json::to_string(&file).expect("Spielstand serialisierbar")
    }

    pub fn from_json(json: &str) -> Result<Self, String> {
        let file: SaveFile =
            serde_json::from_str(json).map_err(|e| format!("Spielstand unlesbar: {e}"))?;
        if file.format_version != SAVE_FORMAT_VERSION {
            return Err(format!(
                "Spielstand-Format {} wird nicht unterstützt (erwartet {}).",
                file.format_version, SAVE_FORMAT_VERSION
            ));
        }
        let mut sim = file.simulation;
        sim.world.rebuild_drainage();
        Ok(sim)
    }
}
