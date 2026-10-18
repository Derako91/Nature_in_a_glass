# ADR 0007 – Menschen: Bedürfnisse, lokales Wissen, keine Steuerung

**Status:** angenommen (2026-10-18)

## Kontext

v0.6 bringt die ersten Bewohner. Die Prinzipien verlangen: keine Befehle (1), Bedürfnisse treiben Handlungen (5), Wissen ist lokal (6), glaubwürdige Folgen bis zum Tod (10). Gleichzeitig muss die Simulation deterministisch, speicherbar und schnell genug für lange Läufe bleiben.

## Entscheidung

- **Ankunft nur durch den Beobachter:** Eingriff `PeopleArrive` (Boot mit fünf Menschen an der Küste), steht in der Chronik. Sonst gibt es keinen Eingriff, der Menschen betrifft.
- **Tagesschritt pro Person** (wie die Tiere), keine Stunden-Simulation. Die Darstellung leitet aus der Tätigkeit des Tages ab, was um welche Uhrzeit zu sehen ist (`appearance::person_state_at`).
- **Bedürfnisse als Zahlen 0…1** (Hunger, Durst, Müdigkeit, Gesundheit, Verletzung). Das dringendste bestimmt das Ziel: Feuer fliehen > Wasser > Nahrung (nahe verlässlichem Wasser) > bei Verletzten bleiben > bei der Gruppe bleiben > erkunden.
- **Lokales Wissen:** Jeder merkt sich höchstens 8 Wasserstellen (gebündelt je Gewässerstück, mit Verlässlichkeit), 8 Sammelstellen und 4 Küstenstellen – nur, was er selbst gesehen hat. Trockene Stellen werden erst vergessen, wenn man davorsteht. Irrtümer sind gewollt.
- **Wege:** Dijkstra über die Nachbarfelder; Stufen über 25 m sind unpassierbar, steile Abstiege kosten mehr und bergen Sturzgefahr. Normal wird im Umkreis von gut einem Tagesmarsch geplant, durstig oder hungrig weiter. Zufriedene Menschen (satt, Wasser hier, bei der Gruppe) planen gar nicht.
- **Gemeinschaft ohne Drehbuch:** Gefährten auf demselben Feld bringen Verletzten Wasser und teilen Vorräte; Gesunde gehen zu Verletzten. Keine Rollen, keine Anführer.
- **Tod ist endgültig:** Tote kommen mit Biografie nach `departed` und bleiben gespeichert; der Leichnam liegt 12 Tage. Nachwuchs gibt es in v0.6 nicht.
- **Zufall** über den eigenen Strom `menschen`. Spielstand-Format 7.

## Konsequenzen

- Auf gewöhnlichen Inseln leben die ersten fünf meist 20–30 Jahre und sterben an Altersschwäche; auf kargen Inseln (wenig verlässliches Wasser) oder in Dürren sterben einige früh. Das ist gewollt und erzählt etwas über die Insel.
- Kosten: mit fünf Menschen etwa halb so viele Jahre pro Sekunde wie ohne. Größere Gruppen brauchen später weitere Abkürzungen (z. B. gemeinsame Wegsuche je Feld).
- Jagd, Fischfang mit Werkzeug, Feuer machen, Unterkünfte, Nachwuchs, Persönlichkeit und Wissensweitergabe bauen auf dieser Grundlage auf.

## Nachtrag v0.7.0 – Nachwuchs

- Paare bilden sich zufällig zwischen erwachsenen, unverbundenen Menschen, die nah beieinander sind (keine Eltern, Kinder, Geschwister oder Halbgeschwister) – keine festen Rollen (Prinzip 4).
- Empfängnis hängt von Ernährung und Gesundheit ab; das begrenzt das Wachstum über die Nahrung der Insel. Stillzeit zwei Jahre.
- Kinder unter 14 entscheiden nicht selbst, sondern ziehen mit ihrer Betreuerin (Mutter, sonst Vater, sonst nächster Erwachsener). Das spart Rechenzeit und verhindert, dass Kinder allein verhungern. Sie übernehmen das Wissen der Betreuerin über Wasser- und Küstenstellen (Prinzip 6).
- Kleine Gruppen sind empfindlich: Ohne Partner außerhalb der eigenen Familie endet eine Linie. Das ist gewollt; neue Boote bringen neue Menschen.

