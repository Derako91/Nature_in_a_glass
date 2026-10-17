# Änderungen

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
