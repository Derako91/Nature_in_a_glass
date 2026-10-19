//! Bildatlas: Codex' Natur-Assets in einer einzigen Textur, damit die
//! isometrische Ansicht alles in einem Durchgang von hinten nach vorne zeichnen
//! kann. Technische Grundlage – wie die Bilder aussehen, gestaltet Codex
//! (docs/gestaltung.md, Sprite-Pakete in 16.6, ADR 0008).
//!
//! **Detailstufen:** Ein Bild darf in mehreren Fassungen vorliegen:
//! `name.png` (Hauptfassung, beliebig fein) und `name@<stufe>.png` (z. B.
//! `@fern` – eine vereinfachte Fernansicht). Aus sehr feinen Bildern (ab
//! [`AUTO_MIN_WIDTH`] px) berechnet der Atlas zusätzlich verkleinerte Stufen
//! (`~2`, `~4`, …). Beim Zeichnen wählt [`Atlas::pick`] die Stufe, die zum
//! Bildschirmmaßstab passt ([`choose_level`]). Alle Stufen werden auf dieselbe
//! Darstellungsbreite gebracht, Fußpunkte in Weltmaß stimmen überein – beim
//! Wechsel springt nichts.
//!
//! Vergrößert wird immer pixelgenau (`NEAREST`); bestehende Pixelgrafiken
//! werden nicht weichgezeichnet. Verkleinert wird über die vorberechneten
//! Stufen, sodass die Textur nie mehr als 1,5-fach verkleinert gezeichnet wird.

use std::collections::{BTreeMap, HashMap};
use std::path::Path;

use eframe::egui::{self, ColorImage, Pos2, Rect, TextureHandle, TextureOptions, Vec2};

mod embedded {
    include!(concat!(env!("OUT_DIR"), "/assets.rs"));
}

/// Ein Bild im Atlas.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Entry {
    /// Ausschnitt in der Textur (0..1).
    pub uv: Rect,
    /// Größe in Bildpunkten des Assets.
    pub size: Vec2,
    /// Fußpunkt bzw. Rautenmitte in Bildpunkten (von links oben).
    pub anchor: Vec2,
}

/// Name des weißen Hilfsfelds.
pub const WHITE: &str = "_white";
/// Ordner, deren Bilder Sprites sind (bekommen Detailstufen). Boden, Wände und
/// Wasser werden auf feste Rauten gelegt und bleiben, wie sie sind.
pub const SPRITE_GROUPS: [&str; 5] = ["vegetation", "animals", "people", "objects", "buildings"];
/// Ab dieser Breite berechnet der Atlas verkleinerte Stufen.
pub const AUTO_MIN_WIDTH: usize = 48;
/// Kleinste berechnete Stufe (Breite in px).
const AUTO_MIN_RESULT: usize = 6;
/// Stärkste Verkleinerung, mit der eine Stufe noch gezeichnet wird.
pub const MAX_DOWNSCALE: f32 = 1.5;

/// Ein geladenes Rohbild.
#[derive(Debug, Clone)]
pub struct RawImage {
    pub name: String,
    pub group: String,
    pub w: usize,
    pub h: usize,
    /// RGBA, nicht vormultipliziert.
    pub px: Vec<u8>,
}

/// Angaben aus den Manifesten.
#[derive(Debug, Clone, Default)]
pub struct ManifestInfo {
    pub anchors: HashMap<String, Vec2>,
    pub aliases: HashMap<String, String>,
    /// Weltmaß (Darstellungsgröße im Grundmaß) je Bild.
    pub display: HashMap<String, Vec2>,
}

/// Eine Detailstufe eines Bildes.
#[derive(Debug, Clone, PartialEq)]
pub struct Level {
    /// "" = Hauptfassung, "@fern" o. ä. = von Codex, "~2" = berechnet (Teiler).
    pub label: String,
    pub entry: Entry,
}

pub struct Atlas {
    pub texture: TextureHandle,
    data: AtlasData,
}

/// Der Inhalt des Atlas ohne Textur (ohne Grafikkarte prüfbar).
#[derive(Debug, Clone, Default)]
pub struct AtlasData {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<egui::Color32>,
    /// Hauptfassungen nach Name.
    entries: HashMap<String, Entry>,
    /// Alle Stufen je Name, nach Feinheit absteigend (breiteste zuerst).
    levels: HashMap<String, Vec<Level>>,
    aliases: HashMap<String, String>,
    display: HashMap<String, Vec2>,
}

/// Das Ergebnis einer Auswahl: welches Bild, in welcher Größe.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pick {
    pub entry: Entry,
    /// Bildschirmpunkte je Bildpunkt dieser Stufe.
    pub px: f32,
    /// Gewählte Stufe: 0 = feinste.
    pub level: usize,
}

impl Pick {
    /// Rechteck auf dem Bildschirm, wenn der Fußpunkt bei `foot` liegt.
    pub fn rect(&self, foot: Pos2) -> Rect {
        Rect::from_min_size(
            foot - self.entry.anchor * self.px,
            self.entry.size * self.px,
        )
    }
}

