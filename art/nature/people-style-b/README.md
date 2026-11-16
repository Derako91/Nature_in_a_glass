# Menschen im neuen Stil – Schritt 2

Basis `73ccfd6` (v0.8.3), Zweig `codex/people-style-b`. Dieses Paket stellt
alle noch alten eingebetteten Menschenbilder auf den Stil der Holztragebilder
um. Der separate Feuerstellen-PR #14 lässt sich unabhängig davon übernehmen.

## Vollständiger Bildsatz

- Erwachsene: fünf Aussehen, beide Richtungen, `idle`, `walk`, `run`, `gather`,
  `eat`, `drink`, `sleep`, `hurt` – 230 neue Phasen.
- Kinder: dieselben fünf Haar-/Erkennungsfarben, beide Richtungen, `idle`,
  `walk`, `gather`, `eat`, `drink`, `sleep` – 150 neue Phasen.
- Tote: zehn neue liegende Bilder, beide Richtungen je Aussehen.
- Die 40 vorhandenen `_build`-Bilder erhalten nur zusätzlichen transparenten
  Seitenrand. Ihre bisherigen 56×120 Pixel bleiben unverändert in der Mitte.
- Der weiße `person_marker_0` bleibt bytegleich; der Beobachter färbt ihn
  weiterhin mit den bestehenden Erkennungsfarben.

Damit enthält der eingebettete Menschenordner 431 PNGs. Es fehlen keine
Manifestdateien oder Phasen; es gibt keine Bild-Aliase oder gespiegelten
Richtungen. Die fünf Aussehen und ihre Erkennungsfarben wurden nicht geändert.
Kinder verwenden eigene Bilder; altersspezifische Säuglingsbilder gehören
weiterhin nicht zur bestehenden Schnittstelle.

## Größe, Fußpunkte und Proportionen

Die **sichtbare Figur** bleibt an den bisherigen Holztragebildern ausgerichtet.
Für ausgestreckte Beine, Arme und liegende Posen sind die Leinwände doppelt so
breit. Das zugehörige `display_size` verdoppelt nur diese transparente Breite;
Kinder behalten acht Dateipixel je Welteinheit. Erwachsene einschließlich Toten
werden nach der [Größenkorrektur](../people-hut-adjustments/README.md) auf 90 %
dargestellt; dafür gelten rund 8,89 Dateipixel je Welteinheit. Kein Motiv wird auf eine bestimmte Breite/Höhe verzerrt.

| Gruppe | Leinwand | `display_size` | Fußpunkt |
| --- | --- | --- | --- |
| Erwachsene, einschließlich `_build` | 112×120 | 12,6×13,5 | 56/112 |
| Kinder | 80×80 | 10×10 | 40/76 |
| Tote | 120×40 | 13,5×4,5 | 60/20 |
| Überblickszeichen | 16×16, unverändert | bestehende feste Bildschirmgröße | unverändert |

Pro Aussehen/Richtung gibt es einen proportionalen Faktor für sämtliche
ursprünglichen Tätigkeiten und Phasen, an der jeweiligen Holztragehöhe kalibriert.
Die überarbeiteten kompakten Posen werden separat am Stehen kalibriert. Separat
generierte Richtungen werden so trotz unterschiedlicher Quellgröße gleich hoch.
Kinder haben einen gemeinsamen proportionalen Faktor je Aussehen. Die bekannten
Animationstakte kommen unverändert aus dem Beobachter, Animationen laufen nur
bei dessen vorgesehener Zeitstufe.

## Quellen und erneuter Export

`sources/` enthält die 15 mit dem Bildgenerator erzeugten Ausgangsbögen:
je Aussehen zwei Erwachsenenbögen und ein Kinderbogen. Vorhandene Holztragebögen
dienten als Referenz. Vier Quellen hatten einen sichtbaren Hintergrund; dieser
wurde mit dem Bildgenerator entfernt. Die verwendeten endgültigen Bögen und
ihre SHA-256-Hashes stehen in `sources.json`. Bei den Kinderbögen werden die
ersten 30 Motive verwendet; zwei zusätzliche Standbilder bleiben ungenutzt.

