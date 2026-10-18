//! Kartenzeichnung: isometrische Ansicht und 2D-Raster, Farben, Auswahl.

use eframe::egui::{self, Color32, Mesh, Pos2, Rect, Shape, Stroke, Vec2};
use nig_core::sim::{RILL_FLOW, STREAM_FLOW};
use nig_core::world::POND_MM;
use nig_core::{Simulation, Terrain, World};

use crate::atlas::{Atlas, Entry};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewMode {
    Iso,
    Map,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layer {
    Nature,
    Height,
    Moisture,
    Water,
    FireRisk,
    Soil,
    Animals,
}

impl Layer {
    pub const ALL: [Layer; 7] = [
        Layer::Nature,
        Layer::Height,
        Layer::Moisture,
        Layer::Water,
        Layer::FireRisk,
        Layer::Soil,
        Layer::Animals,
    ];
    pub fn label(self) -> &'static str {
        match self {
            Layer::Nature => "Natur",
            Layer::Height => "Höhe",
            Layer::Moisture => "Bodenfeuchte",
            Layer::Water => "Wasser",
            Layer::FireRisk => "Brandgefahr",
            Layer::Soil => "Boden/Abtrag",
            Layer::Animals => "Tiere",
        }
    }
}

pub struct Camera {
    /// Bildschirmversatz der Kartenmitte relativ zur Mitte des Zeichenbereichs.
    pub pan: Vec2,
    pub zoom: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            pan: Vec2::new(0.0, 30.0),
            zoom: 1.4,
        }
    }
}

// Grundmaße bei Zoom 1
const ISO_TILE_W: f32 = 16.0;
const ISO_TILE_H: f32 = 8.0;
const ISO_HEIGHT_PX_PER_M: f32 = 0.2;
const MAP_CELL: f32 = 10.0;

fn mix(a: Color32, b: Color32, t: f32) -> Color32 {
    let t = t.clamp(0.0, 1.0);
    let l = |x: u8, y: u8| (x as f32 + (y as f32 - x as f32) * t).round() as u8;
    Color32::from_rgb(l(a.r(), b.r()), l(a.g(), b.g()), l(a.b(), b.b()))
}

fn shade(c: Color32, f: f32) -> Color32 {
    let s = |x: u8| (x as f32 * f).clamp(0.0, 255.0) as u8;
    Color32::from_rgb(s(c.r()), s(c.g()), s(c.b()))
}

pub fn tile_color(world: &World, i: usize, layer: Layer) -> Color32 {
    let t = &world.tiles[i];
    let terrain = world.terrain(i);
    if t.is_ocean() {
        let depth = (-t.elevation / 60.0).clamp(0.0, 1.0);
        return mix(
            Color32::from_rgb(74, 150, 196),
            Color32::from_rgb(18, 46, 104),
            depth,
        );
    }
    match layer {
        Layer::Nature => {
            let base = match terrain {
                Terrain::Beach => Color32::from_rgb(222, 208, 154),
                Terrain::Mountain => Color32::from_rgb(138, 132, 124),
                _ => Color32::from_rgb(150, 126, 88),
            };
            let w = t.moisture_fraction();
            let grass = mix(
                Color32::from_rgb(186, 172, 92),
                Color32::from_rgb(104, 164, 68),
                w * 1.6,
            );
            let mut c = mix(base, grass, t.grass);
            c = mix(c, Color32::from_rgb(118, 130, 60), t.shrub * 0.8);
            // Jungwald heller, Altwald dunkel
            let maturity = (t.forest_age / 60.0).clamp(0.0, 1.0);
            let trees = mix(
                Color32::from_rgb(70, 132, 58),
                Color32::from_rgb(30, 80, 42),
                maturity,
            );
            c = mix(c, trees, t.forest * 0.95);
            c = mix(
                c,
                Color32::from_rgb(122, 104, 84),
                t.deadwood.min(1.0) * 0.7,
            );
            c = mix(
                c,
                Color32::from_rgb(48, 42, 38),
                t.burn_scar * (1.0 - t.grass) * 0.8,
            );
            if t.surface_water > POND_MM {
                c = mix(c, Color32::from_rgb(58, 118, 186), 0.85);
            }
            if t.fire > 0.0 {
                c = mix(c, Color32::from_rgb(245, 110, 30), 0.6 + 0.4 * t.fire);
            }
            c
        }
        Layer::Height => {
            let r = t.elevation / world.max_elevation;
            if r < 0.5 {
                mix(
                    Color32::from_rgb(80, 150, 80),
                    Color32::from_rgb(200, 190, 110),
                    r * 2.0,
                )
            } else {
                mix(
                    Color32::from_rgb(200, 190, 110),
                    Color32::from_rgb(245, 245, 245),
                    (r - 0.5) * 2.0,
                )
            }
        }
        Layer::Moisture => mix(
            Color32::from_rgb(170, 120, 60),
            Color32::from_rgb(40, 90, 200),
            t.moisture_fraction(),
        ),
        Layer::Water => {
            let dry = Color32::from_rgb(60, 56, 50);
            if t.surface_water > POND_MM {
                Color32::from_rgb(80, 160, 255)
            } else {
                // leichte Tönung durch Bodenfeuchte, Wasserläufe als Linien darüber
                mix(dry, Color32::from_rgb(70, 82, 96), t.moisture_fraction())
            }
        }
        // Tiere: gedämpfte Landschaft, Kaninchen als Tönung, Tiere als Punkte darüber
        Layer::Animals => Color32::from_rgb(70, 76, 66),
        Layer::Soil => {
            // Grundton nach Bodendicke: heller Fels … dunkle, tiefe Erde.
            let depth = ((t.soil_depth + t.soil_change) / 1.2).clamp(0.0, 1.0);
            let base = mix(
                Color32::from_rgb(170, 166, 156),
                Color32::from_rgb(92, 64, 40),
                depth,
            );
            // Darüber: Abtrag rot, Ablagerung blaugrün (ab 2 mm pro Jahr kräftig).
            let rate = t.erosion_rate / 0.002;
            if rate < 0.0 {
                mix(
                    base,
                    Color32::from_rgb(214, 74, 42),
                    (-rate).min(1.0) * 0.85,
                )
            } else {
                mix(base, Color32::from_rgb(52, 150, 156), rate.min(1.0) * 0.85)
            }
        }
        Layer::FireRisk => {
            if t.fire > 0.0 {
                return Color32::from_rgb(255, 90, 20);
            }
            let f = (nig_core::fire::flammability(t) / 0.8).clamp(0.0, 1.0);
            if f < 0.5 {
                mix(
                    Color32::from_rgb(60, 110, 70),
                    Color32::from_rgb(220, 200, 80),
                    f * 2.0,
                )
            } else {
                mix(
                    Color32::from_rgb(220, 200, 80),
                    Color32::from_rgb(200, 50, 30),
                    (f - 0.5) * 2.0,
                )
            }
        }
    }
}

