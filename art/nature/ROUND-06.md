# Runde 06 – Kadaver und Spuren

Ergänzt die Tieranimationen gemäß Abschnitt 15.3 und 15.5 der Gestaltungsvorlage:

- `carcass_large`: 9×4 px, zwei entsättigte Farbvarianten, links/rechts.
- `carcass_small`: 4×2 px, dieselben Varianten und Richtungen.
- `tracks_deer`, `tracks_wolf`, `tracks_fox`, `tracks_rabbit`: je 64×32 px,
  außerhalb der Raute transparent, höchstens drei Farben. Hufpaare, unterschiedlich
  große Pfoten und Kaninchensprunggruppen; zusätzliche Fraßmarken für Pflanzenfresser.

Die zwölf PNGs sind eigenständige Pixelzeichnungen im Raster der bisherigen
ComfyUI-basierten Tiergrafiken. Für diese winzigen Ergänzungen wurde keine neue
KI-Generierung ausgeführt. Licht bleibt rechts oben, Schatten links unten mit
Farbe #0C1A34 und Alpha 77. Beide Richtungen werden getrennt beleuchtet.
Kadaver verwenden höchstens vier sichtbare RGB-Farben einschließlich Schatten.

Alle Zuordnungen stehen im vorhandenen `assets/nature/animals/manifest.json`.
Kadaver werden wie die übrigen Tiere über `frame_map` aufgelöst; ihr Anker ist
die ganzzahlige Rastermitte. Die Farbvarianten sind eine Auswahlmöglichkeit,
keine neue Regel zur Art- oder Alterszuordnung. Spuren haben Anker [32,16].
Das Kalb gehört zur Art Hirsch und erhält daher kein separates Spurenfeld.

Spuren haben im PNG harte Transparenz. Der Renderer multipliziert ihre Deckkraft
nach Dichte; die Vorschau zeigt beispielhaft 65 Prozent. Eine Dichte von null
muss unsichtbar bleiben. Umschaltung ab 1 Monat/s, Kadaver-Lebensdauer und Auswahl
liefert später die Spielintegration. Die Grafiken führen keine Simulationsregeln ein.

Neu bauen: `python tools/nature/build_animal_remains.py`; der vollständige
`build_animal_cycles.py` ruft diesen Schritt ebenfalls auf, damit das Manifest
bei erneutem Bauen vollständig bleibt. Voraussetzung ist die vorhandene
Pillow/NumPy-Umgebung. Die Ergebnisse liegen unter `assets/nature/animals/`,
die Prüfübersicht unter `assets/nature/_animals-remains.png`.

Geprüft: Maße, Farbzahl, Alpha-Werte, Rautengrenzen, geringe Spurbelegung und
unterschiedliche Pixelinhalte aller zwölf Ergänzungen. Die Vorschau wurde auf
Gras, Busch- und Waldboden visuell geprüft. Die Spuren sind bewusst dezent;
Arterkennung bei reduziertem Zoom und geeignete Dichte-Deckkraft bleiben im
Spiel zu prüfen. Die bisherigen Einschränkungen der lebenden Kleintiere gelten weiter.

Kadaver und Spuren sind damit als Grafikentwürfe vorhanden; Renderer-Einbindung
und abschließende Abnahme im Spiel stehen weiterhin aus. Die offenen Angaben
in Runde 03/04 beschreiben den damaligen Stand und werden hiermit ergänzt.
