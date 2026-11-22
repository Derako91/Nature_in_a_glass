# Agenten-Zusammenarbeit

## Einstieg (Claude, Codex und weitere Agenten)

1. **Zuerst `versionsstand.md` lesen.** Dort stehen das aktuelle Hauptproblem, bekannte Testergebnisse und die nächste konkrete Prüfung.
2. Dann `AUFGABE.md` (sofern vorhanden), passende `NACHTRAG-*.md`, `SCHNITTSTELLEN.md` (sofern vorhanden) und betroffene Quelldateien lesen.
3. Nachträge sind **chronologische Praxisberichte**, keine stets gültige To-do-Liste. Neuere bestätigte Ergebnisse können ältere Probleme erledigt haben.
4. Den Zustand aus Git/Code/Tests prüfen. Dokumentierte Heimserver-Tests nicht als selbst ausgeführte Tests ausgeben.
5. **Nur eine verantwortliche Person/ein Agent pro Aufgabe und Branch.** Bei Übergabe Commit und offenen Punkt angeben; nicht unkoordiniert dieselbe Datei parallel bearbeiten.
6. Am Ende `versionsstand.md` aktualisieren: **funktioniert / Problem / nächster Schritt / Branch+Commit / ausgeführte Tests / noch ungeprüft**. Keine pauschale Aussage „fertig“ ohne Beleg.
7. **Selbst hochladen, ohne zu fragen:** Am Ende jeder Aufgabe den eigenen Zweig (Codex: `codex/<thema>`) committen, auf GitHub pushen und einen Draft-Pull-Request öffnen. Dennis programmiert nicht und soll weder zum Hochladen auffordern noch Pull Requests übernehmen müssen; Claude prüft und übernimmt Codex-Zweige selbst (Freigabe in CLAUDE.md). Geht das Hochladen technisch nicht, das am Ende in einem Satz sagen statt nachzufragen.
8. Keine Passwörter, Tokens, privaten Rohdaten oder echten Testbücher in Git übernehmen.

## Dateizweck

- `versionsstand.md`: **eine Bildschirmseite zuerst**; aktueller Arbeitsstand und konkrete Übergabe.
- `AUFGABE.md`: ursprünglicher Auftrag und Zielbild.
- `NACHTRAG-*.md`: unveränderte Beobachtungen, Messungen, Fehler und Folgeaufträge.
- `SCHNITTSTELLEN.md`: technische Schnittstellen und lokale Abhängigkeiten.
- `CLAUDE.md`: bestehende verbindliche Projekt- und Rollenregeln.
- `docs/gestaltung.md`: visuelle Spezifikation und Abnahmekriterien; kein Ersatz für den Versionsstand.

## Vorlage für die nächste Übergabe

- **Problem:** Was ist reproduzierbar falsch? Welche Auswirkung?
- **Bereits funktioniert:** Was wurde mit welchem Test nachgewiesen?
- **Änderung:** Welche Dateien/Commits wurden bearbeitet?
- **Prüfung:** Welcher Befehl, welche Umgebung, welches Ergebnis?
- **Offen:** Was konnte nicht geprüft werden und warum?
- **Nächste Aktion:** Ein konkreter, abgegrenzter Auftrag mit Abnahmekriterium.

## Statuskopf in `versionsstand.md` (DevHub-Standard)
Gilt seit 22.11.2026. Vollständiger Standard: `STANDARD-VERSIONSSTAND.md` im Repository `Derako91/Devhub`; jeder PR wird
automatisch danach geprüft (`.github/workflows/devhub.yml`).
- `versionsstand.md` ist die **einzige Statusquelle**: Projektblock (` ```yaml devhub-projekt `), je Arbeitspaket ein
  Abschnitt `## Paket <ID> – <Titel>` mit Block (` ```yaml devhub-paket `), Text (Problem / Funktioniert / Offen /
  Nächster Schritt / Verantwortlich / Branch+Commit / Tests) und eigenem Verlauf; darunter `## Historie` (unverändert).
- **Jeder PR gehört zu einem Paket** (`branch:` im Block = PR-Branch). Neue Arbeit: eigenes Paket im eigenen PR direkt vor
  `## Historie` anlegen. Ausnahmen: Planungs-PR (Branch `plan/…` oder Label `devhub:planung`) und Status-PR (ändert nur
  `versionsstand.md`, z. B. Abschluss nach Merge).
- **Nur den eigenen Paketabschnitt ändern** – nie fremde Pakete, Projektblock oder Historie (außer im Planungs-PR).
- **Revier einhalten:** nur Dateien aus `revier` des eigenen Pakets ändern; sonst `zustand: wartet_auf_dennis` mit `frage`.
- **Zustand** (`geplant`, `bereit`, `in_arbeit`, `wartet_auf_dennis`, `blockiert`, `uebergabe`, `erledigt`, `verworfen`) und
  **Prüfstufe** (`keine`, `behauptet`, `getestet` = automatische Tests, `attrappe`, `live`) getrennt pflegen; jede Stufe über
  `behauptet` braucht einen Eintrag in `tests` (Befehl, Datum, Umgebung, Ergebnis, erfolgreich).
- **`live` gibt nur Dennis frei.** Ein Agent trägt Live-Tests mit `art: live`, `freigabe: offen` ein.
- **Übergabe in zwei Commits:** erst die Arbeit committen; dann ein Commit nur mit `versionsstand.md`: `zustand: uebergabe`,
  `naechster`, `commit` = voller Hash des Arbeitscommits. Danach nichts mehr außer `versionsstand.md`.
- **Nach dem Merge:** Abschluss als Status-PR (`status/…`) oder Commit auf dem Standard-Branch: `zustand: erledigt`,
  `naechster: keiner`, Verlauf „angenommen durch Merge <PR>“; die Prüfstufe bleibt.
- **Konflikt in `versionsstand.md`:** abschnittsweise lösen – fremde Abschnitte vom Ziel-Branch, den eigenen aus dem Branch;
  nie Gits Konfliktblöcke einzeln. Derselbe Abschnitt auf beiden Seiten geändert: Paket `blockiert`, nicht weiterarbeiten.
