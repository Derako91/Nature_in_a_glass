# Agenten-Zusammenarbeit

## Einstieg (Claude, Codex und weitere Agenten)

1. **Zuerst `versionsstand.md` lesen.** Dort stehen das aktuelle Hauptproblem, bekannte Testergebnisse und die nächste konkrete Prüfung.
2. Dann `AUFGABE.md` (sofern vorhanden), passende `NACHTRAG-*.md`, `SCHNITTSTELLEN.md` (sofern vorhanden) und betroffene Quelldateien lesen.
3. Nachträge sind **chronologische Praxisberichte**, keine stets gültige To-do-Liste. Neuere bestätigte Ergebnisse können ältere Probleme erledigt haben.
4. Den Zustand aus Git/Code/Tests prüfen. Dokumentierte Heimserver-Tests nicht als selbst ausgeführte Tests ausgeben.
5. **Nur eine verantwortliche Person/ein Agent pro Aufgabe und Branch.** Bei Übergabe Commit und offenen Punkt angeben; nicht unkoordiniert dieselbe Datei parallel bearbeiten.
6. Am Ende `versionsstand.md` aktualisieren: **funktioniert / Problem / nächster Schritt / Branch+Commit / ausgeführte Tests / noch ungeprüft**. Keine pauschale Aussage „fertig“ ohne Beleg.
7. Keine Passwörter, Tokens, privaten Rohdaten oder echten Testbücher in Git übernehmen.

## Dateizweck

- `versionsstand.md`: **eine Bildschirmseite zuerst**; aktueller Arbeitsstand und konkrete Übergabe.
- `AUFGABE.md`: ursprünglicher Auftrag und Zielbild.
- `NACHTRAG-*.md`: unveränderte Beobachtungen, Messungen, Fehler und Folgeaufträge.
- `SCHNITTSTELLEN.md`: technische Schnittstellen und lokale Abhängigkeiten.
- `CLAUDE.md`: bestehende verbindliche Projekt- und Rollenregeln.\n- `docs/gestaltung.md`: visuelle Spezifikation und Abnahmekriterien; kein Ersatz für den Versionsstand.\n
## Vorlage für die nächste Übergabe

- **Problem:** Was ist reproduzierbar falsch? Welche Auswirkung?
- **Bereits funktioniert:** Was wurde mit welchem Test nachgewiesen?
- **Änderung:** Welche Dateien/Commits wurden bearbeitet?
- **Prüfung:** Welcher Befehl, welche Umgebung, welches Ergebnis?
- **Offen:** Was konnte nicht geprüft werden und warum?
- **Nächste Aktion:** Ein konkreter, abgegrenzter Auftrag mit Abnahmekriterium.
