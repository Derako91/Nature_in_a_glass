# Änderungen

## 0.4.2 – Wölfe halten länger durch

**Geändert**
- **Ein Rudel bleibt ein Rudel:** Ab 8 Wölfen (mit Jungtieren) gibt es keine Welpen mehr. Vorher wuchs das Rudel auf über 20 Tiere, jagte die Hirsche fast aus und verhungerte dann.
- **Seltene Hirsche sind schwerer zu finden:** Wenn es nur noch wenige gibt, misslingen mehr Jagden. So erholen sich die Hirsche eher.
- **Hungrige Wölfe jagen Kaninchen:** Sehr hungrige Wölfe suchen gezielt Felder mit vielen Kaninchen auf, nicht nur zufällig.

**Beobachtet:** Setzt man Wölfe zu über 200 Hirschen, halten sie sich jetzt meist 10–35 Jahre; Hirsche und Wölfe pendeln dabei miteinander. Mit Kaninchen auf der Insel können Wölfe sogar überleben, wenn die Hirsche verschwunden sind.


## 0.4.1 – Die Zeit läuft weiter, Tiere besser abgestimmt

**Behoben**
- **Minimiertes Fenster hielt die Insel an.** Die Simulation läuft jetzt in einem eigenen Faden weiter, auch wenn das Fenster minimiert ist oder verdeckt wird.
- **Kaninchen schrumpften ohne Fressfeinde** (z. B. von 20 auf 4), besonders auf alten, bewaldeten Inseln. Ursachen: Bei Überschwemmung ertranken alle Kaninchen eines Feldes, und beim Abwandern verteilten sie sich so dünn, dass die Reste verloren gingen. Jetzt flüchten sie vor Hochwasser, ziehen gezielt zum Nachbarfeld mit dem meisten Platz, und Waldränder und Lichtungen ernähren auch ein paar Kaninchen.
- **Wölfe verhungerten neben vielen Hirschen,** wenn ein See dazwischen lag: Sie fanden keinen Weg um das Wasser. Jetzt suchen sie den Landweg.
- **Füchse verhungerten im ersten Winter.** Sie jagen jetzt im ganzen Revier (Feld und Nachbarfelder), ziehen weiter, wenn es dort wenig gibt, und finden auch im Winter etwas Kleingetier.
- Kaninchen und Hirsche werden auf einer dicht bewaldeten Insel an den offensten Stellen ausgesetzt.

**Neu**
- **Verlauf umschaltbar:** *Pflanzen* oder *Tiere*. Bei den Tieren ist jede Linie auf ihren eigenen Höchstwert bezogen, damit man vier Wölfe neben tausenden Kaninchen sieht. Mit der Maus über den Verlauf fahren zeigt die Zahlen des Jahres.

**Geändert**
- Wölfe fressen sich am Riss einen Vorrat an und müssen seltener jagen; ein Hirsch gibt mehr her. Füchse brauchen etwas weniger.
- Reviere: Auf der Insel ist Platz für **einen Wolfswurf** und **drei Fuchswürfe** im Jahr. Vorher wurde die Zahl der erwachsenen Tiere begrenzt – dann bekam ein gealtertes Rudel nie mehr Junge und starb geschlossen aus.

**Beobachtet**
- Füchse halten sich mit tausenden Kaninchen jetzt oft Jahrzehnte; es gibt richtige Räuber-Beute-Wellen.
- Wölfe halten sich meist einige Jahre bis etwa ein Jahrzehnt. Die Insel ist für ein Wolfsrudel eigentlich zu klein (echte Rudel brauchen viele Quadratkilometer). Eine größere Insel ist der natürliche nächste Schritt.

**Getestet:** Kaninchen breiten sich auf einer alten, zu über 60 % bewaldeten Insel ohne Fressfeinde auf mehrere hundert aus.


## 0.4.0 – Erste Tiere

