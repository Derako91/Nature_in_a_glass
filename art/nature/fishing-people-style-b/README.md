# Fischfang – eigenes Menschenbild im Stil B

Basis `0edb584` (v0.9.1), Branch `codex/fishing-people-style-b`.
40 neue PNGs: fünf erwachsene Aussehen × beide Richtungen × vier Phasen.
Kurzer Holz-/Steinspeer, leicht vorgebeugt am Ufer, ruhiges Absenken und
Zurücknehmen. Kein garantierter Fang als Animation, keine Kinder mit Speer.
Haare, Kleidung und Erkennungsfarben folgen den vorhandenen Figuren.

## Bilder und Maß

`assets/nature/people/person_<0…4>_fish_<l|r>_f0…f3.png`, vier Phasen à 300 ms.
112×120 RGBA, Fußpunkt 56/112, Weltmaß 12,6×13,5. Der Speer passt in die
bestehende Erwachsenenleinwand; die Auflösung bleibt 8,889 px je Welteinheit.
Jedes Aussehen nutzt einen einzigen proportionalen Faktor für sämtliche Phasen
und beide Richtungen, begrenzt durch seine vorhandene Stehhöhe. Gebückte Phasen
werden nicht einzeln auf Stehhöhe gestreckt. Der Fußpunkt orientiert sich an den
unteren Sohlen, nicht an der Speerspitze.

Die Quellen unter `sources/` wurden mit dem Bildgenerator und den fünf
bestehenden Referenzbögen aus `people-hut-adjustments` erzeugt. Richtungen
separat gezeichnet, keine gespiegelten Phasen. Quellen samt SHA-256 in
`sources.json`, exportierte Dateien/Maße/Faktoren in `export.json`.
[Kontaktbogen](contact.png) zeigt alle tatsächlichen PNGs, keine Spielaufnahme.

## Technische Übergabe – noch kein automatisch ausgewählter Zustand

Die Bilder sind eingebettet und das Manifest enthält den Zustand `fish`.
Die Simulation auf v0.9.1 führt das Fischen jedoch innerhalb der Nahrungsgewinnung
aus, nicht als eigene `Activity::Fish`. Der unveränderte Renderer kennt auch
noch keine vier Fischfangphasen. Ein bloßer Assetimport kann deshalb im normalen
Spiel noch keine automatisch fischende Person zeigen.

Claude muss den tatsächlichen Fischfang als Tätigkeit kenntlich machen, über
`appearance` weiterreichen und `person_frames("fish") = (4, 300 ms)` ergänzen.
Nur tatsächlich ausgeführter Fischfang löst das Bild aus; Sammeln in Wassernähe
reicht nicht. Nacht-/Abendzustände sollen entsprechend der vorhandenen Menschen-
Darstellung weiter funktionieren. Kinder und bestehende Zustände bleiben gleich.

## Wiederholen

```sh
python tools/nature/export_fishing_people.py
python tools/nature/check_fishing_people.py
```

Auch der vollständige Menschenexport ruft den Fischfangexport am Ende auf,
damit dessen Manifest beim Wiederholen nicht verloren geht.

## Wasser an Geländestufen

Der separate **Formatvorschlag**, noch keine implementierte Schnittstelle oder
Bilder, steht in `docs/gestaltung.md` Abschnitt 16.9: Kopf/Mitte/Fuß, linke/rechte
sichtbare Wand, gleiche drei Breiten wie vorhandene Bäche, gekachelte statt
langgezogene Fallstrecke. Darauf kann Claude die Geometrie aufbauen.

## Ausgeführte Prüfungen

- `cargo fmt --all`
- `cargo clippy --release --all-targets -- -D warnings`
- `cargo test --release --workspace`: 97 Tests bestanden, einschließlich
  Vollatlasgrenze und neuem Test aller 40 eingebetteten Fischfangphasen.
- 15 bestehende Python-Tests (`tools/sprite_agent/tests`) bestanden.
- `check_fishing_people.py`: alle 1333 Eltern-PNGs unverändert; bestehende
  Manifestdaten einschließlich Erkennungsfarben und Kindergrößen unverändert.
  Neue Phasen eindeutig, linke Richtung kein Spiegelbild der rechten.
- Wiederholter Fischfangexport: alle 40 PNGs und das Manifest bytegleich.

[Animations-Prüfbogen](animation.gif) zeigt vier echte Phasen mit 300 ms,
alle fünf Aussehen und beide Richtungen; synthetischer Prüfbogen, kein Spielvideo.


## Tatsächliche Spielaufnahmen und Grenze der Probe

Release-Bau mit `cargo build --release -p nature-in-a-glass` bestanden.
Software-OpenGL/Xvfb 1440×900, aktuelle Simulation v0.9.1, Start:

```sh
target/release/nature-in-a-glass --seed 2024 --years 10 --menschen --danach 3 --days 120 --select 19,34 --speed 0 --zoom 32
```

| Zoom | Ohne Bildaustausch | Temporärer Fischfang-Bildaustausch |
| --- | --- | --- |
| 8 | [Eingebettet](eingebettet-zoom8.png) | [Darstellungsprobe](darstellungsprobe-zoom8.png) |
| 32 | [Eingebettet](eingebettet-zoom32.png) | [Darstellungsprobe](darstellungsprobe-zoom32.png) |

**Die Darstellungsprobe zeigt keine echte Fischfangauswahl.** Dafür werden die
40 Fischfangbilder in einem temporären Ordner auf die vorhandenen erwachsenen
`gather`-Namen gelegt und mit `--bilder` geladen. So sieht man Fußpunkt, Größe,
Speer und Abgrenzung zu Kindern in der wirklichen Spielansicht; die Speerfiguren
stehen dabei auch fern vom Ufer. Im unveränderten eingebetteten Spiel bleiben
an denselben Stellen die hockenden Sammelbilder sichtbar. Die neuen `_fish`-
Dateien sind vorhanden, werden aber noch nicht automatisch ausgewählt.

Die Probe ist wiederholbar mit `python tools/nature/preview_fishing_overlay.py`,
danach obiger Spielbefehl plus `--bilder /tmp/fish-preview-overlay`.
Vier Phasen/300 ms werden für die Probe vom bisherigen Sammelzustand verwendet.
Nach Claudes Zustandsanbindung muss echter Fischfang am Ufer einschließlich
Tages-/Abenddarstellung erneut geprüft werden. Visuelle Abnahme offen.
