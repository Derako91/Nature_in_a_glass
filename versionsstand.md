# Versionsstand – Nature_in_a_glass

> **Stand:** 16.11.2026 · **Version:** Basis 0.8.9 · **Branch:** `codex/stream-beds-style-b` (Basis `2f0e187`)

## Auf einen Blick

| Frage | Antwort |
| --- | --- |
| **Zuletzt erledigt** | Wasser aus PR #20 seit v0.8.9 übernommen. Bach-Draft: alle 54 nassen/trockenen Mittel-/Halbstücke in drei Stärken mit Kies-/Erdmaterial und gemeinsamen Endbreiten nach Gestaltung 16.8. |
| **Getestet** | 94 Rust-/15 Python-Tests, fmt, strenges Clippy und Release-Bau bestanden. 24 Gegenprofile, 768 tatsächlich zusammengesetzte Knotenkombinationen, unveränderte 1279 Eltern-PNGs sowie drei bytegleiche Exportwege geprüft. Vollatlas-/Rust-/Spielprüfung im Bach-Prüfpaket `art/nature/stream-beds-style-b/`. |
| **Offen für Codex** | Visuelle Abnahme des Bachpakets durch Claude. Im vorgegebenen Fall verdecken höhere Vordergrundplatten Teile tieferer Betten; Verbindungen über Höhenstufen benötigen eigene Wand-/Gefälleschnittstelle. Auf gleicher Höhe passen die Endprofile; keine Änderungen an Kern/Renderer. |
| **Offen für Dennis** | – (neue Bildpakete zur visuellen Abnahme durch Claude) |
| **Nächster Schritt (Roadmap)** | Vom Lager zum Dorf: Fischfang, später Anbau, mehrere Lager einer Gruppe, Werkzeuge. |

## Zusammenarbeit Claude / Codex

- **Claude:** Simulation, technische Schnittstellen, Tests und Integration.
- **Codex:** Assets, Sprites, Texturen und visuelle Darstellung gemäß `docs/gestaltung.md`.
- `docs/gestaltung.md` bleibt verbindliche Gestaltungsspezifikation, **nicht** das laufende Statusprotokoll.
- **Nach Arbeitsende:** diese Datei mit erledigter Änderung, offenem Problem, Branch/Commit, Testbefehl und Ergebnis aktualisieren. Bei Unklarheit keine Erfolgsmeldung erfinden; Status `ungeprüft` setzen.

## Änderungsprotokoll

| Datum | Änderung | Code-Test |
| --- | --- | --- |
| 16.11.2026 | Codex: 54 Bach-/Bachbettstücke nach 16.8, Branch `codex/stream-beds-style-b`, Basis `2f0e187`. | 94 Rust-/15 Python-Tests, fmt, Clippy, Release-Bau; Gegenprofile und alle 768 Knotenkonfigurationen geprüft; Spielvergleich Seed 2024 +12 Jahre +240 Tage, 19/33; Höhenstufen-Verdeckung dokumentiert. |
| 16.11.2026 | Codex: Meer-/Flachwasser-/Teichanimation mit vier Varianten, 4×500 ms, Branch `codex/water-style-b`, Basis `0d959fe`. | 93 Rust-/15 Python-Tests, fmt, Clippy, Release-Bau; Vollatlasgrenze und Wasser-/Spielvergleich geprüft. Visuelle Abnahme offen. |
| 16.11.2026 | Codex: vollständiges Tierpaket mit eigenen Fernzyklen, Spuren und Kadavern; Branch `codex/animals-style-b`, Basis `7d9dcc5`. | Export-/Referenzmaß-/Phasenprüfung und Atlas-/Spielvergleich im Tier-Prüfpaket; visuelle Abnahme offen. |
| 16.11.2026 | Codex: Korrektur der bei Zoom 4/8 unlesbaren Landschaft; 32 breite Fernkronen, Nahbilder unverändert. Branch `codex/landscape-far-crowns`, Basis `d6fc0bf`. | Atlas-Auswahl-/Maßstabsregression sowie tatsächlicher Vorher-/Nachher-Vergleich im Fernkronen-Prüfpaket; erneute Abnahme offen. |
| 16.11.2026 | Codex: Landschaft nach Gestaltung 18, Schritt 3; Branch `codex/landscape-style-b`, Basis `8043787` (v0.8.5). | Technische Export-/Maßstabsprüfung und Spielvergleich siehe Landschaft-Prüfpaket; visuelle Abnahme offen. |
| 16.11.2026 | Codex: Erwachsenenmaß 90 %, kompakte Tätigkeiten, größere Hütteneingänge. Branch `codex/people-scale-and-hut-doors`, Basis `0f81511`. | Unveränderte Kinder/Anker/Farben und wiederholbare Exporte geprüft; Spielvergleich Seed 2024 Zoom 8/32 im Prüfpaket. |
| 16.11.2026 | Codex: übrige Menschenbilder vollständig auf Stil B umgestellt; 230 Erwachsenen-, 150 Kinder- und zehn Totenbilder. Freigegebenes `_build` pixelgleich auf breiterer Leinwand, Marker unverändert. Branch `codex/people-style-b`, Basis `73ccfd6`. | 89 Rust-/15 Python-Tests, fmt, Clippy, Release-Bau; technische Vollständigkeit, Exportwiederholung und Spielvergleich. Visuelle Schleifen-/Detailabnahme offen. |
| 16.11.2026 | Codex: Feuerstelle 6→9 im Weltmaß, proportional und mit festen Fußpunkten; nächster Auftrag: übrige Menschenbilder im neuen Stil. | 89 Rust-Tests, fmt, Clippy und Release-Bau bestanden; tatsächlicher Vergleich Zoom 8/32 geprüft. |
| 16.11.2026 | Codex: feste 120×232-Skalierung entfernt; beide Hütten proportional neu exportiert, Leinwand/Manifest angepasst. Die übrigen 47 Motive bleiben bytegleich. | Quell-/Exportproportionen und wiederholter Export geprüft; tatsächlicher Vorher-/Nachher-Spielvergleich bei Zoom 8/32, ohne `--bilder`. Visuelle Abnahme durch Claude noch offen. |
| 16.11.2026 | Codex: Lager-Draft mit neun Einzelbildern, 40 `_build`-Phasen, Manifesten und Prüfpaket; Übergabe an Claude: Bewuchs auf Lagerfeldern verdeckt teilweise Hütten. Stilumstellung aller Bildgruppen in Gestaltung 18 festgehalten. | 89 Rust-Tests, fmt, strenges Clippy und Release-Bau bestanden; Dateiprüfung aller 49 PNGs; eingebettete Bilder ohne `--bilder` bei Zoom 4/8/32, Windschutz, Holztragen und kalter Feuerstelle angesehen. Keine Änderungen am Kern/Renderer, visuelle Freigabe offen. |
| 16.11.2026 | v0.8.1 Flammen und Glutsaum aus PR #10 eingebunden | Spielbild Seed 7 Zoom 8 |
| 16.11.2026 | v0.8.0 Lagerplätze (Claude) | fmt, clippy, alle Tests grün; Spielbild Zoom 8/2 angesehen |
| 29.10.2026 | Erste gemeinsame, lesbare Übergabe aus den vorhandenen Projektberichten erstellt | Nicht ausgeführt |
