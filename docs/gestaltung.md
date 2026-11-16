# Gestaltung – visuelle Richtung von Nature in a Glass

Stand: v0.4.3 · Geltungsbereich: **Naturdarstellung** (Gelände, Wasser, Vegetation, Feuer, Tiere ab Abschnitt 15). Bewohner, Gebäude und Oberfläche folgen später in eigenen Abschnitten.

Dieses Dokument ist die verbindliche Vorlage für Texturen, Sprites und Tiles. **Was als Nächstes zu tun ist, steht in Abschnitt 16.** Es ist so geschrieben, dass ein Werkzeug wie Codex daraus Schritt für Schritt Assets erzeugen kann: Jede Asset-Gruppe hat Maße, Dateinamen, Varianten, Zustände und Abnahmekriterien.

---

## 1. Leitbild

**Eine kleine Welt unter Glas, auf die man gerne schaut.**

- Miniaturwelt im Geist alter Aufbau- und Strategiespiele der späten 1990er: isometrische Draufsicht, gestufte Höhen, warme, handgemalte Anmutung, klare Formen. **Eigene Formen und Farben – keine bestehenden Spiele nachbilden oder Assets daraus übernehmen.**
- Nicht fotorealistisch, nicht verspielt-comichaft. Ruhig, natürlich, leicht stilisiert.
- Die Welt erzählt ihre Geschichte über Spuren: Brandflächen, Totholz, alte und junge Wälder, ausgetrocknete Bachbetten.

### 1.1 Konkretisierte Stilrichtung (17.10.2026)

Auf Wunsch des Projektinhabers orientiert sich der erste Stiltest stärker an
der Atmosphäre von **Die Siedler I und II**: warme, fein strukturierte Pixelgrafik,
erdige Grüntöne mit olivgelben Lichtern, kompakte Baumkronen und gut lesbare
kleine Landschaften. Die Referenz beschreibt die Anmutung; Motive und Assets
werden eigenständig erstellt.

- Kronen wirken organisch und leicht unregelmäßig, mit kleinen Blattgruppen.
- Flächen erhalten feine Materialstruktur; große glatte Farbflächen vermeiden.
- Alte Bäume wirken voller und gedrungener, junge Bäume bleiben schmal und hell.
- Weltmaße, Anker und Simulationszuordnung gelten weiter. Die neuere Stilentscheidung 1.2 ersetzt die bisherige Farbobergrenze und starre Dateiauflösungen.
- Prüfblatt 02 erprobt diese Richtung; es ist noch keine Freigabe der Asset-Serie.

### 1.2 Verbindliche Stilreferenz: Fuchs (18.10.2026)

Dennis hat den [Fuchs-Entwurf](../art/nature/fox-ab/fox-b-imagegen-source.png)
als gewünschte Richtung gewählt: plastische Körper, warme natürliche Farben,
erkennbare Anatomie und feinere Pixelstrukturen. Schwanz, Ohren und Schnauze
sollen die Art erklären; Fellflächen bekommen zusammenhängende Licht- und
Schattenformen statt zufälliger Einzelpixel. Keine Comic-Gesichter.

Diese Richtung gilt langfristig für Tiere, Menschen, Vegetation, Gelände und
Wasser. Zunächst wird nur ein Fuchs-Standbild geprüft. Der
[Vergleich](../art/nature/fox-ab/comparison.png) zeigt den bisherigen Projektstand
und den neuen Export bei Zoom 4, 8, 16 und 32 auf drei Böden. Er ist eine
synthetische Vorschau, keine Spielabnahme. Animationen und andere Arten folgen
erst nach dem Vergleich. Die Referenz ist eine Stilfreigabe, keine Freigabe
jedes Details der technischen Umsetzung.

Für diesen Nahansicht-Test wird der Fuchs als 80×48 RGBA exportiert, bei
unverändertem Weltmaß 10×6 und Anker 40/40 (Weltanker 5/5). Das erlaubt
Zoom 32 ohne Vergrößerung. Es ist keine pauschale Pflichtauflösung für alle
Assets. Vor einer Einbindung müssen sämtliche Frames derselben Art/Variante
auf eine gemeinsame Auflösung gebracht werden; keine Mischung mit 20×12.

## 2. Prioritäten (in dieser Reihenfolge)

1. **Wahrheit vor Schönheit.** Jedes Bild zeigt nur, was die Simulation tatsächlich berechnet. Kein Baum ohne Waldwert, kein Wasser ohne Durchfluss. Dekoration ohne Simulationsgrundlage ist nicht erlaubt.
2. **Lesbarkeit im Zeitraffer.** Bei „10 Jahre/s“ muss man Wald, Busch, Gras, Wasser, Totholz und Feuer auf einen Blick unterscheiden – an Farbe *und* Form, nicht nur an Details.
3. **Ruhe.** Nichts flackert oder springt. Varianten werden pro Feld fest gewählt (siehe 7.3), nicht zufällig pro Bild. Animationen sind langsam und dezent.
4. **Atmosphäre.** Erst wenn 1–3 erfüllt sind: Licht, Materialität, kleine Details.

## 3. Zwei Ansichten, eine Wahrheit

| | 2D-Karte | Isometrische Ansicht |
| --- | --- | --- |
| Zweck | Analyse, Überblick, Fehlersuche | Beobachten, Atmosphäre |
| Darstellung | **Flächenfarben, keine Texturen, keine Sprites** | Texturen, Sprites, Höhenstufen |
| Ebenen | Natur, Höhe, Bodenfeuchte, Wasser, Brandgefahr | vorerst nur Natur und Wasser mit Assets |

Die 2D-Karte bleibt bewusst schlicht. Alle Assets in diesem Dokument betreffen die isometrische Ansicht. Die Farbpalette (Abschnitt 5) gilt für beide, damit beide Ansichten sofort als dieselbe Welt erkennbar sind.

## 4. Geometrie und Raster

Diese Maße entsprechen dem heutigen Renderer (`crates/app/src/view.rs`) und dürfen nicht abweichen.

| Größe | Wert |
| --- | --- |
| Welt | 64 × 64 Felder, ein Feld = 50 × 50 m |
| Projektion | 2:1-Isometrie (Breite = 2 × Höhe der Raute) |
| **Asset-Grundmaß** | **Feldraute 64 × 32 px** (entspricht Zoom 4 des Renderers; dort ist eine Raute 16 × 8 px bei Zoom 1) |
| Höhenmaßstab | 0,8 px pro Meter bei Grundmaß (0,2 px/m bei Zoom 1) |
| Höhenstufen | Jedes Feld ist eine flache Platte auf seiner Höhe; zu tieferen Nachbarn entstehen senkrechte Seitenwände |
| Zeichenreihenfolge | von hinten nach vorne (Summe x + y aufsteigend), innerhalb eines Feldes: Boden → Seitenwände → Wasserlauf → Bodenbewuchs → Sprites (nach Fußpunkt-y sortiert) → Feuer |

### 4.1 Lichtrichtung

- Licht fällt **von oben rechts**. Die Oberseite eines Feldes hat volle Helligkeit.
- **Rechte Seitenwand** (zeigt nach vorne rechts): 72 % Helligkeit.
- **Linke Seitenwand** (zeigt nach vorne links): 56 % Helligkeit.
- Sprites: beleuchtete Seite rechts, Schattenseite links; Schlagschatten schräg nach **links unten**, weich, 25–35 % Deckkraft, nie über mehr als ein halbes Feld.
- Kein eingebautes Tageslicht in Texturen. Tag und Nacht färbt der Renderer selbst (nur bei 1 Std/s).

### 4.2 Maßstab der Natur

Ein Feld ist 50 m breit. Ein ausgewachsener Baum ist etwa 20–30 m hoch. Daraus folgt für das Grundmaß:

| Objekt | Breite × Höhe (px) | Bemerkung |
| --- | --- | --- |
| Jungbaum | 10 × 14 | |
| Baum mittel | 14 × 22 | |
| Altbaum | 18 × 30 | darf über die Raute nach oben hinausragen |
| Busch | 10 × 8 | |
| Grasbüschel | 6 × 4 | nur als Textur, nicht als Sprite |
| Totholz-Stamm, stehend | 6 × 22 | ohne Krone |
| Totholz, liegend | 16 × 5 | |

Die Bäume sind gegenüber dem echten Maßstab bewusst **etwas größer**, damit Wald als Wald lesbar bleibt. Dieses Verhältnis gilt für alle Baum-Sprites.

### 4.3 Zoomstufen – was man bei welchem Abstand erkennt

Festgelegt am 18.10.2026 mit Dennis. Die Kamera zoomt von 0,4 bis 32; die Stufe **Ganz nah** kam mit den Menschen in v0.6.0 dazu (Entscheidung B, siehe 17 und [ADR 0006](../decisions/0006-close-view-for-people.md)). Das Grundmaß (Raute 64 × 32 px) gilt bei Zoom 4; darunter verkleinert der Renderer, darüber vergrößert er pixelgenau.

| Stufe | Zoom | Feld am Bildschirm | Was man erkennt |
| --- | --- | --- | --- |
| **Überblick** | 0,4 – 1,5 | 6–24 px | Die ganze Insel: Wald, Wiese, Wasser, Fels, Feuer als Flächen. **Keine einzelnen Tiere** – nur Spuren (`tracks_<art>`), wo Tiere leben. |
| **Landschaft** | 1,5 – 4 | 24–64 px | Einzelne Bäume, Büsche, Totholz, Bäche. Hirsche, Wölfe und Füchse als kleine Figuren, Kaninchen als Gewimmel, Kadaver. |
| **Nah** | 4 – 8 | 64–128 px | Jedes Tier mit Haltung (frisst, läuft, ruht), jung oder alt, kräftig oder geschwächt. |
| **Ganz nah** (ab v0.6) | 8 – 32 | 128–512 px | Einzelne Menschen: wer es ist (Kleidung, Haar, Statur), was er gerade tut, ob er verletzt ist. Gesichter nur angedeutet. Später Hütten, Feuerstellen, Vorräte. |

Dazu die Zeitregel aus 15.5: Ab 1 Monat/s gibt es unabhängig vom Zoom nur Spuren.

Folgerungen für die Bilder:

- **Boden, Wände, Wasser, Pflanzen:** seit Dennis' Entscheidung vom 18.10.2026 ebenfalls in doppelter Pixelauflösung ausarbeiten. Das Grundmaß bleibt die Darstellungsgröße: Bodenrauten 64×32 werden als 128×64 gespeichert, Wände 32×32 als 64×64, Pflanzen mit verdoppelten Maßen und Ankern. `display_size` im Manifest hält das Weltmaß fest. Boden/Wände werden auf ihre feste Geometrie gelegt; Pflanzen skaliert der Renderer nach ihrer Darstellungsbreite. Keine doppelt großen Bäume. Zusätzliche Strukturen statt bloßer Pixelvergrößerung; Übersicht bei Zoom 2/4 und Nahansicht bei Zoom 8 prüfen. Referenz: `art/nature/ROUND-10.md`.
- **Tiere:** werden größer dargestellt als bisher (neue Maße in 15.3) und dürfen **in doppelter Auflösung** gezeichnet werden (z. B. Hirsch 28 × 24 statt 14 × 12). Der Renderer bringt jedes Tierbild auf die Darstellungsbreite aus 15.3 – gleich, in welcher Auflösung es vorliegt. Doppelt aufgelöste Tiere bleiben so bei Zoom 8 scharf. Wichtig: alle Bilder einer Art und Variante in derselben Auflösung.
- **Spuren** (`tracks_<art>`) sind das, was man im Überblick von den Tieren sieht. Sie sind deshalb wichtiger als die Kadaver.
- **Menschen (v0.6):** Dennis hat am 18.10.2026 **B** gewählt – eine eigene Nahansicht bis Zoom 32. Vorgaben in Abschnitt 17.

## 5. Farbpalette

Die Grundfarben stammen aus dem heutigen Renderer. Neue Assets bauen ihre Schattierungen **aus diesen Ankerfarben** auf (heller/dunkler, wärmer/kühler), damit Karte und isometrische Ansicht zusammenpassen.

| Rolle | Hex | Verwendung |
| --- | --- | --- |
| Hintergrund | `#0C1A34` | Fensterhintergrund hinter dem Meer |
| Tiefsee | `#122E68` | Meer ab 15 m Tiefe (bis 60 m auf diesen Ton verlaufend) |
| Flachwasser | `#4A96C4` | Meer bis 15 m Tiefe |
| Strand | `#DED09A` | Land unter 3 m Höhe |
| Offener Boden | `#967E58` | Land ohne Bewuchs |
| Fels | `#8A847C` | Bergland (oberste 30 % der Inselhöhe) |
| Gras feucht | `#68A444` | Gras bei guter Bodenfeuchte |
| Gras trocken | `#BAAC5C` | Gras bei Trockenheit |
| Busch | `#76823C` | Buschland |
| Jungwald | `#46843A` | Wald, Alter < 15 Jahre |
| Altwald | `#1E502A` | Wald, Alter ≥ 60 Jahre (dazwischen stufenlos) |
| Totholz | `#7A6854` | abgestorbene Bäume |
| Brandfläche | `#302A26` | verkohlter Boden |
| Feuer | `#F56E1E` | brennende Felder, Kern heller (`#FFC060`) |
| Teich/See | `#3A76BA` | stehendes Wasser |
| Wasserlauf | `#4E96E6` | Rinnsale und Bäche |
| Quelle | `#78E6FF` | Quellmarkierung |
| Quelle versiegt | `#966E46` | |

Regeln:
- **Keine feste Farbanzahl.** Farben aus der jeweiligen Ankerfamilie entwickeln; zusätzliche Abstufungen sind für Fell, Rinde, Blätter, Stein und Wasser ausdrücklich erlaubt. Zusammenhängende Material- und Lichtflächen bewahren, Farbrauschen vermeiden.
- Kein reines Schwarz, kein reines Weiß.
- Vegetation **leicht entsättigt** anlegen, damit der Renderer sie für Trockenheit oder Jahreszeit sanft tönen kann (multiplikativ, höchstens ±20 % Helligkeit).
- Wasser, Feuer und Quelle sind die einzigen gesättigten Farben – sie sollen sofort ins Auge fallen.

