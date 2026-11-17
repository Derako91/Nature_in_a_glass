//! Seekartenoptik des Beobachters: feste Zeilen, ruhige Zahlen und klare Hierarchie.
use eframe::egui::{self, Color32, FontFamily, FontId, RichText, Stroke, TextStyle};

pub const INK: Color32 = Color32::from_rgb(22, 39, 47);
pub const PAPER: Color32 = Color32::from_rgb(232, 229, 214);
pub const BRASS: Color32 = Color32::from_rgb(214, 188, 132);
pub const MUTED: Color32 = Color32::from_rgb(167, 185, 185);
pub const KEY_W: f32 = 184.0;
pub const VALUE_W: f32 = 190.0;
pub const ROW_H: f32 = 16.0;

pub fn install(ctx: &egui::Context) {
    let mut style = (*ctx.global_style()).clone();
    style
        .text_styles
        .insert(TextStyle::Heading, chart_font(19.0));
    style
        .text_styles
        .insert(TextStyle::Body, FontId::proportional(13.0));
    style
        .text_styles
        .insert(TextStyle::Button, FontId::proportional(13.0));
    style
        .text_styles
        .insert(TextStyle::Small, FontId::proportional(11.5));
    style
        .text_styles
        .insert(TextStyle::Monospace, FontId::monospace(12.0));
    style.spacing.item_spacing = egui::vec2(6.0, 3.0);
    style.spacing.button_padding = egui::vec2(5.0, 2.0);
    style.spacing.window_margin = egui::Margin::same(10);
    let v = &mut style.visuals;
    v.panel_fill = INK;
    v.window_fill = Color32::from_rgb(26, 45, 53);
    v.extreme_bg_color = Color32::from_rgb(13, 28, 36);
    v.faint_bg_color = Color32::from_rgb(28, 47, 55);
    v.code_bg_color = v.extreme_bg_color;
    v.weak_text_color = Some(MUTED);
    v.hyperlink_color = BRASS;
    v.selection.bg_fill = Color32::from_rgb(54, 83, 89);
    v.selection.stroke = Stroke::new(1.0, PAPER);
    v.window_stroke = Stroke::new(1.0, Color32::from_rgb(100, 116, 111));
    v.window_corner_radius = egui::CornerRadius::same(5);
    for (widget, fill, edge) in [
        (
            &mut v.widgets.noninteractive,
            INK,
            Color32::from_rgb(58, 78, 83),
        ),
        (
            &mut v.widgets.inactive,
            Color32::from_rgb(36, 58, 66),
            Color32::from_rgb(73, 95, 100),
        ),
        (&mut v.widgets.hovered, Color32::from_rgb(48, 75, 82), BRASS),
        (&mut v.widgets.active, Color32::from_rgb(62, 86, 87), BRASS),
        (&mut v.widgets.open, Color32::from_rgb(44, 68, 75), BRASS),
    ] {
        widget.bg_fill = fill;
        widget.weak_bg_fill = fill;
        widget.bg_stroke = Stroke::new(1.0, edge);
        widget.fg_stroke = Stroke::new(1.0, PAPER);
        widget.corner_radius = egui::CornerRadius::same(3);
        widget.expansion = 0.0;
    }
    ctx.set_global_style(style);
}

pub fn chart_font(size: f32) -> FontId {
    FontId::new(size, FontFamily::Name("island_chart".into()))
}

fn line(ui: &mut egui::Ui, width: f32, height: f32, label: egui::Label) -> egui::Response {
    ui.allocate_ui_with_layout(
        egui::vec2(width, height),
        egui::Layout::left_to_right(egui::Align::Center),
        |ui| ui.add(label),
    )
    .inner
}

pub fn section(ui: &mut egui::Ui, title: &str) {
    line(
        ui,
        ui.available_width(),
        23.0,
        egui::Label::new(RichText::new(title).font(chart_font(19.0)).color(BRASS)),
    );
}

pub fn key(ui: &mut egui::Ui, text: &str) -> egui::Response {
    line(
        ui,
        KEY_W,
        ROW_H,
        egui::Label::new(RichText::new(text).color(MUTED)).truncate(),
    )
    .on_hover_text(text)
}

pub fn value(ui: &mut egui::Ui, text: &str) -> egui::Response {
    line(
        ui,
        VALUE_W,
        ROW_H,
        egui::Label::new(RichText::new(text).monospace().size(11.5)).truncate(),
    )
    .on_hover_text(text)
}

/// Metadaten behalten auch bei langen Familien-/Ortsangaben eine Zeile.
pub fn detail(ui: &mut egui::Ui, text: &str) {
    line(
        ui,
        ui.available_width(),
        18.0,
        egui::Label::new(RichText::new(text).monospace().size(12.0)).truncate(),
    )
    .on_hover_text(text);
}

/// Die Erkennungsfarbe bleibt im Punkt; Namen bleiben auch bei dunklen Farben lesbar.
pub fn resident_text(name: &str, age: &str, color: Color32, dead: bool) -> egui::text::LayoutJob {
    let mut job = egui::text::LayoutJob::default();
    let font = FontId::monospace(12.0);
    for (text, color) in [
        (
            if dead {
                "† ".to_string()
            } else {
                "● ".to_string()
            },
            if dead { MUTED } else { color },
        ),
        (format!("{name:<12}"), if dead { MUTED } else { PAPER }),
        (format!(" {age:>6}"), MUTED),
    ] {
        job.append(
            &text,
            0.0,
            egui::TextFormat {
                font_id: font.clone(),
                color,
                ..Default::default()
            },
        );
    }
    job
}
