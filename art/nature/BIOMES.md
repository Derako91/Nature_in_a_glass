# Biome-Prüfblatt

Sechs Landschaftsentwürfe im zuletzt gewünschten Stil, ergänzt am 17.10.2026.
Ergebnis: `assets/nature/_biomes-testsheet.png`. Einzelbilder, feste Testbelegungen,
Materialprototypen und Manifest liegen unter `assets/nature/biomes/`.

Die sechs Typen sind feuchter Wald, Wiesenland, trockenes Buschland, Steppe,
Feuchtgebiet und felsiges Bergland. Die Auswahl berücksichtigt die vorhandenen
Felder in `crates/core/src/world.rs` und die Vegetation in
`crates/core/src/vegetation.rs`. Noch gibt es keine Biome-ID, keine Unterscheidung
von Baumarten und keinen Schnee im Kern. Das Prüfblatt ergänzt die Gestaltung,
es implementiert keine Biome-Simulation.

## Lokale ComfyUI-Erzeugung

Die drei zusätzlichen Materialvorlagen `dry_soil`, `wet_soil` und
`highland_rock` liegen mit vollständigen Prompts, API-Workflows und tatsächlichen
Ausführungsprotokollen in `art/nature/comfyui-biomes/`.

Verwendet wurde das vorhandene Modell `sd_xl_base_1.0.safetensors`, 768×768 px,
24 Schritte, CFG 6,5, DPM++ 2M/Karras, Seeds 2026101700–2026101702.
Die finale Verarbeitung bringt die Vorlagen auf das Rautenraster und sechs
Farben. Vegetation, Gras- und Wasserbilder stammen aus Prüfblatt 02.

```powershell
python tools/nature/generate_biome_sources.py
python tools/nature/build_biome_proof.py
```

Ausführung mit der Python-Umgebung der lokalen ComfyUI-Installation; benötigt
Pillow und NumPy. Der erste Aufruf benötigt ComfyUI auf Port 8188 und überspringt
bereits erzeugte Vorlagen. Der zweite erstellt die Landschaftsproben und das
Vergleichsblatt aus gespeicherten Quellen neu.

## Testbelegungen und Prüfung

Jede Landschaft zeigt 4×4 Felder bei zweifacher Pixelvergrößerung. Die JSON-Dateien
neben den Einzelbildern enthalten die bewusst gesetzten Werte pro Feld.
`moisture` ist der normierte Feuchteanteil, `age` das Bestandsalter; die übrigen
Bezeichner entsprechen den gleichnamigen Größen der Simulation. Diese Dateien
sind Designbelegungen und kein Spielstandformat.

Die Baum- und Buschzahlen folgen den Schwellen aus der Gestaltungsvorlage.
Offene Wasserflächen sind an `surface_water > 150` gebunden und tragen keine
Vegetationssprites. Im Wiesenland steht der Bach für `flow = 70`.

Der Build prüft Maße, RGBA-Modus, Farbanzahl und die Transparenz außerhalb der
Raute. Das Manifest enthält die sRGB-Kennzeichnung und die Materialdaten.
Das gesamte Blatt wurde visuell auf unterscheidbare Landschaften, Überlagerung
und lesbare Beschriftung geprüft. Animationen, alle Materialübergänge und die
spätere Zuordnung im Spiel sind noch nicht Bestandteil dieses Gestaltungsstands.
