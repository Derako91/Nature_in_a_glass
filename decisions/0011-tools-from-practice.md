# ADR 0011 – Werkzeuge entstehen aus dem Tun

**Status:** angenommen (2026-11-16)

## Kontext

Fischen und Gärtnern sind seit v0.9/v0.10 Können, das mit Übung wächst. Für den Weg vom Lager zum Dorf fehlen Erfindungen, die einen Sprung bringen und im Dorf weitergegeben werden (Prinzip 6), ohne dass der Beobachter sie verordnet (Prinzip 1).

## Entscheidung

- **Drei Werkzeuge** als Wissen je Person (Bits `Person.tools`): **Netz** (Fisch × 1,6), **Korb** (Gesammeltes × 1,3, tragen +1,5 Rationen), **Steinbeil** (zwei Lasten Holz am Tag).
- **Erfindung aus dem Tun**, nur Erwachsene, mit Zufall je Tag: Netz beim Fischen mit Können ≥ 0,5 (0,3 %), Korb beim Sammeln mit vollem Beutel (0,04 %), Steinbeil beim Holzholen (0,3 %).
- **Weitergabe** durch Abschauen bei Kundigen am selben Ort (1 % je Tag, Kinder ab sechs 2 %).
- **Verlust:** Kann es niemand mehr, meldet die Chronik das; eine spätere Neuerfindung heißt „wiederentdeckt“.
- **Umzug ganzer Lager** (gleiche Version): Wasserstelle 60 Tage trocken, oder 180 Tage ohne Holz bei weniger als 16 Zugehörigen → mit 2 % je Tag ziehen alle zur besten verlässlichen Wasserstelle, die jemand im Lager kennt (5–20 Felder). Das alte Lager wird aufgegeben.
- **Durst (Fehler aus dem Langlauf):** Flache Teiche gelten als weniger verlässlich (0,5 + Tiefe/3 m), Ausgetrocknetes in Sichtweite wird gestrichen, sehr Durstige gehen zur nächsten statt zur besten Stelle.
- Spielstand-Format 14.

## Konsequenzen

- Langlauf 16 Inseln × 120 Jahre: mit Werkzeugen überleben 13 Gruppen (rund 410 Menschen), ohne 9 (rund 250). Vor der Durstkorrektur schadeten die Werkzeuge: Größere Gruppen zogen weiter an die Küste und verdursteten in Dürren an ausgetrockneten Teichen.
- Darauf bauen auf: weitere Werkzeuge (Boot, Tongefäß, Speerschleuder), Spezialisierung im Dorf.

## Nachtrag v0.14.0 – Jagd

- **Wer jagt:** Erwachsene, gesund (Verletzung < 0,2, Gesundheit > 0,6), nicht durstig, mit Hunger über 0,25 oder fast leerem Beutel, nicht auf dem Aufbruch.
- **Hirsch** im Umkreis von 2 Feldern: Alle Bereiten am selben Ort oder daneben jagen gemeinsam; Erfolg je Tag 3 % × Jäger × (0,5 + mittleres Können), höchstens 40 %; Verletzungsgefahr 0,1 % je Jagdtag. Ertrag 30 Rationen: erst in die Beutel der Jäger, dann in den Vorrat ihres Lagers (bis 8 Felder entfernt), der Rest bleibt als Kadaver liegen.
- **Kaninchen** auf dem eigenen Feld (ab einem Tier): allein, 8 % × (0,5 + Können), eine Ration.
- **Können** `hunting` wächst mit jedem Versuch (mehr bei Erfolg) und wird wie Fischen abgeschaut; wer mit dem Boot kommt, kann so viel jagen wie fischen × 0,8.
- Aussterben durch Jagd wird am selben Tag erkannt und mit der Zahl der Erlegten in der Chronik begründet (`Animals.hunted`, `hunted_since`). Spielstand-Format 15.

## Nachtrag v0.15.0 – Tongefäß und Rollen

- **Tongefäß** (Bit 8): Erfindung mit 0,05 % je Tag für Erwachsene am eigenen Lager (Umkreis 1), das einen Garten und mindestens 50 Rationen Vorrat hat, an Wasser. Wirkung: Verderb des Lagervorrats × 0,5, wenn jemand im Lager es kann.
- **Rollen** (`Person.role`): monatlich je Lager für Fischer, Gärtner, Jäger (in dieser Reihenfolge) die erwachsene Person mit dem höchsten Können ab 0,7, die noch keine Rolle hat. Wirkung: Fischer × 1,2 Fang; Gärtnerin am Lager: Wachstum × 1,2; Jäger jagen ab Hunger 0,1 statt 0,25. Spielstand-Format 16.

## Nachtrag v0.16.0 – Einbaum

- **Einbaum** (Bit 16): Erfindung mit 0,2 % je Fischtag für Erwachsene mit Fischen ≥ 0,7, die Netz und Steinbeil kennen, an der Küste.
- Wirkung an Küstenfeldern: Fischgrund + 1,5 Rationen, doppelter Bestand vor Erschöpfung.
- Gefahr: an Bootstagen mit Wind über 0,6 Kentern mit bis zu 1 % (linear bis Wind 1,0) – Todesursache „beim Fischen auf dem Meer ertrunken“.
- Bild: `PersonSprite.tool = "boat"` beim Fischen an der Küste (vor `net`).
- Langlauf 32 Inseln: Überleben gleich mit und ohne Einbaum.
