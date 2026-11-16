# Bachläufe und trockene Bachbetten – Gestaltung 16.8

Basis v0.8.9 (`2f0e187`, Wasser aus PR #20 übernommen), Branch
`codex/stream-beds-style-b`. 54 statische RGBA-Bilder in `assets/nature/water/`:
je Zustand `wet`/`dry`, Stärke 0/1/2 und Teil `c`, `n`, `ne`, `e`, `se`, `s`,
`sw`, `w`, `nw`. Alle auf derselben ganzen 128×64-Leinwand, Weltmaß 64×32,
Anker 64/32; ein eigenes Gruppenmanifest bindet sämtliche Teile ein.

Die Bilder zeigen ausgewaschene Erde/Kies, kleine gerundete Steine und bei
nassen Stücken ein zusammenhängendes blaues Wasserinneres. Trockene Stücke
zeigen dieselbe Bettgeometrie ohne Wasser. Die Materialquellen sind neu mit dem
Bildgenerator erstellt; das Wasser nutzt zusätzlich die gemalte Materialquelle
des bereits übernommenen Wasserpakets. Kein neuer Pflanzenbewuchs, keine
zusätzlichen Simulationszustände und noch keine Bachanimation.

## Anschlüsse und Überlagerung

- Feste gemeinsame Bett- und Wasserbreiten je Stärke in `config.json`,
  unabhängig von Richtung und nass/trocken. Innen leichte Krümmung und
  unregelmäßige Kiesränder, letzte 20 % jedes Halbstücks gerade.
- Gegenüberliegende Endprofile haben dieselbe deckende Breite; Alpha weicht
  höchstens um eine Rundungsstufe, RGB höchstens um zwei von 255 ab. Zusätzlich
  werden ihre geometrischen Querschnitte unabhängig vom Pixelraster geprüft.
- Eine kurze Verlängerung erhält die volle Breite an Kanten/Ecken. Die Bilder
  werden an Eckanschlüssen nicht auf die dort punktförmige Rautenspitze
  zugeschnürt; außerhalb der Bachform bleibt alles transparent.
- Das Mittelstück liefert die gemeinsame Kiesmulde. Halbstücke überzeichnen
  im Knoten nur ihr Wasser/Inneres; ihre Ufer werden dort transparent gehalten.
  So legt ein später gezeichnetes Ufer keinen grauen Querdamm über einen schon
  vorhandenen Wasserarm. Alle 256 Richtungsbelegungen je Stärke – 768 Fälle –
  werden tatsächlich zusammengesetzt und auf verbundene Wasserenden geprüft.

[Alle Einzelteile](parts-contact.png), [Gegenstücke auf gleicher Höhe](joins-contact.png),
[Mittelstück und mehrarmige Knoten](junctions-contact.png). Das sind synthetische
Prüfbögen aus den tatsächlichen PNGs, keine Spielaufnahmen. Die Bodenquellen
und der Vordergrundbewuchs sind keine Bestandteile der Bachbilder.

## Tatsächlicher Spielvergleich

Genau der vorgegebene Spätsommerfall, zusätzlich Überblick/Nahansicht:

```sh
target/release/nature-in-a-glass --seed 2024 --years 12 --days 240 --select 19,33 --zoom 16
```

Ohne `--speed` startet der Beobachter pausiert; alle Aufnahmen zeigen Jahr 13,
Tag 241, 10:00 Uhr. Vorher der unveränderte Elternstand mit Linienrückfall,
nachher vollständig eingebettete Bachbilder ohne `--bilder`. Xvfb 1440×900,
Software-OpenGL, keine Eingriffe in Vegetation, Flussmenge oder Bachrichtungen.

| Zoom | Linienrückfall | Bach-/Bachbettbilder |
| --- | --- | --- |
| 4 | [Vorher](vorher-zoom4.png) | [Nachher](nachher-zoom4.png) |
| 8 | [Vorher](vorher-zoom8.png) | [Nachher](nachher-zoom8.png) |
| 16 | [Vorher](vorher-zoom16.png) | [Nachher](nachher-zoom16.png) |
| 32 | [Vorher](vorher-zoom32.png) | [Nachher](nachher-zoom32.png) |

**Beobachtete Darstellungsgrenze:** Im gewählten Feld verdeckt eine höhere
Vordergrundplatte weiterhin Teile des tieferen Bachbetts, ebenso Pflanzen an
anderen Stellen. Oberseiten mit verschiedener Höhe werden durch diese API
nicht durch senkrechte Wand-/Gefällestücke verbunden. Das ist in den tatsächlichen
Vergleichsbildern sichtbar, auch schon beim Linienrückfall. Die pixelgleichen
Anschlussbreiten lösen die Verbindung auf gleicher Höhe; zusätzliche Anschlüsse
über Höhenstufen wären ein eigener Renderer-/Schnittstellenauftrag an Claude.
Dieser PR verändert dafür weder Höhen noch Zeichenreihenfolge.

## Wiederholen und prüfen

Mit Pillow, NumPy und SciPy:

```sh
python tools/nature/export_stream_beds.py
python tools/nature/check_stream_beds.py
```

Quellenprüfsummen, gemeinsame Breiten und Materialfarbe in `config.json`;
Dateiprüfsummen in `export.json`; Anschluss-/Überlagerungsprüfungen in
`validation.json`. Bach-, Wasser- und vollständiger Landschaftsexport liefern
beim Wiederholen bytegleiche PNGs/Manifeste. Alle 1279 bisherigen PNGs bleiben
unverändert. Der vollständige gemeinsame Atlas wird gegen die bestehende
8192-px-Grenze geprüft. Visuelle Abnahme durch Claude offen.

Ausgeführt: `cargo fmt --all`, `cargo clippy --release --all-targets -- -D warnings`,
`cargo test --release --workspace` (94 bestanden),
`cargo build --release -p nature-in-a-glass`; zusätzlich 15 bestehende Python-Tests
und die oben beschriebenen Assetprüfungen bestanden. Tatsächliche eingebettete
Spielaufnahmen bei Zoom 4/8/16/32 geprüft.