**Neu in der Natur**
- **Vier Tierarten:** Hirsche, Kaninchen, Wölfe und Füchse. Hirsche, Wölfe und Füchse leben als einzelne Tiere mit Alter, Geschlecht und Energiereserven; Kaninchen als Bestand je Feld.
- **Nahrungsnetz:** Kaninchen fressen Gras. Hirsche fressen Gras, Busch und junge Bäume – viele Hirsche bremsen damit die Verjüngung des Waldes. Wölfe jagen Hirsche, schwache und junge leichter, im Rudel erfolgreicher, seltene Beute schwerer. Füchse jagen Kaninchen, fressen an Kadavern und im Sommer auch Kleintiere.
- **Verhalten:** Hirsche ziehen zu Futter, bleiben gern beieinander und fliehen vor Wölfen; hungrige Hirsche suchen weiter. Wölfe nehmen die Spur auf, schleichen sich an und hetzen. Das Rudel frisst gemeinsam am Riss. Junge folgen ihrer Mutter.
- **Leben und Tod:** Junge einmal im Jahr, wenn die Mutter gut genährt ist und ein Partner in der Nähe lebt. Tiere verhungern, werden gerissen, sterben an Altersschwäche oder im Feuer. Jeder Tod hinterlässt für einige Tage einen Kadaver.
- **Keine Tiere von selbst:** Eine neue Insel ist tierfrei. Tiere kommen nur durch den Eingriff **Tiere aussetzen** – und eine ausgestorbene Art kehrt nur zurück, wenn du neue aussetzt.

**Neu im Beobachter**
- Unter *Eingriffe*: **Tiere aussetzen** – Hirsche (6), Kaninchen (20), Wölfe (Rudel aus 4), Füchse (4). Auf das gewählte Feld oder an eine zufällige Stelle; Räuber dort, wo es Beute gibt.
- Ebene **Tiere** und vorläufige Tierpunkte auf Karte und Insel (bis 1 Woche/s, auf der Ebene *Tiere* immer). Die Tierbilder von Codex folgen.
- Insel-Ansicht: Hirsche / Kaninchen, Wölfe / Füchse. Feld-Kasten: Tiere auf dem Feld.
- Chronik: Aussetzen, **Aussterben** einer Art mit Ursachen (wichtiges Ereignis), **Hungerjahre**. Die Jahresbilanz nennt die Bestände.
- Startoption `--tiere hirsche,kaninchen,woelfe,fuechse` mit `--danach N`.

**Beobachtet (Insel 42 und weitere)**
- Ohne Wölfe wachsen sechs Hirsche in 20 Jahren auf einige hundert an. Danach fressen sie Busch und Gras knapp, es folgen Hungerjahre; der Bestand pendelt zwischen etwa 100 und 400. Kaninchen werden dabei oft verdrängt.
- Kaninchen allein wachsen in wenigen Jahren auf über 1.000 bis 4.000.
- Wölfe und Füchse halten sich auf der kleinen Insel meist nur einige Jahre bis Jahrzehnte: Entweder sie jagen ihre Beute knapp und verhungern, oder das Rudel schrumpft durch Alter. Manchmal reißen Wölfe die Hirsche ganz aus. Ein Wolfsrudel lohnt sich erst, wenn viele Hirsche da sind.

**Getestet**
- Neue Insel tierfrei; ausgesetzte Hirsche vermehren sich und fressen Gras und Busch merklich ab.
- Ohne Futter verhungern sie, das Aussterben steht mit Ursache in der Chronik, und sie kommen auch bei wieder gutem Regen nicht zurück.
- Wölfe reißen Hirsche, Füchse dezimieren Kaninchen.
- Gleicher Seed, gleiche Tiere; Speichern und Laden mitten im Tierleben ändert nichts.
- Aussetzen im Wasser wird abgelehnt.

**Spielstände:** Format 6. Alle früheren Spielstände lassen sich laden (ohne Tiere).


## 0.3.0 – Erosion: Die Insel formt sich selbst

Letzter Schritt von v0.3 (Hydrologie und Erosion). Damit ist die Hydrologie abgeschlossen.

**Neu in der Natur**
- **Jedes Feld hat eine Bodenschicht** mit eigener Dicke. Dünner Boden hält weniger Wasser, dort wächst weniger.
- **Abspülung:** Regen und abfließendes Wasser tragen Boden ab – je steiler und je kahler, desto mehr. Bewuchs schützt stark, Brandflächen kaum: Ein Starkregen kurz nach einem Brand reißt am Hang viel Boden mit.
- **Bäche graben sich ein**, auf nacktem Fels viel langsamer als im Boden.
- **Ablagerung:** Wo das Wasser langsamer wird – in Senken, flachen Niederungen und Seen –, setzt sich der Boden ab. **Seen verlanden** über Jahrhunderte.
- **Schwemmland:** Was das Meer erreicht, lagert sich vor der Mündung ab. Wächst die Sandbank über den Meeresspiegel, taucht neues Land auf.
- **Bodenbildung:** Aus dem Gestein entsteht langsam neuer Boden, unter Wald schneller.
- **Seen bekommen Namen** (z. B. Erlensee, Rohrweiher), sobald sie drei Jahre bestehen, und behalten sie.

