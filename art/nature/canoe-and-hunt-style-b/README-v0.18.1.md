> Historischer Entwurf auf v0.18.1 (Commit ba57cff). Für die aktuelle Einbettung siehe README.md; die alten Vorschau-Anweisungen gehören nicht zur v0.22-Prüfung.

# Einbaum und Jagd – Stil-B-Entwurf

Basis `3390433`, v0.18.1; Branch `codex/canoe-and-hunt-style-b`.
**88 Bilder ausschließlich in `art/`, noch nicht produktiv eingebettet.**
Der Einbaum würde mit der bisherigen Bildauswahl auf dem Land stehen;
Meerplatzierung und echte Jagdauswahl übernimmt Claude nach Gestaltung 16.7.

- 40 `person_<0…4>_fish_boat_<l|r>_f0…f3`: sitzend im langen, flachen
  ausgehöhlten Baumstamm, kleines helles Netz, ruhiges Einholen/Auslegen.
- 40 `person_<0…4>_hunt_<l|r>_f0…f3`: geduckt, gespannt, Speer bereit;
  kurze Gewichtsverlagerung, keine ständig erfolgreiche Wurfbewegung.
- Vier `deer_hunted_<0|1>_rest_<l|r>_f0` plus vier gefilterte `@fern`-Stufen:
  erlegter Hirsch mit geschlossenen Augen und Speer, ohne Blutlache.

Alle fünf erwachsenen Aussehen mit bisherigen Frisuren/Erkennungsfarben,
Richtungen eigenständig gezeichnet, keine Kinderbilder. Zwei GIFs zeigen
sämtliche Phasen mit den vorgeschlagenen Bilddauern; Kontaktblätter jeweils
Phase 0 und beide Richtungen. Quellen samt SHA256 in `sources.json`.

## Bildformat zur Anbindung durch Claude

| Satz | Leinwand | Anker | Weltmaß | Dauer |
| --- | --- | --- | --- | --- |
| Jagd | 144×120 | 72/112 | 16,2×13,5 | 4×250 ms |
| Einbaum | 224×144 | 112/128 | 25,2×16,2 | 4×300 ms |
| Erlegter Hirsch | 192×128 | 96/64 | 21,6×14,4 | Standbild |
| Hirsch `@fern` | 48×32 | 24/16 | identisches Weltmaß | Standbild |

Die Leinwand ist nicht die deckende Motivbreite. Jagd hat seitlichen Platz
für den Speer; die Erwachsenen werden dafür weder verkleinert noch gedehnt.
Der Fußpunkt bleibt auf derselben Bodenposition. Einbaum-Anker: Wasserlinie
unter der Rumpfmitte, nicht die Füße der sitzenden Person. Hirsch-Anker:
Motivmitte am Boden. Das komplette Hirschmotiv einschließlich Speer ist etwa
17 Welteinheiten breit. Manifestdateien liegen unter `sprites/people/` und
`sprites/animals/`; die Dateien sind auf die späteren Bildordner vorbereitet.
Fernstufen verwenden das vorhandene `levels`-Format.

Jede Menschenquelle enthält vier gewöhnliche stehende Referenzfiguren;
aus deren Höhe und den unveränderten Erwachsenenbildern wird ein gemeinsamer
proportionaler Faktor pro Quelle ermittelt. Dasselbe Maß gilt für sämtliche
Posen und beide Richtungen. Referenzfiguren werden nicht exportiert. Die
Einbaumquellen haben zwei Spalten/sechs Zeilen, die Jagdquellen vier/drei.
Keine individuelle Anpassung von Breite oder Höhe. `export.json` dokumentiert
Faktoren, Quellbereiche und exportierte Dateien.

**Konkrete Schnittstellenwünsche:**

1. `PersonSprite.boat_tile: Option<u32>` (oder gleichwertige Renderposition)
   für ein angrenzendes Meerfeld; tatsächliches Personenfeld und Simulation
   bleiben unverändert. Erst beim tatsächlich ausgewählten `fish_boat`-Bild
   auf das Meerfeld zeichnen und dort einsortieren. Ohne Bild oder nachts/
   beim Essen bleibt die bisherige Landdarstellung. Persönlicher Versatz
   muss innerhalb der Wasserraute bleiben, damit mehrere Boote sich verteilen.
