# Änderungen

## 0.22.1 – Einbäume auf dem Meer

**Gestaltung** (Codex, PR #26)
- Fischer im Einbaum sind jetzt draußen auf dem Meer zu sehen, Jäger mit Speer, erlegte Hirsche mit eigenem Bild – in allen fünf Aussehen und beiden Blickrichtungen.

**Verbessert**
- Liegen mehrere Einbäume auf demselben Meerfeld, bekommen sie feste Plätze nebeneinander statt sich zu überdecken.

## 0.22.0 – Einbaum und Jäger zum Ansehen

**Neu** (Anschlüsse für Codex' Bilder, PR #26)
- Wer mit dem Einbaum fischt, wird künftig draußen auf dem Meer gezeigt statt am Ufer.
- Wer an einem Tag jagt, bekommt das Jagdbild mit Speer.
- Von Menschen erlegte Hirsche sind als solche zu erkennen und liegen auch dann kurz sichtbar da, wenn alles Fleisch mitgenommen wurde.
- Die Bilder selbst bringt Codex als Nächstes ins Spiel; bis dahin sieht man wie bisher die Fischer am Ufer.

**Spielstände:** Format 20; ältere lassen sich laden.

## 0.21.1 – Oberfläche im Seekartenstil

**Gestaltung** (Codex, PR #28)
- Das ganze Beobachterfenster bekommt den Stil der Inselnamen: dunkles Blaugrün, warme Schrift, Messingtöne, kursive Überschriften.
- Zahlen stehen in festen Spalten mit gleichbreiter Schrift; nichts verrutscht, wenn sich Werte ändern.
- Kompaktere rechte Spalte: Auch mit acht Quellen bleibt Platz für die Chronik. Bewohner-Fenster mit festen Zeilen.
- Bedienung und Funktionen unverändert.

## 0.21.0 – Das Boot kommt

**Neu**
- **Angekündigtes Boot:** Eine neue Insel beginnt leer, oben rechts läuft ein Zähler „Boot in 9 J. 11 Mon.“ in Spielzeit. Nach zehn Jahren – die Insel ist dann ergrünt – landet von selbst ein Boot mit fünf Menschen. Schnellerer Zeitraffer verkürzt das Warten: bei „1 Monat/s“ etwa zwei Minuten, bei „1 Jahr/s“ zehn Sekunden.
- **Zur Ankunft:** Ein Klick auf den Zähler spult im schnellsten Zeitraffer bis zur Landung und kehrt dann ins alte Tempo zurück; die Kamera springt zum Landeplatz.
- **Haken „Menschen kommen von selbst“** neben „Neue Insel“ (standardmäßig an). Aus: Die Insel bleibt ohne Menschen – für lange Naturläufe.
- In der Inselwelt landet das Boot (auch „Boot landen lassen“ ohne gewähltes Feld) an der größten Insel.
- Startoptionen wie `--years`, `--menschen`, `--tiere` starten wie bisher ohne angekündigtes Boot.

**Spielstände:** Format 19; der Zähler läuft nach dem Laden weiter. Ältere Stände lassen sich laden.

## 0.20.0 – Überfahrten

**Neu** (zweite Etappe zur ganzen Welt, ADR 0012)
- **Land am Horizont:** Wer einen Einbaum bauen kann und am Ufer steht, sieht bei ruhigem Wetter manchmal eine andere Insel (bis etwa 30 Felder übers Meer). Die Biografie vermerkt es, das Bewohner-Fenster zeigt „Hat gesehen: …“.
- **Überfahrt:** Wer eine andere Insel kennt, jung genug ist und einen Partner in der Nähe hat, fährt bei ruhigem Wetter mit ihm und den Kindern hinüber – oft, wenn das Lager zu voll ist, Holz fehlt oder Hunger herrscht, selten aus Neugier. Mindestens acht Menschen bleiben auf der Heimatinsel zurück. Ziel ist die gesehene Insel mit den wenigsten Menschen.
- **Neubeginn:** Auf der neuen Insel kennt man nur das Ufer, an dem man gelandet ist: keine Wasserstelle, kein Lager. Können und Werkzeugwissen reisen mit.
- **Gefahr:** Je weiter die Fahrt, desto eher kentert das Boot.
- Chronik: „Zum ersten Mal betreten Menschen …“ und weitere Überfahrten (wichtiges Ereignis). Auf einer einzelnen Insel ändert sich nichts.

**Behoben**
- Neuere Rust-Versionen meldeten eine Prüfwarnung im Kern (Seen-Weitblick); umgeschrieben, gleiches Verhalten.

**Spielstände** aus 0.19 lassen sich laden (Format 18).

**Gestaltung:** Inselnamen in Seekartenschrift (Codex, PR #27).

## 0.19.0 – Die Inselwelt

**Neu** (erste Etappe zur ganzen Welt, ADR 0012)
- **Inselwelt:** In der Größenauswahl neben „Neue Insel“ gibt es jetzt „Inselwelt“ – eine Welt aus fünf bis sechs Inseln verschiedener Größe (zusammen etwa 12–17 km²), jede mit eigenem Gelände, eigenen Bächen und Seen. Pflanzen und Tiere kommen nicht über das Meer.
- **Inseln haben Namen** – „Lerchenland“, „Möweneiland“, „Sonnenwerder“ … Die Namen stehen auf der Karte und im Überblick, im Feld-Kasten beim Gelände, und die Chronik nennt beim Start alle Inseln.
- **Eingriffe je Insel:** Wähle ein Feld auf einer Insel, dann landet das Boot oder werden die Tiere genau dort ausgesetzt.
- Startoption `--groesse inselwelt`.

**Spielstände** aus 0.18 lassen sich laden; ihre Insel bekommt beim Laden einen Namen.

## 0.18.1 – Werkzeuge in der Hand

**Neu** (Pull Request #25)
- **Codex' Werkzeugbilder:** Wer ein Netz kennt, fischt mit dem Netz statt mit dem Speer; wer einen Korb hat, sammelt und isst mit Korb; wer ein Steinbeil kann, trägt es beim Holzholen im Gürtel – bei allen fünf Bewohnern, in beide Richtungen.

## 0.18.0 – Größere Inseln

**Neu**
- **Inselgröße wählbar:** Neben „Neue Insel“ gibt es jetzt die Auswahl klein (wie bisher, etwa 4 km²), mittel (etwa 9 km²) und groß (etwa 14 km²). Größere Inseln haben höhere Berge, mehr Quellen und Seen, oft kleine Nebeninseln vor der Küste – und Platz für mehr Tiere und Dörfer. Sie rechnen langsamer (groß: etwa fünfmal so viel wie klein).
- **Die Natur wächst mit:** Quellen sowie Reviere von Wölfen und Füchsen richten sich nach der Fläche. Auf der kleinen Insel bleibt alles genau wie bisher.
- **Startansicht** zeigt immer die ganze Insel, egal wie groß.
- Startoption `--groesse klein|mittel|gross`, für `nig-sim` `--size 64|96|128`.

## 0.17.0 – Seltene Beute ist scheu

**Geändert**
- **Räuber rotten ihre Beute nicht mehr so leicht aus:** Werden Hirsche oder Kaninchen selten, ziehen sie sich in Dickicht, Baue und steiles Gelände zurück – Wölfe und Füchse finden sie dann kaum noch. Die letzten Tiere überleben eher als ihre Jäger. Vorher fraßen Wölfe die Hirsche oft in wenigen Jahren restlos auf und verhungerten danach selbst; Füchse fraßen frisch ausgesetzte Kaninchen sofort weg.
- **Ergebnis im Langlauf** (8 Inseln, 100 Jahre): Setzt man alle vier Arten zusammen aus, überleben jetzt überall Hirsche und Kaninchen (vorher Hirsche auf 3, Kaninchen auf keiner Insel). Räuber verhungern dann allerdings, weil zu Beginn zu wenig Beute da ist.
- **Hinweis an den Knöpfen „Tiere aussetzen“:** Erst die Beute aussetzen und einige Jahre warten, dann die Räuber. Füchse halten sich so jahrzehntelang; ein Wolfsrudel hält sich auf der kleinen Insel meist nur einige Jahre.

## 0.16.1 – Flüssig auch mit großen Dörfern

**Verbessert**
- **Das Fenster hängt nicht mehr:** Mit vielen Bewohnern kann ein Spieljahr fast eine Sekunde Rechenzeit kosten. Bisher rechnete das Spiel bei hoher Geschwindigkeit bis zu ein Jahr am Stück und das Fenster stand so lange still. Jetzt rechnet es tageweise und lässt nach höchstens 25 Millisekunden das Fenster wieder zum Zug kommen. Bei sehr vielen Menschen läuft die Zeit dann langsamer als eingestellt (bei 150 Menschen etwa ein halbes Jahr pro Sekunde), aber das Bild bleibt ruhig und reagiert.
- **Etwas schneller:** Wegsuchen werden je Ort und Tag nur einmal gerechnet, Seen einmal am Tag aufgelistet, Sammelplätze schneller verglichen – rund 12 % weniger Rechenzeit, bei genau gleichem Verhalten der Menschen.

## 0.16.0 – Der Einbaum

**Neu**
- **Einbaum als fünftes Werkzeug:** Wer geschickt fischt und Netz und Steinbeil kennt, höhlt an der Küste irgendwann mit Feuer und Beil einen Baumstamm aus. Mit dem Einbaum fährt man zum Fischen aufs Meer: Dort gibt es mehr Fisch, und er wird nicht so schnell weniger wie am Ufer.
- **Gefahr auf dem Meer:** Bei starkem Wind kann ein Boot kentern – die Chronik meldet dann, wer beim Fischen ertrunken ist.
- Wie die anderen Werkzeuge wird der Einbaum abgeschaut, oder das Wissen geht verloren.
- **Fenster „Bewohner“:** Rolle und Können stehen in einer festen Zeile; der vollständige Text erscheint beim Darüberfahren.

**Spielstände** aus 0.15 lassen sich laden.

## 0.15.0 – Tongefäße und Rollen

**Neu**
- **Tongefäß als viertes Werkzeug:** Wer an einem Lager mit Garten und gefülltem Vorrat lebt, formt irgendwann aus Lehm vom Ufer ein Gefäß und brennt es im Feuer. In Gefäßen verderben Vorräte nur halb so schnell. Wie bei den anderen Werkzeugen geht das Wissen weiter – oder mit den Toten verloren.
- **Rollen im Lager:** Einmal im Monat zeigt sich in jedem Lager, wer am besten fischt, gärtnert und jagt (ab 70 % Können). Diese Personen gelten als Fischerin, Gärtner, Jägerin … – je Rolle eine, jede mit höchstens einer Rolle. Niemand vergibt das; es ergibt sich aus dem Können und wechselt, wenn jemand stirbt oder überholt wird.
- **Rollen wirken:** Fischer fangen mehr, Gärtnerinnen pflegen die Gärten besser, Jäger ziehen schon bei kleinem Hunger los.
- **Fenster „Bewohner“** zeigt die Rolle; die Lebensgeschichte erzählt, wann jemand zur besten Fischerin am Lager wurde.

**Spielstände** aus 0.14 lassen sich laden.

## 0.14.0 – Jagd

**Neu**
- **Die Menschen jagen:** Wer gesund ist und Hunger hat, jagt, wenn Wild in der Nähe ist – Hirsche zu mehreren, Kaninchen allein. Jagen wird mit Übung besser und wird abgeschaut. Ein Hirsch bringt Fleisch für Tage; was die Jäger nicht tragen, kommt in den Vorrat ihres Lagers oder bleibt für andere liegen. Die Chronik meldet den ersten erlegten Hirsch.
- **Folgen für das Wild:** Die Jagd hält die Hirsche klein – oft kleiner, als sie ohne Menschen wären, wo sie sich vermehren, bis viele verhungern. Auf manchen Inseln jagen die Menschen die Hirsche ganz aus; die Chronik nennt dann die Jagd als Ursache („Menschen haben 548 erlegt“). Danach fehlt das Fleisch, und auch den Wölfen die Beute.
- Ein Hirsch kann sich wehren: Selten wird ein Jäger verletzt.
- **Fenster „Bewohner“** zeigt auch das Können beim Jagen.

**Ergebnis:** Im Langlauf mit Hirschen und Kaninchen (16 Inseln, 120 Jahre) leben am Ende zusammen rund 490 Menschen, auf 5 Inseln sind die Hirsche ausgejagt.

**Spielstände** aus 0.13 lassen sich laden.

## 0.13.0 – Werkzeuge und Umzüge

**Neu**
- **Werkzeuge entstehen aus dem Tun:** Wer oft und geschickt fischt, knüpft irgendwann ein **Netz** und fängt damit viel mehr. Wer oft mit vollen Händen vom Sammeln kommt, flicht einen **Korb** – man sammelt mehr und trägt mehr. Wer oft Holz holt, schlägt sich ein **Steinbeil** und schafft doppelt so viel. Die Chronik meldet jede Erfindung als wichtiges Ereignis.
- **Wissen geht weiter – oder verloren:** Wer mit Kundigen zusammen ist, lernt es mit der Zeit, Kinder schneller. Sterben alle, die es konnten, ist es verloren, bis es jemand neu erfindet.
- **Ganze Lager ziehen um:** Versiegt die Wasserstelle am Lager für zwei Monate, oder fehlt einer kleinen Gruppe ein halbes Jahr lang Holz, ziehen alle zu einer guten Wasserstelle, die jemand von ihnen kennt.
- **Fenster „Bewohner“** zeigt, was jemand kann: Fischen, Gärtnern und die Werkzeuge.

**Behoben**
- **Verdursten an ausgetrockneten Teichen (zweite Ursache):** In Dürren hielten die Menschen flache Teiche weiter für sichere Wasserstellen und gingen dorthin statt zum fließenden Bach – und verdursteten. Jetzt gelten flache Teiche als weniger verlässlich, Ausgetrocknetes in Sichtweite wird gestrichen, und sehr Durstige gehen zur nächsten bekannten Stelle.

**Ergebnis:** Im Langlauf (16 Inseln, 120 Jahre) überleben jetzt 13 Gruppen mit zusammen rund 410 Menschen.

**Spielstände** aus 0.12 lassen sich laden.

## 0.12.0 – Dörfer

**Neu**
- **Dörfer entstehen von selbst:** Liegen mindestens drei bewohnte Lager mit zusammen zehn Menschen nah beieinander, gelten sie als Dorf. Es bekommt einen Namen aus seiner Umgebung – „Möwenbucht“ an der Küste, „Reihersee“ an einem See, „Haselhain“ im Wald, „Felshöhe“ in den Bergen, „Kleeau“ auf der Wiese. Die Chronik meldet das erste Dorf der Insel als wichtiges Ereignis.
- **Nachbarn helfen sich:** Hungert ein Lager, bringen Lager mit vollem Vorrat aus demselben Dorf etwas vorbei.
- **Partner aus dem Nachbarlager:** Junge Leute finden auch in anderen Lagern ihres Dorfes einen Partner, nicht nur bei denen, mit denen sie täglich zusammen sind.
- **Ein Dorf hält sich:** Es bleibt ein Dorf, solange noch zwei bewohnte Lager mit sechs Menschen da sind. Zerfällt es und entsteht später am selben Ort neu, bekommt es seinen alten Namen zurück.
- **Anzeige:** Dorfnamen über den Lagern (bis Zoom 8) und auf der Karte, im Feld-Kasten beim Lager und im Insel-Überblick.
- **Mehr Platz für Bilder:** Der Bildspeicher wächst bei Bedarf in die Breite (bis 8192 × 8192).

**Spielstände** aus 0.11 lassen sich laden.

## 0.11.1 – Wasserfälle im neuen Stil

**Neu** (Pull Request #24)
- **Codex' Bilder für Wasser an Geländestufen:** Bäche laufen über eine Abbruchkante, die Felswand hinunter und mit einem kleinen Aufprall unten weiter, in Bewegung bei 1 Std/s. Trockene Betten zeigen eine ausgewaschene Felsrinne.
- Die Bilder werden enger in den Grafikspeicher gepackt, damit alles weiter in eine Textur passt.

## 0.11.0 – Lager teilen sich

**Neu**
- **Aufbruch zu einem neuen Lager:** Wird ein Lager zu eng (alle vier Hütten voll und mehr als 20 Menschen) oder gibt es in der Umgebung seit Monaten kein Holz mehr, bricht mit etwas Zufall ein junges Paar mit seinen Kindern auf. Ziel ist eine gute Wasserstelle, die die Frau kennt, nicht zu nah und nicht zu weit. Dort legt es ein eigenes Lager an. Kleine Gruppen teilen sich wegen Holzmangel nicht.
- **Die ersten Dörfer:** Auf wasserreichen Inseln wachsen so über Generationen mehrere Lager mit ihren Gärten nebeneinander – auf Insel 42 nach 130 Jahren acht Lager mit gut 40 Menschen.
- **Chronik und Biografien** erzählen, wer warum aufbricht und wohin. Wer nach langer Suche keinen Platz findet, gibt auf.

**Spielstände** aus 0.10 lassen sich laden.

## 0.10.3 – Gärten im neuen Stil

**Neu** (Pull Request #23)
- **Codex' Gartenbilder:** umgegrabene Erde im Winter, helle Keimlinge im Frühjahr, grüne Reihen im Sommer, goldene reife Beete im Herbst und Stoppeln nach der Ernte – je in vier Varianten.

**Behoben**
- **Ernte verdarb auf dem Feld:** Im Sommer füllten Gesammeltes den Vorrat so sehr, dass im Herbst kein Platz für die Ernte blieb. Jetzt fasst jede Hütte zusätzlichen Vorrat, und Gesammeltes belegt höchstens die Hälfte.

## 0.10.2 – Wasser fällt über Stufen

**Neu**
- **Kleine Wasserfälle:** Wo ein Bach über eine Geländestufe nach vorn fließt, sieht man das Wasser jetzt die Felswand hinunterlaufen, bei trockenem Bett eine ausgewaschene Rinne. Vorerst als Streifen; Codex' Bilder folgen.

## 0.10.1 – Fischen mit dem Speer

**Neu** (Pull Request #22)
- **Fischen ist zu sehen:** Wer an einem Tag vor allem gefischt hat, steht mit dem Speer am Ufer – bei allen fünf Bewohnern, in beide Richtungen, mit eigener Bewegung. Das kommt vor allem im Winter vor, wenn es wenig zu sammeln gibt. Abends essen sie wie nach dem Sammeln.

## 0.10.0 – Gärten

**Neu**
- **Die Menschen beginnen mit dem Anbau – von selbst:** Wer schon länger an einem Lager lebt, steckt irgendwann im Frühjahr zum ersten Mal Knollen und Samen in den Boden. Daraus wird ein Garten neben dem Lager; die Chronik meldet den ersten Garten der Insel als wichtiges Ereignis.
- **Ein Jahr im Garten:** Aussaat im Frühjahr, Wachstum im Sommer – nur bei feuchtem Boden, Wärme und wenn jemand jätet –, Ernte im Herbst in den Vorrat des Lagers. In Dürrejahren fällt die Ernte schlecht aus.
- **Gärtnern lernt man:** wie das Fischen durch Übung und durch Abschauen bei Erfahrenen. Wächst das Lager, kommen weitere Gärten dazu.
- **Folgen für die Natur:** Für einen Garten wird ein Stück Land gerodet. Wird er zwei Jahre nicht gepflegt, verwildert er, und es wächst wieder Wiese oder Wald.
- **Vorräte halten länger** (Knollen und Samen lassen sich lagern); ein Lager fasst mehr.
- **Holz von weiter her:** Ist um ein Lager alles abgeholzt, holen die Menschen Holz aus bis zu sechs Feldern Entfernung (gefunden in Fables Probelauf).
- **Anzeige:** Gärten im Bild (vorerst als umgegrabene Erde, bis Codex' Bilder kommen), im Feld-Kasten mit Zustand und Ernte, im Insel-Überblick mit der Zahl der Gärten.

**Behoben**
- Gärten verwilderten nicht, wenn niemand mehr auf der Insel lebte.

**Spielstände** aus 0.9 lassen sich laden.

## 0.9.1 – Bäche und Bachbetten

**Neu** (Pull Request #21)
- **Bäche als durchgehende Läufe:** Codex' Bachstücke ersetzen die blauen Linien – Rinnsale, Bäche und große Bäche schließen von Feld zu Feld nahtlos an, auch an Verzweigungen.
- **Trockene Bachbetten** zeigen Kies und ausgewaschene Erde; nach Regen füllen sie sich wieder mit Wasser.

## 0.9.0 – Fischfang

**Neu**
- **Die Menschen lernen zu fischen:** Wer am Meer, an einem See oder an einem größeren Bach sammelt, fängt auch Fisch – umso mehr, je geübter er ist. Wer mit dem Boot kommt, kann schon ein wenig; Übung macht geschickter. Die Chronik meldet, wenn zum ersten Mal jemand ein geschickter Fischer ist.
- **Können geht weiter:** Kinder und Erwachsene schauen sich das Fischen bei Geübteren ab. Stirbt der letzte Geübte, geht das Können verloren.
- Befischte Stellen geben eine Weile weniger her und erholen sich langsam.
- Paare finden sich etwas leichter, auch mit größerem Altersunterschied.

**Behoben**
- **Verdursten an ausgetrockneten Teichen:** Wer Durst hatte und zu einer Wasserstelle ging, die inzwischen ausgetrocknet war, blieb dort stehen und verdurstete – mitsamt den Kindern –, obwohl in der Nähe ein Bach Wasser führte. Jetzt sehen sich Durstige jeden Tag um, streichen trockene Stellen und suchen weiter.

**Ergebnis:** Vorher starb jede Gruppe nach 80 bis 140 Jahren aus. Jetzt bleibt auf etwa der Hälfte der Inseln eine Gruppe von 10 bis 50 Menschen über Generationen; Hungerjahre begrenzen ihr Wachstum. Ein einzelnes Boot kann weiterhin scheitern – ein zweites hilft.

**Spielstände** aus 0.8 lassen sich laden; die Menschen dort fangen mit wenig Können an.

## 0.8.9 – Wasser im neuen Stil

**Neu** (Pull Request #20)
- **Meer, Flachwasser und Teiche im neuen Stil**, je in vier Varianten, damit die Wasserflächen nicht gekachelt wirken.
- **Das Wasser bewegt sich:** Bei 1 Std/s kräuselt es sich langsam. Bei schnellerer Zeit bleibt es ruhig.

## 0.8.8 – Trockene Bachbetten

**Neu**
- **Bachbetten bleiben sichtbar:** Wo ein Bach in trockenen Zeiten versiegt, sieht man weiter sein Bett als helle Kieslinie. Wenn es wieder regnet, füllt es sich mit Wasser.
- Vorbereitet für Codex' Bachbilder: Das Spiel weiß jetzt für jedes Feld, wohin der Bach weiterläuft, und setzt die Stücke zusammen, sobald es sie gibt.

## 0.8.7 – Tiere im neuen Stil

**Neu** (Pull Request #19)
- **Hirsche, Kitze, Kaninchen, Wölfe und Füchse im neuen Stil** – in allen Haltungen (grasen, gehen, rennen, ruhen), beide Richtungen, dazu Kadaver und Spuren.
- Eigene Fassungen für die Entfernung, damit die Tiere auch bei Zoom 4 und 8 gut zu erkennen sind.

## 0.8.6 – Landschaft im neuen Stil

**Neu** (Pull Requests #17 und #18)
- **Böden, Felswände, Bäume und Büsche im neuen Stil** – passend zu Bewohnern, Hütten und Fuchs. In der Nahansicht sieht man einzelne Nadelbäume, Büsche und Bodenstruktur.
- **Aus der Entfernung bleibt der Wald lesbar:** Bei Zoom 4 und 8 zeigt das Spiel eigene Fassungen mit breiten Kronen, sodass Wald als zusammenhängendes Grün erscheint.

## 0.8.5 – Stimmige Größen am Lager

**Geändert** (Pull Request #16)
- Erwachsene sind etwa 10 % kleiner, Kinder bleiben gleich – das Verhältnis zwischen Groß und Klein wirkt natürlicher.
- Beim Sammeln hocken die Bewohner kompakter und wirken nicht mehr wuchtig.
- Die Hütteneingänge sind höher und breiter: Man kann gebückt hineingehen.

## 0.8.4 – Bewohner im neuen Stil

**Neu**
- **Alle Bewohner im neuen Stil:** Codex hat sämtliche Menschenbilder neu gezeichnet – Erwachsene, Kinder und Tote, alle Tätigkeiten, beide Richtungen. Sie passen jetzt zu den Hütten, zum Holztragen und zum Fuchs. Aussehen und Erkennungsfarben der fünf Bewohner bleiben gleich (Pull Request #15).
- **Größere Feuerstelle** am Lager, sie ist neben den Menschen besser zu erkennen (Pull Request #14).

## 0.8.3 – Hütten in richtiger Form

**Behoben**
- Die Hütten wirkten in die Höhe gestreckt. Codex hat sie neu zugeschnitten: Spitzhütte und Kuppelhütte haben jetzt ihre natürliche, breitere Form (Pull Request #13).

## 0.8.2 – Lager in Codex' neuem Stil

**Neu**
- **Codex' Lagerbilder sind im Spiel:** Feuerstelle (kalt und brennend), Windschutz aus Ästen, zwei Hüttenformen (Spitzhütte und Kuppelhütte) und das verfallene Lager – im neuen, plastischeren Stil.
- **Holz holen ist zu sehen:** Wer Holz für das Lager holt oder baut, hat dafür ein eigenes Bild, bei allen fünf Bewohnern und in beide Richtungen.
- Auf dem Lagerplatz selbst ist freigeräumt: Bäume und Büsche verdecken die Hütten nicht mehr.

## 0.8.1 – Echte Flammen

**Neu**
- **Codex' Feuerbilder sind im Spiel:** Brennende Felder zeigen flackernde Flammen – kleine über Gras und Busch, große im Wald und im Totholz – und einen glühenden Saum am Boden statt der bisherigen orangen Fläche. Bei 1 Std/s flackert jede Flamme in ihrem eigenen Takt.
- Rauch kommt später (Codex überarbeitet ihn noch).

## 0.8.0 – Lagerplätze

**Neu**
- **Die Menschen werden sesshaft – von selbst:** Wer viele Nächte an derselben verlässlichen Wasserstelle verbringt, legt dort einen Lagerplatz an: eine Feuerstelle mit Windschutz aus Ästen. Niemand gibt einen Auftrag; ob und wo ein Lager entsteht, hängt vom Wasser und vom Verhalten der Menschen ab.
- **Hütten:** Wer am Lager ist und gerade nichts Dringendes zu tun hat, holt Holz aus dem Wald in der Nähe. Reicht es, entsteht eine Hütte aus Stangen, Ästen und Rinde – eine für je fünf Menschen, höchstens vier je Lager.
- **Das Holz fehlt im Wald:** Rund um ein Lager lichtet sich der Wald über die Jahre, zuerst verschwindet das Totholz.
- **Schutz und Heimat:** Windschutz, Hütte und Feuer halten nachts die Kälte ab. Ohne Not kehren die Menschen zu ihrem Lager zurück und sammeln lieber in seiner Nähe. Überschüssige Nahrung wird im Lager abgelegt und verdirbt langsam.
- **Lagerfeuer können Brände auslösen:** In trockenen Sommern und bei Wind springen manchmal Funken über.
- **Verfall:** Kommt zwei Jahre niemand mehr, ist ein Lager verlassen; nach einigen Jahren ist nichts mehr davon zu sehen. Ein verlassenes Lager kann wieder bezogen werden.
- **Chronik:** erster Lagerplatz der Insel (wichtiges Ereignis), neue Hütten, verlassene und verschwundene Lager. Die Biografien erzählen, wer ein Lager angelegt, mitgebaut oder es verlassen hat.
- **Anzeige:** Lager im Bild (vorerst Platzhalter, bis Codex' Bilder kommen), auf der Karte als braunes Quadrat; im Feld-Kasten „Lager“ mit Hütten, Holz und Vorrat; im Insel-Überblick die Zahl der Lagerplätze und Hütten.

**Spielstände** aus 0.7 lassen sich laden; die Menschen dort haben noch kein Lager und legen eines an, wenn sie lange genug an einem Ort bleiben.

## 0.7.4 – Bilder mit Rand in richtiger Größe

**Behoben**
- Neue Bilder, die ihr Motiv mit einem durchsichtigen Rand liefern, geben ihr vergrößertes Maß nicht mehr an alte Bilder derselben Tierart weiter. Vorher erschienen die alten Fuchsbilder neben einem neuen Fressbild gut doppelt so groß.

**Übernommen**
- Codex' Ausgabewerkzeug berücksichtigt jetzt durchsichtige Ränder: Das Motiv selbst bekommt das vorgesehene Weltmaß (Pull Request #8).

## 0.7.3 – Korrektur bei teilweise ersetzten Bildern

**Behoben**
- Werden nur einzelne Bilder einer Tierart durch feinere ersetzt (zum Beispiel beim Prüfen neuer Bilder mit `--bilder`), erscheinen die übrigen alten Bilder jetzt weiter in richtiger Größe statt viermal zu klein.

## 0.7.2 – Detailstufen für feine Bilder

**Neu**
- **Detailstufen:** Ein Bild kann jetzt in mehreren Fassungen vorliegen – fein für die Nahansicht, vereinfacht für den Abstand. Das Spiel wählt beim Zoomen selbst die passende; Größe und Standort bleiben dabei gleich.
- Sehr feine Bilder (wie der neue Fuchs oder die Menschen) werden nicht mehr grob verkleinert: Das Spiel rechnet verkleinerte Fassungen vor, sodass nichts flimmert oder zu Pixelbrei zerfällt. Bestehende Pixelgrafik bleibt unverändert scharf.
- **Neue Bilder ansehen ohne Neubau:** `--bilder <ordner>` lädt Bilder aus einem Ordner dazu. Damit kann Codex neue Bilder im echten Spiel prüfen. `--bei x,y` setzt Tiere an einem bestimmten Feld aus.
- Vorbereitet für Gebäude und Gegenstände (Ordner `buildings`, `objects`).

## 0.7.1 – Die Bewohner bekommen Gesichter

**Neu**
- **Codex' Menschenbilder sind im Spiel:** fünf unterschiedliche Bewohner – Haare, Kleidung, Statur und eine eigene Erkennungsfarbe – in allen Tätigkeiten: stehen, gehen, laufen, sammeln, essen, trinken, schlafen, verletzt hinken. Fein genug für die Nahansicht bis Zoom 32. Tote liegen entsättigt am Boden.
- Im Überblick zeigt Codex' Zeichen in der Erkennungsfarbe, wo jemand ist.
- Kinder erscheinen vorerst als kleinere Fassung des Erwachsenenbilds, bis es eigene Kinderbilder gibt.

## 0.7.0 – Familien

**Neu**
- **Paare und Kinder:** Erwachsene, die viel Zeit miteinander verbringen, werden ein Paar. Frauen werden schwanger, Kinder kommen zur Welt und werden zwei Jahre gestillt. Geburten sind gefährlich – für Mutter und Kind.
- **Kinder wachsen in der Familie auf:** Kleine Kinder sind immer bei der Mutter, ab acht sammeln sie mit, mit 14 sind sie erwachsen und gehen eigene Wege. Die Eltern teilen ihr Essen mit ihnen und hungern zur Not lieber selbst.
- **Wissen geht weiter:** Kinder lernen von den Erwachsenen, wo es Wasser und Muscheln gibt – sie wissen es dann schon, bevor sie selbst dort waren.
- **Krankheiten** treffen vor allem Säuglinge und Kleinkinder.
- **Generationen:** Auf wasserreichen Inseln wachsen über Jahrzehnte Gruppen von 20 bis 35 Menschen heran. Auf kargen Inseln kann eine Gruppe nach Hungerjahren so klein werden, dass die Jungen nur noch Geschwister um sich haben – dann stirbt sie aus, außer ein neues Boot kommt.
- **Fenster „Bewohner“** zeigt die Familie (Eltern, Partner, Kinder, Verstorbene mit †), Alter von Säuglingen in Monaten und ob jemand ein Kind erwartet. Im Insel-Überblick steht, wie viele Kinder leben. Die Chronik meldet Geburten; die erste Geburt auf der Insel ist ein wichtiges Ereignis.
- Wer mit dem Boot kommt, ist jetzt 16 bis 34 Jahre alt.

**Hinweis:** Mit vielen Menschen läuft die Simulation langsamer (mit 20 Menschen etwa 1,5 Jahre pro Sekunde). Spielstände aus 0.6 lassen sich laden.

## 0.6.0 – Die ersten Menschen

**Neu**
- **Boot landen lassen:** Neuer Eingriff unter „Menschen“. Fünf Menschen kommen an der Küste an – an einer zufälligen Stelle oder am gewählten Feld. Danach handeln sie selbst; du kannst ihnen nichts befehlen.
- **Bedürfnisse treiben sie an:** Sie trinken an Bächen, Teichen und Seen, sammeln Beeren, Nüsse, Wurzeln, Muscheln und Fisch, essen von toten Tieren, schlafen nachts und suchen im Winter Schutz im Wald.
- **Jeder weiß nur, was er gesehen hat:** Menschen merken sich Wasser- und Sammelstellen. Ein Rinnsal, das sie kennen, kann inzwischen versiegt sein – dann irren sie sich. Wer noch keine verlässliche Wasserstelle kennt, zieht los und erkundet.
- **Sie halten zusammen:** Gefährten bringen Verletzten Wasser, teilen Essen und bleiben bei ihnen.
- **Gefahren:** Durst, Hunger, Kälte, Stürze an steilen Stufen, hungrige Wölfe (wenn jemand allein ist), Feuer und das Alter. Tote liegen einige Tage dort, wo sie gestorben sind.
- **Lebensgeschichten:** Die Zeile „Menschen“ im Insel-Überblick öffnet das Fenster **Bewohner**: alle Lebenden und Verstorbenen mit ihrer Biografie – Ankunft, erste Wasserstelle, Winter, Stürze, Genesung, Verluste, Tod. Im Feld-Kasten sind die Namen der Menschen auf dem Feld anklickbar.
- **Ganz nah heranzoomen:** bis Zoom 32. Die Menschen erscheinen vorerst als einfache Figuren in ihrer Erkennungsfarbe, bis Codex' Bilder kommen; die Landschaft ist ganz nah noch grob gepixelt. Im Überblick und bei schneller Zeit zeigt eine kleine Raute, wo jemand ist.
- Bei 1 Std/s schlafen die Menschen nachts und gehen morgens los.
- Startoptionen `--menschen`, `--bewohner`, `--folgen`.

**Hinweis:** Nachwuchs gibt es noch nicht – die ersten Bewohner sterben irgendwann. Mit Menschen läuft die Simulation etwas langsamer (mit fünf Menschen etwa 6 Jahre pro Sekunde). Spielstände aus 0.4 lassen sich laden.

## 0.4.6 – Feinere Landschaft

**Neu**
- **Landschaft in doppelter Auflösung:** Böden, Felswände, Wasser, Bäume, Büsche und Totholz haben beim Heranzoomen mehr Feinheiten – Risse im Fels, Grashalme, Blattgruppen, Äste. Die Bäume bleiben dabei gleich groß.
- **Vier Varianten** je Boden und Pflanze: Wald und Wiese wirken nicht mehr gekachelt.

## 0.4.5 – Codex' neue Tiere

**Neu**
- **Neu gezeichnete Tiere:** Hirsch mit Geweih, gepunktetes Kalb, Wolf, Fuchs und Kaninchen in doppelter Auflösung – auch ganz nah (Zoom 8) scharf. Je zwei Fellfarben, fressen, gehen, laufen und ruhen.
- **Kadaver:** Gerissene oder verendete Tiere liegen einige Tage auf dem Feld.
- **Trittspuren:** Im Überblick und ab 1 Monat/s zeigen Spuren, wo Tiere leben – je mehr Tiere, desto deutlicher.

## 0.4.4 – Zoomstufen

**Neu**
- **Tiere größer:** Hirsche, Wölfe, Füchse und Kaninchen erscheinen in der Grafik etwa anderthalb- bis zweimal so groß wie bisher – gut erkennbar, aber immer noch niedriger als ein junger Baum. Bis Codex sie schärfer neu zeichnet, sind sie etwas pixelig.
- **Überblick:** Ist die Kamera weit weg (ganze Insel im Bild) oder läuft die Zeit ab 1 Monat/s, zeigt die Grafik keine einzelnen Tiere mehr, sondern einen Hauch Farbe, wo Tiere leben. Später ersetzen Codex' Trittspuren diesen Hauch.
- Kadaver erscheinen, sobald es die Bilder dazu gibt.
- In `docs/gestaltung.md` festgehalten (4.3): was man bei welchem Abstand erkennt, neue Tiergrößen, Tiere dürfen doppelt aufgelöst gezeichnet werden.

## 0.4.3 – Die Insel bekommt ihre Bilder

**Neu**
- **Grafik:** Die isometrische Ansicht zeigt jetzt Codex' gezeichnete Bilder – Boden, Gras, Busch- und Waldboden, Brandflächen, Felswände und Erdkanten, Wasser, Bäume in drei Altersstufen, Büsche, Totholz und die Tiere. Umschalten mit **Grafik** neben der Ebenenwahl; aus = die gewohnten Flächenfarben.
- Bei 1 Std/s laufen und grasen die Tiere sichtbar und gleiten von Feld zu Feld; schneller stehen sie still (nichts zappelt).
- Die 2D-Karte und die Analyse-Ebenen bleiben bewusst schlicht.
- Startoptionen `--flaechen` und `--zoom N`.

**Hinweis:** Das ist ein früher Stand. Kadaver, Spuren, Feuer und Herbstfarben fehlen noch; Codex arbeitet an der Darstellung weiter.


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
