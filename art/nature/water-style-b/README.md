# Meer, Flachwasser und Teiche im Stil B

Basis v0.8.8 (`0d959fe`, Tiere aus PR #19 seit v0.8.7 übernommen).
Branch `codex/water-style-b`.

Drei Wasserarten mit je vier räumlichen Varianten und vier Animationsphasen:
`water_<deep|shallow|pond>_<0…3>_f0…f3.png`. Zusammen 48 Phasen und drei
aktualisierte Standbilder. Darstellungsmaß bleibt 64×32; Dateien 128×64 RGBA,
Anker 64/32. Das passt mit dem vollständigen Tierpaket in den bestehenden Atlas
mit höchstens 8192 px Höhe. Standbilder sind bytegleich mit Variante 0, Phase 0.

Die gemalten Wellen stammen aus der archivierten Materialquelle. Alle Phasen
bewegen dieselben Strukturen nur leicht auf einer geschlossenen kleinen Bahn.
Keine unabhängig wechselnden Texturen, keine zufälligen Lichtblitze und keine
neue Strömungsberechnung im Kern. Teiche bewegen sich am wenigsten. Vier Phasen
à 500 ms bilden eine ruhige Zwei-Sekunden-Schleife. Der vorhandene Beobachter
animiert nur bei 1 Std/s; bei Pause oder schneller Zeit zeigt er `f0`.

Periodische Materialkorrektur vermeidet Spiegelmuster. Die Varianten teilen
pro Phase dieselben UV-Ränder. Die mittlere Wasserfarbe bleibt am bisherigen
Satz kalibriert. Feste Rautenform/Alpha und Weltgröße bleiben in allen Phasen
gleich. Die gemessene Schleifen-Helligkeitsschwankung liegt unter 0,1 %
(Prüfwert in `validation.json`, Grenze laut Gestaltung 15 %).

## Tatsächliches Spiel und Materialproben

- [Tiefsee](water_deep-contact.png), [Flachwasser](water_shallow-contact.png),
  [Teich](water_pond-contact.png): Varianten/Phasen, synthetische Materialproben.
- Je Material die gleichnamige GIF-Datei, vier Varianten mit 4×500 ms.
- [Animation im laufenden eingebetteten Spiel](spielanimation.gif): wirklicher
  Bildschirmausschnitt bei 1 Std/s; kein nachgebauter Spielzustand.

Küste: `--seed 2024 --years 10 --select 21,11 --speed 0`, Zoom 4/8/32.
Teich: gleiche Parameter mit `--select 20,35`, Zoom 8/32.

| Szene | Vorher | Nachher |
| --- | --- | --- |
| Küste Zoom 4 | [Bild](vorher-zoom4.png) | [Bild](nachher-zoom4.png) |
| Küste Zoom 8 | [Bild](vorher-zoom8.png) | [Bild](nachher-zoom8.png) |
| Küste Zoom 32 | [Bild](vorher-zoom32.png) | [Bild](nachher-zoom32.png) |
| Teich Zoom 8 | [Bild](vorher-teich-zoom8.png) | [Bild](nachher-teich-zoom8.png) |
| Teich Zoom 32 | [Bild](vorher-teich-zoom32.png) | [Bild](nachher-teich-zoom32.png) |

Xvfb 1440×900, Software-OpenGL, nach erstem fertigen Bild pausiert. Vorher:
dieselbe v0.8.8-Ausführung mit den unveränderten Standbildern aus `0d959fe` als
separater `--bilder`-Überlagerung. Diese stehen auch unter allen Phasennamen,
damit der Vergleich tatsächlich das alte statische Material zeigt. Nachher:
ausschließlich neue eingebettete Bilder, ohne `--bilder`. Die Überlagerung
enthält nur Wasser; nicht eingebundene Altentwürfe werden nicht mitgeladen.

## Wiederholen und prüfen

Mit Pillow und NumPy:

```sh
python tools/nature/export_water_style_b.py
python tools/nature/check_water_style_b.py
```

`config.json` enthält Quellprüfsumme, feste Zielfarben und Bewegungsmaß.
`export.json` dokumentiert 48 Dateiprüfsummen, Kanten-/Schleifenprüfungen und
Helligkeit je Phase. Auch `export_landscape_style_b.py` wendet diese Wasserstufe
am Ende an; beide Wiederholungswege sind bytegleich. Alle übrigen 1228 PNGs
bleiben unverändert, einschließlich Tier-/Fernbildern und Bach-Altmaterial.

93 Rust-Tests, 15 Python-Tests, Formatprüfung, strenges Clippy und Release-Bau
bestanden, einschließlich vollständigem Atlasgrenztest. Neuer eingebetteter
Wassertest prüft alle Arten/Varianten/Phasen, feste Rautenmaske, identisches
Standbild und ruhige Helligkeit. Visuelle Abnahme durch Claude offen.
Bachläufe und trockene Bachbetten nach neuer Schnittstelle 16.8 folgen separat;
dieses Paket verändert ihre Darstellung nicht.
