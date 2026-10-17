# Runde 03 – Tierformen und Naturkorrekturen

Grundlage ist `docs/gestaltung.md` v0.3, Abschnitt 15, Stand des Hauptzweigs
`9d92e2a`. Diese Runde beginnt die Tiergestaltung und verbessert zwei bestehende
Naturmotive. Sie ändert weder Simulationskern noch Renderer.

## Neue Tierentwürfe

`assets/nature/_animals-testsheet.png` zeigt Hirsch, Kalb, Wolf, Fuchs und Kaninchen
in beiden Richtungen und vier Zuständen. `assets/nature/_animals-graze.gif` zeigt
die zwei Fressbilder als Schleife von 1,2 Sekunden. Die PNGs und ihr eigenes
Manifest liegen unter `assets/nature/animals/`.

Enthalten sind 50 Bilder: fünf Größen/Typen, zwei Richtungen, zwei Fressbilder,
ein Ruhebild und jeweils eine Geh- und Laufpose. Die Geweihkontur ist nur beim
erwachsenen Hirsch vorhanden. Die Blickrichtungen besitzen getrennte Konturen;
die Beleuchtung bleibt jeweils rechts oben. Kein fertiges Sprite wird gespiegelt.

**Noch keine fertige Animationsserie:** Variante 1, Geh- und Laufzyklen mit je
vier Frames, Kadaver und Spuren sind im Manifest ausdrücklich als fehlend
aufgeführt. Die Formen sollen zuerst am Prüfblatt beurteilt werden.

### ComfyUI und Pixelredaktion

Acht lokale SDXL-Studien (vier Arten × zwei angeforderte Richtungen) wurden mit
768×768 Pixeln, 24 Schritten, CFG 6,5, DPM++ 2M/Karras und Seeds
2026101900–2026101907 erzeugt. Prompts und tatsächliche Ausführungsprotokolle
stehen unter `art/nature/animals/`. Rohbilder liegen ausschließlich im ignorierten
Unterordner `raw/` und außerdem im lokalen ComfyUI-Ausgabeordner.

Die Rohstudien sind **Form- und Fellreferenzen, keine direkt ausgeschnittenen
Spielsprites**. Einige Studien treffen die gewünschte Blickrichtung nicht oder
zeigen angeschnittene Körper beziehungsweise Rahmen. Bei den verbindlichen
3–9 Pixel breiten Zielbildern wurden deshalb Konturen, Glieder und Lichtpixel
gezielt neu gesetzt. Diese Pixelredaktion ist vollständig im Build-Skript
beschrieben; sie benötigt die großen Rohbilder nicht zur Wiederholung.

### Prüfung und offene Grenze

- Maße, Alpha-Werte und maximal sechs Farben wurden für alle 50 Bilder geprüft.
- sRGB-Profile sind in den PNGs eingebettet; Schatten verwenden Alpha 77.
- Der Prüfbericht misst den mittleren absoluten Helligkeitsunterschied der
  deckenden Tierpixel zu einem festen Waldboden-Ausschnitt. Die 15-%-Schwelle
  wird geprüft. Das ist eine begrenzte Materialprobe, kein Nachweis für jeden
  möglichen Hintergrund im späteren Spiel.
- Die Halbgrößen-Silhouetten sind sichtbar auf dem Prüfblatt dokumentiert.
  **Das strenge visuelle Abnahmekriterium ist noch nicht vollständig erfüllt:**
  Fuchs und Kaninchen verlieren dabei wesentliche Merkmale. Bei 3×3 Pixeln ist
  das Kaninchen vor allem als kleine Menge beziehungsweise Kleinform erkennbar.
  Die festgelegten Maße wurden deshalb nicht eigenmächtig vergrößert.

## Verbesserte Natur

- Altwald: eigenständige rundere Pixelkontur, geschlossene dunkle Krone, Licht
  rechts oben. Das Blattmuster wird aus der vorhandenen ComfyUI-Vorlage gewonnen.
- Sand: feinere Körnung aus derselben Bodenquelle, weniger auffällige große
  Wiederholungsmuster. Keine neue große Quelldatei erforderlich.

Die aktuellen Natur- und Biome-Prüfblätter wurden mit diesen Änderungen neu
gebaut. Die älteren Vergleichsbilder bleiben historische Vergleiche von Runde 02.

## Erneut erstellen

Mit dem Python der vorhandenen ComfyUI-Umgebung:

```powershell
python tools/nature/generate_animals.py
python tools/nature/build_proof_v3.py
python tools/nature/build_biome_proof.py
python tools/nature/build_animal_proof.py
```

Nur der erste Aufruf benötigt einen laufenden ComfyUI-Dienst. Die Build-Skripte
lesen gespeicherte Vorlagen beziehungsweise die redigierten Pixelkonturen.
Die Tierbilder sind bereits direkt reproduzierbar; die Naturkorrektur verwendet
die bestehenden ComfyUI-Quellen aus den vorigen Runden.
