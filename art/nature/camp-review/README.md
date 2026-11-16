# Lagerplätze – erster Stilentwurf

Auftrag: `docs/gestaltung.md`, 16.3 Punkt 0a / 17.8, 16.11.2026.

`concept.png` zeigt oben kalte Feuerstelle, brennende Feuerstelle und Windschutz;
unten zwei Hüttenvarianten und ein verfallenes Lager. Erzeugt mit dem
Bildgenerator anhand der verbindlichen Stilreferenz
`art/nature/fox-ab/fox-b-imagegen-source.png`. Eigenständige Motive, transparente
Leinwand, Licht von rechts oben. Keine Übernahme in die eingebauten Spielbilder.

Dies ist ein gemeinsames Entwurfsblatt, kein fertiger Sprite-Export. Noch offen:
Einzeldateien mit festen Weltmaßen und Fußpunkten, vier echte Feuerphasen,
Fernansichten, Sichtprüfung im Spiel bei Zoom 4/8/32 und die Menschenzustände
`person_<aussehen>_build_<richtung>_f0…f3` (5 Aussehen, 2 Richtungen).
Vorräte sind laut Auftrag optional. Die Auswahl zu den bestehenden Fuchs- und
Feuerentwürfen ist mit diesem Blatt nicht vorweggenommen.

Technischer Befund für Claude: `crates/app/build.rs` bettet PNGs aus `buildings`
ein, aber nur Haupt-, Tier- und Menschenmanifest. `atlas.rs` übernimmt dieselben
drei eingebauten Manifeste. Ein separates Gebäudemanifest wird dagegen beim
Prüfimport mit `--bilder` eingelesen. Vor der festen Einbindung muss die
Gebäude-Metadatenquelle geklärt werden; keine Rendereränderung in diesem Schritt.

Der Generator liefert sechs statische Motive. Feueranimation, passende
Weltmaße und visuelle Freigabe werden damit ausdrücklich nicht als geprüft
ausgegeben.

## Prüfung und Übergabe

Branch `work`, Ausgangscommit `a289aa3a2e37fb9eaf8b5e4fa84e7bf7fbf71cf3`.
`spiel-vorher-zoom8.png` zeigt den tatsächlichen bisherigen Spielstand:
Seed 2024, 10 Jahre Natur, Menschen landen lassen, weitere 3 Jahre, Zoom 8.
Jahr 14: sieben lebende Menschen, ein Lager mit zwei Hütten; die Hütten sind
weiterhin die bisherigen Platzhalter. Das Konzept wurde noch nicht importiert.

Auf dieser Cloud-Maschine selbst ausgeführt:

- `cargo fmt --all --check`: bestanden.
- `cargo clippy --locked --release --all-targets -- -D warnings`: bestanden.
- `cargo test --locked --release --workspace`: 89 bestanden, 0 fehlgeschlagen,
  0 ignoriert; zusätzlich leere CLI-/Dokumentationstests ohne Testabdeckung.
- `python -m unittest discover -s tools/sprite_agent/tests -v`: 15 bestanden.
- Release-Bau von Spiel und `nig-sim`: bestanden.
- 100 Jahre Seed 42: Prüfsumme `4a8ec9638178bda3`; Speichern und Fortsetzen
  um ein Jahr erfolgreich, Folgeprüfsumme `858423bc7d1e1b32`.
- Spiel auf Xvfb gestartet und Bildschirmfoto angesehen. Erforderlich:
  `-extension MIT-SHM`, Software-Rendering und beschreibbarer Cache.
- Installationsskript erneut vollständig ausgeführt: erfolgreich.
- Sprite-Agent `doctor`: ComfyUI auf dem lokalen Endpunkt nicht vorhanden
  (`Connection refused`); keine lokale GPU-Bildgenerierung validiert.

Installationsskript und korrigierte Startanweisungen sind im Umgebungsentwurf
gespeichert. Veröffentlichung und Wiederherstellung in einer neuen Aufgabe
wurden nicht ausgeführt. Bestehende Quelldateien, Tests, Manifeste, Lockfile und
Projektstatusdokumente blieben während der Einrichtung unverändert.

Nächster Arbeitsschritt: Aus den Lagerentwürfen ein maßhaltiges Einzelbildpaket
mit Manifest und eigener Fernansicht erstellen und über `--bilder` mit diesem
Ausgangsbild bei Zoom 4/8/32 vergleichen. Danach echte Feuerphasen und den
Menschenzustand `_build` ausarbeiten. Noch kein Punkt der Gestaltungs-Abnahme
ist durch dieses Entwurfsblatt allein erledigt.
