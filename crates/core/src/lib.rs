//! Nature in a Glass – Simulationskern.
//!
//! Dieser Kern kennt keine Grafik, kein Netzwerk und keine KI. Er kann ohne
//! Fenster laufen (Tests, Kommandozeile, Server) und liefert bei gleichem Seed
//! und gleichen Eingriffen immer exakt dieselbe Welt.

pub mod appearance;
pub mod climate;
pub mod erosion;
pub mod events;
pub mod fire;
pub mod hydro;
pub mod lakes;
pub mod math;
pub mod rng;
pub mod sim;
pub mod vegetation;
pub mod world;

pub use events::{Event, EventKind};
pub use sim::{Intervention, SeedKind, Simulation, Snapshot, YearStats};
pub use world::{Terrain, Tile, World};