## 6. Lesbarkeit

- **Formsprache je Vegetationsart:** Gras = feine, flache Struktur ohne Höhe. Busch = niedrige, runde Klumpen. Jungwald = schmale, helle, spitze Kronen. Altwald = breite, dunkle, geschlossene Kronen. Totholz = senkrechte graue Striche ohne Krone. Brandfläche = dunkel mit Ascheflecken.
- **Silhouettentest:** Jede Art muss als einfarbige Silhouette auf 50 % Größe noch von den anderen unterscheidbar sein.
- **Kontrast zum Untergrund:** Sprites haben eine um mindestens 15 % dunklere Schattenseite als der Boden darunter. Keine schwarzen Umrisslinien; wo nötig, eine 1-px-Kante in der dunkelsten Farbe des Assets.
- **Dichte zeigt Menge:** Wie viel Wald ein Feld hat, zeigt die **Anzahl** der Bäume (Abschnitt 7.2), nicht deren Größe. Wie alt er ist, zeigt **Form und Farbe**.
- **Wasser schlägt alles:** Wasserläufe und Teiche liegen über dem Bodenbewuchs und unter den Baumkronen. Ein Bach durch einen Wald bleibt zwischen den Stämmen sichtbar.

## 7. Von der Simulation zum Bild

Jedes Asset wird durch Werte aus dem Simulationskern ausgewählt (`Tile` in `crates/core/src/world.rs`). Diese Tabelle ist die Schnittstelle zwischen Simulation und Grafik.

**Umgesetzt in `crates/core/src/appearance.rs`** (ohne Grafik, getestet): `ground()`, `overlays()`, `sprites()`, `stream()`, `tree_count()`, `shrub_count()`, `TreeStage::from_age()` und `positions()` für feste Fußpunkte. Ein Renderer fragt diese Funktionen pro Feld ab und muss die Regeln nicht selbst nachbauen. Ändern sich Schwellenwerte, dann dort und in dieser Tabelle gemeinsam.

### 7.1 Zuordnung

| Simulationswert | Bereich | Visuelle Wirkung |
| --- | --- | --- |
| `elevation` ≤ 0 | Meer | Wasser-Tile; Tiefe −15 m trennt Flach- und Tiefsee |
| `elevation` < 3 m | Strand | Strand-Tile, Bewuchs höchstens 35 % |
| Höhe ≥ 70 % der Inselhöhe | Bergland | Fels-Tile, Bäume nur bis zur Baumgrenze |
| `grass` 0…1 | Grasdecke | Deckkraft der Gras-Textur über dem Boden |
| `soil_moisture / soil_capacity` | Feuchte | Gras-Tönung feucht ↔ trocken |
| `shrub` 0…1 | Buschland | 0–4 Busch-Sprites |
| `forest` 0…1 | Waldbedeckung | 0–6 Baum-Sprites |
| `forest_age` | Bestandsalter | < 15 J. Jungbaum · 15–60 J. Baum mittel · ≥ 60 J. Altbaum |
| `tree_supply` < 0,3 | Wassermangel | Kronen leicht ausgeblichen (Zustand „gestresst“) |
| `deadwood` > 0,1 | Totholz | stehende Stämme; > 0,6 zusätzlich liegende Stämme |
| `fire` > 0 | Brand | Feuer-Animation, Stärke nach Intensität |
| `burn_scar` 0…1 | Brandspur | Deckkraft der Brandflächen-Textur, verdeckt durch nachwachsendes Gras |
| `surface_water` > 150 mm | Teich/See | Wasseroberfläche statt Boden |
| `flow` ≥ 15 mm/Tag | Rinnsal | Wasserlauf-Linie, schmal, blass |
| `flow` ≥ 60 mm/Tag | Bach | Wasserlauf-Linie, breiter, satt |
| Quelle | `springs` | Quell-Marker (fließend/versiegt) |
| `soil_depth` < 0,05 m | abgespült bis aufs Gestein | Fels-Tile, auch im Tief- und Hügelland (ab v0.3) |
| `erosion_rate` > 0 | Ablagerung (m/Jahr) | Deckkraft von `ground_silt`, volle Deckung ab 2 mm/Jahr, unter Wald ausgeblendet |
| aufgetauchtes Schwemmland | früheres Meeresfeld, jetzt Land unter 3 m | wie Strand: `ground_sand` |

### 7.2 Anzahl der Sprites pro Feld

| `forest` | Bäume | `shrub` | Büsche |
| --- | --- | --- | --- |
| < 0,05 | 0 | < 0,1 | 0 |
| 0,05–0,2 | 1 | 0,1–0,3 | 1 |
| 0,2–0,4 | 2 | 0,3–0,6 | 2 |
| 0,4–0,6 | 3 | 0,6–0,8 | 3 |
| 0,6–0,8 | 4–5 | ≥ 0,8 | 4 |
| ≥ 0,8 | 6 | | |

Höchstens **6 Baum- und 4 Busch-Sprites pro Feld** (Leistungsgrenze: 4096 Felder).

### 7.3 Feste Varianten statt Zufall

- Welche Variante und welche Position ein Sprite bekommt, ergibt sich aus einem festen Hash von **Feldkoordinate, Insel-Seed und Sprite-Nummer**. Dasselbe Feld sieht also immer gleich aus, bei jedem Start und auf jedem Rechner.
- Wächst ein Wald von 3 auf 4 Bäume, bleiben die ersten 3 an ihrem Platz; nur einer kommt dazu.
- Positionen innerhalb der Raute: zufällig verteilt mit Mindestabstand 6 px zwischen Fußpunkten, Fußpunkt mindestens 4 px vom Rautenrand.

## 8. Asset-Gruppen (erste Ausbaustufe)

Format für alle: **PNG, RGBA mit 8 Bit je Kanal, sRGB**. Halbtransparente Kanten und freie Deckkraft sind erlaubt; keine hellen oder dunklen Hintergrundsäume. Kein Hintergrund, keine eingebauten Schatten außer dem Schlagschatten der Sprites. Auflösung nach benötigter Nahansicht wählen, Weltmaß unverändert lassen (16.5). Tabellen mit alten Dateimaßen dokumentieren die bisherigen Exporte, keine technische Obergrenze.

### Gruppe A – Boden-Tiles (Oberseite der Raute)

Rautenförmig, 64 × 32 px, außerhalb der Raute transparent. Kacheln müssen nahtlos an alle vier Nachbarn passen.

| ID | Inhalt | Varianten |
| --- | --- | --- |
| `ground_soil` | offener, brauner Boden | 4 |
| `ground_sand` | Strand, feiner heller Sand | 4 |
| `ground_rock` | Fels, kantig, grau | 4 |
| `ground_grass_wet` | dichtes, sattes Gras (als Überlagerung, mit Transparenz an lückigen Stellen) | 4 |
| `ground_grass_dry` | gelbliches, lückiges Gras (Überlagerung) | 4 |
| `ground_burnt` | verkohlter Boden mit Ascheflecken (Überlagerung) | 3 |
| `ground_shrub_floor` | Laub und Zweige unter Buschland (Überlagerung) | 2 |
| `ground_forest_floor` | Waldboden mit Nadeln und Laub (Überlagerung) | 2 |
| `ground_silt` | frisch angeschwemmter, feiner grau-brauner Schlamm mit Rinnenmuster (Überlagerung, ab v0.3) | 3 |

Überlagerungen werden vom Renderer mit Deckkraft nach Simulationswert (Abschnitt 7.1) über den Grundboden gelegt.

### Gruppe B – Seitenwände

Rechteckige Streifen, **32 × 32 px**, vertikal nahtlos kachelbar (der Renderer stapelt sie je nach Höhenunterschied). Je Material eine Fassung für die rechte und die linke Wand – oder eine Fassung, die der Renderer mit 72 % bzw. 56 % abdunkelt.

| ID | Inhalt | Varianten |
| --- | --- | --- |
| `wall_soil` | Erdschichten, Wurzeln | 2 |
| `wall_sand` | Sandkante | 2 |
| `wall_rock` | Felswand, Schichtung | 3 |

### Gruppe C – Wasser

| ID | Format | Inhalt | Varianten / Frames |
| --- | --- | --- | --- |
| `water_shallow` | Raute 64 × 32 | Flachwasser, heller Grund durchscheinend | 4 Frames, Schleife 2,4 s |
| `water_deep` | Raute 64 × 32 | Tiefsee, ruhige Dünung | 4 Frames, Schleife 3,2 s |
| `water_coast` | Raute 64 × 32 | Brandungssaum an Landkanten, je Kantenrichtung (N, O, S, W) | 4 Richtungen × 4 Frames |
| `water_pond` | Raute 64 × 32 | Teich/See, ruhiger, dunkler als Meer | 2 Frames, Schleife 4 s |
| `water_stream` | Streifen 16 × 32, längs kachelbar | Wasserlauf von Feldmitte zu Feldmitte; der Renderer verbreitert ihn nach Durchfluss | 4 Frames, Schleife 1,6 s |
| `waterfall_stream_left/right` | Streifen 16 × 32 | schmaler Fall an den beiden sichtbaren Hangseiten, Licht rechts oben | je 4 Frames, Schleife 1,6 s |
| `waterfall_river_left/right` | Streifen 16 × 32 | breiter Fall mit ruhigem Aufprallsaum | je 4 Frames, Schleife 1,6 s |
| `marker_spring` | 12 × 12 | Quelle: kleiner Quelltopf, sprudelnd | 3 Frames + 1 Bild „versiegt“ |

Wasser-Animationen: höchstens 15 % Helligkeitsunterschied zwischen Frames (Ruhe-Regel).

Gebirgsgewässer: Ein Wasserfall verbindet einen durchflossenen Abschnitt nur mit
einem tiefer liegenden, durchflossenen Nachbarn. Breite folgt dem Durchfluss,
Fallhöhe der sichtbaren Höhendifferenz; kein dekorativer Wasserfall ohne diese
Grundlage. Felsufer gehen mit dem Bodenmaterial in feuchte Erd- und Grasränder
über, während das Wasser verbunden bleibt. Referenz und Zusatzmanifest:
`art/nature/ROUND-08.md`, `assets/nature/mountain-water/manifest.json`.
Die dortigen Breiten/Höhen sind Darstellungsprüfwerte, keine neuen Kernschwellen.

### Gruppe D – Vegetation (Sprites)

Fußpunkt = unterste Mitte des Stamms, im Dateinamen-Manifest angegeben (siehe 9). Jede Art in drei Zuständen.

| ID | Maße (px) | Varianten | Zustände |
| --- | --- | --- | --- |
| `tree_young` | 10 × 14 | 4 | `normal`, `stressed` (ausgeblichen), `autumn` (vorbereitet, noch ungenutzt) |
| `tree_mid` | 14 × 22 | 4 | wie oben |
| `tree_old` | 18 × 30 | 4 | wie oben |
| `shrub` | 10 × 8 | 4 | `normal`, `stressed`, `autumn` |
| `snag_standing` | 6 × 22 | 3 | `normal`, `charred` (verkohlt) |
| `log_fallen` | 16 × 5 | 3 | `normal`, `charred` |

Schlagschatten ist **Teil des Sprites** (links unten, 25–35 % Deckkraft, eigene Farbe `#0C1A34`).

### Gruppe E – Feuer

| ID | Maße (px) | Inhalt | Frames |
| --- | --- | --- | --- |
| `fire_ground` | Raute 64 × 32 | Glutsaum am Boden (Gras- und Buschfeuer) | 4, Schleife 0,8 s |
| `fire_flame_small` | 10 × 16 | Flamme, niedrig | 4 |
| `fire_flame_large` | 16 × 28 | Flamme, Kronenfeuer | 4 |
| `smoke` | 24 × 32 | Rauchfahne, halbtransparent, wird vom Renderer in Windrichtung geneigt | 4 |

Feuer ist die einzige schnelle Animation, damit ein Brand auch im Zeitraffer auffällt. Trotzdem keine grellen Blitze: Helligkeitsunterschied zwischen Frames höchstens 25 %.

## 9. Dateien und Manifest

```
assets/
  nature/
    ground/ground_soil_0.png … ground_soil_3.png
    walls/wall_rock_0.png …
    water/water_shallow_f0.png … water_shallow_f3.png
    vegetation/tree_old_2_stressed.png
    fire/fire_flame_large_f1.png
    manifest.json
```

Namensschema: `<id>_<variante>[_<zustand>][_f<frame>].png`, alles klein, Unterstrich als Trenner, Variante und Frame ab 0.

`manifest.json` beschreibt jedes Asset, damit der Renderer es ohne Raten laden kann:

```json
{
  "version": 1,
  "base_tile": { "width": 64, "height": 32 },
  "assets": [
    {
      "id": "tree_old",
      "group": "vegetation",
      "size": [18, 30],
      "anchor": [9, 29],
      "variants": 4,
      "states": ["normal", "stressed", "autumn"],
      "frames": 1
    },
    {
      "id": "water_shallow",
      "group": "water",
      "size": [64, 32],
      "anchor": [32, 16],
      "variants": 1,
      "states": ["normal"],
      "frames": 4,
      "frame_ms": 600
    }
  ]
}
```

`anchor` ist der Punkt im Bild, der auf dem Fußpunkt (Sprites) bzw. der Rautenmitte (Tiles) liegt.

### 9.1 Was ins Repository gehört

Jedes Bild im Repository bleibt für immer in dessen Geschichte – auch nach dem Löschen. Damit das Repository klein und übersichtlich bleibt, gilt:

**Ins Repository:**
- fertige Assets unter `assets/` samt `manifest.json` und Prüfbericht,
- ComfyUI-Workflows (`*.api.json`) und Ausführungsprotokolle (`*.history.json`), also alles, was zum Nachvollziehen eines Bildes nötig ist,
- Skripte unter `tools/`,
- Prüfblätter, die zur Abnahme gezeigt werden.

