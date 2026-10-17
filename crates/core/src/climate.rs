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

    // --- seit Spielstand-Format 2 (v0.2) ---
    /// Windrichtung als Index in [`WIND_DIRS`] (wohin der Wind weht).
    #[serde(default)]
    pub wind_dir: u8,
    /// Windstärke 0 (Flaute) … 1 (Sturm).
    #[serde(default)]
    pub wind_speed: f32,
    /// Gewitter heute.
    #[serde(default)]
    pub thunder: bool,
    /// Blitzeinschläge auf der Insel heute.
    #[serde(default)]
    pub lightning_strikes: u8,
    /// Trockenes Gewitter: Blitze, aber kaum Regen.
    #[serde(default)]
    pub dry_thunder: bool,
}

/// Acht Windrichtungen (dx, dy), in derselben Reihenfolge wie die Namen.
pub const WIND_DIRS: [(i32, i32); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];

/// Woher der Wind kommt (Kartenoben = Norden).
pub const WIND_FROM: [&str; 8] = [
    "West", "Nordwest", "Nord", "Nordost", "Ost", "Südost", "Süd", "Südwest",
];

impl Weather {
    /// Einheitsvektor der Windrichtung.
    pub fn wind_vector(&self) -> (f32, f32) {
        let (dx, dy) = WIND_DIRS[(self.wind_dir % 8) as usize];
        let len = ((dx * dx + dy * dy) as f32).sqrt();
        (dx as f32 / len, dy as f32 / len)
    }

    pub fn wind_text(&self) -> String {
        let strength = match self.wind_speed {
            s if s < 0.15 => "still",
            s if s < 0.4 => "schwach",
            s if s < 0.7 => "mäßig",
            s if s < 0.9 => "stark",
            _ => "Sturm",
        };
        if self.wind_speed < 0.15 {
            strength.to_string()
        } else {
            format!(
                "{} aus {}",
                strength,
                WIND_FROM[(self.wind_dir % 8) as usize]
            )
        }
    }
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
            wind_dir: 0,
            wind_speed: 0.3,
            thunder: false,
            lightning_strikes: 0,
            dry_thunder: false,
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
        // Ohne Niederschlag im Klima gibt es auch keine Regenwolken und Gewitter.
        let rains = rng.chance(p.clamp(0.0, 0.95)) && climate.annual_rain_mm > 0.0;
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

        // Wind: meist aus Westen, Wetterlagen halten ein paar Tage an.
        let (u_change, u_dir, u_speed) = (rng.next_f32(), rng.next_f32(), rng.next_f32());
        if u_change < 0.3 {
            self.wind_dir = if u_dir < 0.6 {
                // vorherrschend: aus West, Nordwest oder Südwest (weht nach Ost)
                [0u8, 1, 7][(u_dir / 0.2) as usize % 3]
            } else {
                ((u_dir - 0.6) / 0.4 * 8.0) as u8 % 8
            };
        }
        self.wind_speed = (0.1 + 0.6 * u_speed + if self.storm { 0.35 } else { 0.0 }).min(1.0);

        // Gewitter: an warmen Regentagen, im Sommer häufiger.
        let (u_thunder, u_dry, u_strikes) = (rng.next_f32(), rng.next_f32(), rng.next_f32());
        let heat = ((self.temp_c - 16.0) / 10.0).clamp(0.0, 1.0);
        self.thunder = rains && (self.storm || u_thunder < 0.35 * heat);
        self.dry_thunder = self.thunder && !self.storm && u_dry < 0.3;
        if self.dry_thunder {
            self.rain_mm *= 0.1;
        }
        self.lightning_strikes = if self.thunder {
            1 + (u_strikes * 3.0) as u8
        } else {
            0
        };
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
