# Runde 05 – Biome und Übergänge

Die vorhandenen lokalen ComfyUI-Materialvorlagen werden räumlich zusammenhängend
abgetastet und mit nachbarabhängigen Pixelmasken verbunden. Neue Rohbildkopien
sind dafür nicht nötig. Die Vegetation verwendet feste, unterschiedlich verteilte
Positionen statt identischer Reihen auf jedem Feld.

Die sechs Einzelproben zeigen Wald mit Lichtung, Wiesenland mit Bach,
trockenes Buschland, Steppe, Feuchtgebiet und felsiges Bergland. Eine zusätzliche
12×12-Prüflandschaft verbindet feuchte und trockene Bereiche, Waldsäume und Fels.
Zwölf harte 64×32-Übergangsmasken liegen unter `assets/nature/transitions/`.
Die dortigen JSON-Dateien enthalten synthetische Testbelegungen, keine Spielstände.
Das Manifest beschreibt Masken und Wassergrenzen. Zusammengesetzte Vorschauen
enthalten mehrere Materialpaletten; die Masken selbst verwenden jeweils eine Farbe.

Wasserflächen werden nur innerhalb von Feldern mit `surface_water > 150`
gezeichnet, Bachlinien bei `flow >= 60`. Vegetationsdichte und Bodenmischung
folgen den Testwerten. Die Höhen sind ausschließlich Prüfwerte für die Darstellung.
Es werden keine Simulationsregeln verändert und keine neuen Biomtypen eingeführt.

Erneut erzeugen: `python tools/nature/build_biome_transitions.py` mit Pillow und
NumPy in der vorhandenen ComfyUI-Python-Umgebung. Kein laufender KI-Dienst nötig.
Die beiden Prüfblätter wurden visuell kontrolliert. Masken werden beim Bau auf
harte Transparenz und Einhaltung der Rautengrenzen geprüft.

Offen bleiben die Einbindung in den Renderer, Prüfung mit echten Nachbarwerten
und Zoomstufen sowie organischere Höhenkonturen und Bachkurven. Die sichtbaren
Felsstufen und Bachsegmente sind Gestaltungsprototypen, keine fertige Geländeengine.
