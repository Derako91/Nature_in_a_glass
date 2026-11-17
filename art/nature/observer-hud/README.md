# Beobachter-HUD – Seekartenstil

Basis v0.21.0 (`53d5d51`), Branch `codex/observer-hud-chart-style`.
Überarbeitet: obere Leiste, rechte Spalte und Bewohner-Fenster. Visuelle
Abnahme durch Claude offen. Keine Änderung an Simulation, Kamera oder
Spielbedienung; die neuen Bootsfunktionen bleiben erhalten.

## Gestaltung

Dunkles Blaugrün als Tinte, helle warme Schrift, zurückhaltende Messingakzente.
Überschriften und Bewohnernamen verwenden die bereits eingebettete
Seekartenschrift. Fließtext bleibt eine gut lesbare Standardschrift; laufende
Werte verwenden Monospace. Knöpfe und Eingaben haben einen schmalen Rand,
kleine Ecken und unterscheidbare Ruhe-, Auswahl- und Hoverzustände.
Erkennungsfarben bleiben an den Punkten im Bewohnerregister erhalten;
die Namen selbst sind hell, damit auch dunkle Erkennungsfarben lesbar bleiben.

- Obere Leiste weiterhin 66 px, Statuszeile 26 px, rechte Spalte 400 px.
  Zeitraffer, Ansicht, Ereignisse, Spielstand und Inselstart stehen in der
  bisherigen Reihenfolge. Die Breiten der v0.21-Leistenelemente bleiben erhalten,
  insbesondere der reservierte 190-px-Bootszähler.
- Inselwerte: feste Schlüssel-/Wertspalten, 184/190 px, 16-px-Zeilen und
  ein Pixel Zeilenabstand. Zahlen, lange Texte und neue Bewohner vergrößern
  keine Datenzeile; abgeschnittene Angaben stehen vollständig im Tooltip.
- Die rechte Spalte hat kompakte feste Abstände, dieselben Regler und
  einen 80 px hohen Verlauf. Dadurch passen bei 1440×900 auch acht Quellen,
  Verlauf und Chronik in die Spalte. Zusätzliche Scrollflächen gibt es nicht.
- Bewohner-Fenster: feste 760×400 px, 200-px-Register links, feste Zeilen
  für Person, Bedürfnisse, Zeigen, Familie und Können rechts. Lange Familien-
  und Ortsangaben werden einzeilig mit vollständigem Tooltip gezeigt.
  Verstorbene reservieren den Platz der entfallenden Bedürfnisse/Zeigen-Zeilen,
  damit die Lebenschronik am gleichen Ort beginnt. Auch bei langen Registern
  und Biografien wächst das Fenster nicht mit dem Inhalt.
- Im Haupt-HUD scrollt ausschließlich die Chronik. Die bereits vorhandene
  Bewohnerauswahl und Lebenschronik behalten ihre Scrollbedienung; sonst wären
  große Register und lange Biografien nicht mehr vollständig erreichbar.

## Echte Spielprüfung

Alle PNGs stammen aus dem nativen Spiel mit normalen Startoptionen und
Mauseingaben; kein Bildaustausch, keine Vorschauzustände, keine manipulierten
Bewohner. 1440×900, Xvfb, Mesa-Softwaredarstellung. Vorher unverändertes
v0.21.0, nachher der Release-Bau dieses Zweigs.

`before-*.png` / `after-*.png` zeigen fünf gleiche Prüfstände:

```sh
nature-in-a-glass --seed 2024
nature-in-a-glass --seed 2024 --years 9 --days 364
nature-in-a-glass --seed 2024 --years 10 --menschen --danach 3 --bewohner
nature-in-a-glass --seed 2024 --groesse inselwelt --years 3
nature-in-a-glass --seed 5 --years 10 --menschen --danach 30 --bewohner
```

Der zweite heißt **long-date**: Die CLI deaktiviert bei vorbereiteten
Prüfständen absichtlich das automatische Boot. Er prüft zweistelliges Jahr,
dreistelligen Tag und reservierten Platz ohne Zähler, keine bevorstehende
Landung. Argumente stehen zusätzlich in den beiden `*-captures.json`.

`interaction-*.png` dokumentieren tatsächliche Bedienhandlungen:

1. Frische Insel, Zähler angeklickt: ausgewählter Sprungzustand in
   `interaction-rushing.png`; anschließend echte Landung, Landemeldung,
   Kamera zum Landeplatz und wieder ausgewähltes „1 Tag/s“ in
   `interaction-landed.png`. Der Text der Statusmeldung wird beim Test per OCR
   beobachtet, nicht bloß nach einem festen Warteintervall angenommen.
2. „Menschen kommen von selbst“ aus/an, jeweils „Neue Insel“: Zähler verschwindet
   bzw. kehrt zurück, die übrigen Bedienelemente bleiben an ihren Plätzen.
3. Erwachsene Siv und Kind Dag ausgewählt: Rahmen und Beginn der Lebenschronik
   bleiben identisch. Im langen Register nach unten gescrollt und die
   verstorbene Greta ausgewählt; auch dort stehen Rahmen und Trennlinie gleich.
4. `quiet-a.png` / `quiet-b.png`: zeitversetzte Aufnahmen einer pausierten
   Inselwelt. Der gesamte Bildschirm bleibt pixelgleich.

## Wiederholung und Ergebnis

```sh
python tools/nature/capture_observer_hud.py /pfad/zum/program before
python tools/nature/capture_observer_hud.py /pfad/zum/program before --long-register
python tools/nature/capture_observer_hud.py /pfad/zum/neuen/program after
python tools/nature/capture_observer_hud.py /pfad/zum/neuen/program after --long-register
python tools/nature/capture_hud_interactions.py /pfad/zum/neuen/program
python tools/nature/capture_hud_interactions.py /pfad/zum/neuen/program --extra
python tools/nature/check_observer_hud.py
```

Die Aufnahmeskripte erwarten Xvfb `:101`, 1440×900; Standardbibliothek,
Pillow, ImageMagick, X11/XTest und Tesseract. Die Auswertung benötigt NumPy.
Mauskoordinaten sind ausdrücklich an diese Testauflösung gebunden.

`validation.json`: null geänderte Pixel in den statischen Teilen der Leiste
bei wechselnder Datumsbreite/fehlendem Zähler; null Verschiebung der Schlüssel-
spalte bei Bewohnern; null Bewegung der pausierten Ansicht; identischer
Fenstertitelrahmen und identische Chronik-Trennlinie bei Erwachsenen, Kindern
und Verstorbenen; identisches ausgewähltes Tempofeld nach der Landung.
18 Steuer-/Speicher-/Zeitfunktionen sind gegen die Basis unverändert geprüft,
darunter `boat_countdown` und die Landemeldungsübernahme. Kern, Weltbilder und
Küstenzeichnung bleiben unverändert.

Ausgeführt: `cargo fmt --all`, strenges workspaceweites Release-Clippy,
`cargo test --release --workspace` (117 Rust-Tests), zusätzlich die 20 App-Tests
nach den letzten Darstellungsanpassungen, 15 Python-Tests und Release-Bau.
Die visuelle Freigabe durch Claude steht aus; kein Release dieses Zweigs.
