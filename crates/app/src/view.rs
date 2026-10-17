//! Kartenzeichnung: isometrische Ansicht und 2D-Raster, Farben, Auswahl.

use eframe::egui::{self, Color32, Mesh, Pos2, Rect, Shape, Stroke, Vec2};
use nig_core::sim::STREAM_FLOW;
use nig_core::world::POND_MM;
use nig_core::{Simulation, Terrain, World};

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
}

impl Layer {
    pub const ALL: [Layer; 5] = [
        Layer::Nature,
        Layer::Height,
        Layer::Moisture,
        Layer::Water,
        Layer::FireRisk,
    ];
    pub fn label(self) -> &'static str {
        match self {
            Layer::Nature => "Natur",
            Layer::Height => "Höhe",
            Layer::Moisture => "Bodenfeuchte",
            Layer::Water => "Wasser",
            Layer::FireRisk => "Brandgefahr",
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

fn iso_lift(elevation: f32, zoom: f32) -> f32 {
    elevation.max(0.0) * ISO_HEIGHT_PX_PER_M * zoom
}

/// Ab diesem Durchfluss (mm/Tag) wird ein Rinnsal gezeichnet.
pub const RILL_FLOW: f32 = 3.0;

/// Breite eines Wasserlaufs in Bildpunkten, wächst mit der Wassermenge.
fn stream_width(flow: f32, zoom: f32) -> f32 {
    (flow.sqrt() * 0.32).clamp(0.8, 5.0) * zoom * 0.55
}

fn stream_color(flow: f32, light: f32) -> Color32 {
    let a = 0.5 + 0.5 * (flow / STREAM_FLOW).clamp(0.0, 1.0);
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

pub fn draw(
    ui: &mut egui::Ui,
    sim: &Simulation,
    mode: ViewMode,
    layer: Layer,
    cam: &mut Camera,
    selected: Option<usize>,
    light: f32,
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
                    let col = night_tint(tile_color(world, i, layer), light);
                    let n = Pos2::new(c.x, c.y - hh);
                    let east = Pos2::new(c.x + hw, c.y);
                    let south = Pos2::new(c.x, c.y + hh);
                    let west = Pos2::new(c.x - hw, c.y);
                    // Seitenwände zu tieferen Nachbarn vorne
                    let drop_r = iso_lift(e, z) - iso_lift(height_of(x as i32 + 1, y as i32), z);
                    if drop_r > 0.0 {
                        let d = Vec2::new(0.0, drop_r);
                        quad(
                            &mut mesh,
                            [east, south, south + d, east + d],
                            shade(col, 0.72),
                        );
                    }
                    let drop_l = iso_lift(e, z) - iso_lift(height_of(x as i32, y as i32 + 1), z);
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
                    mesh.add_colored_rect(r, night_tint(tile_color(world, i, layer), light));
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
