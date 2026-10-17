# ADR 0002 – Laufzeit, Simulationskern und Darstellung

**Status:** angenommen (2026-10-17)

## Kontext

Die Engine muss zwei Dinge gleichzeitig können, die sich normalerweise widersprechen:

1. **Tiefe Zeit:** Jahrhunderte bis Jahrtausende im Zeitraffer simulieren (Vegetation, Boden, später Geologie und Generationen).
2. **Nahaufnahme:** Jederzeit anhalten und einen einzelnen Bewohner Minute für Minute beobachten.

Weitere Anforderungen aus ADR 0001 und den Prinzipien: headless lauffähig (Tests, TheCave), deterministisch mit festem Seed, Speichern/Laden mit Versionierung, KI nur über Adapter, Darstellung strikt getrennt.

Rahmenbedingungen des Projekts:

- Dennis programmiert nicht selbst. Der Code wird mit Claude geschrieben; Dennis beobachtet, testet und entscheidet über das Spieldesign. Es muss daher **ohne Entwicklungsumgebung startbar** sein: herunterladen, doppelklicken.
- Zuerst eine **Desktop-Anwendung**, später optional im Browser.
- Darstellung: **isometrische Ansicht plus 2D-Rasterkarte**, im Stil alter Strategiespiele. Keine 3D-Engine nötig.

## Entscheidung

- **Sprache: Rust.** Ein Kern für Desktop, Kommandozeile/Server und später WebAssembly. Ergebnis ist eine einzelne ausführbare Datei ohne Laufzeitinstallation. Die Lernkurve spielt keine Rolle, weil Dennis den Code nicht selbst schreibt; der strenge Compiler fängt dafür viele Fehler ab, bevor sie im Spiel landen.
- **Aufbau als Cargo-Workspace:**
  - `crates/core` – die Simulation. Keine Grafik, kein Netzwerk, keine KI.
  - `crates/cli` – Kommandozeilenprogramm `nig-sim` für Zeitraffer-Läufe, Auswertungen und Benchmarks (z. B. auf TheCave).
  - `crates/app` – Desktop-Beobachter `nature-in-a-glass`.
- **Darstellung: egui/eframe.** Fenster, Bedienelemente und die Kartenzeichnung (Isometrie und Raster) aus einer Bibliothek; läuft unter Windows, macOS, Linux und später auch im Browser. Reicht für M1–M3. Wird später mehr Grafik nötig (Sprites, Animationen), kann die Darstellung ausgetauscht werden, ohne den Kern anzufassen.
- **Fertige Programme über GitHub Actions:** Bei jeder Version werden Downloads für Windows, macOS und Linux unter *Releases* erstellt.

## Zeit in mehreren Auflösungen

Keine einheitliche Tick-Rate. Jedes System hat seinen eigenen Takt; ein Scheduler ruft sie nach Simulationszeit auf.

| System | Takt in M1 |
| --- | --- |
| Uhr, Tageslicht | Stunde (nur Anzeige) |
| Wetter, Niederschlag, Temperatur | Tag |
| Oberflächenwasser, Abfluss, Bodenfeuchte, Quellen | Tag |
| Vegetation (Gras, Wald) | Tag |
| Jahresbilanz / Chronik | Jahr |

Später kommen feinere Takte für Bewohner (Minuten) hinzu, und Bereiche ohne Beobachter werden gröber weitergerechnet.

## Determinismus

- Eigener Zufallsgenerator (SplitMix64) mit getrennten Strömen pro System.
- Im Kern nur Grundrechenarten (+ − × ÷, Wurzel); keine Winkelfunktionen oder Exponentialfunktionen der Plattform, da diese sich zwischen Betriebssystemen minimal unterscheiden können.
- Feste Iterationsreihenfolge; Prüfsumme über den Weltzustand. Ein Test stellt sicher, dass gleicher Seed → gleiche Prüfsumme.

## Speichern

Spielstand als versioniertes JSON (`format_version`, Engine-Version, Seed, Zustand, Chronik). Lesbar und für den kleinen MVP schnell genug. Ein Binärformat oder SQLite folgt, wenn Welten oder Chroniken groß werden.

## Verworfene Optionen

- **TypeScript durchgängig:** schneller Start, aber Performance-Decke beim Zeitraffer und schwerer abzusichernder Determinismus.
- **Godot:** fertiger Renderer, aber Simulation rückt nah an die Engine; headless-Betrieb und Kerntests umständlicher.
- **Python:** gut für Auswertungen, als Engine zu langsam für viele Bewohner. Kann weiter als Analysewerkzeug für exportierte Läufe dienen.
- **Bevy:** mächtig, aber für M1 schwergewichtig und mit häufigen API-Brüchen. Bleibt Kandidat für eine spätere grafischere Darstellung.

## Konsequenzen

- Jede Änderung wird über automatische Tests und CI abgesichert, da Dennis Code nicht selbst prüfen kann.
- Spielbare Stände kommen als Downloads, nicht als Quellcode-Anleitung.
- Der Kern bleibt grafikfrei; eine Browser-Version ist ein weiteres Frontend, kein Neubau.