/// Wählt aus Stufen mit den Feinheiten `factors` (Bildpunkte je Weltpunkt,
/// absteigend) die passende für `screen` (Bildschirmpunkte je Weltpunkt):
/// die feinste, die höchstens [`MAX_DOWNSCALE`]-fach verkleinert würde;
/// gibt es keine, die gröbste (kleinste).
pub fn choose_level(factors: &[f32], screen: f32) -> usize {
    if factors.is_empty() {
        return 0;
    }
    let screen = screen.max(1e-3);
    for (k, &f) in factors.iter().enumerate() {
        if f / screen <= MAX_DOWNSCALE {
            return k;
        }
    }
    factors.len() - 1
}

/// Halbiert ein Bild (2 × 2 Bildpunkte → 1) mit vormultiplizierter Deckkraft.
/// Die Deckkraft wird leicht nachgeschärft, damit Umrisse klar bleiben und
/// keine halbdurchsichtigen Säume entstehen.
pub fn halve(w: usize, h: usize, px: &[u8]) -> (usize, usize, Vec<u8>) {
    let (w2, h2) = (w.div_ceil(2).max(1), h.div_ceil(2).max(1));
    let mut out = vec![0u8; w2 * h2 * 4];
    for y in 0..h2 {
        for x in 0..w2 {
            let (mut r, mut g, mut b, mut a, mut n) = (0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32);
            for dy in 0..2 {
                for dx in 0..2 {
                    let (sx, sy) = ((x * 2 + dx).min(w - 1), (y * 2 + dy).min(h - 1));
                    let k = (sy * w + sx) * 4;
                    let al = px[k + 3] as f32 / 255.0;
                    r += px[k] as f32 * al;
                    g += px[k + 1] as f32 * al;
                    b += px[k + 2] as f32 * al;
                    a += al;
                    n += 1.0;
                }
            }
            let k = (y * w2 + x) * 4;
            if a > 0.0 {
                out[k] = (r / a).round().clamp(0.0, 255.0) as u8;
                out[k + 1] = (g / a).round().clamp(0.0, 255.0) as u8;
                out[k + 2] = (b / a).round().clamp(0.0, 255.0) as u8;
            }
            let cover = a / n;
            // Nachschärfen: unter 25 % weg, über 75 % voll
            let sharp = ((cover - 0.25) * 2.0).clamp(0.0, 1.0);
            out[k + 3] = (sharp * 255.0).round() as u8;
        }
    }
    (w2, h2, out)
}

/// Dateiname ohne Endung → (Grundname, Stufe). `fox_0@fern` → (`fox_0`, `@fern`).
pub fn split_level(name: &str) -> (&str, &str) {
    match name.find('@') {
        Some(i) => (&name[..i], &name[i..]),
        None => (name, ""),
    }
}

fn decode(bytes: &[u8]) -> Option<(usize, usize, Vec<u8>)> {
    let mut dec = png::Decoder::new(std::io::Cursor::new(bytes));
    dec.set_transformations(png::Transformations::EXPAND | png::Transformations::ALPHA);
    let mut reader = dec.read_info().ok()?;
    let mut buf = vec![0; reader.output_buffer_size()?];
    let info = reader.next_frame(&mut buf).ok()?;
    let (w, h) = (info.width as usize, info.height as usize);
    let px = &buf[..info.buffer_size()];
    let rgba = match info.color_type {
        png::ColorType::Rgba => px.to_vec(),
        png::ColorType::GrayscaleAlpha => px
            .chunks(2)
            .flat_map(|c| [c[0], c[0], c[0], c[1]])
            .collect(),
        png::ColorType::Rgb => px.chunks(3).flat_map(|c| [c[0], c[1], c[2], 255]).collect(),
        png::ColorType::Grayscale => px.iter().flat_map(|&g| [g, g, g, 255]).collect(),
        _ => return None,
    };
    Some((w, h, rgba))
}

/// Fußpunkte, Doppelbilder und Weltmaße aus Manifest-Texten (JSON).
pub fn parse_manifests<'a>(sources: impl IntoIterator<Item = &'a str>) -> ManifestInfo {
    let mut info = ManifestInfo::default();
    let stem = |f: &str| {
        let name = f.rsplit('/').next().unwrap_or(f);
        name.strip_suffix(".png").unwrap_or(name).to_string()
    };
    for src in sources {
        let Ok(v) = serde_json::from_str::<serde_json::Value>(src) else {
            continue;
        };
        for a in v["assets"].as_array().into_iter().flatten() {
            let anchor = &a["anchor"];
            let anchor = match (anchor[0].as_f64(), anchor[1].as_f64()) {
                (Some(x), Some(y)) => Some(Vec2::new(x as f32, y as f32)),
                _ => None,
            };
            let ds = &a["display_size"];
            let display = match (ds[0].as_f64(), ds[1].as_f64()) {
                (Some(x), Some(y)) if x > 0.0 => Some(Vec2::new(x as f32, y as f32)),
                _ => None,
            };
            for f in a["files"].as_array().into_iter().flatten() {
                if let Some(f) = f.as_str() {
                    let s = stem(f);
                    if let Some(an) = anchor {
                        info.anchors.insert(s.clone(), an);
                    }
                    if let Some(d) = display {
                        info.display.insert(split_level(&s).0.to_string(), d);
                    }
                }
            }
            // Detailstufen im Manifest: "levels": [{"label": "@fern", "files": [...], "anchor": [...]}]
            for l in a["levels"].as_array().into_iter().flatten() {
                let la = &l["anchor"];
                let lan = match (la[0].as_f64(), la[1].as_f64()) {
                    (Some(x), Some(y)) => Some(Vec2::new(x as f32, y as f32)),
                    _ => None,
                };
                for f in l["files"].as_array().into_iter().flatten() {
                    if let (Some(f), Some(an)) = (f.as_str(), lan) {
                        info.anchors.insert(stem(f), an);
                    }
                }
            }
        }
        if let Some(map) = v["frame_aliases"].as_object() {
            for (k, target) in map {
                if let Some(t) = target.as_str() {
                    info.aliases.insert(stem(k), stem(t));
                }
            }
        }
    }
    info
}

