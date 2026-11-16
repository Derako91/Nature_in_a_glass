# Werkzeuge in der Hand – Stil B

Draft zu Gestaltung 17.11, Basis `af87656` (v0.13.0), Branch
`codex/people-tools-style-b`. Visuelle Freigabe durch Claude offen.

140 neue Einzelbilder: Netz beim Fischen (40), Korb beim Sammeln (40) und
Essen (20), Steinbeil beim Holztragen (40). Fünf erwachsene Aussehen, jede
Richtung eigenständig gezeichnet. Das Beil bleibt beim Tragen im Gürtel;
die Holztrageschleife stellt keinen ständig erfolgreichen Axtschlag dar.
Kinder und die bisherigen Tätigkeiten bleiben unverändert.

## Maße und Export

112 × 120 RGBA, Fußpunkt 56/112, Weltmaß 12,6 × 13,5; vier Phasen zu
300 ms für Netz/Sammelkorb, zwei zu 600 ms beim Essen, vier zu 250 ms
beim Holztragen. Erkennungsfarben und Frisuren folgen den bestehenden fünf
Aussehen. Helles Fasernetz, brauner Flechtkorb, dunkle Feuersteinklinge.

15 originale Bildquellen in `sources/`, Herkunft und SHA256 in
`sources.json`. Die obersten vier Figuren jeder Quelle sind ausschließlich
stehende Größenreferenzen. Der Export ermittelt daraus **einen gemeinsamen
proportionalen Faktor pro Quelle**, für beide Richtungen und beim Korb für
beide Tätigkeiten. Die Größenreferenzen werden nicht eingebettet. Abgesetzte
Körbe werden erhalten, ohne den Sohlenanker zum Korb zu verschieben.

Keine getrennte Breiten-/Höhenskalierung, kein Spiegeln. Zu breite Posen
wurden im Quellbild neu gezeichnet (kompaktere Hocke bzw. kürzere Schritte),
nicht nachträglich zusammengedrückt. Deckende Breite höchstens bisheriges
Maximum derselben Tätigkeit/Aussehen plus ein Rasterpixel; Höhe höchstens
plus vier Pixel für abweichende Haltung. `export.json` nennt alle Maße und
Faktoren. Die niedrige Hocke entsteht durch die Pose, nicht durch kleinere
Köpfe. Der Korb beim Essen steht neben dem Knie innerhalb der bisherigen
Breite.

```sh
python tools/nature/export_people_tools.py
python tools/nature/check_people_tools.py
```

Pillow, NumPy und SciPy benötigt. Der vollständige Export
`tools/nature/export_people_style_b.py` ruft den Werkzeugexport nach den
bisherigen Erwachsenen-, Kinder-, Größenkorrektur- und Fischfangexporten auf.
Wiederholung erhält alle Sprite-PNGs und das Manifest bytegleich. Das ältere
`assets/nature/_people-testsheet.png` wird vom vollständigen Export neu
berechnet und wurde danach auf den unveränderten Bestand zurückgesetzt;
es gehört nicht zum eingebetteten Spielbildpaket.

## Ansichten und Prüfung

`*-contact.png` zeigt alle fünf Aussehen und beide Richtungen in Phase 0;
die vier GIFs zeigen sämtliche Phasen mit der tatsächlichen Bilddauer.
`zoom8-tools-comparison.png` ist eine technische Größenprobe (jeweils bisher /
Werkzeug), oben in Zielpixelgröße und darunter dreifach vergrößert. Die
Software-Atlasfilterung ist damit nicht exakt nachgebildet; maßgeblich sind
die echten Spielaufnahmen.

Die `before-*.png` stammen vom unveränderten v0.13.0-Programm, `after-*.png`
vom neu gebauten Programm mit eingebetteten Werkzeugbildern, ohne `--bilder`
und ohne Austausch von Tätigkeiten oder Kenntnissen. Befehle in den jeweiligen
`*-captures.json`:

```sh
./target/release/nature-in-a-glass --seed 5 --years 10 --menschen --danach 30 --select 19,44 --zoom 32
./target/release/nature-in-a-glass --seed 5 --years 10 --menschen --danach 30 --select 22,48 --zoom 32
./target/release/nature-in-a-glass --seed 5 --years 10 --menschen --danach 30 --days 2 --select 19,44 --zoom 32
```

Jeweils zusätzlich Zoom 8. Lager: Sammeln/Essen mit Korb, Feld 22/48: Nils
(Aussehen 3) fischt mit Netz. Nach zwei weiteren regulären Tagen trägt Greta
(Aussehen 0) Holz mit Beil im Gürtel. Ein unveränderter Diagnose-Lauf derselben
Simulation bestätigt am ersten Tag neun Erwachsene mit allen drei Kenntnissen
und ein Kind ohne Werkzeugfassung. Der Spielstand ist pausiert bei 10 Uhr;
die GIFs dienen ergänzend zur Prüfung aller Bildphasen, nicht als angebliche
Spielaufnahmen. Die gemeinsame Schnittstelle und der bestehende Fallback
bleiben unverändert.

`validation.json` dokumentiert Matrix, Manifestgleichheit, Quellhashes,
Maßgrenzen und 1483 unveränderte Eltern-PNGs. Der neue Rust-Assettest prüft,
dass alle 140 Bilder mit den korrekten Ankern/Weltmaßen eingebettet sind und
keine identischen Animationsphasen vorliegen. Der bestehende Atlas-Test prüft
alle Zoomstufen und die beiden 8192-Pixel-Grenzen.

Ausgeführt auf dieser Basis: `cargo fmt --all`,
`cargo clippy --release --all-targets -- -D warnings`,
`cargo test --release --workspace` (109 Tests), `cargo build --release`,
15 Python-Tests unter `tools/sprite_agent/tests`, Werkzeugexport und
Bestandsprüfung. Während der Arbeit weiterentwickeltes `main` enthält bereits
v0.14/v0.15; diese Bildänderung greift nicht in deren Simulation ein. Der
hier dokumentierte deterministische Spielvergleich bezieht sich auf die
Auftragsbasis v0.13.0, nicht auf die neueren Simulationsstände.
