# Wasser an Geländestufen – Gestaltung 16.9

Basis `3ac3bc1` (v0.10.3, Gärten und Fischfang bereits enthalten), Branch
`codex/stream-drops-style-b`. 90 neue PNGs in `assets/nature/water/`:
72 nasse Bilder (drei Stärken × zwei sichtbare Wandseiten × drei Teile × vier
Phasen), 18 trockene Standbilder. Keine Änderung an Simulation, Wassermenge,
Flussverbindungen oder Fallgeometrie aus v0.10.2.

## Bilder und Anschluss

`stream_drop_<wet|dry>_<0|1|2>_<left|right>_<head|body|foot>_f<phase>.png`.
Links/rechts sind Bildschirm-Wandseiten; Eckverbindungen verwenden laut API
die rechte Fassung. Beide Seiten haben separat erzeugte Felsmaterialien und
eine eigene Helligkeit, keine gespiegelten Felszeichnungen.

| Teil | Datei | Normiertes Weltmaß | Anker |
| --- | --- | --- | --- |
| `head` | 32×16 RGBA | 8×4 | 16/0, oben mittig |
| `body` | 32×64 RGBA | 8×16 | 16/0, oben mittig |
| `foot` | 32×16 RGBA | 8×4 | 16/0, oben mittig |

Der Renderer setzt Breite, Schräge und Höhe anhand der wirklichen Wand.
Das Mittelstück ist senkrecht kachelbar; letzte Teilstücke schneidet der
vorhandene Renderer ab, kurze Stufen teilen sich Kopf und Fuß.

- Wasseranteile folgen exakt den drei Verhältnissen Wasser-/Bettradius aus
  `stream-beds-style-b/config.json`: 0,012/0,035; 0,027/0,055; 0,050/0,085.
  Die normierte Streifenbreite ist keine zusätzliche feste Bachbreite.
- Wasserfarbe am oberen/unteren Anschluss entspricht dem Bachblau 65/128/163.
  Kopf und Fuß gehen von den Bachmaterialfarben zu beschattetem Wandgestein
  über; kleine helle Umbruch-/Aufprallstellen innerhalb des Wasserinneren.
- Kopfende/Mittelstückanfang, Mittelstückende/-anfang und Mittelstückende/
  Fußanfang besitzen identische RGBA-Zeilen, ebenso oberer/unterer Bachanschluss.
  Diese 120 Vergleiche prüfen ganze Teile; ein abgeschnittenes letztes
  Mittelstück wird zusätzlich im tatsächlichen Spiel betrachtet.
- Endzeilen bleiben über alle vier Phasen gleich. Wasser bewegt sich innerhalb
  seines Querschnitts nach unten, Fels und Randmaske bleiben stehen. Feine helle
  Strähnen stammen aus dem gemalten Wassermaterial. Vier Phasen à 500 ms,
  Schleife zwei Sekunden; maximale gemessene mittlere Helligkeitsschwankung
  ca. 1,09 %, deutlich unter der 15-%-Grenze.
- Trockene Rinnen zeigen ausgewaschenen, vertieften Stein, ohne Wasser und
  ohne Animation. Gleiche Alpha-Geometrie wie die entsprechenden nassen Teile.

Neu erzeugter Materialatlas unter `sources/materials.png`: feuchtes/trockenes
Wandgestein links/rechts, fließendes Wasser und Schaummaterial. Stilreferenzen
sind die übernommenen Wasser-/Bachmaterialien. Quellenprüfsumme und Maße in
`config.json`, sämtliche Exportprüfsummen in `export.json`.

[Prüfbogen mit zwei gestapelten Mittelstücken](joins-contact.png) und
[Animation aller sechs nassen Kombinationen](animation.gif) sind synthetische
Bögen aus tatsächlichen PNGs, keine Spielaufnahme. Die trockene Reihe bleibt
in der Animation stehen.

## Atlas: neue Bilder passen gemeinsam mit den Gärten

Das bisherige Regalpacken ließ Restplätze älterer Reihen frei. Mit Gärten,
Fischfang und den 90 Wandteilen ergab das Dimensionsmodell 8228 px Inhaltshöhe;
die nächste Texturhöhe wäre 16384 px und damit über der bestehenden Grenze.

Der Packer nutzt jetzt die erste schon vorhandene Reihe mit genügend Platz
und Höhe. Sortierung, 2048-px-Breite und ein Pixel Abstand bleiben fest.
Keine Bildverkleinerung, keine Änderung an Auflösung, Darstellungsmaß,
Fußpunkt, Detailstufen oder Quelldateien. Im gleichen Dimensionsmodell sinkt
die Inhaltshöhe auf 7992 px, die Textur bleibt bei 2048×8192.
[Dimensionsmodell](atlas-layout.json); der echte Vollatlas wird im Rust-Test
mit sämtlichen eingebetteten Bildern aufgebaut.

