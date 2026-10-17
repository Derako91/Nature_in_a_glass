# Nature in a Glass

**Eine Welt erschaffen. Den ersten Impuls geben. Beobachten, was entsteht.**

*Nature in a Glass* ist ein geplantes, langfristiges Open-Source-Projekt: eine eigenständige, beobachtbare Welt- und Gesellschaftssimulation ohne Siegbedingung, Quests oder direkte Steuerung ihrer Bewohner. Der Spieler definiert die Ausgangslage und kann die **Umwelt** beeinflussen – die Bewohner handeln eigenständig und tragen die Konsequenzen. Tonal ein Cozy-Beobachtungsspiel, dessen simulierte Welt durchaus harte Folgen haben kann.

> **Status:** Version 0.1 – *Die Insel lebt ohne Menschen.* Eine Insel mit Wetter, Jahreszeiten, Wasserkreislauf, Bächen, Seen, Gras und Wald läuft als Desktop-Programm (isometrisch und als 2D-Karte) und im Zeitraffer über Jahrhunderte. Noch keine Bewohner.

**Loslegen:** fertige Programme für Windows, macOS und Linux unter [Releases](../../releases). Anleitung: [docs/STARTEN.md](docs/STARTEN.md).

## Vision

- Eine kleine, prozedural generierte Insel lebt auch ohne Menschen: Wasser, Wetter, Vegetation, Tiere und später geologisch plausible Ereignisse.
- Eine Handvoll individueller Bewohner hat Bedürfnisse, Persönlichkeit, Wahrnehmung, Gedächtnis und begrenztes Wissen.
- Handlungen, Beziehungen, Arbeitsteilung, Entdeckungen und – später – Siedlungen entstehen aus Regeln, Anreizen, Versuchen und Erfahrungen, nicht aus einem Drehbuch.
- Naturereignisse haben Ursachen, ihre Folgen bleiben. Wälder können nachwachsen; Tote kehren nicht zurück; Ruinen bleiben Spuren der Vergangenheit.
- Wissen ist verteilt: Es kann geheim bleiben, gelehrt, dokumentiert, missverstanden, wiederentdeckt und verloren werden. Auch Schriften und Sprachen haben eine Geschichte.
- Der Spieler beobachtet und verändert auf Wunsch **Bedingungen** (z. B. die Wasserzufuhr einer Quelle), nicht die Gedanken und Bewegungen Einzelner.
- Die Simulation muss ohne LLM laufen; lokale KI ist ein optionaler Baustein für ausgewählte Gespräche, Reflexionen und Erfindungshypothesen.

## Unterschied zu AI Town

AI Town bleibt ein **separates Testlabor** für Agentendialog, Erinnerungen und soziale Bewertung. Nature in a Glass ist keine Erweiterung oder Fork von AI Town. Übernommen werden erst erprobte Ideen und, sofern Lizenz und technische Eignung es erlauben, separat geprüfte Komponenten.

## Mögliche Startszenarien

1. **Schiffbruch:** Fünf Erwachsene mit Sprache und vorhandenem Alltags- und Technikwissen stranden auf einer Insel.
2. **Neuanfang:** Fünf Erwachsene beginnen mit grundlegenden Bedürfnissen, Lernfähigkeit und minimalem technischem Vorwissen.
3. **Ursprung (Fernziel):** Eine Welt beginnt vor der menschlichen Gesellschaft; langfristig kann evolutionäre Entwicklung simuliert werden.

Diese Szenarien sind Anfangszustände derselben Simulation, nicht drei separate Engines.

## Erste spielbare Version (MVP)

Eine deterministisch testbare kleine Insel (z. B. 64 × 64 Rasterfelder) mit gespeichertem Weltzustand, Zeit, Wasser, Bodenfeuchte, Pflanzenwachstum und einer Beobachtungsoberfläche. Danach wird die Natur ausgebaut (Buschland, Feuer, Erosion, Tiere), bevor in v0.6 fünf Bewohner mit Hunger, Durst, Müdigkeit und einfachen Aktionen hinzukommen – siehe [Roadmap](docs/ROADMAP.md). **Kein KI-Modell und keine vollständige Zivilisation im ersten Meilenstein.**

## Dokumente

- [Spielprinzipien](docs/PRINCIPLES.md)
- [Technische Architektur](docs/ARCHITECTURE.md)
- [Roadmap](docs/ROADMAP.md)
- [Starten und Bedienung](docs/STARTEN.md)
- [Entscheidung: Simulation unabhängig von KI](decisions/0001-independent-simulation.md)
- [Entscheidung: Rust, egui, Zeit in mehreren Auflösungen](decisions/0002-runtime-and-rendering.md)

## Aufbau des Codes

| Ordner | Inhalt |
| --- | --- |
| `crates/core` | Die Simulation (Insel, Wetter, Wasser, Vegetation, Chronik, Speichern). Keine Grafik, keine KI. |
| `crates/app` | Desktop-Beobachter `nature-in-a-glass` |
| `crates/cli` | Kommandozeile `nig-sim` für Zeitraffer-Läufe ohne Fenster |

Selbst bauen (nur nötig für Entwicklung): [Rust](https://rustup.rs) installieren, dann `cargo run --release -p nature-in-a-glass`. Tests: `cargo test --release`.

## Arbeitsweise

Kleine reproduzierbare Simulationstests; feste Seeds für Vergleiche; versionierte Spielzustände; strikte Trennung von Engine, Darstellung und KI. Hardware-Limits für einen späteren Test auf TheCave sind ein Designziel, keine Voraussetzung für die Architektur.