/// Helligkeit nach Tageszeit (nur Darstellung).
pub fn daylight(hour: f32) -> f32 {
    let sun = ((hour - 6.0) / 12.0 * std::f32::consts::PI).sin();
    0.6 + 0.4 * sun.clamp(0.0, 1.0).sqrt()
}

fn night_tint(c: Color32, light: f32) -> Color32 {
    if light >= 0.999 {
        return c;
    }

    Color32::from_rgb(
        (c.r() as f32 * light * 0.8) as u8,
        (c.g() as f32 * light * 0.85) as u8,
        (c.b() as f32 * (light * 0.9 + 0.1)).min(255.0) as u8,
    )
}

pub struct DrawResult {
    pub hovered: Option<usize>,
}

fn iso_center(x: u32, y: u32, w: u32, h: u32, zoom: f32) -> Pos2 {
    let fx = x as f32 - w as f32 / 2.0;
    let fy = y as f32 - h as f32 / 2.0;
    Pos2::new(
        (fx - fy) * ISO_TILE_W * 0.5 * zoom,
        (fx + fy) * ISO_TILE_H * 0.5 * zoom,
    )
}

/// Kameraversatz, der ein Feld in die Bildmitte rückt.
pub fn focus_pan(world: &World, i: usize, mode: ViewMode, zoom: f32) -> Vec2 {
    let (x, y) = world.xy(i);
    match mode {
        ViewMode::Iso => {
            let c = iso_center(x, y, world.width, world.height, zoom);
            -(c.to_vec2() - Vec2::new(0.0, iso_lift(world.tiles[i].elevation, zoom)))
        }
        ViewMode::Map => {
            let cell = MAP_CELL * zoom;
            let fx = x as f32 + 0.5 - world.width as f32 * 0.5;
            let fy = y as f32 + 0.5 - world.height as f32 * 0.5;
            -Vec2::new(fx, fy) * cell
        }
    }
}

fn iso_lift(elevation: f32, zoom: f32) -> f32 {
    elevation.max(0.0) * ISO_HEIGHT_PX_PER_M * zoom
}

/// Breite eines Wasserlaufs in Bildpunkten, wächst mit der Wassermenge.
fn stream_width(flow: f32, zoom: f32) -> f32 {
    (flow.sqrt() * 0.32).clamp(0.8, 5.0) * zoom * 0.55
}

fn stream_color(flow: f32, light: f32) -> Color32 {
    // schwache Rinnsale blass, kräftige Bäche satt
    let a = 0.3 + 0.7 * ((flow - RILL_FLOW) / (2.0 * STREAM_FLOW)).clamp(0.0, 1.0);
    let c = night_tint(Color32::from_rgb(78, 150, 230), light);
    Color32::from_rgba_unmultiplied(c.r(), c.g(), c.b(), (a * 255.0) as u8)
}

