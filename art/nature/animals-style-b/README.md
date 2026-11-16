# Schritt 4: vollständiges Tierpaket im Stil B

Basis v0.8.6, `7d9dcc5`; Branch `codex/animals-style-b`.

472 PNGs ersetzen sämtliche bisherigen Tierbilder und ergänzen eigene
Fernfassungen sowie Spurenvarianten:

| Gruppe | Nah | Eigenes `@fern` |
| --- | ---: | ---: |
| Hirsch, Hirschkalb, Wolf, Fuchs, Kaninchen; je zwei Fellvarianten, beide Richtungen, Fressen 2, Gehen 4, Laufen 4, Ruhen 1 Phase | 220 | 220 |
| Große/kleine Kadaver; je zwei Varianten, beide Richtungen | 8 | 8 |
| Spuren für vier Arten, je vier Varianten | 16 | – (Bodenüberlagerung) |

Natürliche Anatomie, warmes Fell, Licht rechts oben. Beide Richtungen und alle
Phasen stammen aus eigenen Bildgenerator-Zellen; keine gespiegelten Bilder,
keine vervielfältigten Ersatzphasen. Der Hirsch hat ein eigenes Kalb ohne Geweih;
Wolf-/Fuchsjungtiere verkleinert weiterhin der vorhandene Beobachter.

## Maßstab, Fußpunkte und Fernansicht

Die sichtbare Breite des bisherigen ersten Fressbilds jeder Fellvariante/Richtung
ist die Kalibrierung. `config.json` archiviert diese Maße. Je Art/Fell/Richtung
gilt ein gemeinsamer proportionaler Faktor für sämtliche Tätigkeiten. Kein
Bild wird auf ein festes Rechteck verzerrt. Zusätzlicher transparenter Spielraum
nimmt ausgestreckte Läufe und Schwanzspitzen auf; `display_size` kompensiert
ihn, sodass Tiere dadurch nicht wachsen.

Nahbilder haben acht, Fernbilder zwei Dateipixel je Welteinheit. Die vierfach
kleineren Anker der Fernbilder ergeben exakt denselben Weltfußpunkt. Der Atlas
wählt `@fern` bei Zoom 4/8, die Zwischenstufe bei 16 und das Nahbild bei 32.
Ein Regressionstest prüft alle 228 Tier-/Kadaverphasen in allen vier Zoomstufen,
einschließlich vorhandener Dateien, tatsächlicher Auswahl und stabiler Anker.

Der große Sicherheitsrand des Entwurfsexports überschritt zunächst die
Atlasgrenze. Der endgültige Export entfernt pro Art einen gemeinsamen leeren
Rand über sämtliche Zustände, Fellvarianten, Richtungen und Detailstufen.
Rückeinsetzen des Ausschnitts wird pixelgenau geprüft: keine Bildpixel gehen
verloren. Der Ankerabzug erhält die Weltposition jedes Pixels. Der Atlas bleibt
innerhalb der unveränderten 8192-px-Grenze; keine Rendereränderung.

## Prüfpaket

- [Hirsch](deer-contact.png), [Kalb](deer_young-contact.png), [Wolf](wolf-contact.png),
  [Fuchs](fox-contact.png), [Kaninchen](rabbit-contact.png): sämtliche Phasen,
  oben Nah-, darunter Fernfassung; synthetische Detailprüfung.
- Je Art `*-graze.gif`, `*-walk.gif`, `*-run.gif`: Schleifen mit beiden
  Fellvarianten/Richtungen, Nah und Fern. Originale Takte 600/200/100 ms.
- [Lesbarkeit auf Erde, Gras und Waldboden](readability.png): tatsächliches
  Weltmaß bei Zoom 8 und 32, synthetischer Vergleich.
- [Kadaver](carcasses-contact.png), [Spuren auf Gras](tracks-contact.png).

Tatsächlicher Vorher-/Nachher-Vergleich ohne `--bilder`, eingebetteter Release-Bau:

```sh
target/release/nature-in-a-glass --seed 2024 --years 10 --tiere hirsche,kaninchen,woelfe,fuechse --bei 47,47 --select 47,47 --speed 0 --zoom 32
```

| Zoom | Vorher | Nachher |
| --- | --- | --- |
| 4 | [Bild](vorher-zoom4.png) | [Bild](nachher-zoom4.png) |
| 8 | [Bild](vorher-zoom8.png) | [Bild](nachher-zoom8.png) |
| 32 | [Bild](vorher-zoom32.png) | [Bild](nachher-zoom32.png) |

Offenes, nach vorne abfallendes Feld, damit höhere Vordergrundplatten die
kleinen Tiere nicht verdecken. Das Aussetzen erfolgt über den normalen
Spieleingriff der Startparameter. Keine direkten Änderungen an Tierzuständen.
Xvfb 1440×900, Software-OpenGL, nach dem ersten fertigen Bild pausiert. Beim
Winterstart ruhen die Tiere; nicht jede Geh-/Lauf-/Kalb-/Kadaverphase wurde
natürlich im Spiel ausgelöst. Diese sind vollständig auf Kontaktbögen/GIFs und
im eingebetteten Atlas geprüft. Visuelle Abnahme durch Claude offen.

[Ausschnitt des laufenden Spiels](spielbewegung.gif), gleiche Startparameter
plus `--days 1`: natürlich fortgeschrittene Tierzustände nach einem Tag.

Technisch bestanden: 91 Rust-Tests, 15 Python-Tests, Formatprüfung, strenges
Clippy und Release-Bau. Der vollständige eingebettete Atlas bleibt unter
8192 px Höhe. Wiederholter Export sämtlicher 472 PNGs und Manifest bytegleich.

## Wiederholen

Mit Pillow, NumPy und SciPy:

```sh
python tools/nature/export_animals_style_b.py
python tools/nature/check_animals_style_b.py
python tools/nature/build_animals_style_b_proof.py
```

`sources/`: 20 Bögen mit je vier Spalten/sechs Reihen (je Art/Fellvariante ein
Nah- und Fernbogen), zwei Kadaverbögen und ein Spurenbogen. Die erste Zeile zeigt
Fressen rechts hoch/tief, links hoch/tief; danach Gehen rechts/links und Laufen
rechts/links mit je vier Phasen. Letzte Zeile: Ruhen rechts/links und zwei
stehende Kalibrierbilder. Die letzten zwei Bilder werden nicht als zusätzliche
Phase eingebunden. Quellen und SHA-256 sind archiviert; der Export ist unabhängig
vom Bildgenerator wiederholbar.

`export.json` dokumentiert alle Dateien/Anker/Faktoren; `validation.json` prüft
Vollständigkeit, unterscheidbare Phasen, sichtbare Referenzbreiten und unveränderte
andere Bildgruppen. Die alten Validierungsdateien unter `assets/nature/animals/`
verweisen auf den neuen Satz; frühere Sechsfarben-/Kleinstmaß-Prüfungen gelten
für diese Stil-B-Bilder nicht mehr. Landschaft, Menschen, Gebäude, Wasser und
Feuer bleiben unverändert. Wasser folgt im zweiten separaten PR.
