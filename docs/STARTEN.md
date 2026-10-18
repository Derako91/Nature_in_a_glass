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
| Feld untersuchen | Auf ein Feld klicken → Kasten unten links |
| Damm / Durchbruch | Im Feld-Kasten: *Damm +2 m* oder *Durchbruch −2 m* |
| Ansicht wechseln | **Isometrisch** oder **Karte** |
| Ebene wechseln | Natur, Höhe, Bodenfeuchte, Wasser, Brandgefahr, Boden/Abtrag, Tiere |
| Wichtige Ereignisse | *Nur Chronik*, *Anhalten* oder *Anhalten und hinfahren* (Brand, Dürre, Waldsterben, Keimung …) |
| Ereignisort finden | Eintrag in der *Chronik* anklicken |
| Ursachen eines Ereignisses | Maus über den Chronik-Eintrag halten |
| Neue Insel | Seed eintragen (oder 🎲) → **Neue Insel** |
| Speichern / Laden | Ordner `spielstaende` neben dem Programm |

Tag und Nacht sind nur bei langsamem Zeitraffer sichtbar, sonst würde das Bild flackern.

## Eingriffe

Rechts unter *Eingriffe* veränderst du **Bedingungen**: Jahresregen, Jahresmitteltemperatur, wie leicht das Grundwasser an den Quellen austritt, und du kannst einzelne Baum- oder Buschsamen fallen lassen. Was daraus folgt – Dürre, versiegende Bäche, sterbender oder wachsender Wald – entscheidet die Simulation. Jeder Eingriff steht in der Chronik.

Zum Quellregler: Das Wasser der Quellen stammt aus dem Grundwasser der Insel. Ein hoher Wert lässt es leichter austreten – aber mehr, als nachsickert, kommt auf Dauer nicht heraus. Bei 300 % leert sich das Grundwasser, und die Quelle wird wieder schwächer.

Zu den Samen: Ein Samen landet an einem zufälligen Ort, den niemand kennt. Keimt er, steht in der Chronik, wo. Etwa drei Jahre bleibt er keimfähig.

Ideen zum Ausprobieren:

- Eine Quelle auf 0 % stellen und beobachten, wie der Bach darunter austrocknet.
- Einen Bach mit ein paar Klicks auf *Damm +2 m* aufstauen und zusehen, wie sich das Wasser einen anderen Weg sucht – dann mit *Durchbruch* den Stau ablaufen lassen. Am besten auf der Ebene *Wasser*.
- Den Jahresregen auf 300 mm senken und 20 Jahre laufen lassen. Danach wieder auf 950 mm stellen: Der Wald kehrt nur langsam und nur von seinen Rändern zurück.
- Zwei Inseln mit demselben Seed nebeneinander laufen lassen – sie entwickeln sich exakt gleich.
- „Paradies, dann alles aus“: Regen hoch, 150 Jahre warten, dann Regen und Quellen auf 0. Der Wald stirbt über Monate, Totholz bleibt stehen – und wenn danach ein Gewitter kommt, brennt es.
- Den Wald aussterben lassen, dann einen Baumsamen fallen lassen und warten, ob irgendwo ein neuer Wald entsteht.
- **Tiere aussetzen** (unter *Eingriffe*): Hirsche, Kaninchen, Wölfe oder Füchse kommen auf das gewählte Feld oder, ohne Auswahl, an eine zufällige Stelle. Die Insel ist anfangs tierfrei. Zuerst Hirsche aussetzen und 10–15 Jahre warten, bis es viele sind – dann ein Wolfsrudel. Wölfe brauchen viele Hirsche, Füchse viele Kaninchen. Die Punkte auf der Karte sind vorläufig, bis die Tierbilder eingebunden sind; bei schneller Zeit werden sie ausgeblendet, auf der Ebene *Tiere* bleiben sie sichtbar.
- Hirsche ohne Wölfe vermehren sich, bis Gras und Busch knapp werden – dann folgen Hungerjahre. Schau auf den Busch-Anteil und darauf, ob junger Wald noch hochkommt.
- Ebene *Boden/Abtrag*: Rot verliert Boden, Blaugrün sammelt ihn, hell ist nackter Fels, dunkelbraun tiefe Erde. Einen Waldbrand abwarten und dann beim nächsten Starkregen hinschauen: Auf der Brandfläche spült es den Boden ab, bis das Gras zurück ist.
- Mit 10 Jahren/s über Jahrtausende laufen lassen: Seen verlanden langsam, an Bachmündungen taucht Schwemmland aus dem Meer auf. Jeder See hat einen Namen – er steht im Feld-Kasten und in der Chronik.
- *Wichtige Ereignisse: Anhalten und hinfahren* einstellen und mit 10 Jahren/s laufen lassen – der Beobachter hält beim nächsten Brand an.

## Startoptionen (für Fortgeschrittene)

`nature-in-a-glass --seed 7 --years 100 --view karte --layer wasser`

- `--seed N` – welche Insel
- `--years N` – vor dem Öffnen N Jahre vorspulen
- `--view karte` – mit der 2D-Karte starten
- `--layer hoehe|feuchte|wasser|brand|boden|tiere` – Startebene
- `--flaechen` – mit Flächenfarben statt Grafik starten; `--zoom N` – Startzoom (0,4 bis 8)
- `--tiere hirsche,kaninchen,woelfe,fuechse` – nach `--years` Jahren diese Tiere aussetzen; `--danach N` lässt sie anschließend N Jahre leben
- `--speed 0…5` – sofort mit Zeitraffer starten (0 = 1 Std/s, 5 = 10 Jahre/s)
- `--days N` – zusätzlich N Tage vorspulen
- `--reaction 0|1|2` – bei wichtigen Ereignissen: nur Chronik, anhalten, anhalten und hinfahren
- `--select x,y` – ein Feld vorauswählen

## Ohne Fenster: `nig-sim`

Im selben Ordner liegt `nig-sim`, die Simulation ohne Grafik, z. B. für lange Läufe auf einem Server:

```
nig-sim --seed 42 --years 500 --map --events --save welt.json
```

Gibt eine Jahrestabelle, die Chronik und eine Textkarte aus. Mit `nig-sim --load welt.json` geht es weiter. Kopierst du die Datei als `spielstaende/welt.json` neben das Fenster-Programm, öffnet *Laden* genau diesen Stand – beide nutzen dasselbe Format.
