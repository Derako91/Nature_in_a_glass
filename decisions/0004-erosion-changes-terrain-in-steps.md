# ADR 0004 – Erosion verändert das Gelände in gesammelten Schritten

**Status:** angenommen (2026-10-17)

## Kontext

Mit v0.3.0 verändert die Simulation erstmals selbst das Gelände: Wasser trägt Boden ab und lagert ihn anderswo ab. Die täglichen Beträge sind winzig (oft Millionstel Meter), die Geländehöhen dagegen bis über 450 m. Eine `f32`-Zahl um 450 m kann sich nur in Schritten von etwa 0,03 mm ändern. Tägliche Änderungen direkt auf die Höhe zu addieren, hätte sie zum großen Teil verschluckt – langsamer Abtrag unter Wald wäre verschwunden und die Bodenbilanz offen geblieben.

Außerdem muss das Gelände auf allen Plattformen bitgenau gleich bleiben (Referenz-Prüfsumme), und Speichern/Laden mitten im Jahr darf den Verlauf nicht ändern.

## Entscheidung

- Jedes Feld sammelt Änderungen in eigenen kleinen Zahlen (`soil_change` für Boden, `rock_change` für Gestein). Erst wenn sie zusammen mindestens 1 mm erreichen, werden sie zum Jahreswechsel in `elevation` und `soil_depth` übernommen.
- Alle Rechnungen, die die Bodendicke brauchen (z. B. „Wie viel Boden ist noch da?“), nehmen Dicke plus gesammelte Änderung.
- Die Bilanz wird über `elevation + soil_change + rock_change` aller Felder geprüft (`erosion::total_ground`, Test `sediment_balance_is_closed`).
- Sedimente folgen dem Hauptabfluss (`downstream`) in derselben Reihenfolge wie das Wasser (vom höchsten zum tiefsten Wasserspiegel); die Reihenfolge gibt `hydro::step_day` heraus.
- Land sinkt durch Erosion nie unter 0,5 m (wie beim Geländeeingriff); Meer wird erst ab 0,3 m Ablagerung über dem Meeresspiegel zu Land.
- Die Sammelwerte gehören zum Spielstand (Format 5) und zur Prüfsumme.

## Konsequenzen

- Das Gelände springt nie sichtbar; Höhenänderungen kommen in Schritten ab 1 mm.
- Langsamer Abtrag geht nicht verloren; die Bodenbilanz ist bis auf Rundung geschlossen (unter 1 % des bewegten Bodens).
- Das Wasserhaltevermögen eines Feldes folgt einmal im Jahr der Bodendicke. Wasser, das ein dünner gewordener Boden nicht mehr hält, tritt an die Oberfläche – die Wasserbilanz bleibt geschlossen.
- Nebenbefund: Schrumpfende Werte (z. B. Gras auf nacktem Fels) blieben als „subnormale“ Gleitkommazahlen hängen und verlangsamten lange Läufe. Der Kern setzt solche Restwerte unter 10⁻³⁰ monatlich auf null (`World::flush_tiny`).
