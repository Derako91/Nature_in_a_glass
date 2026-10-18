# Wiederverwendete Grafik-Infrastruktur

Untersuchung am 18.10.2026, vor Umsetzung des Sprite-Agenten.

| Vorhanden | Verwendung |
| --- | --- |
| `tools/nature/generate_sources.py`, `generate_animals.py` | SDXL-Graph, lokale `/prompt`-/`history`-/`view`-Schnittstelle, Seeds und Protokollierung als Basis |
| `tools/nature/revise_sources.py` | VAEEncode-/img2img-Aufbau für gleichgerichtete Bildvariationen |
| ComfyUI `user/default/workflows/01-img2img.json` | Bereits eingerichteter Bild-zu-Bild-Workflow untersucht; kein paralleles neues Generierungssystem |
| Workflows 02 Inpainting, 03 Hochskalieren, 04 ControlNet | Vorhanden, für den ersten Standansichten-Test nicht eingesetzt |
| Workflow 05 Bild zu 3D | Bewusst nicht eingesetzt; keine 3D-Pipeline gebaut |
| IPAdapter PLUS SDXL + CLIP-ViT-H | Vorhandene Referenzbindung im lokalen Praxistest verwendet |
| SDXL Base 1.0 | Bereits für die früheren Naturstudien verwendet, hier wiederverwendet |
| `build_fox_comparison.py` | Freigegebene Referenz, Weltmaß 10×6 und Anker 5/5 als Ausgangspunkt |
| `build_animal_proof.py`, `build_landscape_detail.py` | Vorhandene Bodenproben und Zoom-Prüfprinzip übernommen |
| Pillow, NumPy, SciPy in ComfyUI-venv | Vorhandene Bildverarbeitung genutzt; keine zusätzliche Installation |

ComfyUI liegt lokal unter
`L:/Comfy-Desktop/ComfyUI-Installs/ComfyUI/ComfyUI`;
gemeinsame Modelle unter `L:/Comfy-Desktop/ComfyUI-Shared/models`.
Das API meldete ausdrücklich `NVIDIA GeForce RTX 3070 Ti`, 8 GB.
Ein gesondertes ausführbares Programm namens Günther wurde in den untersuchten
Projekt-/Workflow-Verzeichnissen nicht gefunden. Wiederverwendet wurde deshalb
die nachweisbar vorhandene lokale ComfyUI-Infrastruktur, ohne eine unbekannte
Günther-Schnittstelle zu behaupten.

Für den Test wurde ComfyUI an `127.0.0.1:8188` mit `--lowvram` gestartet.
Es wurden keine Modelle heruntergeladen und keine bestehenden Workflows
überschrieben. Ein nicht verwendetes Swarm-Animationsmodul meldete beim Start
eine fehlende optionale `imageio_ffmpeg`-Abhängigkeit; die vier Bildaufträge
liefen trotzdem erfolgreich. Diese fremde Installation wurde nicht verändert.

Unterschied der Aufbereitung: Die alten Zeichenskripte bleiben unverändert.
Der neue Agent übernimmt erzeugte Bilder, stellt sie frei und exportiert LODs,
statt Tiere automatisch durch die alten parametrischen Konturen zu ersetzen.
Dadurch bleibt ein Fehlschlag der lokalen Bilderzeugung im Vergleich sichtbar.
