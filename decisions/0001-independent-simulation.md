# ADR 0001 – Eigenständige Simulation, KI optional

**Status:** vorgeschlagen

## Kontext
AI Town eignet sich als experimenteller Referenzpunkt für Dialog und Erinnerung, stellt jedoch keine allgemeine Natur-, Material- und Zivilisationssimulation dar. Ein kleiner Homeserver darf durch LLM-Inferenz nicht blockiert werden.

## Entscheidung
Nature in a Glass wird als eigenes Projekt aufgebaut. Die Simulation erhält einen deterministischen Kern mit strukturierter Welt und erlaubten Aktionen. Optional angeschlossene Sprachmodelle dürfen Dialoge, Bewertungen oder Handlungs-/Erfindungshypothesen liefern, jedoch keine ungültigen Weltzustände unmittelbar schreiben.

## Konsequenzen
- Das System lässt sich ohne GPU oder KI testen und in Zeitraffer ausführen.
- KI-Funktionen können später ausgetauscht oder extern berechnet werden.
- Neue Spielmechaniken benötigen explizite Validierung und physikalisch/ökologisch begründete Regeln.
- Das Projekt ist zunächst mehr Engineering-Arbeit als eine direkte Modifikation von AI Town.
