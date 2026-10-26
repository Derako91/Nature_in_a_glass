# Feuerpaket – Ergänzung 26.10.2026

Alle vier Gruppen aus Gestaltung Abschnitt 8/E liegen jetzt als **Prüfentwürfe**
vor, jeweils vier Phasen à 200 ms. Keine Freigabe, keine Spielintegration.

| Gruppe | PNG-Abmessung | Weltmaß | Bezugspunkt | Prüfblatt |
|---|---|---|---|---|
| Kleine Flamme | 40×64 | 10×16 | 20/62 | [klein](comparison.png) |
| Große Flamme | 64×112 | 16×28 | 32/110 | [groß](../fire-large-review/comparison.png) |
| Glutsaum | 256×128 | Bodenraute 64×32 | Mitte 128/64 | [Glut](../fire-ground-review/comparison.png) |
| Rauch | 96×128 | 24×32 | 48/126 | [Rauch](../smoke-review/comparison.png) |

Jeder Ordner enthält vier RGBA-Dateien, Manifest, `validation.json`, den
verwendeten `prompt.txt`, Kontaktbogen und `animation.gif`. Alle Vorschauen
sind synthetisch. Die bestehenden kleinen Flammen bleiben unverändert.

## Technische Prüfung und visuelle Grenzen

- Gemeinsamer Skalierungsfaktor je Folge, festes Basisniveau, transparente Ränder.
- Integrierte alpha-gewichtete Luma: großes Feuer 5,15 % Schwankung,
  Glutsaum 13,47 %. Rauch zunächst 32,83 %, durch Absenkung der Deckkraft
  dichterer Frames auf 0,25 % reduziert. Faktoren im Bericht dokumentiert.
  Dies misst Gesamtlicht, nicht die Wahrnehmung lokaler bewegter Flammen.
- Rauch maximal etwa 55 % Deckkraft; dichterer Hintergrund bleibt sichtbar.
- Glutsaum wird auf die 2:1-Bodenraute begrenzt, Mitte bleibt transparent.
- Rauch ist weicher/volumetrischer als die Flammen und bereits leicht rechts
  geneigt. **Noch nicht zur Übernahme empfohlen:** Stil und Zusammenspiel mit
  Windneigung prüfen, gegebenenfalls neutraler und kantiger neu gestalten.
- Feuer und Glut sind als Entwürfe brauchbar, aber noch nicht im Spiel geprüft.
  Übergang letzter/erster Frame, Lesbarkeit bei Zoom 4/8 und Überlagerung von
  Vegetation müssen im Spiel beurteilt werden. Keine Fernfassungen angefertigt.

## An Claude

Bitte bestehende Rückfrage in Gestaltung 16.7 zur Einbettung/Animation beantworten.
`fire_ground` ist eine Bodenüberlagerung, kein stehendes Sprite; sein Mittelpunkt
ist kein Fußpunkt. Rauch bekommt Windneigung durch den Renderer erst nach
Abnahme einer geeigneten neutralen Vorlage. Die Manifest-Zeitangaben sind
Produktionsmetadaten und nicht bereits von der Engine unterstützt.

## Erzeugung

Je Gruppe ein eigener eingebauter image_gen-Auftrag mit vier zusammengehörigen
Phasen. Technische Aufbereitung über das erweiterte vorhandene Skript:

```text
python tools/nature/build_fire_review.py <Quellbild> --kind large
python tools/nature/build_fire_review.py <Quellbild> --kind ground
python tools/nature/build_fire_review.py <Quellbild> --kind smoke
```

Vorhandene Ziele werden nicht überschrieben. Rohausgaben bleiben lokal in
`runs/fire-large-source`, `runs/fire-ground-source`, `runs/smoke-source`.
Quellhashes stehen in den jeweiligen Berichten. Keine neue Infrastruktur,
keine Änderungen an freigegebenen Assets, Engine oder Simulation.
