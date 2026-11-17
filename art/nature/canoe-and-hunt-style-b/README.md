# Einbaum und Jagd – eingebettet auf v0.22.0

PR #26, Branch `codex/canoe-and-hunt-style-b`, Integrationsbasis `a39bb5b`.
Alle **88 Bilder** liegen jetzt auch in `assets/nature/people/` bzw.
`assets/nature/animals/`, mit Weltmaßen und Ankern in den jeweiligen Manifesten.
Claudes fertige Schnittstellen wählen sie aus; die produktive Zeichnung und
Simulation entsprechen unverändert v0.22.0. Visuelle Abnahme durch Claude offen.

| Bilder | Leinwand | Anker | Weltmaß | Dauer |
| --- | --- | --- | --- | --- |
| 40 `person_<0…4>_fish_boat_<l|r>_f0…f3` | 224×144 | 112/128 | 25,2×16,2 | 4×300 ms |
| 40 `person_<0…4>_hunt_<l|r>_f0…f3` | 144×120 | 72/112 | 16,2×13,5 | 4×250 ms |
| 4 `deer_hunted_<0|1>_rest_<l|r>_f0` | 192×128 | 96/64 | 21,6×14,4 | Standbild |
| 4 Hirsch-`@fern` | 48×32 | 24/16 | gleiches Weltmaß | Standbild |

Die Körpermaße der fünf Erwachsenen und alle Originalzeichnungen aus dem
Entwurf bleiben erhalten, ebenso beide eigenständig gezeichneten Richtungen.
Die zusätzliche Jagd-Leinwandbreite gehört dem Speer; die Körperauflösung
bleibt gleich. Einbaum-Anker: Wasserlinie unter dem Rumpf. Hirsch-Anker:
Motivmitte. Keine Kinderbilder. Quellen/Hashes/Faktoren in `sources.json` und
`export.json`, alle Phasen in `boat.gif` und `hunt.gif`.

## Echte Spielprüfung

Die **aktuellen sechs Aufnahmen** liegen in [v0.22-integration/](v0.22-integration/).
Sie stammen vom normalen Release-Programm mit eingebetteten Bildern, ohne
`--bilder`, vorläufige Meerplatzierung, erzwungene Tätigkeiten oder Galeriemotive.
Alle drei Fälle wurden bei Zoom 8 und 32 angesehen:

**Einbäume – Nutzer-Prüfstand:**

```sh
nature-in-a-glass --seed 5 --groesse gross --years 10 --tiere hirsche --menschen --danach 30 --select 45,85 --zoom 32
```

Die Fischer liegen auf Meerfeldern, die Rümpfe und Personen erscheinen in den
Manifestmaßen. Die Beobachtung aus dem Spiel: Auf gemeinsam genutzten
Meerfeldern überlagern sich einzelne Boote trotz des kleinen persönlichen
Versatzes. Das ist im Küstenbild sichtbar und bleibt ein Platzierungspunkt für
Claude. Zeichnung und `boat_tile`-Versatz wurden hier nicht verändert.

**Jagd – tatsächlicher Jagdtag, direkte CLI:**

```sh
nature-in-a-glass --seed 5 --groesse gross --years 10 --tiere hirsche --bei 28,64 --menschen --days 9 --select 28,65 --zoom 32
```

Eine Hirschgruppe wird über den normalen Eingriff am Ankunftsfeld ausgesetzt.
Die Bewohner beginnen selbst zu jagen. Im Bild sind Greta und Lone mit dem
Speer sichtbar. `hunting_today` wird absichtlich nicht im Spielstand gespeichert;
deshalb wird dieser Fall direkt abgespielt und nicht aus einer Datei geladen.

**Erlegter Hirsch – regulärer Spielstand:**

Der Küsten-Prüfstand bekommt nach den 30 Menschenjahren am Lager 44/77 fünf
weitere Hirschgruppen über `Intervention::ReleaseAnimals`. Danach läuft die
Simulation normal weiter. Am 22. zusätzlichen Tag erlegen die Bewohner einen
Hirsch auf 44/77. Der Kadaver hat bereits null Fleisch und noch zwei Tage
Sichtbarkeit; auch dieser Fall ist damit tatsächlich im Bild geprüft. Der regulär gespeicherte Zustand liegt komprimiert als
`v0.22-integration/deer.json.gz` bei; `hunted` ist darin wahr. Die Aufnahme
zeigt auch den echten Chronikeintrag über den Jagderfolg.

`fixture-probe.rs` dokumentiert den Erzeuger; weder Hunger, Können, Tätigkeit
noch Kadaverflag werden direkt gesetzt. Zum Ansehen den Spielstand neben eine
separate Kopie des Programms nach `spielstaende/welt.json` entpacken und mit
**Laden** öffnen. Die Aufnahmeskripte erledigen das in einem eigenen temporären
Ordner und überschreiben keine bestehenden Spielstände.

`cases.json` und `captures.json` enthalten alle Argumente und die Art des
Prüffalls. Die gespeicherten Bilder zeigen pausierte Ansichten um 10 Uhr;
die GIFs ergänzen die Prüfung der übrigen Phasen, sind keine Spielaufnahmen.
Alte v0.18-Entwurfsaufnahmen bleiben historische Vergleichsbilder. Ihr Kontext
steht in `README-v0.18.1.md`; der alte Vorschau-Bauer wurde entfernt.

## Export und technische Prüfung

```sh
python tools/nature/export_canoe_and_hunt.py
python tools/nature/check_canoe_and_hunt.py
python tools/nature/capture_canoe_hunt_review.py target/release/nature-in-a-glass --display :102
```

Pillow/NumPy/SciPy; für Aufnahmen Xvfb 1440×900, ImageMagick, X11/XTest und
Tesseract. Einzelne Fälle über `--only coast`, `--only hunt`, `--only deer`.
Der Export regeneriert proportionale Einzelbilder und ergänzt ausschließlich
seine eigenen Manifest-Einträge. Die vollständigen Menschen- und Tierexporte
rufen ihn ebenfalls auf, damit die neuen Bilder/Größen nicht wieder verloren
gehen. Alle Spiel-PNGs und beide Manifeste bleiben bei erneutem Export
bytegleich. Das ältere, nicht eingebettete `_people-testsheet.png` wird vom
bestehenden Menschenexport neu berechnet und wurde auf den Bestand zurückgesetzt.

Geprüft: 88er-Matrix, Originalquellen, Dateihashes, unveränderte bisherige
Bilddateien und Manifestdaten, Richtungen/Phasen sowie Fußpunkte/Weltmaße.
Der neue eingebettete Assettest prüft Größen und Anker aller Zustände; der
bestehende Atlas-Test prüft Auflösungsauswahl und beide Texturgrenzen.

Bestanden: fmt, strenges workspaceweites Release-Clippy, **119 Rust-Tests**,
zusätzlich 21 App-Tests nach der Exportprüfung, **15 Python-Tests**, Release-Bau,
beide Voll-Exportwege, Manifest-/Bestandsprüfung und die sechs nativen Ansichten.
Keine Versionsanhebung; Claude übernimmt nach visueller Freigabe.