2. Echten Zustand `hunt` für Jagdhandlungen liefern und im Beobachter mit
   `(4,250 ms)` aufnehmen. Flucht-`run` bleibt Flucht; nicht pauschal durch
   Jagd ersetzen. Tageszeit/Ruhe und Kinderregeln wie bei den anderen Tätigkeiten.
3. `deer_hunted` nur für tatsächlich durch Menschen erlegte Hirsche
   auswählen. Andere Kadaver behalten ihre bestehenden Bilder. Das derzeitige
   Kadaverformat liefert nur die Größenklasse; die Unterscheidung fehlt noch.

## Prüfung auf großer Insel

Unverändertes v0.18.1, 128×128 Felder:

```sh
./target/release/nature-in-a-glass --seed 5 --groesse gross --years 10 --tiere hirsche --menschen --danach 30 --select 47,84 --zoom 32
```

Außerdem Zoom 8. `baseline-coast-*.png` sind echte unveränderte Spielaufnahmen.
Die Diagnose `fixture-probe.rs` verwendet nur öffentliche Simulationsaufrufe,
keine direkt veränderten Bewohner; Ergebnisse in `fixture-observations.txt`.
Am ersten Tag sind mehrere Küstenfischer mit `tool=boat` vorhanden, unter
anderem Person 1 auf 47/84, benachbarte Meerfelder 46/84 und 46/85.

Die vier **`preview-*.png` sind ausdrücklich Entwurfsaufnahmen**, mit sichtbarem
Hinweisbanner. Sie stammen aus einem temporären nativen Beobachter mit
`--bilder <dieses Prüfpaket>/sprites`. Für die Vorschau werden ausschließlich
lokale Renderkopien der natürlichen Küstenfischer auf ein benachbartes
Meerfeld gesetzt. Person 8 auf 44/77 zeigt Jagd und einen dekorativen Hirsch
als Bildgalerie; das behauptet keine echte Jagdhandlung oder echten Kadaver.
Die Spielsimulation selbst wird dabei nicht geändert. Maßgeblich für die
spätere Freigabe ist Claudes echte Anbindung, nicht diese Vorschau.

Wiederholbare Entwurfsprüfung:

```sh
python tools/nature/export_canoe_and_hunt.py
python tools/nature/check_canoe_and_hunt.py
python tools/nature/build_canoe_hunt_preview.py --output /tmp/nature-canoe-hunt-preview
python tools/nature/capture_canoe_hunt_review.py /tmp/nature-canoe-hunt-preview --preview --display :100
```

Der Vorschau-Bauer sichert die beiden Darstellungsdateien und das vorhandene
Programm, baut die abweichende Vorschau und stellt Produktivdateien und
Programm in `finally` wieder her. Der genaue Testunterschied ist in
`review-preview.patch` sichtbar. Er wird nicht in Produktivcode übernommen.
`*-captures.json` hält Startargumente und Art der jeweiligen Aufnahme fest.

Technisch geprüft: 88 Dateien/Matrix, echte Transparenz, getrennte Phasen,
eigenständig gezeichnete Richtungen, Quellhashes, Anker und Kalibrierfaktoren.
Alle 1672 Dateien unter `assets/nature/` bleiben gegenüber der Basis bytegleich.
Erneuter Export muss sämtliche Sprite-PNGs und Manifeste bytegleich erzeugen.

`cargo fmt --all`, 113 Rust-Tests, 15 Python-Tests und der Release-Bau bestehen
auf der unveränderten Basis. **Strenges Clippy scheitert bereits im
unveränderten Kern:** `crates/core/src/people.rs:1119`, `needless_range_loop`
beim Durchgehen von `s.lakes_today` (Rust 1.100.0). Keine Unterdrückung und keine
Kernänderung in diesem Bild-Draft. Claude muss den Basisfehler separat beheben.
Visuelle Abnahme und produktive Anbindung bleiben offen.
