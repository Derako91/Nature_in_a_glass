# Menschen nach Abschnitt 17 – Nahansicht

Ersetzt den vorläufigen Maßstab und die frei gewählten Zustände aus HUMANS-01.
Die alten Grundfiguren unter `humans/` bleiben als getrennter Vorentwurf erhalten;
für die spätere Bewohnerdarstellung ist ausschließlich `people/manifest.json`
maßgeblich. Die Kamera-/Simulationsintegration übernimmt Claude gemäß ADR 0006.

## Lieferumfang

- Fünf feste Aussehen (drei weiblich, zwei männlich), IDs 0–4, mit unterschiedlicher
  Statur, Haar-/Kleidungsfarbe und rotem, blauem, violettem, ockerfarbenem bzw.
  grünem Erkennungsband. Die Farben stehen im Manifest.
- `person`: 56×120 Pixel, Darstellung 7×15, Fußpunkt [28,112] in Dateipixeln.
- Acht Zustände: idle 2×800 ms, walk 4×200 ms, run 4×100 ms, gather 4×300 ms,
  eat/drink jeweils 2×600 ms, sleep ein Bild, hurt 4×250 ms; je links/rechts.
- Zehn `person_dead_<aussehen>_<richtung>_f0.png`, 120×40 Pixel, Darstellung
  15×5, Anker [60,20], entsättigte Palette.
- `person_marker_0.png`, weißes 16×16-Zeichen für Renderer-Tönung,
  Darstellung 8×8, Anker [8,14].
- Insgesamt 241 unterschiedliche PNGs; keine identischen Dateien doppelt abgelegt.

Die Konturen sind neu in Dateiauflösung gezeichnet, nicht aus dem kleinen
Vorentwurf hochskaliert. Das bearbeitbare Pixelprogramm `build_people.py` baut
Formen und Beleuchtung getrennt auf, sodass das Licht bei beiden Richtungen
rechts oben bleibt. Keine neue KI-Generierung. Maximal sechs Farben einschließlich
Schatten, sRGB, Alpha 0/77/255. Schattenversatz vier Dateipixel entspricht einem
halben Grundpixel bei achtfacher Auflösung. Keine Kindergrafiken in dieser Runde.

## Prüfung

Der Bau prüft Maße, Farben, Alpha und die Anzahl verschiedener Phasen sowie
Bildzahl und Zeitangaben der sieben tätigkeitsbezogenen GIFs. Die Tafeln zeigen
alle fünf Aussehen, idle/walk in beiden Richtungen und den Größenvergleich bei
Zoom 4, 8 und 32. Weitere Tafeln zeigen sämtliche Zustände und die Verstorbenen.
Die Tätigkeitstafel wurde visuell kontrolliert; diese sind Prüfzeichnungen,
keine Spielaufnahmen. Identitätserkennung bei Zoom 8 unter wechselndem Licht
und Landschaftshintergrund bleibt ein Abnahmepunkt im späteren Beobachter.

Ein Widerspruch in der Vorlage bleibt zur Abstimmung: 17.1 fordert Menschen
höchstens so hoch wie einen Jungbaum, die Tabellen geben 15 bzw. 14 Grundpixel
an. Verwendet werden die konkreten Maße aus 17.3. Der Vergleich zeigt diesen
Unterschied offen; keine heimliche Änderung des Baummaßstabs.

## Integration

Nur `assets/nature/people/manifest.json` verwenden: Bildmaße, display_size,
Dateipixelanker, appearance/direction/frame und Millisekunden sind angegeben.
Der bestehende Atlas bettet `people/` noch nicht ein. Zustandswahl, Zeitraffer,
Zoom bis 32, Markerfarbe, Schwäche-Tönung, Namen und Lebensdauer der Verstorbenen
kommen aus der späteren Simulation/Beobachterintegration. Keine Menschen wurden
in den laufenden Spielstand eingefügt. Vierfach aufgelöste Nahlandschaft folgt
als eigene Runde nach dieser Menschenvorbereitung.