**Neu im Beobachter**
- Ebene **Boden/Abtrag**: Rot verliert Boden, Blaugrün sammelt ihn; heller Grundton ist nackter Fels, dunkelbraun tiefe Erde.
- Insel-Ansicht: **Seen** (benannte Seen und Wasserfelder) und **Boden** (mittlere Dicke, Abtrag im letzten Jahr).
- Feld-Kasten: **Bodendicke** und wie viel das Feld je 100 Jahre verliert oder gewinnt; der Name des Sees.
- Chronik: **Abspülung** nach starkem Regen (mit Hinweis auf Brandflächen), **ein See hat sich gebildet**, **See verlandet**, **See verschwunden**, **mit anderem See zusammengewachsen**, **neues Land**. Ein verschwundener oder verlandeter See zählt zu den wichtigen Ereignissen.
- Kommandozeile: Spalten für Abtrag und Bodendicke.

**Behoben**
- Sehr lange Läufe wurden mit der Zeit langsamer: Winzige Restwerte (z. B. letzte Grashalme auf nacktem Fels) blieben in einem Zahlenbereich hängen, mit dem der Prozessor nur sehr langsam rechnet. Sie werden jetzt regelmäßig auf null gesetzt; die Simulation läuft über Jahrtausende gleich schnell.

**Beobachtet (Insel 42)**
- Unter Wald geht kaum Boden verloren; auf Brandflächen am Hang verliert der Boden bei Starkregen im ersten Jahr mehrere Millimeter, bis das Gras zurück ist.
- Die Insel verliert im Jahr einige hundert m³ Boden, etwa zwei Drittel davon gelangen ins Meer.
- Der 28 m tiefe Erlensee füllt sich langsam (einige Zentimeter je Jahrhundert). Über 3000 Jahre nimmt die Zahl der Wasserfelder in Senken von gut 40 auf etwa 25 ab, weil flache Tümpel verlanden.
- Etwa alle hundert Jahre taucht an einer Bachmündung ein Stück Schwemmland auf.
- Bäche schneiden sich ein: Die Zahl der dauerhaft fließenden Bachfelder steigt über 3000 Jahre von etwa 40 auf 60. Der Wald bleibt bei 70–85 %.

**Getestet**
- Kahler Hang verliert bei Regen viel mehr Boden als ein bewachsener; die Chronik meldet die Abspülung.
- Bodenbilanz geschlossen: Was oben fehlt, liegt unten oder im Meer.
- 300 Jahre: Land wird nicht zu Meer, Wasserhaltevermögen folgt der Bodendicke, Seeböden heben sich.
- Seen haben eindeutige, bleibende Namen; gleicher Seed, gleiche Namen; ein ausgetrockneter See wird gemeldet.

**Spielstände:** Format 5. Spielstände aus allen früheren Versionen werden übernommen: Die Bodendicke ergibt sich aus dem bisherigen Wasserhaltevermögen, bestehende Seen werden beim Laden erkannt und nach drei Jahren benannt. Gleiche Inseln entwickeln sich wegen der Erosion anders als in 0.2.


## 0.2.5 – Chronik bleibt auch nach Jahrtausenden lebendig

**Behoben**
- Nach etwa 20.000 Jahren kamen **keine neuen Einträge mehr in die Chronik**. Die Jahresbilanzen hatten den ganzen Platz belegt, und jedes neue Ereignis wurde sofort wieder gelöscht. Jetzt haben Ereignisse (bis 20.000) und Jahresbilanzen (bis 5.000) getrennte Plätze; es fallen jeweils die ältesten heraus. Eingriffe und die Entstehung der Insel bleiben immer stehen.
- Spielstände aus sehr langen Läufen werden beim Laden aufgeräumt und zeigen danach wieder neue Ereignisse.

**Spielstände** bleiben kompatibel (Format 4). Die Natur rechnet unverändert.


## 0.2.4 – Grundwasser als echter Füllstand

Dritter Schritt von v0.3 (Hydrologie).

**Neu**
- Der Grundwasserkörper der Insel hat ein **Fassungsvermögen**. Die Anzeige ist jetzt ein echter Füllstand zwischen 0 und 100 % und nennt zusätzlich die Menge in Millionen m³.
- **Voller Grundwasserkörper läuft über:** Weiteres Sickerwasser fließt unterirdisch ins Meer und wird in der Wasserbilanz gezählt.
- **Quellen liegen höher als der Grund des Grundwasserkörpers:** Unter 5 % Füllstand versiegen sie, obwohl darunter noch Wasser steht. Ohne Regen werden sie zuerst zum dünnen Rinnsal.
- Chronik: **Grundwasser-Tiefstand** (unter 15 %, mit Ursachen) und **Erholung** (über 30 %). Der Tiefstand zählt zu den wichtigen Ereignissen.

