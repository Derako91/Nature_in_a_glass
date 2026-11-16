# Hütten: Proportionen korrigiert

Die Exportoption `fit = (120, 232)` streckte beide Hütten unabhängig von der
Form im Quellbogen. Sie ist entfernt. Beide Achsen verwenden denselben Faktor;
die Leinwandhöhe ergibt sich aus der Quellform. Weltbreite 20 gemäß Dennis'
Korrekturauftrag, Fußpunkt unten mittig mit vier Pixeln Rand.

| Motiv | Quellausschnitt | Datei | Weltmaß | Fußpunkt |
| --- | --- | --- | --- | --- |
| Hütte 0 | 330×375 | 160×186 | 20×23,25 | 80/182 |
| Hütte 1 | 345×358 | 160×170 | 20×21,25 | 80/166 |

`source-camp-atlas.png` ist der unveränderte ursprüngliche Produktionsbogen
(Hash identisch zum ersten Lagerexport). Keine neuen Motive oder Änderungen
am Spielcode. Die übrigen 47 Einzelbilder sind nach Hashvergleich unverändert.

Erneut exportieren, vom Repository-Hauptordner:

```sh
python tools/nature/export_camp_sprites.py art/nature/camp-hut-proportions/source-camp-atlas.png --huts-only
```

Der gezielte Export aktualisiert nur die beiden Hütten, ihre Manifesteinträge,
Exportnachweise und den Kontaktbogen. Eine Wiederholung ergibt bytegleiche
Hütten und dasselbe Manifest. Der vollständige Erstexport verwendet dieselbe
proportionale Größenberechnung.

## Spielvergleich

`vorher-zoom8.png` / `vorher-zoom32.png`: Ausgangsstand `7b20283`, v0.8.2.
`nachher-zoom8.png` / `nachher-zoom32.png`: derselbe Code mit korrigierten,
direkt eingebetteten Bildern; kein `--bilder`.

```sh
nature-in-a-glass --seed 2024 --years 10 --menschen --danach 3 --select 19,34 --zoom 8
nature-in-a-glass --seed 2024 --years 10 --menschen --danach 3 --select 19,34 --zoom 32
```

Beide Hütten im tatsächlichen Spiel angesehen: breitere natürliche Form,
unterschiedliche Dachformen, weiterhin derselbe Standpunkt zur Feuerstelle.
Die geometrische Prüfung in `validation.json` vergleicht die Proportionen des
Quellausschnitts mit dem sichtbaren exportierten Motiv; Abweichungen bleiben
unter zwei Pixeln Rundungstoleranz. Die endgültige gestalterische Abnahme bleibt
bei Claude/Dennis. Frühere Spielbilder im Ordner `camp-review` dokumentieren
den damaligen Stand; die aktuellen Vergleiche liegen hier.

## Prüfungen

Formatprüfung, strenges Clippy und Release-Bau bestanden.
`cargo test --locked --release --workspace`: 89 Tests bestanden, keine Fehler.
Gezielter Wiederholungsexport bytegleich; vollständiger Erstexport im
ignorierten Prüfverzeichnis liefert dieselben Hüttenpixel.
