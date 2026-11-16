# ADR 0009 – Lagerplätze entstehen aus dem Bleiben

**Status:** angenommen (2026-11-16)

## Kontext

Nach v0.7 ziehen Gruppen von 20–35 Menschen über die Insel, schlafen aber jede Nacht unter freiem Himmel. Die Roadmap sieht als Nächstes Sesshaftigkeit vor – ohne Bauaufträge des Beobachters (Prinzip 1), mit Ursachen (Prinzip 2) und mit Rückwirkung auf die Natur.

## Entscheidung

- **Anlass statt Auftrag:** Wer 25 Nächte in derselben Gegend (Umkreis 3 Felder) verbringt und dort an verlässlichem Wasser steht (Verlässlichkeit ≥ 0,6), legt mit etwas Zufall ein Lager an. Mindestabstand zwischen bewohnten Lagern: 5 Felder. Ein verlassenes Lager in der Nähe wird wieder bezogen statt ein neues angelegt.
- **Zugehörigkeit** ist ein Merkmal der Person (`Person.camp`). Wer am Lager übernachtet und noch keines hat, gehört dazu; wer 40 Nächte bei einem anderen Lager verbringt, zieht um; wer 60 Nächte weit weg verbringt, kehrt nicht zurück. Kinder erben das Lager der Mutter.
- **Verhalten:** Wer ein Lager hat, kehrt ohne Not dorthin zurück (statt zur Gruppenmitte) und sammelt lieber in seiner Nähe. Nachts schützen Windschutz (0,5), Hütte (0,85) und Feuer (+0,3) vor Kälte.
- **Holz aus dem Wald:** Freie Erwachsene am Lager holen Holz, solange Brennholzvorrat oder eine nötige Hütte fehlt. Eine Tageslast entspricht 0,4 % eines vollen Waldfeldes, Totholz zuerst, im Umkreis von 3 Feldern. Eine Hütte braucht 8 Lasten und bietet fünf Menschen Platz; höchstens vier Hütten je Lager. Feuer brennt täglich zum Kochen, bei unter 6 °C auch zum Wärmen.
- **Feuer kann überspringen:** Brennt das Lagerfeuer, kann es bei trockenen Nachbarfeldern und Wind einen Brand auslösen (Chronik: Ursache „Lagerfeuer“, wird wie ein Blitzbrand verfolgt).
- **Vorräte:** Satte Erwachsene mit vollem Beutel legen Überschuss ab, Hungrige nehmen; Vorräte verderben (1,5 % am Tag, bei Frost 0,5 %).
- **Verfall:** Ungenutzt verliert ein Lager 1/4 seines Zustands im Jahr. Nach zwei Jahren ohne Besuch ist es verlassen, bei Zustand 0 verschwindet es.
- **Technik:** `people::step_day` bekommt die Welt veränderbar, sammelt Holzschlag und Funken aber in `Effects` und wendet sie erst nach dem Tag an – die Entscheidungen des Tages sehen alle denselben Weltzustand. Zufall weiter über den Strom `menschen`. Spielstand-Format 9 (Felder mit `serde(default)`, Format 8 lädt ohne Umwandlung).

## Konsequenzen

- Auf wasserreichen Inseln entsteht meist im ersten Jahr nach der Landung ein Lager mit ein, zwei Hütten; der Wald im Umkreis lichtet sich über Jahrzehnte. Auf kargen Inseln findet sich oft kein Ort, an dem man lange genug bleiben kann.
- Brände durch Lagerfeuer sind selten (in Probeläufen etwa alle 10–20 Jahre je bewohntem Lager in trockenen Sommern) – ein neues, nachvollziehbares Risiko der Sesshaftigkeit.
- Darauf bauen auf: Dörfer aus mehreren Lagern, Werkzeuge, Fischfang, Anbau, eigene Rollen beim Bauen.
