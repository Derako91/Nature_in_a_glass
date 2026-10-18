# Runde 09 – Tiere für die Nahansicht

Erster Auftrag aus der aktualisierten Gestaltungsvorlage 16.3: alle lebenden
Tiere neu zeichnen, in doppelter Auflösung gemäß 4.3 und 15.3. Hirsch 28×24,
Kalb 18×16, Wolf 26×16, Fuchs 20×12, Kaninchen 12×10 Pixel. Das sind neue
Konturen mit zusätzlichen anatomischen Details, keine vergrößerten alten PNGs.
Die vorhandenen ComfyUI-Studien bleiben die Form-/Farbvorlage; keine neue
KI-Generierung. Die Konturen sind als bearbeitbare Pixelzeichenanweisungen in
`tools/nature/build_animal_detail.py` hinterlegt.

## Inhalt und Einbindung

220 Bildzuordnungen: beide Richtungen, zwei Fellvarianten, Fressen (2), Gehen
(4), Laufen (4), Ruhen (1). Alle 220 Bilder unterscheiden sich; es sind in diesem
Satz keine Alias-Verweise nötig. Gleiche Dateinamen wie bisher, Manifest mit
Bildmaßen und Fußpunkt in der neuen Auflösung. Alle Frames einer Art haben
denselben Fußpunkt, je Art [Breite/2 abgerundet, Höhe−2]. Zeiten unverändert:
Fressen 600 ms je Frame, Gehen 200 ms, Laufen 100 ms.

Der aktuelle Renderer skaliert bereits nach Referenzbreite. Deshalb sind keine
Änderungen unter `crates/` nötig. Hintergrund-, Vegetations- und Wasserbilder
bleiben im bisherigen Grundmaß. Die neue Auflösung gilt nur für lebende Tiere.
Kadaver und Spuren folgen ihrem eigenen Auftrag und werden nicht stillschweigend
mit einer Tieranimation überschrieben.

## Prüfung

Maße, maximal sechs RGB-Farben inklusive Schatten, Alpha 0/77/255, mindestens
15 Prozent mittlerer absoluter Helligkeitskontrast auf festen Gras-, Busch- und
Waldbodenproben sowie vier unterschiedliche Bilder in allen 40 Bewegungszyklen
werden beim Bau geprüft. Phasenänderungen stehen in `animals/validation.json`.
Beleuchtung wird nach der Kontur rechts oben gesetzt, nicht mit dem Tier gespiegelt.

`_animals-detail-comparison.png` zeigt die Rastergrößen bei Zoom 2, 4 und 8 und
einen Detailvergleich mit dem Hauptzweig-Stand `2babb42`. Der alte Vergleich wird
direkt aus Git gelesen, ohne zusätzliche Quellbildkopien. Die beiden Seiten des
Detailvergleichs haben gleiche Breite, das jeweilige Seitenverhältnis bleibt erhalten.
`_animals-detail-states.png` zeigt Fressen, Ruhe und Bewegung in beiden Richtungen.
`_animals-detail-motion.gif` zeigt beide Fellvarianten mit korrektem Gehen/Laufen.

Diese Prüfbilder sind keine Bildschirmfotos aus dem Beobachter. Hier fehlen eine
lokale Rust-Bauumgebung und ein neu gebauter Beobachter mit den neuen eingebetteten
Bildern. Die in 16.4 geforderte Vorher-/Nachher-Spielaufnahme bleibt ausdrücklich
offen. Formen bei Zoom 4/8 visuell überprüft; bei Zoom 2 bleiben die Tiere klein.
Es wird keine abgeschlossene Spielabnahme bei allen Beleuchtungen behauptet.

Neu bauen: `python tools/nature/build_animal_detail.py`. Auch der bisherige
Einstieg `build_animal_cycles.py` ruft jetzt diesen aktuellen Bau auf. Ältere
Prüfblätter in Runde 03/04 bleiben historische Dokumentation. Nächste Priorität
aus 16.3 ist Feuer; diese Runde schließt nur den ersten Grafikauftrag ab.
