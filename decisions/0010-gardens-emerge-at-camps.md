# ADR 0010 – Gärten entstehen am Lager

**Status:** angenommen (2026-11-16)

## Kontext

Mit Lagern und Fischfang (v0.8, v0.9) bleiben Gruppen auf etwa der Hälfte der Inseln über Generationen. Für den Schritt vom Lager zum Dorf braucht es eine Nahrung, die man anbauen und lagern kann – ohne Bauauftrag des Beobachters (Prinzip 1) und mit Rückwirkung auf die Natur.

## Entscheidung

- **Entdeckung mit Zufall:** An einem Lager, das mindestens ein Jahr besteht, kann im Frühjahr (Tag 80–140) eine freie erwachsene Person zum ersten Mal Knollen und Samen in den Boden stecken (0,2 % je Tag ohne Vorkenntnis, 1–3 % mit Können). Ein Garten je vier Menschen am Lager, höchstens sechs.
- **Ort:** bestes Feld im Umkreis von 2 um das Lager (feuchter, tiefer Boden, kein Wasser, kein Lager). Es wird gerodet (Wald und Totholz weg, Busch und Gras stark zurück) und bei jeder Aussaat wieder freigemacht.
- **Jahreslauf:** Aussaat im Frühjahr, Wachstum bis Tag 240 nach Bodenfeuchte, Wärme (über 6 °C) und Pflege (Jäten alle paar Tage durch freie Erwachsene am Lager; ohne Pflege nur 40 %), Ernte von Tag 240 bis 320 in den Lagervorrat (bis 8 Rationen je Erwachsenem und Tag). Volle Ernte 150 Rationen × (0,4 + 0,6 × Können). Was nicht geerntet wird, verdirbt.
- **Können** `farming` 0..1 wie beim Fischen: Übung bei der Gartenarbeit, Abschauen bei Geübteren (ADR 0007, Nachtrag v0.9).
- **Vorräte** halten jetzt länger (1 % Verlust am Tag, bei Frost 0,4 %), das Lager fasst bis 150 Rationen.
- **Verwildern:** zwei Jahre ohne Pflege (oder ein Jahr, wenn das Lager verlassen ist) – der Garten verschwindet, die Natur wächst wieder.
- **Holz:** Ist im Umkreis von 3 Feldern kein Holz mehr, holen die Menschen es aus bis zu 6 Feldern Entfernung (Befund aus Fables Probelauf auf v0.8.9).
- Spielstand-Format 11.

## Konsequenzen

- Langlauf (8 Inseln, ein Boot, 120 Jahre): 6 von 8 Gruppen überleben, meist mit 15–49 Menschen; der erste Garten entsteht 2–13 Jahre nach der Landung, später 4–10 Gärten je Insel. Dürrejahre bringen schlechte Ernten.
- Darauf bauen auf: Dörfer aus mehreren Lagern, Werkzeuge, Vorratsgruben, Haustiere.
