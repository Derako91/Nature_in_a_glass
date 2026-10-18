# ADR 0008 – Detailstufen für Sprites

**Status:** angenommen (2026-10-18)

## Kontext

Dennis hat den detailreichen Stil B (Fuchs-Entwurf) gewählt. Ein Spielvergleich zeigte: Das 8-fach aufgelöste Bild ist bei Zoom 16/32 deutlich schöner, bei Zoom 4/8 aber schlechter lesbar als der bisherige Fuchs, weil der Beobachter es pixelgenau (ohne Glättung) um das 4- bis 8-Fache verkleinerte. Künftig kommen Menschen, Gebäude und Gegenstände in ähnlicher Feinheit. Bestehende Pixelgrafik soll nicht weichgezeichnet werden.

## Entscheidung

- **Detailstufen je Bild** im Atlas: Hauptfassung `name.png` plus beliebige Stufen `name@<stufe>.png` von Codex. Für Sprites ab 48 px Breite berechnet der Atlas zusätzlich halbierte Stufen (vormultiplizierte Mittelung über 2 × 2, Deckkraft leicht nachgeschärft), bis zur Darstellungsgröße bzw. bis knapp über eine vorhandene Fernansicht.
- **Auswahl** (`atlas::choose_level`): die feinste Stufe, die höchstens 1,5-fach verkleinert gezeichnet würde, sonst die gröbste. Damit wird die Textur nie stark verkleinert (kein Flimmern, keine zerfallenden Silhouetten), und eine eigene Fernansicht greift automatisch dort, wo sie passt.
- **Gleiches Weltmaß:** Jede Stufe wird auf die Darstellungsbreite gebracht (aus `display_size` im Manifest, sonst Tabellen der Gestaltung); Fußpunkte werden je Stufe gelesen oder aus der Hauptfassung umgerechnet. Beim Stufenwechsel bleibt die Figur an derselben Stelle und gleich groß (Test `fox_levels_follow_zoom`).
- **Textur bleibt `NEAREST`**, eine einzige Atlastextur, ein Zeichendurchgang (ADR 0005). Boden, Wände, Wasser und Sprites unter 48 px bleiben unverändert.
- **Einheitliche Schnittstelle** (`Atlas::pick`) für Pflanzen, Tiere, Menschen, Tote, Kadaver und künftige Gebäude/Gegenstände (`objects`, `buildings` werden schon eingebettet). Fehlende Bilder oder Stufen führen zu Ersatzdarstellungen, nie zum Absturz.
- **`--bilder <ordner>`** lädt Bilder zur Laufzeit dazu, damit Codex Pakete im echten Spiel prüfen kann, ohne Rust zu bauen.
- Der Simulationskern ist nicht betroffen.

## Konsequenzen

- Der Atlas wird etwas größer (berechnete Stufen etwa ein Drittel zusätzlich für feine Bilder); ein Test stellt sicher, dass er unter 8192 px Höhe bleibt.
- Berechnete Stufen sind eine Rückfallebene: Bei Zoom 8 gut, bei Zoom 4 verliert ein feines Tier seine Merkmale. Für Stil B liefert Codex deshalb eigene Fernansichten (gestaltung.md 16.6).
- Echte Mipmaps der Grafikkarte sind nicht nötig; sie würden alle Bilder glätten und an den Rändern der Atlasbilder ineinanderlaufen.
