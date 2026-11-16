# Lagerplätze und Holztragen – Draft im neuen Stil

Erster Schritt der von Dennis beauftragten schrittweisen Umstellung aller
Spielbilder auf Stil B. Vorgabe: `docs/gestaltung.md` 17.8; Ausgangsstand
`8218fc2` (alle Ordner-Manifeste werden eingebettet). Branch `codex/camp-sprites`.

## Inhalt

- `assets/nature/buildings/`: kalte Feuerstelle, vier Feuerphasen, Windschutz,
  zwei Hüttenvarianten und Ruine; eigenes `manifest.json`.
- `assets/nature/people/`: 40 neue `person_<aussehen>_build_<l|r>_f0…f3.png`,
  fünf Aussehen, zwei eigens erzeugte Richtungen, vier Phasen à 250 ms.
  Der Zustand zeigt Holztragen, passend zur tatsächlichen Tätigkeit im Kern.
- `einzelbilder.png`: synthetischer Kontaktbogen, `build.gif` und
  `camp-fire.gif`: synthetische Animationsvorschauen.
- `spiel-*.png` und `spiel-animation.gif`: tatsächliche Spielaufnahmen.
  `concept.png` bleibt als ursprüngliche Stilvorlage erhalten.

Die neuen Dateien sind direkt für das Einbetten vorbereitet. Kein Renderer,
keine Simulation und keine bestehende Tätigkeit wurden geändert. Der PR bleibt
bis zur gestalterischen Abnahme ein Draft. Vorräte sind laut 17.8 optional und
nicht enthalten; ein eigenes `camp_hut_ruin` gehört ebenfalls nicht zu diesem
Paket. Der Beobachter zeichnet alte Hütten beim Verfall selbst niedrig/dunkel.

## Maße und Ruhe

| Motiv | Dateimaß | Weltmaß | Fußpunkt in Dateipixeln |
| --- | --- | --- | --- |
| kalte Feuerstelle | 48×32 | 9×6 | 24/30 |
| Feuer, alle vier Phasen | 48×56 | 9×10,5 | 24/52 |
| Windschutz | 96×112 | 12×14 | 48/108 |
| Hütte 0 | 160×186 | 20×23,25 | 80/182 |
| Hütte 1 | 160×170 | 20×21,25 | 80/166 |
| Ruine | 48×40 | 9×7,5 | 24/38 |
| Menschen `_build` | 56×120 | 7×15 | 28/112 |

Die Hütten werden proportional aus dem Quellmotiv skaliert. Die frühere
Normierung auf 120×232 verzerrte beide Motive und ist entfernt. Weltbreite 20,
Leinwandhöhe aus dem echten Seitenverhältnis, Fußpunkt unten mittig.
Vorher-/Nachher-Spielaufnahmen bei Zoom 8/32 und der reproduzierbare Quellbogen
liegen unter [camp-hut-proportions](../camp-hut-proportions/README.md).
Die Menschen haben je Aussehen einen gemeinsamen Skalierungsfaktor für alle
acht Bilder; kein Aufblasen einzelner Posen. Erkennungsfarben und Haarformen
entsprechen den fünf bestehenden Aussehen. Linke Figuren wurden separat
generiert, nicht gespiegelt. Fußpunkte bleiben über sämtliche Phasen fest.

Das Lagerfeuer verwendet denselben kalten Steinring in allen Phasen und vier
verschiedene Flammen aus dem bereits übernommenen Bildgenerator-Feuerpaket
(PR #10). So bleibt der Steinring ruhig, während nur die Flamme wechselt.
Die integrierte Helligkeit schwankt maximal um 5,46 %. Animationstakt und
Stillstand im Zeitraffer kommen weiterhin aus dem vorhandenen Beobachter.
Es gibt keine behauptete handgezeichnete Fernfassung: Der Atlas berechnet die
Zwischenstufen; die echten Aufnahmen bei Zoom 4/8/32 zeigen das Ergebnis.

## Tatsächlich im Spiel geprüft

Alle Aufnahmen stammen aus dem neu gebauten Programm **ohne `--bilder`**;
damit sind auch das eingebettete Gebäudemanifest und die Bildsuche geprüft.
Seed 2024, zunächst zehn Jahre Natur, dann Menschen landen lassen:

| Aufnahme | Startoptionen zusätzlich zu `--seed 2024 --years 10 --menschen` |
| --- | --- |
| vorher / nachher Zoom 8 | `--danach 3 --select 19,34 --zoom 8` |
| nachher Zoom 4 / 32 | `--danach 3 --select 19,34 --zoom 4` bzw. `32` |
| Windschutz, noch keine Hütte | `--days 118 --select 19,34 --zoom 32` |
| Holztragen, Aussehen 1 nach links | `--days 118 --select 18,33 --zoom 32` |
| kalte Feuerstelle | `--days 335 --select 19,34 --zoom 32` |
| tatsächliche Animation | wie Windschutz, zusätzlich `--speed 0`; zwölf Aufnahmen im Abstand von 200 ms |

Vorher: aus Commit `8218fc2` separat gebautes Programm. Nachher: neue Assets
in demselben Code. Der Simulationszustand bleibt vergleichbar. Spielstartlogs
waren bei den Bildprüfungen leer. Der virtuelle Bildschirm benötigt in dieser
Cloud `-extension MIT-SHM`, Software-Rendering und einen beschreibbaren Cache.

## Prüfnachweise

- 49 PNGs in RGBA, transparente Außenränder und alle Manifestverweise geprüft.
- Je Richtung/Aussehen vier unterschiedliche Bauphasen, keine Bild-Aliase.
- Vier unterschiedliche Feuerbilder mit unverändertem vorderem Steinring.
- `cargo fmt --all --check`, strenges Clippy und Release-Bau bestanden.
- `cargo test --locked --release --workspace`: 89 bestanden, keine Fehler,
  keine ignorierten Tests; leere Dokumentationstests separat ohne Abdeckung.
- `validation.json` enthält technische Ergebnisse; `export.json` Dateihashes,
  Quellenhashes, Fußpunkte und technische Aufbereitung.

`tools/nature/export_camp_sprites.py` trennt die mit `image_gen` erzeugten
Rasterbögen in Einzeldateien. Die Rohbögen bleiben im lokalen Generatorordner;
die fertigen Spielbilder und Prüfnachweise sind im Repository. Der Export
überschreibt keine bereits vorhandenen Einzeldateien. Für einen erneuten
Erstexport werden ein Zielstand ohne diese neuen Dateien und die drei
Rohbögen benötigt: Lager, Menschen rechts, Menschen links.

## Noch offen / nächste Bildgruppe

Gestalterische Abnahme, Übergang vom neuen `_build` zu den bisherigen
Menschenzuständen und eine länger laufende Sichtprüfung der Animationen.
Die beiden Richtungen sind separat erzeugt; Details wie Holzlage und
Fellkragen können daher leicht variieren. Sie sind als Draft zu beurteilen,
nicht als bereits freigegebener, vollkommen konsistenter Animationssatz.
Die Ruine ist technisch geprüft und auf dem Kontaktbogen sichtbar, wurde aber
nicht durch einen natürlich verlassenen Lagerplatz in einer Spielsitzung geprüft.

Die Zurücknahme von Bäumen und Büschen auf Lagerfeldern wurde von Claude in
v0.8.2 übernommen. Die Proportionskorrektur verändert nur den Bildexport und
die zugehörigen Metadaten. Der nächste Stilblock umfasst die übrigen Zustände aller fünf
Menschen, danach Landschaft/Pflanzen, Tiere und Wasser; siehe Gestaltung 18.
