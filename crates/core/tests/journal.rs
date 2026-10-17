//! Chronik in sehr langen Läufen.

use nig_core::events::{EventKind, Journal, MAX_EVENTS, MAX_YEAR_SUMMARIES};

fn push(j: &mut Journal, kind: EventKind, n: u64) {
    j.push(n, kind, None, format!("{kind:?} {n}"), vec![]);
}

/// Bis 0.2.4 füllten nach 20.000 Jahren die Jahresbilanzen den ganzen Platz;
/// danach wurde jedes neue Ereignis sofort wieder gelöscht und die Chronik
/// blieb stehen.
#[test]
fn new_events_still_appear_after_many_millennia() {
    let mut j = Journal::default();
    push(&mut j, EventKind::WorldCreated, 0);
    push(&mut j, EventKind::Intervention, 1);
    for year in 0..30_000 {
        push(&mut j, EventKind::YearSummary, 10 + year);
    }
    push(&mut j, EventKind::FireStart, 99_999);

    let last = j.events.last().unwrap();
    assert_eq!(last.kind, EventKind::FireStart, "neues Ereignis fehlt");
    let summaries = j
        .events
        .iter()
        .filter(|e| e.kind == EventKind::YearSummary)
        .count();
    assert_eq!(summaries, MAX_YEAR_SUMMARIES);
    // Die jüngsten Bilanzen bleiben, die ältesten fallen heraus.
    assert!(j.events.iter().any(|e| e.time_hours == 10 + 29_999));
    assert!(!j.events.iter().any(|e| e.time_hours == 10));
    // Entstehung und Eingriffe bleiben immer.
    assert_eq!(j.events[0].kind, EventKind::WorldCreated);
    assert_eq!(j.events[1].kind, EventKind::Intervention);
}

#[test]
fn ordinary_events_are_capped_oldest_first() {
    let mut j = Journal::default();
    for n in 0..(MAX_EVENTS as u64 + 500) {
        push(&mut j, EventKind::HeavyRain, n);
    }
    assert_eq!(j.events.len(), MAX_EVENTS);
    assert_eq!(j.events[0].time_hours, 500);
}

/// Übervolle Chronik aus einem alten Spielstand wird auf einmal gekürzt.
#[test]
fn trim_repairs_overfull_journal() {
    let mut j = Journal::default();
    for n in 0..(MAX_YEAR_SUMMARIES as u64 * 3) {
        j.events.push(nig_core::events::Event {
            id: n,
            time_hours: n,
            kind: EventKind::YearSummary,
            location: None,
            message: String::new(),
            causes: vec![],
        });
    }
    j.trim();
    assert_eq!(j.events.len(), MAX_YEAR_SUMMARIES);
    assert_eq!(j.events[0].time_hours, MAX_YEAR_SUMMARIES as u64 * 2);
}
