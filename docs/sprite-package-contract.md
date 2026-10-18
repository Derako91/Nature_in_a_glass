# Sprite-Paket v1 – Vorschlag zur Abstimmung mit Claude

Stand 18.10.2026. **Noch nicht von Claude bestätigt.** Dennis weist Claude auf
diese Datei hin; ein GitHub-Eintrag ist keine automatische Benachrichtigung.
Codex bearbeitet `tools/sprite_agent/`, Profile und Prüfassets. Claude bearbeitet
Renderer, Atlas und dessen LOD-Unterstützung. Keine parallelen Rendereränderungen.

## Übergabe

Ein Paket ist ein eigener, unveränderlicher Laufordner mit `manifest.json`,
`job.json`, `snapshot.json`, `style-source.md`, `validation.json`,
`extraction.json`, `review.html`, Kontaktbögen und `sprites/`. Ein ZIP daneben
enthält den gesamten Lauf. `raw/` und `workflows/` dokumentieren lokale
Generierung und Seeds. Vorschläge bleiben außerhalb von `assets/nature/`.

Das Manifest hat `schema_version: 1`, `package_id`, `asset_id`, `category`,
`display_size` und `anchor` in **Welteinheiten des Grundmaßes bei Zoom 4**.
Jeder Frame enthält Zustand, Richtung, Variante, Frameindex, Dauer in ms und
`lods`. Jede LOD enthält Dateipfad, Maße, Anker in **Dateipixeln**, Skalierung,
Zoom-Untergrenze, Erzeugungsmethode und SHA256. Alle Dateien: PNG RGBA 8 Bit,
sRGB, freie Farbzahl und Alpha. Keine Spiegelung als Ersatz für eine Richtung.

Beispiel: Fuchs `display_size: [10,6]`, Weltanker `[5,5]`:

| LOD | Datei B×H | Dateifußpunkt | vorgeschlagener Zoombereich |
| --- | --- | --- | --- |
| lod1 | 10×6 | 5/5 | 1,5 bis <6 |
| lod2 | 20×12 | 10/10 | 6 bis <12 |
| lod4 | 40×24 | 20/20 | 12 bis <24 |
| lod8 | 80×48 | 40/40 | ab 24 |

Das sind **Vorschläge**, keine Renderer-Vorgaben. Unter Zoom 1,5 und ab
1 Monat/s gelten weiterhin die Spurenregeln der Engine. Der Produzent stellt
keine Tierdichte dar. Beim LOD-Wechsel müssen Weltfußpunkt, Weltmaß, Variante,
Zustand und Animationsphase unverändert bleiben. Hysterese gegen Hin-/Herspringen
nahe einer Schwelle ist Sache des Renderers.

Richtungen sind bildschirmbezogen: `se` rechts unten, `sw` links unten,
`ne` rechts oben, `nw` links oben. Licht bleibt rechts oben. Die heutige Engine
verwendet nur `l/r`; eine Zuordnung `sw→l`, `se→r` ist möglich. Rückansichten
werden **nicht** stillschweigend in bestehende IDs umbenannt. Dateiname:
`<id>_<variante>_<zustand>_<richtung>_f<index>__<lod>.png`.

## Sicherheit der Freigabe

`technical_status: passed` bestätigt nur die messbaren Eigenschaften.
`visual_status: pending_human_review` bleibt bis zur menschlichen Sichtprüfung
offen. Ein gesondertes `approval.json` nennt Prüfer, Notiz und Manifest-Hash.
Kein automatischer Import, kein Überschreiben freigegebener Assets. Das erste
Fuchspaket umfasst vier Standansichten; keine vorgetäuschten Laufanimationen.

Automatische Fußpunkte sind zunächst eine **Heuristik** (untere Motivmitte).
Anatomisch korrekte Fußpunkte, stimmige Richtung, gleiche Figur und kleine
Silhouette müssen im Kontaktbogen geprüft werden. LOD1/2 sind gefilterte,
farblich vereinfachte Kandidaten; sie sind nicht automatisch eine künstlerisch
neu gezeichnete kleine Fassung.

## Rückmeldung von Claude erbeten

- Passen Manifest, Bildpfade und Grundmaß ohne Änderungen an der Simulation?
- Welche LOD-Stufen und Umschaltgrenzen soll der Atlas tatsächlich verwenden?
- Soll die Engine vier Richtungen unterstützen oder zunächst `l/r` importieren?
- Wie sollen freigegebene Pakete in den vorhandenen Atlas übernommen werden?

Claude kann seine Antwort in einem separaten Abschnitt dieser Datei oder im
zugehörigen Pull Request ergänzen. Bis zur Bestätigung ist nichts integriert.
