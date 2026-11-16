# Gärten im Stil B – Gestaltung 17.9

Basis `25ea9cb` (v0.10.2, inklusive Fischfang aus PR #22), Branch `codex/garden-style-b`.
20 neue Bilder in `assets/nature/ground/`, fünf Zustände × vier Varianten:

| Zustand | Bildinhalt |
| --- | --- |
| `bare` | dunkle umgegrabene Erde, sechs durchgehende Furchen |
| `sprout` | kleine helle Keimlinge, viel freie Erde |
| `growing` | dichte grüne Reihen, breite Knollen-/Bohnenblätter und Samengräser |
| `ripe` | volle Pflanzen, goldene Samenstände, gelblich werdende Blätter |
| `stubble` | abgeerntete Stängel, Blätter- und Strohreste, wieder viel freie Erde |

Die vier Varianten unterscheiden sich in Pflanzung, Bodenstruktur und
Mischung aus Blattgemüse/Bohnen/Samengräsern. Gleiche Reihenrichtung und
Wachstumsfolge, keine modernen Geräte, keine Zäune oder Mauereinfassungen.
Alle bestehenden Bilder bleiben unverändert. Die Simulation wählt Zustand und
Variante bereits selbst; hier werden keine Gärten oder Erntewerte verändert.

## Quelle und Projektion

Ein neu generierter, archivierter Materialatlas mit vier Spalten und fünf
Zeilen (`sources/materials.png`) auf Basis des vorhandenen Landschaftsstils.
Sommerbewuchs und frische Keimlinge wurden im Bildgenerator bewusst grüner
ausgearbeitet, damit sie sich auch im Überblick von der goldenen Reifereihe
unterscheiden. Keine
zufällig umgefärbten Phasen oder wiederverwendeten Wildgrasbilder.

Die flachen Materialquadrate werden auf die feste isometrische Bodenraute
projiziert. 128×64 RGBA, Anker 64/32, Weltmaß 64×32, Auflösung 2 px/Welteinheit.
Die ganze Leinwand ist eine Bodenraute; das Beet liegt in ihren inneren 90 %,
mit schmalem weichem Übergang zur vorhandenen Bodenoberfläche. Außerhalb der
Raute und des Beets bleibt das Bild transparent. Sämtliche 20 Bilder besitzen
dieselbe Alpha-Maske; beim Zustandswechsel springt weder Lage noch Rand.

Das eigene `ground/manifest.json` wird wie die anderen Gruppenmanifeste
eingebettet. Einträge `garden_<zustand>`, je vier Varianten, ein Standbild.
Die bestehende Renderer-Schnittstelle wählt `garden_<zustand>_<variante>`;
keine Produktionsänderung an Kern oder Renderer.

[Kontaktbogen aller tatsächlichen PNGs](contact.png), keine Spielaufnahme.
Quellprüfsumme und Projektionsparameter in `config.json`, Dateiprüfsummen und
Quellausschnitte in `export.json`, Prüfergebnisse in `validation.json`.

## Wiederholen

```sh
python tools/nature/export_gardens.py
python tools/nature/check_gardens.py
```

Pillow/NumPy nötig. Gartenexport und vollständiger Landschaftsexport liefern
bei Wiederholung bytegleiche PNGs und Manifeste. Der Landschaftsexport ruft
am Ende den Gartenexport auf. Alle 1373 bisherigen PNGs bleiben bytegleich.

## Tatsächlicher Spielvergleich

Vergleich auf `cc542e8` (v0.10.1), vor dem reinen Schnittstellen-Update v0.10.2:
Elternstand mit Platzhalterbeeten gegen den Release-Bau mit
vollständig eingebetteten Gartenbildern, ohne `--bilder`. Gleiche Simulation,
Seed, Datum, ausgewähltes Feld, Blick und Vergrößerung, Software-OpenGL/Xvfb
1440×900. Sommer/Ernte wie beauftragt, zusätzlich Nahansicht:

```sh
target/release/nature-in-a-glass --seed 300 --years 10 --menschen --danach 20 --days 200 --select 25,20 --zoom 8
target/release/nature-in-a-glass --seed 300 --years 10 --menschen --danach 20 --days 260 --select 25,20 --zoom 8
```

Ohne `--speed` beginnt der Beobachter pausiert. Beide Aufnahmen zeigen Jahr 31,
10:00 Uhr; +200 Tage entspricht Anzeige Tag 201, +260 Tage Anzeige Tag 261.

| Fall | Platzhalter | Eingebettete Gartenbilder |
| --- | --- | --- |
| Sommer, Zoom 8 | [Vorher](vorher-tag200-zoom8.png) | [Nachher](nachher-tag200-zoom8.png) |
| Sommer, Zoom 32 | [Vorher](vorher-tag200-zoom32.png) | [Nachher](nachher-tag200-zoom32.png) |
| Ernte, Zoom 8 | [Vorher](vorher-tag260-zoom8.png) | [Nachher](nachher-tag260-zoom8.png) |
| Ernte, Zoom 32 | [Vorher](vorher-tag260-zoom32.png) | [Nachher](nachher-tag260-zoom32.png) |

Die Kontaktbilder prüfen alle fünf Stadien und vier Varianten. Nicht jeder
Zustand muss gleichzeitig im selben Spielstand vorkommen; tatsächliche
Ernte und Übergänge bestimmt der Kern. Im vorgegebenen Herbstfall sind die
Beete noch golden/reif und der Lagervorrat voll; dieser Vergleich ist kein
Nachweis gleichzeitig sichtbarer Stoppeln. Visuelle Abnahme durch Claude offen.


## Ausgeführte Prüfungen auf der aktualisierten Basis v0.10.2

`cargo fmt --all`, `cargo clippy --release --all-targets -- -D warnings`,
`cargo test --release --workspace` (101 bestanden),
`cargo build --release -p nature-in-a-glass` bestanden. Zusätzlich 15 bestehende
Python-Tests (`tools/sprite_agent/tests`) und `check_gardens.py` bestanden.
Der gemeinsame Vollatlas einschließlich Fischfangbildern liegt innerhalb
8192 px. Garten- und vollständiger Landschaftsexport wurden bytegleich
wiederholt; alle 1373 Eltern-PNGs blieben unverändert.

Sommer/Herbst wurden im tatsächlichen Release-Spiel ohne Bildüberschreibungen
bei Zoom 8/32 verglichen. 128×64 ist die kompakte Bodenauflösung mit 2 px je
Welteinheit; in der Nahansicht wird die Textur entsprechend vergrößert. Alle
fünf Zustände stehen zusätzlich im Kontaktbogen zur visuellen Abnahme bereit.

Nach dem Aktualisieren auf die Wasserfall-Schnittstelle v0.10.2 erneut ohne
`--bilder` im Spiel geprüft: [Sommer, Zoom 8](v0102-tag200-zoom8.png),
[Herbst, Zoom 8](v0102-tag260-zoom8.png). Die vorherigen Vergleichspaare bleiben
als exakt gleicher v0.10.1-Stand vor/nach dem Bildaustausch erhalten.
