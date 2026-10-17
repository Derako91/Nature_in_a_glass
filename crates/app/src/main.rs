//! Nature in a Glass – Desktop-Beobachter.

#![cfg_attr(windows, windows_subsystem = "windows")]

mod view;

use std::path::PathBuf;

use eframe::egui::{self, Color32, RichText};
use nig_core::sim::{HOURS_PER_DAY, STREAM_FLOW};
use nig_core::world::POND_MM;
use nig_core::{EventKind, Intervention, Simulation, Terrain};
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
    eframe::run_native(
        "Nature in a Glass",
        options,
        Box::new(move |_cc| {
            let mut app = App::new(seed);
            app.sim.advance_years(years);
            if view.as_deref() == Some("karte") {
                app.mode = ViewMode::Map;
            }
            if let Some(l) = layer {
                app.layer = match l.as_str() {
                    "hoehe" => Layer::Height,
                    "feuchte" => Layer::Moisture,
                    "wasser" => Layer::Water,
                    _ => Layer::Nature,
                };
            }
            app.sync_pending();
            Ok(Box::new(app))
        }),
    )
}

struct App {
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
    status: String,
    // Eingriffe: Werte während des Ziehens, angewendet beim Loslassen
    pending_rain: f32,
    pending_temp: f32,
    pending_springs: Vec<f32>,
}

impl App {
    fn new(seed: u64) -> Self {
        let sim = Self::fresh(seed);
        let mut app = Self {
            running: false,
            speed: 1,
            owed_hours: 0.0,
            mode: ViewMode::Iso,
            layer: Layer::Nature,
            camera: Camera::default(),
            selected: None,
            seed_text: seed.to_string(),
            show_year_summaries: false,
            status: String::new(),
            pending_rain: 0.0,
            pending_temp: 0.0,
            pending_springs: Vec::new(),
            sim,
        };
        app.sync_pending();
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

    fn advance(&mut self, dt: f64) {
        if !self.running {
            return;
        }
        self.owed_hours += SPEEDS[self.speed].1 * dt;
        self.owed_hours = self.owed_hours.min(MAX_HOURS_PER_FRAME);
        let whole = self.owed_hours.floor();
        if whole >= 1.0 {
            self.sim.advance_hours(whole as u64);
            self.owed_hours -= whole;
        }
    }

    fn season_name(&self) -> &'static str {
        match self.sim.day_of_year() {
            0..=58 | 334..=364 => "Winter",
            59..=151 => "Frühling",
            152..=243 => "Sommer",
            _ => "Herbst",
        }
    }

    fn time_text(&self) -> String {
        format!(
            "Jahr {} · Tag {} · {:02}:00 · {}",
            self.sim.year(),
            self.sim.day_of_year() + 1,
            self.sim.hour_of_day(),
            self.season_name()
        )
    }

