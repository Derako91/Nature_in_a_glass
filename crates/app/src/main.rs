//! Nature in a Glass – Desktop-Beobachter.

#![cfg_attr(windows, windows_subsystem = "windows")]

mod atlas;
mod view;

use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use eframe::egui::{self, Color32, RichText};
use nig_core::animals::Species;
use nig_core::hydro::{CourseKind, M3_PER_TILE_MM, liters_per_second, watercourse};
use nig_core::sim::{HOURS_PER_DAY, STREAM_FLOW};

/// Zahl mit deutschem Dezimalkomma.
fn de(v: f32, decimals: usize) -> String {
    format!("{v:.decimals$}").replace('.', ",")
}

/// Alter kurz für Listen: „34 J.“ bzw. „5 Mon.“
fn age_short(days: u32) -> String {
    if days < 365 {
        format!("{} Mon.", days / 30)
    } else {
        format!("{} J.", days / 365)
    }
}

/// Alter ausgeschrieben: „34 Jahre“, „1 Jahr“, „5 Monate“.
fn age_long(days: u32) -> String {
    match days {
        d if d < 60 => format!("{} Tage", d.max(1)),
        d if d < 365 => format!("{} Monate", d / 30),
        d if d < 730 => "1 Jahr".to_string(),
        d => format!("{} Jahre", d / 365),
    }
}

/// Stärke eines Bedürfnisses in Worten (0 = gar nicht, 1 = tödlich).
fn level(v: f32) -> &'static str {
    match v {
        v if v < 0.15 => "keiner",
        v if v < 0.4 => "leicht",
        v if v < 0.7 => "stark",
        _ => "lebensgefährlich",
    }
}

/// Ganze Zahl mit Tausenderpunkten.
fn thousands(v: f32) -> String {
    let n = v.round().max(0.0) as u64;
    let s = n.to_string();
    let mut out = String::new();
    for (k, c) in s.chars().enumerate() {
        if k > 0 && (s.len() - k).is_multiple_of(3) {
            out.push('.');
        }
        out.push(c);
    }
    out
}
use nig_core::world::POND_MM;
use nig_core::{EventKind, Intervention, SeedKind, Simulation, Terrain};
use view::{Camera, Layer, ViewMode};

/// Zeitraffer-Stufen: (Bezeichnung, Simulationsstunden pro echter Sekunde)
const SPEEDS: [(&str, f64); 6] = [
    ("1 Std/s", 1.0),
    ("1 Tag/s", 24.0),
    ("1 Woche/s", 168.0),
    ("1 Monat/s", 730.0),
    ("1 Jahr/s", 8760.0),
    ("10 Jahre/s", 87600.0),
];

/// Wie der Beobachter auf wichtige Ereignisse reagiert.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EventReaction {
    /// Die Welt läuft ungestört weiter, Ereignisse stehen nur in der Chronik.
    ChronicleOnly,
    /// Die Zeit hält an, eine Meldung erscheint.
    Pause,
    /// Die Zeit hält an und die Kamera fährt zum Ort.
    PauseAndFocus,
}

impl EventReaction {
    const ALL: [EventReaction; 3] = [
        EventReaction::ChronicleOnly,
        EventReaction::Pause,
        EventReaction::PauseAndFocus,
    ];
    fn label(self) -> &'static str {
        match self {
            EventReaction::ChronicleOnly => "Nur Chronik",
            EventReaction::Pause => "Anhalten",
            EventReaction::PauseAndFocus => "Anhalten und hinfahren",
        }
    }
}

/// Höchstens so viel Simulationszeit pro Bild, damit das Fenster reagiert.
const MAX_HOURS_PER_FRAME: f64 = 24.0 * 365.0;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Nature in a Glass")
            .with_inner_size([1440.0, 900.0])
            .with_min_inner_size([900.0, 600.0]),
        ..Default::default()
    };
    // Optionale Startparameter, z. B. `--seed 7 --years 100 --view karte`
    let arg = |name: &str| std::env::args().skip_while(|a| a != name).nth(1);
    let seed = arg("--seed").and_then(|s| s.parse().ok()).unwrap_or(42);
    let years: u64 = arg("--years").and_then(|s| s.parse().ok()).unwrap_or(0);
    let view = arg("--view");
    let layer = arg("--layer");
    let speed: Option<usize> = arg("--speed").and_then(|s| s.parse().ok());
    let days: u64 = arg("--days").and_then(|s| s.parse().ok()).unwrap_or(0);
    let reaction: Option<usize> = arg("--reaction").and_then(|s| s.parse().ok());
    // `--tiere hirsche,kaninchen,woelfe,fuechse` setzt nach `--years` Jahren Tiere
    // aus; `--danach N` lässt sie anschließend N Jahre leben.
    let release: Vec<Species> = arg("--tiere")
        .map(|s| {
            s.split(',')
                .filter_map(|t| match t.trim() {
                    "hirsche" => Some(Species::Deer),
                    "kaninchen" => Some(Species::Rabbit),
                    "woelfe" => Some(Species::Wolf),
                    "fuechse" => Some(Species::Fox),
                    _ => None,
                })
                .collect()
        })
        .unwrap_or_default();
    let after: u64 = arg("--danach").and_then(|s| s.parse().ok()).unwrap_or(0);
    // `--menschen` lässt nach `--years` Jahren ein Boot landen, `--bewohner`
    // öffnet das Fenster mit den Lebensgeschichten.
    let people = std::env::args().any(|a| a == "--menschen");
    let people_window = std::env::args().any(|a| a == "--bewohner");
    // `--folgen` richtet die Kamera (mit `--zoom`) auf den ersten Menschen aus.
    let follow = std::env::args().any(|a| a == "--folgen");
    // `--flaechen` startet mit Flächenfarben statt Grafik
    let no_graphics = std::env::args().any(|a| a == "--flaechen");
    let zoom: Option<f32> = arg("--zoom").and_then(|s| s.parse().ok());
    let select: Option<(u32, u32)> = arg("--select").and_then(|s| {
        let (x, y) = s.split_once(',')?;
        Some((x.trim().parse().ok()?, y.trim().parse().ok()?))
    });
    eframe::run_native(
        "Nature in a Glass",
        options,
        Box::new(move |cc| {
            let mut app = App::new(seed);
            app.atlas = atlas::Atlas::load(&cc.egui_ctx);
            app.graphics = app.atlas.is_some() && !no_graphics;
            app.sim.advance_years(years);
            for &species in &release {
                app.sim
                    .apply(Intervention::ReleaseAnimals { species, at: None });
            }
            if people {
                app.sim.apply(Intervention::PeopleArrive { at: None });
            }
            app.sim.advance_years(after);
            app.sim.advance_days(days);
            app.seen_event = app.last_event_id();
            if people_window {
                app.show_people = true;
                app.bio_person = app.sim.people.list.first().map(|p| p.id);
            }
            if follow && let (Some(z), Some(p)) = (zoom, app.sim.people.list.first()) {
                let i = p.tile as usize;
                app.selected = Some(i);
                app.camera.zoom = z.clamp(0.4, view::MAX_ZOOM);
                app.camera.pan = view::focus_pan(&app.sim.world, i, app.mode, app.camera.zoom);
            }
            if let Some(r) = reaction {
                app.reaction = EventReaction::ALL[r.min(2)];
            }
            if view.as_deref() == Some("karte") {
                app.mode = ViewMode::Map;
            }
            if let Some(l) = layer {
                app.layer = match l.as_str() {
                    "hoehe" => Layer::Height,
                    "feuchte" => Layer::Moisture,
                    "wasser" => Layer::Water,
                    "brand" => Layer::FireRisk,
                    "boden" => Layer::Soil,
                    "tiere" => Layer::Animals,
                    _ => Layer::Nature,
                };
            }
            if let Some(z) = zoom {
                app.camera.zoom = z.clamp(0.4, view::MAX_ZOOM);
            }
            if let Some((x, y)) = select {
                let w = &app.sim.world;
                let i = w.idx(x.min(w.width - 1), y.min(w.height - 1));
                app.selected = Some(i);
                if zoom.is_some() {
                    app.camera.pan = view::focus_pan(w, i, app.mode, app.camera.zoom);
                }
            }
            if let Some(k) = speed {
                app.speed = k.min(SPEEDS.len() - 1);
                app.running = true;
            }
            app.sync_pending();
            Ok(Box::new(app))
        }),
    )
}