fn segment(mesh: &mut Mesh, a: Pos2, b: Pos2, width: f32, c: Color32) {
    let d = b - a;
    let len = d.length();
    if len < 0.01 {
        return;
    }
    let n = Vec2::new(-d.y, d.x) / len * (width * 0.5);
    quad(mesh, [a + n, b + n, b - n, a - n], c);
}

fn quad(mesh: &mut Mesh, p: [Pos2; 4], c: Color32) {
    let i = mesh.vertices.len() as u32;
    for v in p {
        mesh.colored_vertex(v, c);
    }
    mesh.add_triangle(i, i + 1, i + 2);
    mesh.add_triangle(i, i + 2, i + 3);
}

/// Einstellungen für die Grafik-Darstellung (Codex' Assets).
#[derive(Clone, Copy)]
pub struct Gfx<'a> {
    pub atlas: &'a Atlas,
    /// Laufende Zeit in Sekunden (für Animationen).
    pub time_s: f64,
    /// Bei 1 Std/s: Tiere bewegen sich sichtbar und gleiten zwischen Feldern.
    pub animate: bool,
    /// Anteil des Tages (0..1) für das Gleiten.
    pub day_frac: f32,
}

fn tex_quad(mesh: &mut Mesh, p: [Pos2; 4], uv: [Pos2; 4], c: Color32) {
    let i = mesh.vertices.len() as u32;
    for k in 0..4 {
        mesh.vertices.push(egui::epaint::Vertex {
            pos: p[k],
            uv: uv[k],
            color: c,
        });
    }
    mesh.add_triangle(i, i + 1, i + 2);
    mesh.add_triangle(i, i + 2, i + 3);
}

fn tex_rect(mesh: &mut Mesh, r: Rect, e: &Entry, c: Color32) {
    tex_quad(
        mesh,
        [
            r.left_top(),
            r.right_top(),
            r.right_bottom(),
            r.left_bottom(),
        ],
        [
            e.uv.left_top(),
            e.uv.right_top(),
            e.uv.right_bottom(),
            e.uv.left_bottom(),
        ],
        c,
    );
}

fn with_alpha(c: Color32, a: f32) -> Color32 {
    let a = a.clamp(0.0, 1.0);
    Color32::from_rgba_premultiplied(
        (c.r() as f32 * a) as u8,
        (c.g() as f32 * a) as u8,
        (c.b() as f32 * a) as u8,
        (255.0 * a) as u8,
    )
}

