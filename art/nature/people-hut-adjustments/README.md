# Menschenmaß, Hocke und Hütteneingänge

Basis: `0f81511` (v0.8.4), Branch `codex/people-scale-and-hut-doors`.

Alle 280 Erwachsenen-/Totenphasen einschließlich Holztragen werden über das
Manifest proportional auf 90 % dargestellt: stehend rund 11 statt 12 Einheiten.
Alle 150 Kinder-PNGs und ihre Manifest-Einträge bleiben unverändert (rund 8 hoch).
Sämtliche Fußpunkte und die Erkennungsfarben bleiben erhalten.

40 Sammel-, 20 Ess- und 20 Trinkbilder sind neu kompakt gezeichnet. Kopf und
Schultern werden je Aussehen/Richtung am bestehenden Stehen kalibriert; die
Figuren werden proportional verkleinert, nicht seitlich zusammengedrückt.
Die Ausgangshocke ist ungefähr 5–6 Einheiten breit; einzelne Armphasen sind
schmaler. Essen/Trinken bleiben ebenfalls kompakt, ohne neue Schalen/Becher.
Exakte Maße je Phase: `people-adjustment.json`, zusammengefasst `validation.json`.

Beide Hütteneingänge haben nominal 4,5×9 Einheiten; der zusammenhängende dunkle
Innenraum misst nach dem Export 4,5×8,875. Nur Tür und unmittelbare Umrandung
wurden aus neu erzeugten Entwürfen übernommen. Gesamte Alpha-Kontur, Bildgröße,
Hüttenmanifest und sämtliche Pixel außerhalb der dokumentierten Türregion bleiben
unverändert. Details: `door-adjustment.json`.

## Tatsächlicher Spielvergleich

Ohne `--bilder`, mit den im Release-Bau eingebetteten Bildern:

```sh
target/release/nature-in-a-glass --seed 2024 --years 10 --menschen --danach 3 --days 120 --select 19,34 --speed 0 --zoom 8
target/release/nature-in-a-glass --seed 2024 --years 10 --menschen --danach 3 --days 120 --select 19,34 --speed 0 --zoom 32
```

Xvfb, 1440×900, Software-OpenGL. Für vergleichbare Bildschirmfotos wurde nach
dem ersten gezeichneten Bild mit Leertaste pausiert. Alle Aufnahmen zeigen den
natürlich simulierten Zustand (Jahr 14, Tag 121, 11:00), keine Eingriffe in die
Bewohner. Die vorgegebene Lagerstelle und die hockende Erwachsene am
Hütteneingang sind in beiden Zoomstufen sichtbar.

| Zoom | Vorher (v0.8.4) | Nachher |
| --- | --- | --- |
| 8 | [Bild](vorher-zoom8.png) | [Bild](nachher-zoom8.png) |
| 32 | [Bild](vorher-zoom32.png) | [Bild](nachher-zoom32.png) |

Alle fünf Aussehen und beide Richtungen einschließlich Ess-/Trinkposen:
[Kontaktbogen](../people-style-b/person-contact.png). Animationsschleifen liegen
im gleichen Ordner als `person-gather.gif`, `person-eat.gif`, `person-drink.gif`.
Seltene Zustände einschließlich Toten wurden auf Kontaktbögen geprüft, nicht
sämtlich in natürlichen Spielsituationen. Visuelle Abnahme durch Claude offen.

## Quellen und Wiederholung

`sources/` archiviert die mit dem Bildgenerator erstellten Entwürfe und die zwei
Originalhütten. SHA-256-Prüfsummen stehen in `config.json`. Je Kompaktbogen:
vier stehende Kalibrierbilder, vier Sammelphasen rechts, vier links, vier Ess-
und vier Trinkphasen. Für den Export werden nur die Sammelreihen aus `compact-*`
und die letzten zwei Reihen der bereinigten `hands-*` verwendet. Nicht verwendete
Entwurfsreihen können abweichende Requisiten enthalten.

```sh
python tools/nature/refine_people_and_huts.py
```

Benötigt Pillow, NumPy und SciPy wie die bestehende Menschen-Pipeline. Der Export
setzt absolute Manifestmaße; wiederholtes Ausführen schrumpft nicht weiter.
Die bestehenden `export_people_style_b.py` und `export_camp_sprites.py --huts-only` wenden
diese Korrektur ebenfalls an. Wiederholungsprüfung inklusive beider Exporte:
alle betroffenen Spiel-PNGs und beide Manifeste bytegleich.

Prüfungen: 15 Python-Tests; Formatprüfung, strenges Clippy, vollständige Rust-Tests
und Release-Bau. Technische Invarianten und Messwerte in `validation.json`.