/// Zeitgeber: lässt die Simulation in einem eigenen Faden laufen – auch wenn
/// das Fenster minimiert ist und keine Bilder gezeichnet werden. Während ein
/// Bild gezeichnet wird, leiht sich die Oberfläche die Simulation aus.
struct Clock {
    sim: Simulation,
    running: bool,
    speed: usize,
    reaction: EventReaction,
    owed_hours: f64,
    seen_event: u64,
    /// Die Oberfläche hält gerade die Simulation.
    ui_busy: bool,
    stop_message: Option<String>,
    focus: Option<usize>,
}

impl Clock {
    fn last_event_id(&self) -> u64 {
        self.sim.journal.events.last().map(|e| e.id).unwrap_or(0)
    }

    fn advance(&mut self, dt: f64) {
        if !self.running {
            return;
        }
        self.owed_hours += SPEEDS[self.speed].1 * dt;
        self.owed_hours = self.owed_hours.min(MAX_HOURS_PER_FRAME);
        let whole = self.owed_hours.floor();
        if whole < 1.0 {
            return;
        }
        self.owed_hours -= whole;
        if self.reaction == EventReaction::ChronicleOnly {
            self.sim.advance_hours(whole as u64);
            self.seen_event = self.last_event_id();
            return;
        }
        // Tageweise vorrücken, damit wir genau beim Ereignis anhalten
        let mut left = whole as u64;
        while left > 0 {
            let to_next_day = HOURS_PER_DAY - self.sim.time_hours % HOURS_PER_DAY;
            let step = left.min(to_next_day);
            self.sim.advance_hours(step);
            left -= step;
            if self.check_important_events() {
                self.owed_hours = 0.0;
                break;
            }
        }
    }

    /// Prüft neue Ereignisse; hält bei wichtigen an. Gibt `true` zurück, wenn angehalten.
    fn check_important_events(&mut self) -> bool {
        let seen = self.seen_event;
        self.seen_event = self.last_event_id();
        let found = self
            .sim
            .journal
            .events
            .iter()
            .rev()
            .take_while(|e| e.id > seen)
            .filter(|e| e.kind.is_important())
            .last()
            .cloned();
        let Some(e) = found else {
            return false;
        };
        self.running = false;
        self.stop_message = Some(format!("Angehalten: {}", e.message));
        if self.reaction == EventReaction::PauseAndFocus
            && let Some((x, y)) = e.location
        {
            let w = &self.sim.world;
            self.focus = Some(w.idx(x.min(w.width - 1), y.min(w.height - 1)));
        }
        true
    }
}

/// Startet den Faden des Zeitgebers.
fn spawn_clock(clock: Arc<Mutex<Clock>>) {
    std::thread::spawn(move || {
        let mut last = Instant::now();
        loop {
            std::thread::sleep(Duration::from_millis(4));
            let mut c = clock.lock().unwrap_or_else(|e| e.into_inner());
            let now = Instant::now();
            if c.ui_busy {
                continue; // Zeit läuft weiter und wird gleich nachgeholt
            }
            let dt = (now - last).as_secs_f64().min(0.25);
            last = now;
            c.advance(dt);
        }
    });
}

struct App {
    clock: Arc<Mutex<Clock>>,
    /// Ob die Oberfläche gerade die echte Simulation hält.
    holds_sim: bool,
    sim: Simulation,
    running: bool,
    speed: usize,
    owed_hours: f64,
    mode: ViewMode,
    layer: Layer,
    camera: Camera,
    selected: Option<usize>,
    seed_text: String,
    show_year_summaries: bool,
    /// Verlauf: Tierbestände statt Pflanzen.
    history_animals: bool,
    /// Fenster „Bewohner“ und die Person, deren Biografie gezeigt wird.
    show_people: bool,
    bio_person: Option<u32>,
    /// Codex' Bilder (falls eingebettet) und ob sie gezeigt werden.
    atlas: Option<atlas::Atlas>,
    graphics: bool,
    status: String,
    reaction: EventReaction,
    /// Höchste Ereignis-ID, die der Beobachter schon geprüft hat.
    seen_event: u64,
    // Eingriffe: Werte während des Ziehens, angewendet beim Loslassen
    pending_rain: f32,
    pending_temp: f32,
    pending_springs: Vec<f32>,
}

impl App {
    fn new(seed: u64) -> Self {
        let sim = Self::fresh(seed);
        // Platzhalter, den der Zeitgeber hält, solange die Oberfläche die
        // echte Simulation hat (winzige Insel, kostet nichts).
        let placeholder = Simulation::with_size(seed, 8, 8);
        let clock = Arc::new(Mutex::new(Clock {
            sim: placeholder,
            running: false,
            speed: 1,
            reaction: EventReaction::ChronicleOnly,
            owed_hours: 0.0,
            seen_event: 0,
            ui_busy: true,
            stop_message: None,
            focus: None,
        }));
        spawn_clock(clock.clone());
        let mut app = Self {
            clock,
            holds_sim: true,
            running: false,
            speed: 1,
            owed_hours: 0.0,
            mode: ViewMode::Iso,
            layer: Layer::Nature,
            camera: Camera::default(),
            selected: None,
            seed_text: seed.to_string(),
            show_year_summaries: false,
            history_animals: false,
            show_people: false,
            bio_person: None,
            atlas: None,
            graphics: false,
            status: String::new(),
            reaction: EventReaction::ChronicleOnly,
            seen_event: 0,
            pending_rain: 0.0,
            pending_temp: 0.0,
            pending_springs: Vec::new(),
            sim,
        };
        app.sync_pending();
        app.seen_event = app.sim.journal.events.last().map(|e| e.id).unwrap_or(0);
        app
    }

