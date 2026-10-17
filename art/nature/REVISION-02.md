# Prüfblatt 02 – Richtung Siedler I & II

Die Stilkorrektur vom 17.10.2026 ist in `docs/gestaltung.md`, Abschnitt 1.1,
festgehalten. Das aktuelle Prüfblatt liegt weiterhin unter
`assets/nature/_testsheet.png`, der direkte Vergleich unter
`assets/nature/_comparison-v2.png`. Die erste Fassung bleibt vollständig in
`art/nature/proof-01/` erhalten.

Vier Vegetationsvorlagen wurden lokal mit ComfyUI und SDXL neu erzeugt.
Die Bildvorlagen aus eigenen Motiven der ersten Runde und die API-Workflows liegen in `comfyui-v2/`.
Die Bildvariation verwendet 512×768 Pixel, 32 Schritte, CFG 7, DPM++ 2M/Karras,
Denoise 0,52 und Seeds 2026101800–2026101803. Zwei ungeeignete Variationen
mit abstrakten Formvorlagen wurden nach Sichtprüfung verworfen: Die erste war
zu flach, die zweite zeigte teils einzelne Blätter statt Bäume. Beide liegen
zur Nachvollziehbarkeit unter `comfyui-v2-flat-rejected/` und
`comfyui-v2-leaf-rejected/` lokal vor; diese verworfenen Zwischenstände werden
nicht ins Repository hochgeladen.

Die Materialvorlagen und der abgestorbene Stamm stammen aus der ersten
ComfyUI-Runde; deren originale Workflows und Protokolle stehen unter `comfyui/`.
Die neue Verarbeitung verwendet wärmere Lichtfarben und kompaktere Kronen.
Alle Sprites behalten ihre vorgegebenen Dateimaße und Fußpunkte. Die eigentliche
Motivhöhe darf kleiner als die Bildhöhe sein; oben bleibt dann Transparenz.

Erneute Erstellung mit der vorhandenen ComfyUI-Python-Umgebung:

```powershell
python tools/nature/revise_sources.py
python tools/nature/build_proof_v2.py
```

Der erste Befehl erzeugt die vier Motive neu und überschreibt die Vorlagen der
zweiten Runde. Der zweite benötigt Pillow, NumPy und SciPy und baut die aktuellen
Assets, Prüfblätter und den Prüfbericht aus den gespeicherten Vorlagen neu.
Die bisherigen Befehle aus `README.md` bauen weiterhin Fassung 01.

Die Insel nutzt dieselben Testbelegungen wie Fassung 01. Es wurden keine
Spielmechanik, Gebäude, Figuren oder zusätzlichen Dekorationen hinzugefügt.
Animationen und endgültige Varianten bleiben der nächsten Arbeitsstufe nach
gestalterischer Abnahme vorbehalten.

Visuelle Referenz: [historischer offizieller Siedler-II-Screenshot, archiviert bei MobyGames](https://www.mobygames.com/game/settlers-ii-veni-vidi-vici/promo/promoImageId%2C99359/).
Die Referenz wurde nur zur Einordnung betrachtet und nicht als Bildinput
oder als Quelle einzelner Assets verwendet.