```sh
python tools/nature/export_people_style_b.py
```

Benötigt Pillow, NumPy und SciPy wie die bestehenden Sprite-Werkzeuge.
Der Export erkennt ganze zusammenhängende Figuren, ordnet sie zeilenweise
nach Bodenhöhe und dann links→rechts. Er schneidet nicht an angenommenen
Rastergrenzen durch Füße oder Köpfe. Schwache Alpha-Reste außerhalb der
Silhouette werden entfernt, die Motive proportional skaliert und an festen
Fußpunkten platziert. Exportnachweise stehen in `export.json`.

## Ansehen und prüfen

- `person-contact.png` und `person_child-contact.png`: alle Aussehen,
  Zustände und Richtungen als synthetischer Kontaktbogen.
- `person-*.gif` und `person_child-*.gif`: synthetische Phasenvergleiche.
- `assets/nature/_people-testsheet.png`: synthetischer Größenvergleich von
  Stehen/Gehen mit Hirsch und Jungbaum bei Zoom 4/8/32. Dieser Bogen stellt
  Größen dar; die tatsächliche LOD-Auswahl wird im Spiel geprüft.
- `vorher-*.png` / `nachher-*.png`: tatsächliche Vorher-/Nachher-Aufnahmen,
  gleiche Simulation und Kamera, direkt eingebettete Bilder ohne `--bilder`.

Grundlage der Spielbilder:

```sh
nature-in-a-glass --seed 2024 --years 10 --menschen --danach 3 --select 19,34 --zoom 32
nature-in-a-glass --seed 2024 --years 10 --menschen --danach 3 --select 16,35 --zoom 8
nature-in-a-glass --seed 2024 --years 10 --menschen --danach 3 --days 1 --select 19,34 --zoom 32
```

Für den gezielten Erwachsenen-/Kindervergleich wurde ein Tag weiter simuliert:
Nils steht dann tatsächlich mit einer sammelnden Erwachsenen auf dem freien
Lagerfeld 19/34. Beide werden dort bei Zoom 4/8/32 vor und nach der Umstellung
gezeigt. Auf dicht bewachsenen Feldern können Figuren weiterhin von vorhandenen
Bäumen verdeckt werden; dafür wurde weder Wald entfernt noch der Renderer
geändert.

## Abnahme

Die Dateiprüfung kontrolliert alle 431 PNGs, transparente Ränder, vollständige
Manifestverweise und unterschiedliche Phasen. Alte und gepolsterte `_build`
werden pixelgenau verglichen. Die endgültige gestalterische Freigabe bleibt
offen: vor allem Schleifenruhe, kleine Frisur-/Faltenunterschiede zwischen
separat erzeugten Richtungen und die Lesbarkeit der Kinder im laufenden Spiel.
Rare Zustände wie Tod und Verletzung werden im Kontaktbogen gezeigt; eine
natürliche Spielsituation für jeden einzelnen Zustand wird nicht behauptet.

## Technische Ergebnisse

Formatprüfung, strenges Clippy, Release-Bau, 89 Rust-Tests und 15 Python-Tests
bestanden. Alle 431 PNGs und Manifestverweise geprüft. Wiederholter Export
erzeugt dieselben PNGs und dasselbe Manifest bytegenau. Die gerenderten
Spielbilder verwenden die direkt eingebetteten Dateien ohne `--bilder`.

`spiel-animation.gif` zeigt tatsächliche Aufnahmen bei 1 Std/s.
`spiel-abend.png` zeigt die sammelnde Erwachsene beim Essen (19 Uhr);
`spiel-nacht.png` zeigt Erwachsene und Kind schlafend (22 Uhr), nach natürlichem
Zeitablauf und anschließendem Pausieren per Leertaste.
