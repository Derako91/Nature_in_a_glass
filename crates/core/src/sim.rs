//! Simulationsablauf: Uhr, Scheduler, Eingriffe, Statistik, Speichern.

use serde::{Deserialize, Serialize};

use crate::animals::{self, Animals, Cause, Species};
use crate::climate::{Climate, Weather};
use crate::erosion::{self, ErosionDay};
use crate::events::{EventKind, Journal};
use crate::fire;
use crate::hydro::{self, HydroDay};
use crate::lakes::{LakeChange, Lakes};
use crate::people::{self, NewsKind, People};
use crate::rng::Rng;
use crate::vegetation;
use crate::world::{POND_MM, World};

pub const HOURS_PER_DAY: u64 = 24;
pub const DAYS_PER_YEAR: u64 = 365;
pub const HOURS_PER_YEAR: u64 = HOURS_PER_DAY * DAYS_PER_YEAR;

/// Version des Spielstand-Formats. Erhöhen, wenn sich die Struktur ändert.
pub const SAVE_FORMAT_VERSION: u32 = 7;
pub const ENGINE_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const DEFAULT_SIZE: u32 = 64;

/// Unter diesem Füllstand meldet die Chronik einen Grundwasser-Tiefstand.
pub const GROUNDWATER_LOW: f32 = 0.15;
/// Ab diesem Füllstand gilt das Grundwasser wieder als erholt.
pub const GROUNDWATER_RECOVERED: f32 = 0.3;

/// Fassungsvermögen des Grundwasserkörpers als Vielfaches des alten
/// Bezugswerts (300 Tage Grundschüttung). Beim Start ist er zu zwei Dritteln voll.
pub const AQUIFER_CAPACITY_FACTOR: f32 = 1.5;

/// Ab diesem Durchfluss (mm/Tag) zählt eine Kachel als Bach.
pub const STREAM_FLOW: f32 = 60.0;

/// Ab diesem Durchfluss (mm/Tag) fließt auf einem Feld ein sichtbares Rinnsal.
/// Darunter ist es flächiger Abfluss, wie er nach Regen überall vorkommt.
pub const RILL_FLOW: f32 = 15.0;

/// Ab diesem Inselmittel (mm an einem Tag) gilt Regen als Starkregen.
pub const HEAVY_RAIN_MM: f32 = 40.0;

/// Ab so vielen Feldern mit über 2 mm Bodenverlust an einem Tag meldet die
/// Chronik eine Abspülung.
pub const EROSION_EVENT_TILES: u32 = 8;

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
    // seit v0.2
    #[serde(default)]
    pub shrub_pct: f32,
    #[serde(default)]
    pub deadwood_pct: f32,
    #[serde(default)]
    pub fires: u32,
    #[serde(default)]
    pub burned_tiles: u32,
    // seit v0.3
    #[serde(default)]
    pub permanent_stream_tiles: u32,
    /// Abfluss ins Meer im ganzen Jahr (m³).
    #[serde(default)]
    pub discharge_m3: f32,
    // seit v0.3.0
    /// Im Jahr abgetragener Boden (m³).
    #[serde(default)]
    pub eroded_m3: f32,
    /// Davon ins Meer getragen (m³).
    #[serde(default)]
    pub sediment_to_sea_m3: f32,
    /// Mittlere Bodendicke der Insel (m).
    #[serde(default)]
    pub soil_depth_m: f32,
    // seit v0.4: Tierbestände zum Jahresende
    #[serde(default)]
    pub deer: u32,
    #[serde(default)]
    pub wolves: u32,
    #[serde(default)]
    pub foxes: u32,
    #[serde(default)]
    pub rabbits: u32,
    /// seit v0.6: lebende Menschen zum Jahresende
    #[serde(default)]
    pub people: u32,
}

/// Momentaufnahme der ganzen Insel.
#[derive(Debug, Clone, Default)]
pub struct Snapshot {
    pub forest_pct: f32,
    pub grass_pct: f32,
    pub shrub_pct: f32,
    pub deadwood_pct: f32,
    pub moisture_pct: f32,
    pub deep_moisture_pct: f32,
    pub pond_tiles: u32,
    pub stream_tiles: u32,
    pub rill_tiles: u32,
    pub burning_tiles: u32,
    pub tree_tiles: u32,
    /// Wasserläufe nach Art (Felder).
    pub permanent_tiles: u32,
    pub seasonal_tiles: u32,
    pub seasonal_dry_tiles: u32,
    pub rain_channel_tiles: u32,
}

/// Welche Art von Samen der Beobachter ausstreut.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SeedKind {
    Tree,
    Shrub,
}

impl SeedKind {
    pub fn label(self) -> &'static str {
        match self {
            SeedKind::Tree => "Baumsamen",
            SeedKind::Shrub => "Buschsamen",
        }
    }
}

/// Ein ruhender Samen im Boden, ausgebracht durch einen Eingriff.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Seed {
    pub tile: u32,
    pub kind: SeedKind,
    /// Wie viele Tage der Samen noch keimfähig ist.
    pub days_left: u16,
}

/// Samen bleiben etwa drei Jahre keimfähig.
pub const SEED_VIABLE_DAYS: u16 = 3 * 365;

