//! Klima (langfristige Parameter) und Wetter (täglicher Zustand).

use serde::{Deserialize, Serialize};

use crate::math::season;
use crate::rng::Rng;

/// Langfristige Klimaparameter der Insel. Der Beobachter darf sie verändern.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Climate {
    /// Durchschnittlicher Jahresniederschlag auf Meereshöhe (mm).
    pub annual_rain_mm: f32,
    /// Wie viel trockener der Sommer ist (0 = gleichmäßig, 1 = sehr trocken).
    pub summer_dryness: f32,
    /// Jahresmitteltemperatur auf Meereshöhe (°C).
    pub mean_temp_c: f32,
    /// Halbe Spanne zwischen Winter und Sommer (°C).
    pub temp_amplitude_c: f32,
}

impl Default for Climate {
    fn default() -> Self {
        Self {
            annual_rain_mm: 950.0,
            summer_dryness: 0.45,
            mean_temp_c: 13.0,
            temp_amplitude_c: 8.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Weather {
    /// Niederschlag heute auf Meereshöhe im Inselmittel (mm).
    pub rain_mm: f32,
    /// Ob heute ein Starkregen niedergeht.
    pub storm: bool,
    /// Zentrum des Regengebiets (Kachelkoordinaten).
    pub center_x: f32,
    pub center_y: f32,
    /// Tagesmitteltemperatur auf Meereshöhe (°C).
    pub temp_c: f32,
    /// Ob es gestern geregnet hat (Wetterlagen halten oft an).
    pub rained_yesterday: bool,
}

impl Default for Weather {
    fn default() -> Self {
        Self {
            rain_mm: 0.0,
            storm: false,
            center_x: 0.0,
            center_y: 0.0,
            temp_c: 10.0,
            rained_yesterday: false,
        }
    }
}

/// Grundwahrscheinlichkeit eines Regentages.
const BASE_RAIN_CHANCE: f32 = 0.32;

impl Weather {
    /// Würfelt das Wetter des Tages aus.
    pub fn roll(&mut self, climate: &Climate, day_of_year: u32, w: u32, h: u32, rng: &mut Rng) {
        let s = season(day_of_year);
        let seasonal = 1.0 - climate.summer_dryness * s;
        let mut p = BASE_RAIN_CHANCE * seasonal;
        if self.rained_yesterday {
            p += 0.18;
        }
        // Mittlere Regenmenge pro Regentag so, dass die Jahressumme passt.
        let mean_per_rain_day = climate.annual_rain_mm / (BASE_RAIN_CHANCE * 365.0 * 0.88);
        let rains = rng.chance(p.clamp(0.0, 0.95));
        let (u1, u2) = (rng.next_f32(), rng.next_f32());
        self.storm = rains && rng.chance(0.012);
        self.rain_mm = if rains {
            // Produkt zweier Zufallszahlen: meist wenig, manchmal viel.
            let mut r = mean_per_rain_day * 4.0 * u1 * u2;
            if self.storm {
                r = r * 3.0 + mean_per_rain_day * 4.0;
            }
            r
        } else {
            0.0
        };
        self.rained_yesterday = rains;
        self.center_x = rng.next_f32() * w as f32;
        self.center_y = rng.next_f32() * h as f32;
        let noise = (rng.next_f32() + rng.next_f32() - 1.0) * 3.0;
        let cloud_cooling = if rains { -1.5 } else { 0.0 };
        self.temp_c = climate.mean_temp_c + climate.temp_amplitude_c * s + noise + cloud_cooling;
    }

    /// Niederschlag an einer bestimmten Kachel.
    pub fn rain_at(&self, x: u32, y: u32, rel_elevation: f32, w: u32) -> f32 {
        if self.rain_mm <= 0.0 {
            return 0.0;
        }
        let dx = x as f32 - self.center_x;
        let dy = y as f32 - self.center_y;
        let dist = (dx * dx + dy * dy).sqrt() / w as f32;
        let cell = (1.35 - dist * 1.4).clamp(0.45, 1.35);
        // Berge fangen mehr Regen ab (Steigungsregen).
        let orographic = 0.75 + 0.7 * rel_elevation;
        self.rain_mm * cell * orographic
    }

    /// Temperatur an einer Kachel (−0,65 °C je 100 m).
    pub fn temp_at(&self, elevation: f32) -> f32 {
        self.temp_c - elevation.max(0.0) * 0.0065
    }
}

/// Potenzielle Verdunstung (mm/Tag) aus der Temperatur.
pub fn potential_evaporation(temp_c: f32) -> f32 {
    (0.2 * (temp_c + 2.0)).max(0.2)
}
