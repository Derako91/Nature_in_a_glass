//! Bildatlas: Codex' Natur-Assets in einer einzigen Textur, damit die
//! isometrische Ansicht alles in einem Durchgang von hinten nach vorne zeichnen
//! kann. Technische Grundlage – wie die Bilder zusammengesetzt werden, gestaltet
//! Codex (docs/gestaltung.md).

use std::collections::HashMap;

use eframe::egui::{self, ColorImage, Pos2, Rect, TextureHandle, TextureOptions, Vec2};

mod embedded {
    include!(concat!(env!("OUT_DIR"), "/assets.rs"));
}

/// Ein Bild im Atlas.
#[derive(Debug, Clone, Copy)]
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

pub struct Atlas {
    pub texture: TextureHandle,
    entries: HashMap<String, Entry>,
    aliases: HashMap<String, String>,
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

/// Fußpunkte aus den Manifesten: Dateiname ohne Endung → Anker.
fn anchors() -> (HashMap<String, Vec2>, HashMap<String, String>) {
    let mut out = HashMap::new();
    let mut aliases = HashMap::new();
    let stem = |f: &str| {
        let name = f.rsplit('/').next().unwrap_or(f);
        name.strip_suffix(".png").unwrap_or(name).to_string()
    };
    for src in [embedded::MANIFEST, embedded::ANIMAL_MANIFEST]
        .into_iter()
        .flatten()
    {
        let Ok(v) = serde_json::from_str::<serde_json::Value>(src) else {
            continue;
        };
        for a in v["assets"].as_array().into_iter().flatten() {
            let anchor = &a["anchor"];
            let (Some(x), Some(y)) = (anchor[0].as_f64(), anchor[1].as_f64()) else {
                continue;
            };
            for f in a["files"].as_array().into_iter().flatten() {
                if let Some(f) = f.as_str() {
                    out.insert(stem(f), Vec2::new(x as f32, y as f32));
                }
            }
        }
        if let Some(map) = v["frame_aliases"].as_object() {
            for (k, target) in map {
                if let Some(t) = target.as_str() {
                    aliases.insert(stem(k), stem(t));
                }
            }
        }
    }
    (out, aliases)
}

impl Atlas {
    /// Lädt alle eingebetteten Bilder. `None`, wenn keine vorhanden sind.
    pub fn load(ctx: &egui::Context) -> Option<Self> {
        let (anchor_of, aliases) = anchors();
        let mut imgs: Vec<(&str, usize, usize, Vec<u8>)> = embedded::IMAGES
            .iter()
            .filter_map(|(name, bytes)| decode(bytes).map(|(w, h, px)| (*name, w, h, px)))
            .collect();
        if imgs.is_empty() {
            return None;
        }
        // Weißes Feld für einfarbige Flächen (Bachlinien, Feuer) im selben Atlas
        imgs.push((WHITE, 4, 4, vec![255; 4 * 4 * 4]));
        // Regalpacken: nach Höhe sortiert in Reihen, 1 Pixel Abstand
        imgs.sort_by(|a, b| b.2.cmp(&a.2).then(a.0.cmp(b.0)));
        // Breit genug für die feinen Nahansicht-Bilder (ADR 0006)
        const W: usize = 2048;
        let (mut x, mut y, mut row_h) = (1usize, 1usize, 0usize);
        let mut places = Vec::with_capacity(imgs.len());
        for (_, w, h, _) in &imgs {
            if x + w + 1 > W {
                x = 1;
                y += row_h + 1;
                row_h = 0;
            }
            places.push((x, y));
            x += w + 1;
            row_h = row_h.max(*h);
        }
        let height = (y + row_h + 1).next_power_of_two();
        let mut pixels = vec![egui::Color32::TRANSPARENT; W * height];
        let mut entries = HashMap::new();
        for ((name, w, h, px), (ox, oy)) in imgs.iter().zip(places) {
            for row in 0..*h {
                for col in 0..*w {
                    let k = (row * w + col) * 4;
                    pixels[(oy + row) * W + ox + col] = egui::Color32::from_rgba_unmultiplied(
                        px[k],
                        px[k + 1],
                        px[k + 2],
                        px[k + 3],
                    );
                }
            }
            let uv = Rect::from_min_max(
                Pos2::new(ox as f32 / W as f32, oy as f32 / height as f32),
                Pos2::new((ox + w) as f32 / W as f32, (oy + h) as f32 / height as f32),
            );
            let size = Vec2::new(*w as f32, *h as f32);
            let anchor = anchor_of
                .get(*name)
                .copied()
                .unwrap_or(Vec2::new(size.x * 0.5, size.y - 1.0));
            entries.insert(name.to_string(), Entry { uv, size, anchor });
        }
        let image = ColorImage {
            size: [W, height],
            source_size: Vec2::new(W as f32, height as f32),
            pixels,
        };
        let texture = ctx.load_texture("natur-atlas", image, TextureOptions::NEAREST);
        Some(Self {
            texture,
            entries,
            aliases,
        })
    }

    /// Bild nach Dateiname (ohne Endung), Doppelbilder über die Verweise.
    pub fn get(&self, name: &str) -> Option<&Entry> {
        self.entries.get(name).or_else(|| {
            self.aliases
                .get(name)
                .and_then(|target| self.entries.get(target))
        })
    }

    /// Boden, Wand oder Wasser: `<id>_<variante>`, notfalls Variante 0.
    pub fn tile(&self, id: &str, variant: u32) -> Option<&Entry> {
        self.get(&format!("{id}_{variant}"))
            .or_else(|| self.get(&format!("{id}_0")))
    }

    /// Pflanzen: `<id>_<variante>_<zustand>`, notfalls `normal` bzw. Variante 0.
    pub fn sprite(&self, id: &str, variant: u32, state: &str) -> Option<&Entry> {
        self.get(&format!("{id}_{variant}_{state}"))
            .or_else(|| self.get(&format!("{id}_{variant}_normal")))
            .or_else(|| self.get(&format!("{id}_0_normal")))
    }

    /// Tiere: `<id>_<variante>_<zustand>_<richtung>_f<bild>`.
    pub fn animal(
        &self,
        id: &str,
        variant: u8,
        state: &str,
        dir: &str,
        frame: u32,
    ) -> Option<&Entry> {
        self.get(&format!("{id}_{variant}_{state}_{dir}_f{frame}"))
            .or_else(|| self.get(&format!("{id}_{variant}_{state}_{dir}_f0")))
            .or_else(|| self.get(&format!("{id}_0_graze_{dir}_f0")))
    }

    /// Mitte des weißen Felds – UV für einfarbige Dreiecke.
    pub fn white_uv(&self) -> Pos2 {
        self.entries
            .get(WHITE)
            .map(|e| e.uv.center())
            .unwrap_or(Pos2::ZERO)
    }
}
