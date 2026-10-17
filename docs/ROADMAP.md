# Roadmap – schrittweise, nicht als starre Fertigstellungstermine

**Ziel:** Die vorhandene Insel schrittweise zu einem lebendigen, sich selbst verändernden Ökosystem entwickeln. Die langfristige Vision bleibt eine offene Weltsimulation mit autonomen Bewohnern, Evolution und prozedural generierten Welten.

Jede Ausbaustufe soll bereits für sich interessant zu beobachten sein. Umfang und Reihenfolge dürfen sich nach den Ergebnissen der Simulationstests verändern.

## Unveränderte Grundprinzipien

- Der Simulationskern funktioniert ohne Fenster und ohne KI.
- Ereignisse benötigen Ursachen und haben bleibende Folgen.
- Spieler verändern Ausgangsbedingungen, nicht gewünschte Resultate.
- Die Welt darf scheitern, Arten verlieren und ihren ökologischen Zustand dauerhaft wechseln.
- Seeds, Zeitverläufe und Speicherstände bleiben reproduzierbar.
- Bestehende Speicherstände möglichst kompatibel halten; erforderliche Migrationen ausdrücklich dokumentieren.

Ausführlich: [PRINCIPLES.md](PRINCIPLES.md).

---

## ✅ v0.1 – Die Insel lebt ohne Menschen (veröffentlicht)

- [x] Eigenständiges Repository, Prinzipien, Architektur, [ADR 0002](../decisions/0002-runtime-and-rendering.md).
- [x] Tests und CI auf Windows, macOS, Linux; fertige Downloads unter Releases.
- [x] Insel mit Seed (64×64), isometrische Ansicht und 2D-Karte.
- [x] Zeitraffer, Speichern, Laden.
- [x] Wetter mit Jahreszeiten, Regen, Starkregen, Dürren.
- [x] Wasser: Versickern, Abfluss, Bäche, Seen, Quellen, Grundwasser.
- [x] Gras und Wald mit Samenausbreitung und Dürreschäden.
- [x] Chronik mit Ursachen; Eingriffe in Klima und Quellen.
- [x] v0.1.1: ruhiges Fenster ohne verrutschende Elemente.

## v0.2 – Samen, Buschland und Feuer

- [ ] Ein Umwelt-Eingriff kann einen einzelnen Samen an einem zufälligen, nicht angezeigten Ort eintragen.
- [ ] Keimung, Wachstum und Vermehrung folgen natürlichen Bedingungen; kein garantierter Erfolg.
- [ ] Buschland als weitere Vegetationsstufe mit eigener Trockenheits- und Brandempfindlichkeit.
- [ ] Gewitter können bei passenden Bedingungen Brände auslösen.
- [ ] Wind, Feuchtigkeit, Brennmaterial und Gelände beeinflussen die Brandausbreitung.
- [ ] Brände hinterlassen dauerhafte, natürlich regenerierende Spuren.

