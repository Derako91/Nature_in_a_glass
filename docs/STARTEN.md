# Nature in a Glass starten

## Herunterladen

1. Auf GitHub unter **Releases** die neueste Version öffnen.
2. Die passende ZIP-Datei laden:
   - Windows: `nature-in-a-glass-windows.zip`
   - macOS: `nature-in-a-glass-macos.zip`
   - Linux: `nature-in-a-glass-linux.zip`
3. Entpacken und **`nature-in-a-glass`** starten.

Das Programm ist (noch) nicht signiert. Beim ersten Start fragt das Betriebssystem nach:

- **Windows:** „Der Computer wurde durch Windows geschützt“ → *Weitere Informationen* → *Trotzdem ausführen*.
- **macOS:** Rechtsklick auf die Datei → *Öffnen* → *Öffnen*. Falls das nicht reicht: Systemeinstellungen → Datenschutz & Sicherheit → *Dennoch öffnen*.

## Bedienung

| Aktion | So geht's |
| --- | --- |
| Zeit laufen lassen / anhalten | **▶ Weiter** bzw. **⏸ Pause** oder Leertaste |
| Zeitraffer | 1 Std/s bis 10 Jahre/s in der oberen Leiste |
| Einen Tag weiter | **+1 Tag** |
| Karte verschieben | Mit gedrückter Maustaste ziehen |
| Zoomen | Mausrad |
| Feld untersuchen | Auf ein Feld klicken → rechts unter *Feld* |
| Ansicht wechseln | **Isometrisch** oder **Karte** |
| Ebene wechseln | Natur, Höhe, Bodenfeuchte, Wasser |
| Ereignisort finden | Eintrag in der *Chronik* anklicken |
| Ursachen eines Ereignisses | Maus über den Chronik-Eintrag halten |
| Neue Insel | Seed eintragen (oder 🎲) → **Neue Insel** |
| Speichern / Laden | Ordner `spielstaende` neben dem Programm |

Tag und Nacht sind nur bei langsamem Zeitraffer sichtbar, sonst würde das Bild flackern.

## Eingriffe

Rechts unter *Eingriffe* veränderst du **Bedingungen**: Jahresregen, Jahresmitteltemperatur und die Schüttung der Quellen. Was daraus folgt – Dürre, versiegende Bäche, sterbender oder wachsender Wald – entscheidet die Simulation. Jeder Eingriff steht in der Chronik.

Ideen zum Ausprobieren:

- Eine Quelle auf 0 % stellen und beobachten, wie der Bach darunter austrocknet.
- Den Jahresregen auf 300 mm senken und 20 Jahre laufen lassen. Danach wieder auf 950 mm stellen: Der Wald kehrt nur langsam und nur von seinen Rändern zurück.
- Zwei Inseln mit demselben Seed nebeneinander laufen lassen – sie entwickeln sich exakt gleich.

## Startoptionen (für Fortgeschrittene)

`nature-in-a-glass --seed 7 --years 100 --view karte --layer wasser`

- `--seed N` – welche Insel
- `--years N` – vor dem Öffnen N Jahre vorspulen
- `--view karte` – mit der 2D-Karte starten
- `--layer hoehe|feuchte|wasser` – Startebene
- `--speed 0…5` – sofort mit Zeitraffer starten (0 = 1 Std/s, 5 = 10 Jahre/s)

## Ohne Fenster: `nig-sim`

Im selben Ordner liegt `nig-sim`, die Simulation ohne Grafik, z. B. für lange Läufe auf einem Server:

```
nig-sim --seed 42 --years 500 --map --events --save welt.json
```

Gibt eine Jahrestabelle, die Chronik und eine Textkarte aus. Mit `nig-sim --load welt.json` geht es weiter. Kopierst du die Datei als `spielstaende/welt.json` neben das Fenster-Programm, öffnet *Laden* genau diesen Stand – beide nutzen dasselbe Format.