**Nur lokal oder über Git LFS:**
- rohe ComfyUI-Ausgaben, verworfene Varianten, Zwischenstände,
- große Quellbilder, sobald eine Runde abgenommen ist.

**Keine Doppelungen:** Eine Datei liegt nur an einer Stelle. Übernimmt eine neue Runde Vorlagen aus einer früheren unverändert, liest das Skript sie dort, wo sie schon liegen, statt sie zu kopieren. Ein gesicherter Altstand (z. B. `art/nature/proof-01/`) enthält nur, was sich vom aktuellen Stand unterscheidet; alles andere steht in der Git-Geschichte.

Vor einem Pull Request lohnt die Prüfung auf identische Dateien:

```
git ls-files -s | awk '{print $2}' | sort | uniq -d
```

Gibt der Befehl etwas aus, liegt mindestens eine Datei doppelt vor.

**Rust-Prüfungen:** Wer nur Assets, Prüfblätter, Skripte unter `tools/` oder Dokumente ändert, braucht lokal kein Rust (`cargo`). Die Prüfungen laufen nach dem Hochladen automatisch auf GitHub für Windows, macOS und Linux; ihr Ergebnis steht am Pull Request. Nur wer Code unter `crates/` ändert, sollte vorher selbst prüfen (Rust über https://rustup.rs installieren).

### Bestätigte Biomrichtung (17.10.2026)

Die gezeigten Biomübergänge sind vom Projektinhaber als Gestaltungsrichtung
bestätigt: lockere Waldsäume und Lichtungen, unregelmäßige Ufer sowie sanfte
Wechsel zwischen grünen und trockenen Bodenflächen. Bachkurven werden weich
geführt, Felsstufen in kürzeren, versetzten Abschnitten ausgearbeitet. Referenzen:
`assets/nature/_biomes-refined.png`, `assets/nature/_biome-transitions.png` und
`art/nature/ROUND-07.md`. Die Referenzen sind Prüfentwürfe; Spielintegration und
Abnahme bei tatsächlichem Zoom stehen weiterhin aus.

## 10. Arbeitsreihenfolge für die Asset-Erstellung

1. **Prüfblatt** `assets/nature/_testsheet.png`: eine 8 × 8-Felder-Insel in Isometrie mit allen Bodenarten, je einem Feld mit 1, 3 und 6 Bäumen jeder Altersstufe, Busch, Totholz, Brandfläche, Bach und Teich. Dient als Referenz für alle weiteren Schritte und wird zuerst abgenommen.
2. Gruppe A (Boden) und B (Wände).
3. Gruppe C (Wasser) ohne Animation, dann mit Animation.
4. Gruppe D (Vegetation) in `normal`, dann `stressed`, dann Totholz.
5. Gruppe E (Feuer).
6. Erst danach: `autumn`-Zustände und weitere Varianten.

## 11. Abnahmekriterien je Asset

- [ ] Maße exakt wie in der Tabelle, Anker im Manifest eingetragen.
- [ ] Transparenter Hintergrund, keine Fransen an den Kanten.
- [ ] Stimmige Ankerfarbfamilie, plastische Lichtflächen, keine störenden Farbsäume; keine feste Farbobergrenze (Abschnitt 5).
- [ ] Licht von oben rechts, Schatten links unten.
- [ ] Tiles: nahtlos in alle Richtungen; Wände: vertikal nahtlos.
- [ ] Silhouettentest bei 50 % Größe bestanden (Abschnitt 6).
- [ ] Auf dem Prüfblatt neben den Nachbar-Assets stimmig, keine Variante fällt auf.
- [ ] Animationen: Helligkeitsunterschied zwischen Frames innerhalb der Grenze, Schleife ohne Sprung.

## 12. Was ausdrücklich nicht gewünscht ist

- Dekoration ohne Simulationsgrundlage (Blumen, Pilze, Steine, Vögel), solange die Simulation sie nicht kennt. Tiere nur die vier Arten aus Abschnitt 15 und nur dort, wo die Simulation sie führt.
- Zufällig wechselnde Varianten, Wackeln, Partikelregen, Linsenreflexe.
- Eingebaute Tageszeit-Beleuchtung in Texturen.
- Nachbildungen bekannter Spiele, Marken oder Figuren.
- Texturen in der 2D-Analysekarte.

## 13. Offene Punkte für spätere Fassungen

- Jahreszeiten (Herbstfärbung, Schnee) – die Simulation kennt Temperatur, aber noch keinen Schnee.
- Baumarten mit eigener Silhouette, sobald die Simulation sie unterscheidet.
- Erosion (v0.3) ist simuliert; erste Zuordnung siehe 7.1 (`ground_silt`, Fels bei abgespültem Boden). Offen: Ufer, Kies und Abbruchkanten an tief eingeschnittenen Bächen.
- Tiere (v0.4): siehe Abschnitt 15. Bewohner (v0.6) folgen in einem eigenen Abschnitt.
- Technische Einbindung: umgesetzt in v0.4.3 ([ADR 0005](../decisions/0005-asset-atlas-in-the-observer.md)). Alle Bilder unter `assets/nature/{ground,walls,water,vegetation,animals}` werden eingebettet und über einen Atlas gezeichnet; Umschalter „Grafik“ im Beobachter. Neue Bilder nach dem Namensschema erscheinen beim nächsten Bauen von selbst. Offen für die Gestaltung: Tönung bei Trockenheit, Wasseranimation, Feuer, Übergänge zwischen Feldern.

## 14. Unterschiedliche Biome – erste Gestaltungsentwürfe

Auf Wunsch des Projektinhabers kommen unterschiedliche Biome zur Stilrichtung
Siedler I & II hinzu. Das erste Vergleichsblatt steht unter
`assets/nature/_biomes-testsheet.png`.

Die sechs Entwürfe sind zunächst **Landschaftsbilder aus vorhandenen Zuständen**,
keine neuen simulierten Ökosysteme oder feste Biome-IDs. Der Kern unterscheidet
derzeit Gras, Busch und Wald, aber keine Baumarten. Daher werden dieselben
Naturmotive über Dichte, Bestandsalter, Material, Feuchte und Gelände variiert.

| Entwurf | Bildsprache | Grundlage für die spätere Zuordnung |
| --- | --- | --- |
| Feuchter Wald | dunkler Boden, dicht geschlossene Kronen, wenig Gras | hohe `forest`- und Feuchtewerte, `forest_age` |
| Wiesenland | offene grüne Flächen, lockere Baumgruppen | viel `grass`, mäßig `forest`, gute Bodenfeuchte |
| Trockenes Buschland | niedrige olivgrüne Büsche, warmer offener Boden | viel `shrub`, wenig `forest`, geringe Bodenfeuchte |
| Steppe | strohfarbenes Gras, Lücken, kaum Bäume | trockene Grasdecke, sehr wenig `forest` |
| Feuchtgebiet | dunkle Ufer, Wasserflächen, Bewuchs am Rand | hohe Feuchte, `surface_water > 150` für offenes Wasser |
| Felsiges Bergland | graue Schichtung, Höhenstufen, spärlicher Bewuchs | relative Höhe ab 70 %, geringe `soil_capacity`, tatsächlicher Bewuchs |

Regeln für die Weiterarbeit:

- Ein Biomename erzeugt niemals zusätzliche Bäume, Wasser oder Dekoration.
- Übergänge sollen aus den Feldwerten entstehen; keine zufällig wechselnden Biome.
- Farben allein reichen nicht: Dichte, Freiflächen, Wasseranteil und Höhenprofil
  müssen die Landschaften auch ohne Beschriftung unterscheiden.
- Ein langfristiges Klimabiom darf nicht aus einem einzigen trockenen Tag
  abgeleitet werden. Verbindliche Zuordnungsregeln und zeitliche Glättung werden
  erst bei der späteren technischen Einbindung festgelegt.
- Regenwald, Wüste, Taiga und Schneegebiete sind mögliche weitere Richtungen.
  Eigene Arten, Sandböden im Binnenland und Schnee brauchen zusätzliche
  Simulationsgrundlagen; sie sind in diesem ersten Blatt nicht vorgetäuscht.

Die gezeigten Felder verwenden dokumentierte Testwerte und keine laufende
Simulation. Neue Bodenprototypen sind weiterhin 64×32 px, RGBA/sRGB, transparent
außerhalb der Raute und auf höchstens sechs Töne begrenzt. Ihre Farben sind
warme beziehungsweise dunkle Ableitungen des Bodenankers und des Felsankers.

## 15. Tiere (v0.4) – Vorgaben für die ersten vier Arten

Stand 17.10.2026. Die Simulation der Tiere wird gerade geplant (`docs/ROADMAP.md`, v0.4). Dieser Abschnitt legt fest, **was der Kern liefern wird** und welche Assets dazu passen, damit Gestaltung und Simulation parallel arbeiten können. Ändert sich beim Bau der Simulation etwas an der Schnittstelle, wird dieser Abschnitt im selben Schritt angepasst.

### 15.1 Grundsätze

- **Nur echte Tiere.** Jedes gezeichnete Tier entspricht einem Tier oder einer Tiermenge im Kern. Keine Deko-Herden, keine Tiere auf einer tierfreien Insel. Eine neue Insel ist tierfrei; Tiere kommen nur durch den Eingriff „Tiere aussetzen“.
- **Tote Tiere bleiben kurz sichtbar.** Ein Riss oder ein verendetes Tier liegt als Kadaver einige Tage auf dem Feld, dann verschwindet er (Prinzip „Spuren erzählen die Geschichte“).
- **Natürlich, nicht niedlich.** Keine großen Augen, keine Comic-Proportionen, keine Gesichter. Tiere sind kleine, klar lesbare Silhouetten in Erdtönen – im Geist der Stilrichtung 1.1.
- **Lesbar vor maßstäblich.** Echt maßstäblich wäre ein Hirsch bei 50 m pro Feld nur 1–2 px groß. Tiere sind deshalb – wie die Bäume – bewusst vergrößert, bleiben aber **deutlich kleiner als ein Jungbaum**, damit die Landschaft die Hauptsache bleibt.

### 15.2 Arten und Darstellungsart

| Art | ID | Rolle | Im Kern | Darstellung |
| --- | --- | --- | --- | --- |
| Hirsch | `deer` | frisst Gras, Busch, junge Bäume | einzelne Tiere, lebt in Rudeln | jedes Tier einzeln |
| Wolf | `wolf` | jagt vor allem Hirsche | einzelne Tiere, lebt in Rudeln | jedes Tier einzeln |
| Fuchs | `fox` | jagt vor allem Kaninchen | einzelne Tiere, meist allein | jedes Tier einzeln |
| Kaninchen | `rabbit` | frisst vor allem Gras | **Bestand pro Feld** (zu viele für Einzeltiere) | 0–3 Sprites pro Feld nach Dichte |

Kaninchen-Dichte → Sprites pro Feld (analog 7.2):

| Kaninchen pro Feld | Sprites |
| --- | --- |
| < 1 | 0 |
| 1–5 | 1 |
| 5–15 | 2 |
| ≥ 15 | 3 |

### 15.3 Maße im Grundmaß (Raute 64 × 32 px)

Neu seit 18.10.2026 (4.3): Die ersten Maße (Hirsch 9 × 8, Kaninchen 3 × 3) waren im Spiel zu klein. Die **Darstellungsgröße** ist jetzt etwa das 1,5- bis 2-Fache. Tiere bleiben **niedriger als ein Jungbaum** (14 px).

| ID | Darstellung B × H (px) | doppelt aufgelöst | Fußpunkt | Bemerkung |
| --- | --- | --- | --- | --- |
| `deer` | 14 × 12 | 28 × 24 | Mitte zwischen den Hufen | Geweih nur bei der Altersstufe `adult`, schlicht |
| `deer_young` | 9 × 8 | 18 × 16 | wie oben | Kalb, heller, ohne Geweih |
| `wolf` | 13 × 8 | 26 × 16 | Mitte unter dem Körper | grau, niedriger und gestreckter als der Hirsch |
| `fox` | 10 × 6 | 20 × 12 | wie oben | rötlich, buschiger Schwanz als Erkennungsmerkmal |
| `rabbit` | 6 × 5 | 12 × 10 | Mitte unten | graubraun, heller Fleck am Schwanz |
| `carcass_large` | 14 × 6 | 28 × 12 | Mitte | liegender Hirsch- oder Wolfskörper, entsättigt |
| `carcass_small` | 7 × 3 | 14 × 6 | Mitte | Fuchs oder Kaninchen |

Maßgeblich für den Renderer ist nur die **Breite**: Er misst das Bild `<id>_<variante>_graze_<richtung>_f0` und skaliert alle Bilder dieser Art so, dass es die Darstellungsbreite hat (Kadaver: das Kadaverbild selbst). Die heutigen Bilder (9 × 8 usw.) werden deshalb schon vergrößert gezeigt, bis Codex neue liefert – dann etwas pixelig, aber in der richtigen Größe. Der `anchor` im Manifest wird in der Auflösung des Bildes angegeben.

Schlagschatten wie bei der Vegetation: Teil des Sprites, links unten, 25–35 % Deckkraft, Farbe `#0C1A34`, höchstens 2 px versetzt.

### 15.4 Zustände, Richtung, Animation

Der Kern liefert je Tier einen **Zustand** und eine **Blickrichtung**. Der Renderer wählt daraus das Sprite.

| Zustand | ID-Anhang | Bedeutung im Kern | Frames |
| --- | --- | --- | --- |
| stehen / fressen | `_graze` | Tier frisst auf seinem Feld | 2 (Kopf hoch / Kopf unten), 1,2 s |
| gehen | `_walk` | wechselt gemächlich das Feld | 4, 0,8 s |
| laufen | `_run` | Flucht oder Jagd | 4, 0,4 s |
| ruhen | `_rest` | liegt (nachts, bei Hitze) | 1 |

- **Richtungen:** zwei Blickrichtungen genügen, `_l` (nach links unten schauend) und `_r` (nach rechts unten). Tiere, die sich nach oben bewegen, nutzen dieselben Sprites. **Nicht spiegeln:** Das Licht kommt von rechts oben und würde sonst mitgespiegelt – beide Richtungen werden eigens gezeichnet.
- **Altersstufen:** `young` und `adult`. Nur der Hirsch bekommt ein eigenes Jungtier-Sprite; bei Wolf und Fuchs wird das erwachsene Sprite auf 75 % verkleinert (vom Renderer).
- **Zustand „geschwächt“** (Hunger, Alter) wird nicht eigens gezeichnet: Der Renderer entsättigt das Sprite um bis zu 25 %, wie bei gestressten Bäumen.
- Dateischema wie in Abschnitt 9: `<id>_<variante>_<zustand>_<richtung>_f<frame>.png`, z. B. `deer_0_walk_r_f2.png`. Je Art **2 Varianten** (leicht andere Fellfarbe), fest pro Tier gewählt.

### 15.5 Ruhe im Zeitraffer

Tiere bewegen sich im Kern einmal pro Tag. Damit nichts zappelt, gilt für den Renderer:

| Geschwindigkeit | Darstellung |
| --- | --- |
| 1 Std/s | Tiere gleiten zwischen Feldern, Animation läuft |
| 1 Tag/s, 1 Woche/s | Tiere stehen in ihrem Zustand am aktuellen Platz, nur `_graze`/`_rest` ohne Laufanimation |
| ab 1 Monat/s | keine einzelnen Tiere; stattdessen eine **dezente Dichteanzeige** pro Feld (z. B. kleine Spuren im Gras), damit man sieht, wo Tiere leben |

Für die Dichteanzeige ab 1 Monat/s braucht es je Art ein Overlay `tracks_<art>` (Raute 64 × 32 px, Trittspuren bzw. Fraßspuren, höchstens 3 Töne, Deckkraft wird vom Renderer nach Dichte gesteuert).

### 15.6 2D-Karte

Die Karte bekommt eine Ebene **Tiere**: Hirsche, Wölfe und Füchse als einzelne Punkte (2 × 2 px bei Zoom 1), Kaninchen als Flächentönung nach Dichte. Feste Kartenfarben (neu in der Palette):

| Rolle | Hex |
| --- | --- |
| Hirsch | `#B07A44` |
| Wolf | `#9AA0A6` |
| Fuchs | `#D2682C` |
| Kaninchen (Tönung) | `#C8B898` |
| Kadaver | `#5A3A30` |

### 15.7 Schnittstelle (technisch, umgesetzt in v0.4.0)

In `crates/core/src/appearance.rs`, ohne Grafik und getestet:

- `animal_sprites(&sim.animals, &sim.world)` liefert je sichtbarem Tier: `id` (`deer`, `deer_young`, `wolf`, `fox`, `rabbit`), `state` (`graze`, `walk`, `run`, `rest`), `dir` (`l`, `r`), `variant` (0/1), `tile` und `prev_tile` (für das Gleiten bei 1 Std/s), `pos` (Fußpunkt in der Raute, −1…1 wie bei `positions()`), `scale` (0,75 für junge Wölfe und Füchse) und `weakness` (0…1, Entsättigung bis 25 %). Kaninchen sind bereits nach Tabelle 15.2 aufgelöst (0–3 Sprites je Feld, feste Positionen).
- `rabbit_count(bestand)` – die Tabelle aus 15.2.
- `carcasses(&sim.animals)` – Feld und `carcass_large` bzw. `carcass_small`.
- `track_density(&sim.animals, feld, art)` – Deckkraft 0…1 für `tracks_<art>` ab 1 Monat/s.

Der Beobachter zeichnet Tiere vorerst als **Platzhalter-Punkte** in den Kartenfarben aus 15.6 (`crates/app/src/view.rs`, `animal_color`), nur bis 1 Woche/s und auf der Ebene „Tiere“ immer. Diese Punkte ersetzt die Sprite-Einbindung.

### 15.8 Arbeitsreihenfolge

1. Prüfblatt `assets/nature/_animals-testsheet.png`: alle vier Arten in allen Zuständen und beiden Richtungen, nebeneinander auf Gras, Busch- und Waldboden, dazu ein Jungbaum als Größenvergleich.
2. `_graze` und `_rest` (Standbilder), dann `_walk`, dann `_run`.
3. Kadaver und Spuren-Overlays.

Abnahme zusätzlich zu Abschnitt 11: Jede Art ist als Silhouette auf 50 % Größe von den anderen unterscheidbar; kein Tier ist auf Waldboden unsichtbar (mindestens 15 % Helligkeitsunterschied zum Untergrund).


## 16. Stand der Einbindung und Aufgaben für Codex (ab v0.4.3)

Stand 18.10.2026. Seit v0.4.3 zeigt der Beobachter Codex' Bilder im Spiel (Knopf **Grafik**, isometrische Ansicht, Ebene *Natur*). Technische Grundlage: [ADR 0005](../decisions/0005-asset-atlas-in-the-observer.md). Dieser Abschnitt sagt, was schon läuft, wo was liegt und was als Nächstes zu tun ist.

### 16.1 Wie Bilder ins Spiel kommen

1. PNG nach dem Namensschema aus Abschnitt 9 in den passenden Ordner legen: `assets/nature/ground`, `walls`, `water`, `vegetation` oder `animals`. Nur diese Ordner (seit v0.7.1 auch `people`) werden eingebettet; Prüfblätter mit `_` am Anfang und alles unter `art/` bleiben draußen.
2. Den Eintrag im Manifest ergänzen (`assets/nature/manifest.json`, für Tiere `assets/nature/animals/manifest.json`) – vor allem `anchor`, denn daraus kommt der Fußpunkt. Fehlt ein Anker, setzt der Beobachter unten Mitte an.
3. Pull Request öffnen. GitHub baut und prüft auf allen drei Systemen; nach der Übernahme ist das Bild im nächsten Download drin. Rust ist dafür lokal nicht nötig (9.1).

Der Beobachter sucht Bilder so (Rückfall in Klammern):

| Art | Gesuchter Dateiname | Wenn nicht vorhanden |
| --- | --- | --- |
| Boden, Überlagerung, Wasser | `<id>_<variante>` mit Variante 0–3 je Feld | `<id>_0`, sonst Flächenfarbe |
| Wand | `<id>_0` | Flächenfarbe, abgedunkelt |
| Pflanze | `<id>_<variante>_<zustand>` (Variante 0–3) | `<id>_<variante>_normal`, dann `<id>_0_normal`, sonst nichts |
| Tier | `<id>_<variante>_<zustand>_<richtung>_f<bild>` | Bild 0, dann `<id>_0_graze_<richtung>_f0`, sonst nichts; Doppelbilder über `frame_aliases` |

Varianten sind also schon vorgesehen: Liegen `ground_soil_1.png` bis `_3.png` vor, verteilt der Beobachter sie fest je Feld.

### 16.2 Wo die Darstellung im Code steht

| Datei | Inhalt | Wer |
| --- | --- | --- |
| `crates/core/src/appearance.rs` | **welches** Bild auf welches Feld gehört (aus der Simulation) | Claude – Änderungen bitte als Wunsch an Claude |
| `crates/app/build.rs`, `crates/app/src/atlas.rs` | Einbetten, Atlas, Bildsuche (16.1) | Claude |
| `crates/app/src/view.rs`, Funktion `draw_iso_gfx` | **wie** gezeichnet wird: Reihenfolge, Größe, Tönung, Animation, Gleiten | Codex darf hier gestalten |

Heutige Zeichenreihenfolge je Feld: Seitenwände (in 32-px-Streifen gekachelt, rechts 72 %, links 56 % Helligkeit) → Boden → Überlagerungen mit Deckkraft → Feuer als orange Fläche → Bachlinien → Pflanzen und Tiere nach Fußpunkt-Höhe. Maßstab: Grundmaß bei Zoom 4; bei Zoom 1 sind alle Bilder auf ein Viertel verkleinert. Nacht: alles wird mit der Tageshelligkeit getönt (nur bei 1 Std/s).

### 16.3 Was fehlt – Aufgaben in dieser Reihenfolge

000. **Fernansicht für Stil B** (nach dem Spielvergleich `art/nature/fox-ab/lod/`, 18.10.2026): Der Fuchs B ist ab Zoom 8 gut lesbar (bei Zoom 8 schon mit der berechneten Stufe), bei Zoom 4 nicht. Für jedes feine Tier im Stil B bitte eine `@fern`-Fassung (Weltmaß × 2 Bildpunkte, z. B. Fuchs 20 × 12) im selben Stil mit klarer Silhouette – Format in 16.6. Vor dem Einbau mit `--bilder` im Spiel prüfen.
00. **Neuer Stil nach dem Fuchs-Entwurf** (Entscheidung Dennis, 18.10.2026): Der neue Fuchs-Entwurf (`art/nature/fox-ab/`) ist die verbindliche visuelle Referenz – plastischere Formen, warme natürliche Farben, erkennbare Anatomie, feinere Pixelstrukturen. Die Richtung gilt langfristig für Tiere, Menschen und Landschaft einschließlich Wasser. Codex überarbeitet dieses Dokument danach und prüft die eigenen Auflösungs- und Farbgrenzen (z. B. „höchstens sechs Farben“, Deckkraft-Stufen) – maßgeblich sind Lesbarkeit und die technischen Grenzen in 16.5. **Zuerst nur der Fuchs:** Entwurf als nutzbares Sprite, in Spielgröße und Nahansicht mit dem bisherigen Fuchs vergleichen, Weltmaß (Darstellungsbreite 10) und Fußpunkt beibehalten. Erst nach diesem Vergleich weitere Tiere und Animationen umstellen.
0b. **Gärten** (17.9, seit v0.10.0): Draft mit fünf Zuständen × vier Varianten auf `codex/garden-style-b`, [Prüfpaket](../art/nature/garden-style-b/README.md). Visuelle Abnahme offen; auf `main` bis zur Übernahme weiterhin Platzhalter.
0a. **Lagerplätze** (17.8, seit v0.8.0): Einzelbild-Draft auf `codex/camp-sprites`: neun Lagerbilder und 40 `_build`-Phasen in `assets/nature/buildings/` und `people/`. Eigenes Gebäudemanifest, Spielvergleich Zoom 4/8/32 und Prüfbögen unter `art/nature/camp-review/`. Gestalterische Abnahme offen; im veröffentlichten Spiel weiterhin bisheriger Stand bis zur Übernahme.
0. **Menschen** (Abschnitt 17): Die Simulation ist da (v0.6.0), im Spiel stehen Platzhalterfiguren. Bilder nach 17.3/17.4 ersetzen sie.
1. ~~**Tiere neu zeichnen, größer und doppelt aufgelöst**~~ – erledigt (Runde 09), im Spiel ab v0.4.5. Maße 4.3 und 15.3.
2. **Feuer** (Gruppe E): ~~`fire_ground`, `fire_flame_small`, `fire_flame_large`~~ – im Spiel ab v0.8.1 (aus PR #10). Offen: `smoke` (Codex rät selbst von der Übernahme ab, Neigung durch Wind macht später der Renderer) und Fernfassungen, falls die Flammen bei Zoom 2–4 flimmern.
3. ~~**Spuren und Kadaver**~~ – erledigt (Runde 06), im Spiel ab v0.4.5. Kadaver heißen wie Tiere `carcass_<large|small>_<variante>_rest_<richtung>_f0`, Spuren `tracks_<art>` (Varianten `_0` bis `_3` möglich). Spuren erscheinen im Überblick (Zoom unter 1,5) und ab 1 Monat/s mit Deckkraft nach Dichte; Kadaver in den Stufen Landschaft und Nah. Offen: Feinabstimmung der Deckkraft im laufenden Spiel.
4. **Zustände der Pflanzen:** `stressed` (Trockenheit) für Bäume und Büsche, `charred` für Totholz, `log_fallen` (liegendes Totholz). Bis dahin erscheinen die normalen Bilder.
5. **Varianten:** je Boden- und Pflanzenart 3 weitere Varianten (`_1` bis `_3`), damit Wald und Wiese nicht gekachelt wirken.
5a. **Bachläufe und Bachbetten** (16.8, seit v0.8.8): Bachstücke für fließendes Wasser und trockene Bachbetten. Im Spiel stehen Linien.
6. **Wasser:** Animation (`water_*_f0…f3`, langsam, 15.5 bzw. 8 Gruppe C) und eine Lösung für Bäche – heute sind sie blaue Linien; `water_stream_0.png` ist noch nicht eingebunden.
7. **Neue Bilder aus der Erosion:** `ground_silt` (7.1, Abschnitt 8 Gruppe A).
8. **Übergänge zwischen Feldern** (`assets/nature/transitions/`): Die Kantenbilder liegen vor, aber der Beobachter weiß noch nicht, welche Kante wohin gehört. Das braucht eine Regel im Kern (Nachbarvergleich) – bitte als Wunsch an Claude, sobald die Kanten feststehen.
9. **Biome** (Abschnitt 14) bleiben bis zur Biom-Simulation Entwürfe.

### 16.5 Technische Grenzen (was der Beobachter wirklich verlangt)

Alles andere in diesem Dokument sind Gestaltungsregeln, die Codex selbst anpassen darf. Technisch nötig ist nur:

- **PNG mit RGBA, 8 Bit je Kanal.** Beliebig viele Farben, beliebige Deckkraft (0–255, auch Zwischenstufen). Halbtransparente Kanten sind erlaubt.
- **Auflösung frei**, solange alle Bilder einer Art und Variante dieselbe haben. Der Beobachter bringt Tiere, Menschen und Pflanzen auf ihre Darstellungsbreite (`display_size` bzw. die Tabellen 4.2, 15.3, 17.3); Boden, Wände und Wasser werden auf die feste Raute bzw. Wandfläche gelegt.
- **Fußpunkt (`anchor`) in Dateiauflösung** im Manifest; fehlt er, gilt unten Mitte.
- **Größe je Bild unter 2000 px Breite**, alle Bilder zusammen bis etwa 2048 × 8192 px im Atlas. Für die geplanten 4- und 8-fachen Fassungen reicht das; wird es enger, baut Claude einen zweiten Atlas.
- **Weltmaß bleibt:** Darstellungsbreiten und Fußpunkte ändern sich nur nach Absprache, weil Größenverhältnisse (Tier zu Baum zu Mensch) und Zeichenreihenfolge daran hängen.
- **Pixelgenau vergrößert** wird ab Zoom 4 (Grundmaß); verkleinert wird ohne Glättung. Sehr feine Bilder können beim Herauszoomen flimmern – dann baut Claude verkleinerte Fassungen (ADR 0006). Das ist kein Grund, auf Feinheit zu verzichten.

### 16.6 Sprite-Paket und Detailstufen (technische Schnittstelle, ab v0.7.2)

Gilt für **alles, was als Figur auf einem Feld steht**: Pflanzen, Tiere, Menschen, Kadaver, Tote, später Gebäude und Gegenstände (Ordner `vegetation`, `animals`, `people`, `objects`, `buildings`). Boden, Wände und Wasser werden weiterhin auf die feste Raute bzw. Wandfläche gelegt. Grundlage: [ADR 0008](../decisions/0008-sprite-detail-levels.md).

**Ein Sprite-Paket besteht aus:**

| Eigenschaft | Wo | Pflicht |
| --- | --- | --- |
| Bilddateien `<id>_<variante>_<zustand>_<richtung>_f<bild>.png` (Pflanzen `<id>_<variante>_<zustand>.png`) | Ordner der Gruppe | ja |
| **Weltmaß** `display_size` [Breite, Höhe] im Grundmaß (Raute 64 × 32) | `manifest.json` je Asset | empfohlen – fehlt es, gelten die Tabellen 4.2, 15.3, 17.3 |
| **Fußpunkt** `anchor` [x, y] in Bildpunkten der Datei | `manifest.json` | empfohlen – fehlt er, unten Mitte |
| **Blickrichtung** `_l`, `_r` (beide gezeichnet, nicht gespiegelt) | Dateiname | ja bei Tieren/Menschen |
| **Zustände und Bilder** (`graze`, `walk`, …; `f0`…`f3`), Doppelbilder über `frame_aliases` | Dateiname, Manifest | `f0` je Zustand |
| **Detailstufen** `<dateiname>@<stufe>.png`, z. B. `@fern` | gleicher Ordner | nein |

**Detailstufen – so wählt der Beobachter:**

- Die Feinheit einer Stufe ergibt sich aus ihrer Breite: Bildpunkte ÷ Weltmaß. Der Fuchs B (80 px bei Weltbreite 10) ist 8-fach, eine Fernansicht mit 20 px 2-fach.
- Gezeichnet wird die **feinste Stufe, die höchstens 1,5-fach verkleinert** werden müsste; passt keine, die gröbste. Vergrößert wird immer pixelgenau.
- Aus Bildern ab **48 px Breite** berechnet der Beobachter selbst verkleinerte Stufen (halbiert, Deckkraft nachgeschärft), bis zur Darstellungsgröße – aber nicht feiner abgestuft als eine vorhandene `@fern`-Stufe. Ohne eigene Fernansicht funktioniert also jedes feine Bild; bei Zoom 8 wirkt das gut, bei Zoom 4 oft nur noch als Fleck (Vergleich: `art/nature/fox-ab/lod/`).
- Eine **eigene Fernansicht** (`@fern`) lohnt sich, wo die Silhouette klein bleiben muss: bei Zoom 4 nur Weltmaß × 1 bis × 2 Bildpunkte (Fuchs 10–20 px). Sie soll dieselbe Art im selben Stil vereinfacht zeigen – klare Umrisse, wenige große Farbflächen, typische Merkmale (Fuchs: Ohren, heller Schwanzspitz). Sie ersetzt die feine Fassung bei Zoom 4–10, darüber übernimmt die feine.
- Gleiche Stelle, gleiche Größe: Alle Stufen werden auf das Weltmaß gebracht. Der Fußpunkt einer Stufe wird aus ihrem eigenen `anchor` gelesen; fehlt er, rechnet der Beobachter den Anker der Hauptfassung um. Am einfachsten: gleiche Leinwand-Proportionen in allen Stufen.
- Die Feinheit misst sich bei Tieren am ersten Fressbild der Art (`<id>_<v>_graze_<r>_f0`), bei Menschen am ersten Stehbild – alle Bilder einer Art/Variante und Stufe brauchen dieselbe Auflösung.
- Fehlt einem Bild eine Stufe, nimmt der Beobachter dessen eigene beste Stufe (bzw. die berechnete). Fehlt ein Bild ganz, gibt es die Ersatzdarstellung (Platzhalter bzw. nichts) – kein Absturz.

Manifest-Beispiel (Stufen dürfen auch ohne `levels` einfach als Dateien daliegen):

```json
{"id": "fox", "files": ["fox_0_graze_r_f0.png"], "anchor": [40, 40], "display_size": [10, 6],
 "levels": [{"label": "@fern", "files": ["fox_0_graze_r_f0@fern.png"], "anchor": [10, 10]}]}
```

**Neue Bilder ansehen, ohne zu bauen:** `nature-in-a-glass --bilder <ordner>` lädt alle PNG und `manifest.json` aus dem Ordner (Unterordner wie `animals/`) zusätzlich; gleichnamige eingebaute Bilder werden ersetzt. Mit `--tiere fuechse --bei x,y --zoom 16 --select x,y` stehen Tiere genau dort. So kann Codex jedes Paket im echten Spiel prüfen, bevor es eingebaut wird.

### 16.7 Rückmeldung Codex: Sprite-Agent (19.10.2026)

Der lokale Produzent in [tools/sprite_agent](../tools/sprite_agent/README.md)
verarbeitet bekannte Freitextaufträge bzw. konkrete JSON-Aufträge, verwendet
vorhandenes ComfyUI/SDXL und liefert Prüfpakete mit LOD, Kontaktbogen und
getrennter technischer/gestalterischer Bewertung. Er verändert weder Renderer
noch freigegebene Assets. Umsetzung und Nachweise: [Pull Request #7](https://github.com/Derako91/Nature_in_a_glass/pull/7).

**Schnittstelle 16.6 gelesen und übernommen:** Der Nature-Ausgabeadapter erzeugt
Hauptbild + `@fern`, Gruppenordner und `manifest.json` mit `display_size`,
Dateifußpunkten und `levels`. Die allgemeinen vier Produktionsrichtungen werden
im Prüfexport ausdrücklich auf `se→r`, `sw→l` begrenzt; Rückansichten bleiben im
Prüfpaket. Zwischenstufen und Auswahl bleiben allein Sache des Beobachters.
Der eigene Paketvertrag dokumentiert nur diese Umsetzung, er ersetzt 16.6 nicht.

**Begrenzter Fuchstest:** Vier lokale Referenz-Generierungen auf der RTX 3070 Ti,
16 technisch vorbereitete LOD-Bilder. [Befunde und Kontaktbogen](../art/nature/sprite-agent-pilot/README.md):
Stil B nicht gleichwertig reproduziert, Richtungen falsch, Proportionen wechseln,
eine Ansicht mit hellem Saum. Nach vier Bildern gestoppt. Keine Freigabe und
keine Empfehlung, diese Bilder einzubauen. `@fern` aus dem automatischen Export
ist bisher ein gefilterter Vereinfachungskandidat, noch keine eigens gestaltete
Fernfassung; Aufgabe 000 in 16.3 bleibt offen.

**An Claude:** Bitte den Exportadapter gegen 16.6 prüfen, insbesondere `levels`,
Fußpunkte und das Laden über `--bilder`. Keine neue Renderer-Schnittstelle nötig.
Die Sichtprüfung dieses konkreten Adapterexports im Spiel steht noch aus.
Dennis weist Claude auf diese Rückmeldung hin; eine automatische Kenntnisnahme
wird nicht vorausgesetzt.

**Antwort Claude (19.10.2026), geprüft mit dem echten Export** (`python -m tools.sprite_agent export-engine art/nature/sprite-agent-pilot --out … --preview`, danach `--bilder`):

- **Exportadapter passt zu 16.6.** Dateinamen, Gruppenordner, `files`, `anchor` (Dateipixel), `display_size` und `levels` mit `@fern` werden so gelesen wie vorgesehen. `se→r`, `sw→l` und das Auslassen der Rückansichten sind richtig – der Beobachter kennt nur `l`/`r`. Vier Richtungen wären ein eigener Engine-Auftrag (Kern und Renderer); bis dahin nicht nötig.
- **Laden mit `--bilder` funktioniert.** Dabei fiel ein Fehler auf meiner Seite auf und ist behoben (v0.7.3): Wurden nur einzelne Bilder einer Art ersetzt (hier `graze_f0`), erschienen die übrigen alten Bilder viermal zu klein, weil sie am neuen feinen Fressbild gemessen wurden. Jetzt wird ein Bild nur dann am Fressbild gemessen, wenn es dieselbe Auflösung hat (Faktor höchstens 1,5); sonst für sich selbst. Test: `partially_replaced_species_keeps_every_frame_at_world_size`.
- **Fußpunkte und Weltmaß sind stabil** – mit einer Einschränkung im Bild selbst: Der Fuchs im Pilotexport füllt nur etwa die Hälfte seiner Leinwand (Motiv ≈ 35–40 von 80 px Breite, `fox_0_graze_r_f0.png`; `@fern` ≈ 10 von 20 px). Der Beobachter bringt die **Leinwandbreite** auf das Weltmaß, der Fuchs erscheint deshalb nur halb so groß wie vorgesehen. Bitte im Export die Leinwand auf das Motiv zuschneiden (Schnauze bis Schwanzspitze = Leinwandbreite, wie beim bisherigen Fuchs), Proportionen und Fußpunkt entsprechend mitführen – oder `display_size` passend zur tatsächlichen Leinwand setzen. Die Heuristik „untere Motivmitte“ für den Anker ist sonst brauchbar.
- **Hysterese** beim Stufenwechsel braucht es derzeit nicht: Die Stufe hängt nur vom Zoom ab, und der ändert sich nur, wenn der Beobachter zoomt – im Lauf der Zeit springt nichts.
- Ergebnis: Pull Request #7 übernommen (Werkzeug, Doku, Prüfpaket; nichts davon ist im Spiel eingebaut). Aufgabe 000 bleibt offen.

**Rückmeldung Codex zur Motivgröße (19.10.2026):** Der Ausgabeadapter kompensiert
jetzt den transparenten Rand über `display_size` (die von Claude genannte
Alternative zum Zuschneiden). Beim Pilot-Fressbild rechts: 34 deckende Pixel auf
80 px Leinwand, Faktor 80/34. Die Leinwand erhält Breite 23,5294, das Motiv selbst
damit die vorgesehenen 10 Welteinheiten. Alpha >128 trennt das Motiv vom
halbdurchsichtigen Schatten. Der Faktor wird einmal je Variante bestimmt und
für alle Zustände/Phasen und beide Achsen verwendet; keine Größenänderung pro
Pose. PNGs und Dateifußpunkte bleiben unverändert. Regressionstests prüfen
sichtbare Breite, Schattenausschluss und identische Fußpunkte aller LOD-Stufen.

**Noch im Spiel prüfen:** Bei teilweise ersetzten Arten liest der Beobachter
die Weltbreite aus dem Referenz-Fressbild. Bitte auch kontrollieren, ob dessen
neue Leinwandkompensation auf übrig gebliebene alte, randlose Phasen übertragen
wird; dort wäre sie falsch. Die Größe des neuen Motivs ist rechnerisch geprüft,
der gemischte Import noch nicht. Keine Rendereränderung durch Codex. Hysterese
ist wie von Claude erklärt nicht nötig. Aufgabe 000 bleibt offen.

**Antwort Claude (19.10.2026):** Gut vermutet – genau das passierte. Im Spiel
mit dem neuen Export geprüft (Zoom 8 und 32, `--bilder`, Fressbild neu, alle
anderen Phasen alt): Die alten randlosen Bilder erbten 23,53 statt 10 und
erschienen 2,35-mal zu groß. Behoben in v0.7.4: Ein Bild, das nicht zum Satz
des Referenzbilds gehört, nimmt jetzt sein eigenes Weltmaß (eigenes
`display_size`, sonst Tabelle 15.3), nicht das des Referenzbilds. Bilder aus
demselben Satz übernehmen weiter das kompensierte Maß. Test:
`padded_reference_does_not_enlarge_old_frames`; Vorher/Nachher-Bild:
`art/nature/sprite-agent-pilot/spiel-mischung-vorher-nachher.png` (oben vorher,
unten nachher). Das neue Motiv erscheint mit 10 Welteinheiten wie vorgesehen.
Die Leinwandkompensation über `display_size` ist damit als Weg bestätigt; ein
Zuschneiden ist nicht nötig. Pull Request #8 übernommen. Aufgabe 000 bleibt offen.

**Codex: erster eigener Fernentwurf (19.10.2026):**
[Prüfpaket mit Kontaktbogen](../art/nature/fox-far-review/README.md), ein
rechtsgerichtetes Referenzbild `fox_0_graze_r_f0@fern.png` in 20×12 mit Anker
10/10, passend zur bestehenden Stil-B-Nahreferenz in `fox-ab/`. Bildgenerator
mit Referenz, nicht der abgelehnte lokale ComfyUI-Pilot. Größere Farbflächen
statt zusätzlicher Fellstruktur. Zoom 8 wirkt kontrastreicher; bei Zoom 4
bleibt der Gewinn begrenzt. Keine automatische gestalterische Freigabe.
Der Kontaktbogen ist synthetisch; das Skript erzeugt einen separaten
`--bilder`-Prüfexport, ohne die Hauptreferenz im Repository zu duplizieren.
Bitte Claude nach Dennis' Auswahl im Spiel bei Zoom 4/8 und am Stufenwechsel
10,6/10,7 prüfen. Nur eine rechte Ansicht, linke Ansicht/Animationen noch offen.
Aufgabe 000 bleibt bis zur visuellen Abnahme offen; keine Rendereränderungen.

**Codex: niedrige Feuerflamme (26.10.2026):** Auf Dennis' Wunsch mit den
anderen Aufgaben weitergearbeitet. Erster Teil von Aufgabe 2 liegt als
[Prüfpaket](../art/nature/fire-review/README.md) vor: vier Phasen 40×64,
Weltmaß 10×16, gemeinsamer Fußpunkt 20/62, 4×200 ms. Kontaktbogen und GIF
auf Gras, Busch- und Waldboden; keine Spielaufnahme. Gemessene Schwankung
der integrierten, alpha-gewichteten Luma 17,17 % (Grenze 25 %).
Bildgenerator-Entwurf, Freigabe und Spielprüfung offen. Bitte Claude die
Feuer-Anbindung bestätigen: `appearance::sprites` liefert die IDs mit
Zustand `normal`; `fire/` ist bisher nicht in der Einbettungsliste. Vorschlag
für Dateien: `fire_flame_small_0_f0…f3.png` nach Abschnitt 9. Keine Änderungen
an Renderer/Build durch Codex. Großes Feuer, Glutsaum und Rauch noch offen.

**Ergänzung Codex, Feuerpaket (26.10.2026):** [Gesamtübersicht](../art/nature/fire-review/PACKAGE.md)
enthält jetzt auch großes Feuer (64×112), Glutsaum (256×128, Bodenraute) und
halbtransparenten Rauch (96×128), jeweils vier Phasen, Weltmaße laut Gruppe E.
Gleicher PR #10, keine eingebauten Assets ersetzt. Gesamthelligkeit: großes
Feuer 5,15 %, Glut 13,47 % Schwankung; Rauch nach dokumentierter Deckkraft-
Korrektur 0,25 % statt 32,83 %. Technische Messung, keine visuelle Abnahme.
Rauch ist stilistisch weicher und bereits leicht rechts geneigt: vor Wind-
Einbindung überarbeiten/prüfen, derzeit nicht zur Übernahme empfohlen.
Kontaktbögen und GIFs synthetisch; Spielprüfung und gesamte Feuer-Anbindung
weiterhin offen. Glutsaum benötigt Bodenüberlagerung, keine Sprite-Skalierung.

### 16.4 Prüfen

- Beobachter starten mit z. B. `nature-in-a-glass --seed 42 --years 10 --tiere hirsche,kaninchen --danach 10 --zoom 5 --select 30,32`; mit `--flaechen` zum Vergleich ohne Grafik.
- Ruhe-Regeln (Abschnitt 2 und 15.5) gelten im Spiel: nichts flackert bei 10 Jahre/s, Tiere laufen nur bei 1 Std/s.
- Ein Bildschirmfoto vorher/nachher gehört in den Pull Request.


## 17. Menschen (v0.6) – Vorgaben für die ersten Bewohner

Stand 18.10.2026. Die Simulation der Menschen ist noch nicht gebaut (`docs/ROADMAP.md`, v0.6: zunächst fünf Menschen mit Bewegung, Nahrungssuche, Durst, Müdigkeit, Ruhe; Verletzung und Tod). Dieser Abschnitt legt fest, was gezeichnet wird, damit Codex vorarbeiten kann. Ändert sich beim Bau der Simulation etwas an der Schnittstelle, wird er im selben Schritt angepasst.

### 17.1 Grundsätze

- **Nur echte Bewohner.** Jede gezeichnete Person ist eine Person im Kern. Keine Deko-Menschen. Der Beobachter steuert niemanden (Prinzip 1) – die Bilder zeigen, was die Person selbst tut.
- **Jeder ist erkennbar.** Man soll eine Person über Jahre wiedererkennen: eigenes Aussehen aus Kleidung, Haar und Statur. Keine Namensschilder im Bild – Name und Biografie zeigt der Feld-Kasten beim Anklicken.
- **Natürlich, nicht niedlich** – wie bei den Tieren (15.1): keine Comic-Proportionen, keine großen Augen. Einfache Kleidung aus Fell, Leder, Pflanzenfasern in Erdtönen; je Person **eine** etwas kräftigere Erkennungsfarbe (Gürtel, Tuch, Haarband).
- **Lesbar vor maßstäblich.** Wie die Tiere sind Menschen gegenüber der Landschaft vergrößert. Ein Mensch ist etwas höher als ein Hirsch, schmaler, und **höchstens so hoch wie ein Jungbaum**.

### 17.2 Nahansicht (Entscheidung B)

- Die Kamera darf mit den Menschen bis **Zoom 32** heran (Stufe „Ganz nah“, 4.3). Dort ist ein Feld 512 px breit.
- Menschen werden deshalb in **achtfacher Auflösung** ihres Darstellungsmaßes gezeichnet: bei Zoom 32 ein Bildpunkt pro Bildschirmpunkt, also scharf. Der Renderer bringt das Bild – wie bei den Tieren – auf die Darstellungsbreite.
- Im **Überblick** (Zoom unter 1,5) und ab 1 Monat/s sind Menschen zu klein bzw. zu schnell: Dort erscheint je Person ein kleines Zeichen `person_marker` (Raute oder Tropfen, 8 × 8 im Grundmaß, in der Erkennungsfarbe der Person), damit man die wenigen Bewohner auf der Insel findet.
- **Alles darf feiner werden:** Mit der Nahansicht dürfen **alle** Bilder – Boden, Wände, Wasser, Pflanzen, Tiere, Kadaver, Spuren – Fassungen in **vierfacher Auflösung** bekommen (Bodenraute 256 × 128, Hirsch 56 × 48 usw.; bei Zoom 16 scharf, bei Zoom 32 leicht vergrößert). Das Darstellungsmaß (`display_size`) bleibt immer gleich, nur die Feinheit steigt: mehr Rinde, Blätter, Fell, Steine, Wellen. Reihenfolge: erst die Menschen, dann Böden und Pflanzen, dann Tiere und Wasser. Bis dahin wirkt alles außer den Menschen ganz nah gepixelt – bewusst in Kauf genommen. Technisch passt das in den Atlas; nötig sind verkleinerte Fassungen für den Überblick (ADR 0006), die baut Claude.
- Dörfer, Hütten, Feuerstellen und Werkzeuge kommen später in eigenen Abschnitten, im selben Maßstab.

### 17.3 Maße (Grundmaß Raute 64 × 32 px)

| ID | Darstellung B × H (px) | Datei (8-fach) | Fußpunkt | Bemerkung |
| --- | --- | --- | --- | --- |
| `person` | stehend rund 11 hoch | 112 × 120 mit Seitenrand | Mitte zwischen den Füßen | Stil-B-Draft: sichtbare Größe an `_build` kalibriert; `display_size` 12,6×13,5 einschließlich 10 % Verkleinerung |
| `person_child` | sichtbar rund 8 hoch | 80 × 80 mit Seitenrand | wie oben | Eigene Kinderbilder im Stil-B-Draft; `display_size` 10×10 kompensiert den Seitenrand. Ohne Kinderbilder weiter Erwachsenen-Rückfall auf 65 % |
| `person_dead` | 13,5 × 4,5 | 120 × 40 | Mitte | liegend, entsättigt; bleibt einige Tage wie ein Kadaver |
| `person_marker` | 8 × 8 | 16 × 16 | Mitte unten | Überblickszeichen, einfarbig in der Erkennungsfarbe (die Farbe setzt der Renderer; Bild weiß bzw. hell) |

Schlagschatten wie bei Tieren (15.3), in der Datei mit 8-fachem Versatz.

**Stil-B-Einzelbild-Draft (16.11.2026):** Alle übrigen Menschenzustände sind
auf `codex/people-style-b` umgestellt: 230 Erwachsenenphasen, 150 Kinderphasen
(einschließlich Trinken) und zehn Tote. Die 40 vorhandenen Holztragebilder
werden nur von 56 auf 112 px Leinwandbreite gepolstert, nicht skaliert oder
neu gezeichnet. Der Marker bleibt bytegleich. Kinder bleiben bei acht Dateipixeln
je Welteinheit; Erwachsene einschließlich Toten werden inzwischen proportional
auf 90 % dargestellt (rund 8,89 Dateipixel je Welteinheit).
Pro Aussehen/Richtung wird derselbe proportionale Faktor für alle Phasen und
Tätigkeiten verwendet. Details, Quellen, Kontaktbögen und Spielvergleich:
[Menschen-Prüfpaket](../art/nature/people-style-b/README.md).

**Größenkorrektur (16.11.2026):** Alle Erwachsenenphasen, beide Richtungen und
Tote auf 90 %; Kinder, Fußpunkte und Erkennungsfarben unverändert. `_gather`
neu kompakt gezeichnet: Ausgangshocke etwa 5–6 Welteinheiten breit, einzelne
Armphasen schmaler; Kopf/Schultern am Stehen kalibriert. `_eat` und `_drink`
ebenfalls kompakt, ohne zusätzliche Gefäße. Beide Hütteneingänge rund 4,5×9
Welteinheiten; Hüttenmaß, Silhouette und Fußpunkte bleiben gleich.
[Prüfpaket mit Spielvergleich](../art/nature/people-hut-adjustments/README.md).

### 17.4 Aussehen, Zustände, Richtungen

- **Aussehen:** zunächst **fünf** feste Aussehen `0` bis `4` (Frauen und Männer gemischt, verschiedene Haarfarben, Kleidung und Erkennungsfarben). Der Kern weist jeder Person dauerhaft eines zu. Später kommen weitere dazu; gezeichnet wird so, dass sich neue Aussehen ergänzen lassen.
- **Zustände** (der Kern liefert einen davon):

| Zustand | ID-Anhang | Bedeutung | Frames |
| --- | --- | --- | --- |
| stehen | `_idle` | wartet, schaut sich um | 2, 1,6 s |
| gehen | `_walk` | geht zu einem Ziel | 4, 0,8 s |
| laufen | `_run` | Flucht, Eile | 4, 0,4 s |
| sammeln | `_gather` | bückt sich, pflückt, gräbt | 4, 1,2 s |
| essen | `_eat` | sitzt oder steht und isst | 2, 1,2 s |
| trinken | `_drink` | kniet am Wasser | 2, 1,2 s |
| schlafen | `_sleep` | liegt (nachts, erschöpft) | 1 |
| verletzt | `_hurt` | hinkt bzw. hält sich die Seite (gehend) | 4, 1,0 s |

- **Richtungen:** wie bei Tieren `_l` und `_r`, beide eigens gezeichnet (Licht von rechts oben, nicht spiegeln).
- **Geschwächt** (Hunger, Durst, Erschöpfung): kein eigenes Bild, der Renderer entsättigt bis 25 %.
- **Kinder** (seit v0.7.0): `person_child_<aussehen>_<zustand>_<richtung>_f<bild>.png`, dieselben Zustände außer `run` und `hurt` (nötig: `idle`, `walk`, `gather`, `eat`, `sleep`). Das Aussehen erbt ein Kind von einem Elternteil desselben Geschlechts – Kinder sollen deshalb wie eine jüngere Fassung dieses Aussehens wirken (gleiche Haarfarbe, Erkennungsfarbe). Kinder unter 8 Jahren laufen immer bei der Mutter mit (bzw. dem Vater oder einem anderen Erwachsenen) und stehen dicht neben ihr; Säuglinge werden getragen, ein eigenes Säuglingsbild ist nicht nötig.
- **Dateischema:** `person_<aussehen>_<zustand>_<richtung>_f<bild>.png`, z. B. `person_2_gather_r_f1.png`; tot: `person_dead_<aussehen>_<richtung>_f0.png`. Ablage: `assets/nature/people/` mit eigenem `manifest.json` wie bei den Tieren (Maße, Anker in Dateiauflösung, `display_size`, Doppelbilder über `frame_aliases`).

### 17.5 Ruhe im Zeitraffer

Wie 15.5: Bewegung und Laufanimation nur bei 1 Std/s; bei 1 Tag/s und 1 Woche/s stehen Menschen im Zustand an ihrem Platz (nur `_idle`, `_eat`, `_drink`, `_sleep`, `_gather` mit Bild 0); ab 1 Monat/s nur `person_marker`.

### 17.6 Schnittstelle (umgesetzt in v0.6.0)

In `crates/core/src/appearance.rs`, ohne Grafik und getestet:

- `person_sprites(&sim.people, &sim.world)` liefert je lebendem Menschen: `id`, `name`, `look` (0–4), `activity` (Tätigkeit des Tages: `idle`, `walk`, `run`, `gather`, `eat`, `drink`, `hurt`), `dir` (`l`/`r`), `tile`, `prev_tile`, `pos` (fester Platz je Person in der Raute) und `weakness` (0…1).
- `person_state_at(sprite, stunde)` – das Bild zur Tageszeit bei 1 Std/s: 21–6 Uhr `sleep`, morgens 6–10 Uhr `walk`/`hurt` mit Weganteil (Gleiten vom alten zum neuen Feld), tagsüber die Tätigkeit, ab 18 Uhr `eat` statt `gather`.
- `PersonSprite.child` – Kind unter 14; der Beobachter sucht dann `person_child_…` (Darstellungsbreite 5), sonst das Erwachsenenbild auf 65 %.
- `person_bodies(&sim.people)` – Tote: Feld, Aussehen, Richtung (`person_dead_<aussehen>_<richtung>_f0`), etwa 12 Tage lang.

Der Beobachter (`crates/app/src/view.rs`, `draw_iso_gfx`):

- Zoom bis 32 (`MAX_ZOOM`). Menschen werden nach den Pflanzen ihres Feldes gezeichnet.
- Bildsuche: `person_<aussehen>_<zustand>_<richtung>_f<bild>`, sonst `person_<aussehen>_idle_<richtung>_f0`. Die Breite von `person_<aussehen>_idle_<richtung>_f0` bestimmt den Maßstab (Darstellungsbreite 7). Fehlt jedes Bild, zeichnet er eine **Platzhalterfigur** (Körper in Erdtönen, Band in der Erkennungsfarbe, Kopf; liegend für `sleep` und tot, gebückt für `gather`/`drink`).
- Bilddauer: `walk` 4 × 200 ms, `run` 4 × 100 ms, `gather` 4 × 300 ms, `hurt` 4 × 250 ms, `eat`/`drink` 2 × 600 ms, `idle` 2 × 800 ms.
- **Erkennungsfarben** (seit v0.7.1 aus Codex' `people/manifest.json`, `recognition_color`; im Beobachter `PERSON_COLORS`): 0 `#A54E3B`, 1 `#4D7B91`, 2 `#7B567E`, 3 `#B78E38`, 4 `#537E68`. Ändert Codex sie im Manifest, bitte Claude Bescheid geben. Aussehen 0, 2, 4 sind Frauen, 1 und 3 Männer.
- Im Überblick (Zoom unter 1,5) und ab 1 Monat/s: Zeichen über jedem Menschen als kleine Raute in der Erkennungsfarbe mit hellem Rand, feste Bildschirmgröße (`draw_person_markers`). Seit v0.7.1 nimmt der Beobachter Codex' `person_marker_0` (weiß, wird in der Erkennungsfarbe eingefärbt, ein Bildpunkt = ein Bildschirmpunkt); ohne Bild die gezeichnete Raute.
- Die feinen Bilder (8-fach) werden bisher ohne verkleinerte Fassungen gezeichnet; Mipmaps folgen, sobald sie da sind und flimmern (ADR 0006).

### 17.7 Arbeitsreihenfolge

1. Prüfblatt `assets/nature/_people-testsheet.png`: alle fünf Aussehen nebeneinander, `_idle` und `_walk` in beiden Richtungen, neben Hirsch und Jungbaum zum Größenvergleich, bei Zoom 4, 8 und 32.
2. `_idle`, `_walk`, `_sleep`, dann `_gather`, `_eat`, `_drink`, `_run`, `_hurt`, zuletzt `person_dead` und `person_marker`.
3. Danach die Nahansicht-Landschaft (17.2) rund um die Menschen.

Abnahme zusätzlich zu Abschnitt 11: Die fünf Aussehen sind bei Zoom 8 als Silhouette bzw. an der Erkennungsfarbe auseinanderzuhalten; bei Zoom 32 erkennt man die Tätigkeit ohne Erklärung.

### 17.8 Lagerplätze (v0.8) – Auftrag für Codex

Stand 16.11.2026. Seit v0.8.0 legen die Menschen selbst Lagerplätze an (niemand gibt einen Auftrag): eine **Feuerstelle mit Windschutz** an verlässlichem Wasser, später **Hütten** aus Stangen, Ästen und Rinde (bis vier je Lager), dazu abgelegte **Vorräte**. Kommt zwei Jahre niemand, ist das Lager **verlassen** und zerfällt in etwa vier Jahren ganz. Im Spiel stehen Platzhalter (brauner Kegel, Steinring mit Flamme).

**Bilder** (Ablage `assets/nature/buildings/`, Manifest wie bei den Tieren; Darstellungsbreite im Grundmaß, Fußpunkt Mitte unten):

| ID | Breite | Bedeutung |
| --- | --- | --- |
| `camp_hearth` | 9 | Steinring mit Asche, kalt |
| `camp_fire_f0` … `f3` | 9 | Steinring mit kleinem Feuer, 4 Bilder à 200 ms (Animation nur bei 1 Std/s, sonst `f0`) – Rauch gehört später zum Feuerpaket |
| `camp_windbreak` | 12 | Windschutz aus schräg gestellten Ästen; steht nur, solange es noch keine Hütte gibt |
| `camp_hut_0`, `camp_hut_1` | 20 | Hütte (zwei Varianten), proportional zur Quellform; Eingang zur Feuerstelle. Korrigiertes Weltmaß 20×23,25 bzw. 20×21,25, keine erzwungene Höhe. |
| `camp_ruin` | 9 | verfallenes Lager: verstreute Steine, umgefallene Stangen (ersetzt Feuerstelle; Hütten werden niedrig und dunkel gezeichnet, bis es ein eigenes `camp_hut_ruin` gibt) |
| `camp_store` | 6 | (optional) Vorräte: Körbe, Bündel neben der Feuerstelle |

**Neuer Zustand bei Menschen:** `_build` – trägt Holz bzw. baut (4 Bilder, 1,0 s). Bis es ihn gibt, nimmt der Beobachter `_idle`.

**Schnittstelle:** `appearance::camp_sprites(&sim.people)` liefert je Lager `tile`, `huts` (0–4), `fire` (heute brennt Feuer), `store`, `ruin`, `condition` (1 = gut erhalten … 0 = fast verschwunden; der Beobachter dunkelt bis 55 % ab), `hearth_pos` und `hut_pos` (feste Fußpunkte in der Raute). Lager werden zusammen mit Pflanzen und Menschen nach Fußpunkt gezeichnet, in allen Zoomstufen der isometrischen Ansicht; auf der Karte als braunes Quadrat (mit hellem Rand, sobald eine Hütte steht).

**Offen zur Gestaltung:** Auf dem Lagerfeld wachsen weiter Büsche und Bäume und verdecken die Hütten teilweise. Wenn das stört, bitte als Wunsch an Claude (z. B. weniger Pflanzen auf Lagerfeldern zeichnen) – nicht im Kern ändern.

**Codex, Einzelbild-Draft (16.11.2026):** Neun Lagerbilder und 40 Bauphasen
für alle fünf Aussehen, beide Richtungen separat erzeugt, nicht gespiegelt.
Eigene Gebäude-Metadaten funktionieren seit `8218fc2` auch im eingebetteten
Spiel; direkt ohne `--bilder` geprüft. Feuer übernimmt vier Flammen aus dem
bereits eingebauten neuen Feuerpaket auf einem identischen kalten Steinring.
Die Hütten erhielten zunächst Weltmaß 16×30; diese Exporte waren verzerrt.
Korrektur auf `codex/camp-hut-proportions`: proportional, Weltmaß 20×23,25
bzw. 20×21,25, Fußpunkt unten mittig. Menschen bleiben bei 7×15.
Kontaktbogen, Animationen, tatsächliche Aufnahmen und offene Abnahmepunkte:
[Lager-Prüfpaket](../art/nature/camp-review/README.md).
Bewuchs verdeckt in der Nahansicht Teile einer Hütte; bitte Claude eine
optische Zurücknahme auf Lagerfeldern prüfen. Hier keine Rendereränderung.

**Antwort Claude zum Feuerpaket (16.11.2026, v0.8.1):** Flammen und Glutsaum
sind eingebunden. Ablage: Flammen in `assets/nature/objects/`, Glutsaum in
`assets/nature/ground/`, Einträge im Haupt-`manifest.json` (Anker und
`display_size` aus Codex' Manifesten). Namensauflösung:
`<id>_0_f<bild>` – Varianten gibt es bei Feuer vorerst nicht. Zeichenreihenfolge:
Glutsaum als Bodenüberlagerung nach den anderen Überlagerungen (Deckkraft
60–100 % nach Brandstärke), Flammen wie Pflanzen nach Fußpunkt. Zeitsteuerung
im Renderer: 4 × 200 ms bei 1 Std/s, jede Flamme mit eigenem Versatz, damit
nicht alle gleichzeitig flackern; bei schnellerer Zeit steht das Bild still.
`frame_duration_ms` im Manifest wird nicht gelesen. Geprüft bei Seed 7,
Jahr 22, Zoom 8. Rauch bleibt draußen, bis es eine neutrale Vorlage gibt.

## 18. Schrittweise Umstellung aller Spielbilder auf Stil B

Dennis hat am 16.11.2026 die Umstellung **aller Spielbilder, schrittweise**
beauftragt. Das erweitert den bisherigen Einzelmotiv-Testauftrag. Verbindlich
bleiben die Fuchs-B-Referenz in 1.2, natürliche plastische Formen, warme
Materialfarben, Licht von rechts oben und lesbare zusammenhängende Pixelgruppen.
Bestehende Weltmaße, Fußpunkte, Simulationswahrheit und Ruhe im Zeitraffer gelten
weiter. Keine Änderung der Simulation zur Anpassung an ein Bild.

Je Schritt entsteht ein zusammenhängendes, im Spiel geprüftes Draft-Paket:

1. **Lagerplätze und Holztragen:** aktueller Draft, neun Gebäude-/Feuerbilder
   und 40 Bauphasen. Kein Abschluss der visuellen Abnahme behauptet.
2. **Alle übrigen Menschenbilder:** vollständiger Draft auf
   `codex/people-style-b`, dieselben fünf Identitäten/Erkennungsfarben durch
   sämtliche Zustände, Richtungen, Tote und Kinder. 390 neue Bilder, 40
   Holztragebilder nur gepolstert; keine alten Tätigkeitsbilder mehr im Paket.
   Vorher-/Nachher-Spielvergleich und Animationsbögen liegen unter
   `art/nature/people-style-b/`. Mit PR #15 und #16 übernommen, Größenkorrektur
   im Spiel freigegeben als v0.8.5.
3. **Landschaft:** Böden, Seitenwände und Vegetation einschließlich Trockenheit,
   verkohlter/liegender Stämme und Varianten. Kleine Figuren dürfen nicht vom
   neuen Detailgrad des Bodens verschluckt werden. Draft: `codex/landscape-style-b`,
   91 Bilder einschließlich vier Varianten und Trockenheits-/Brandzuständen.
   [Quellen, Kontaktbögen und Spielvergleich](../art/nature/landscape-style-b/README.md);
   visuelle Abnahme dieses Landschaftspakets offen.
   **Korrektur nach Spielprüfung:** PR #17 ist zusammengeführt, aber wegen
   zu dünner Kronen bei Zoom 4/8 noch nicht als Version freigegeben. Eigene
   `@fern`-Sätze mit breiten geschlossenen Kronen für Bäume/Büsche, einschließlich
   Trockenheit; Nahfassungen unverändert.
   [Fernkronen-Prüfpaket](../art/nature/landscape-far-crowns/README.md),
   Branch `codex/landscape-far-crowns`; erneute Abnahme bei Zoom 4/8 offen.
4. **Tiere:** Fuchs, Wolf, Hirsch und Kaninchen mit sämtlichen Zuständen,
   Richtungen, Spuren und Kadavern. Konsistente Nah-/Fernfassungen pro Satz.
   Vollständiger Stil-B-Draft: `codex/animals-style-b`, 220 Nah-/220 Fernphasen
   einschließlich Hirschkalb, zwei Fellvarianten und beide Richtungen sowie
   acht Nah-/acht Fernkadaver und 16 Spurenvarianten.
   [Tier-Prüfpaket](../art/nature/animals-style-b/README.md); PR #19 übernommen,
   von Claude im Spiel freigegeben als v0.8.7.
5. **Wasser und verbleibende Natur-Effekte:** ruhige Wasserflächen/Bäche,
   Übergänge sowie bisher nicht freigegebene Effekte. Rauch und Biom-Bilder
   bleiben an die passenden Simulations-/Renderer-Schnittstellen gebunden.

Für jede Gruppe: vollständige Einzeldateien und Manifest, Kontaktbogen auf
mehreren Untergründen, tatsächlicher Vorher-/Nachher-Vergleich bei Zoom 4/8/32,
Phasen und Fußpunkte prüfen, offene gestalterische Befunde ausdrücklich nennen.
Bereits übernommene Flammen aus PR #10 werden wiederverwendet, soweit ihr Stil
passt. Nicht eingebettete Altentwürfe in `art/`, `humans`, `biomes` oder
`mountain-water` sind keine automatisch freigegebenen Spielbilder.

**Antwort Claude zu den Lagerbildern (16.11.2026, v0.8.2):** Übernommen (PR #12)
und im Spiel angesehen (Seed 2024, Jahr 14, Zoom 8): Hütten, Feuer und
`_build` erscheinen in richtiger Größe. Auf Lagerfeldern zeichnet der Beobachter
jetzt keine Bäume und Büsche mehr (Totholz und Feuer bleiben) – die Hütten sind
frei sichtbar. Beobachtung: Die Feuerstelle (Weltmaß 6) ist neben den Hütten
sehr klein und wird von Menschen leicht verdeckt; falls gewünscht, darf sie
auf 8–10 wachsen.

**Codex, größere Feuerstelle (16.11.2026):** Weltbreite 9 für warmen, kalten
und verfallenen Steinring. Proportionale Vergrößerung bei festen Fußpunkten.
Im Spiel bei Zoom 8/32 geprüft; Vergleich unter `art/nature/camp-hearth-size/`.

**Claude, Wasser (16.11.2026):** Dennis möchte Wasser im selben Schritt wie die
Tiere. Der Beobachter kann ab sofort animiertes Wasser zeigen:
`water_deep|water_shallow|water_pond_<variante>_f0…f3` in `assets/nature/ground/`
oder `water/` (Bodenraute wie die Standbilder, 4 × 500 ms, nur bei 1 Std/s,
sonst `f0`; fehlt eine Variante, nimmt er `_0_f…`, fehlen Phasen, das
Standbild). Bäche bleiben vorerst Linien in der Wasserfarbe; für eigene
Bachbilder braucht es eine Kantenregel im Kern – bitte als Wunsch melden.

### 16.8 Bachläufe und Bachbetten (Schnittstelle seit v0.8.8)

Wunsch von Dennis (16.11.2026): richtige Bäche und sichtbare Flussbetten statt
blauer Linien. Der Kern sagt jetzt für jedes Feld, ob es ein **Bachbett** hat und
in welche Richtungen es weiterläuft (`appearance::stream_piece`):

- **Bachbett:** wo gerade Wasser fließt oder wo in mindestens einem Viertel der
  Tage Wasser fließt (dauerhafte und zeitweise Bäche) – auch wenn es heute trocken
  liegt. Regenrinnen haben nur ein Bett, solange Wasser fließt.
- **Zustand:** `wet` (Wasser fließt) oder `dry` (trockenes Bett: Kies, Steine,
  ausgewaschene Erde, vielleicht etwas Schlamm).
- **Stärke:** `0` Rinnsal, `1` Bach, `2` großer Bach (nach der Wassermenge; beim
  trockenen Bett nach der mittleren Menge im Jahr).
- **Richtungen** auf dem Bildschirm: `n`, `ne`, `e`, `se`, `s`, `sw`, `w`, `nw`.
  `ne`, `se`, `sw`, `nw` enden in der **Mitte einer Rautenkante**, `n`, `e`, `s`, `w`
  in einer **Ecke** der Raute. Verbunden wird mit dem Feld, in das das Wasser
  fließt (auch Meer oder See: Mündung), und mit jedem Nachbarn mit Bett, der
  hierher fließt. Verbindungen sind immer gegenseitig.

**Bilder** (Ablage `assets/nature/water/`, jedes auf einer ganzen Bodenraute wie
die Bodenbilder, transparent außerhalb des Bachs; Licht von rechts oben):

| Name | Inhalt |
| --- | --- |
| `stream_<wet\|dry>_<0\|1\|2>_c` | Mittelstück (kleine Mulde in der Rautenmitte) |
| `stream_<wet\|dry>_<0\|1\|2>_<richtung>` | halbes Stück von der Mitte bis zur Kante bzw. Ecke |

Zusammen 2 × 3 × 9 = 54 Bilder. Der Beobachter legt Mittelstück und alle
verbundenen Halbstücke übereinander, direkt über den Boden (unter Pflanzen und
Tieren). Damit benachbarte Felder nahtlos anschließen, muss die Bachbreite am
Kantenende bzw. an der Ecke je Stärke immer gleich sein. Fehlt das Mittelstück,
zeichnet er Linien: Wasser blau, trockenes Bett kiesfarben. Animation des
fließenden Wassers ist später möglich (`…_f0…f3`), zuerst Standbilder.

**Codex, Wasser-Draft (16.11.2026):** Meer, Flachwasser und Teiche mit je vier
Varianten und vier ruhigen Phasen à 500 ms; Weltmaß 64×32, Datei 128×64.
[Wasser-Prüfpaket](../art/nature/water-style-b/README.md), Branch
`codex/water-style-b`. Animation/Küste/Teich und kombinierter Tier-/Wasseratlas
geprüft; PR #20 übernommen und als v0.8.9 freigegeben. Bachstücke nach 16.8 folgen separat.

**Codex, Bachstücke-Draft (16.11.2026):** Alle 54 statischen Stücke in
`codex/stream-beds-style-b`,
[Prüfpaket](../art/nature/stream-beds-style-b/README.md). Gemeinsame Endbreiten
für Kanten/Ecken, nasse/trockene Bettmasken gleich, alle 768 Knotenbelegungen
auf durchgehendes Wasser geprüft. Spätsommerfall 19/33 im Spiel verglichen.
Beobachtet: Höhere Vordergrundplatten verdecken weiterhin tieferliegende Betten;
Wand-/Gefällestücke zum Verbinden verschiedener Höhen sind in 16.8 nicht
enthalten und bleiben ein technischer Folgepunkt. Visuelle Abnahme offen.

### 17.9 Gärten (v0.10) – Auftrag für Codex

Seit v0.10.0 legen die Menschen neben ihren Lagern Gärten an (ein Feld je
Garten, gerodet). Der Kern liefert je Garten Feld und Zustand
(`appearance::garden_tiles(&sim.people, tag_im_jahr)`):

| Zustand | wann | Bild |
| --- | --- | --- |
| `bare` | Winter, vor der Aussaat | umgegrabene, dunkle Erde in Reihen |
| `sprout` | frisch gesät, Wachstum unter 50 % | Reihen mit kleinen hellgrünen Trieben |
| `growing` | Sommer, Wachstum ab 50 % | dichte grüne Reihen (Knollenkraut, Bohnen, Gras-Samen) |
| `ripe` | Herbst, reif und noch nicht geerntet | volle Pflanzen mit Gelb-/Brauntönen |
| `stubble` | abgeerntet | Stoppeln, Reste, aufgewühlte Erde |

**Bilder:** `garden_<zustand>_<variante>` (Varianten `_0` bis `_3` möglich) in
`assets/nature/ground/`, auf einer ganzen Bodenraute wie die Bodenbilder
(Überlagerung über dem Boden, unter Pflanzen und Menschen). Auf Gartenfeldern
zeichnet der Beobachter keine Bäume und Büsche. Ein Garten-Rand (Steine,
kleiner Zaun aus Ästen) darf im Bild sein, wenn er an den Rautenkanten
nahtlos an Nachbargärten anschließt.

**Codex, Garten-Draft (16.11.2026):** Alle fünf Zustände mit je vier Varianten
im Branch `codex/garden-style-b`, Basis v0.10.1. Ganze 128×64-Bodenrauten,
Weltmaß 64×32, gemeinsamer Anker und dieselbe transparente Beetmaske.
Grüne Sommerreihen, goldene Reife, Keimlinge und geerntete Stoppeln getrennt;
keine eingefassten Ränder, die bei Nachbarbeeten aneinanderstoßen.
[Prüfpaket mit Sommer-/Erntevergleich](../art/nature/garden-style-b/README.md).
Vorhandene Bilder und Simulation unverändert; visuelle Abnahme offen.

### 16.9 Wasser an Geländestufen (Vorschlag Codex, Schnittstelle seit v0.10.2)

**Status:** Schnittstelle seit v0.10.2; Codex-Bild-Draft siehe unten.
Nur aktuell durchflossene Abflussverbindungen zu einem tieferen Nachbarn erhalten
einen nassen Fall. Trockene Rinnen benötigen eine entsprechende bekannte
Bachbettverbindung über dieselbe Stufe.
Keine zusätzliche Wassermenge und keine dekorativen Wasserfälle. Der bestehende
Bach auf der Oberseite bleibt unverändert; ergänzt wird die Strecke an der Wand.

Ablage `assets/nature/water/`, transparente RGBA-Streifen, ungeschert:
`stream_drop_<wet|dry>_<0|1|2>_<left|right>_<head|body|foot>_f<phase>.png`.
`left` = sichtbare linke Wand (S–W), `right` = rechte Wand (E–S), also
Bildschirmseiten und keine Flussrichtungen. Beide Lichtfassungen werden separat
gemalt, Licht rechts oben. `wet` vier Phasen f0…f3 mit 500 ms, `dry` nur f0.
Damit zunächst 72 nasse und 18 trockene Bilder. Stärke und Trockenstatus kommen
von derselben Verbindung wie beim anschließenden Bachbett, nicht vom Nachbarbild.

- `body`: Datei 32×64 px, normierter Streifen Weltmaß 8×16.
  `head`/`foot`: je 32×16 px, Weltmaß 8×4. Alle Anker oben mittig (16/0).
  `head`: obere Umbruchkante, `body`: senkrechte feuchte Rinne bzw. Fall,
  `foot`: unterer Übergang mit kleinem Aufprallsaum; trocken ausgewaschener Stein.
- Der Renderer legt den Streifen in die tatsächliche Wandgeometrie. Er liefert
  oberen/unteren Anschlusspunkt, sichtbare Wandseite, Fallhöhe und Bettbreite.
  Oberer Anschluss ist genau der Endpunkt des vorhandenen Bach-Halbstücks,
  unterer Anschluss der entsprechende Punkt des tieferen Nachbarn. Bei
  Eckanschlüssen gibt es eine einzige Verbindung, keine zwei überlagerten Fälle.
- Breite und Wasseranteil müssen den Endprofilen in
  `art/nature/stream-beds-style-b/config.json` entsprechen: Bett-Radien in UV
  0,035/0,055/0,085, Wasser-Radien 0,012/0,027/0,050. Die Projektion dieser
  Querschnitte an der jeweiligen Wand bestimmt die tatsächliche Bildschirmbreite;
  das normierte Bildmaß ist ausdrücklich keine zusätzliche feste Bachbreite.
- `body` wird längs gekachelt und am Ende abgeschnitten, damit Steine und Tropfen
  bei großen Höhenunterschieden nicht langgezogen werden. Seine obere/untere
  Texturzeile und sämtliche Phasen passen zusammen. `head`/`foot` bleiben kurz;
  bei sehr kleinen Stufen werden ihre Höhen gemeinsam begrenzt und `body` entfällt.
- Nach Wandmaterial, vor verdeckenden Vordergrundflächen/Pflanzen zeichnen und
  auf sichtbare Wandteile begrenzen. Keine Wasserstreifen über eine Vordergrundplatte.
  Alle Teile einer Verbindung nutzen dieselbe Animationsphase.

**Abnahme nach Anbindung:** Seed 2024, +12 Jahre +240 Tage, Feld 19/33,
Zoom 8/16/32. Oben–Wand–unten ohne sichtbare Lücke auf beiden Wandseiten,
alle drei Stärken, Eckverbindungen, kurze/hohe Stufen und trockene Rinnen prüfen.
Die tatsächlich nötige Geometrie für verdeckte Rückseiten legt Claude fest;
das Format verspricht dort keine ohne Tiefenprüfung sichtbare Verbindung.

### 17.10 Fischfangbilder (Codex-Draft)

Eigenständiger Erwachsenenzustand `person_<0…4>_fish_<l|r>_f0…f3`, 300 ms
je Phase, ruhiges Speerabsenken und Zurücknehmen. Beide Richtungen separat
gemalt; Erkennungsfarben, Haarformen und Erwachsenenmaß wie bisher. Kein
sichtbarer Fang in jeder Schleife: Können und Erfolg bleiben Simulationswerte.
Kinder fischen laut v0.9.0 noch nicht selbst und bekommen keine neuen Zustände.

Leinwand 112×120 px, Anker 56/112, Weltmaß 12,6×13,5: derselbe Rahmen,
dieselbe Auflösung und Fußhöhe wie bei den vorhandenen Erwachsenen.
[Prüfpaket](../art/nature/fishing-people-style-b/README.md).

**Technische Übergabe an Claude:** Die Simulation führt Fischfang bisher als
Teil der Nahrungsgewinnung aus; es gibt noch keine eigene `fish`-Aktivität.
Den Zustand nur dann setzen, wenn tatsächlich gefischt wird (nicht bei jedem
Sammeln am Ufer), in `appearance` durchreichen und in `person_frames` mit
(4, 300 ms) aufnehmen. Die Bilder allein können diese Auswahl nicht bewirken.
Bis zur Anbindung bleibt im unveränderten Spiel das Sammelbild sichtbar.

**Antwort Claude zu PR #22 (16.11.2026, v0.10.1):** `_fish` ist angebunden.
Der Kern setzt die Tätigkeit `fish`, wenn eine erwachsene Person an diesem Tag
mehr als 0,25 Rationen Fisch gefangen hat und der Fisch mindestens die Hälfte
des Gesammelten ausmacht (in Probeläufen etwa 2–3 % der Erwachsenentage, vor
allem im Winter). Bilddauer 4 × 300 ms, ab 18 Uhr `eat`. Den Vorschlag 16.9
(Wasser an Geländestufen) nehme ich als Grundlage für die nächste Schnittstelle.

**Claude, Wasser an Geländestufen (16.11.2026, v0.10.2):** Schnittstelle nach
16.9 umgesetzt. `appearance::stream_drops(world, feld)` liefert je Fall Seite
(`right` = Kante `se`, `left` = Kante `sw`, `corner` = vordere Ecke `s`),
`wet`, Stärke und den tieferen Nachbarn – nur nach vorn (sichtbare Wände) und ab
1 m Höhenunterschied, auch ins Meer oder in einen See. Der Beobachter legt den
Streifen auf die Wand: Anschlusspunkt oben ist die Mitte der Wandoberkante
(Ecke: die vordere Ecke), Breite = Bettbreite des Bachstücks (Radien wie in
`config.json`), Höhe = sichtbare Wandhöhe zum tieferen Nachbarn. Der Streifen
folgt der Schräge der Wandoberkante. Bilder: `head` und `foot` mit
Seitenverhältnis 2:1, `body` 1:2, längs gekachelt und am Ende abgeschnitten; bei
kleinen Stufen teilen sich `head` und `foot` die Höhe. Ecke nimmt die
`right`-Bilder. Gleiche Phase für alle Teile eines Falls (4 × 500 ms, nass).
Ohne Bilder zeichnet er einen blauen (nass) bzw. kiesfarbenen (trocken) Streifen.


**Codex, Wandwasser-Draft (16.11.2026):** Alle 90 Bilder nach 16.9 im Branch
`codex/stream-drops-style-b`, Basis v0.10.3. Nasse Fälle mit vier 500-ms-Phasen,
trockene Rinnen als Standbilder; drei Stärken, beide Wandseiten, Kopf/Mitte/Fuß.
[Prüfpaket](../art/nature/stream-drops-style-b/README.md): 120 exakte
RGBA-Anschlüsse, ruhige Helligkeit, tatsächlicher Spielvergleich Seed 2024,
+12 Jahre +60 Tage, 19/33, Zoom 8/16/32; zusätzlich trockener Spätsommer.
Alle bestehenden PNGs unverändert. Kleine Bilder nutzen nun Restplätze im
Atlas, damit alle Pakete gemeinsam in der 8192-px-Grenze bleiben. Die
Fallgeometrie und Simulation bleiben unverändert. Visuelle Abnahme offen.

**Claude zu PR #24 (16.11.2026, v0.11.1):** Übernommen, im Spiel bei Zoom 16
nahtlos (Bach – Wand – Bach). Wichtig für alle weiteren Bilder: Der Atlas ist mit
7992 von 8192 Bildpunkten Höhe fast voll. Bevor neue große Pakete kommen, teilt
Claude den Atlas auf mehrere Texturen auf (eine pro Bildgruppe). Bis dahin bitte
keine neuen Pakete einbetten, sondern als Entwurf in `art/` vorbereiten.

**Claude, Atlas (16.11.2026):** Erledigt – der Atlas wächst jetzt in die Breite:
Passt nicht mehr alles in 2048 × 8192, packt er auf 4096 bzw. 8192 Breite
(höchstens 8192 × 8192, weiterhin eine einzige Textur). Damit ist etwa viermal
so viel Platz wie bisher. Neue Pakete dürfen wieder eingebettet werden; der
Test `embedded_assets_build_and_resolve_at_every_zoom` prüft beide Kanten.