impl AtlasData {
    /// Packt alle Bilder, berechnet Detailstufen und Fußpunkte.
    pub fn build(images: Vec<RawImage>, info: ManifestInfo) -> Self {
        // Gruppieren nach Grundname, um je Bild die Stufen zu kennen
        let mut by_base: BTreeMap<String, Vec<RawImage>> = BTreeMap::new();
        for img in images {
            by_base
                .entry(split_level(&img.name).0.to_string())
                .or_default()
                .push(img);
        }
        // (Atlas-Name, Grundname, Stufe, Teiler für berechnete Stufen, Bild)
        let mut items: Vec<(String, String, String, usize, RawImage)> = Vec::new();
        for (base, mut imgs) in by_base {
            imgs.sort_by(|a, b| b.w.cmp(&a.w).then(a.name.cmp(&b.name)));
            let smallest_explicit = imgs.iter().map(|i| i.w).min().unwrap_or(0);
            let sprite = imgs
                .iter()
                .any(|i| SPRITE_GROUPS.contains(&i.group.as_str()));
            // Berechnete Stufen aus der feinsten Fassung, aber nicht feiner
            // abgestuft als eine vorhandene Fernansicht.
            let mut autos = Vec::new();
            if sprite
                && let Some(top) = imgs.first()
                && top.w >= AUTO_MIN_WIDTH
            {
                let floor = if imgs.len() > 1 {
                    (smallest_explicit as f32 * 1.5).ceil() as usize
                } else {
                    AUTO_MIN_RESULT
                };
                let (mut w, mut h, mut px) = (top.w, top.h, top.px.clone());
                let mut div = 1;
                loop {
                    if w.div_ceil(2) < floor.max(AUTO_MIN_RESULT) {
                        break;
                    }
                    let (w2, h2, p2) = halve(w, h, &px);
                    div *= 2;
                    (w, h, px) = (w2, h2, p2);
                    autos.push((
                        div,
                        RawImage {
                            name: format!("{}~{div}", top.name),
                            group: top.group.clone(),
                            w,
                            h,
                            px: px.clone(),
                        },
                    ));
                }
            }

            for img in imgs {
                let label = split_level(&img.name).1.to_string();
                items.push((img.name.clone(), base.clone(), label, 1, img));
            }
            for (div, img) in autos {
                items.push((img.name.clone(), base.clone(), format!("~{div}"), div, img));
            }
        }
        items.push((
            WHITE.to_string(),
            WHITE.to_string(),
            String::new(),
            1,
            RawImage {
                name: WHITE.into(),
                group: String::new(),
                w: 4,
                h: 4,
                px: vec![255; 64],
            },
        ));

        // Regalpacken: nach Höhe sortiert in Reihen, 1 Pixel Abstand
        items.sort_by(|a, b| b.4.h.cmp(&a.4.h).then(a.0.cmp(&b.0)));
        // Breit genug für die feinen Nahansicht-Bilder (ADR 0006)
        const W: usize = 2048;
        let (mut x, mut y, mut row_h) = (1usize, 1usize, 0usize);
        let mut places = Vec::with_capacity(items.len());
        for it in &items {
            let (w, h) = (it.4.w.min(W - 2), it.4.h);
            if x + w + 1 > W {
                x = 1;
                y += row_h + 1;
                row_h = 0;
            }
            places.push((x, y));
            x += w + 1;
            row_h = row_h.max(h);
        }
        let height = (y + row_h + 1).next_power_of_two();
        let mut pixels = vec![egui::Color32::TRANSPARENT; W * height];
        let mut entries = HashMap::new();
        let mut levels: HashMap<String, Vec<Level>> = HashMap::new();
        // Fußpunkte der Hauptfassungen, um Stufen ohne eigenen Anker abzuleiten
        let mut primary_size: HashMap<String, Vec2> = HashMap::new();
        for it in &items {
            if it.2.is_empty() {
                primary_size.insert(it.1.clone(), Vec2::new(it.4.w as f32, it.4.h as f32));
            }
        }
        let default_anchor = |base: &str, size: Vec2| -> Vec2 {
            // Anker der Hauptfassung, auf diese Größe umgerechnet – so bleibt
            // der Fußpunkt in Weltmaß gleich.
            match (info.anchors.get(base), primary_size.get(base)) {
                (Some(&a), Some(&ps)) if ps.x > 0.0 => a * (size.x / ps.x),
                _ => Vec2::new(size.x * 0.5, size.y - 1.0),
            }
        };
        for ((name, base, label, div, img), (ox, oy)) in items.into_iter().zip(places) {
            let w = img.w.min(W - 2);
            for row in 0..img.h {
                for col in 0..w {
                    let k = (row * img.w + col) * 4;
                    pixels[(oy + row) * W + ox + col] = egui::Color32::from_rgba_unmultiplied(
                        img.px[k],
                        img.px[k + 1],
                        img.px[k + 2],
                        img.px[k + 3],
                    );
                }
            }
            let uv = Rect::from_min_max(
                Pos2::new(ox as f32 / W as f32, oy as f32 / height as f32),
                Pos2::new(
                    (ox + w) as f32 / W as f32,
                    (oy + img.h) as f32 / height as f32,
                ),
            );
            let size = Vec2::new(w as f32, img.h as f32);
            let anchor = if div > 1 {
                // Berechnete Stufe: Anker der Quelle geteilt
                let src = name.split('~').next().unwrap_or(&base);
                info.anchors
                    .get(src)
                    .map(|&a| a / div as f32)
                    .unwrap_or_else(|| default_anchor(&base, size))
            } else {
                info.anchors
                    .get(&name)
                    .copied()
                    .unwrap_or_else(|| default_anchor(&base, size))
            };
            let entry = Entry { uv, size, anchor };
            if label.is_empty() {
                entries.insert(base.clone(), entry);
            }
            levels.entry(base).or_default().push(Level { label, entry });
        }
        for l in levels.values_mut() {
            l.sort_by(|a, b| {
                b.entry
                    .size
                    .x
                    .total_cmp(&a.entry.size.x)
                    .then(a.label.cmp(&b.label))
            });
        }
        // Grundnamen, die nur als Stufe vorliegen (z. B. nur `@fern`): die
        // feinste Stufe gilt als Hauptfassung.
        for (base, l) in &levels {
            if !entries.contains_key(base)
                && let Some(first) = l.first()
            {
                entries.insert(base.clone(), first.entry);
            }
        }
        AtlasData {
            width: W,
            height,
            pixels,
            entries,
            levels,
            aliases: info.aliases,
            display: info.display,
        }
    }

