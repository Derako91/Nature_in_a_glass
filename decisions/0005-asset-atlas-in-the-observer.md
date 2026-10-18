# ADR 0005 – Codex' Bilder als eingebetteter Atlas im Beobachter

**Status:** angenommen (2026-10-18, Freigabe Dennis)

## Kontext

Codex hat Boden-, Wand-, Wasser-, Pflanzen- und Tierbilder nach `docs/gestaltung.md` erstellt (`assets/nature/`). Der Beobachter zeichnete bisher nur Flächenfarben. Für die Einbindung waren zu klären: Wie kommen die Bilder in den Download, wie werden sie in der richtigen Reihenfolge gezeichnet, und wer ist wofür zuständig.

## Entscheidung

- **Eingebettet:** `crates/app/build.rs` bettet alle PNG aus `assets/nature/{ground,walls,water,vegetation,animals}` und die beiden Manifeste beim Bauen ins Programm ein. Prüfblätter (`_*.png`) und Entwürfe bleiben draußen. Der Download braucht keinen zusätzlichen Ordner.
- **Ein Atlas:** `crates/app/src/atlas.rs` packt alle Bilder beim Start in eine einzige Textur (Regalpacken, 1 px Abstand, `NEAREST`). Fußpunkte kommen aus den Manifesten, doppelte Tierbilder aus `frame_aliases`. Ein weißes Hilfsfeld erlaubt einfarbige Dreiecke (Bachlinien, Feuer) im selben Netz.
- **Ein Durchgang:** Die isometrische Ansicht legt pro Feld Wände → Boden → Überlagerungen → Feuer → Bachlinien → Pflanzen und Tiere (nach Fußpunkt) in ein Netz, von hinten nach vorne. Dadurch verdecken vordere Bäume hintere richtig.
- **Welche Bilder wohin:** ausschließlich über `nig_core::appearance` (Abschnitt 7 und 15 der Gestaltung). Der Renderer erfindet keine Regeln.
- **Umschalter „Grafik“** neben der Ebenenwahl; ohne Grafik bleiben die Flächenfarben. Die 2D-Karte und alle Analyse-Ebenen bleiben flächig (gestaltung.md, Abschnitt 3). Startoption `--flaechen`.
- **Zuständigkeit:** Diese technische Grundlage stammt von Claude. Wie die Bilder zusammengesetzt, getönt und animiert werden, gestaltet Codex weiter.

## Konsequenzen

- Neue Bilder von Codex erscheinen nach dem nächsten Bauen von selbst, wenn sie dem Namensschema folgen (`<id>_<variante>[_<zustand>][_f<frame>].png`).
- Fehlende Bilder (z. B. Kadaver, Spuren, Feuer, gestresste Bäume) fallen auf vorhandene Fassungen oder Flächenfarben zurück.
- Das Programm wird um die Größe der Bilder größer (derzeit wenige hundert Kilobyte).
