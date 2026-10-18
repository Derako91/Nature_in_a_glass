# Menschen – Vorbereitung 01

Beauftragt: eine männliche und eine weibliche erwachsene Grundfigur im bisherigen
Sprite-Stil. Zwei Kleidungsvarianten (grün und erdfarben), beide Blickrichtungen,
Stehen sowie Gehen, Tragen und eine neutrale Werkzeugbewegung. Beide Grundfiguren
erhalten dieselben Zustände. Keine neue Berufs- oder Simulationsregel.

Vorläufiger Maßstab: 8×12 Darstellungsbildpunkte, gespeichert als 16×24 Pixel;
Fußpunkt [8,22] in Bildauflösung. Damit niedriger als der Jungbaum (14 px).
Diese Arbeitsannahme ermöglicht die Vorbereitung; die Menschensimulation und
ihre endgültige Zoom-/Maßstabsentscheidung sind weiterhin offen.

Je Grundfigur und Kleidungsvariante: links/rechts, Stehen ein Bild, Gehen vier
Bilder zu 200 ms, Tragen vier Bilder zu 200 ms, Arbeiten vier Bilder zu 300 ms.
Insgesamt 104 Bildzuordnungen; gleiche Pixelinhalte werden über frame_aliases
referenziert. Das Manifest liegt separat unter `assets/nature/humans/`.
Dieser Ordner wird vom aktuellen Spielatlas nicht eingebettet.

Die Figuren sind direkt im bearbeitbaren Pixelprogramm aufgebaut; keine neue
ComfyUI-Generierung. Licht rechts oben wird nach der Kontur gesetzt und bleibt
bei beiden Richtungen unverändert. Maximal sechs Farben einschließlich Schatten,
Alpha 0/77/255, sRGB. Größe, Farbzahl, Transparenz und vier unterschiedliche
Bewegungsphasen werden beim Bau geprüft. Die Vorschau dauert 2,4 Sekunden;
unveränderte GIF-Zwischenbilder werden ohne Zeitverlust zusammengefasst.

Neu bauen: `python tools/nature/build_human_prototypes.py` in der bisherigen
Pillow/NumPy-Umgebung. Ansichten: `_humans-prototypes.png`, `_humans-motion.gif`
und `_humans-scale.png` unter `assets/nature/`. Die Figuren und der Maßstab wurden
auf den Prüfblättern betrachtet; eine Abnahme auf echten Landschaften, bei Nacht
und im Spiel steht aus. Es handelt sich um erste Grundfiguren, keine fertige
Bewohnerdarstellung. Kinder, Alterung, weitere Haut-/Haarvarianten und konkrete
Berufe sind nicht Teil dieses ersten Pakets.
