# Runde 04 – Bewegung und Fellvarianten

Auf Grundlage der vom Projektinhaber positiv bewerteten Tierformen ergänzt
diese Runde die beiden Bewegungszyklen und eine zweite Fellvariante.

## Ergebnis

- Hirsch, Hirschkalb, Wolf, Fuchs und Kaninchen, jeweils links/rechts und in
  zwei fest auswählbaren Fellvarianten.
- Fressen: zwei Bilder, 600 ms je Bild, 1,2 Sekunden pro Schleife.
- Gehen: vier unterschiedliche Bilder, 200 ms je Bild, 0,8 Sekunden pro Schleife.
- Laufen: vier unterschiedliche Bilder, 100 ms je Bild, 0,4 Sekunden pro Schleife.
- Ruhen: ein Bild.
- Insgesamt 220 Bildzuordnungen; identische Posen werden gemäß Abschnitt 9.1
  nur einmal als PNG gespeichert. `frame_map` im Manifest ordnet jede Kombination
  aus Variante, Richtung und Frame einer Datei zu. `frame_aliases` dokumentiert
  gemeinsam genutzte Bilder. Dateinamen dürfen daher nicht blind konstruiert werden.

Die Bewegung arbeitet mit den redigierten Pixelkonturen der vorhandenen lokalen
ComfyUI-Studien. Für diese Fortsetzung wurden keine neuen großen KI-Rohbilder
benötigt. Die Richtungen behalten unabhängige Konturen und festes Licht von rechts.

## Vorschauen

- `assets/nature/_animals-motion-grass.gif`: alle Zyklen und Varianten auf Gras.
- `assets/nature/_animals-motion-shrub.gif`: dieselbe Prüfung auf Buschboden.
- `assets/nature/_animals-motion-forest.gif`: dieselbe Prüfung auf Waldboden.
- `assets/nature/_animals-frames.png`: die vier Phasen nebeneinander.
- `assets/nature/_animals-testsheet.png`: aktualisierte Formenübersicht.
- `assets/nature/_animals-graze.gif`: aktualisierte Fressschleife.

Alle drei Bewegungs-GIFs enthalten acht Bilder zu jeweils 100 ms: Die Gehphasen
halten je zwei Bilder, die Laufphasen wechseln jedes Bild und wiederholen sich
zweimal. Damit lassen sich beide Geschwindigkeiten im selben Blatt vergleichen.

## Verbesserungen und Prüfung

Füchse erhalten eine deutlichere helle Schwanzfläche; Kaninchen zeigen wechselnde
Ohr- und Sprungformen. Die Maße bleiben unverändert. Die Erkennbarkeit bei nur
50 Prozent Größe bleibt ein offener visueller Punkt, insbesondere bei 3×3 Pixeln.
Es wird keine bestandene vollständige Formenabnahme behauptet.

Für alle 220 Zuordnungen prüft das Skript Maße, maximal sechs Farben und Alpha
0/77/255. Alle 40 Geh-/Laufzyklen enthalten vier tatsächlich unterschiedliche
Pixelbilder. Die Silhouettenänderung beim Übergang von Bild 3 zu Bild 0 darf
nicht mehr als zwei Pixel über dem stärksten inneren Übergang liegen. Die
Anker sind pro Art über sämtliche Frames und Varianten konstant.

Der mittlere absolute Helligkeitsunterschied zu je einem festen Gras-, Busch-
und Waldboden-Ausschnitt liegt bei mindestens 15 Prozent. Diese Materialprobe
ersetzt nicht die spätere Sichtprüfung bei wechselndem Zoom und Beleuchtung im
Spiel. Die Phasen wurden zusätzlich als Kontaktblatt visuell geprüft und die
GIF-Bildzahlen und Zeitangaben nach dem Speichern kontrolliert.

Neu bauen: `python tools/nature/build_animal_cycles.py` mit der bestehenden
ComfyUI-Python-Umgebung (Pillow/NumPy). Das Skript benötigt keinen laufenden
ComfyUI-Dienst. Der frühere `build_animal_proof.py` bleibt die dokumentierte
erste Runde; für den aktuellen Stand ist das neue Skript maßgeblich.

Kadaver, Spuren und Spielintegration sind weiterhin offen und nicht Teil dieser Runde.