    fn resolve<'a>(&'a self, name: &'a str) -> Option<&'a str> {
        if self.entries.contains_key(name) {
            Some(name)
        } else {
            self.aliases
                .get(name)
                .map(|s| s.as_str())
                .filter(|t| self.entries.contains_key(*t))
        }
    }

    /// Hauptfassung nach Name, Doppelbilder über die Verweise.
    pub fn get(&self, name: &str) -> Option<&Entry> {
        self.resolve(name).and_then(|n| self.entries.get(n))
    }

    /// Weltmaß aus dem Manifest (`display_size`), falls angegeben.
    pub fn display_size(&self, name: &str) -> Option<Vec2> {
        let n = self.resolve(name).unwrap_or(name);
        self.display.get(n).copied()
    }

    pub fn levels(&self, name: &str) -> &[Level] {
        self.resolve(name)
            .and_then(|n| self.levels.get(n))
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// Die passende Stufe von `name` für den Bildschirmmaßstab `screen`
    /// (Bildschirmpunkte je Weltpunkt). `reference` bestimmt die Feinheit
    /// jeder Stufe (bei Tieren das erste Fressbild, damit alle Bilder einer
    /// Art gleich groß werden), `display_w` ist die Darstellungsbreite in
    /// Weltpunkten. Fehlt `name`, gibt es `None` – der Aufrufer zeichnet
    /// dann eine Ersatzdarstellung.
    pub fn pick(&self, name: &str, reference: &str, display_w: f32, screen: f32) -> Option<Pick> {
        self.pick_sized(name, reference, display_w, display_w, screen)
    }

    /// Wie [`pick`](Self::pick), aber mit getrenntem Weltmaß: `ref_w` gilt,
    /// wenn das Bild am Referenzbild gemessen wird, `own_w`, wenn es aus
    /// einem anderen Satz stammt und für sich selbst bemessen wird. So erbt
    /// ein altes randloses Bild nicht das vergrößerte Weltmaß eines neuen
    /// Bildes mit transparentem Rand (gestaltung.md 16.7).
    pub fn pick_sized(
        &self,
        name: &str,
        reference: &str,
        ref_w: f32,
        own_w: f32,
        screen: f32,
    ) -> Option<Pick> {
        let own = self.levels(name);
        if own.is_empty() {
            return None;
        }
        let main_w = |v: &[Level]| {
            v.iter()
                .find(|l| l.label.is_empty())
                .unwrap_or(&v[0])
                .entry
                .size
                .x
                .max(1.0)
        };
        // Die Referenz gilt nur für Bilder derselben Auflösung. Ein Bild aus
        // einem anderen Satz (z. B. noch nicht ersetzte alte Bilder neben
        // neuen feinen) wird für sich selbst bemessen.
        let refs = {
            let r = self.levels(reference);
            let same_set = !r.is_empty() && {
                let q = main_w(own) / main_w(r);
                (1.0 / MAX_DOWNSCALE..=MAX_DOWNSCALE).contains(&q)
            };
            if same_set { (r, ref_w) } else { (own, own_w) }
        };
        let (refs, display_w) = refs;
        let display_w = display_w.max(0.01);
        let factors: Vec<f32> = refs.iter().map(|l| l.entry.size.x / display_w).collect();
        let k = choose_level(&factors, screen);
        let label = &refs[k].label;
        // Dieselbe Stufe des gesuchten Bildes; fehlt sie, wird unter den
        // eigenen Stufen gewählt (umgerechnet über die Hauptfassungen).
        let (lvl, ref_w) = match own.iter().position(|l| &l.label == label) {
            Some(i) => (i, refs[k].entry.size.x),
            None => {
                let conv = main_w(refs) / main_w(own);
                let own_f: Vec<f32> = own
                    .iter()
                    .map(|l| l.entry.size.x * conv / display_w)
                    .collect();
                let i = choose_level(&own_f, screen);
                (i, own[i].entry.size.x * conv)
            }
        };
        let entry = own[lvl].entry;
        Some(Pick {
            entry,
            px: screen * display_w / ref_w.max(1.0),
            level: lvl,
        })
    }

    /// Boden, Wand oder Wasser: `<id>_<variante>`, notfalls Variante 0.
    pub fn tile(&self, id: &str, variant: u32) -> Option<&Entry> {
        self.get(&format!("{id}_{variant}"))
            .or_else(|| self.get(&format!("{id}_0")))
    }

    /// Erstes vorhandenes Bild aus einer Liste von Namen.
    pub fn first_of(&self, names: &[String]) -> Option<String> {
        names
            .iter()
            .find_map(|n| self.resolve(n).map(|s| s.to_string()))
    }

    /// Pflanzen: `<id>_<variante>_<zustand>`, notfalls `normal` bzw. Variante 0.
    pub fn sprite_name(&self, id: &str, variant: u32, state: &str) -> Option<String> {
        self.first_of(&[
            format!("{id}_{variant}_{state}"),
            format!("{id}_{variant}_normal"),
            format!("{id}_0_normal"),
        ])
    }

    /// Tiere und Menschen: `<id>_<variante>_<zustand>_<richtung>_f<bild>`.
    pub fn animal_name(
        &self,
        id: &str,
        variant: u8,
        state: &str,
        dir: &str,
        frame: u32,
    ) -> Option<String> {
        self.first_of(&[
            format!("{id}_{variant}_{state}_{dir}_f{frame}"),
            format!("{id}_{variant}_{state}_{dir}_f0"),
            format!("{id}_0_graze_{dir}_f0"),
        ])
    }

    /// Mitte des weißen Felds – UV für einfarbige Dreiecke.
    pub fn white_uv(&self) -> Pos2 {
        self.entries
            .get(WHITE)
            .map(|e| e.uv.center())
            .unwrap_or(Pos2::ZERO)
    }
}

/// Liest alle PNG und Manifeste aus einem Ordner (Unterordner = Gruppe).
fn load_dir(dir: &Path) -> (Vec<RawImage>, Vec<String>) {
    let mut images = Vec::new();
    let mut manifests = Vec::new();
    let mut stack = vec![(dir.to_path_buf(), String::new())];
    while let Some((d, group)) = stack.pop() {
        let Ok(rd) = std::fs::read_dir(&d) else {
            continue;
        };
        let mut paths: Vec<_> = rd.filter_map(|e| e.ok()).map(|e| e.path()).collect();
        paths.sort();
        for p in paths {
            let fname = p.file_name().map(|f| f.to_string_lossy().to_string());
            if p.is_dir() {
                let g = fname.unwrap_or_default();
                stack.push((p, if group.is_empty() { g } else { group.clone() }));
            } else if fname.as_deref() == Some("manifest.json") {
                if let Ok(s) = std::fs::read_to_string(&p) {
                    manifests.push(s);
                }
            } else if p.extension().is_some_and(|x| x == "png")
                && !fname.as_deref().unwrap_or("_").starts_with('_')
                && let Ok(bytes) = std::fs::read(&p)
                && let Some((w, h, px)) = decode(&bytes)
            {
                let name = p.file_stem().unwrap().to_string_lossy().to_string();
                images.push(RawImage {
                    name,
                    group: group.clone(),
                    w,
                    h,
                    px,
                });
            }
        }
    }
    (images, manifests)
}

impl Atlas {
    /// Lädt alle eingebetteten Bilder, dazu (falls angegeben) Bilder aus einem
    /// Ordner – diese ersetzen gleichnamige eingebettete. `None`, wenn es gar
    /// keine Bilder gibt.
    pub fn load(ctx: &egui::Context, extra: Option<&Path>) -> Option<Self> {
        let mut images: Vec<RawImage> = embedded::IMAGES
            .iter()
            .filter_map(|(name, group, bytes)| {
                decode(bytes).map(|(w, h, px)| RawImage {
                    name: name.to_string(),
                    group: group.to_string(),
                    w,
                    h,
                    px,
                })
            })
            .collect();
        let mut manifests: Vec<String> = [
            embedded::MANIFEST,
            embedded::ANIMAL_MANIFEST,
            embedded::PEOPLE_MANIFEST,
        ]
        .into_iter()
        .flatten()
        .map(|s| s.to_string())
        .collect();
        if let Some(dir) = extra {
            let (imgs, mans) = load_dir(dir);
            // Gleichnamige Bilder werden ersetzt; neue Stufen kommen dazu.
            for img in imgs {
                images.retain(|i| i.name != img.name);
                images.push(img);
            }
            manifests.extend(mans);
        }
        if images.is_empty() {
            return None;
        }
        let info = parse_manifests(manifests.iter().map(|s| s.as_str()));
        let data = AtlasData::build(images, info);
        let image = ColorImage {
            size: [data.width, data.height],
            source_size: Vec2::new(data.width as f32, data.height as f32),
            pixels: data.pixels.clone(),
        };
        let texture = ctx.load_texture("natur-atlas", image, TextureOptions::NEAREST);
        Some(Self { texture, data })
    }
}

impl std::ops::Deref for Atlas {
    type Target = AtlasData;
    fn deref(&self) -> &AtlasData {
        &self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn img(name: &str, group: &str, w: usize, h: usize, rgba: [u8; 4]) -> RawImage {
        RawImage {
            name: name.into(),
            group: group.into(),
            w,
            h,
            px: rgba.repeat(w * h),
        }
    }

    /// Fuchs-Fall: fein (80 px, 8-fach) und Fernansicht (20 px, 2-fach) bei
    /// Darstellungsbreite 10. Zoom 4/8 → Fernansicht, Zoom 16/32 → fein
    /// (bei 16 die berechnete Halbierung).
    #[test]
    fn fox_levels_follow_zoom() {
        let mut info = ManifestInfo::default();
        info.anchors
            .insert("fox_0_graze_r_f0".into(), Vec2::new(40.0, 40.0));
        info.anchors
            .insert("fox_0_graze_r_f0@fern".into(), Vec2::new(10.0, 10.0));
        let d = AtlasData::build(
            vec![
                img("fox_0_graze_r_f0", "animals", 80, 48, [200, 100, 40, 255]),
                img(
                    "fox_0_graze_r_f0@fern",
                    "animals",
                    20,
                    12,
                    [200, 100, 40, 255],
                ),
            ],
            info,
        );
        let labels: Vec<&str> = d
            .levels("fox_0_graze_r_f0")
            .iter()
            .map(|l| l.label.as_str())
            .collect();
        // fein, berechnete Halbierung (40 px), Fernansicht – keine Halbierung
        // unter die Fernansicht
        assert_eq!(labels, vec!["", "~2", "@fern"]);
        let n = "fox_0_graze_r_f0";
        let at = |zoom: f32| d.pick(n, n, 10.0, zoom / 4.0).unwrap();
        let label = |p: Pick| d.levels(n)[p.level].label.clone();
        assert_eq!(label(at(4.0)), "@fern");
        assert_eq!(label(at(8.0)), "@fern");
        assert_eq!(label(at(16.0)), "~2");
        assert_eq!(label(at(32.0)), "");
        // Gleiche Größe und gleicher Fußpunkt auf dem Bildschirm, gleich
        // welche Stufe gezeichnet wird
        let foot = Pos2::new(100.0, 100.0);
        for zoom in [4.0, 8.0, 10.6, 10.8, 16.0, 21.0, 22.0, 32.0] {
            let p = at(zoom);
            let r = p.rect(foot);
            let s = zoom / 4.0;
            assert!(
                (r.width() - 10.0 * s).abs() < 0.01,
                "Breite bei Zoom {zoom}"
            );
            let world_anchor = p.entry.anchor * (p.px / s);
            assert!(
                (world_anchor - Vec2::new(5.0, 5.0)).length() < 0.01,
                "Anker bei Zoom {zoom}"
            );
        }
    }

    /// Nur ein feines Bild ohne Fernansicht: Der Atlas berechnet Stufen bis
    /// zur Darstellungsgröße und wählt sie selbst.
    #[test]
    fn detailed_sprite_alone_gets_automatic_far_levels() {
        let d = AtlasData::build(
            vec![img(
                "person_0_idle_r_f0",
                "people",
                56,
                120,
                [90, 80, 60, 255],
            )],
            ManifestInfo::default(),
        );
        let widths: Vec<f32> = d
            .levels("person_0_idle_r_f0")
            .iter()
            .map(|l| l.entry.size.x)
            .collect();
        assert_eq!(widths, vec![56.0, 28.0, 14.0, 7.0]);
        let n = "person_0_idle_r_f0";
        // Zoom 4: Darstellungsbreite 7 → 7-px-Stufe ohne Verkleinerung
        assert_eq!(d.pick(n, n, 7.0, 1.0).unwrap().entry.size.x, 7.0);
        assert_eq!(d.pick(n, n, 7.0, 8.0).unwrap().entry.size.x, 56.0);
    }

    /// Bestehende Pixelgrafiken (kleiner als 48 px, Boden) bleiben unverändert:
    /// keine berechneten Stufen, keine Weichzeichnung.
    #[test]
    fn existing_pixel_art_is_left_alone() {
        let d = AtlasData::build(
            vec![
                img("deer_0_graze_r_f0", "animals", 28, 24, [150, 100, 60, 255]),
                img("ground_grass_0", "ground", 128, 64, [80, 140, 60, 255]),
            ],
            ManifestInfo::default(),
        );
        assert_eq!(d.levels("deer_0_graze_r_f0").len(), 1);
        assert_eq!(d.levels("ground_grass_0").len(), 1);
        let p = d
            .pick("deer_0_graze_r_f0", "deer_0_graze_r_f0", 14.0, 0.5)
            .unwrap();
        assert_eq!(p.entry.size.x, 28.0);
    }

    #[test]
    fn missing_images_and_levels_do_not_panic() {
        let d = AtlasData::build(
            vec![
                img("fox_0_graze_r_f0", "animals", 80, 48, [200, 100, 40, 255]),
                img(
                    "fox_0_graze_r_f0@fern",
                    "animals",
                    20,
                    12,
                    [200, 100, 40, 255],
                ),
                // Laufbild ohne Fernansicht
                img("fox_0_walk_r_f1", "animals", 80, 48, [200, 100, 40, 255]),
            ],
            ManifestInfo::default(),
        );
        assert!(d.pick("gibt_es_nicht", "auch_nicht", 10.0, 1.0).is_none());
        // Fehlende Fernansicht: eigene berechnete Stufe, in richtiger Größe
        let p = d
            .pick("fox_0_walk_r_f1", "fox_0_graze_r_f0", 10.0, 1.0)
            .unwrap();
        assert!((p.entry.size.x * p.px - 10.0).abs() < 0.01);
        assert!(p.entry.size.x <= 20.0, "nicht die 80-px-Fassung");
        // Fehlende Referenz: eigene Stufen
        assert!(d.pick("fox_0_walk_r_f1", "fehlt", 10.0, 2.0).is_some());
        // Leere Liste
        assert_eq!(choose_level(&[], 1.0), 0);
    }

    #[test]
    fn halving_keeps_colors_and_silhouette() {
        // Linke Hälfte deckend rot, rechte durchsichtig
        let (w, h) = (4, 2);
        let mut px = vec![0u8; w * h * 4];
        for y in 0..h {
            for x in 0..2 {
                let k = (y * w + x) * 4;
                px[k..k + 4].copy_from_slice(&[200, 40, 20, 255]);
            }
        }
        let (w2, h2, out) = halve(w, h, &px);
        assert_eq!((w2, h2), (2, 1));
        assert_eq!(&out[0..4], &[200, 40, 20, 255]);
        assert_eq!(out[7], 0, "durchsichtig bleibt durchsichtig");
        // Ein Bildpunkt halb bedeckt: Farbe bleibt rein (kein Grauschleier)
        let mut px = vec![0u8; 2 * 2 * 4];
        px[0..4].copy_from_slice(&[200, 40, 20, 255]);
        px[4..8].copy_from_slice(&[200, 40, 20, 255]);
        let (_, _, out) = halve(2, 2, &px);
        assert_eq!(&out[0..3], &[200, 40, 20]);
        assert!(out[3] > 100 && out[3] < 255);
    }

    #[test]
    fn manifest_levels_and_display_size_are_read() {
        let m = r#"{"assets":[{"id":"fox","files":["fox_0_graze_r_f0.png"],
            "anchor":[40,40],"display_size":[10,6],
            "levels":[{"label":"@fern","files":["fox_0_graze_r_f0@fern.png"],"anchor":[10,10]}]}]}"#;
        let info = parse_manifests([m]);
        assert_eq!(info.anchors["fox_0_graze_r_f0@fern"], Vec2::new(10.0, 10.0));
        assert_eq!(info.display["fox_0_graze_r_f0"], Vec2::new(10.0, 6.0));
        assert_eq!(split_level("a_b@fern"), ("a_b", "@fern"));
    }

    /// Ohne eigenen Anker übernimmt eine Stufe den Anker der Hauptfassung,
    /// umgerechnet – der Fußpunkt bleibt in Weltmaß gleich.
    #[test]
    fn level_without_anchor_inherits_primary_anchor() {
        let mut info = ManifestInfo::default();
        info.anchors
            .insert("tree_old_0_normal".into(), Vec2::new(36.0, 58.0));
        let d = AtlasData::build(
            vec![
                img(
                    "tree_old_0_normal",
                    "vegetation",
                    72,
                    120,
                    [60, 120, 50, 255],
                ),
                img(
                    "tree_old_0_normal@fern",
                    "vegetation",
                    18,
                    30,
                    [60, 120, 50, 255],
                ),
            ],
            info,
        );
        let fern = d
            .levels("tree_old_0_normal")
            .iter()
            .find(|l| l.label == "@fern")
            .unwrap();
        assert_eq!(fern.entry.anchor, Vec2::new(9.0, 14.5));
    }

    /// Alle eingebauten Bilder: Der Atlas passt in eine Textur, die jede
    /// Grafikkarte kann, und jedes Tier und jeder Mensch erscheint bei jedem
    /// Zoom in seiner Darstellungsbreite.
    #[test]
    fn embedded_assets_build_and_resolve_at_every_zoom() {
        let images: Vec<RawImage> = embedded::IMAGES
            .iter()
            .filter_map(|(name, group, bytes)| {
                decode(bytes).map(|(w, h, px)| RawImage {
                    name: name.to_string(),
                    group: group.to_string(),
                    w,
                    h,
                    px,
                })
            })
            .collect();
        if images.is_empty() {
            return;
        }
        let info = parse_manifests(
            [
                embedded::MANIFEST,
                embedded::ANIMAL_MANIFEST,
                embedded::PEOPLE_MANIFEST,
            ]
            .into_iter()
            .flatten(),
        );
        let d = AtlasData::build(images, info);
        assert!(d.height <= 8192, "Atlas {} px hoch", d.height);
        for (id, w) in [
            ("deer", 14.0),
            ("wolf", 13.0),
            ("fox", 10.0),
            ("rabbit", 6.0),
            ("person", 7.0),
        ] {
            for dir in ["l", "r"] {
                let state = if id == "person" { "idle" } else { "graze" };
                let Some(n) = d.first_of(&[format!("{id}_0_{state}_{dir}_f0")]) else {
                    continue;
                };
                for zoom in [1.0f32, 2.0, 4.0, 8.0, 16.0, 32.0] {
                    let s = zoom / 4.0;
                    let p = d.pick(&n, &n, w, s).expect("Bild vorhanden");
                    let shown = p.entry.size.x * p.px;
                    assert!((shown - w * s).abs() < 0.01, "{n} bei Zoom {zoom}: {shown}");
                    assert!(
                        p.px >= 1.0 / MAX_DOWNSCALE - 1e-3 || p.level + 1 == d.levels(&n).len()
                    );
                }
            }
        }
    }

    /// Teilweise ersetzt: Nur das Fressbild ist neu und fein (80 px), die
    /// übrigen Bilder der Art sind noch die alten (20 px). Beide müssen in
    /// ihrer Darstellungsbreite erscheinen – die alten nicht viermal zu klein.
    #[test]
    fn partially_replaced_species_keeps_every_frame_at_world_size() {
        let d = AtlasData::build(
            vec![
                img("fox_0_graze_r_f0", "animals", 80, 48, [200, 100, 40, 255]),
                img(
                    "fox_0_graze_r_f0@fern",
                    "animals",
                    20,
                    12,
                    [200, 100, 40, 255],
                ),
                img("fox_0_rest_r_f0", "animals", 20, 12, [200, 100, 40, 255]),
                // leicht breiteres Laufbild derselben (feinen) Auflösung
                img("fox_0_walk_r_f1", "animals", 88, 48, [200, 100, 40, 255]),
            ],
            ManifestInfo::default(),
        );
        for zoom in [4.0f32, 8.0, 16.0, 32.0] {
            let s = zoom / 4.0;
            let rest = d
                .pick("fox_0_rest_r_f0", "fox_0_graze_r_f0", 10.0, s)
                .unwrap();
            let shown = rest.entry.size.x * rest.px;
            assert!(
                (shown - 10.0 * s).abs() < 0.01,
                "altes Bild bei Zoom {zoom}: {shown}"
            );
            // Gleiche Auflösung wie die Referenz: am Fressbild gemessen
            let walk = d
                .pick("fox_0_walk_r_f1", "fox_0_graze_r_f0", 10.0, s)
                .unwrap();
            let w = walk.entry.size.x * walk.px;
            assert!((w - 11.0 * s).abs() < 0.01, "Laufbild bei Zoom {zoom}: {w}");
        }
    }

    /// Neues Fressbild mit transparentem Rand: Sein Weltmaß ist vergrößert
    /// (Motiv 34 von 80 px → Leinwand 23,53). Alte randlose Bilder derselben
    /// Art dürfen das nicht erben, neue Bilder des Satzes schon.
    #[test]
    fn padded_reference_does_not_enlarge_old_frames() {
        let d = AtlasData::build(
            vec![
                img("fox_0_graze_r_f0", "animals", 80, 48, [200, 100, 40, 255]),
                img("fox_0_walk_r_f0", "animals", 80, 48, [200, 100, 40, 255]),
                img("fox_0_rest_r_f0", "animals", 20, 12, [200, 100, 40, 255]),
            ],
            ManifestInfo::default(),
        );
        let padded = 10.0 * 80.0 / 34.0;
        for zoom in [4.0f32, 8.0, 16.0, 32.0] {
            let s = zoom / 4.0;
            let shown = |n: &str| {
                let p = d
                    .pick_sized(n, "fox_0_graze_r_f0", padded, 10.0, s)
                    .unwrap();
                p.entry.size.x * p.px
            };
            let old = shown("fox_0_rest_r_f0");
            assert!(
                (old - 10.0 * s).abs() < 0.01,
                "altes Bild bei Zoom {zoom}: {old}"
            );
            let new = shown("fox_0_walk_r_f0");
            assert!(
                (new - padded * s).abs() < 0.01,
                "neues Bild bei Zoom {zoom}: {new}"
            );
        }
    }
}
