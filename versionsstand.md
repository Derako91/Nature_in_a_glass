# Versionsstand – Nature_in_a_glass

> **Stand der Dokumentation:** 29.10.2026 · **Bezugs-Commit:** `f24cbd9559e432d5a69b8411a513c661d2e0d864` · **Prüfstatus:** aus vorhandenen Projektdateien rekonstruiert; Code, CI und Heimserver wurden für diese Übersicht **nicht erneut getestet**.
>
> **Wichtig:** „Offen“ bedeutet hier *zu prüfen bzw. laut letztem Praxisbericht noch zu bearbeiten* – nicht automatisch, dass der Fehler im aktuellen Code weiterhin existiert.

## Auf einen Blick

| Frage | Antwort |
| --- | --- |
| **Hauptthema / Problem** | Gestaltung und Integration |
| **Was nachweislich bereits lief** | Claude: Simulation, Tests und Integration; Codex: Texturen, Sprites und Darstellung |
| **Was jetzt geklärt werden muss** | Gestaltungsaufgaben aus Abschnitt 16 gegen Implementierung, offene PRs und Bildprüfung abgleichen. Nicht behaupten, ein offener Punkt sei ein Laufzeitfehler. |
| **Nächster sinnvoller Test** | Codex/Claude-Übergabe nach Änderungen an Assets oder Renderer |
| **Letzter eigener Test dieser Datei** | Keiner – Dokumentationsübergabe, keine Codeausführung |

## Offene Punkte – zuerst prüfen

1. **Ist das gemeldete Problem noch vorhanden?** Prüfe den aktuellen Code, den letzten Commit und gegebenenfalls den Livebetrieb, bevor du etwas änderst.
2. **Falls ja:** Gestaltungsaufgaben aus Abschnitt 16 gegen Implementierung, offene PRs und Bildprüfung abgleichen. Nicht behaupten, ein offener Punkt sei ein Laufzeitfehler.
3. **Nach der Änderung:** Codex/Claude-Übergabe nach Änderungen an Assets oder Renderer. Ergebnis mit Datum, Commit und konkretem Test protokollieren.

## Zusammenarbeit Claude / Codex

- **Claude:** Simulation, technische Schnittstellen, Tests und Integration.\n- **Codex:** Assets, Sprites, Texturen und visuelle Darstellung gemäß `docs/gestaltung.md`.\n- `docs/gestaltung.md` bleibt verbindliche Gestaltungsspezifikation, **nicht** das laufende Statusprotokoll.
- **Vor Arbeitsbeginn:** diese Datei, `AUFGABE.md` (falls vorhanden), die relevanten `NACHTRAG-*.md` und die betroffenen Quelldateien lesen.
- **Nach Arbeitsende:** diese Datei mit tatsächlich erledigter Änderung, offenem Problem, Branch/Commit, Testbefehl und Ergebnis aktualisieren.
- **Bei Unklarheit:** keine Erfolgsmeldung erfinden; Status `ungeprüft` setzen und konkrete Prüfung nennen.

## Letzte Übergabe

- **Von:** Dokumentationsanalyse (kein ausführender Coding-Agent)
- **An:** nächster bearbeitender Agent
- **Auftrag:** den oben genannten Problemstand mit dem aktuellen Code abgleichen und erst dann gezielt korrigieren.
- **Quellen:** CLAUDE.md; docs/gestaltung.md; docs/PRINCIPLES.md
- **Nicht verändern:** bestehende Aufgaben-/Nachtragshistorie; keine Funktionalität ohne gesonderten Auftrag umbauen.

## Änderungsprotokoll

| Datum | Änderung | Code-Test |
| --- | --- | --- |
| 29.10.2026 | Erste gemeinsame, lesbare Übergabe aus den vorhandenen Projektberichten erstellt | Nicht ausgeführt |