**Behoben**
- Die Grundwasser-Anzeige konnte über 100 % steigen (sie war nur ein Verhältnis zu einem Bezugswert).

**Hinweis:** Die Prozentzahlen sind niedriger als vorher, weil sie sich jetzt auf das volle Fassungsvermögen beziehen. Auf Insel 42 liegt der Füllstand über Jahrhunderte zwischen etwa 20 und 80 %.

**Spielstände:** Format 4. Ältere Spielstände bekommen beim Laden ein Fassungsvermögen.


## 0.2.3 – Dauerhafte Bäche, zeitweise Bäche, Regenrinnen

Zweiter Schritt von v0.3 (Hydrologie).

**Neu**
- Jedes Feld merkt sich ein Jahr lang, wie viel und wie beständig dort Wasser fließt. Daraus wird jeder Wasserlauf eingeteilt: **dauerhaft** (fließt fast das ganze Jahr, z. B. unter Quellen), **zeitweise** (fließt einen Teil des Jahres, sonst trockenes Bachbett), **Regenrinne** (nur nach Regen).
- Insel-Ansicht: *Bäche dauernd / zeitw. / Regen* und **Abfluss ins Meer** in m³ statt Feldzählung.
- Quellen in **Litern pro Sekunde**; der Feld-Kasten zeigt Durchfluss in L/s und die Art des Wasserlaufs.
- Jahresbilanz: dauerhafte Bachfelder und Jahresabfluss ins Meer (m³).
- Nebenbei: mehrjährige Bodenfeuchte je Feld als Grundlage für spätere Biome.

**Beobachtet (Insel 42, Jahr 30)**
- Etwa 40 Felder dauerhafter Bach, etwa 60 Felder Winterbach, der im Hochsommer trockenfällt.
- Die beiden Quellen schütten zusammen 9–14 L/s.
- Rund 910.000 m³ fließen im Jahr ins Meer – 23 % des Niederschlags.

**Spielstände:** Format 3. Spielstände aus 0.1 und 0.2 werden übernommen; das Jahresgedächtnis wird aus dem aktuellen Zustand geschätzt und ist nach einem Jahr genau.


## 0.2.2 – Wasser folgt dem Wasserspiegel

Erster Schritt von v0.3 (Hydrologie).

**Neu**
- **Abfluss nach Wasserspiegel:** Wasser fließt nicht mehr auf festen Bahnen, sondern jeden Tag dorthin, wo Gelände *und* Wasserstand tiefer liegen – aufgeteilt auf alle tieferen Nachbarn, steilere bekommen mehr. Wasser staut sich in Senken, bis es überläuft, tritt bei Starkregen in flachem Gelände über die Ufer und läuft danach wieder ab.
- **Damm und Durchbruch:** Im Feld-Kasten lässt sich das Gelände eines Landfeldes um 2 m anheben oder absenken. Ein Damm staut einen Bach, ein Durchbruch lässt einen Stau oder See auslaufen. Wohin das Wasser dann fließt, entscheidet die Insel. Jeder Eingriff steht in der Chronik. Meer lässt sich nicht aufschütten, Land nicht zu Meer graben.
- Simulation gut 20 % schneller trotz aufwändigerem Abfluss.

**Beobachtet**
- Senken füllen sich jetzt vollständig: Unter Quelle 1 von Insel 42 liegt ein bis zu 28 m tiefer See.

**Getestet**
- Damm hält einen Bach auf, Wasser staut sich oder sucht einen Umweg, Durchbruch stellt den Bach wieder her; Wasserbilanz bleibt dabei geschlossen.
- Durchbruch lässt einen Stau ablaufen.
- Starkregen überschwemmt flache Felder und die Überschwemmung läuft wieder ab.
- Geländeeingriffe sind begrenzt und erzeugen kein Meer.

**Spielstände** bleiben unverändert kompatibel (Format 2). Gleiche Inseln entwickeln sich wegen des neuen Abflusses anders als in 0.2.0.


## 0.2.0 – Samen, Buschland und Feuer

