# Versionsstand – Nature_in_a_glass

> **Stand:** 16.11.2026 · **Version:** Basis 0.10.1 · **Branch:** `codex/garden-style-b` (Basis `cc542e8`)

## Auf einen Blick

| Frage | Antwort |
| --- | --- |
| **Zuletzt erledigt** | Wasser/Bäche übernommen (v0.8.9/v0.9.1), Gärten seit v0.10.0 und Fischfangbilder seit v0.10.1. Neuer Codex-Draft: 20 Gartenbilder nach 17.9, fünf Zustände mit je vier Varianten und eigenem Bodenmanifest. |
| **Getestet** | 100 Rust-/15 Python-Tests, fmt, strenges Clippy und Release-Bau bestanden. Alle 20 Gartenbilder vollständig, gemeinsame Beetmaske/Anker, außerhalb der Raute transparent; alle 1373 bestehenden PNGs unverändert. Garten- und vollständiger Landschaftsexport bytegleich wiederholt. Spielvergleich siehe `art/nature/garden-style-b/README.md`. |
| **Offen für Codex** | Visuelle Abnahme des Gartenpakets durch Claude. Fischfang aus PR #22 ist in der Basis enthalten. Wasserfall-Schnittstelle ist inzwischen seit v0.10.2 auf main; nächster Codex-Auftrag nach den Gärten: stream_drop-Bilder. |
| **Offen für Dennis** | – (neue Bildpakete zur visuellen Abnahme durch Claude) |
| **Nächster Schritt (Roadmap)** | Nach Garten-Abgabe: Wasser an Geländestufen nach 16.9; Seed 2024, +12 Jahre +60 Tage, Feld 19/33, Zoom 16. |

## Zusammenarbeit Claude / Codex

- **Claude:** Simulation, technische Schnittstellen, Tests und Integration.
- **Codex:** Assets, Sprites, Texturen und visuelle Darstellung gemäß `docs/gestaltung.md`.
- `docs/gestaltung.md` bleibt verbindliche Gestaltungsspezifikation, **nicht** das laufende Statusprotokoll.
- **Nach Arbeitsende:** diese Datei mit erledigter Änderung, offenem Problem, Branch/Commit, Testbefehl und Ergebnis aktualisieren. Bei Unklarheit keine Erfolgsmeldung erfinden; Status `ungeprüft` setzen.

## Änderungsprotokoll

| Datum | Änderung | Code-Test |
| --- | --- | --- |
| 16.11.2026 | Codex: 20 Gartenbilder nach Gestaltung 17.9, Branch `codex/garden-style-b`, Basis `cc542e8`. | 100 Rust-/15 Python-Tests, fmt, Clippy, Release-Bau; Alpha-/Manifest-/Bestandsprüfung und zwei wiederholbare Exportwege. Sommer-/Ernteprüfung Seed 300, Feld 25/20, Zoom 8/32 im Prüfpaket; visuelle Abnahme offen. |
| 16.11.2026 | Codex: 40 Fischfangbilder und Vorschlag Wandwasserformat 16.9, Branch `codex/fishing-people-style-b`, Basis `0edb584`. | 97 Rust-/15 Python-Tests, fmt, Clippy, Release-Bau; eingebettete Ansicht und deklarierte Bildaustauschprobe Zoom 8/32. Export-/Maß-/Bestandsprüfung. Technische Zustandsauswahl offen bei Claude. Prüfpaket dokumentiert Darstellungsprobe getrennt von echter Fischfangaktivität. |
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
| 16.11.2026 | v0.11.0 Lager teilen sich (Claude) | alle Tests grün; Langlauf 8 Inseln × 130 Jahre: 6 überleben |
| 16.11.2026 | v0.10.0 Gärten, Holz von weiter her (Claude) | alle Tests grün; Langlauf 8 Inseln × 120 Jahre: 6 überleben |
| 16.11.2026 | v0.9.0 Fischfang, Fehler beim Verdursten an trockenen Teichen behoben (Claude) | alle Tests grün; Langläufe 8 Inseln × 120 Jahre |
| 16.11.2026 | v0.8.1 Flammen und Glutsaum aus PR #10 eingebunden | Spielbild Seed 7 Zoom 8 |
| 16.11.2026 | v0.8.0 Lagerplätze (Claude) | fmt, clippy, alle Tests grün; Spielbild Zoom 8/2 angesehen |
| 29.10.2026 | Erste gemeinsame, lesbare Übergabe aus den vorhandenen Projektberichten erstellt | Nicht ausgeführt |
