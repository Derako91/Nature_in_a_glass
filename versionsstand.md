# Versionsstand – Nature_in_a_glass

> **Stand:** 16.11.2026 · **Version:** 0.8.3 (Hütten in richtiger Form) · **Branch:** `claude/vibrant-meitner-8r0hoc` → `main`

## Auf einen Blick

| Frage | Antwort |
| --- | --- |
| **Zuletzt erledigt** | Codex: gestreckte Hütten korrigiert. Proportionaler Export, Weltbreite 20, Höhe aus Quellform und Fußpunkt unten mittig. Neuer Draft auf `codex/camp-hut-proportions`, Basis `7b20283`; Vergleich in `art/nature/camp-hut-proportions/`. |
| **Getestet** | Aktuelle Hütten-Korrektur: 89 Rust-Tests, Formatprüfung, strenges Clippy und Release-Bau bestanden. Proportions-/Hashvergleich, wiederholter Export und tatsächliche Spielbilder Zoom 8/32 geprüft. |
| **Offen für Codex** | Nach Abnahme des Lager-Drafts übrige Menschenzustände stilistisch angleichen. Dennis hat die schrittweise Umstellung aller Spielbilder beauftragt; Reihenfolge in Gestaltung 18. Ruine noch nicht an einem natürlich verlassenen Lager geprüft. |
| **Offen für Dennis** | – (PR #9 und #10 übernommen; Feuer im Spiel seit 0.8.1, Fuchs B Fernfassung liegt als Entwurf in art/) |
| **Nächster Schritt (Roadmap)** | Vom Lager zum Dorf: Fischfang, später Anbau, mehrere Lager einer Gruppe, Werkzeuge. |

## Zusammenarbeit Claude / Codex

- **Claude:** Simulation, technische Schnittstellen, Tests und Integration.
- **Codex:** Assets, Sprites, Texturen und visuelle Darstellung gemäß `docs/gestaltung.md`.
- `docs/gestaltung.md` bleibt verbindliche Gestaltungsspezifikation, **nicht** das laufende Statusprotokoll.
- **Nach Arbeitsende:** diese Datei mit erledigter Änderung, offenem Problem, Branch/Commit, Testbefehl und Ergebnis aktualisieren. Bei Unklarheit keine Erfolgsmeldung erfinden; Status `ungeprüft` setzen.

## Änderungsprotokoll

| Datum | Änderung | Code-Test |
| --- | --- | --- |
| 16.11.2026 | Codex: feste 120×232-Skalierung entfernt; beide Hütten proportional neu exportiert, Leinwand/Manifest angepasst. Die übrigen 47 Motive bleiben bytegleich. | Quell-/Exportproportionen und wiederholter Export geprüft; tatsächlicher Vorher-/Nachher-Spielvergleich bei Zoom 8/32, ohne `--bilder`. Visuelle Abnahme durch Claude noch offen. |
| 16.11.2026 | Codex: Lager-Draft mit neun Einzelbildern, 40 `_build`-Phasen, Manifesten und Prüfpaket; Übergabe an Claude: Bewuchs auf Lagerfeldern verdeckt teilweise Hütten. Stilumstellung aller Bildgruppen in Gestaltung 18 festgehalten. | 89 Rust-Tests, fmt, strenges Clippy und Release-Bau bestanden; Dateiprüfung aller 49 PNGs; eingebettete Bilder ohne `--bilder` bei Zoom 4/8/32, Windschutz, Holztragen und kalter Feuerstelle angesehen. Keine Änderungen am Kern/Renderer, visuelle Freigabe offen. |
| 16.11.2026 | v0.8.1 Flammen und Glutsaum aus PR #10 eingebunden | Spielbild Seed 7 Zoom 8 |
| 16.11.2026 | v0.8.0 Lagerplätze (Claude) | fmt, clippy, alle Tests grün; Spielbild Zoom 8/2 angesehen |
| 29.10.2026 | Erste gemeinsame, lesbare Übergabe aus den vorhandenen Projektberichten erstellt | Nicht ausgeführt |
