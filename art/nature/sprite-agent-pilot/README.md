# Sprite-Agent: erster lokaler Praxistest

18.10.2026, Basis `7b3bd88`, RTX 3070 Ti (8 GB), lokales ComfyUI.
**Technischer Durchlauf bestanden. Gestalterische Reproduktion nicht bestanden.**
Das Paket ist nicht freigegeben, nicht im Spiel eingebaut und kein Ersatz
für den von Dennis gewählten Fuchs B.

## Umfang und Ergebnis

Ein natürlicher Auftrag wurde in einen konkreten Fuchs-Auftrag übersetzt:
vier Standansichten (`se/sw/ne/nw`), eine Fellvariante, ein Frame je Ansicht.
Vier echte SDXL-Generierungen, je 24 Schritte auf 768×768, IP-Adapter PLUS mit
der freigegebenen Fuchsreferenz, Referenzgewicht 0,7. Danach automatischer
Export von **16 RGBA-PNGs**: je Ansicht 10×6, 20×12, 40×24 und 80×48.
Das ist das vollständige Paket dieses begrenzten Standansichten-Auftrags,
**kein** vollständiger Fuchs-Animationssatz.

- `identity.png`: Referenz und vier lokale Ergebnisse vergrößert.
- `contact-01.png`: auf Gras, Buschboden und Waldboden bei Zoom 4/8/16/32.
- `review.html`: lokale Sichtprüfung und Links auf Auftrag/Manifest.
- `manifest.json`: konkreter Vorschlag für Claude, mit LOD-Weltmaßen und Ankern.
- `validation.json`: technische Prüfungen, kleine Silhouetten und Prüfhinweise.
- `extraction.json`: automatisch entfernte Bildrandreste und Warnungen.
- `workflows/`: tatsächlich ausgeführte API-Graphen und Historien, nicht nur
  nachträglich rekonstruierte Prompts.

## Sichtbefund (Codex, keine Freigabe von Dennis)

1. **Richtungen falsch:** Auch die als `sw`, `ne` und `nw` angeforderten Bilder
   zeigen überwiegend einen nach rechts orientierten Fuchs. Die Dateinamen
   dokumentieren den Auftrag, nicht eine bestätigte korrekte Blickrichtung.
2. **Identität driftet:** Beine, Rücken, Kopf und Schwanz ändern ihre Proportionen.
   Die erste Ansicht wirkt hochbeiniger und stärker wie vereinfachte Pixelgrafik.
   Der plastische, geduckte Körper der Stil-B-Referenz wird nicht gleichwertig gehalten.
3. **Freistellung begrenzt:** Der zweite Entwurf enthält einen hellen Saum und
   einen mitgenerierten Schatten. Randbasierte Freistellung beseitigt äußere
   Hintergrundinseln, kann diesen Saum aber nicht sicher von hellem Fell trennen.
4. **LOD ist kein Ersatz für Gestaltung:** Geglättete kleine Fassungen sind
   vorhanden. Bei 10×6 bleiben naturgemäß nur wenige Pixel; die richtige kleine
   Silhouette muss weiterhin geprüft bzw. eigens gestaltet werden.

Nach diesen vier Generierungen wurde der Bildversuch beendet. Keine zusätzlichen
Modelle installiert, keine weiteren Varianten produziert, keine Qualität
schöngeredet. Nächster sinnvoller Bildversuch wäre explizite Pose-/Richtungsführung
mit passenden Vorlagen; das ist ein gezielter Folgeauftrag und keine unendliche
Seed-Suche. Eine konsistente Animationsproduktion ist mit diesem Setup noch
nicht belegt.

## Reproduzierbarkeit und Ablage

Quelle: `art/nature/fox-ab/fox-b-imagegen-source.png` (keine zweite Kopie im Git).
Rohbilder, Referenz-Zwischenbild und vollständiges Lauf-ZIP bleiben entsprechend
Gestaltung 9.1 lokal unter `runs/sprite-agent/fox-20261018-223020-df2872`.
Das ZIP liegt daneben. Hier sind nur die Prüfexpor­te, Berichte und Workflows
versioniert. Die vier Seeds stehen im Journal. GPU-/Bibliotheksversionen können
die exakten Pixel einer erneuten Generierung verändern.

Das JSON-Profil und die Pipeline sind unter `tools/sprite_agent/` dokumentiert.
Für neue Aufträge kann der normale Ablauf ohne weitere Codex-Einzelarbeit laufen;
die menschliche Auswahl und stilistische Freigabe bleiben absichtlich notwendig.
Der später ergänzte Engine-Adapter folgt Claudes inzwischen veröffentlichter
Schnittstelle in Gestaltung 16.6 / v0.7.2. Er kann Hauptbild + `@fern` für `--bilder`
ausgeben; die Bilder bleiben trotz passendem Dateiformat gestalterisch ungeeignet.
Details und offene Sichtprüfung: `docs/sprite-package-contract.md`.
