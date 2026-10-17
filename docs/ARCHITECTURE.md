# Architekturentwurf 0.1

## Designziel

Eine **headless, reproduzierbare Simulationsengine** verarbeitet begrenzte Zeitschritte. Die grafische Oberfläche liest ihre Zustände und kann erlaubte Umwelt-Eingriffe als Events einreichen. Weder Browser noch LLM sind für den Simulationsfortschritt erforderlich.

```text
UI / Beobachtung / Godmode
           │ Commands (nur definierte Welteingriffe)
           ▼
Event- und Simulationskern ──→ Ereignisjournal / Snapshots
           │
   ┌───────┼────────────────────────┐
   ▼       ▼                        ▼
Welt      Lebewesen             Soziales & Wissen
Terrain   Bedürfnisse           Beziehungen, Lernen
Wasser    Wahrnehmung           Später Kultur/Technik
Pflanzen  Aktionen
   └───────┴────────────────────────┘
           │ optionale, limitierte Abfragen
           ▼
      KI-Adapter (z.B. Ollama)
```

## Module / Zuständigkeiten

- `core`: Simulationsuhr, reproduzierbare Zufallsquelle, Scheduler, Ereignisse, Saves und Snapshots.
- `world`: Raster, Höhen, Wasser, Wetter, Boden, Vegetation, später Tiere und Geologie.
- `life`: Zustände, Bedürfnisse, Wahrnehmung, Aktionsplanung, Bewegung und Sterblichkeit.
- `society`: Beziehungen, Kooperation, später Rollen und Gruppen.
- `knowledge`: persönliche Erfahrungen, Fähigkeiten, Lehren, Materialien und zunächst validierte Rezepte; später experimentelle Konstruktionen.
- `history`: persistente Weltchronik, Artefakte, Gebäudeverfall und Herkunftsnachweise.
- `godmode`: geprüfte Änderungen von Parametern/Quellen; niemals direkte NPC-Kontrolle.
- `ai`: optionale Adapter für Dialoge, Reflexionen und Hypothesen mit Budgets/Timeouts/Fallbacks.
- `frontend`: Karte, Zeitleiste, Bewohneransicht und Parameteroberfläche.

## Modellierungsprinzipien

- Simulationskern und UI entkoppeln; tickbasierte Engine mit separaten Updatefrequenzen für Bewegung, Bedürfnisse und Umwelt.
- Raster/Chunk-basierte Welt, deren abwesende Bereiche grob oder ereignisbasiert weiterberechnet werden können.
- Ein Eventjournal mit Version, Seed, Simulationszeit, Ursache und Wirkung; periodische Snapshots für schnelles Laden.
- Speicherung beginnt einfach (lokale Dateien/SQLite als Kandidaten, noch nicht final entschieden), später bei Bedarf serverfähig.
- Keine direkte Netzwerk- oder Ollama-Abhängigkeit im Kern: Adapter erhalten strukturierte, begrenzte Aufträge und liefern Vorschläge, die validiert werden.
- Beziehungsbewertungen sind Beobachtungs-/Analysewerte, nicht automatisch Steuerungswerte.

## Erste Datentypen (Beispiel, noch kein endgültiges Schema)

`World { seed, simTime, tiles, entities, events }`

`Tile { elevation, soilMoisture, water, vegetation, resources }`

`Agent { id, traits, needs, location, memoryRefs, knownSkills }`

`Event { id, time, type, actor?, location?, causes[], effects[] }`

`Intervention { type, region, strength, duration, initiatingObserver }`

## Performance und Sicherheit

- Keine permanente LLM-Abfrage pro Agent/Tick.
- Konfigurierbare CPU-/RAM-Budgets, Pausenregel und saubere Isolation von TheCave-Produktivdiensten.
- Tests mit deaktivierter KI müssen vollständige Simulationszyklen ermöglichen.
- Vor KI-Integration: Profiling und Benchmarks für 5/50/200 Bewohner, ohne diese Größen bereits als unterstützt zu behaupten.

## Umsetzung in Version 0.1

Entschieden in [ADR 0002](../decisions/0002-runtime-and-rendering.md): Rust-Workspace mit grafikfreiem Kern.

| Modul (Entwurf oben) | Umgesetzt in |
| --- | --- |
| `core` | `crates/core/src/sim.rs` (Uhr, Scheduler, Eingriffe, Speichern), `rng.rs`, `math.rs` |
| `world` | `world.rs` (Insel, Boden, Abflussnetz), `climate.rs`, `hydro.rs`, `vegetation.rs` |
| `history` | `events.rs` (Chronik mit Ursachen) |
| `godmode` | `Intervention` in `sim.rs` – nur Klima und Quellen |
| `frontend` | `crates/app` (egui), `crates/cli` |

Ablauf eines Tages: Wetter würfeln → Regen und Quellen → Versickern und Abfluss bergab → Verdunstung, Pflanzenwasser, Grundwasserneubildung → Gras und Wald → Ereignisse erkennen. Jahreswechsel: Jahresbilanz in Chronik und Verlauf.

Eine Kachel ist 50 × 50 m, die Insel damit gut 3 × 3 km. Wassermengen werden in Millimetern auf einer Kachel gerechnet.

## Offen

Grad der deterministischen Reproduzierbarkeit mit LLM sowie die Tiefe des Material- und Geologiemodells werden separat entschieden.