Die Regression mit verschieden großen Farbflächen prüft jeden gepackten Pixel,
Maße, einen eigenen Fußpunkt sowie identische Ergebnisse bei umgekehrter
Eingabereihenfolge. Kleine Bilder nutzen dort frühere Lücken und halten den
Atlas bei 512 statt 1024 px Höhe. Dies ist die einzige produktive Änderung
außerhalb der Bild-/Exportdateien; keine Änderung am Wasserfall-Renderer.

## Tatsächliche Spielprüfung

```sh
target/release/nature-in-a-glass --seed 2024 --years 12 --days 60 --select 19,33 --zoom 16
```

Zusätzlich Zoom 8/32 sowie +240 Tage (Spätsommer, trockene Rinnen) bei Zoom
16/32. Ohne `--speed` pausiert der Beobachter; Jahr 13, Tag 61 bzw. 241,
10:00 Uhr. Software-OpenGL/Xvfb, 1440×900, keine Änderungen an Wasserständen,
Pflanzen oder Flussrichtungen. Neue Bilder vollständig eingebettet, kein
`--bilder` im Nachher-Spiel.

| Fall | Einfarbige Platzhalter | Eingebettete Bilder |
| --- | --- | --- |
| +60 Tage, Zoom 8 | [Vorher](vorher-tag60-zoom8.png) | [Nachher](nachher-tag60-zoom8.png) |
| +60 Tage, Zoom 16 | [Vorher](vorher-tag60-zoom16.png) | [Nachher](nachher-tag60-zoom16.png) |
| +60 Tage, Zoom 32 | [Vorher](vorher-tag60-zoom32.png) | [Nachher](nachher-tag60-zoom32.png) |
| +240 Tage, Zoom 16 | [Vorher](vorher-tag240-zoom16.png) | [Nachher](nachher-tag240-zoom16.png) |
| +240 Tage, Zoom 32 | [Vorher](vorher-tag240-zoom32.png) | [Nachher](nachher-tag240-zoom32.png) |

Ausgangsaufnahmen auf v0.10.2 (`25ea9cb`), Nachher auf v0.10.3 plus diesem PR.
Der inzwischen übernommene Garten-/Vorratsstand betrifft in diesem menschenleeren
Naturfall keine Bewohner oder Gärten. Alle bestehenden PNGs bleiben bytegleich;
die Rasteranordnung im Atlas verändert sich, nicht die vorhandenen Motive.

Die vorhandene API liefert nur sichtbare vordere Wandverbindungen. Weiterhin
verdeckte Bachabschnitte hinter höheren Vordergrundplatten werden durch diese
Bilder nicht künstlich darübergelegt. Sichtbare Wandanschlüsse werden ohne
zusätzliche Wasserfälle außerhalb der gelieferten Verbindungen dargestellt.

## Wiederholen und prüfen

```sh
python tools/nature/export_stream_drops.py
python tools/nature/check_stream_drops.py
```

Pillow/NumPy erforderlich. Wandwasser-, Bachbett-, Wasser- und vollständiger
Landschaftsexport liefern dieselben PNGs/Manifeste bei Wiederholung. Der
Bachbettexport erhält die anderen Familien und ihre Reihenfolge; sein Prüfer
unterscheidet `stream_wet`/`stream_dry` von `stream_drop`.

103 Rust-/15 bestehende Python-Tests, fmt, strenges Release-Clippy und
Release-Bau bestanden. Alter Bachprüfer weiterhin grün: 54 Teile, 24 Gegenprofile,
768 Knotenkombinationen. Alle 1393 Eltern-PNGs unverändert. Visuelle Abnahme
durch Claude offen.


[Animation im tatsächlichen Spiel](spiel-animation.gif): acht vollständige
Fensteraufnahmen im Abstand von 500 ms bei 1 Std/s, gleicher Seed/Feld,
+60 Tage, Zoom 32. Zwei Sekunden lange Phasenschleifen sind mehrfach erfasst;
keine nachträglich eingesetzten oder aus dem Prüfbogen erzeugten Spielbilder.
Der Exportprüfer bestätigt unveränderte Randmasken und feste äußere Felsfarben
über die Phasen. [Pixelvergleich](native-comparison.json): Im stehenden
Frühlings-/Sommervergleich ändern sich bei Zoom 16 jeweils 680 Pixel im
sichtbaren Wandstreifen; der übrige Bildinhalt bleibt gleich.
