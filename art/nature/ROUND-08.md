# Runde 08 – Gebirgsgewässer und Wasserfälle

Vier zusammenhängende Prüfbelegungen zeigen Gebirgsbach und Gebirgsfluss zur
Wiese sowie Übergänge von Fels zu Waldboden und Feuchtgebiet. Die vorhandenen
ComfyUI-Bodenmaterialien werden weiterverwendet. Die Wasserformen sind im
Pixelraster erzeugt; für diese Runde wurde kein neues KI-Rohbild generiert.

## Lieferumfang

- 16 Wasserfall-PNGs: schmal/breit, linke/rechte sichtbare Hangseite, vier Frames.
- Vier längs kachelbare Wasserlauf-PNGs, 16×32 px, Wiederholung alle acht Zeilen.
- Vier Szenen mit JSON-Prüfbelegungen, Standbild und animierte Gesamtübersicht.
- Eigenes Zusatzmanifest unter `assets/nature/mountain-water/manifest.json`.

Alle Einzelassets haben höchstens sechs Farben, harte Transparenz und sRGB.
Animation: vier Frames zu 400 ms, Schleife 1,6 s. Wasserfallanker [8,0] an der
Oberkante; die Vorlage enthält 28 Pixel Fallstrecke plus Aufprallsaum. Der
Prüfrenderer passt ihre Höhe mit nächstem Nachbarn an die jeweilige Fallhöhe an.
Die beiden Hangseiten behalten Licht von rechts oben. Hintere, verdeckte
Hangseiten erhalten keine gesonderten Sprites.

Die bisherigen `water/water_stream_0.png`-Prototypen bleiben historische Assets;
der neue animierte Satz steht ausschließlich im Zusatzmanifest. Keine identischen
Bildkopien werden angelegt. Die Kurven im Szenenrenderer werden geometrisch
aus den Nachbaranschlüssen gezeichnet, der gerade Streifensatz ist separat für
die spätere Renderer-Einbindung bereitgestellt.

## Anschlussregeln und Grenzen

Wasser nur auf Feldern mit Durchfluss; Kaskaden nur zum durchflossenen, tieferen
Nachbarn. Der Ufersaum mischt feuchten Fels und Erde nach vorhandenem Felsanteil.
Die synthetischen Prüfwerte `height_px` und `visual_width` ändern keine Simulation.
Die beiden Beispielbreiten sind keine neue Trennung zwischen Bach und Fluss im
Kern. Die Integration muss Höhe, Breite und Nachbarschaft aus realen Werten liefern.
In den ruhigen Zeitstufen kann Frame 0 verwendet werden; die endgültige zeitliche
Umschaltung bleibt beim Renderer. Quellenmarker sind nicht Teil dieser Runde.

## Prüfung und Herstellung

`python tools/nature/build_mountain_water.py` in der bisherigen Pillow/NumPy-
Umgebung erzeugt das Paket. Prüft Maße, maximal sechs Farben, harte Transparenz,
vier verschiedene Frames und höchstens 15 Prozent Helligkeitsschwankung an jedem
sichtbaren Pixel. Dazu: reziproke Nachbarn, abwärts gerichtete Fließstrecke,
Rautengrenzen, kein Wasser ohne Durchfluss, kein Fall ohne Gefälle sowie positive
Fallprobe. Die GIF-Datei wird auf vier Frames zu je 400 ms geprüft.

Das Prüfblatt wurde visuell kontrolliert. Die Grafiken sind zur Übernahme bereit;
die Prüfung im Spiel mit echten Höhendaten, Zoom und Verdeckung durch Vegetation
bleibt offen. Es werden keine Änderungen unter `crates/` vorgenommen.