/// Erlaubte Eingriffe des Beobachters: nur Bedingungen, nie Bewohner.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Intervention {
    /// Schüttung einer Quelle verändern (1.0 = natürlich).
    SetSpringFactor { spring: usize, factor: f32 },
    /// Jahresniederschlag des Klimas ändern (mm).
    SetAnnualRain { mm: f32 },
    /// Jahresmitteltemperatur ändern (°C).
    SetMeanTemp { celsius: f32 },
    /// Einen einzelnen Samen an einem zufälligen, nicht angezeigten Ort fallen lassen.
    ScatterSeed { kind: SeedKind },
    /// Eine kleine Gruppe Tiere aussetzen – an einem Feld oder, ohne Angabe,
    /// an einer zufälligen Stelle: Räuber bei ihrer Beute, sonst im Grasland.
    ReleaseAnimals {
        species: Species,
        at: Option<(u32, u32)>,
    },
    /// Gelände eines Landfeldes anheben (Damm, positiv) oder absenken
    /// (Graben/Durchbruch, negativ), in Metern. Das Feld bleibt Land.
    ChangeTerrain { x: u32, y: u32, meters: f32 },
    /// Ein Boot mit fünf Menschen landet an der Küste – an einem Feld oder,
    /// ohne Angabe, an einer zufälligen Küstenstelle. Danach handeln die
    /// Menschen selbst (Prinzip 1).
    PeopleArrive { at: Option<(u32, u32)> },
}

/// Höchste Änderung je Eingriff (m).
pub const MAX_TERRAIN_STEP: f32 = 20.0;
/// Land sinkt durch Eingriffe nicht unter diese Höhe (kein neues Meer).
pub const MIN_LAND_ELEVATION: f32 = 0.5;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
struct YearAccumulator {
    rain_mm: f32,
    storms: u32,
    drought_days: u32,
    #[serde(default)]
    fires: u32,
    #[serde(default)]
    burned: u32,
    #[serde(default)]
    discharge_m3: f32,
    /// Seit Format 5: abgetragener Boden und Bodenfracht ins Meer (m³).
    #[serde(default)]
    eroded_m3: f32,
    #[serde(default)]
    sediment_to_sea_m3: f32,
}

/// Verfolgt einen laufenden Brand (mehrere Brandherde zählen zusammen).
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
struct FireTracker {
    active: bool,
    since_hours: u64,
    origin: (u32, u32),
    burned: u32,
    doused: bool,
}

/// Verfolgt den Waldbestand der Insel für die Chronik.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
struct ForestTracker {
    /// Höchststand seit der letzten Waldsterben-Meldung (%).
    reference_pct: f32,
    reference_hours: u64,
    has_trees: bool,
    treeless_since: u64,
    burned_since_reference: u32,
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
    /// Bezugswert aus v0.1/v0.2 (300 Tage Grundschüttung); bleibt für alte Spielstände.
    pub aquifer_ref: f32,
    /// Fassungsvermögen des Grundwasserkörpers (mm-Summe über Felder), seit Format 4.
    #[serde(default)]
    pub aquifer_capacity: f32,
    pub journal: Journal,
    pub history: Vec<YearStats>,
    rng_weather: Rng,
    rng_vegetation: Rng,
    year: YearAccumulator,
    drought: DroughtTracker,
    // seit v0.2
    #[serde(default)]
    rng_fire: Rng,
    #[serde(default)]
    rng_seeds: Rng,
    #[serde(default)]
    pub seeds: Vec<Seed>,
    #[serde(default)]
    fire_tracker: FireTracker,
    #[serde(default)]
    forest_tracker: ForestTracker,
    /// Ob das Grundwasser gerade als Tiefstand gemeldet ist.
    #[serde(default)]
    groundwater_low: bool,
    /// Höhenänderung je Feld im laufenden Jahr (m), seit Format 5.
    #[serde(default)]
    erosion_year: Vec<f32>,
    /// Benannte Seen, seit Format 5.
    #[serde(default)]
    pub lakes: Lakes,
    /// Tiere, seit Format 6.
    #[serde(default)]
    pub animals: Animals,
    #[serde(default)]
    rng_animals: Rng,
    /// Menschen, seit Format 7.
    #[serde(default)]
    pub people: People,
    #[serde(default)]
    rng_people: Rng,

