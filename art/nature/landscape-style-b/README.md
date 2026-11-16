# Schritt 3: Landschaft im Stil B

Basis v0.8.5 (`8043787`), Branch `codex/landscape-style-b`. Entwurf zur visuellen
Abnahme: Böden, Seitenwände und Vegetation; Wasser und Tiere folgen in eigenen
Schritten nach Gestaltung 18.

91 Einzelbilder:

- Neun Bodenmaterialien mit je vier Varianten: Erde, Sand, Fels, feuchtes und
  trockenes Gras, Brandfläche, Busch-/Waldboden und neu angeschwemmter Schlamm.
- Sieben Seitenwandbilder: je zwei Erde/Sand, drei Fels. Der heutige Beobachter
  wählt bei Wänden ausschließlich Variante 0; die übrigen sind vorbereitet.
- 48 Pflanzenbilder: Jung-, Mittel- und Altbaum, Busch, stehendes und liegendes
  Totholz, jeweils vier Varianten und zwei Zustände. Bäume/Büsche normal und
  trockenheitsgestresst, Totholz normal und verkohlt. Diese Zustände fragt die
  bestehende Simulation bereits ab; bisher fiel der Atlas auf Normalbilder
  zurück. Herbstbilder werden noch nicht ausgewählt und gehören nicht zum Paket.

## Gestaltung und Maßstab

Warme natürliche Materialien, plastische Rinde/Blattgruppen, Licht rechts oben.
Bodenmaterialien bleiben kontrastarm; die mittlere Materialfarbe ist am bisherigen
Satz kalibriert, damit die Landschaft kleine Figuren nicht verschluckt. Zusätzliche
Materialdetails stammen aus neu erzeugten Vorlagen, nicht aus Pixelvergrößerung.

Alle bestehenden Weltmaße und Weltfußpunkte bleiben erhalten. Bodenrauten sind
256×128, Wände 128×128, Pflanzen in achtfacher Dateiauflösung. Motive werden
proportional auf ihre Leinwand gebracht. Normale/gestresste beziehungsweise
verkohlte Paare teilen je Variante denselben Skalierungsfaktor. Anker stehen
im Hauptmanifest `assets/nature/manifest.json`. Neue liegende Stämme haben Weltmaß
16×5 und Fußpunkt 8/4; die übrigen Pflanzen behalten ihre bisherigen Weltanker.

Die Bodenmaterialien werden periodisch im UV-Quadrat angelegt, auf die feste
Raute projiziert und teilen ihre Randstruktur zwischen Varianten. Wände sind
oben/unten bytegleich und damit vertikal wiederholbar. Varianten sind fest,
keine Animation und kein zusätzlicher Zufall. Wasserkacheln, Flammen, Menschen,
Gebäude und Tiere bleiben unverändert. Simulation und Renderer bleiben unverändert.

## Wiederholen und prüfen

Mit Pillow, NumPy und SciPy:

```sh
python tools/nature/export_landscape_style_b.py
python tools/nature/check_landscape_style_b.py
python tools/nature/build_landscape_style_b_proof.py
```

`base-manifest.json` enthält den unveränderten Ausgangsstand. `sources/` enthält
nur die endgültig verwendeten Bildgenerator-Quellen; SHA-256 und Materialmittel
stehen in `config.json`. Die drei Materialspalten beziehungsweise neun Materialzellen
werden ohne Beschriftungsränder entnommen. Pflanzen werden freigestellt und
proportional exportiert; bei mittleren/alten Bäumen und stehendem Totholz werden zusammenhängende Motive
statt starrer Rasterzellen ausgeschnitten. Nicht verwendete Quellränder bleiben
außerhalb der exportierten Motive.

[Lesbarkeit auf Erde, Gras und Brandfläche](readability.png), mit Erwachsenen
und unverändertem Kind im tatsächlichen Weltmaß.

Kontaktbögen: [Böden](ground-contact.png), [Wände](walls-contact.png),
[Pflanzen und Zustände](vegetation-contact.png). Diese Bögen sind synthetische
Prüfansichten; tatsächliche Spielaufnahmen sind gesondert dokumentiert.

## Spielvergleich

Die Lagerstelle aus dem letzten Auftrag wird mit eingebetteten Bildern bei Zoom
4, 8 und 32 geprüft:

```sh
target/release/nature-in-a-glass --seed 2024 --years 10 --menschen --danach 3 --days 120 --select 19,34 --speed 0 --zoom 32
```

Zusätzlich alter Wald, Seed 42, 80 Jahre, Auswahl 25/11, Zoom 8 und 32. Für
vergleichbare Bildschirmfotos jedes Vorher-/Nachher-Paar zur gleichen angezeigten
Spielzeit pausiert (11:00 bei Lager-Zoom 4, sonst 12:00 Uhr).
Kein Eingriff in die Bewohner oder Vegetationswerte. Xvfb, 1440×900, Software-OpenGL.

| Szene | Vorher | Nachher |
| --- | --- | --- |
| Lager Zoom 4 | [Bild](vorher-zoom4.png) | [Bild](nachher-zoom4.png) |
| Lager Zoom 8 | [Bild](vorher-zoom8.png) | [Bild](nachher-zoom8.png) |
| Lager Zoom 32 | [Bild](vorher-zoom32.png) | [Bild](nachher-zoom32.png) |
| Wald Zoom 8 | [Bild](vorher-wald-zoom8.png) | [Bild](nachher-wald-zoom8.png) |
| Wald Zoom 32 | [Bild](vorher-wald-zoom32.png) | [Bild](nachher-wald-zoom32.png) |

Technische Ergebnisse stehen in `validation.json`. Visuelle Abnahme durch Claude
offen. Seltene Trockenheits-/Brand-/Totholzzustände werden zusätzlich auf den
Kontaktbögen geprüft; nicht jede Variante wurde natürlich im Spiel ausgelöst.

## Ausgeführte technische Prüfungen

- 89 Rust-Tests und 15 Python-Tests bestanden.
- `cargo fmt --all`, strenges Release-Clippy und Release-Bau bestanden.
- Wiederholter Export: alle 91 PNGs und das Hauptmanifest bytegleich.
- Sämtliche bestehenden Weltmaße/Weltanker und alle 864 anderen PNGs unverändert.
- Periodische UV-Ränder, gemeinsame Materialränder zwischen Varianten und
  vertikale Wiederholbarkeit der Wand-PNGs geprüft.
