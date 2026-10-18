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

## ✅ v0.2 – Samen, Buschland und Feuer (veröffentlicht)

- [x] Ein Umwelt-Eingriff kann einen einzelnen Samen an einem zufälligen, nicht angezeigten Ort eintragen.
- [x] Keimung, Wachstum und Vermehrung folgen natürlichen Bedingungen; kein garantierter Erfolg.
- [x] Buschland als weitere Vegetationsstufe mit eigener Trockenheits- und Brandempfindlichkeit.
- [x] Gewitter können bei passenden Bedingungen Brände auslösen.
- [x] Wind, Feuchtigkeit, Brennmaterial und Gelände beeinflussen die Brandausbreitung.
- [x] Brände hinterlassen dauerhafte, natürlich regenerierende Spuren.

Umsetzungshinweise:
- Samen-Eingriff: Die Chronik vermerkt den Eingriff, aber **nicht den Ort**. Gelingt die Keimung, erscheint später ein natürliches Ereignis („Ein junger Gehölzbestand ist entstanden bei …“) – der Ort wird also erst durch Beobachtung bekannt.
- Wind wird eine neue Wettergröße (Richtung, Stärke) und ist zunächst nur für Feuer relevant.
- **Wasserreserven der Bäume** (aus Dennis' Austrocknungs-Experiment): Erreichbare Wassertiefe wächst mit dem Bestandsalter, ist aber durch die Bodentiefe begrenzt. Der Wasserbedarf wächst mit der Bestandsdichte. Alte Bestände sind also nicht grundsätzlich dürrefester – auf tiefem Boden halten sie länger durch, auf dünnem Boden am Hang nicht. Bestandsalter ist eine erste Näherung; Baumarten mit eigenem Wasserbedarf folgen später.
- **Totholz:** Abgestorbener Wald bleibt als stehendes Totholz erhalten, ist sichtbar, zerfällt über Jahre und ist besonders gutes Brennmaterial. Ereigniskette ohne Garantie: Dürre → Totholz → vielleicht Brand → Neubesiedlung durch Gras, Busch, vielleicht Wald.
- **Chronik für Vegetation:** Waldsterben (z. B. Halbierung des Bestands), „letzter Baum der Insel“, erste Neubesiedlung.
- **Bei wichtigen Ereignissen anhalten:** Schalter im Beobachter; Zeit stoppt, Kamera springt zum Ort. Sonst verpasst man bei 10 Jahren/s ganze Baumgenerationen.
- Fester Test „Paradies, dann alles aus“ (150 Jahre üppig, dann kein Regen, keine Quellen): Wald stirbt langsamer als in v0.1 (dort 50 % nach 7 Monaten, 0 % nach 4 Jahren), hinterlässt Totholz und meldet es in der Chronik.
- Spielstand-Format steigt auf 2; Spielstände aus v0.1 werden beim Laden übernommen (kein Buschland, keine Brände, Wind ruhig). Die Migration wird in den Release-Notizen genannt.

## ✅ v0.3 – Hydrologie und Erosion (veröffentlicht als 0.2.2–0.3.0)

- [x] Wasserbilanz und Bedeutung der Grundwasser-Prozentanzeige überprüfen (v0.2.4).
- [x] **Abfluss nach Wasserspiegel** statt fester Fließrichtung – Stau, Überlauf, Überschwemmung (v0.2.2).
- [x] **Damm und Durchbruch** als Geländeeingriff (v0.2.2).
- [x] Permanente Bäche, zeitweise Bäche und Starkregenabfluss unterscheiden; Mengen in m³ und L/s (v0.2.3).
- [x] Grundwasser als echter Füllstand mit Überlauf und Quellschwelle (v0.2.4).
- [x] Seen eigens benennen; Verlanden und Verschwinden in der Chronik (v0.3.0).
- [x] Bodenabtrag und Sedimentablagerung durch Regen und fließendes Wasser; Bodenbildung aus Gestein; Schwemmland an Mündungen (v0.3.0).
- [x] Gelände verändert sich über lange Zeiträume, ohne die Reproduzierbarkeit zu verlieren (v0.3.0, Referenz-Prüfsumme auf allen Plattformen).

- [x] **Quellwasser und Oberflächenabfluss sichtbar machen** (Darstellung als Linien und Rinnsal-Statistik schon in v0.2; offen: Abfluss nach Wasserspiegel, Mengenanzeige): prüfen, ob das Wasser physikalisch nachvollziehbar verteilt wird und ob 2D-Wasserkarte und isometrische Ansicht denselben Wasserzustand korrekt anzeigen. Bestehende Wasserläufe sollen abhängig von ihrer Wassermenge sichtbar anschwellen oder austrocknen.

Befund aus Dennis' Quellen-Test (v0.1.1, nachgeprüft mit `tests/springs.rs`):
- Der Kern ist korrekt: Quellwasser geht in die Wasserbilanz ein, fließt bergab, der Bach trocknet bei 0 % aus und kehrt bei 100 % zurück; die Schüttung folgt dem Regler.
- **Statistik sättigt:** „Bachfelder“ zählt nur Felder über 60 mm/Tag. Unter beiden Quellen liegen zusammen nur 21 Felder bis zum Meer – sind die voll, bringt 300 % kein einziges Feld dazu. Im Winter überdeckt Regen die Quellen völlig. → Statt Felder zählen: Wassermenge anzeigen (z. B. Abfluss ins Meer, Schüttung je Quelle), Rinnsale getrennt von Bächen.
- **300 % hält nicht an:** Der Regler verändert, wie leicht das Grundwasser austritt, nicht, wie viel Wasser nachkommt. Bei 300 % leert sich das Grundwasser der Insel in wenigen Monaten, danach fällt die Schüttung fast auf den alten Wert zurück. Physikalisch ehrlich, aber für den Spieler nicht erkennbar. → **Entschieden:** Regler bleibt *Austritt* – die Geologie bestimmt, woher das Wasser kommt ([ADR 0003](../decisions/0003-spring-control-is-outflow.md)). Der Regler erklärt sich in der Oberfläche.
- **Darstellung zu schwach:** Wasserläufe unter etwa 30 mm/Tag sind unsichtbar, darüber wird nur die Feldfarbe leicht getönt. → Wasserläufe als Linien von Feld zu Feld, Dicke nach Wassermenge.
- **Einzelne Abflussbahn:** Jedes Feld gibt sein Wasser an genau ein Nachbarfeld ab. Wasser kann sich deshalb nicht verbreitern, nicht über eine Wiese ausufern und keinen neuen Weg suchen. → Abfluss nach Wasserspiegel statt fester Richtung (Voraussetzung für Überschwemmung und für Dämme, siehe unten).

Umsetzungshinweise:
- ~~Die Grundwasser-Anzeige ist ein Verhältnis zu einem Bezugswert und kann über 100 % steigen.~~ Erledigt in v0.2.4: echter Füllstand.
- ~~Erosion ändert Höhen → Abflussnetz muss in größeren Abständen neu berechnet werden.~~ Nicht nötig: Der Abfluss richtet sich ohnehin täglich nach dem Wasserspiegel; Höhen ändern sich in Schritten ab 1 mm.

## ✅ v0.4 – Erste Tiere (veröffentlicht als 0.4.0)

- [x] Einfaches Nahrungsnetz: Hirsche und Kaninchen fressen Pflanzen, Wölfe jagen Hirsche, Füchse Kaninchen.
- [x] Fortpflanzung, Nahrungsknappheit, Wanderung und Sterblichkeit (Hunger, Jagd, Alter, Feuer).
- [x] Keine automatisch garantierte Stabilität und kein künstliches Wiedererscheinen ausgestorbener Populationen.
- [x] Tierverhalten ohne LLM ausführbar.
- [ ] Tier-Sprites von Codex einbinden (Schnittstelle steht, gestaltung.md 15.7); Kadaver und Spuren.
- [ ] Abstimmung (0.4.1 verbessert): Füchse halten sich oft Jahrzehnte, Wölfe meist nur einige Jahre – die Insel (gut 4 km²) ist für ein Rudel zu klein. Idee: größere Inseln wählbar machen.

Entschieden (17.10.2026, Dennis):
- **Erst Erosion (v0.3) abschließen**, dann Tiere.
- **Vier Arten:** Hirsche und Kaninchen (Pflanzenfresser), Wölfe und Füchse (Räuber). Hirsche fressen Gras, Busch und junge Bäume und bremsen damit den Wald; Kaninchen fressen vor allem Gras; Wölfe jagen vor allem Hirsche, Füchse vor allem Kaninchen.
- **Ankunft nur durch den Beobachter:** Eingriff „Tiere aussetzen“ (wie beim Samen), steht in der Chronik. Eine neue Insel ist tierfrei. Ausgestorbene Arten kehren nicht von selbst zurück.
- Anlass: Dennis' Dauerlauf (36.000 Jahre) bleibt stabil bei etwa 85 % Wald – ohne Pflanzenfresser hat der Wald kaum Gegenspieler außer Dürre und Feuer.

## v0.5 – Darstellungsqualität

- [ ] Erkennbare Alters- und Wachstumsstadien der Vegetation.
- [ ] Visuelle Unterschiede zwischen Gras, Buschland, Jungwald, Altwald und verbrannten Flächen.
- [ ] Ruhige, stabile Benutzeroberfläche; sichtbar animiertes Wetter und Wasser.
- [ ] 2D-Analysekarte und isometrische Ansicht bleiben getrennte Darstellungen derselben Simulation.

Hinweis: Damit Jung- und Altwald unterscheidbar sind, braucht der Kern ab v0.2 ein Bestandsalter je Feld. Das wird in v0.2 gleich mit angelegt.

## ✅ v0.6 – Erste Bewohner (v0.6.0)

- [x] Fünf sichtbare Menschen mit Bewegung, Nahrungssuche, Durst, Müdigkeit und Ruhephasen. Sie kommen nur durch den Beobachter auf die Insel (Eingriff „Boot landen lassen“).
- [x] Keine direkten Spielerbefehle.
- [x] Keine komplexe KI: Bedürfnisse wählen das Ziel, Wege meiden steile Stufen, jeder kennt nur, was er gesehen hat (Wasser- und Sammelstellen, Küste).
- [x] Chronik und anklickbare Biografien (Fenster „Bewohner“); Verletzung (Sturz, Wölfe), Not und Tod (Durst, Hunger, Kälte, Feuer, Alter) als Spielzustände. Gefährten teilen Wasser und Essen und bleiben bei Verletzten.
- [x] Eigene Nahansicht bis Zoom 32 (ADR 0006); Platzhalterfiguren, bis Codex' Bilder kommen (gestaltung.md 17).
- [ ] Offen: Nachwuchs, Persönlichkeit, Beziehungen, Wissensweitergabe (Lehren, Nachahmen), Werkzeuge, Unterkünfte, Feuer machen, Jagd und Fischfang als eigene Tätigkeiten.

---

## ✅ v0.7 – Nachwuchs (v0.7.0)

- [x] Paare finden sich unter Erwachsenen, die viel beieinander sind (keine nahen Verwandten). Gefährten bleiben beieinander, wenn nichts drängt.
- [x] Schwangerschaft (9 Monate), Geburt mit Risiken für Mutter und Kind, Stillzeit von zwei Jahren, Abstand zwischen Geburten.
- [x] Kinder ziehen mit der Familie, sammeln ab 8 Jahren mit, bekommen von den Erwachsenen ab und lernen von ihnen die Wasserstellen und Küstenplätze (Wissen geht weiter, Prinzip 6). Mit 14 sind sie erwachsen.
- [x] Krankheiten treffen vor allem Säuglinge und Kleinkinder. Familie (Eltern, Partner, Kinder) im Fenster „Bewohner“.
- Ergebnis: Auf wasserreichen Inseln wachsen über Generationen Gruppen von 20–35 Menschen; auf kargen Inseln schrumpfen sie nach Hungerzeiten und sterben aus, wenn die Jungen nur noch Geschwister um sich haben – ein weiteres Boot kann helfen.

## Als Nächstes: Sesshaftigkeit (Unterkünfte, Lager, Dörfer)

Die Bewohner bauen selbst – der Beobachter gibt keine Bauaufträge (Prinzip 1).
- Unterschlupf und Lagerplatz: Wer immer wieder an dieselben Wasser- und Sammelstellen zurückkehrt, legt dort einen Lagerplatz an (Feuerstelle, Windschutz). Schutz vor Kälte statt nur Wald.
- Hütten aus Holz, Ästen, Fell; Holz kommt aus dem Wald und fehlt dort danach (Rückwirkung auf die Natur).
- Vorräte, die man nicht mehr tragen muss; Fischfang und später Anbau als Voraussetzung für größere, feste Siedlungen – auf 4 km² können sonst nur wenige Sammler leben.
- Siedlungen entstehen dort, wo Wasser, Nahrung und Schutz zusammenkommen, und verfallen, wenn sie verlassen werden.

## Später: Biome

Entwurf von Codex: Feuchter Wald, Wiesenland, Trockenes Buschland, Steppe, Feuchtgebiet, Felsiges Bergland.

- Ein Biom ist ein **Ergebnis**, keine Vorgabe: Es beschreibt, was die Simulation auf einem Feld hervorgebracht hat (Wald, Busch, Gras, Feuchte, Wasser, Gelände).
- Zuordnung über **mehrjährige Durchschnittsfeuchte** (nicht die heutige), mit Trägheit an den Grenzen – sonst springt die Insel mit den Jahreszeiten zwischen Biomen.
- Auf der kleinen Testinsel kommen bei normalem Klima vor allem Wald, Wiese, Busch, Fels und Feuchtgebiet vor; Steppe erst nach starken Klimaeingriffen oder auf größeren, trockeneren Welten.
- Unabhängig vom Biom treten Zustände auf: Brandfläche, Totholz, Jungwald, Strand.

## Langfristig: Interaktive Hydrologie und Landschaftsgestaltung

Der Spaß, mit den Naturgesetzen einer kleinen Welt zu spielen – wie als Kind ein Damm im Bach. Der Beobachter bestimmt nicht, wohin das Wasser fließt; er verändert nur ein Hindernis und schaut, was passiert.

- Ein Hindernis in einen Bach legen: Das Wasser staut sich oder fließt darum herum.
- Ein Hindernis entfernen: Das Wasser nimmt möglicherweise einen neuen Weg.
- Einen Durchbruch zwischen zwei Senken öffnen: Ein kleiner See läuft langsam ab.
- Starkregen lässt einen Bach anschwellen und eine Wiese überschwemmen.
- Nach Jahrzehnten hat das Wasser den Boden abgetragen und den Bachlauf dauerhaft verändert (Erosion, v0.3).

Wasser folgt dabei tatsächlich Gelände und physikalischen Regeln, keine Textur entlang einer vorberechneten Linie.

Technische Voraussetzungen:
- Abfluss nach Wasserspiegel (Gelände + Wasserstand) statt fester Fließrichtung; Fließrichtungen neu berechnen, sobald sich Gelände ändert.
- Gelände-Eingriffe als neue Art von `Intervention` (Damm, Durchbruch, Hindernis) – sie verändern Bedingungen und landen in der Chronik.
- Feinere räumliche Auflösung: Bei 50 × 50 m pro Feld verschwindet ein einzelner Stein. Kleine Dämme und Geländeänderungen gehen schon früher, für Steine braucht es z. B. ein feineres Detailraster dort, wo der Beobachter hinschaut.
- Wasserbilanz bleibt bei jedem Eingriff geschlossen (Regressionstest 4).

## Wichtige Regressionstests

| # | Test | Stand |
| --- | --- | --- |
| 1 | Wald wächst über Jahrhunderte, stirbt bei extremer Dürre und kommt ohne Samenquelle nicht automatisch zurück. | ✅ `ecology.rs` |
| 2 | Gras kann nach Rückkehr des Regens wieder wachsen. | ✅ `ecology.rs` |
| 3 | Ein zufälliger Samen kann bei geeigneten Bedingungen eine neue Waldpopulation begründen. | ✅ `ecology.rs` (inkl. Gegenprobe Wüste) |
| 4 | Starkregen erzeugt nachvollziehbaren Oberflächenabfluss und keine unbegrenzte Wasservermehrung. | ✅ `ecology.rs` (Wasserbilanz bei 2500 mm) |
| 5 | Feuer kann ausbleiben, sich begrenzt ausbreiten oder größere Flächen zerstören – abhängig von Bedingungen. | ✅ `ecology.rs` |
| 6 | Die Simulation bleibt bei jahrhundertelangem Zeitraffer stabil. | ✅ 500 Jahre |
| 7 | Ein identischer Seed mit identischen Eingriffen liefert auf unterstützten Plattformen identische Ergebnisse. | ✅ (Referenz-Prüfsumme auf Windows, macOS, Linux) |
| 8 | Quellen aus- und wieder einschalten: Schüttung folgt dem Regler, Bach trocknet aus und kehrt zurück, reproduzierbar. | ✅ `tests/springs.rs` (ohne Regen gemessen) |
| 9 | Spielstände aus v0.1 lassen sich laden und weiterrechnen. | ✅ `ecology.rs` |

Alle Tests liegen in `crates/core/tests/` und laufen bei jeder Änderung auf Windows, macOS und Linux.

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
