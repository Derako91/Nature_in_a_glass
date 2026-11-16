# Versionsstand – Nature_in_a_glass

> **Stand:** 16.11.2026 · **Version:** 0.8.0 (Lagerplätze) · **Branch:** `claude/vibrant-meitner-8r0hoc` → `main`

## Auf einen Blick

| Frage | Antwort |
| --- | --- |
| **Zuletzt erledigt** | v0.8.0: Menschen legen selbst Lagerplätze an, bauen Hütten mit Holz aus dem Wald, legen Vorräte an; Lagerfeuer können Brände auslösen; verlassene Lager verfallen. ADR 0009. |
| **Getestet** | `cargo fmt`, `cargo clippy -D warnings`, `cargo test --release --workspace` grün; vier neue Tests in `crates/core/tests/people.rs` (Lager an Wasser + Hütten, Holzschlag lichtet Wald, Verfall, Speichern/Laden). Bildschirmfotos Seed 2024, Zoom 8 und 2: Lager mit zwei Hütten und Feuer sichtbar (Platzhalter). |
| **Offen für Codex** | Bilder für Lagerplätze und Zustand `_build` (gestaltung.md 16.3 Punkt 0a, Abschnitt 17.8). |
| **Offen für Dennis** | Auswahl zu Codex' Entwürfen: Fuchs B Fernansicht (PR #9), Feuerpaket (PR #10) – beide Entwürfe, noch nicht im Spiel. |
| **Nächster Schritt (Roadmap)** | Vom Lager zum Dorf: Fischfang, später Anbau, mehrere Lager einer Gruppe, Werkzeuge. |

## Zusammenarbeit Claude / Codex

- **Claude:** Simulation, technische Schnittstellen, Tests und Integration.
- **Codex:** Assets, Sprites, Texturen und visuelle Darstellung gemäß `docs/gestaltung.md`.
- `docs/gestaltung.md` bleibt verbindliche Gestaltungsspezifikation, **nicht** das laufende Statusprotokoll.
- **Nach Arbeitsende:** diese Datei mit erledigter Änderung, offenem Problem, Branch/Commit, Testbefehl und Ergebnis aktualisieren. Bei Unklarheit keine Erfolgsmeldung erfinden; Status `ungeprüft` setzen.

## Änderungsprotokoll

| Datum | Änderung | Code-Test |
| --- | --- | --- |
| 16.11.2026 | Codex: Cloud-Entwicklungsumgebung geprüft; erster Lager-Stilentwurf und Spielbild unter `art/nature/camp-review/`. Noch keine neuen Lagerbilder eingebaut; nächster Schritt: Einzelbilder, Maßstab und Spielvergleich. Branch `codex/camp-review`, Ausgangscommit `a289aa3`. | 89 Rust-Tests, 15 Python-Tests, fmt, Clippy und Release-Bau bestanden; 100 Jahre mit Speichern/Weiterladen und tatsächlicher Spielstart geprüft. ComfyUI nicht eingerichtet; Entwurfsintegration und Animationen ungeprüft. |
| 16.11.2026 | v0.8.0 Lagerplätze (Claude) | fmt, clippy, alle Tests grün; Spielbild Zoom 8/2 angesehen |
| 29.10.2026 | Erste gemeinsame, lesbare Übergabe aus den vorhandenen Projektberichten erstellt | Nicht ausgeführt |
