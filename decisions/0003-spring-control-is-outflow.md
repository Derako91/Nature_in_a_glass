# ADR 0003 – Der Quellregler steuert den Austritt, nicht die Wassermenge

**Status:** angenommen (2026-10-17, Entscheidung von Dennis)

## Kontext

Beim Testen von v0.1.1 fiel auf, dass 300 % Quellleistung kaum sichtbare Wirkung hatte. Die Nachprüfung (`crates/core/tests/springs.rs`) zeigte: Der Kern rechnet richtig – das Wasser der Quellen stammt aus dem Grundwasser der Insel. Ein höherer Regler leert dieses Grundwasser schneller; sobald es knapp wird, fällt die Schüttung zurück. Mehr, als nachsickert, kann auf Dauer nicht herauskommen.

Zur Wahl standen:

- **A – Austritt:** Der Regler verändert, wie leicht das Grundwasser an der Quelle austritt. Die Wassermenge bestimmt die Geologie der Insel.
- **B – Zufuhr von außen:** Die Quelle wird zusätzlich aus einem Wasserleiter außerhalb der Insel gespeist; der Regler skaliert diese Zufuhr dauerhaft.

## Entscheidung

**A.** „Die Simulation entscheidet anhand der Geologie, woher das Wasser kommt. Du veränderst nur die Bedingungen.“ (Dennis)

## Konsequenzen

- Die Wasserbilanz der Insel bleibt geschlossen; es gibt keine Wasserquelle von außen.
- Der Regler erklärt sich in der Oberfläche selbst (Hinweis beim Darüberfahren).
- Die Grundwasser-Anzeige wird damit wichtiger. In v0.3 wird sie zu einem echten Füllstand (siehe Roadmap).
- Emergente Folge, schon in v0.2 beobachtbar: Je dichter der Wald, desto mehr Wasser verbraucht er und desto weniger erreicht das Grundwasser – die Quellen werden über Jahrhunderte schwächer.