**Neu in der Natur**
- **Buschland** als Zwischenstufe zwischen Gras und Wald. Junge Bäume keimen bevorzugt im Schutz von Büschen.
- **Bestandsalter:** Jeder Wald hat ein Alter. Jungwald ist heller, Altwald dunkler.
- **Wasserreserven der Bäume:** Ältere Bäume reichen mit ihren Wurzeln tiefer – aber nur so tief, wie der Boden es zulässt. Dichter Wald braucht mehr Wasser. Derselbe Dürresommer wirkt deshalb je nach Ort ganz unterschiedlich.
- **Bäume sterben bei Dürre allmählich:** Sie zehren erst von Reserven, dann beschleunigt sich das Sterben. Ein normaler Sommer tötet keinen Wald mehr.
- **Totholz:** Abgestorbene Bäume bleiben stehen, sind als graubraune Flächen sichtbar und zerfallen über Jahre.
- **Wind** mit Richtung und Stärke, meist aus Westen.
- **Gewitter und Blitzschlag**, auch trockene Gewitter mit kaum Regen.
- **Feuer:** Ein Blitz zündet nur, wenn genug trockenes Brennmaterial da ist. Das Feuer breitet sich mit dem Wind und bergauf schneller aus; Bäche, Teiche, nasse Böden und Regen bremsen oder löschen es. Zurück bleibt eine verkohlte Fläche, auf der zuerst Gras, dann Busch und – von den Rändern her – wieder Wald wächst.
- **Samen ausstreuen:** Ein einzelner Baum- oder Buschsamen landet an einem zufälligen, verborgenen Ort. Ob er keimt, entscheiden Feuchte, Temperatur und Platz. Erst eine Keimung verrät in der Chronik, wo er gelandet ist.

**Neu im Beobachter**
- **Wasserläufe als Linien**, deren Dicke der Wassermenge folgt – in der isometrischen Ansicht und auf der Karte. Schwache Rinnsale blass, kräftige Bäche satt.
- Statistik **„Bäche / Rinnsale“** statt nur Bachfelder.
- Neue Ebene **Brandgefahr**.
- **Wichtige Ereignisse:** *Nur Chronik*, *Anhalten* oder *Anhalten und hinfahren*. Wichtig sind Brand, Dürre, Waldsterben, letzter Baum, Rückkehr der Bäume, Keimung, versiegte Quelle.
- Neue Werte in Insel- und Feld-Ansicht: Busch, Totholz, Tiefenwasser, Waldalter, Wasserversorgung der Bäume, Feuer, Wind.
- Der Quellregler erklärt sich beim Darüberfahren ([ADR 0003](../decisions/0003-spring-control-is-outflow.md)).
- Neue Chronik-Einträge: Brände mit Ursache und Größe, Waldsterben, letzter Baum, Rückkehr der Bäume, Keimung, nicht aufgegangene Samen.

**Behoben**
- Bei 0 mm Jahresregen gab es trotzdem Gewitter.
- Nach dem automatischen Anhalten (Ereignisse geschehen um Mitternacht) wurde das Bild nachtdunkel. Tag und Nacht sind jetzt nur bei 1 Std/s zu sehen.

**Spielstände**
- Das Speicherformat steigt auf Version 2. **Spielstände aus 0.1 lassen sich weiter laden.** Sie starten ohne Buschland und Totholz; vorhandene Wälder bekommen ein geschätztes Alter (dichter Wald = älter), der tiefe Boden ist zu 60 % gefüllt. Die Chronik vermerkt die Übernahme.
- Gleiche Inseln entwickeln sich in 0.2 anders als in 0.1 – die Natur hat neue Regeln.

**Getestet** (alle auf Windows, macOS und Linux)
- Wald stirbt bei völliger Dürre langsamer als in 0.1 (Hälfte nach rund 310 statt 210 Tagen), hinterlässt Totholz, die Chronik meldet es.
- Ohne Samenquelle kommt kein Wald zurück; ein einzelner Samen kann einen neuen Wald begründen; in der Wüste geht kein Samen auf.
- Gras kehrt nach dem Regen zurück.
- Wasserbilanz geschlossen, auch bei 2500 mm Regen.
- Nasser Wald brennt nicht, ausgedörrter Wald mit Totholz großflächig.
- 500 Jahre Zeitraffer stabil.
- Quellen aus- und wieder einschalten nachvollziehbar.

## 0.1.1 – Ruhiges Fenster
- Nichts verrutscht mehr, während die Zeit läuft. Feld-Infos als Kasten über der Karte.
- Starkregen erst ab 40 mm im Inselmittel in der Chronik.

## 0.1.0 – Die Insel lebt ohne Menschen
- Erste Version: Insel, Wetter, Wasser, Gras, Wald, Chronik, Eingriffe.
