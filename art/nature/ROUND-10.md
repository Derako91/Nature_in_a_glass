# Runde 10 – Landschaft für die Nahansicht

Dennis hat am 18.10.2026 die detailreichere Landschaft in doppelter Auflösung
beauftragt. Diese Entscheidung ersetzt den bisherigen Satz in 4.3, dass nur
Tiere größere Bilder benötigen. Die Darstellungsgrößen aus 4.2 bleiben erhalten.

## Ergebnis

59 PNGs: acht Bodenarten mit je vier Varianten, drei Wände, vier Wasserbilder
und fünf Pflanzenarten mit je vier Varianten. Alle Dateien in den bereits
eingebetteten Ordnern; alte `_0`-Dateien werden ersetzt, `_1` bis `_3` ergänzen
die vorhandene Variantensuche. Keine neuen Simulationstypen.

- Boden und stehendes Wasser: 128×64 statt 64×32 Pixel.
- Wände: 64×64 statt 32×32; Laufwasserstreifen: 32×64 statt 16×32.
- Pflanzen: doppelte Bildmaße und Anker; z. B. Altbaum 36×60, Busch 20×16.
- Feinere Materialkörnung, Felsrisse und Grasstriche; neu aufgebaute Kronen mit
  Blattgruppen, sichtbaren Stämmen und Ästen, gesplittertes stehendes Totholz.

Die vorhandenen ComfyUI-Materialvorlagen werden in höherer Auflösung neu
abgetastet und im bearbeitbaren Pixelprogramm redigiert. Keine neue KI-Generierung.
`tools/nature/build_landscape_detail.py` erzeugt das Paket reproduzierbar aus
den Vorlagen und dem Grundmanifest des Git-Stands `2babb42`.

## Maßstab und Einbindung

Das Manifest enthält `size` für die Bildpixel, `display_size` für das Grundmaß,
`resolution_scale: 2` und Anker in Bildpixeln. Die Boden-/Wandgeometrie im
Beobachter hat bereits eine feste Größe. Pflanzen wurden dagegen bisher direkt
nach Bildpixeln gezeichnet. `draw_iso_gfx` normiert nun ihre Breite auf die
Werte aus 4.2. So bleiben auch Fußpunkte und Schatten an der gleichen Weltposition.
Unbekannte Spritearten behalten das bisherige Verhalten. Atlas und Simulation
bleiben unverändert. Die älteren Biom-/Tierprüfprogramme reduzieren geladene
Materialien auf ihr Prüfgrundmaß, damit keine doppelt großen Pflanzen entstehen.

## Prüfung und offene Punkte

Maße, maximal sechs Farben, Alpha 0/77/255 und tatsächliche Zusatzdetails gegenüber
einer bloßen Pixelvergrößerung werden beim Bau geprüft. Das Vergleichsblatt zeigt
dieselbe feste Belegung und Weltgröße, links bisherige Bilder, rechts neue Bilder
mit Varianten. Es ist eine Renderprobe, keine Aufnahme aus dem laufenden Spiel.

Die Rust-Änderung am Pflanzenmaßstab benötigt die regulären Format-/Clippy-/Tests
auf GitHub. Lokal ist cargo nicht installiert. Eine tatsächliche Spielaufnahme
vorher/nachher und die Abnahme bei Zoom 2/4/8 bleiben daher offen. Die geänderten
Pflanzenkonturen und Materialvarianten wurden auf dem Prüfblatt visuell kontrolliert.
Wasseranimation und noch nicht eingebundene Biomübergänge werden hier nicht ersetzt.
