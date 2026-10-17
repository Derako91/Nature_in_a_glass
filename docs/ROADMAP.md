# Roadmap – schrittweise, nicht als starre Fertigstellungstermine

## M0 – Projektfundament
- [ ] Eigenständiges Repository anlegen; unabhängig von AI Town.
- [x] Projektvision und unverhandelbare Spielprinzipien dokumentieren.
- [x] Grobe modulare Architektur entwerfen.
- [ ] Laufzeit, Sprache, Rendering und Speichermodell entscheiden.
- [ ] Automatische Tests und CI einrichten.

## M1 – Die Insel lebt ohne Menschen
- [ ] Welt mit Seed generieren und rendern (MVP: z. B. 64×64).
- [ ] Simulation anhalten, beschleunigen, speichern, laden.
- [ ] Einfacher Regen-/Wasservorrats-/Bodenfeuchte-Kreislauf.
- [ ] Pflanzenwachstum und -verfall.
- [ ] Reproduzierbarer Test über 100 simulierte Jahre; Zustands- und Ereignisverlauf auswerten.

## M2 – Fünf Bewohner
- [ ] Bedürfnisse: Hunger, Durst, Müdigkeit, Schutz.
- [ ] Wahrnehmung, Bewegung, einfache Handlungen (trinken, sammeln, ruhen).
- [ ] Individuelle Merkmale ohne festgelegte Rollen.
- [ ] Ereignisprotokoll und anklickbare Biografien.
- [ ] Scheitern, Verletzung und Tod als Spielzustände.

## M3 – Soziales Leben
- [ ] Gespräche, Erinnerungen, persönliche Beziehungen (gerichtet/asymmetrisch).
- [ ] KI optional, klein und ressourcenschonend; Erkenntnisse aus AI Town evaluieren.
- [ ] Unterscheidung Weltfakt / Behauptung / persönlicher Glaube.
- [ ] Beziehungen beobachten, ohne sie automatisch durch Scores zu erzwingen.

## M4 – Entdecken und Bauen
- [ ] Erste Materialien, Werkzeuge und validierte Bau-/Herstellungsregeln.
- [ ] Bedürfnisse können eigene Versuche auslösen.
- [ ] Individuelles Können und Wissensweitergabe.
- [ ] Gebäude verändern die Karte und altern.
- [ ] Später: frei vorgeschlagene Konstruktionen mit physikalischer Validierung.

## M5 – Generationen und Geschichte
- [ ] Fortpflanzung, Entwicklung, Alterung.
- [ ] Gesellschaftliche Rollen entstehen aus Handlungen und Vertrauen.
- [ ] Artefakte, Ruinen, Wissensverlust, schriftliche Überlieferung.
- [ ] Umweltkatastrophen mit Voraussetzungen und dauerhaften Folgen.

## Fernziel
Ursprung/Evolution, vielfältige Kulturen, Sprachwandel, komplexere Erfindungen, mehrere Siedlungen, umfangreiche Zeitraffer und Weltchronik.

## Erste Issue-Vorschläge
1. Architekturentscheidung: Runtime und Rendering vergleichen.
2. Deterministischer Simulationskern mit Seed und Zeitschritten.
3. Kartenmodell + Speichern/Laden.
4. Wetter/Bodenfeuchte/Vegetation minimal simulieren.
5. Beobachtungs-UI mit Zeitsteuerung und Ereignisjournal.
