# ADR 0012 – Von einer Insel zur Welt, von der Steinzeit bis heute

**Status:** angenommen als Richtung (2026-11-16), Etappen werden einzeln entschieden

## Kontext

Dennis' Fernziel: Eine ganze Welt, die sich von einer einzigen Insel aus besiedelt, und eine Entwicklung von der Steinzeit bis zum heutigen Tag – ohne Befehle des Beobachters (Prinzip 1), mit Ursachen und bleibenden Folgen (Prinzip 2), mit Wissen, das weitergegeben oder vergessen wird (Prinzip 6).

Stand v0.18: eine Insel (wählbar bis 128 × 128 Felder, etwa 14 km²), Menschen als Einzelne mit Bedürfnissen und Lebensgeschichte, Lager, Gärten, Dörfer, fünf erfundene Werkzeuge. Schon 150 Menschen kosten etwa eine Sekunde Rechenzeit je Spieljahr.

## Entscheidung (Richtung)

1. **Welt:** Festland mit vorgelagerten Inseln, nicht nur ein Archipel – Städte, Handel und Wanderung brauchen Platz. Die Welt wird aus Regionen zusammengesetzt; jede Region ist ein Gelände wie heute die Insel. Die erste Besiedlung beginnt weiterhin mit einem Boot auf einer Insel.
2. **Detailstufen der Simulation (wie bei den Bildern, ADR 0008):**
   - **Nah:** wo der Beobachter hinschaut – einzelne Menschen, Tiere, Lebensgeschichten (heutiger Stand).
   - **Fern:** Siedlungen werden als Ganzes gerechnet – Bevölkerung nach Alter, Vorräte, Felder, Können, Werkzeuge, Beziehungen zu Nachbarn. Natur je Region in groben Mittelwerten.
   - **Übergänge** ohne Bruch: Wird eine ferne Siedlung angeschaut, entstehen aus ihren Zahlen wieder Einzelne (mit erfundenen, aber stimmigen Lebensgeschichten ab diesem Moment); wird sie verlassen, werden die Einzelnen wieder zusammengefasst. Die Chronik bleibt durchgehend.
   - Determinismus bleibt: gleiche Welt und Eingriffe → gleiche Geschichte, unabhängig davon, wohin der Beobachter schaut (die ferne Rechnung darf vom Blick nicht abhängen; die nahe ist eine Verfeinerung, keine andere Geschichte).
3. **Fortschritt aus Wissen, kein Technologiebaum zum Abarbeiten:** Jede Erfindung hat Voraussetzungen im Tun und in der Umwelt (Ton für Gefäße, Erz für Metall, Überschuss für Spezialisten), wird weitergegeben, kann verloren gehen und anderswo unabhängig neu entstehen. Große Schwellen – Ackerbau, Viehhaltung, Metall, Schrift, Staat, Industrie – sind Folgen von Bevölkerungsdichte, Überschuss und Austausch, nicht von Zeit.
4. **Zeit:** Jahrtausende Spielzeit müssen in vertretbarer Zeit laufen. Deshalb: ferne Rechnung in groben Schritten (Monate statt Tage), Nutzung mehrerer Prozessorkerne für unabhängige Regionen.
5. **Ausbreitung übers Meer:** Küsten am Horizont werden Wissen; seetüchtigere Boote entstehen aus dem Einbaum; Aufbrüche zu anderen Inseln folgen denselben Gründen wie heute der Lageraufbruch (Enge, Holz, Wasser). Pflanzen und Tiere breiten sich nicht selbst übers Meer aus, Menschen können sie mitnehmen.

## Etappen (jede für sich spielbar)

1. **Archipel:** mehrere Inseln in einer Welt; Eingriffe je Insel.
2. **Küsten sehen, übers Meer fahren:** Wissen um ferne Küsten, seetüchtiges Boot, Überfahrten mit Gefahren.
3. **Ferne Siedlungen:** zusammengefasste Rechnung und Übergänge Nah ↔ Fern; mehrere Kerne.
4. **Festland:** größere, aus Regionen zusammengesetzte Welt.
5. **Große Schwellen:** Ackerbau als Lebensgrundlage, Viehhaltung, Metall, Austausch zwischen Siedlungen …

## Konsequenzen

- Bevor Etappe 3 steht, bleiben Inseln klein genug, dass alle Menschen einzeln gerechnet werden können (Grenze heute etwa 150–200 Menschen bei flüssiger Darstellung).
- Neue Systeme werden so gebaut, dass sie später auch in zusammengefasster Form rechenbar sind (Können, Werkzeuge, Rollen, Vorräte sind schon Zahlen – das hilft).
- Codex' Bildwelt braucht für ferne Siedlungen Überblicksbilder (Dorf, Stadt, Felder als Fläche) – Auftrag folgt mit Etappe 3.