Umsetzungshinweise:
- Samen-Eingriff: Die Chronik vermerkt den Eingriff, aber **nicht den Ort**. Gelingt die Keimung, erscheint später ein natürliches Ereignis („Ein junger Gehölzbestand ist entstanden bei …“) – der Ort wird also erst durch Beobachtung bekannt.
- Wind wird eine neue Wettergröße (Richtung, Stärke) und ist zunächst nur für Feuer relevant.
- **Wasserreserven der Bäume** (aus Dennis' Austrocknungs-Experiment): Erreichbare Wassertiefe wächst mit dem Bestandsalter, ist aber durch die Bodentiefe begrenzt. Der Wasserbedarf wächst mit der Bestandsdichte. Alte Bestände sind also nicht grundsätzlich dürrefester – auf tiefem Boden halten sie länger durch, auf dünnem Boden am Hang nicht. Bestandsalter ist eine erste Näherung; Baumarten mit eigenem Wasserbedarf folgen später.
- **Totholz:** Abgestorbener Wald bleibt als stehendes Totholz erhalten, ist sichtbar, zerfällt über Jahre und ist besonders gutes Brennmaterial. Ereigniskette ohne Garantie: Dürre → Totholz → vielleicht Brand → Neubesiedlung durch Gras, Busch, vielleicht Wald.
- **Chronik für Vegetation:** Waldsterben (z. B. Halbierung des Bestands), „letzter Baum der Insel“, erste Neubesiedlung.
- **Bei wichtigen Ereignissen anhalten:** Schalter im Beobachter; Zeit stoppt, Kamera springt zum Ort. Sonst verpasst man bei 10 Jahren/s ganze Baumgenerationen.
- Fester Test „Paradies, dann alles aus“ (150 Jahre üppig, dann kein Regen, keine Quellen): Wald stirbt langsamer als in v0.1 (dort 50 % nach 7 Monaten, 0 % nach 4 Jahren), hinterlässt Totholz und meldet es in der Chronik.
- Spielstand-Format steigt auf 2; Spielstände aus v0.1 werden beim Laden übernommen (kein Buschland, keine Brände, Wind ruhig). Die Migration wird in den Release-Notizen genannt.

## v0.3 – Hydrologie und Erosion

- [ ] Wasserbilanz und Bedeutung der Grundwasser-Prozentanzeige überprüfen.
- [ ] Grundwasser, Seen, permanente Bäche und temporären Starkregenabfluss unterscheiden.
- [ ] Bodenabtrag und Sedimentablagerung durch Regen und fließendes Wasser.
- [ ] Gelände verändert sich über lange Zeiträume, ohne die Reproduzierbarkeit zu verlieren.

Umsetzungshinweise:
- Die Grundwasser-Anzeige ist in v0.1 ein Verhältnis zu einem Bezugswert (etwa 300 Tage volle Quellschüttung) und kann über 100 % steigen. Sie soll zu einem echten Füllstand mit Ober- und Untergrenze werden.
- Erosion ändert Höhen → Abflussnetz muss in größeren Abständen neu berechnet werden (z. B. jährlich), nicht täglich.

## v0.4 – Erste Tiere

- [ ] Einfaches Nahrungsnetz, zunächst Pflanzenfresser und Räuber.
- [ ] Fortpflanzung, Nahrungsknappheit, Wanderung und Sterblichkeit.
- [ ] Keine automatisch garantierte Stabilität und kein künstliches Wiedererscheinen ausgestorbener Populationen.
- [ ] Tierverhalten ohne LLM ausführbar.

## v0.5 – Darstellungsqualität

- [ ] Erkennbare Alters- und Wachstumsstadien der Vegetation.
- [ ] Visuelle Unterschiede zwischen Gras, Buschland, Jungwald, Altwald und verbrannten Flächen.
- [ ] Ruhige, stabile Benutzeroberfläche; sichtbar animiertes Wetter und Wasser.
- [ ] 2D-Analysekarte und isometrische Ansicht bleiben getrennte Darstellungen derselben Simulation.

Hinweis: Damit Jung- und Altwald unterscheidbar sind, braucht der Kern ab v0.2 ein Bestandsalter je Feld. Das wird in v0.2 gleich mit angelegt.

## v0.6 – Erste Bewohner

- [ ] Zunächst fünf sichtbare Menschen mit einfacher Bewegung, Nahrungssuche, Durst, Müdigkeit und Ruhephasen.
- [ ] Keine direkten Spielerbefehle.
- [ ] Noch keine komplexe KI, Erfindungssysteme oder vollständig simulierte Gesellschaft erforderlich.
- [ ] Ereignisprotokoll und anklickbare Biografien; Scheitern, Verletzung und Tod als Spielzustände.
- [ ] Individuelle Persönlichkeit, Erinnerungen, Beziehungen und Wissensweitergabe bleiben spätere Entwicklungsziele.

---

## Wichtige Regressionstests

| # | Test | Stand |
| --- | --- | --- |
| 1 | Wald wächst über Jahrhunderte, stirbt bei extremer Dürre und kommt ohne Samenquelle nicht automatisch zurück. | teilweise: „ohne Samenquelle keine Rückkehr“ ✅, Dürre-Absterben fehlt |
| 2 | Gras kann nach Rückkehr des Regens wieder wachsen. | offen |
| 3 | Ein zufälliger Samen kann bei geeigneten Bedingungen eine neue Waldpopulation begründen. | v0.2 |
| 4 | Starkregen erzeugt nachvollziehbaren Oberflächenabfluss und keine unbegrenzte Wasservermehrung. | offen (Wasserbilanz-Test) |
| 5 | Feuer kann ausbleiben, sich begrenzt ausbreiten oder größere Flächen zerstören – abhängig von Bedingungen. | v0.2 |
| 6 | Die Simulation bleibt bei jahrhundertelangem Zeitraffer stabil. | 100 Jahre ✅, auf 500 Jahre erweitern |
| 7 | Ein identischer Seed mit identischen Eingriffen liefert auf unterstützten Plattformen identische Ergebnisse. | ✅ (Referenz-Prüfsumme auf Windows, macOS, Linux) |

Die offenen Tests 1, 2, 4 und 6 betreffen v0.1 und werden zu Beginn von v0.2 ergänzt.

---

## Später (nach v0.6)

### Soziales Leben
- Gespräche, Erinnerungen, persönliche Beziehungen (gerichtet/asymmetrisch).
- KI optional, klein und ressourcenschonend; Erkenntnisse aus AI Town evaluieren.
- Unterscheidung Weltfakt / Behauptung / persönlicher Glaube.
- Beziehungen beobachten, ohne sie automatisch durch Scores zu erzwingen.

### Entdecken und Bauen
- Erste Materialien, Werkzeuge und validierte Bau-/Herstellungsregeln.
- Bedürfnisse können eigene Versuche auslösen.
- Individuelles Können und Wissensweitergabe.
- Gebäude verändern die Karte und altern.
- Später: frei vorgeschlagene Konstruktionen mit physikalischer Validierung.

### Generationen und Geschichte
- Fortpflanzung, Entwicklung, Alterung.
- Gesellschaftliche Rollen entstehen aus Handlungen und Vertrauen.
- Artefakte, Ruinen, Wissensverlust, schriftliche Überlieferung.

### Fernziel
Ursprung/Evolution, vielfältige Kulturen, Sprachwandel, komplexere Erfindungen, mehrere Siedlungen, prozedural generierte größere Welten, umfangreiche Zeitraffer und Weltchronik.