    /// Neue Insel, Uhr auf 10 Uhr morgens gestellt (innerhalb des ersten
    /// Tages läuft dabei noch kein Tagesprozess).
    fn fresh(seed: u64) -> Simulation {
        let mut sim = Simulation::new(seed);
        sim.advance_hours(10);
        sim
    }

    fn sync_pending(&mut self) {
        self.pending_rain = self.sim.climate.annual_rain_mm;
        self.pending_temp = self.sim.climate.mean_temp_c;
        self.pending_springs = self.sim.world.springs.iter().map(|s| s.factor).collect();
    }

    fn replace_sim(&mut self, sim: Simulation) {
        self.seen_event = sim.journal.events.last().map(|e| e.id).unwrap_or(0);
        self.sim = sim;
        self.selected = None;
        self.owed_hours = 0.0;
        self.sync_pending();
    }

    /// Spielstände liegen im Ordner `spielstaende` neben dem Programm.
    fn save_path() -> PathBuf {
        let base = std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|d| d.to_path_buf()))
            .unwrap_or_else(|| PathBuf::from("."));
        base.join("spielstaende").join("welt.json")
    }

    fn save(&mut self) {
        let path = Self::save_path();
        let result = std::fs::create_dir_all(path.parent().unwrap())
            .and_then(|_| std::fs::write(&path, self.sim.to_json()));
        self.status = match result {
            Ok(()) => format!("Gespeichert: {}", path.display()),
            Err(e) => format!("Speichern fehlgeschlagen: {e}"),
        };
    }

    fn load(&mut self) {
        let path = Self::save_path();
        match std::fs::read_to_string(&path) {
            Ok(json) => match Simulation::from_json(&json) {
                Ok(sim) => {
                    self.replace_sim(sim);
                    self.status = format!("Geladen: {}", path.display());
                }
                Err(e) => self.status = e,
            },
            Err(_) => self.status = format!("Kein Spielstand unter {}", path.display()),
        }
    }

    fn last_event_id(&self) -> u64 {
        self.sim.journal.events.last().map(|e| e.id).unwrap_or(0)
    }

    /// Übernimmt die Simulation vom Zeitgeber für die Dauer eines Bildes.
    fn take_from_clock(&mut self) {
        let mut c = self.clock.lock().unwrap_or_else(|e| e.into_inner());
        if self.holds_sim {
            return;
        }
        std::mem::swap(&mut self.sim, &mut c.sim);
        self.holds_sim = true;
        c.ui_busy = true;
        self.running = c.running;
        self.owed_hours = c.owed_hours;
        self.seen_event = c.seen_event;
        if let Some(msg) = c.stop_message.take() {
            self.status = msg;
        }
        if let Some(i) = c.focus.take() {
            self.selected = Some(i);
            self.camera.pan = view::focus_pan(&self.sim.world, i, self.mode, self.camera.zoom);
        }
    }

    /// Gibt die Simulation an den Zeitgeber zurück, samt Einstellungen.
    fn give_to_clock(&mut self) {
        let mut c = self.clock.lock().unwrap_or_else(|e| e.into_inner());
        std::mem::swap(&mut self.sim, &mut c.sim);
        self.holds_sim = false;
        c.running = self.running;
        c.speed = self.speed;
        c.reaction = self.reaction;
        c.owed_hours = self.owed_hours;
        c.seen_event = self.seen_event;
        c.ui_busy = false;
    }

    fn season_name(&self) -> &'static str {
        match self.sim.day_of_year() {
            0..=58 | 334..=364 => "Winter",
            59..=151 => "Frühling",
            152..=243 => "Sommer",
            _ => "Herbst",
        }
    }

    /// Zeitanzeige mit fester Zeichenzahl, damit nichts daneben verrutscht.
    fn time_text(&self) -> String {
        format!(
            "Jahr {:>5} · Tag {:>3} · {:02}:00 · {:<8}",
            self.sim.year(),
            self.sim.day_of_year() + 1,
            self.sim.hour_of_day(),
            self.season_name()
        )
    }

    fn top_bar(&mut self, ui: &mut egui::Ui) {
        // Zeile 1: Zeit und Zeitraffer
        ui.horizontal(|ui| {
            ui.add_sized(
                [150.0, 24.0],
                egui::Label::new(RichText::new("Nature in a Glass").strong().size(16.0)),
            );
            ui.separator();
            ui.add_sized(
                [390.0, 24.0],
                egui::Label::new(RichText::new(self.time_text()).monospace().size(15.0)),
            );
            ui.separator();
            let play = if self.running { "Pause" } else { "▶ Weiter" };
            if ui
                .add_sized([90.0, 24.0], egui::Button::new(play))
                .on_hover_text("Leertaste")
                .clicked()
            {
                self.running = !self.running;
            }
            for (k, (label, _)) in SPEEDS.iter().enumerate() {
                let b = egui::Button::selectable(self.speed == k, *label);
                if ui.add_sized([82.0, 24.0], b).clicked() {
                    self.speed = k;
                    self.running = true;
                }
            }
            if ui
                .add_sized([70.0, 24.0], egui::Button::new("+1 Tag"))
                .clicked()
            {
                self.sim.advance_hours(HOURS_PER_DAY);
            }
        });
        // Zeile 2: Ansicht, Spielstand, neue Insel
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.mode, ViewMode::Iso, "Isometrisch");
            ui.selectable_value(&mut self.mode, ViewMode::Map, "Karte");
            egui::ComboBox::from_id_salt("layer")
                .width(130.0)
                .selected_text(self.layer.label())
                .show_ui(ui, |ui| {
                    for l in Layer::ALL {
                        ui.selectable_value(&mut self.layer, l, l.label());
                    }
                });
            let gfx_ok = self.atlas.is_some() && self.mode == ViewMode::Iso;
            if ui
                .add_enabled(gfx_ok, egui::Button::selectable(self.graphics, "Grafik"))
                .on_hover_text(
                    "Isometrische Ansicht mit den gezeichneten Bildern (Ebene Natur). \
                     Aus: schlichte Flächenfarben. Die Grafik ist ein früher Stand.",
                )
                .clicked()
            {
                self.graphics = !self.graphics;
            }
            ui.separator();
            ui.label("Wichtige Ereignisse:");
            egui::ComboBox::from_id_salt("reaction")
                .width(170.0)
                .selected_text(self.reaction.label())
                .show_ui(ui, |ui| {
                    for r in EventReaction::ALL {
                        ui.selectable_value(&mut self.reaction, r, r.label());
                    }
                })
                .response
                .on_hover_text(
                    "Wichtig sind: Brand, Dürre, Waldsterben, letzter Baum, \
                     Rückkehr der Bäume, Keimung eines Samens, versiegte Quelle, \
                     Grundwasser-Tiefstand.",
                );
            ui.separator();
            if ui.button("Speichern").clicked() {
                self.save();
            }
            if ui.button("Laden").clicked() {
                self.load();
            }
            ui.separator();
            ui.label("Seed");
            ui.add(egui::TextEdit::singleline(&mut self.seed_text).desired_width(70.0));
            if ui.button("Neue Insel").clicked() {
                match self.seed_text.trim().parse::<u64>() {
                    Ok(seed) => {
                        self.replace_sim(Self::fresh(seed));
                        self.camera = Camera::default();
                        self.status = format!("Neue Insel aus Seed {seed}");
                    }
                    Err(_) => self.status = "Seed muss eine ganze Zahl sein".into(),
                }
            }
            if ui.button("🎲").on_hover_text("Zufälliger Seed").clicked() {
                let t = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_nanos() as u64)
                    .unwrap_or(1);
                self.seed_text = (t % 1_000_000).to_string();
            }
        });
    }

    /// Statuszeile am unteren Rand – immer gleich hoch, auch wenn leer.
    fn status_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.set_height(20.0);
            let text = if self.status.is_empty() {
                "Ziehen = Karte verschieben · Mausrad = Zoom · Klick = Feld untersuchen · Leertaste = Pause"
            } else {
                self.status.as_str()
            };
            ui.add(egui::Label::new(RichText::new(text).weak()).truncate());
        });
    }

    /// Rechte Spalte: alle Abschnitte haben eine feste Höhe, nur die
    /// Chronik darunter scrollt in ihrem eigenen Bereich.
    fn side_panel(&mut self, ui: &mut egui::Ui) {
        self.island_section(ui);
        ui.separator();
        self.intervention_section(ui);
        self.seed_row(ui);
        self.animal_row(ui);
        self.people_row(ui);
        ui.separator();
        self.history_section(ui);
        ui.separator();
        self.journal_section(ui);
    }

    fn island_section(&mut self, ui: &mut egui::Ui) {
        let snap = self.sim.snapshot();
        let wx = &self.sim.weather;
        ui.heading("Insel");
        egui::Grid::new("island")
            .num_columns(2)
            .min_col_width(170.0)
            .striped(true)
            .show(ui, |ui| {
                let row = |ui: &mut egui::Ui, k: &str, v: String| {
                    let r = ui.label(k);
                    ui.add(egui::Label::new(v).truncate());
                    ui.end_row();
                    r
                };
                let weather = if wx.storm {
                    format!("Starkregen, {:.0} mm", wx.rain_mm)
                } else if wx.rain_mm > 0.5 {
                    format!("Regen, {:.0} mm", wx.rain_mm)
                } else {
                    "trocken".into()
                };
                let weather = if wx.dry_thunder {
                    "trockenes Gewitter".to_string()
                } else if wx.thunder {
                    format!("{weather}, Gewitter")
                } else {
                    weather
                };
                row(ui, "Wetter heute", weather);
                row(ui, "Wind", wx.wind_text());
                row(ui, "Temperatur (Küste)", format!("{:.1} °C", wx.temp_c));
                row(
                    ui,
                    "Wald / Busch / Gras",
                    format!(
                        "{:.0} % / {:.0} % / {:.0} %",
                        snap.forest_pct, snap.shrub_pct, snap.grass_pct
                    ),
                );
                row(ui, "Totholz", format!("{:.1} %", snap.deadwood_pct));
                row(
                    ui,
                    "Boden- / Tiefenfeuchte",
                    format!(
                        "{:.0} % / {:.0} %",
                        snap.moisture_pct, snap.deep_moisture_pct
                    ),
                );
                row(
                    ui,
                    "Bäche dauernd / zeitw. / Regen",
                    format!(
                        "{} / {} / {} Felder",
                        snap.permanent_tiles,
                        snap.seasonal_tiles + snap.seasonal_dry_tiles,
                        snap.rain_channel_tiles
                    ),
                )
                .on_hover_text(
                    "Felder mit Wasserlauf. Dauerhaft: fließt fast das ganze Jahr. \
                     Zeitweise: fließt einen Teil des Jahres, sonst trockenes Bett. \
                     Regen: Rinne, die nur nach Regen Wasser führt.",
                );
                row(
                    ui,
                    "Abfluss ins Meer",
                    format!(
                        "{} m³ heute",
                        thousands(self.sim.last_water.to_sea * M3_PER_TILE_MM)
                    ),
                );
                row(
                    ui,
                    "Seen",
                    format!(
                        "{} benannt · {} Wasserfelder",
                        self.sim.lakes.current().count(),
                        snap.pond_tiles
                    ),
                )
                .on_hover_text(
                    "Zusammenhängende stehende Gewässer ab drei Feldern bekommen einen \
                     Namen. Wasserfelder zählt auch kleine Tümpel.",
                );
                let (soil_m3, soil_depth) = self
                    .sim
                    .history
                    .last()
                    .map(|y| (y.eroded_m3, y.soil_depth_m))
                    .unwrap_or((0.0, self.sim.mean_soil_depth()));
                row(
                    ui,
                    "Boden",
                    format!(
                        "Ø {} m · Abtrag {} m³/J.",
                        de(soil_depth, 2),
                        thousands(soil_m3)
                    ),
                )
                .on_hover_text(
                    "Mittlere Dicke der Bodenschicht und wie viel Boden das Wasser im \
                     letzten Jahr gelöst hat (m³ pro Jahr). Pflanzen schützen den Boden, Brandflächen \
                     kaum. Was abgetragen wird, lagert sich in Senken, Seen und vor der \
                     Küste wieder ab.",
                );
                row(
                    ui,
                    "Grundwasser",
                    format!(
                        "{:.0} % · {} Mio. m³",
                        self.sim.groundwater_level() * 100.0,
                        de(self.sim.groundwater_m3() / 1_000_000.0, 2)
                    ),
                )
                .on_hover_text(
                    "Füllstand des Grundwasserkörpers der Insel. Er füllt sich durch \
                     Sickerwasser und speist die Quellen. Unter 5 % liegt das Wasser \
                     unter den Quellaustritten – die Quellen versiegen. Ist er voll, \
                     fließt weiteres Sickerwasser unterirdisch ins Meer.",
                );
                let an = &self.sim.animals;
                row(
                    ui,
                    "Hirsche / Kaninchen",
                    format!(
                        "{} / {}",
                        an.total(Species::Deer),
                        thousands(an.total(Species::Rabbit) as f32)
                    ),
                );
                row(
                    ui,
                    "Wölfe / Füchse",
                    format!("{} / {}", an.total(Species::Wolf), an.total(Species::Fox)),
                );
                let pe = &self.sim.people;
                ui.label("Menschen");
                if pe.alive() + pe.departed.len() == 0 {
                    ui.label("–");
                } else {
                    let kids = pe.list.iter().filter(|p| p.is_child()).count();
                    let text = if kids > 0 {
                        format!(
                            "{} leben ({kids} Kinder) · {} verstorben  ›",
                            pe.alive(),
                            pe.departed.len()
                        )
                    } else {
                        format!(
                            "{} leben · {} verstorben  ›",
                            pe.alive(),
                            pe.departed.len()
                        )
                    };
                    if ui
                        .link(text)
                        .on_hover_text("Bewohner und ihre Lebensgeschichten")
                        .clicked()
                    {
                        self.show_people = true;
                    }
                }
                ui.end_row();
                row(
                    ui,
                    "Feuer",
                    if snap.burning_tiles > 0 {
                        if snap.burning_tiles == 1 {
                            "1 Feld brennt".to_string()
                        } else {
                            format!("{} Felder brennen", snap.burning_tiles)
                        }
                    } else {
                        "–".into()
                    },
                );
            });
    }

    /// Feld-Infos: immer dieselben Zeilen, damit die Spalte nicht springt.
    fn tile_section(&mut self, ui: &mut egui::Ui) {
        let dash = || "–".to_string();
        let mut rows: Vec<(&str, String)> = vec![
            ("Position", dash()),
            ("Gelände", dash()),
            ("Höhe", dash()),
            ("Temperatur", dash()),
            ("Bodenfeuchte", dash()),
            ("Tiefenwasser", dash()),
            ("Oberflächenwasser", dash()),
            ("Durchfluss", dash()),
            ("Gras / Busch / Wald", dash()),
            ("Waldalter", dash()),
            ("Wasser für Bäume", dash()),
            ("Totholz", dash()),
            ("Feuer", dash()),
            ("Trockentage", dash()),
            ("Boden", dash()),
            ("Tiere", dash()),
            ("Menschen", dash()),
        ];
        let mut here: Vec<(u32, String)> = Vec::new();
        if let Some(i) = self.selected {
            let world = &self.sim.world;
            let t = &world.tiles[i];
            let (x, y) = world.xy(i);
            let mut kind = match world.terrain(i) {
                Terrain::DeepOcean => "Tiefsee",
                Terrain::ShallowOcean => "Flachwasser",
                Terrain::Beach => "Strand",
                Terrain::Lowland => "Niederung",
                Terrain::Hills => "Hügelland",
                Terrain::Mountain => "Bergland",
            }
            .to_string();
            if !t.is_ocean() {
                if let Some(name) = self.sim.lakes.name_at(i as u32) {
                    kind.push_str(", ");
                    kind.push_str(name);
                } else if t.surface_water > POND_MM {
                    kind.push_str(", Teich");
                } else if t.flow > STREAM_FLOW {
                    kind.push_str(", Bach");
                }
                if world.springs.iter().any(|s| s.x == x && s.y == y) {
                    kind.push_str(", Quelle");
                }
            }
            rows[0].1 = format!("{x}, {y}");
            rows[1].1 = kind;
            rows[2].1 = format!("{:.0} m", t.elevation);
            if !t.is_ocean() {
                rows[3].1 = format!("{:.1} °C", self.sim.weather.temp_at(t.elevation));
                rows[4].1 = format!(
                    "{:.0} % von {:.0} mm",
                    t.moisture_fraction() * 100.0,
                    t.soil_capacity
                );
                rows[5].1 = format!(
                    "{:.0} % von {:.0} mm",
                    t.deep_fraction() * 100.0,
                    t.deep_capacity()
                );
                rows[6].1 = format!("{:.0} mm", t.surface_water);
                let wc = watercourse(t);
                let kind = match (wc.kind, wc.flowing) {
                    (CourseKind::Permanent, _) => " · dauerhafter Bach",
                    (CourseKind::Seasonal, true) => " · zeitweiser Bach",
                    (CourseKind::Seasonal, false) => " · trockenes Bachbett",
                    (CourseKind::RainChannel, _) => " · Regenrinne",
                    _ => "",
                };
                rows[7].1 = format!("{} L/s{kind}", de(liters_per_second(t.flow), 1));
                rows[8].1 = format!(
                    "{:.0} % / {:.0} % / {:.0} %",
                    t.grass * 100.0,
                    t.shrub * 100.0,
                    t.forest * 100.0
                );
                rows[9].1 = if t.forest > 0.0 {
                    format!("{:.0} Jahre", t.forest_age)
                } else {
                    dash()
                };
                rows[10].1 = if t.forest > 0.0 {
                    format!("{:.0} %", t.tree_supply * 100.0)
                } else {
                    dash()
                };
                rows[11].1 = format!("{:.0} %", t.deadwood * 100.0);
                rows[12].1 = if t.fire > 0.0 {
                    "brennt".into()
                } else if t.burn_scar > 0.05 {
                    format!("Brandspur {:.0} %", t.burn_scar * 100.0)
                } else {
                    dash()
                };
                rows[13].1 = t.drought_days.to_string();
                // Höhenänderung je Jahrhundert, gerundet auf ganze Millimeter
                let per_century = t.erosion_rate * 100_000.0;
                let trend = if per_century <= -1.0 {
                    format!(" · −{:.0} mm/100 J.", -per_century)
                } else if per_century >= 1.0 {
                    format!(" · +{:.0} mm/100 J.", per_century)
                } else {
                    " · stabil".to_string()
                };
                rows[14].1 = format!("{} m{trend}", de(t.soil_depth + t.soil_change, 2));
                let an = &self.sim.animals;
                let mut parts = Vec::new();
                for sp in [Species::Deer, Species::Rabbit, Species::Wolf, Species::Fox] {
                    let n = an.count_at(i as u32, sp);
                    if n > 0 {
                        let (one, many) = sp.names();
                        parts.push(format!("{n} {}", if n == 1 { one } else { many }));
                    }
                }
                if !parts.is_empty() {
                    rows[15].1 = parts.join(" · ");
                }
                here = self
                    .sim
                    .people
                    .list
                    .iter()
                    .filter(|p| p.tile == i as u32)
                    .map(|p| (p.id, p.name.clone()))
                    .collect();
                if here.is_empty() && self.sim.people.bodies.iter().any(|b| b.tile == i as u32) {
                    rows[16].1 = "ein Toter liegt hier".into();
                }
            }
        }
        egui::Grid::new("tile")
            .num_columns(2)
            .min_col_width(130.0)
            .striped(true)
            .show(ui, |ui| {
                for (k, v) in rows {
                    ui.label(k);
                    if k == "Menschen" && !here.is_empty() {
                        // Namen anklickbar: öffnet die Biografie
                        ui.horizontal(|ui| {
                            for (id, name) in &here {
                                if ui.link(name).clicked() {
                                    self.bio_person = Some(*id);
                                    self.show_people = true;
                                }
                            }
                        });
                    } else {
                        ui.add(egui::Label::new(v).truncate());
                    }
                    ui.end_row();
                }
            });
    }

    fn intervention_section(&mut self, ui: &mut egui::Ui) {
        ui.heading("Eingriffe");
        ui.label(RichText::new("Du veränderst Bedingungen, nicht das, was daraus folgt.").weak());
        let commit = |r: &egui::Response| r.drag_stopped() || (r.changed() && !r.dragged());

        let r = ui.add(
            egui::Slider::new(&mut self.pending_rain, 0.0..=3000.0)
                .text("Jahresregen (mm)")
                .step_by(10.0),
        );
        if commit(&r) && self.pending_rain != self.sim.climate.annual_rain_mm {
            self.sim.apply(Intervention::SetAnnualRain {
                mm: self.pending_rain,
            });
        }
        let r = ui.add(
            egui::Slider::new(&mut self.pending_temp, -5.0..=25.0)
                .text("Jahresmittel (°C)")
                .step_by(0.5),
        );
        if commit(&r) && self.pending_temp != self.sim.climate.mean_temp_c {
            self.sim.apply(Intervention::SetMeanTemp {
                celsius: self.pending_temp,
            });
        }
        let n = self.sim.world.springs.len();
        self.pending_springs.resize(n, 1.0);
        for k in 0..n {
            let s = &self.sim.world.springs[k];
            let label = if s.dry {
                format!("Quelle {} · versiegt", k + 1)
            } else {
                format!(
                    "Quelle {} · {:>4} L/s",
                    k + 1,
                    de(liters_per_second(s.current), 1)
                )
            };
            let mut pct = self.pending_springs[k] * 100.0;
            let r = ui
                .add(
                    egui::Slider::new(&mut pct, 0.0..=300.0)
                        .text(label)
                        .suffix(" %"),
                )
                .on_hover_text(
                    "Wie leicht das Grundwasser hier austritt. Das Wasser selbst stammt \
                     aus dem Grundwasser der Insel: Mehr, als dort nachsickert, kann auf \
                     Dauer nicht herauskommen. Bei hohem Wert leert sich das Grundwasser, \
                     und die Quelle wird wieder schwächer.",
                );
            self.pending_springs[k] = pct / 100.0;
            if commit(&r) && self.pending_springs[k] != self.sim.world.springs[k].factor {
                self.sim.apply(Intervention::SetSpringFactor {
                    spring: k,
                    factor: self.pending_springs[k],
                });
            }
        }
    }

    /// Gelände am gewählten Feld verändern: Damm aufschütten oder Durchbruch graben.
    fn terrain_buttons(&mut self, ui: &mut egui::Ui) {
        let Some(i) = self.selected else { return };
        let land = !self.sim.world.tiles[i].is_ocean();
        let (x, y) = self.sim.world.xy(i);
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            ui.add_enabled_ui(land, |ui| {
                let hint = "Verändert nur das Gelände. Wohin das Wasser dann fließt, \
                            entscheidet die Insel.";
                if ui.button("Damm +2 m").on_hover_text(hint).clicked() {
                    self.sim
                        .apply(Intervention::ChangeTerrain { x, y, meters: 2.0 });
                }
                if ui.button("Durchbruch −2 m").on_hover_text(hint).clicked() {
                    self.sim
                        .apply(Intervention::ChangeTerrain { x, y, meters: -2.0 });
                }
            });
        });
    }

    fn seed_row(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Samen fallen lassen:");
            for kind in [SeedKind::Tree, SeedKind::Shrub] {
                if ui
                    .button(kind.label())
                    .on_hover_text(
                        "Ein einzelner Samen landet an einem zufälligen Ort. Wo, verrät \
                         erst eine Keimung in der Chronik – falls die Bedingungen passen.",
                    )
                    .clicked()
                {
                    self.sim.apply(Intervention::ScatterSeed { kind });
                }
            }
            ui.label(RichText::new(format!("im Boden: {:>2}", self.sim.seeds.len())).weak());
        });
    }

    fn animal_row(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Tiere aussetzen:");
            let at = self.selected.and_then(|i| {
                let t = &self.sim.world.tiles[i];
                (!t.is_ocean() && t.surface_water <= POND_MM).then(|| self.sim.world.xy(i))
            });
            for sp in [Species::Deer, Species::Rabbit, Species::Wolf, Species::Fox] {
                let hint = match sp {
                    Species::Deer => "6 Hirsche (2 männlich, 4 weiblich).",
                    Species::Rabbit => "20 Kaninchen.",
                    Species::Wolf => "Ein Rudel aus 4 Wölfen. Sie brauchen viele Hirsche.",
                    Species::Fox => "4 Füchse. Sie brauchen Kaninchen.",
                };
                let place = if at.is_some() {
                    "Sie kommen auf das gewählte Feld."
                } else {
                    "Ohne gewähltes Feld landen sie an einer zufälligen Stelle – \
                     Räuber dort, wo es Beute gibt."
                };
                if ui
                    .small_button(sp.names().1)
                    .on_hover_text(format!(
                        "{hint} {place} Was danach geschieht, entscheidet die Insel. \
                         Ausgestorbene Arten kommen nur zurück, wenn du neue aussetzt."
                    ))
                    .clicked()
                {
                    self.sim
                        .apply(Intervention::ReleaseAnimals { species: sp, at });
                }
            }
        });
    }

    fn people_row(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Menschen:");
            let at = self.selected.and_then(|i| {
                let t = &self.sim.world.tiles[i];
                (!t.is_ocean() && t.surface_water <= POND_MM).then(|| self.sim.world.xy(i))
            });
            let place = if at.is_some() {
                "Es landet am gewählten Feld."
            } else {
                "Ohne gewähltes Feld landet es an einer zufälligen Stelle der Küste."
            };
            if ui
                .small_button("Boot landen lassen")
                .on_hover_text(format!(
                    "Ein Boot mit fünf Menschen kommt auf die Insel. {place} Danach                      handeln sie selbst: Du kannst ihnen nichts befehlen."
                ))
                .clicked()
            {
                self.sim.apply(Intervention::PeopleArrive { at });
            }
        });
    }

    /// Fenster mit allen Bewohnern (lebend und verstorben) und der
    /// Lebensgeschichte der gewählten Person.
    fn people_window(&mut self, ctx: &egui::Context) {
        if !self.show_people {
            return;
        }
        let mut open = true;
        let year_day = |h: u64| {
            let d = h / HOURS_PER_DAY;
            format!("J{} T{}", d / 365 + 1, d % 365)
        };
        egui::Window::new("Bewohner")
            .open(&mut open)
            .fixed_size([620.0, 310.0])
            .default_pos([16.0, 80.0])
            .collapsible(false)
            .show(ctx, |ui| {
                let pe = &self.sim.people;
                ui.horizontal_top(|ui| {
                    ui.vertical(|ui| {
                        ui.set_width(200.0);
                        egui::ScrollArea::vertical()
                            .id_salt("people_list")
                            .max_height(300.0)
                            .show(ui, |ui| {
                                for p in &pe.list {
                                    let sel = self.bio_person == Some(p.id);
                                    let text = RichText::new(format!(
                                        "● {} · {}",
                                        p.name,
                                        age_short(p.age_days)
                                    ))
                                    .color(view::person_color(p.look));
                                    if ui.selectable_label(sel, text).clicked() {
                                        self.bio_person = Some(p.id);
                                    }
                                }
                                if !pe.departed.is_empty() {
                                    ui.separator();
                                    ui.label(RichText::new("Verstorben").weak());
                                }
                                for d in pe.departed.iter().rev() {
                                    let sel = self.bio_person == Some(d.person.id);
                                    let text = RichText::new(format!(
                                        "† {} · {}",
                                        d.person.name,
                                        age_short(d.person.age_days)
                                    ))
                                    .weak();
                                    if ui.selectable_label(sel, text).clicked() {
                                        self.bio_person = Some(d.person.id);
                                    }
                                }
                            });
                    });
                    ui.separator();
                    ui.vertical(|ui| {
                        let alive = self
                            .bio_person
                            .and_then(|id| pe.list.iter().find(|p| p.id == id));
                        let dead = self
                            .bio_person
                            .and_then(|id| pe.departed.iter().find(|d| d.person.id == id));
                        let Some(p) = alive.or(dead.map(|d| &d.person)) else {
                            ui.label(RichText::new("Wähle links eine Person.").weak());
                            return;
                        };
                        ui.heading(&p.name);
                        let sex = match (p.sex, p.is_child()) {
                            (nig_core::people::Sex::Female, false) => "Frau",
                            (nig_core::people::Sex::Male, false) => "Mann",
                            (nig_core::people::Sex::Female, true) => "Mädchen",
                            (nig_core::people::Sex::Male, true) => "Junge",
                        };
                        if let Some(d) = dead {
                            ui.label(format!(
                                "{sex}, {} · {} ({})",
                                age_long(p.age_days),
                                d.cause.label(),
                                year_day(d.died_hours)
                            ));
                        } else {
                            let (x, y) = self.sim.world.xy(p.tile as usize);
                            let state = if p.pregnant_days > 0 {
                                format!("{}, erwartet ein Kind,", p.activity.label())
                            } else {
                                p.activity.label().to_string()
                            };
                            ui.label(format!(
                                "{sex}, {} · {state} bei ({x}, {y})",
                                age_long(p.age_days),
                            ));
                            ui.label(
                                RichText::new(format!(
                                    "Hunger {} · Durst {} · Gesundheit {} · trägt {} Rationen",
                                    level(p.hunger),
                                    level(p.thirst),
                                    level(1.0 - p.health),
                                    de(p.pack, 1)
                                ))
                                .weak(),
                            );
                            if ui.small_button("Zeigen").clicked() {
                                self.selected = Some(p.tile as usize);
                                self.camera.pan = view::focus_pan(
                                    &self.sim.world,
                                    p.tile as usize,
                                    self.mode,
                                    self.camera.zoom,
                                );
                            }
                        }
                        // Familie: Namen, Verstorbene mit †
                        let name_of = |id: u32| -> Option<String> {
                            pe.list
                                .iter()
                                .find(|q| q.id == id)
                                .map(|q| q.name.clone())
                                .or_else(|| {
                                    pe.departed
                                        .iter()
                                        .find(|d| d.person.id == id)
                                        .map(|d| format!("{} †", d.person.name))
                                })
                        };
                        let mut family = Vec::new();
                        let parents: Vec<String> = [p.mother, p.father]
                            .into_iter()
                            .filter_map(name_of)
                            .collect();
                        if !parents.is_empty() {
                            family.push(format!("Eltern: {}", parents.join(", ")));
                        }
                        if let Some(n) = name_of(p.partner) {
                            family.push(format!("Partner: {n}"));
                        }
                        let kids: Vec<String> =
                            p.children.iter().copied().filter_map(name_of).collect();
                        if !kids.is_empty() {
                            family.push(format!("Kinder: {}", kids.join(", ")));
                        }
                        if !family.is_empty() {
                            ui.add(
                                egui::Label::new(RichText::new(family.join(" · ")).weak()).wrap(),
                            );
                        }
                        ui.separator();
                        egui::ScrollArea::vertical()
                            .id_salt("bio")
                            .max_height(200.0)
                            .stick_to_bottom(true)
                            .show(ui, |ui| {
                                for e in &p.bio {
                                    ui.horizontal_wrapped(|ui| {
                                        ui.label(
                                            RichText::new(year_day(e.time_hours))
                                                .weak()
                                                .monospace(),
                                        );
                                        ui.label(&e.text);
                                    });
                                }
                            });
                    });
                });
            });
        if !open {
            self.show_people = false;
        }
    }

    fn history_section(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.heading("Verlauf");
            ui.add_space(8.0);
            ui.selectable_value(&mut self.history_animals, false, "Pflanzen");
            ui.selectable_value(&mut self.history_animals, true, "Tiere")
                .on_hover_text(
                    "Bestände zum Jahresende. Jede Linie ist auf ihren eigenen \
                     Höchstwert bezogen – so sieht man auch die wenigen Wölfe \
                     neben tausenden Kaninchen. Fahr mit der Maus über den Verlauf.",
                );
        });
        let hist = &self.sim.history;
        let width = ui.available_width();
        let (resp, painter) = ui.allocate_painter(egui::vec2(width, 110.0), egui::Sense::hover());
        let r = resp.rect.shrink2(egui::vec2(4.0, 6.0));
        painter.rect_filled(resp.rect, 4.0, ui.visuals().extreme_bg_color);
        let n = hist.len();
        let series: [(Color32, &str); 4] = if self.history_animals {
            [
                (Color32::from_rgb(176, 122, 68), "Hirsche"),
                (Color32::from_rgb(200, 184, 152), "Kaninchen"),
                (Color32::from_rgb(154, 160, 166), "Wölfe"),
                (Color32::from_rgb(210, 104, 44), "Füchse"),
            ]
        } else {
            [
                (Color32::from_rgb(60, 150, 70), "Wald"),
                (Color32::from_rgb(140, 150, 70), "Busch"),
                (Color32::from_rgb(190, 180, 90), "Gras"),
                (Color32::from_rgb(80, 150, 230), "Grundwasser"),
            ]
        };
        let value = |y: &nig_core::YearStats, si: usize| -> f32 {
            if self.history_animals {
                match si {
                    0 => y.deer as f32,
                    1 => y.rabbits as f32,
                    2 => y.wolves as f32,
                    _ => y.foxes as f32,
                }
            } else {
                match si {
                    0 => y.forest_pct,
                    1 => y.shrub_pct,
                    2 => y.grass_pct,
                    _ => y.aquifer_pct.min(100.0),
                }
            }
        };
        if n < 2 {
            painter.text(
                resp.rect.center(),
                egui::Align2::CENTER_CENTER,
                "Nach zwei Jahren erscheint hier der Verlauf.",
                egui::FontId::proportional(13.0),
                ui.visuals().weak_text_color(),
            );
            ui.horizontal(|ui| {
                for (col, name) in series {
                    ui.label(RichText::new(format!("■ {name}")).color(col));
                }
            });
            return;
        }
        let x_at = |k: usize| r.left() + r.width() * k as f32 / (n - 1) as f32;
        // Pflanzen in Prozent, Tiere je Linie auf ihren Höchstwert bezogen
        let scale: Vec<f32> = (0..4)
            .map(|si| {
                if self.history_animals {
                    hist.iter().map(|y| value(y, si)).fold(1.0f32, f32::max)
                } else {
                    100.0
                }
            })
            .collect();
        for (si, (col, _)) in series.iter().enumerate() {
            let pts: Vec<egui::Pos2> = hist
                .iter()
                .enumerate()
                .map(|(k, y)| {
                    let f = (value(y, si) / scale[si]).clamp(0.0, 1.0);
                    egui::pos2(x_at(k), r.bottom() - r.height() * f)
                })
                .collect();
            painter.add(egui::Shape::line(pts, egui::Stroke::new(1.5, *col)));
        }
        // Werte des Jahres unter dem Mauszeiger
        if let Some(m) = resp.hover_pos() {
            let k = (((m.x - r.left()) / r.width()) * (n - 1) as f32)
                .round()
                .clamp(0.0, (n - 1) as f32) as usize;
            let y = &hist[k];
            painter.vline(
                x_at(k),
                r.y_range(),
                egui::Stroke::new(1.0, Color32::from_white_alpha(60)),
            );
            let text = if self.history_animals {
                format!(
                    "Jahr {}: {} Hirsche, {} Kaninchen, {} Wölfe, {} Füchse",
                    y.year,
                    y.deer,
                    thousands(y.rabbits as f32),
                    y.wolves,
                    y.foxes
                )
            } else {
                format!(
                    "Jahr {}: Wald {:.0} %, Busch {:.0} %, Gras {:.0} %, Grundwasser {:.0} %",
                    y.year, y.forest_pct, y.shrub_pct, y.grass_pct, y.aquifer_pct
                )
            };
            resp.on_hover_text_at_pointer(text);
        }
        ui.horizontal(|ui| {
            for (col, name) in series {
                ui.label(RichText::new(format!("■ {name}")).color(col));
            }
            ui.label(RichText::new(format!("· Jahr 1–{}", n)).weak());
        });
    }

    fn journal_section(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.heading("Chronik");
            ui.checkbox(&mut self.show_year_summaries, "Jahresbilanzen");
        });
        let mut jump = None;
        let mut shown = 0;
        let events = &self.sim.journal.events;
        let world = &self.sim.world;
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                for e in events.iter().rev() {
                    if e.kind == EventKind::YearSummary && !self.show_year_summaries {
                        continue;
                    }
                    shown += 1;
                    if shown > 300 {
                        break;
                    }
                    let day = e.time_hours / HOURS_PER_DAY;
                    let color = match e.kind {
                        EventKind::DroughtStart | EventKind::SpringDry => {
                            Color32::from_rgb(220, 150, 80)
                        }
                        EventKind::FireStart | EventKind::FireEnd => {
                            Color32::from_rgb(240, 120, 60)
                        }
                        EventKind::ForestDieback | EventKind::LastTree => {
                            Color32::from_rgb(200, 160, 120)
                        }
                        EventKind::Germination | EventKind::TreesReturn => {
                            Color32::from_rgb(140, 220, 120)
                        }
                        EventKind::SeedFailed => Color32::from_rgb(150, 150, 150),
                        EventKind::GroundwaterLow => Color32::from_rgb(220, 150, 80),
                        EventKind::GroundwaterRecovered => Color32::from_rgb(120, 200, 120),
                        EventKind::HeavyRain => Color32::from_rgb(120, 170, 240),
                        EventKind::Intervention => Color32::from_rgb(230, 200, 90),
                        EventKind::DroughtEnd | EventKind::SpringFlowing => {
                            Color32::from_rgb(120, 200, 120)
                        }
                        _ => ui.visuals().text_color(),
                    };
                    let text = format!("J{} T{} · {}", day / 365 + 1, day % 365 + 1, e.message);
                    let mut r = ui.add(
                        egui::Label::new(RichText::new(text).color(color).size(12.5))
                            .sense(egui::Sense::click()),
                    );
                    if !e.causes.is_empty() {
                        r = r.on_hover_text(format!("Ursachen:\n• {}", e.causes.join("\n• ")));
                    }
                    if r.clicked()
                        && let Some((x, y)) = e.location
                    {
                        jump = Some(world.idx(x.min(world.width - 1), y.min(world.height - 1)));
                    }
                }
                if shown == 0 {
                    ui.label(RichText::new("Noch nichts geschehen.").weak());
                }
            });
        if let Some(i) = jump {
            self.selected = Some(i);
        }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.take_from_clock();
        let space = ui.input(|i| i.key_pressed(egui::Key::Space));
        let typing = ui.ctx().memory(|m| m.focused().is_some());
        if space && !typing {
            self.running = !self.running;
        }

        egui::Panel::top("top")
            .resizable(false)
            .exact_size(66.0)
            .show(ui, |ui| {
                ui.add_space(4.0);
                self.top_bar(ui);
            });
        egui::Panel::bottom("status")
            .resizable(false)
            .exact_size(26.0)
            .show(ui, |ui| self.status_bar(ui));
        egui::Panel::right("side")
            .resizable(false)
            .exact_size(400.0)
            .show(ui, |ui| self.side_panel(ui));
        let ctx = ui.ctx().clone();
        self.people_window(&ctx);
        egui::CentralPanel::no_frame().show(ui, |ui| {
            // Tag und Nacht nur bei 1 Std/s oder Pause – schneller würde es
            // flackern. Die Helligkeit gleitet stufenlos zwischen den Stunden.
            // (Ereignisse geschehen um Mitternacht – ein Anhalten danach soll
            // das Bild nicht verdunkeln, deshalb zählt nur die Stufe 1 Std/s.)
            let slow = self.speed == 0;
            let light = if slow {
                let hour = self.sim.hour_of_day() as f32 + self.owed_hours as f32;
                view::daylight(hour)
            } else {
                1.0
            };
            let (resp, result) = view::draw(
                ui,
                &self.sim,
                self.mode,
                self.layer,
                &mut self.camera,
                self.selected,
                light,
                self.speed <= 2,
                self.atlas
                    .as_ref()
                    .filter(|_| self.graphics)
                    .map(|a| view::Gfx {
                        atlas: a,
                        time_s: ui.input(|i| i.time),
                        animate: self.speed == 0 && self.running,
                        day_frac: (self.sim.hour_of_day() as f32 + self.owed_hours as f32) / 24.0,
                        hourly: self.speed == 0,
                    }),
            );
            if resp.clicked() {
                self.selected = result.hovered;
            }
            // Feld-Infos als fester Kasten unten links über der Karte
            if self.selected.is_some() {
                let pos = resp.rect.left_bottom() + egui::vec2(12.0, -12.0);
                egui::Area::new(egui::Id::unique("tile_info"))
                    .fixed_pos(pos)
                    .pivot(egui::Align2::LEFT_BOTTOM)
                    .order(egui::Order::Foreground)
                    .show(ui.ctx(), |ui| {
                        egui::Frame::popup(ui.style()).show(ui, |ui| {
                            ui.set_width(300.0);
                            ui.horizontal(|ui| {
                                ui.heading("Feld");
                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |ui| {
                                        if ui.button("✕").on_hover_text("Schließen").clicked() {
                                            self.selected = None;
                                        }
                                    },
                                );
                            });
                            self.tile_section(ui);
                            self.terrain_buttons(ui);
                        });
                    });
            }
        });

        if self.running {
            ui.ctx().request_repaint();
        }
        self.give_to_clock();
    }
}
