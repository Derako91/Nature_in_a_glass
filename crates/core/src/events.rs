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

/// Obergrenzen, damit sehr lange Läufe nicht unbegrenzt Speicher belegen.
/// Ereignisse und Jahresbilanzen haben getrennte Grenzen: Sonst füllen die
/// Jahresbilanzen nach 20.000 Jahren den ganzen Platz, und jedes neue
/// Ereignis würde sofort wieder gelöscht. Eingriffe und die Entstehung der
/// Insel bleiben immer erhalten. Es fallen jeweils die ältesten heraus.
pub const MAX_EVENTS: usize = 20_000;
pub const MAX_YEAR_SUMMARIES: usize = 5_000;

/// In welche Grenze ein Eintrag fällt; `None` = bleibt immer.
fn limit_class(kind: EventKind) -> Option<bool> {
    match kind {
        EventKind::Intervention | EventKind::WorldCreated => None,
        EventKind::YearSummary => Some(true),
        _ => Some(false),
    }
}

impl Journal {
    /// Hält beide Obergrenzen ein, indem die ältesten Einträge der jeweiligen
    /// Art entfallen. Bringt auch übervolle Chroniken aus alten Spielständen
    /// auf einmal in Ordnung.
    pub fn trim(&mut self) {
        let (mut summaries, mut regular) = (0usize, 0usize);
        for e in &self.events {
            match limit_class(e.kind) {
                Some(true) => summaries += 1,
                Some(false) => regular += 1,
                None => {}
            }
        }
        let mut drop_summaries = summaries.saturating_sub(MAX_YEAR_SUMMARIES);
        let mut drop_regular = regular.saturating_sub(MAX_EVENTS);
        if drop_summaries == 0 && drop_regular == 0 {
            return;
        }
        self.events.retain(|e| match limit_class(e.kind) {
            Some(true) if drop_summaries > 0 => {
                drop_summaries -= 1;
                false
            }
            Some(false) if drop_regular > 0 => {
                drop_regular -= 1;
                false
            }
            _ => true,
        });
    }

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
        if self.events.len() > MAX_EVENTS.min(MAX_YEAR_SUMMARIES) {
            self.trim();
        }
    }
}