    fn top_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_wrapped(|ui| {
            ui.label(RichText::new("Nature in a Glass").strong().size(16.0));
            ui.separator();
            ui.label(RichText::new(self.time_text()).monospace().size(15.0));
            ui.separator();
            let play = if self.running {
                "⏸ Pause"
            } else {
                "▶ Weiter"
            };
            if ui.button(play).on_hover_text("Leertaste").clicked() {
                self.running = !self.running;
            }
            for (k, (label, _)) in SPEEDS.iter().enumerate() {
                if ui.selectable_label(self.speed == k, *label).clicked() {
                    self.speed = k;
                    self.running = true;
                }
            }
            if ui.button("+1 Tag").clicked() {
                self.sim.advance_hours(HOURS_PER_DAY);
            }
            ui.separator();
            ui.selectable_value(&mut self.mode, ViewMode::Iso, "Isometrisch");
            ui.selectable_value(&mut self.mode, ViewMode::Map, "Karte");
            egui::ComboBox::from_id_salt("layer")
                .selected_text(self.layer.label())
                .show_ui(ui, |ui| {
                    for l in Layer::ALL {
                        ui.selectable_value(&mut self.layer, l, l.label());
                    }
                });
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
            if !self.status.is_empty() {
                ui.separator();
                ui.label(RichText::new(&self.status).weak());
            }
        });
    }

    fn side_panel(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            self.island_section(ui);
            ui.separator();
            self.tile_section(ui);
            ui.separator();
            self.intervention_section(ui);
            ui.separator();
            self.history_section(ui);
            ui.separator();
            self.journal_section(ui);
        });
    }

    fn island_section(&mut self, ui: &mut egui::Ui) {
        let snap = self.sim.snapshot();
        let wx = &self.sim.weather;
        ui.heading("Insel");
        egui::Grid::new("island")
            .num_columns(2)
            .striped(true)
            .show(ui, |ui| {
                let row = |ui: &mut egui::Ui, k: &str, v: String| {
                    ui.label(k);
                    ui.label(v);
                    ui.end_row();
                };
                let weather = if wx.storm {
                    format!("Starkregen, {:.0} mm", wx.rain_mm)
                } else if wx.rain_mm > 0.5 {
                    format!("Regen, {:.0} mm", wx.rain_mm)
                } else {
                    "trocken".into()
                };
                row(ui, "Wetter heute", weather);
                row(ui, "Temperatur (Küste)", format!("{:.1} °C", wx.temp_c));
                row(ui, "Wald", format!("{:.1} %", snap.forest_pct));
                row(ui, "Gras", format!("{:.1} %", snap.grass_pct));
                row(ui, "Bodenfeuchte", format!("{:.0} %", snap.moisture_pct));
                row(ui, "Bachfelder", snap.stream_tiles.to_string());
                row(ui, "Teich-/Seefelder", snap.pond_tiles.to_string());
                row(
                    ui,
                    "Grundwasser",
                    format!("{:.0} %", self.sim.aquifer / self.sim.aquifer_ref * 100.0),
                );
            });
    }

    fn tile_section(&mut self, ui: &mut egui::Ui) {
        ui.heading("Feld");
        let Some(i) = self.selected else {
            ui.label(RichText::new("Klicke auf die Karte, um ein Feld zu untersuchen.").weak());
            return;
        };
        let world = &self.sim.world;
        let t = &world.tiles[i];
        let (x, y) = world.xy(i);
        let terrain = match world.terrain(i) {
            Terrain::DeepOcean => "Tiefsee",
            Terrain::ShallowOcean => "Flachwasser",
            Terrain::Beach => "Strand",
            Terrain::Lowland => "Niederung",
            Terrain::Hills => "Hügelland",
            Terrain::Mountain => "Bergland",
        };
        let mut kind = terrain.to_string();
        if !t.is_ocean() {
            if t.surface_water > POND_MM {
                kind.push_str(", Teich/See");
            } else if t.flow > STREAM_FLOW {
                kind.push_str(", Bach");
            }
            if world.springs.iter().any(|s| s.x == x && s.y == y) {
                kind.push_str(", Quelle");
            }
        }
        egui::Grid::new("tile")
            .num_columns(2)
            .striped(true)
            .show(ui, |ui| {
                let row = |ui: &mut egui::Ui, k: &str, v: String| {
                    ui.label(k);
                    ui.label(v);
                    ui.end_row();
                };
                row(ui, "Position", format!("{x}, {y}"));
                row(ui, "Gelände", kind);
                row(ui, "Höhe", format!("{:.0} m", t.elevation));
                if !t.is_ocean() {
                    row(
                        ui,
                        "Temperatur",
                        format!("{:.1} °C", self.sim.weather.temp_at(t.elevation)),
                    );
                    row(
                        ui,
                        "Bodenfeuchte",
                        format!(
                            "{:.0} % von {:.0} mm",
                            t.moisture_fraction() * 100.0,
                            t.soil_capacity
                        ),
                    );
                    row(
                        ui,
                        "Oberflächenwasser",
                        format!("{:.0} mm", t.surface_water),
                    );
                    row(ui, "Durchfluss", format!("{:.0} mm/Tag", t.flow));
                    row(ui, "Gras", format!("{:.0} %", t.grass * 100.0));
                    row(ui, "Wald", format!("{:.0} %", t.forest * 100.0));
                    if t.drought_days > 0 {
                        row(ui, "Trockentage", t.drought_days.to_string());
                    }
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
            let label = format!(
                "Quelle {} ({}, {}) – {:.0} mm/Tag{}",
                k + 1,
                s.x,
                s.y,
                s.current,
                if s.dry { ", versiegt" } else { "" }
            );
            let mut pct = self.pending_springs[k] * 100.0;
            let r = ui.add(
                egui::Slider::new(&mut pct, 0.0..=300.0)
                    .text(label)
                    .suffix(" %"),
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

    fn history_section(&mut self, ui: &mut egui::Ui) {
        ui.heading("Verlauf");
        let hist = &self.sim.history;
        if hist.len() < 2 {
            ui.label(RichText::new("Nach zwei Jahren erscheint hier der Verlauf.").weak());
            return;
        }
        let width = ui.available_width();
        let (resp, painter) = ui.allocate_painter(egui::vec2(width, 110.0), egui::Sense::hover());
        let r = resp.rect.shrink2(egui::vec2(4.0, 6.0));
        painter.rect_filled(resp.rect, 4.0, ui.visuals().extreme_bg_color);
        let n = hist.len();
        let x_at = |k: usize| r.left() + r.width() * k as f32 / (n - 1) as f32;
        let y_at = |pct: f32| r.bottom() - r.height() * (pct / 100.0).clamp(0.0, 1.0);
        let series = [
            (Color32::from_rgb(60, 150, 70), "Wald"),
            (Color32::from_rgb(190, 180, 90), "Gras"),
            (Color32::from_rgb(80, 150, 230), "Grundwasser"),
        ];
        for (si, (col, _)) in series.iter().enumerate() {
            let pts: Vec<egui::Pos2> = hist
                .iter()
                .enumerate()
                .map(|(k, y)| {
                    let v = match si {
                        0 => y.forest_pct,
                        1 => y.grass_pct,
                        _ => y.aquifer_pct.min(100.0),
                    };
                    egui::pos2(x_at(k), y_at(v))
                })
                .collect();
            painter.add(egui::Shape::line(pts, egui::Stroke::new(1.5, *col)));
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
        for e in self.sim.journal.events.iter().rev() {
            if e.kind == EventKind::YearSummary && !self.show_year_summaries {
                continue;
            }
            shown += 1;
            if shown > 300 {
                break;
            }
            let day = e.time_hours / HOURS_PER_DAY;
            let color = match e.kind {
                EventKind::DroughtStart | EventKind::SpringDry => Color32::from_rgb(220, 150, 80),
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
                jump = Some(self.sim.world.idx(
                    x.min(self.sim.world.width - 1),
                    y.min(self.sim.world.height - 1),
                ));
            }
        }
        if shown == 0 {
            ui.label(RichText::new("Noch nichts geschehen.").weak());
        }
        if let Some(i) = jump {
            self.selected = Some(i);
        }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let (dt, space) = ui.input(|i| (i.stable_dt as f64, i.key_pressed(egui::Key::Space)));
        let typing = ui.ctx().memory(|m| m.focused().is_some());
        if space && !typing {
            self.running = !self.running;
        }
        self.advance(dt.min(0.25));

        egui::Panel::top("top").show(ui, |ui| {
            ui.add_space(4.0);
            self.top_bar(ui);
            ui.add_space(4.0);
        });
        egui::Panel::right("side")
            .resizable(true)
            .default_size(380.0)
            .min_size(300.0)
            .show(ui, |ui| self.side_panel(ui));
        egui::CentralPanel::no_frame().show(ui, |ui| {
            // Tag und Nacht nur bei langsamer Zeit, sonst würde das Bild flackern
            let light = if SPEEDS[self.speed].1 <= 24.0 || !self.running {
                view::daylight(self.sim.hour_of_day())
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
            );
            if resp.clicked() {
                self.selected = result.hovered;
            }
        });

        if self.running {
            ui.ctx().request_repaint();
        }
    }
}