    /// Wasserbilanz des letzten Tages (für Auswertung und Tests).
    #[serde(skip)]
    pub last_water: HydroDay,
    /// Abtrag des letzten Tages.
    #[serde(skip)]
    pub last_erosion: ErosionDay,
    #[serde(skip)]
    scratch_through: Vec<f32>,
    #[serde(skip)]
    scratch_order: Vec<u32>,
    #[serde(skip)]
    scratch_erosion: erosion::Scratch,
    #[serde(skip)]
    scratch_vegetation: vegetation::Scratch,
    #[serde(skip)]
    scratch_fire: fire::Scratch,
    #[serde(skip)]
    scratch_people: people::Scratch,
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
            aquifer_capacity: (aquifer_ref * AQUIFER_CAPACITY_FACTOR).max(1.0),
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
            rng_fire: Rng::stream(seed, "feuer"),
            rng_seeds: Rng::stream(seed, "samen"),
            seeds: Vec::new(),
            fire_tracker: FireTracker::default(),
            forest_tracker: ForestTracker::default(),
            groundwater_low: false,
            erosion_year: Vec::new(),
            lakes: Lakes::default(),
            animals: Animals::default(),
            rng_animals: Rng::stream(seed, "tiere"),
            people: People::default(),
            rng_people: Rng::stream(seed, "menschen"),
            last_water: HydroDay::default(),
            last_erosion: ErosionDay::default(),
            scratch_through: Vec::new(),
            scratch_order: Vec::new(),
            scratch_erosion: erosion::Scratch::default(),
            scratch_vegetation: vegetation::Scratch::default(),
            scratch_fire: fire::Scratch::default(),
            scratch_people: people::Scratch::default(),
        };
        sim.forest_tracker.has_trees = sim.world.tiles.iter().any(|t| t.forest > 0.0);
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
            self.aquifer_capacity,
            &mut self.scratch_through,
            &mut self.scratch_order,
        );
        self.last_water = day;
        if self.erosion_year.len() != n {
            self.erosion_year = vec![0.0; n];
        }
        let soil_day = erosion::step_day(
            &mut self.world,
            &self.weather,
            &self.scratch_through,
            &self.scratch_order,
            &mut self.erosion_year,
            &mut self.scratch_erosion,
        );
        self.last_erosion = soil_day;
        self.year.eroded_m3 += soil_day.eroded * erosion::M3_PER_TILE_M;
        self.year.sediment_to_sea_m3 += soil_day.to_sea * erosion::M3_PER_TILE_M;
        vegetation::step_day(
            &mut self.world,
            &self.weather,
            &mut self.rng_vegetation,
            &mut self.scratch_vegetation,
        );
        let fire_day = fire::step_day(
            &mut self.world,
            &self.weather,
            &mut self.rng_fire,
            &mut self.scratch_fire,
        );
        let doy = self.day_of_year();
        let animal_day = animals::step_day(
            &mut self.animals,
            &mut self.world,
            &self.weather,
            doy,
            &mut self.rng_animals,
        );
        self.report_extinctions(&animal_day.extinct);
        let news = people::step_day(
            &mut self.people,
            &self.world,
            &mut self.animals,
            &self.weather,
            &self.lakes,
            doy,
            self.time_hours,
            &mut self.rng_people,
            &mut self.scratch_people,
        );
        self.report_people(news);
        self.step_seeds();
        if self.day_of_year().is_multiple_of(30) {
            self.world.flush_tiny();
        }

        self.year.rain_mm += day.rain_mm;
        self.year.discharge_m3 += day.to_sea * hydro::M3_PER_TILE_MM;
        self.detect_events(day.rain_mm);
        self.detect_erosion_events(&soil_day, day.rain_mm);
        self.detect_fire_events(&fire_day);
        self.detect_forest_events();
    }

    /// Ausgebrachte Samen keimen nur, wenn die Bedingungen am Ort stimmen.
    fn step_seeds(&mut self) {
        let now = self.time_hours;
        let max_e = self.world.max_elevation;
        let mut k = 0;
        while k < self.seeds.len() {
            let roll = self.rng_seeds.next_f32();
            let seed = self.seeds[k].clone();
            let i = seed.tile as usize;
            let t = &self.world.tiles[i];
            let temp = self.weather.temp_at(t.elevation);
            let w = t.moisture_fraction();
            let rel = t.elevation / max_e;
            let occupied = match seed.kind {
                SeedKind::Tree => t.forest > 0.01,
                SeedKind::Shrub => t.shrub > 0.01,
            };
            let suitable = !t.is_ocean()
                && !occupied
                && t.fire <= 0.0
                && t.surface_water <= POND_MM
                && w > 0.35
                && (8.0..30.0).contains(&temp)
                && (seed.kind == SeedKind::Shrub || rel < 0.85);
            if suitable && roll < 0.02 {
                let (x, y) = self.world.xy(i);
                let t = &mut self.world.tiles[i];
                let what = match seed.kind {
                    SeedKind::Tree => {
                        t.forest = 0.02;
                        t.forest_age = 0.0;
                        "ein junger Baum"
                    }
                    SeedKind::Shrub => {
                        t.shrub = 0.02;
                        "ein junger Busch"
                    }
                };
                let moisture = w * 100.0;
                self.journal.push(
                    now,
                    EventKind::Germination,
                    Some((x, y)),
                    format!("Bei ({x}, {y}) ist {what} aufgegangen."),
                    vec![
                        format!("{} des Beobachters", seed.kind.label()),
                        format!("Bodenfeuchte {moisture:.0} %, {temp:.0} °C"),
                    ],
                );
                self.seeds.swap_remove(k);
                continue;
            }
            if seed.days_left <= 1 {
                let reason = if occupied {
                    "Der Platz war schon bewachsen".to_string()
                } else {
                    "Die Bedingungen am Ort passten nie".to_string()
                };
                self.journal.push(
                    now,
                    EventKind::SeedFailed,
                    None,
                    format!(
                        "Ein ausgestreuter {} ist nicht aufgegangen.",
                        seed.kind.label()
                    ),
                    vec![reason],
                );
                self.seeds.swap_remove(k);
                continue;
            }
            self.seeds[k].days_left -= 1;
            k += 1;
        }
    }

    fn detect_fire_events(&mut self, day: &fire::FireDay) {
        let now = self.time_hours;
        for s in day.strikes.iter().filter(|s| s.ignited) {
            let (x, y) = self.world.xy(s.tile);
            let t = &self.world.tiles[s.tile];
            let mut causes = vec![
                if self.weather.dry_thunder {
                    "Trockenes Gewitter".to_string()
                } else {
                    "Gewitter".to_string()
                },
                format!("Bodenfeuchte {:.0} %", t.moisture_fraction() * 100.0),
            ];
            if t.deadwood > 0.1 {
                causes.push(format!("Totholz {:.0} %", t.deadwood * 100.0));
            }
            causes.push(self.weather.wind_text().to_string());
            if !self.fire_tracker.active {
                self.fire_tracker = FireTracker {
                    active: true,
                    since_hours: now,
                    origin: (x, y),
                    burned: 0,
                    doused: false,
                };
                self.year.fires += 1;
            }
            self.journal.push(
                now,
                EventKind::FireStart,
                Some((x, y)),
                format!("Ein Blitz setzt bei ({x}, {y}) trockene Vegetation in Brand."),
                causes,
            );
        }
        if self.fire_tracker.active {
            self.fire_tracker.burned += day.newly_burning;
            self.year.burned += day.newly_burning;
            self.forest_tracker.burned_since_reference += day.newly_burning;
            if day.rain_doused {
                self.fire_tracker.doused = true;
            }
            let still = self.world.tiles.iter().any(|t| t.fire > 0.0);
            if !still {
                let ft = std::mem::take(&mut self.fire_tracker);
                let days = (now - ft.since_hours) / HOURS_PER_DAY + 1;
                let size = match ft.burned {
                    0..=3 => "Kleinbrand",
                    4..=40 => "Brand",
                    41..=200 => "Großbrand",
                    _ => "Feuersturm",
                };
                let reason = if ft.doused {
                    "Regen hat das Feuer gelöscht"
                } else {
                    "Das Feuer fand kein trockenes Brennmaterial mehr"
                };
                self.journal.push(
                    now,
                    EventKind::FireEnd,
                    Some(ft.origin),
                    format!(
                        "{size} erloschen nach {days} {}: {} verbrannt.",
                        if days == 1 { "Tag" } else { "Tagen" },
                        if ft.burned == 1 {
                            "1 Feld".to_string()
                        } else {
                            format!("{} Felder", ft.burned)
                        }
                    ),
                    vec![reason.to_string()],
                );
            }
        }
    }

    fn detect_forest_events(&mut self) {
        let now = self.time_hours;
        let (mut sum, mut land, mut first_tree) = (0.0f32, 0u32, None);
        for (i, t) in self.world.tiles.iter().enumerate() {
            if t.is_ocean() {
                continue;
            }
            land += 1;
            sum += t.forest;
            if first_tree.is_none() && t.forest > 0.0 {
                first_tree = Some(i);
            }
        }
        let pct = sum / land.max(1) as f32 * 100.0;
        let deep_pct = self.deep_moisture_pct();
        let soil_pct = self.drought.smoothed * 100.0;
        let ft = &mut self.forest_tracker;
        if pct > ft.reference_pct {
            ft.reference_pct = pct;
            ft.reference_hours = now;
            ft.burned_since_reference = 0;
        }
        if ft.reference_pct >= 5.0 && pct <= ft.reference_pct * 0.5 && pct > 0.0 {
            let since_year = ft.reference_hours / HOURS_PER_YEAR + 1;
            let mut causes = Vec::new();
            if ft.burned_since_reference > 0 {
                causes.push(format!("{} Felder verbrannt", ft.burned_since_reference));
            }
            causes.push(format!(
                "Bodenfeuchte {soil_pct:.0} %, Tiefenwasser {deep_pct:.0} %"
            ));
            let msg = format!(
                "Waldsterben: Seit Jahr {since_year} ist der Wald von {:.0} % auf {:.0} % der Insel geschrumpft.",
                ft.reference_pct, pct
            );
            ft.reference_pct = pct;
            ft.reference_hours = now;
            ft.burned_since_reference = 0;
            self.journal
                .push(now, EventKind::ForestDieback, None, msg, causes);
        }
        let ft = &mut self.forest_tracker;
        match (ft.has_trees, first_tree) {
            (true, None) => {
                ft.has_trees = false;
                ft.treeless_since = now;
                ft.reference_pct = 0.0;
                self.journal.push(
                    now,
                    EventKind::LastTree,
                    None,
                    "Der letzte Baum der Insel ist abgestorben.",
                    vec!["Ohne Bäume gibt es keine Baumsamen mehr".into()],
                );
            }
            (false, Some(i)) => {
                ft.has_trees = true;
                let years = (now - ft.treeless_since) / HOURS_PER_YEAR;
                let loc = self.world.xy(i);
                self.journal.push(
                    now,
                    EventKind::TreesReturn,
                    Some(loc),
                    format!(
                        "Erstmals seit {years} {} wachsen wieder Bäume auf der Insel.",
                        if years == 1 { "Jahr" } else { "Jahren" }
                    ),
                    Vec::new(),
                );
            }
            _ => {}
        }
    }

    fn deep_moisture_pct(&self) -> f32 {
        let (mut sum, mut n) = (0.0, 0u32);
        for t in &self.world.tiles {
            if !t.is_ocean() && t.deep_capacity() > 0.0 {
                sum += t.deep_fraction();
                n += 1;
            }
        }
        sum / n.max(1) as f32 * 100.0
    }

    fn detect_events(&mut self, rain_today: f32) {
        let now = self.time_hours;

        // Nur wirklich heftige Unwetter kommen in die Chronik.
        if self.weather.storm && rain_today >= HEAVY_RAIN_MM {
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
                format!(
                    "Dürre: Die Böden sind ausgetrocknet ({:.0} % Feuchte).",
                    d.smoothed * 100.0
                ),
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
                vec![format!(
                    "Niederschlag der letzten 60 Tage: {:.0} mm",
                    rain60
                )],
            );
        }

        // Grundwasser
        let level = self.groundwater_level();
        if !self.groundwater_low && level < GROUNDWATER_LOW {
            self.groundwater_low = true;
            let rain_year: f32 = self.drought.recent_rain.iter().sum::<f32>() * 365.0 / 60.0;
            self.journal.push(
                now,
                EventKind::GroundwaterLow,
                None,
                format!(
                    "Das Grundwasser der Insel ist auf {:.0} % gesunken.",
                    level * 100.0
                ),
                vec![
                    format!("Niederschlag zuletzt hochgerechnet etwa {rain_year:.0} mm im Jahr"),
                    format!(
                        "Wald bedeckt {:.0} % der Insel und verbraucht viel Wasser",
                        self.forest_tracker_pct()
                    ),
                ],
            );
        } else if self.groundwater_low && level > GROUNDWATER_RECOVERED {
            self.groundwater_low = false;
            self.journal.push(
                now,
                EventKind::GroundwaterRecovered,
                None,
                format!(
                    "Das Grundwasser hat sich auf {:.0} % erholt.",
                    level * 100.0
                ),
                Vec::new(),
            );
        }

        // Quellen
        let fill = level;
        for k in 0..self.world.springs.len() {
            let s = &mut self.world.springs[k];
            let ratio = if s.base_rate > 0.0 {
                s.current / s.base_rate
            } else {
                0.0
            };
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
        let n = self.world.tiles.len();
        if self.erosion_year.len() != n {
            self.erosion_year = vec![0.0; n];
        }
        let new_land = erosion::apply_year(&mut self.world, &mut self.erosion_year);
        self.report_new_land(&new_land);
        let finished = self.year() - 1;
        let changes = self.lakes.update(&self.world, finished);
        self.report_lakes(changes);
        let snap = self.snapshot();
        let stats = YearStats {
            year: finished,
            rain_mm: self.year.rain_mm,
            forest_pct: snap.forest_pct,
            grass_pct: snap.grass_pct,
            moisture_pct: snap.moisture_pct,
            pond_tiles: snap.pond_tiles,
            stream_tiles: snap.stream_tiles,
            aquifer_pct: self.groundwater_level() * 100.0,
            storms: self.year.storms,
            drought_days: self.year.drought_days,
            shrub_pct: snap.shrub_pct,
            deadwood_pct: snap.deadwood_pct,
            fires: self.year.fires,
            burned_tiles: self.year.burned,
            permanent_stream_tiles: snap.permanent_tiles,
            discharge_m3: self.year.discharge_m3,
            eroded_m3: self.year.eroded_m3,
            sediment_to_sea_m3: self.year.sediment_to_sea_m3,
            soil_depth_m: self.mean_soil_depth(),
            deer: self.animals.total(Species::Deer),
            wolves: self.animals.total(Species::Wolf),
            foxes: self.animals.total(Species::Fox),
            rabbits: self.animals.total(Species::Rabbit),
            people: self.people.alive() as u32,
        };
        self.report_animal_year(finished);
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
                "Jahr {}: {:.0} mm Regen, Wald {:.1} %{}, Busch {:.0} %, Gras {:.0} %, Totholz {:.0} %{}.",
                finished,
                stats.rain_mm,
                stats.forest_pct,
                trend,
                stats.shrub_pct,
                stats.grass_pct,
                stats.deadwood_pct,
                if stats.burned_tiles > 0 {
                    format!(", {} Felder verbrannt", stats.burned_tiles)
                } else {
                    String::new()
                }
            ) + &self.animal_census_text(&stats),
            Vec::new(),
        );
        self.history.push(stats);
        self.year = YearAccumulator::default();
    }

    // ---------- Auswertung ----------

    /// Mittlere Dicke der Bodenschicht über alle Landfelder (m).
    pub fn mean_soil_depth(&self) -> f32 {
        let (mut sum, mut n) = (0.0, 0u32);
        for t in &self.world.tiles {
            if !t.is_ocean() {
                sum += t.soil_depth + t.soil_change;
                n += 1;
            }
        }
        sum / n.max(1) as f32
    }

    /// Starker Bodenabtrag an einem Tag kommt in die Chronik.
    fn detect_erosion_events(&mut self, d: &ErosionDay, rain_today: f32) {
        if d.heavy_tiles < EROSION_EVENT_TILES {
            return;
        }
        let m3 = (d.eroded - d.deposited).max(0.0) * erosion::M3_PER_TILE_M;
        let mut causes = vec![format!("Regen heute im Mittel {rain_today:.0} mm")];
        if d.heavy_burned_tiles > 0 {
            causes.push(format!(
                "{} der Felder sind Brandflächen ohne schützenden Bewuchs",
                d.heavy_burned_tiles
            ));
        } else {
            causes.push("Kahler oder spärlich bewachsener Boden".into());
        }
        let loc = d.worst_tile.map(|i| self.world.xy(i as usize));
        self.journal.push(
            self.time_hours,
            EventKind::SoilErosion,
            loc,
            format!(
                "Abspülung: Das Wasser trägt auf {} Feldern Boden ab, zusammen etwa {:.0} m³.",
                d.heavy_tiles, m3
            ),
            causes,
        );
    }

    /// Tierbestände für die Jahresbilanz („; Hirsche 34, Wölfe 4“).
    fn animal_census_text(&self, y: &YearStats) -> String {
        let ever = &self.animals.ever;
        let mut parts = Vec::new();
        for (s, n) in [
            (Species::Deer, y.deer),
            (Species::Rabbit, y.rabbits),
            (Species::Wolf, y.wolves),
            (Species::Fox, y.foxes),
        ] {
            if ever[s.index()] {
                parts.push(format!("{} {}", s.names().1, n));
            }
        }
        if parts.is_empty() {
            String::new()
        } else {
            format!(" Tiere: {}.", parts.join(", "))
        }
    }

    fn report_people(&mut self, news: Vec<people::PeopleNews>) {
        for n in news {
            let (x, y) = self.world.xy(n.tile as usize);
            let (kind, cause) = match n.kind {
                NewsKind::Injured => (EventKind::PersonInjured, "Verletzung"),
                NewsKind::Died => (EventKind::PersonDied, "Tod"),
                NewsKind::AllGone => (EventKind::PeopleGone, "letzter Mensch"),
            };
            self.journal.push(
                self.time_hours,
                kind,
                Some((x, y)),
                n.text,
                vec![cause.into()],
            );
        }
    }

    fn report_extinctions(&mut self, gone: &[Species]) {
        for &s in gone {
            let log = &self.animals.deaths_since;
            let mut causes = Vec::new();
            if let Some(c) = log.main_cause(s) {
                causes.push(format!(
                    "Die meisten {} sind zuletzt {}",
                    s.names().1,
                    c.label()
                ));
            }
            match s {
                Species::Wolf | Species::Fox => {
                    let prey = if s == Species::Wolf {
                        Species::Deer
                    } else {
                        Species::Rabbit
                    };
                    causes.push(format!(
                        "Noch {} {} auf der Insel",
                        self.animals.total(prey),
                        prey.names().1
                    ));
                }
                _ => {
                    let snap = self.snapshot();
                    causes.push(format!(
                        "Gras bedeckt {:.0} %, Busch {:.0} % der Insel",
                        snap.grass_pct, snap.shrub_pct
                    ));
                }
            }
            causes
                .push("Ausgestorbene Arten kehren nur zurück, wenn neue ausgesetzt werden".into());
            self.journal.push(
                self.time_hours,
                EventKind::AnimalsExtinct,
                None,
                format!("Die letzten {} der Insel sind gestorben.", s.names().1),
                causes,
            );
            self.animals.deaths_since.counts[s.index()] = [0; 5];
        }
    }

    /// Jahresrückblick der Tiere: Hungerjahre kommen in die Chronik.
    fn report_animal_year(&mut self, year: u64) {
        let d = self.animals.deaths.clone();
        for s in [Species::Deer, Species::Wolf, Species::Fox] {
            let starved = d.get(s, Cause::Starved);
            let alive = self.animals.total(s);
            // Ein Hungerjahr: mindestens 10 Tiere und die Hälfte des Bestands verhungert.
            if starved >= 10 && starved >= alive {
                let snap = self.snapshot();
                let mut causes = vec![format!(
                    "{} {} leben noch",
                    alive,
                    if alive == 1 { s.names().0 } else { s.names().1 }
                )];
                match s {
                    Species::Deer => causes.push(format!(
                        "Gras {:.0} %, Busch {:.0} % – zu wenig Futter für so viele Tiere",
                        snap.grass_pct, snap.shrub_pct
                    )),
                    Species::Wolf => causes.push(format!(
                        "Nur noch {} Hirsche als Beute",
                        self.animals.total(Species::Deer)
                    )),
                    _ => causes.push(format!(
                        "Nur noch {} Kaninchen als Beute",
                        self.animals.total(Species::Rabbit)
                    )),
                }
                self.journal.push(
                    self.time_hours,
                    EventKind::AnimalsHunger,
                    None,
                    format!(
                        "Hungerjahr {year}: {starved} {} sind verhungert.",
                        if starved == 1 {
                            s.names().0
                        } else {
                            s.names().1
                        }
                    ),
                    causes,
                );
            }
        }
        self.animals.deaths = Default::default();
        self.animals.births = [0; 4];
    }

    fn report_lakes(&mut self, changes: Vec<LakeChange>) {
        let now = self.time_hours;
        for c in changes {
            let (kind, lake, msg, causes) = match c {
                LakeChange::Formed { lake } => {
                    let (x, y) = self.world.xy(lake.anchor() as usize);
                    let msg = format!(
                        "Bei ({x}, {y}) liegt ein See: der {} ({} Felder, bis zu {:.1} m tief).",
                        lake.name,
                        lake.tiles.len(),
                        lake.max_depth_m
                    );
                    (
                        EventKind::LakeFormed,
                        lake,
                        msg,
                        vec!["Wasser sammelt sich in einer Senke".into()],
                    )
                }
                LakeChange::Silted { lake, years } => {
                    let msg = format!(
                        "Der {} ist verlandet: Abgelagerter Boden hat ihn nach {years} Jahren aufgefüllt.",
                        lake.name
                    );
                    (
                        EventKind::LakeGone,
                        lake,
                        msg,
                        vec!["Bäche tragen Boden in den See".into()],
                    )
                }
                LakeChange::DriedUp { lake, years } => {
                    let msg = format!("Der {} ist nach {years} Jahren verschwunden.", lake.name);
                    let causes = vec![
                        "Seit drei Jahren kein stehendes Wasser mehr".into(),
                        format!("Grundwasser bei {:.0} %", self.groundwater_level() * 100.0),
                    ];
                    (EventKind::LakeGone, lake, msg, causes)
                }
                LakeChange::Merged { lake, into } => {
                    let msg = format!("Der {} ist im {into} aufgegangen.", lake.name);
                    (
                        EventKind::LakeGone,
                        lake,
                        msg,
                        vec!["Die Wasserflächen sind zusammengewachsen".into()],
                    )
                }
            };
            let loc = self.world.xy(lake.anchor() as usize);
            self.journal.push(now, kind, Some(loc), msg, causes);
        }
    }

    fn report_new_land(&mut self, tiles: &[u32]) {
        for &i in tiles {
            let (x, y) = self.world.xy(i as usize);
            self.journal.push(
                self.time_hours,
                EventKind::NewLand,
                Some((x, y)),
                format!("Bei ({x}, {y}) ist aus abgelagertem Boden neues Land aufgetaucht."),
                vec!["Bäche tragen seit langer Zeit Boden ins Meer".into()],
            );
        }
    }

    fn island_moisture(&self) -> f32 {
        let (mut sum, mut n) = (0.0, 0u32);
        for t in &self.world.tiles {
            if !t.is_ocean() {
                sum += t.moisture_fraction();
                n += 1;
            }
        }
        if n == 0 { 0.0 } else { sum / n as f32 }
    }

    fn forest_tracker_pct(&self) -> f32 {
        let (mut sum, mut n) = (0.0, 0u32);
        for t in &self.world.tiles {
            if !t.is_ocean() {
                sum += t.forest;
                n += 1;
            }
        }
        sum / n.max(1) as f32 * 100.0
    }

    /// Füllstand des Grundwasserkörpers (0 = leer, 1 = voll).
    pub fn groundwater_level(&self) -> f32 {
        (self.aquifer / self.aquifer_capacity.max(1.0)).clamp(0.0, 1.0)
    }

    /// Grundwasser in m³.
    pub fn groundwater_m3(&self) -> f32 {
        self.aquifer * hydro::M3_PER_TILE_MM
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
            s.shrub_pct += t.shrub;
            s.deadwood_pct += t.deadwood.min(1.0);
            s.moisture_pct += t.moisture_fraction();
            s.deep_moisture_pct += t.deep_fraction();
            if t.fire > 0.0 {
                s.burning_tiles += 1;
            }
            if t.forest > 0.0 {
                s.tree_tiles += 1;
            }
            let wc = hydro::watercourse(t);
            match (wc.kind, wc.flowing) {
                (hydro::CourseKind::Permanent, _) => s.permanent_tiles += 1,
                (hydro::CourseKind::Seasonal, true) => s.seasonal_tiles += 1,
                (hydro::CourseKind::Seasonal, false) => s.seasonal_dry_tiles += 1,
                (hydro::CourseKind::RainChannel, _) => s.rain_channel_tiles += 1,
                _ => {}
            }
            if t.surface_water > POND_MM {
                s.pond_tiles += 1;
            }
            if t.flow > STREAM_FLOW {
                s.stream_tiles += 1;
            } else if t.flow >= RILL_FLOW {
                s.rill_tiles += 1;
            }
        }
        let k = 100.0 / n.max(1) as f32;
        s.forest_pct *= k;
        s.grass_pct *= k;
        s.shrub_pct *= k;
        s.deadwood_pct *= k;
        s.moisture_pct *= k;
        s.deep_moisture_pct *= k;
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
            feed(t.shrub.to_bits() as u64);
            feed(t.forest_age.to_bits() as u64);
            feed(t.deadwood.to_bits() as u64);
            feed(t.deep_moisture.to_bits() as u64);
            feed(t.fire.to_bits() as u64);
            feed(t.flow_year.to_bits() as u64);
            feed(t.flow_permanence.to_bits() as u64);
            feed(t.moisture_year.to_bits() as u64);
            feed(t.elevation.to_bits() as u64);
            feed(t.soil_depth.to_bits() as u64);
            feed(t.soil_change.to_bits() as u64);
            feed(t.rock_change.to_bits() as u64);
        }
        feed(self.seeds.len() as u64);
        feed(self.journal.events.len() as u64);
        for x in &self.animals.list {
            feed(x.id as u64);
            feed(x.tile as u64);
            feed(x.energy.to_bits() as u64);
        }
        for r in &self.animals.rabbits {
            feed(r.to_bits() as u64);
        }
        for p in &self.people.list {
            feed(p.id as u64);
            feed(p.tile as u64);
            feed(p.health.to_bits() as u64);
            feed(p.hunger.to_bits() as u64);
            feed(p.thirst.to_bits() as u64);
        }
        feed(self.people.departed.len() as u64);
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
            Intervention::ChangeTerrain { x, y, meters } => {
                if *x >= self.world.width || *y >= self.world.height {
                    return;
                }
                let i = self.world.idx(*x, *y);
                let t = &mut self.world.tiles[i];
                if t.is_ocean() {
                    return;
                }
                let step = meters.clamp(-MAX_TERRAIN_STEP, MAX_TERRAIN_STEP);
                let old = t.elevation;
                t.elevation = (old + step).max(MIN_LAND_ELEVATION);
                let new = t.elevation;
                self.world.max_elevation = self.world.max_elevation.max(new);
                let what = if new >= old {
                    "schüttet einen Damm auf"
                } else {
                    "gräbt einen Durchbruch"
                };
                (
                    Some((*x, *y)),
                    format!("Der Beobachter {what} bei ({x}, {y}): {old:.1} m → {new:.1} m."),
                )
            }
            Intervention::ReleaseAnimals { species, at } => {
                let tile = match at {
                    Some((x, y)) if *x < self.world.width && *y < self.world.height => {
                        let i = self.world.idx(*x, *y);
                        let t = &self.world.tiles[i];
                        if t.is_ocean() || t.surface_water > POND_MM {
                            return;
                        }
                        i as u32
                    }
                    Some(_) => return,
                    None if !self.animals.prey_tiles(*species).is_empty() => {
                        // Räuber dort aussetzen, wo es Beute gibt
                        let prey = self.animals.prey_tiles(*species);
                        prey[self.rng_animals.below(prey.len() as u32) as usize]
                    }
                    None => {
                        // Offenes Grasland, notfalls irgendein Feld mit Gras
                        let pick = |min_grass: f32, max_forest: f32| -> Vec<u32> {
                            (0..self.world.tiles.len() as u32)
                                .filter(|&i| {
                                    let t = &self.world.tiles[i as usize];
                                    !t.is_ocean()
                                        && t.surface_water <= POND_MM
                                        && t.grass > min_grass
                                        && t.forest < max_forest
                                })
                                .collect()
                        };
                        let mut land = pick(0.5, 0.3);
                        if land.is_empty() {
                            // Dicht bewaldete Insel: die offensten Stellen
                            let mut all = pick(0.0, 1.1);
                            all.sort_by(|&a, &b| {
                                let open = |i: u32| {
                                    let t = &self.world.tiles[i as usize];
                                    t.grass + t.shrub - t.forest
                                };
                                open(b).total_cmp(&open(a)).then(a.cmp(&b))
                            });
                            all.truncate((all.len() / 10).max(1));
                            land = all;
                        }
                        if land.is_empty() {
                            return;
                        }
                        land[self.rng_animals.below(land.len() as u32) as usize]
                    }
                };
                let n = self
                    .animals
                    .release(&self.world, *species, tile, &mut self.rng_animals);
                let (x, y) = self.world.xy(tile as usize);
                (
                    Some((x, y)),
                    format!(
                        "Der Beobachter setzt {n} {} bei ({x}, {y}) aus.",
                        species.names().1
                    ),
                )
            }
            Intervention::PeopleArrive { at } => {
                let ok = |t: &crate::world::Tile| !t.is_ocean() && t.surface_water <= POND_MM;
                let tile = match at {
                    Some((x, y)) if *x < self.world.width && *y < self.world.height => {
                        let i = self.world.idx(*x, *y);
                        if !ok(&self.world.tiles[i]) {
                            return;
                        }
                        i as u32
                    }
                    Some(_) => return,
                    None => {
                        let coast: Vec<u32> = (0..self.world.tiles.len())
                            .filter(|&i| {
                                ok(&self.world.tiles[i])
                                    && self
                                        .world
                                        .neighbors(i)
                                        .any(|j| self.world.tiles[j].is_ocean())
                            })
                            .map(|i| i as u32)
                            .collect();
                        if coast.is_empty() {
                            return;
                        }
                        coast[self.rng_people.below(coast.len() as u32) as usize]
                    }
                };
                let names = self
                    .people
                    .arrive(&self.world, tile, now, &mut self.rng_people);
                let (x, y) = self.world.xy(tile as usize);
                (
                    Some((x, y)),
                    format!(
                        "Der Beobachter lässt ein Boot bei ({x}, {y}) landen: {} gehen an Land.",
                        names.join(", ")
                    ),
                )
            }
            Intervention::ScatterSeed { kind } => {
                let land: Vec<u32> = (0..self.world.tiles.len() as u32)
                    .filter(|&i| !self.world.tiles[i as usize].is_ocean())
                    .collect();
                if land.is_empty() {
                    return;
                }
                let tile = land[self.rng_seeds.below(land.len() as u32) as usize];
                self.seeds.push(Seed {
                    tile,
                    kind: *kind,
                    days_left: SEED_VIABLE_DAYS,
                });
                // Der Ort bleibt verborgen – erst eine Keimung verrät ihn.
                (
                    None,
                    format!(
                        "Der Beobachter lässt einen {} auf die Insel fallen. Wo er landet, weiß niemand.",
                        kind.label()
                    ),
                )
            }
        };
        self.journal.push(
            now,
            EventKind::Intervention,
            loc,
            msg,
            vec!["Eingriff des Beobachters".into()],
        );
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
        let mut sim = file.simulation;
        match file.format_version {
            SAVE_FORMAT_VERSION => {}
            1 => {
                sim.migrate_from_v1();
                sim.migrate_from_v2();
                sim.migrate_from_v3();
                sim.migrate_from_v4();
            }
            2 => {
                sim.migrate_from_v2();
                sim.migrate_from_v3();
                sim.migrate_from_v4();
            }
            3 => {
                sim.migrate_from_v3();
                sim.migrate_from_v4();
            }
            4 => sim.migrate_from_v4(),
            5 | 6 => {}
            other => {
                return Err(format!(
                    "Spielstand-Format {other} wird nicht unterstützt (erwartet 1 bis {SAVE_FORMAT_VERSION})."
                ));
            }
        }
        sim.world.rebuild_drainage();
        if file.format_version < 6 {
            sim.rng_animals = Rng::stream(sim.world.seed ^ sim.time_hours, "tiere");
        }
        if file.format_version < 7 {
            sim.rng_people = Rng::stream(sim.world.seed ^ sim.time_hours, "menschen");
        }
        if file.format_version < 5 {
            // Bestehende Seen bekommen sofort ihre Namen.
            let year = sim.year().saturating_sub(1);
            let changes = sim.lakes.update(&sim.world, year);
            sim.report_lakes(changes);
        }
        // Spielstände aus sehr langen Läufen bis 0.2.4 haben eine übervolle Chronik.
        sim.journal.trim();
        Ok(sim)
    }

    /// Spielstände aus Format 4 übernehmen: Die Bodendicke ergibt sich aus
    /// dem bisherigen Wasserhaltevermögen.
    fn migrate_from_v4(&mut self) {
        for t in self.world.tiles.iter_mut() {
            t.soil_depth = if t.is_ocean() {
                0.0
            } else {
                t.soil_capacity / crate::world::water_per_soil_meter(t.elevation)
            };
            t.soil_change = 0.0;
            t.rock_change = 0.0;
            t.erosion_rate = 0.0;
        }
        self.erosion_year = vec![0.0; self.world.tiles.len()];
    }

    /// Spielstände aus Format 3 übernehmen: Das Grundwasser bekommt ein
    /// Fassungsvermögen; was darüber liegt, ist sofort übergelaufen.
    fn migrate_from_v3(&mut self) {
        self.aquifer_capacity = (self.aquifer_ref * AQUIFER_CAPACITY_FACTOR).max(1.0);
        self.aquifer = self.aquifer.min(self.aquifer_capacity);
    }

    /// Spielstände aus v0.2 übernehmen: Das Jahresgedächtnis der Wasserläufe
    /// wird aus dem heutigen Zustand geschätzt und füllt sich im ersten Jahr.
    fn migrate_from_v2(&mut self) {
        for t in self.world.tiles.iter_mut() {
            t.flow_year = t.flow;
            t.flow_permanence = if t.flow >= RILL_FLOW { 1.0 } else { 0.0 };
            t.moisture_year = t.moisture_fraction();
        }
    }

    /// Spielstände aus v0.1 übernehmen: Buschland, Totholz und Brände gibt
    /// es dort noch nicht. Bestehende Wälder bekommen ein geschätztes Alter
    /// (dichter Wald = älter), der tiefe Boden startet zu 60 % gefüllt.
    fn migrate_from_v1(&mut self) {
        let seed = self.world.seed;
        for t in self.world.tiles.iter_mut() {
            t.shrub = 0.0;
            t.deadwood = 0.0;
            t.fire = 0.0;
            t.burn_scar = 0.0;
            t.tree_supply = 1.0;
            t.forest_age = if t.forest > 0.0 {
                10.0 + t.forest * 50.0
            } else {
                0.0
            };
            t.deep_moisture = t.deep_capacity() * 0.6;
        }
        self.weather.wind_speed = 0.3;
        self.rng_fire = Rng::stream(seed ^ self.time_hours, "feuer");
        self.rng_seeds = Rng::stream(seed ^ self.time_hours, "samen");
        let pct = self.snapshot().forest_pct;
        self.forest_tracker = ForestTracker {
            reference_pct: pct,
            reference_hours: self.time_hours,
            has_trees: pct > 0.0,
            treeless_since: self.time_hours,
            burned_since_reference: 0,
        };
        self.journal.push(
            self.time_hours,
            EventKind::WorldCreated,
            None,
            "Spielstand aus Version 0.1 übernommen: Buschland, Totholz und Feuer kommen ab jetzt hinzu.",
            Vec::new(),
        );
    }
}