/// Isometrische Ansicht mit Codex' Bildern. Erste technische Fassung: Boden,
/// Überlagerungen, Wände, Pflanzen und Tiere werden in einem Durchgang von
/// hinten nach vorne gelegt. Feinheiten der Darstellung gestaltet Codex.
#[allow(clippy::too_many_arguments)]
fn draw_iso_gfx(
    painter: &egui::Painter,
    sim: &Simulation,
    animals: &[nig_core::appearance::AnimalSprite],
    g: Gfx<'_>,
    origin: Pos2,
    rect: Rect,
    z: f32,
    mouse: Option<Pos2>,
    light: f32,
) -> Option<usize> {
    use nig_core::appearance as ap;
    let world = &sim.world;
    let (w, h) = (world.width, world.height);
    let (hw, hh) = (ISO_TILE_W * 0.5 * z, ISO_TILE_H * 0.5 * z);
    // Asset-Grundmaß gilt bei Zoom 4 (Raute 64 × 32)
    let scale = z / 4.0;
    let atlas = g.atlas;
    let white = atlas.white_uv();
    let lit = night_tint(Color32::WHITE, light);
    let mut mesh = Mesh::with_texture(atlas.texture.id());
    let mut hovered = None;
    let height_of = |x: i32, y: i32| -> f32 {
        if x < 0 || y < 0 || x >= w as i32 || y >= h as i32 {
            0.0
        } else {
            world.tile(x as u32, y as u32).elevation
        }
    };
    let center_of = |i: usize| -> Pos2 {
        let (x, y) = world.xy(i);
        origin + iso_center(x, y, w, h, z).to_vec2()
            - Vec2::new(0.0, iso_lift(world.tiles[i].elevation, z))
    };
    // Zoomstufen (gestaltung.md 4.3): im Überblick und bei schneller Zeit
    // keine Einzeltiere, sondern Spuren, wo Tiere leben.
    let individuals = z >= OVERVIEW_ZOOM && !animals.is_empty();
    let tracks = !individuals && (!sim.animals.list.is_empty() || sim.animals.rabbit_total() > 0.0);
    // Tiere je Feld
    let mut by_tile: Vec<Vec<usize>> = vec![Vec::new(); world.tiles.len()];
    if individuals {
        for (k, a) in animals.iter().enumerate() {
            by_tile[a.tile as usize].push(k);
        }
    }
    let mut carcass_at: Vec<Vec<&'static str>> = vec![Vec::new(); world.tiles.len()];
    if individuals {
        for (tile, id) in ap::carcasses(&sim.animals) {
            carcass_at[tile as usize].push(id);
        }
    }
    let carcass_entry = |id: &str| atlas.tile(id, 0).or_else(|| atlas.sprite(id, 0, "normal"));
    // Spurendichte je Feld: Hirsche, Wölfe, Füchse (Kaninchen aus dem Bestand)
    let mut pack: Vec<[u16; 3]> = Vec::new();
    if tracks {
        pack = vec![[0; 3]; world.tiles.len()];
        for a in &sim.animals.list {
            let k = a.species.index();
            if k < 3 {
                pack[a.tile as usize][k] += 1;
            }
        }
    }
    let track_ids = ["tracks_deer", "tracks_wolf", "tracks_fox", "tracks_rabbit"];
    let track_entries: Vec<[Option<&Entry>; 4]> = track_ids
        .iter()
        .map(|id| [0, 1, 2, 3].map(|v| atlas.tile(id, v)))
        .collect();
    let solid = |mesh: &mut Mesh, p: [Pos2; 4], c: Color32| {
        tex_quad(mesh, p, [white; 4], c);
    };

    for s in 0..(w + h - 1) {
        for x in 0..w {
            let Some(y) = s.checked_sub(x) else { continue };
            if y >= h {
                continue;
            }
            let i = world.idx(x, y);
            let t = &world.tiles[i];
            let e = t.elevation;
            let c = center_of(i);
            if c.x < rect.left() - hw * 2.0
                || c.x > rect.right() + hw * 2.0
                || c.y < rect.top() - 200.0
                || c.y > rect.bottom() + 200.0
            {
                continue;
            }
            let ground = ap::ground(world, i);
            let variant = (nig_core::rng::hash2(x as i32, y as i32, world.seed) % 4) as u32;
            let n = Pos2::new(c.x, c.y - hh);
            let east = Pos2::new(c.x + hw, c.y);
            let south = Pos2::new(c.x, c.y + hh);
            let west = Pos2::new(c.x - hw, c.y);

            // Seitenwände, in Streifen von 32 Bildpunkten gekachelt
            let wall = atlas.tile(ground.wall_id(), 0);
            for (drop, a, b, f) in [
                (
                    iso_lift(e, z) - iso_lift(height_of(x as i32 + 1, y as i32), z),
                    east,
                    south,
                    0.72,
                ),
                (
                    iso_lift(e, z) - iso_lift(height_of(x as i32, y as i32 + 1), z),
                    south,
                    west,
                    0.56,
                ),
            ] {
                if drop <= 0.0 {
                    continue;
                }
                let col = shade(lit, f);
                match wall {
                    Some(we) if !ground.is_water() => {
                        let seg = 32.0 * scale;
                        let mut top = 0.0;
                        while top < drop {
                            let bottom = (top + seg).min(drop);
                            let v0 = we.uv.top();
                            let v1 = v0 + (we.uv.bottom() - v0) * ((bottom - top) / seg);
                            let (dt, db) = (Vec2::new(0.0, top), Vec2::new(0.0, bottom));
                            tex_quad(
                                &mut mesh,
                                [a + dt, b + dt, b + db, a + db],
                                [
                                    Pos2::new(we.uv.left(), v0),
                                    Pos2::new(we.uv.right(), v0),
                                    Pos2::new(we.uv.right(), v1),
                                    Pos2::new(we.uv.left(), v1),
                                ],
                                col,
                            );
                            top = bottom;
                        }
                    }
                    _ => {
                        let base = night_tint(shade(tile_color(world, i, Layer::Nature), f), light);
                        let d = Vec2::new(0.0, drop);
                        solid(&mut mesh, [a, b, b + d, a + d], base);
                    }
                }
            }

            // Boden und Überlagerungen
            let top_rect =
                Rect::from_min_max(Pos2::new(c.x - hw, c.y - hh), Pos2::new(c.x + hw, c.y + hh));
            match atlas.tile(ground.asset_id(), variant) {
                Some(ge) => tex_rect(&mut mesh, top_rect, ge, lit),
                None => solid(
                    &mut mesh,
                    [n, east, south, west],
                    night_tint(tile_color(world, i, Layer::Nature), light),
                ),
            }
            for o in ap::overlays(t) {
                if let Some(oe) = atlas.tile(o.id, variant) {
                    tex_rect(&mut mesh, top_rect, oe, with_alpha(lit, o.alpha));
                }
            }
            if t.fire > 0.0 {
                let fc = Color32::from_rgb(245, 110, 30);
                solid(
                    &mut mesh,
                    [n, east, south, west],
                    with_alpha(fc, 0.35 + 0.4 * t.fire),
                );
            }

            // Spuren statt Einzeltieren (15.5, 4.3)
            if tracks && !t.is_ocean() {
                for (k, entries) in track_entries.iter().enumerate() {
                    let d = if k < 3 {
                        (pack[i][k] as f32 / 4.0).min(1.0)
                    } else {
                        ap::track_density(
                            &sim.animals,
                            i as u32,
                            nig_core::animals::Species::Rabbit,
                        )
                    };
                    if d <= 0.0 {
                        continue;
                    }
                    match entries[variant as usize] {
                        Some(te) => tex_rect(&mut mesh, top_rect, te, with_alpha(lit, d)),
                        // Platzhalter, bis es die Spurenbilder gibt: Hauch
                        // der Kartenfarbe
                        None => solid(
                            &mut mesh,
                            [n, east, south, west],
                            with_alpha(night_tint(TRACK_COLORS[k], light), 0.35 * d),
                        ),
                    }
                }
            }

            // Wasserläufe als Linien
            if !t.is_ocean() {
                let flat = iso_center(x, y, w, h, z);
                let mut line = |a: Pos2, b: Pos2, flow: f32| {
                    let d = b - a;
                    let len = d.length();
                    if len < 0.01 {
                        return;
                    }
                    let nn = Vec2::new(-d.y, d.x) / len * (stream_width(flow, z) * 0.5);
                    solid(
                        &mut mesh,
                        [a + nn, b + nn, b - nn, a - nn],
                        stream_color(flow, light),
                    );
                };
                if t.flow >= RILL_FLOW
                    && let Some(d) = world.downstream[i]
                {
                    let (dx, dy) = world.xy(d as usize);
                    let dir = iso_center(dx, dy, w, h, z) - flat;
                    line(c, c + dir * 0.5, t.flow);
                }
                for nb in world.neighbors(i) {
                    let up = &world.tiles[nb];
                    if up.flow >= RILL_FLOW && world.downstream[nb] == Some(i as u32) {
                        let (ux, uy) = world.xy(nb);
                        let dir = iso_center(ux, uy, w, h, z) - flat;
                        line(c + dir * 0.5, c, up.flow);
                    }
                }
            }

            // Pflanzen und Tiere, nach Fußpunkt von hinten nach vorne
            enum Thing<'b> {
                Plant(&'b ap::Sprite),
                Animal(&'b ap::AnimalSprite),
                Carcass(&'static str, f32, f32),
            }
            let plants = ap::sprites(world, i);
            let mut things: Vec<(f32, Thing)> = plants
                .iter()
                .map(|p| (p.v, Thing::Plant(p)))
                .chain(
                    by_tile[i]
                        .iter()
                        .map(|&k| (animals[k].pos.1, Thing::Animal(&animals[k]))),
                )
                .chain(carcass_at[i].iter().enumerate().map(|(k, &id)| {
                    let (u, v) = [(0.15, 0.3), (-0.35, 0.15), (0.45, 0.0)][k % 3];
                    (v, Thing::Carcass(id, u, v))
                }))
                .collect();
            things.sort_by(|a, b| a.0.total_cmp(&b.0));
            for (_, th) in things {
                match th {
                    Thing::Plant(p) => {
                        let Some(pe) = atlas.sprite(p.id, p.variant_seed % 4, p.state) else {
                            continue;
                        };
                        let foot = c + Vec2::new(p.u * hw, p.v * hh);
                        let min = foot - pe.anchor * scale;
                        tex_rect(
                            &mut mesh,
                            Rect::from_min_size(min, pe.size * scale),
                            pe,
                            lit,
                        );
                    }
                    Thing::Animal(a) => {
                        // Bei schneller Zeit kein Laufen: stehen bzw. ruhen (15.5)
                        let (state, frames, ms) = match (a.state, g.animate) {
                            ("rest", _) => ("rest", 1, 1000.0),
                            ("walk", true) => ("walk", 4, 200.0),
                            ("run", true) => ("run", 4, 100.0),
                            _ => ("graze", 2, 600.0),
                        };
                        let frame = if g.animate {
                            ((g.time_s * 1000.0 / ms) as u64 % frames) as u32
                        } else {
                            0
                        };
                        let Some(ae) = atlas.animal(a.id, a.variant, state, a.dir, frame) else {
                            continue;
                        };
                        let mut foot = c + Vec2::new(a.pos.0 * hw, a.pos.1 * hh);
                        if g.animate && a.prev_tile != a.tile {
                            let from = center_of(a.prev_tile as usize)
                                + Vec2::new(a.pos.0 * hw, a.pos.1 * hh);
                            foot = from + (foot - from) * g.day_frac;
                        }
                        // Auf die Darstellungsbreite bringen, gleich in
                        // welcher Auflösung das Bild gezeichnet ist (4.3).
                        let reference = atlas
                            .animal(a.id, a.variant, "graze", a.dir, 0)
                            .map_or(ae.size.x, |r| r.size.x)
                            .max(1.0);
                        let s = scale * a.scale * animal_width(a.id) / reference;
                        let tint = shade(lit, 1.0 - 0.25 * a.weakness);
                        tex_rect(
                            &mut mesh,
                            Rect::from_min_size(foot - ae.anchor * s, ae.size * s),
                            ae,
                            tint,
                        );
                    }
                    Thing::Carcass(id, u, v) => {
                        let Some(ce) = carcass_entry(id) else {
                            continue;
                        };
                        let foot = c + Vec2::new(u * hw, v * hh);
                        let s = scale * animal_width(id) / ce.size.x.max(1.0);
                        tex_rect(
                            &mut mesh,
                            Rect::from_min_size(foot - ce.anchor * s, ce.size * s),
                            ce,
                            lit,
                        );
                    }
                }
            }

            if let Some(m) = mouse {
                let dx = (m.x - c.x).abs() / hw;
                let dy = (m.y - c.y).abs() / hh;
                if dx + dy <= 1.0 {
                    hovered = Some(i);
                }
            }
        }
    }
    painter.add(Shape::mesh(mesh));
    hovered
}

/// Unterhalb dieses Zooms („Überblick“, gestaltung.md 4.3) erscheinen keine
/// einzelnen Tiere, sondern Spuren.
pub const OVERVIEW_ZOOM: f32 = 1.5;

/// Darstellungsbreite im Grundmaß (Raute 64 × 32), gestaltung.md 4.3/15.3.
/// Jedes Tierbild wird auf diese Breite gebracht – gleich, ob es in
/// einfacher oder doppelter Auflösung gezeichnet ist. Doppelt aufgelöste
/// Bilder bleiben so bei Zoom 8 scharf.
fn animal_width(id: &str) -> f32 {
    match id {
        "deer" => 14.0,
        "deer_young" => 9.0,
        "wolf" => 13.0,
        "fox" => 10.0,
        "rabbit" => 6.0,
        "carcass_large" => 14.0,
        "carcass_small" => 7.0,
        _ => 8.0,
    }
}

/// Platzhalterfarben der Spuren: Hirsch, Wolf, Fuchs, Kaninchen (15.6).
const TRACK_COLORS: [Color32; 4] = [
    Color32::from_rgb(176, 122, 68),
    Color32::from_rgb(154, 160, 166),
    Color32::from_rgb(210, 104, 44),
    Color32::from_rgb(200, 184, 152),
];

/// Kartenfarben der Tiere (gestaltung.md 15.6). Platzhalter, bis die
/// Tier-Sprites eingebunden sind.
fn animal_color(id: &str) -> Color32 {
    match id {
        "deer" | "deer_young" => Color32::from_rgb(176, 122, 68),
        "wolf" => Color32::from_rgb(154, 160, 166),
        "fox" => Color32::from_rgb(210, 104, 44),
        _ => Color32::from_rgb(200, 184, 152),
    }
}

fn animal_radius(id: &str, scale: f32) -> f32 {
    scale
        * match id {
            "deer" => 1.0,
            "deer_young" | "wolf" => 0.8,
            "fox" => 0.65,
            _ => 0.45,
        }
}

#[allow(clippy::too_many_arguments)]
pub fn draw(
    ui: &mut egui::Ui,
    sim: &Simulation,
    mode: ViewMode,
    layer: Layer,
    cam: &mut Camera,
    selected: Option<usize>,
    light: f32,
    show_animals: bool,
    gfx: Option<Gfx<'_>>,
) -> (egui::Response, DrawResult) {
    let (resp, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::click_and_drag());
    let rect = resp.rect;
    painter.rect_filled(rect, 0.0, Color32::from_rgb(12, 26, 52));

    // Kamera: ziehen = verschieben, Mausrad = zoomen (zum Mauszeiger hin)
    if resp.dragged() {
        cam.pan += resp.drag_delta();
    }
    if resp.hovered() {
        let scroll = ui.input(|i| i.smooth_scroll_delta.y);
        let pinch = ui.input(|i| i.zoom_delta());
        let factor = (scroll * 0.0015).exp() * pinch;
        if (factor - 1.0).abs() > 1e-4 {
            let old = cam.zoom;
            cam.zoom = (cam.zoom * factor).clamp(0.4, 8.0);
            if let Some(mp) = resp.hover_pos() {
                let origin = rect.center() + cam.pan;
                let rel = mp - origin;
                cam.pan += rel - rel * (cam.zoom / old);
            }
        }
    }

    let world = &sim.world;
    let origin = rect.center() + cam.pan;
    let mouse = resp.hover_pos();
    let mut hovered = None;
    let (w, h) = (world.width, world.height);
    let mut mesh = Mesh::default();
    let show_streams = matches!(layer, Layer::Nature | Layer::Water);
    // Einzelne Tiere nur bei langsamer Zeit (sonst zappelt es), auf der
    // Ebene „Tiere“ immer.
    let animals = if show_animals || layer == Layer::Animals {
        nig_core::appearance::animal_sprites(&sim.animals, &sim.world)
    } else {
        Vec::new()
    };
    let rabbit_tint = |i: usize, c: Color32| -> Color32 {
        if layer != Layer::Animals {
            return c;
        }
        let r = sim.animals.rabbits.get(i).copied().unwrap_or(0.0);
        mix(
            c,
            Color32::from_rgb(200, 184, 152),
            (r / 15.0).clamp(0.0, 1.0) * 0.8,
        )
    };

    match mode {
        ViewMode::Iso => {
            let z = cam.zoom;
            let (hw, hh) = (ISO_TILE_W * 0.5 * z, ISO_TILE_H * 0.5 * z);
            let height_of = |x: i32, y: i32| -> f32 {
                if x < 0 || y < 0 || x >= w as i32 || y >= h as i32 {
                    0.0
                } else {
                    world.tile(x as u32, y as u32).elevation
                }
            };
            if let Some(g) = gfx.filter(|_| layer == Layer::Nature) {
                hovered = draw_iso_gfx(&painter, sim, &animals, g, origin, rect, z, mouse, light);
            } else {
                mesh.reserve_vertices((w * h * 12) as usize);
                // Von hinten nach vorne zeichnen (Maleralgorithmus)
                for s in 0..(w + h - 1) {
                    for x in 0..w {
                        let Some(y) = s.checked_sub(x) else { continue };
                        if y >= h {
                            continue;
                        }
                        let i = world.idx(x, y);
                        let e = world.tiles[i].elevation;
                        let c = origin + iso_center(x, y, w, h, z).to_vec2()
                            - Vec2::new(0.0, iso_lift(e, z));
                        if c.x < rect.left() - hw * 2.0
                            || c.x > rect.right() + hw * 2.0
                            || c.y < rect.top() - 200.0
                            || c.y > rect.bottom() + 200.0
                        {
                            continue;
                        }
                        let col = night_tint(rabbit_tint(i, tile_color(world, i, layer)), light);
                        let n = Pos2::new(c.x, c.y - hh);
                        let east = Pos2::new(c.x + hw, c.y);
                        let south = Pos2::new(c.x, c.y + hh);
                        let west = Pos2::new(c.x - hw, c.y);
                        // Seitenwände zu tieferen Nachbarn vorne
                        let drop_r =
                            iso_lift(e, z) - iso_lift(height_of(x as i32 + 1, y as i32), z);
                        if drop_r > 0.0 {
                            let d = Vec2::new(0.0, drop_r);
                            quad(
                                &mut mesh,
                                [east, south, south + d, east + d],
                                shade(col, 0.72),
                            );
                        }
                        let drop_l =
                            iso_lift(e, z) - iso_lift(height_of(x as i32, y as i32 + 1), z);
                        if drop_l > 0.0 {
                            let d = Vec2::new(0.0, drop_l);
                            quad(
                                &mut mesh,
                                [south, west, west + d, south + d],
                                shade(col, 0.56),
                            );
                        }
                        quad(&mut mesh, [n, east, south, west], col);

                        // Wasserlauf: halbe Strecke zum Abfluss und von jedem Zufluss
                        if show_streams && !world.tiles[i].is_ocean() {
                            let flat = iso_center(x, y, w, h, z);
                            let t = &world.tiles[i];
                            if t.flow >= RILL_FLOW
                                && let Some(d) = world.downstream[i]
                            {
                                let (dx, dy) = world.xy(d as usize);
                                let dir = iso_center(dx, dy, w, h, z) - flat;
                                segment(
                                    &mut mesh,
                                    c,
                                    c + dir * 0.5,
                                    stream_width(t.flow, z),
                                    stream_color(t.flow, light),
                                );
                            }
                            for nb in world.neighbors(i) {
                                let up = &world.tiles[nb];
                                if up.flow >= RILL_FLOW && world.downstream[nb] == Some(i as u32) {
                                    let (ux, uy) = world.xy(nb);
                                    let dir = iso_center(ux, uy, w, h, z) - flat;
                                    segment(
                                        &mut mesh,
                                        c + dir * 0.5,
                                        c,
                                        stream_width(up.flow, z),
                                        stream_color(up.flow, light),
                                    );
                                }
                            }
                        }

                        if let Some(m) = mouse {
                            let dx = (m.x - c.x).abs() / hw;
                            let dy = (m.y - c.y).abs() / hh;
                            if dx + dy <= 1.0 {
                                hovered = Some(i); // der vorderste Treffer gewinnt
                            }
                        }
                    }
                }
                painter.add(Shape::mesh(mesh));

                for a in &animals {
                    let (x, y) = world.xy(a.tile as usize);
                    let e = world.tiles[a.tile as usize].elevation;
                    let c = origin + iso_center(x, y, w, h, z).to_vec2()
                        - Vec2::new(0.0, iso_lift(e, z))
                        + Vec2::new(a.pos.0 * hw, a.pos.1 * hh);
                    if !rect.contains(c) {
                        continue;
                    }
                    let r = (animal_radius(a.id, a.scale) * 1.6 * z).max(1.2);
                    painter.circle_filled(c, r, night_tint(animal_color(a.id), light));
                }
            }

            let outline = |i: usize, color: Color32| {
                let (x, y) = world.xy(i);
                let e = world.tiles[i].elevation;
                let c =
                    origin + iso_center(x, y, w, h, z).to_vec2() - Vec2::new(0.0, iso_lift(e, z));
                let pts = vec![
                    Pos2::new(c.x, c.y - hh),
                    Pos2::new(c.x + hw, c.y),
                    Pos2::new(c.x, c.y + hh),
                    Pos2::new(c.x - hw, c.y),
                ];
                painter.add(Shape::closed_line(pts, Stroke::new(1.5, color)));
            };
            for s in &world.springs {
                let i = world.idx(s.x, s.y);
                let e = world.tiles[i].elevation;
                let c = origin + iso_center(s.x, s.y, w, h, z).to_vec2()
                    - Vec2::new(0.0, iso_lift(e, z));
                let col = if s.dry {
                    Color32::from_rgb(150, 110, 70)
                } else {
                    Color32::from_rgb(120, 230, 255)
                };
                painter.circle_filled(c, (2.5 * z).max(2.5), col);
            }
            if let Some(i) = hovered {
                outline(i, Color32::from_white_alpha(140));
            }
            if let Some(i) = selected {
                outline(i, Color32::from_rgb(255, 214, 60));
            }
        }
        ViewMode::Map => {
            let cell = MAP_CELL * cam.zoom;
            let top_left = origin - Vec2::new(w as f32, h as f32) * cell * 0.5;
            for y in 0..h {
                for x in 0..w {
                    let i = world.idx(x, y);
                    let r = Rect::from_min_size(
                        top_left + Vec2::new(x as f32, y as f32) * cell,
                        Vec2::splat(cell),
                    );
                    if !r.intersects(rect) {
                        continue;
                    }
                    mesh.add_colored_rect(
                        r,
                        night_tint(rabbit_tint(i, tile_color(world, i, layer)), light),
                    );
                    if let Some(m) = mouse
                        && r.contains(m)
                    {
                        hovered = Some(i);
                    }
                }
            }
            if show_streams {
                let center = |i: usize| {
                    let (x, y) = world.xy(i);
                    top_left + Vec2::new(x as f32 + 0.5, y as f32 + 0.5) * cell
                };
                for i in 0..world.tiles.len() {
                    let t = &world.tiles[i];
                    if t.is_ocean() || t.flow < RILL_FLOW {
                        continue;
                    }
                    if let Some(d) = world.downstream[i] {
                        segment(
                            &mut mesh,
                            center(i),
                            center(d as usize),
                            stream_width(t.flow, cam.zoom) * 1.2,
                            stream_color(t.flow, light),
                        );
                    }
                }
            }
            painter.add(Shape::mesh(mesh));
            for a in animals.iter().filter(|a| a.id != "rabbit") {
                let (x, y) = world.xy(a.tile as usize);
                let c = top_left
                    + Vec2::new(x as f32 + 0.5, y as f32 + 0.5) * cell
                    + Vec2::new(a.pos.0, a.pos.1) * cell * 0.45;
                if !rect.contains(c) {
                    continue;
                }
                let r = (animal_radius(a.id, a.scale) * cell * 0.22).max(1.2);
                painter.circle_filled(c, r, night_tint(animal_color(a.id), light));
            }
            let cell_rect = |i: usize| {
                let (x, y) = world.xy(i);
                Rect::from_min_size(
                    top_left + Vec2::new(x as f32, y as f32) * cell,
                    Vec2::splat(cell),
                )
            };
            for s in &world.springs {
                let col = if s.dry {
                    Color32::from_rgb(150, 110, 70)
                } else {
                    Color32::from_rgb(120, 230, 255)
                };
                painter.circle_filled(
                    cell_rect(world.idx(s.x, s.y)).center(),
                    (cell * 0.3).max(2.0),
                    col,
                );
            }
            if let Some(i) = hovered {
                painter.rect_stroke(
                    cell_rect(i),
                    0.0,
                    Stroke::new(1.0, Color32::from_white_alpha(140)),
                    egui::StrokeKind::Inside,
                );
            }
            if let Some(i) = selected {
                painter.rect_stroke(
                    cell_rect(i),
                    0.0,
                    Stroke::new(2.0, Color32::from_rgb(255, 214, 60)),
                    egui::StrokeKind::Inside,
                );
            }
        }
    }

    (resp, DrawResult { hovered })
}
