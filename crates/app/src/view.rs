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

impl Camera {
    /// Startansicht, in die die ganze Insel passt (größere Welten weiter weg).
    pub fn for_world(world: &nig_core::world::World) -> Self {
        let n = world.width.max(world.height) as f32;
        Self {
            pan: Vec2::new(0.0, 30.0 * 64.0 / n),
            zoom: 1.4 * 64.0 / n,
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
/// Bettbreite (Radius als Anteil der Rautenbreite) je Stärke, passend zu
/// Codex' Bachstücken (`art/nature/stream-beds-style-b/config.json`).
const BED_RADIUS: [f32; 3] = [0.035, 0.055, 0.085];
const WATER_RADIUS: [f32; 3] = [0.012, 0.027, 0.050];

/// Ein Fall an der Wand: oben `head`, längs gekachelt `body`, unten `foot`
/// (`stream_drop_<wet|dry>_<stärke>_<left|right>_<teil>_f<bild>`). Der Streifen
/// folgt der Schräge der Wand (`k` = Steigung der Wandoberkante), damit oben
/// und unten keine Lücke entsteht. Ohne Bilder: einfarbiger Streifen.
#[allow(clippy::too_many_arguments)]
fn draw_stream_drop(
    mesh: &mut Mesh,
    atlas: &Atlas,
    g: &Gfx<'_>,
    white: Pos2,
    m: Pos2,
    k: f32,
    drop: f32,
    hw: f32,
    d: &nig_core::appearance::StreamDrop,
    tile: usize,
    light: f32,
    lit: Color32,
) {
    if drop < 1.0 {
        return;
    }
    let size = (d.size as usize).min(2);
    let width = 2.0 * BED_RADIUS[size] * 2.0 * hw;
    let half = width * 0.5;
    // Viereck für den senkrechten Abschnitt t0..t1 unter der Oberkante
    let quad = |t0: f32, t1: f32| {
        [
            m + Vec2::new(-half, -k * half + t0),
            m + Vec2::new(half, k * half + t0),
            m + Vec2::new(half, k * half + t1),
            m + Vec2::new(-half, -k * half + t1),
        ]
    };
    let state = if d.wet { "wet" } else { "dry" };
    let side = if d.side == "left" { "left" } else { "right" };
    let frame = if d.wet && g.animate {
        (((g.time_s * 1000.0 / 500.0) as u64 + tile as u64 * 3) % 4) as u32
    } else {
        0
    };
    let part = |p: &str| {
        atlas
            .get(&format!("stream_drop_{state}_{size}_{side}_{p}_f{frame}"))
            .or_else(|| atlas.get(&format!("stream_drop_{state}_{size}_{side}_{p}_f0")))
    };
    if let (Some(head), Some(body), Some(foot)) = (part("head"), part("body"), part("foot")) {
        // Seitenverhältnis der Bilder bleibt: Kopf/Fuß 2:1, Rinne 1:2
        let mut cap = width * 0.5;
        if drop < 2.0 * cap {
            cap = drop * 0.5;
        }
        let uv = |e: &Entry, f0: f32, f1: f32| {
            let v0 = e.uv.top() + (e.uv.bottom() - e.uv.top()) * f0;
            let v1 = e.uv.top() + (e.uv.bottom() - e.uv.top()) * f1;
            [
                Pos2::new(e.uv.left(), v0),
                Pos2::new(e.uv.right(), v0),
                Pos2::new(e.uv.right(), v1),
                Pos2::new(e.uv.left(), v1),
            ]
        };
        let piece = width * 2.0;
        let mut t = cap;
        while t < drop - cap - 0.5 {
            let t1 = (t + piece).min(drop - cap);
            tex_quad(mesh, quad(t, t1), uv(body, 0.0, (t1 - t) / piece), lit);
            t = t1;
        }
        tex_quad(mesh, quad(0.0, cap), uv(head, 0.0, 1.0), lit);
        tex_quad(mesh, quad(drop - cap, drop), uv(foot, 0.0, 1.0), lit);
    } else {
        let color = if d.wet {
            let c = night_tint(Color32::from_rgb(110, 170, 235), light);
            Color32::from_rgba_unmultiplied(c.r(), c.g(), c.b(), 230)
        } else {
            let c = night_tint(Color32::from_rgb(170, 158, 130), light);
            Color32::from_rgba_unmultiplied(c.r(), c.g(), c.b(), 200)
        };
        tex_quad(mesh, quad(0.0, drop), [white; 4], color);
        if d.wet {
            // heller Wasserkern
            let core = WATER_RADIUS[size] / BED_RADIUS[size];
            let inner = |t0: f32, t1: f32| {
                let h = half * core;
                [
                    m + Vec2::new(-h, -k * h + t0),
                    m + Vec2::new(h, k * h + t0),
                    m + Vec2::new(h, k * h + t1),
                    m + Vec2::new(-h, -k * h + t1),
                ]
            };
            tex_quad(
                mesh,
                inner(0.0, drop),
                [white; 4],
                night_tint(Color32::from_rgb(200, 230, 250), light),
            );
        }
    }
}

/// Kartenversatz je Bildschirmrichtung (Reihenfolge wie `appearance::STREAM_DIRS`).
const DIR_OFFSETS: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
];

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
    /// Bei 1 Std/s (auch angehalten): Menschen schlafen nachts, gehen morgens.
    pub hourly: bool,
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
    detail: bool,
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
    // Kadaver wie Tiere benannt: `<id>_<variante>_rest_<richtung>_f0`
    let carcass_name = |id: &str, k: usize| {
        let (variant, dir) = [(0, "l"), (1, "r"), (1, "l")][k % 3];
        atlas
            .animal_name(id, variant, "rest", dir, 0)
            .or_else(|| atlas.first_of(&[format!("{id}_0")]))
    };
    // Menschen: einzeln ab der Stufe Landschaft und bei langsamer Zeit,
    // sonst nur als Zeichen (17.2).
    let people_close = z >= OVERVIEW_ZOOM && detail;
    let persons = ap::person_sprites(&sim.people, world);
    let mut people_at: Vec<Vec<usize>> = vec![Vec::new(); world.tiles.len()];
    let mut bodies_at: Vec<Vec<(u8, &'static str)>> = vec![Vec::new(); world.tiles.len()];
    let hour = g.day_frac * 24.0;
    // Was gerade gezeigt wird: bei 1 Std/s nach Tageszeit, sonst die Tätigkeit
    let shown_state = |p: &ap::PersonSprite| -> (&'static str, f32) {
        if g.hourly {
            ap::person_state_at(p, hour)
        } else {
            // Zeitraffer: am Platz, ohne Laufen (17.5)
            let st = match p.activity {
                "walk" | "run" | "hurt" => "idle",
                a => a,
            };
            (st, 1.0)
        }
    };
    // Im Einbaum beim Fischen: aufs Meerfeld (Platz und Sortierung, 17.12)
    let on_boat = |p: &ap::PersonSprite| -> Option<u32> {
        let b = p.boat_tile?;
        (shown_state(p).0 == "fish"
            && atlas
                .get(&format!("person_{}_fish_boat_{}_f0", p.look, p.dir))
                .is_some())
        .then_some(b)
    };
    // Mehrere Einbäume auf einem Meerfeld: feste Plätze nebeneinander statt
    // übereinander (Rangfolge in der festen Reihenfolge der Menschen).
    let mut boat_slot: Vec<(u32, usize)> = Vec::new();
    if people_close {
        let mut boats_on: Vec<(u32, usize)> = Vec::new();
        for (k, p) in persons.iter().enumerate() {
            let at = match on_boat(p) {
                Some(b) => {
                    let n = match boats_on.iter_mut().find(|e| e.0 == b) {
                        Some(e) => {
                            e.1 += 1;
                            e.1 - 1
                        }
                        None => {
                            boats_on.push((b, 1));
                            0
                        }
                    };
                    boat_slot.push((p.id, n));
                    b
                }
                None => p.tile,
            };
            people_at[at as usize].push(k);
        }
        for (tile, look, dir) in ap::person_bodies(&sim.people) {
            bodies_at[tile as usize].push((look, dir));
        }
    }
    // Gärten: Zustand je Feld
    let mut garden_at: Vec<Option<&'static str>> = vec![None; world.tiles.len()];
    for (tile, stage) in ap::garden_tiles(&sim.people, sim.day_of_year()) {
        garden_at[tile as usize] = Some(stage);
    }
    // Lagerplätze: immer sichtbar (sie sind Teil der Landschaft)
    let camps = ap::camp_sprites(&sim.people);
    let mut camps_at: Vec<Vec<usize>> = vec![Vec::new(); world.tiles.len()];
    for (k, c) in camps.iter().enumerate() {
        camps_at[c.tile as usize].push(k);
    }
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
        .map(|id| [0, 1, 2, 3].map(|v| atlas.tile(id, v).or_else(|| atlas.get(id))))
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

            // Wasser über Geländestufen (16.9): Streifen auf der vorderen Wand
            for d in ap::stream_drops(world, i) {
                let (tx, ty) = world.xy(d.to as usize);
                let drop = iso_lift(e, z) - iso_lift(height_of(tx as i32, ty as i32), z);
                let (m, k) = match d.side {
                    "right" => (east + (south - east) * 0.5, -0.5),
                    "left" => (south + (west - south) * 0.5, 0.5),
                    _ => (south, 0.0),
                };
                draw_stream_drop(
                    &mut mesh, atlas, &g, white, m, k, drop, hw, &d, i, light, lit,
                );
            }

            // Boden und Überlagerungen
            let top_rect =
                Rect::from_min_max(Pos2::new(c.x - hw, c.y - hh), Pos2::new(c.x + hw, c.y + hh));
            // Wasser darf animiert sein: `water_<art>_<variante>_f0…f3`
            // (langsam, 4 × 500 ms, nur bei 1 Std/s); sonst das Standbild.
            let animated = ground.is_water().then(|| {
                let frame = if g.animate {
                    ((g.time_s * 1000.0 / 500.0) as u64 % 4) as u32
                } else {
                    0
                };
                let id = ground.asset_id();
                atlas
                    .get(&format!("{id}_{variant}_f{frame}"))
                    .or_else(|| atlas.get(&format!("{id}_0_f{frame}")))
            });
            match animated
                .flatten()
                .or_else(|| atlas.tile(ground.asset_id(), variant))
            {
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
            if let Some(stage) = garden_at[i] {
                match atlas.tile(&format!("garden_{stage}"), variant) {
                    Some(ge) => tex_rect(&mut mesh, top_rect, ge, lit),
                    None => garden_placeholder(&mut mesh, white, c, hw, hh, stage, light),
                }
            }
            if t.fire > 0.0 {
                // Glutsaum (Codex, Gruppe E): Bodenüberlagerung, 4 Bilder à 200 ms
                let glow = atlas.get(&format!("fire_ground_0_f{}", fire_frame(&g, i, 0)));
                match glow {
                    Some(ge) => {
                        tex_rect(&mut mesh, top_rect, ge, with_alpha(lit, 0.6 + 0.4 * t.fire))
                    }
                    None => solid(
                        &mut mesh,
                        [n, east, south, west],
                        with_alpha(Color32::from_rgb(245, 110, 30), 0.35 + 0.4 * t.fire),
                    ),
                }
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

            // Bachläufe und trockene Bachbetten (gestaltung.md 16.8): Codex'
            // Bachstücke, sonst Linien – Wasser blau, trockenes Bett erdfarben.
            if let Some(piece) = ap::stream_piece(world, i) {
                let state = if piece.wet { "wet" } else { "dry" };
                let base = format!("stream_{state}_{}", piece.size);
                if let Some(ce) = atlas.get(&format!("{base}_c")) {
                    tex_rect(&mut mesh, top_rect, ce, lit);
                    for &d in &piece.dirs {
                        if let Some(de) = atlas.get(&format!("{base}_{}", ap::STREAM_DIRS[d])) {
                            tex_rect(&mut mesh, top_rect, de, lit);
                        }
                    }
                } else {
                    let flat = iso_center(x, y, w, h, z);
                    let width = stream_width(piece.flow.max(RILL_FLOW), z);
                    let color = if piece.wet {
                        stream_color(piece.flow, light)
                    } else {
                        let c = night_tint(Color32::from_rgb(186, 170, 134), light);
                        Color32::from_rgba_unmultiplied(c.r(), c.g(), c.b(), 210)
                    };
                    let centers: Vec<Vec2> = piece
                        .dirs
                        .iter()
                        .map(|&d| {
                            let (dx, dy) = DIR_OFFSETS[d];
                            iso_center((x as i32 + dx) as u32, (y as i32 + dy) as u32, w, h, z)
                                - flat
                        })
                        .collect();
                    for dir in centers {
                        let b = c + dir * 0.5;
                        let dd = b - c;
                        let len = dd.length();
                        if len < 0.01 {
                            continue;
                        }
                        let nn = Vec2::new(-dd.y, dd.x) / len * (width * 0.5);
                        solid(&mut mesh, [c + nn, b + nn, b - nn, c - nn], color);
                    }
                }
            }

            // Pflanzen und Tiere, nach Fußpunkt von hinten nach vorne
            enum Thing<'b> {
                Plant(&'b ap::Sprite),
                Animal(&'b ap::AnimalSprite),
                Carcass(&'static str, f32, f32, usize),
                Person(&'b ap::PersonSprite),
                Body(u8, &'static str),
                /// Lager: Feuerstelle (`None`) oder Hütte Nr. k
                Camp(&'b ap::CampSprite, Option<usize>),
            }
            let mut plants = ap::sprites(world, i);
            // Auf Lager- und Gartenfeldern ist freigeräumt: keine Bäume und
            // Büsche, die Hütten oder Beete verdecken (Totholz und Feuer bleiben).
            if !camps_at[i].is_empty() || garden_at[i].is_some() {
                plants.retain(|p| !p.id.starts_with("tree") && p.id != "shrub");
            }
            let mut things: Vec<(f32, Thing)> = plants
                .iter()
                .map(|p| (p.v, Thing::Plant(p)))
                .chain(
                    by_tile[i]
                        .iter()
                        .map(|&k| (animals[k].pos.1, Thing::Animal(&animals[k]))),
                )
                .chain(
                    people_at[i]
                        .iter()
                        // Menschen nach den Pflanzen ihres Feldes: Sie sind die
                        // Hauptsache im Bild.
                        .map(|&k| (persons[k].pos.1 + 2.0, Thing::Person(&persons[k]))),
                )
                .chain(
                    bodies_at[i]
                        .iter()
                        .map(|&(look, dir)| (0.2, Thing::Body(look, dir))),
                )
                .chain(camps_at[i].iter().flat_map(|&k| {
                    let c = &camps[k];
                    std::iter::once((c.hearth_pos.1, Thing::Camp(c, None))).chain(
                        c.hut_pos
                            .iter()
                            .enumerate()
                            .map(move |(h, p)| (p.1, Thing::Camp(c, Some(h)))),
                    )
                }))
                .chain(carcass_at[i].iter().enumerate().map(|(k, &id)| {
                    let (u, v) = [(0.15, 0.3), (-0.35, 0.15), (0.45, 0.0)][k % 3];
                    (v, Thing::Carcass(id, u, v, k))
                }))
                .collect();
            things.sort_by(|a, b| a.0.total_cmp(&b.0));
            for (_, th) in things {
                match th {
                    Thing::Plant(p) => {
                        // Flammen sind animiert (`fire_flame_*_0_f0…f3`), jede
                        // Flamme in eigenem Takt.
                        let name = if p.id.starts_with("fire_flame") {
                            let f = fire_frame(&g, i, p.variant_seed as usize);
                            atlas.first_of(&[format!("{}_0_f{f}", p.id)])
                        } else {
                            atlas.sprite_name(p.id, p.variant_seed % 4, p.state)
                        };
                        let Some(name) = name else {
                            continue;
                        };
                        let foot = c + Vec2::new(p.u * hw, p.v * hh);
                        // Weltmaß (4.2) – Dateiauflösung und Detailstufe
                        // wählt der Atlas.
                        let width = display_width(atlas, &name, plant_width(p.id));
                        if let Some(pk) = atlas.pick(&name, &name, width, scale) {
                            tex_rect(&mut mesh, pk.rect(foot), &pk.entry, lit);
                        }
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
                        let Some(name) = atlas.animal_name(a.id, a.variant, state, a.dir, frame)
                        else {
                            continue;
                        };
                        let mut foot = c + Vec2::new(a.pos.0 * hw, a.pos.1 * hh);
                        if g.animate && a.prev_tile != a.tile {
                            let from = center_of(a.prev_tile as usize)
                                + Vec2::new(a.pos.0 * hw, a.pos.1 * hh);
                            foot = from + (foot - from) * g.day_frac;
                        }
                        // Alle Bilder einer Art messen sich am ersten
                        // Fressbild, damit sie gleich groß erscheinen (15.3).
                        let reference = atlas
                            .animal_name(a.id, a.variant, "graze", a.dir, 0)
                            .unwrap_or_else(|| name.clone());
                        let width = display_width(atlas, &reference, animal_width(a.id));
                        // Aus einem anderen Bildsatz (alt neben neu): eigenes Maß
                        let own = display_width(atlas, &name, animal_width(a.id));
                        let tint = shade(lit, 1.0 - 0.25 * a.weakness);
                        if let Some(pk) =
                            atlas.pick_sized(&name, &reference, width, own, scale * a.scale)
                        {
                            tex_rect(&mut mesh, pk.rect(foot), &pk.entry, tint);
                        }
                    }
                    Thing::Person(p) => {
                        let (state, t) = shown_state(p);
                        let boat = on_boat(p).is_some();
                        let mut foot = if boat {
                            // Feste Plätze auf dem Meerfeld: Mitte, dann vorn
                            // rechts, hinten links, vorn links, hinten rechts …
                            const SLOTS: [(f32, f32); 6] = [
                                (0.0, 0.0),
                                (0.45, 0.4),
                                (-0.45, -0.4),
                                (-0.45, 0.4),
                                (0.45, -0.4),
                                (0.0, 0.7),
                            ];
                            let n = boat_slot
                                .iter()
                                .find(|e| e.0 == p.id)
                                .map(|e| e.1)
                                .unwrap_or(0);
                            let (sx, sy) = SLOTS[n % SLOTS.len()];
                            c + Vec2::new(sx * hw, sy * hh)
                        } else {
                            c + Vec2::new(p.pos.0 * hw, p.pos.1 * hh)
                        };
                        if !boat && t < 1.0 && p.prev_tile != p.tile {
                            let from = center_of(p.prev_tile as usize)
                                + Vec2::new(p.pos.0 * hw, p.pos.1 * hh);
                            foot = from + (foot - from) * t;
                        }
                        let (frames, ms) = person_frames(state);
                        let frame = if g.animate && frames > 1 {
                            ((g.time_s * 1000.0 / ms) as u64 % frames as u64) as u32
                        } else {
                            0
                        };
                        let tint = shade(lit, 1.0 - 0.25 * p.weakness);
                        // Kinder: eigene Bilder `person_child_…` (17.3), sonst
                        // die Erwachsenenbilder kleiner.
                        let child_art = p.child
                            && atlas
                                .get(&format!("person_child_{}_idle_{}_f0", p.look, p.dir))
                                .is_some();
                        let (id, width) = match (p.child, child_art) {
                            (true, true) => ("person_child", PERSON_CHILD_WIDTH),
                            (true, false) => ("person", PERSON_WIDTH * CHILD_SCALE),
                            _ => ("person", PERSON_WIDTH),
                        };
                        let idle = format!("{id}_{}_idle_{}_f0", p.look, p.dir);
                        // Mit Werkzeug eigenes Bild (17.11), sonst das gewöhnliche
                        let with_tool = p
                            .tool
                            .map(|t| format!("{id}_{}_{state}_{t}_{}_f{frame}", p.look, p.dir));
                        let name = with_tool
                            .and_then(|n| atlas.first_of(&[n]))
                            .or_else(|| {
                                atlas.first_of(&[format!(
                                    "{id}_{}_{state}_{}_f{frame}",
                                    p.look, p.dir
                                )])
                            })
                            .or_else(|| atlas.first_of(std::slice::from_ref(&idle)));
                        let measured = |n: &str| {
                            if child_art || !p.child {
                                display_width(atlas, n, width)
                            } else {
                                width
                            }
                        };
                        let picked = name.and_then(|n| {
                            atlas.pick_sized(&n, &idle, measured(&idle), measured(&n), scale)
                        });
                        match picked {
                            Some(pk) => tex_rect(&mut mesh, pk.rect(foot), &pk.entry, tint),
                            None => person_placeholder(
                                &mut mesh,
                                white,
                                foot,
                                if p.child { scale * CHILD_SCALE } else { scale },
                                state,
                                person_color(p.look),
                                light,
                                p.weakness,
                            ),
                        }
                    }
                    Thing::Body(look, dir) => {
                        let foot = c + Vec2::new(0.1 * hw, 0.2 * hh);
                        let name = format!("person_dead_{look}_{dir}_f0");
                        let width = display_width(atlas, &name, 15.0);
                        match atlas.pick(&name, &name, width, scale) {
                            Some(pk) => {
                                tex_rect(&mut mesh, pk.rect(foot), &pk.entry, shade(lit, 0.8))
                            }
                            None => person_placeholder(
                                &mut mesh,
                                white,
                                foot,
                                scale,
                                "dead",
                                person_color(look),
                                light,
                                0.6,
                            ),
                        }
                    }
                    Thing::Camp(cs, part) => {
                        let tint = shade(lit, 0.55 + 0.45 * cs.condition);
                        let (pos, name, table) = match part {
                            None => {
                                let frame = if g.animate {
                                    ((g.time_s * 1000.0 / 200.0) as u64 % 4) as u32
                                } else {
                                    0
                                };
                                let name = if cs.ruin {
                                    "camp_ruin".to_string()
                                } else if cs.fire {
                                    format!("camp_fire_f{frame}")
                                } else {
                                    "camp_hearth".to_string()
                                };
                                (cs.hearth_pos, name, 6.0)
                            }
                            Some(h) => (cs.hut_pos[h], format!("camp_hut_{}", h % 2), 16.0),
                        };
                        let foot = c + Vec2::new(pos.0 * hw, pos.1 * hh);
                        let width = display_width(atlas, &name, table);
                        match atlas.pick(&name, &name, width, scale) {
                            Some(pk) => tex_rect(&mut mesh, pk.rect(foot), &pk.entry, tint),
                            None => {
                                camp_placeholder(&mut mesh, white, foot, scale, cs, part, light)
                            }
                        }
                        // Ohne Hütte: Windschutz neben der Feuerstelle
                        if part.is_none() && cs.huts == 0 && !cs.ruin {
                            let foot = c + Vec2::new(-0.35 * hw, -0.15 * hh);
                            let name = "camp_windbreak";
                            let width = display_width(atlas, name, 12.0);
                            match atlas.pick(name, name, width, scale) {
                                Some(pk) => tex_rect(&mut mesh, pk.rect(foot), &pk.entry, tint),
                                None => windbreak_placeholder(&mut mesh, white, foot, scale, light),
                            }
                        }
                    }
                    Thing::Carcass(id, u, v, k) => {
                        let Some(name) = carcass_name(id, k) else {
                            continue;
                        };
                        let foot = c + Vec2::new(u * hw, v * hh);
                        let width = display_width(atlas, &name, animal_width(id));
                        if let Some(pk) = atlas.pick(&name, &name, width, scale) {
                            tex_rect(&mut mesh, pk.rect(foot), &pk.entry, lit);
                        }
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
    if !people_close {
        draw_person_markers(painter, sim, &persons, origin, z, light, Some(atlas));
    }
    // Inselnamen im Überblick, wenn es mehrere Inseln gibt
    if z <= 2.0 {
        for isl in named_islands(world) {
            let (x, y) = world.xy(isl.center as usize);
            let e = world.tiles[isl.center as usize].elevation;
            let p = origin + iso_center(x, y, w, h, z).to_vec2() - Vec2::new(0.0, iso_lift(e, z));
            island_label(painter, p, &isl.name);
        }
    }
    // Dorfnamen aus der Ferne (bis Zoom 8), über der Dorfmitte
    if z <= 8.0 {
        for v in &sim.people.villages {
            // Über den Lagern selbst: Mitte ihrer Bildschirmorte
            let pts: Vec<Pos2> = v
                .camps
                .iter()
                .filter_map(|id| sim.people.camps.iter().find(|c| c.id == *id))
                .map(|c| {
                    let (x, y) = world.xy(c.tile as usize);
                    let e = world.tiles[c.tile as usize].elevation;
                    origin + iso_center(x, y, w, h, z).to_vec2() - Vec2::new(0.0, iso_lift(e, z))
                })
                .collect();
            if pts.is_empty() {
                continue;
            }
            let cx = pts.iter().map(|p| p.x).sum::<f32>() / pts.len() as f32;
            let cy = pts.iter().map(|p| p.y).sum::<f32>() / pts.len() as f32;
            village_label(painter, Pos2::new(cx, cy - hh * 2.0), &v.name);
        }
    }
    hovered
}

/// Inseln mit Namen auf der Karte: nur, wenn die Welt mehrere hat.
fn named_islands(world: &nig_core::world::World) -> Vec<&nig_core::world::Island> {
    let named: Vec<_> = world.islands.iter().filter(|i| i.tiles >= 40).collect();
    if named.len() > 1 { named } else { Vec::new() }
}

/// Eigene Seekartenschrift nur für Inselnamen; die Oberfläche behält ihre Standardschrift.
pub fn install_island_font(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "island_chart".into(),
        egui::FontData::from_static(include_bytes!("../../../assets/fonts/IMFeENit28P.ttf")).into(),
    );
    fonts.families.insert(
        egui::FontFamily::Name("island_chart".into()),
        vec!["island_chart".into()],
    );
    ctx.set_fonts(fonts);
}

/// Inselname wie auf einer alten Seekarte: kursive Serifenschrift,
/// warmes Elfenbein und schmaler dunkler Rand auf Meer und Bewuchs.
fn island_label(painter: &egui::Painter, p: Pos2, name: &str) {
    let font = egui::FontId::new(22.0, egui::FontFamily::Name("island_chart".into()));
    for d in [
        Vec2::new(1.0, 0.0),
        Vec2::new(-1.0, 0.0),
        Vec2::new(0.0, 1.0),
        Vec2::new(0.0, -1.0),
        Vec2::new(1.0, 1.0),
        Vec2::new(-1.0, 1.0),
        Vec2::new(1.0, -1.0),
        Vec2::new(-1.0, -1.0),
    ] {
        painter.text(
            p + d,
            egui::Align2::CENTER_CENTER,
            name,
            font.clone(),
            Color32::from_rgba_unmultiplied(19, 30, 38, 220),
        );
    }
    painter.text(
        p,
        egui::Align2::CENTER_CENTER,
        name,
        font,
        Color32::from_rgb(244, 228, 184),
    );
}

/// Dorfname als Schrift mit dunklem Rand, gut lesbar auf jedem Untergrund.
pub fn village_label(painter: &egui::Painter, p: Pos2, name: &str) {
    let font = egui::FontId::proportional(15.0);
    for d in [
        Vec2::new(1.0, 1.0),
        Vec2::new(-1.0, 1.0),
        Vec2::new(1.0, -1.0),
        Vec2::new(-1.0, -1.0),
    ] {
        painter.text(
            p + d,
            egui::Align2::CENTER_BOTTOM,
            name,
            font.clone(),
            Color32::from_black_alpha(200),
        );
    }
    painter.text(
        p,
        egui::Align2::CENTER_BOTTOM,
        name,
        font,
        Color32::from_rgb(250, 240, 210),
    );
}

/// Weltmaß eines Bildes: aus dem Manifest (`display_size`, Codex), sonst
/// aus den Tabellen der Gestaltung.
fn display_width(atlas: &Atlas, name: &str, table: f32) -> f32 {
    atlas
        .display_size(name)
        .map(|d| d.x)
        .filter(|w| *w > 0.0)
        .unwrap_or(table)
}

/// Bild 0…3 einer Feueranimation (200 ms je Bild). Bei schneller Zeit steht
/// sie still; der Versatz je Feld und Flamme verhindert gleichen Takt.
fn fire_frame(g: &Gfx<'_>, tile: usize, k: usize) -> u32 {
    let offset = (tile * 7 + k * 3) as u64;
    if g.animate {
        (((g.time_s * 1000.0 / 200.0) as u64 + offset) % 4) as u32
    } else {
        (offset % 4) as u32
    }
}

/// Darstellungsbreite der Pflanzen im Grundmaß (gestaltung.md 4.2).
fn plant_width(id: &str) -> f32 {
    match id {
        "tree_young" | "shrub" => 10.0,
        "tree_mid" => 14.0,
        "tree_old" => 18.0,
        "snag_standing" => 6.0,
        "log_fallen" => 16.0,
        _ => 10.0,
    }
}

/// Darstellungsbreite eines Menschen im Grundmaß (17.3).
const PERSON_WIDTH: f32 = 7.0;
/// Kinder (`person_child`, 17.3): 5 × 10.
const PERSON_CHILD_WIDTH: f32 = 5.0;
/// Ohne eigene Kinderbilder: Erwachsenenbild bzw. Platzhalter verkleinert.
const CHILD_SCALE: f32 = 0.65;

/// Bilder und Bilddauer je Zustand (17.4).
fn person_frames(state: &str) -> (u32, f64) {
    match state {
        "walk" => (4, 200.0),
        "run" => (4, 100.0),
        "gather" | "fish" => (4, 300.0),
        "hurt" | "build" | "hunt" => (4, 250.0),
        "eat" | "drink" => (2, 600.0),
        "idle" => (2, 800.0),
        _ => (1, 1000.0),
    }
}

/// Erkennungsfarben der fünf Aussehen – aus Codex' Menschenbildern
/// (`assets/nature/people/manifest.json`, `recognition_color`).
pub const PERSON_COLORS: [Color32; 5] = [
    Color32::from_rgb(0xA5, 0x4E, 0x3B),
    Color32::from_rgb(0x4D, 0x7B, 0x91),
    Color32::from_rgb(0x7B, 0x56, 0x7E),
    Color32::from_rgb(0xB7, 0x8E, 0x38),
    Color32::from_rgb(0x53, 0x7E, 0x68),
];

pub fn person_color(look: u8) -> Color32 {
    PERSON_COLORS[look as usize % PERSON_COLORS.len()]
}

/// Einfache Figur, solange es keine Bilder gibt: Körper in Erdtönen mit
/// einem Band in der Erkennungsfarbe, darüber der Kopf. Maße im Grundmaß.
#[allow(clippy::too_many_arguments)]
fn person_placeholder(
    mesh: &mut Mesh,
    white: Pos2,
    foot: Pos2,
    scale: f32,
    state: &str,
    color: Color32,
    light: f32,
    weakness: f32,
) {
    let tint = |c: Color32| night_tint(shade(c, 1.0 - 0.25 * weakness), light);
    let cloth = tint(Color32::from_rgb(118, 92, 64));
    let skin = tint(Color32::from_rgb(214, 172, 132));
    let band = tint(color);
    let mut rect = |x0: f32, y0: f32, x1: f32, y1: f32, c: Color32| {
        let a = foot + Vec2::new(x0, y0) * scale;
        let b = foot + Vec2::new(x1, y1) * scale;
        tex_quad(
            mesh,
            [a, Pos2::new(b.x, a.y), b, Pos2::new(a.x, b.y)],
            [white; 4],
            c,
        );
    };
    match state {
        "sleep" | "dead" => {
            // liegend
            rect(-6.0, -3.0, 4.0, 0.0, cloth);
            rect(-1.5, -3.0, 0.5, 0.0, band);
            rect(4.0, -3.0, 6.5, -0.5, skin);
        }
        "gather" | "drink" => {
            // gebückt
            rect(-2.0, -7.0, 2.0, 0.0, cloth);
            rect(-2.0, -5.0, 2.0, -4.0, band);
            rect(1.0, -8.5, 3.5, -6.0, skin);
        }
        _ => {
            rect(-1.8, -12.0, 1.8, 0.0, cloth);
            rect(-1.8, -8.0, 1.8, -6.8, band);
            rect(-1.3, -15.0, 1.3, -12.2, skin);
        }
    }
}

/// Platzhalter für Lagerplätze, bis Codex' Bilder kommen (17.8):
/// Steinring mit Flammen, Hütte als brauner Kegel.
fn camp_placeholder(
    mesh: &mut Mesh,
    white: Pos2,
    foot: Pos2,
    scale: f32,
    cs: &nig_core::appearance::CampSprite,
    part: Option<usize>,
    light: f32,
) {
    let fade = 0.5 + 0.5 * cs.condition;
    let tint = |c: Color32| night_tint(shade(c, fade), light);
    let mut quad = |p: [Vec2; 4], c: Color32| {
        tex_quad(mesh, p.map(|v| foot + v * scale), [white; 4], c);
    };
    match part {
        None => {
            let stone = tint(Color32::from_rgb(120, 116, 108));
            quad(
                [
                    Vec2::new(-3.5, -0.5),
                    Vec2::new(0.0, -2.2),
                    Vec2::new(3.5, -0.5),
                    Vec2::new(0.0, 1.2),
                ],
                stone,
            );
            if cs.fire && !cs.ruin {
                // Flamme leuchtet auch nachts
                let flame = Color32::from_rgb(240, 150, 50);
                quad(
                    [
                        Vec2::new(-1.6, -0.8),
                        Vec2::new(0.0, -6.0),
                        Vec2::new(1.6, -0.8),
                        Vec2::new(0.0, -0.2),
                    ],
                    flame,
                );
            }
            if cs.store {
                let sack = tint(Color32::from_rgb(170, 140, 90));
                quad(
                    [
                        Vec2::new(4.0, -0.5),
                        Vec2::new(4.0, -3.0),
                        Vec2::new(6.5, -3.0),
                        Vec2::new(6.5, -0.5),
                    ],
                    sack,
                );
            }
        }
        Some(_) => {
            let (dark, light_side) = if cs.ruin {
                (
                    tint(Color32::from_rgb(80, 66, 50)),
                    tint(Color32::from_rgb(100, 84, 64)),
                )
            } else {
                (
                    tint(Color32::from_rgb(104, 78, 50)),
                    tint(Color32::from_rgb(140, 108, 70)),
                )
            };
            let top = if cs.ruin { -5.0 } else { -11.0 };
            quad(
                [
                    Vec2::new(-6.0, 0.0),
                    Vec2::new(0.0, top),
                    Vec2::new(0.0, top),
                    Vec2::new(0.0, 2.0),
                ],
                dark,
            );
            quad(
                [
                    Vec2::new(0.0, 2.0),
                    Vec2::new(0.0, top),
                    Vec2::new(0.0, top),
                    Vec2::new(6.0, 0.0),
                ],
                light_side,
            );
            if !cs.ruin {
                let door = tint(Color32::from_rgb(50, 38, 28));
                quad(
                    [
                        Vec2::new(0.6, 1.6),
                        Vec2::new(1.4, -3.5),
                        Vec2::new(2.6, -3.0),
                        Vec2::new(2.6, 1.0),
                    ],
                    door,
                );
            }
        }
    }
}

/// Platzhalter für Gärten (17.9): Beetreihen in Erde, je nach Zustand mit
/// Grün oder reifem Gelb.
fn garden_placeholder(
    mesh: &mut Mesh,
    white: Pos2,
    c: Pos2,
    hw: f32,
    hh: f32,
    stage: &str,
    light: f32,
) {
    let soil = night_tint(Color32::from_rgb(112, 84, 56), light);
    let plant = match stage {
        "sprout" => Some(Color32::from_rgb(140, 178, 80)),
        "growing" => Some(Color32::from_rgb(86, 150, 60)),
        "ripe" => Some(Color32::from_rgb(206, 176, 72)),
        "stubble" => Some(Color32::from_rgb(160, 140, 96)),
        _ => None,
    };
    // Umgegrabene Erde auf fast der ganzen Raute
    let k = 0.82;
    tex_quad(
        mesh,
        [
            c + Vec2::new(0.0, -hh * k),
            c + Vec2::new(hw * k, 0.0),
            c + Vec2::new(0.0, hh * k),
            c + Vec2::new(-hw * k, 0.0),
        ],
        [white; 4],
        night_tint(Color32::from_rgb(132, 100, 66), light),
    );
    // Vier Reihen von links oben nach rechts unten, innerhalb der Raute
    for r in 0..4 {
        let f = -0.6 + r as f32 * 0.4;
        let a = c + Vec2::new((f - 0.25) * hw * 0.5, (f + 0.25) * hh * 0.5 - hh * 0.25);
        let b = a + Vec2::new(hw * 0.5, hh * 0.5);
        let n = Vec2::new(0.0, hh * 0.09);
        tex_quad(mesh, [a - n, b - n, b + n, a + n], [white; 4], soil);
        if let Some(pc) = plant {
            let pc = night_tint(pc, light);
            let n2 = Vec2::new(0.0, hh * 0.05);
            let up = Vec2::new(0.0, -hh * 0.06);
            tex_quad(
                mesh,
                [a - n2 + up, b - n2 + up, b + n2 + up, a + n2 + up],
                [white; 4],
                pc,
            );
        }
    }
}

fn windbreak_placeholder(mesh: &mut Mesh, white: Pos2, foot: Pos2, scale: f32, light: f32) {
    let c = night_tint(Color32::from_rgb(112, 92, 60), light);
    let p = [
        Vec2::new(-5.0, 0.0),
        Vec2::new(-3.0, -5.0),
        Vec2::new(5.0, -3.0),
        Vec2::new(4.0, 1.0),
    ];
    tex_quad(mesh, p.map(|v| foot + v * scale), [white; 4], c);
}

/// Überblick: ein Zeichen je Mensch in seiner Erkennungsfarbe, damit man die
/// wenigen Bewohner auf der Insel findet (17.2). Bleibt in fester
/// Bildschirmgröße.
fn draw_person_markers(
    painter: &egui::Painter,
    sim: &Simulation,
    persons: &[nig_core::appearance::PersonSprite],
    origin: Pos2,
    z: f32,
    light: f32,
    atlas: Option<&Atlas>,
) {
    let world = &sim.world;
    let (w, h) = (world.width, world.height);
    // Codex' Zeichen (weiß, wird eingefärbt), 1 Bildpunkt = 1 Bildschirmpunkt
    let marker = atlas.and_then(|a| a.get("person_marker_0").map(|e| (a, e)));
    let mut mesh = marker.map(|(a, _)| Mesh::with_texture(a.texture.id()));
    for p in persons {
        let (x, y) = world.xy(p.tile as usize);
        let e = world.tiles[p.tile as usize].elevation;
        let c = origin + iso_center(x, y, w, h, z).to_vec2() - Vec2::new(0.0, iso_lift(e, z))
            + Vec2::new(p.pos.0, p.pos.1) * 4.0;
        if let (Some((_, me)), Some(mesh)) = (marker, mesh.as_mut()) {
            let min = c - me.anchor;
            tex_rect(
                mesh,
                Rect::from_min_size(min, me.size),
                me,
                night_tint(person_color(p.look), light),
            );
            continue;
        }
        let r = 4.0;
        let tip = c + Vec2::new(0.0, -2.0);
        let pts = vec![
            tip + Vec2::new(0.0, -2.0 * r),
            tip + Vec2::new(r, -r),
            tip,
            tip + Vec2::new(-r, -r),
        ];
        painter.add(Shape::convex_polygon(
            pts,
            night_tint(person_color(p.look), light),
            Stroke::new(1.0, Color32::from_rgb(245, 240, 225)),
        ));
    }
    if let Some(mesh) = mesh {
        painter.add(Shape::mesh(mesh));
    }
}

/// Größter Zoom: Stufe „Ganz nah“ (gestaltung.md 4.3, ADR 0006).
pub const MAX_ZOOM: f32 = 32.0;

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
            cam.zoom = (cam.zoom * factor).clamp(0.4, MAX_ZOOM);
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
                hovered = draw_iso_gfx(
                    &painter,
                    sim,
                    &animals,
                    g,
                    origin,
                    rect,
                    z,
                    mouse,
                    light,
                    show_animals,
                );
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
                for p in nig_core::appearance::person_sprites(&sim.people, world) {
                    let (x, y) = world.xy(p.tile as usize);
                    let e = world.tiles[p.tile as usize].elevation;
                    let c = origin + iso_center(x, y, w, h, z).to_vec2()
                        - Vec2::new(0.0, iso_lift(e, z))
                        + Vec2::new(p.pos.0 * hw, p.pos.1 * hh);
                    if !rect.contains(c) {
                        continue;
                    }
                    let r = (1.4 * z).max(2.5);
                    painter.circle(
                        c,
                        r,
                        night_tint(person_color(p.look), light),
                        Stroke::new(1.0, Color32::from_rgb(245, 240, 225)),
                    );
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
            for cs in nig_core::appearance::camp_sprites(&sim.people) {
                let (x, y) = world.xy(cs.tile as usize);
                let c = top_left + Vec2::new(x as f32 + 0.5, y as f32 + 0.5) * cell;
                if !rect.contains(c) {
                    continue;
                }
                let r = (cell * 0.32).max(2.5);
                let col = if cs.ruin {
                    Color32::from_rgb(120, 100, 80)
                } else {
                    Color32::from_rgb(150, 100, 50)
                };
                painter.rect_filled(
                    Rect::from_center_size(c, Vec2::splat(r * 2.0)),
                    0.0,
                    night_tint(col, light),
                );
                if cs.huts > 0 && !cs.ruin {
                    painter.rect_stroke(
                        Rect::from_center_size(c, Vec2::splat(r * 2.0)),
                        0.0,
                        Stroke::new(1.0, Color32::from_rgb(245, 240, 225)),
                        egui::StrokeKind::Outside,
                    );
                }
            }
            for p in nig_core::appearance::person_sprites(&sim.people, world) {
                let (x, y) = world.xy(p.tile as usize);
                let c = top_left
                    + Vec2::new(x as f32 + 0.5, y as f32 + 0.5) * cell
                    + Vec2::new(p.pos.0, p.pos.1) * cell * 0.45;
                if !rect.contains(c) {
                    continue;
                }
                painter.circle(
                    c,
                    (cell * 0.3).max(2.5),
                    night_tint(person_color(p.look), light),
                    Stroke::new(1.0, Color32::from_rgb(245, 240, 225)),
                );
            }
            for v in &sim.people.villages {
                let (x, y) = world.xy(v.center as usize);
                let c = top_left + Vec2::new(x as f32 + 0.5, y as f32 - 1.0) * cell;
                if rect.contains(c) {
                    village_label(&painter, c, &v.name);
                }
            }
            // Inselnamen, wenn es mehrere Inseln gibt
            for isl in named_islands(world) {
                let (x, y) = world.xy(isl.center as usize);
                let c = top_left + Vec2::new(x as f32 + 0.5, y as f32 + 0.5) * cell;
                if rect.contains(c) {
                    island_label(&painter, c, &isl.name);
                }
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
