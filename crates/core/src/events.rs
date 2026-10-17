//! Weltchronik: Ereignisse mit Zeitpunkt, Ort, Ursachen und Wirkung.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum EventKind {
    WorldCreated,
    HeavyRain,
    DroughtStart,
    DroughtEnd,
    SpringDry,
    SpringFlowing,
    Intervention,
    YearSummary,
    // seit v0.2
    FireStart,
    FireEnd,
    ForestDieback,
    LastTree,
    TreesReturn,
    Germination,
    SeedFailed,
    // seit v0.3
    GroundwaterLow,
    GroundwaterRecovered,
}

impl EventKind {
    pub fn label(self) -> &'static str {
        match self {
            EventKind::WorldCreated => "Welt",
            EventKind::HeavyRain => "Starkregen",
            EventKind::DroughtStart => "Dürre",
            EventKind::DroughtEnd => "Dürre-Ende",
            EventKind::SpringDry => "Quelle",
            EventKind::SpringFlowing => "Quelle",
            EventKind::Intervention => "Eingriff",
            EventKind::YearSummary => "Jahr",
            EventKind::FireStart => "Feuer",
            EventKind::FireEnd => "Feuer",
            EventKind::ForestDieback => "Waldsterben",
            EventKind::LastTree => "Wald",
            EventKind::TreesReturn => "Wald",
            EventKind::Germination => "Keimung",
            EventKind::SeedFailed => "Samen",
            EventKind::GroundwaterLow => "Grundwasser",
            EventKind::GroundwaterRecovered => "Grundwasser",
        }
    }

    /// Ereignisse, bei denen der Beobachter auf Wunsch anhält.
    pub fn is_important(self) -> bool {
        matches!(
            self,
            EventKind::DroughtStart
                | EventKind::SpringDry
                | EventKind::FireStart
                | EventKind::ForestDieback
                | EventKind::LastTree
                | EventKind::TreesReturn
                | EventKind::Germination
                | EventKind::GroundwaterLow
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Event {
    pub id: u64,
    pub time_hours: u64,
    pub kind: EventKind,
    pub location: Option<(u32, u32)>,
    pub message: String,
    /// Was dazu geführt hat (Prinzip 2: Ereignisse brauchen Ursachen).
    pub causes: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Journal {
    pub events: Vec<Event>,
    next_id: u64,
}

/// Obergrenze, damit sehr lange Läufe nicht unbegrenzt Speicher belegen.
/// Jahresbilanzen und Eingriffe bleiben immer erhalten.
const MAX_EVENTS: usize = 20_000;

impl Journal {
    pub fn push(
        &mut self,
        time_hours: u64,
        kind: EventKind,
        location: Option<(u32, u32)>,
        message: impl Into<String>,
        causes: Vec<String>,
    ) {
        self.events.push(Event {
            id: self.next_id,
            time_hours,
            kind,
            location,
            message: message.into(),
            causes,
        });
        self.next_id += 1;
        if self.events.len() > MAX_EVENTS {
            let keep = |e: &Event| {
                matches!(
                    e.kind,
                    EventKind::YearSummary | EventKind::Intervention | EventKind::WorldCreated
                )
            };
            if let Some(pos) = self.events.iter().position(|e| !keep(e)) {
                self.events.remove(pos);
            }
        }
    }
}
