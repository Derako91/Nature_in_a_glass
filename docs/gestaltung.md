# Gestaltung – visuelle Richtung von Nature in a Glass

Stand: v0.2 · Geltungsbereich: **Naturdarstellung** (Gelände, Wasser, Vegetation, Feuer). Bewohner, Gebäude und Oberfläche folgen später in eigenen Abschnitten.

Dieses Dokument ist die verbindliche Vorlage für Texturen, Sprites und Tiles. Es ist so geschrieben, dass ein Werkzeug wie Codex daraus Schritt für Schritt Assets erzeugen kann: Jede Asset-Gruppe hat Maße, Dateinamen, Varianten, Zustände und Abnahmekriterien.

---

## 1. Leitbild

**Eine kleine Welt unter Glas, auf die man gerne schaut.**

- Miniaturwelt im Geist alter Aufbau- und Strategiespiele der späten 1990er: isometrische Draufsicht, gestufte Höhen, warme, handgemalte Anmutung, klare Formen. **Eigene Formen und Farben – keine bestehenden Spiele nachbilden oder Assets daraus übernehmen.**
- Nicht fotorealistisch, nicht verspielt-comichaft. Ruhig, natürlich, leicht stilisiert.
- Die Welt erzählt ihre Geschichte über Spuren: Brandflächen, Totholz, alte und junge Wälder, ausgetrocknete Bachbetten.

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
- Höchstens **6 Töne pro Asset** (inklusive Schatten und Glanzlicht), aus der Familie der jeweiligen Ankerfarbe.
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

Format für alle: **PNG, 32 Bit mit Transparenz, sRGB**, keine halbtransparenten Fransen an den Rändern (Kanten entweder voll deckend oder voll transparent, Ausnahme: Schatten, Wasser, Feuer). Kein Hintergrund, keine eingebauten Schatten außer dem Schlagschatten der Sprites.

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
| `marker_spring` | 12 × 12 | Quelle: kleiner Quelltopf, sprudelnd | 3 Frames + 1 Bild „versiegt“ |

Wasser-Animationen: höchstens 15 % Helligkeitsunterschied zwischen Frames (Ruhe-Regel).

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
- [ ] Höchstens 6 Töne, alle aus der Familie der Ankerfarbe (Abschnitt 5).
- [ ] Licht von oben rechts, Schatten links unten.
- [ ] Tiles: nahtlos in alle Richtungen; Wände: vertikal nahtlos.
- [ ] Silhouettentest bei 50 % Größe bestanden (Abschnitt 6).
- [ ] Auf dem Prüfblatt neben den Nachbar-Assets stimmig, keine Variante fällt auf.
- [ ] Animationen: Helligkeitsunterschied zwischen Frames innerhalb der Grenze, Schleife ohne Sprung.

## 12. Was ausdrücklich nicht gewünscht ist

- Dekoration ohne Simulationsgrundlage (Blumen, Tiere, Pilze, Steine), solange die Simulation sie nicht kennt.
- Zufällig wechselnde Varianten, Wackeln, Partikelregen, Linsenreflexe.
- Eingebaute Tageszeit-Beleuchtung in Texturen.
- Nachbildungen bekannter Spiele, Marken oder Figuren.
- Texturen in der 2D-Analysekarte.

## 13. Offene Punkte für spätere Fassungen

- Jahreszeiten (Herbstfärbung, Schnee) – die Simulation kennt Temperatur, aber noch keinen Schnee.
- Baumarten mit eigener Silhouette, sobald die Simulation sie unterscheidet.
- Erosion und neue Bachläufe (v0.3) – erfordert Assets für Ufer, Kies und Abbruchkanten.
- Tiere (v0.4) und Bewohner (v0.6) in eigenen Abschnitten.
- Technische Einbindung: Der Renderer zeichnet heute Flächenfarben ohne Texturen. Für diese Assets braucht er einen Textur-Atlas mit Manifest-Loader; das wird als eigenes ADR entschieden, sobald die ersten Assets abgenommen sind.
