# Sprite-Paket v1 – Produktionsformat und Engine-Export

Stand 19.10.2026. Claude hat inzwischen die Engine-Schnittstelle in
`gestaltung.md` 16.6 / ADR 0008 veröffentlicht (v0.7.2). Der Ausgabeadapter
folgt dieser Vorgabe. **Die Agenten-Implementierung ist noch nicht von Claude geprüft.** Dennis weist Claude auf
diese Datei hin; ein GitHub-Eintrag ist keine automatische Benachrichtigung.
Codex bearbeitet `tools/sprite_agent/`, Profile und Prüfassets. Claude bearbeitet
Renderer, Atlas und dessen LOD-Unterstützung. Keine parallelen Rendereränderungen.

## Verbindliche Engine-Ausgabe seit v0.7.2

`export-engine PAKET --out NEUER_ORDNER --preview` erstellt einen separaten
Prüfexport für `nature-in-a-glass --bilder NEUER_ORDNER`:

- Hauptfassung ohne LOD-Suffix, 2×-Fernfassung als `@fern` im Gruppenordner.
- `manifest.json` mit `assets`, `files`, `anchor`, `display_size` und
  `levels: [{label: "@fern", files: [...], anchor: [...]}]`.
- Angeforderte `se/sw` werden explizit `r/l` zugeordnet. `ne/nw` bleiben nur im
  Produktionspaket und stehen als ausgelassen im Exportbericht. Keine Spiegelung.
- Zwischenstufen und Auswahl übernimmt die Engine nach Claudes 1,5-Regel.
- Export ist keine Installation. Ohne `--preview` ist ein Freigabeprotokoll mit
  passendem Manifest-Hash nötig. Existierende Exportziele werden nie überschrieben.

Die automatischen kleinen Bilder heißen im Prüfexport technisch `@fern`, sind
aber noch keine gestalterisch freigegebenen Fernzeichnungen. Der Fuchstest
ist wegen falscher Richtungen und Stildrift nicht zur Übernahme empfohlen.
Der Adapter kann diese Bildmängel nicht beheben.

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

Das sind interne Vorschaugrenzen des allgemeinen Produktionspakets, **keine
Renderer-Vorgaben**. Der Nature-Export verwendet Hauptbild + 2×-Fernbild und
Claudes automatische Stufenwahl. Unter Zoom 1,5 und ab
1 Monat/s gelten weiterhin die Spurenregeln der Engine. Der Produzent stellt
keine Tierdichte dar. Beim LOD-Wechsel müssen Weltfußpunkt, Weltmaß, Variante,
Zustand und Animationsphase unverändert bleiben. Hysterese gegen Hin-/Herspringen
nahe einer Schwelle ist Sache des Renderers.

Richtungen sind bildschirmbezogen: `se` rechts unten, `sw` links unten,
`ne` rechts oben, `nw` links oben. Licht bleibt rechts oben. Die heutige Engine
verwendet nur `l/r`; der Export ordnet `sw→l`, `se→r` zu. Rückansichten
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

## Noch von Claude zu prüfen

- Entspricht der Exportadapter seiner veröffentlichten Schnittstelle 16.6?
- Bleiben beim Prüfladen mit `--bilder` Fußpunkte und Weltmaß stabil?
- Vier echte Ansichten bleiben ein separater Engine-Auftrag; der jetzige
  Export benötigt diese Erweiterung ausdrücklich nicht.

Claude kann seine Antwort in einem separaten Abschnitt dieser Datei oder im
zugehörigen Pull Request ergänzen. Absprachen bleiben gemäß CLAUDE.md in
`gestaltung.md` 16.3/16.6 maßgeblich; diese Datei dokumentiert die Umsetzung.
Es ist nichts integriert.
