# Breite Fernkronen für Wald und Buschland

Basis `d6fc0bf` (PR #17 zusammengeführt, weiterhin v0.8.5). Die Nahansicht von
PR #17 wurde positiv beurteilt; die Freigabe der Landschaft ist wegen zu
schmaler, lückiger Wald-/Buschsilhouetten bei Zoom 4/8 noch offen.

32 eigene `@fern`-Bilder ergänzen Jung-, Mittel-, Altbaum und Busch: je vier
Varianten in normalem und trockenheitsgestresstem Zustand. Breite geschlossene
Kronen, zusammenhängende Licht-/Schattenflächen und kurze erkennbare Stämme
ersetzen die feinen freistehenden Äste in der Fernansicht. Die Baumanzahl und
sämtliche Vegetationswerte der Simulation bleiben gleich.

Alle 955 vorhandenen PNGs bleiben bytegleich, insbesondere die positiv
beurteilten Nahbilder sowie Böden und Wände. Das Hauptmanifest erhält nur vier
zusätzliche `levels`-Einträge. Fernbilder haben ein Viertel der Breite/Höhe und
Ankerkoordinaten der Nahbilder (zwei Dateipixel je Welteinheit). Damit stimmen
Leinwand-Weltmaß und Fußpunkt über den Stufenwechsel hinweg exakt überein.

## Auswahl im echten Atlas

Der vorhandene Atlas wählt `@fern` bei Zoom 2, 4 und 8; bei Zoom 16 verwendet er
die berechnete Zwischenstufe der Nahfassung und bei Zoom 32 deren Hauptbild.
Dafür ist keine Änderung des Renderers nötig. Ein Regressionstest baut den
Atlas aus den tatsächlich eingebetteten Pflanzenbildern und prüft alle 32
Normal-/Trockenheitsbilder bei fünf Zoomstufen: richtige Detailstufe, Weltmaß
und Fußpunkt. Name: `embedded_vegetation_far_crowns_follow_zoom_and_anchor`.

## Prüfpaket

[Fern-Kontaktbogen](fern-contact.png), vergrößert, keine Spielaufnahme.

Tatsächliche Vorher-/Nachher-Aufnahmen der beanstandeten Stelle:

```sh
target/release/nature-in-a-glass --seed 2024 --years 10 --menschen --danach 3 --days 120 --select 19,34 --speed 0 --zoom 8
```

| Zoom | PR #17, schmale Kronen | Mit eigenen Fernkronen |
| --- | --- | --- |
| 4 | [Vorher](vorher-zoom4.png) | [Nachher](nachher-zoom4.png) |
| 8 | [Vorher](vorher-zoom8.png) | [Nachher](nachher-zoom8.png) |
| 32 | [Vorher](vorher-zoom32.png) | [Nachher](nachher-zoom32.png) |

Zusätzlich alter Wald: Seed 42, `--years 80 --select 25,11 --speed 0`:
[Zoom 8 vorher](vorher-wald-zoom8.png) / [nachher](nachher-wald-zoom8.png).

Xvfb 1440×900, Software-OpenGL, mit eingebetteten Bildern ohne `--bilder`.
Für vergleichbare Beleuchtung je Vorher-/Nachher-Paar zur gleichen angezeigten
Spielzeit pausiert. Die Vorher-Aufnahmen bei Zoom 4/8/32 und im alten Wald
stammen aus dem unveränderten Prüfpaket von PR #17.

[Direkter Dreiervergleich bei Zoom 8](vergleich-zoom8.png): Ausschnitte der
tatsächlichen Spielaufnahmen vor der Landschaftsumstellung, aus PR #17 und mit
den neuen Fernkronen. Alle drei Aufnahmen zeigen dieselbe Stelle um 12:00 Uhr.
Visuelle Abnahme der neuen Fernkronen durch Claude offen.

## Wiederholen

Mit Pillow, NumPy, SciPy und den archivierten Quellen:

```sh
python tools/nature/export_landscape_far.py
python tools/nature/check_landscape_far.py
```

Auch `export_landscape_style_b.py` wendet die Fern-Ergänzung nach seinem
Hauptexport wieder an. Beide Exportwege liefern beim Wiederholen dieselben
PNG-Dateien und dasselbe Hauptmanifest. `config.json` enthält Quellprüfsummen;
`export.json` dokumentiert Dateimaße/Anker, `validation.json` die Prüfungen und
Silhouettenflächen im gleichen Weltmaß. Flächen sind farbunabhängig mit Alpha
gewichtet, keine Schätzung zusätzlicher Bäume oder Vegetationsmenge.

Ausgeführt: 90 Rust-Tests, 15 Python-Tests, Formatprüfung, strenges Release-Clippy
und Release-Bau bestanden. Exportwiederholung beider Werkzeuge bytegleich;
alle 955 vorhandenen PNGs unverändert. Seltene Trockenheitsvarianten vollständig
auf dem Kontaktbogen und im eingebetteten Auswahltest geprüft, nicht sämtlich
natürlich im Spiel ausgelöst.
