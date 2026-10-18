# Fuchs – unabhängiger Einzelbildversuch

Grundlage: docs/gestaltung.md, Abschnitte 4, 5, 6 und 15; Projektstand 17b078c.
Vergleichspartner: assets/nature/animals/fox_0_graze_r_f0.png (20×12,
Darstellung 10×6, Anker 10/10). Dieser Projektstand wurde in Runde 09
per Pixelzeichenskript auf Basis älterer ComfyUI-Studien ausgearbeitet und
ist kein unverändertes Günther-Ergebnis.

Variante B: fox-b-imagegen-source.png, unabhängig mit dem eingebauten
Imagegen-Werkzeug aus Textvorgaben erzeugt, ohne Günther/ComfyUI-Pipeline.
Ein Fuchs, stehend mit Kopf oben, Blick rechts unten, transparent.

Historisch erfüllte die Generierung weder das damals verlangte 20×12-Raster
noch die Grenze von sechs Farben. Dennis hat gerade diesen detailreicheren
Stil anschließend ausdrücklich gewählt. Die aktualisierte Gestaltungsvorlage
erlaubt freie Farbzahl und Auflösung; das frühere Urteil ist damit überholt.

## Nutzbarer Einzelbildexport

`fox_0_graze_r_f0.png`: 80×48 RGBA, sRGB, Darstellungsmaß 10×6,
Anker 40/40 in Dateipixeln (= 5/5 im Weltmaß, wie bisher).
`manifest.json` beschreibt die einzelne Bildzuordnung. Kein Spielasset ersetzt:
Eine einzelne größere Datei zwischen alten Animationsbildern würde deren
Skalierung verändern. Deshalb liegt dieser Prüfexport außerhalb des Atlas.

`comparison.png` zeigt A/B bei identischem Weltmaß und Fußpunkt auf drei
Böden, Zoom 4/8/16/32, mit der pixelgenauen Skalierung des Beobachters.
Keine Spielaufnahme. Bei Zoom 4 bleiben beide Tiere wenige Pixel klein;
die plastische Form wird vor allem bei Zoom 16/32 sichtbar. Der neue Fuchs
hat eine stärkere Draufsicht als der alte; kein exakt gleicher Umriss.
Die Bildquelle bleibt unverändert mit versioniert. Der Export beschneidet
fast transparente Außenreste, setzt den blaugrauen Schatten auf 30 Prozent
und verkleinert die Referenz mit erhaltener Transparenz. Keine neue KI-Runde.

Nachbau: `python tools/nature/build_fox_comparison.py` (Pillow erforderlich;
Prüfblatt-Schrift Segoe UI unter Windows). `validation.json` enthält Maße,
Quellhash und Farb-/Deckkraftzahlen. Spielabnahme und weitere Frames bleiben
offen; das Paket ist zunächst zur visuellen Entscheidung gedacht.

## Spielaufnahmen (Claude, 18.10.2026)

Probeversion des Beobachters, in der alle Fuchsbilder durch `fox_0_graze_r_f0.png`
(bzw. gespiegelt für `_l`) ersetzt waren, Anker 40/40. Insel 7, Tag 22 nach dem
Aussetzen, Feld (12, 36). Jeweils links bisher, rechts neu.

- `spiel-zoom-4-8.png` – oben Zoom 4, unten Zoom 8 (beide dreifach vergrößert).
- `spiel-zoom-16-32.png` – oben Zoom 16, unten Zoom 32.

Befund: Bei Zoom 16 und 32 wirkt der neue Fuchs deutlich natürlicher und plastischer.
Bei Zoom 4 und 8 verliert er dagegen seine Form: Das 8-fach aufgelöste Bild wird
ohne Glättung stark verkleinert, die Silhouette (Ohren, heller Schwanzspitz)
zerfällt in Einzelpixel, während der alte Fuchs dort klarer lesbar bleibt.
Abhilfe technisch (Claude): verkleinerte Fassungen bzw. geglättete Verkleinerung
im Atlas (ADR 0006). Gestalterisch (Codex): prüfen, ob für Zoom 4–8 eine eigene
kleine Fassung mit klarer Silhouette nötig ist.

