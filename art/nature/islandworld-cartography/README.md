# Inselwelt: Küsten, Meeresräume und Seekartenschrift

Basis `66d829e`, v0.19.0; Branch `codex/islandworld-cartography`.
Optische Änderung der Inselnamen: eingebettete **IM FELL English Italic**,
22 Bildschirm-Pixel, warmes Elfenbein mit schmalem blauschwarzem Rand.
Schriftlizenz und Herkunft stehen unter `assets/fonts/` (SIL OFL 1.1).
Nur die eigene Schriftfamilie der Inselnamen verwendet die neue Schrift.

## Befund

- Seed 2024 erzeugt sechs beschriftete größere Inseln. In der Gesamtkarte
  sind Landflächen, helle Flachwasserzonen und tiefblaue Meeresstraßen klar
  unterscheidbar. Die Namen sind den Inseln gut zuzuordnen; im gezeigten
  Gesamtausschnitt überlagern sie einander nicht.
- Zoom 1 und 2 wurden in Karte und Isometrie angesehen. Die Namen bleiben
  in fester Bildschirmgröße lesbar über grünem Land, Bewuchs und Wasser.
  Die neue kursive Serifenschrift unterscheidet sie von Oberflächen- und
  Dorfbeschriftungen, ohne zusätzliche Schilder über die Küsten zu legen.
- In der **Karte** erscheinen Namen auch bei Zoom 8. Gezielte Aufnahmen
  von Möweneiland und Bernsteinwerder prüfen Umlaut und langen Namen.
  In der **isometrischen Nahansicht** sind Inselnamen nach der bestehenden
  Regel oberhalb Zoom 2 ausgeblendet. Die Küstenaufnahme bei Zoom 8 zeigt
  stattdessen die scharfe Land-/Flachwassergrenze und ruhig lesbare Wasserflächen.
- Beim voreingestellten Bildausschnitt passen bei Zoom 1 nicht alle Inseln
  auf den Bildschirm. Am Rand können Namen angeschnitten sein; dies ist
  im unveränderten Stand bereits so. Zentrierte Aufnahmen zeigen die
  jeweiligen Namen vollständig. Zusätzlich liegen ganze Karten bei Zoom
  0,45 und ganze isometrische Übersichten bei Zoom 0,4 bei.
- Im **isometrischen Meer bei Zoom 1 und 2** sieht man ein feines regelmäßiges
  dunkles Punktraster der Wasserrauten. Es ist schon im Vorherbild vorhanden.
  Die Meeresstraßen bleiben erkennbar; dieses Paket ändert weder Wassertiles
  noch deren Zeichnung. Falls der Rastereindruck stört, bleibt die
  Wasser-Fernsicht ein eigener optischer Folgepunkt.

Die Änderung betrifft ausschließlich Schriftfamilie, Schriftgröße,
Schriftfarbe und Rand. Inselnamen, Ankerpositionen, Kamera, Auswahl-/Zoomregeln,
Meeresflächen, Küsten und Simulation bleiben im vorhandenen Code.
Der überprüfte vollständige Kartenvergleich hat **null veränderte Pixel
außerhalb der sechs Textbereiche**. Auch die Pausenprüfung hat null
veränderte Pixel zwischen zwei zeitversetzten Aufnahmen desselben Ausschnitts.

## Wiederholbare Spielprüfung

24 echte native Spielaufnahmen: zwölf vorher, zwölf nachher, ohne Bildaustausch,
ohne Entwurfsbeobachter und ohne veränderte Spielzustände. Start jeweils:

```sh
nature-in-a-glass --seed 2024 --groesse inselwelt --years 3 --zoom 1
nature-in-a-glass --seed 2024 --groesse inselwelt --years 3 --view karte --zoom 1
```

Jeweils ebenso Zoom 2 und 8. Zusatzfälle:

```sh
nature-in-a-glass --seed 2024 --groesse inselwelt --years 3 --view karte --zoom 0.45
nature-in-a-glass --seed 2024 --groesse inselwelt --years 3 --zoom 0.4
nature-in-a-glass --seed 2024 --groesse inselwelt --years 3 --view karte --zoom 2 --select 105,76
nature-in-a-glass --seed 2024 --groesse inselwelt --years 3 --view karte --zoom 8 --select 58,38
nature-in-a-glass --seed 2024 --groesse inselwelt --years 3 --view karte --zoom 8 --select 137,57
nature-in-a-glass --seed 2024 --groesse inselwelt --years 3 --zoom 8 --select 86,58
```

Bildnamen und exakte Argumente in `before-captures.json` / `after-captures.json`.
`name-details-comparison.png` sind native, unvergrößerte Ausschnitte der sechs
Inselnamen, bisher / nachher. Die vollständigen Aufnahmen bleiben daneben erhalten.

```sh
python tools/nature/capture_islandworld_review.py /pfad/zum/bisherigen/programm before --display :100
python tools/nature/capture_islandworld_review.py /pfad/zum/neuen/programm after --display :100
python tools/nature/check_islandworld_review.py
```

1440×900, Xvfb, Mesa-Softwaredarstellung. Die Auswertung benötigt Pillow,
NumPy und `fc-query` (fontconfig). `validation.json`: 24 Aufnahmen,
Zeichenabdeckung einschließlich ÄÖÜäöüß, originaler Font-SHA256,
ruhige Pausenansicht und unveränderte Bildbereiche außerhalb der Namen.

Bestanden: `cargo fmt --all`, 114 Rust-Tests, 15 Python-Tests,
`cargo build --release` und strenges Clippy **für das geänderte App-Paket**
(`cargo clippy --release -p nature-in-a-glass --no-deps -- -D warnings`).
Das vorgeschriebene **workspaceweite** Clippy scheitert am vorhandenen
Kernfehler `crates/core/src/people.rs:1119` (`needless_range_loop`, Rust 1.100.0),
wie schon auf der Basis. Keine Warnung unterdrückt, kein Kerncode geändert.
Visuelle Freigabe durch Claude offen.
