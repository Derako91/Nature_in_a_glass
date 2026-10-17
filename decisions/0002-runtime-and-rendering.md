# ADR 0002 – Laufzeit, Simulationskern und Darstellung

**Status:** Entwurf zur Diskussion (noch nicht entschieden)

## Kontext

Die Engine muss zwei Dinge gleichzeitig können, die sich normalerweise widersprechen:

1. **Tiefe Zeit:** Jahrhunderte bis Jahrtausende im Zeitraffer simulieren (Vegetation, Boden, später Geologie und Generationen).
2. **Nahaufnahme:** Jederzeit anhalten und einen einzelnen Bewohner Minute für Minute beobachten.

Weitere Anforderungen aus ADR 0001 und den Prinzipien: headless lauffähig (Tests, TheCave), deterministisch mit festem Seed, Speichern/Laden mit Versionierung, KI nur über Adapter, Darstellung strikt getrennt.

## Kernidee unabhängig von der Sprache: Zeit in mehreren Auflösungen

Keine einheitliche Tick-Rate für alles. Jedes System bekommt seinen eigenen Takt:

| System | Takt (Vorschlag) |
| --- | --- |
| Bewegung/Handlung eines Bewohners | Minuten |
| Bedürfnisse | ~10 Minuten |
| Wetter, Niederschlag | Stunden |
| Bodenfeuchte, Wasserabfluss | Stunden bis Tage |
| Vegetation | Tage |
| Geologie, Klima | Jahre |

Ohne Bewohner (M1) fällt die feinste Stufe weg – deshalb ist der Zeitraffer über 100 Jahre billig. Später werden Bereiche ohne Beobachter und ohne Bewohner grob weitergerechnet (*Level of Detail für Zeit*), beobachtete Bereiche fein. Ein Scheduler (Prioritätswarteschlange nach Simulationszeit) ruft die Systeme auf; „vorspulen“ heißt einfach, bis zum nächsten relevanten Zeitpunkt zu springen.

## Optionen

### A) Rust-Kern + Web-Oberfläche (WASM)
- Kern als Rust-Bibliothek, dazu ein CLI-Runner für headless Läufe und Benchmarks.
- Derselbe Kern nach WebAssembly kompiliert, Darstellung im Browser (TypeScript + Canvas/PixiJS).
- **Pro:** sehr schnell, Determinismus gut kontrollierbar, ein Kern für Browser, Desktop und Server, kein Spiele-Engine-Lock-in.
- **Contra:** steilste Lernkurve, langsamere Iteration am Anfang.

### B) TypeScript durchgängig
- Kern in TypeScript, headless unter Node, UI im Browser.
- **Pro:** schnellster Start, eine Sprache, sofort sichtbare Ergebnisse.
- **Contra:** Performance-Decke bei Tausenden Jahren × großer Karte × vielen Bewohnern; Gleitkomma-Determinismus über Plattformen schwerer abzusichern.

### C) Godot (C# oder GDScript)
- **Pro:** Rendering, Kamera, UI und Szenen fertig; angenehm für ein „Glas“ zum Anschauen.
- **Contra:** Simulation rückt nah an die Engine; headless-Betrieb und reine Kerntests sind umständlicher; widerspricht ein Stück weit der Trennung Engine/Darstellung.

### D) Python-Prototyp
- **Pro:** ideal zum Ausprobieren von Ökologie-Formeln (NumPy), Plots für Auswertungen.
- **Contra:** Agentenlogik wird früh zu langsam; später Neuschreiben wahrscheinlich.

## Empfehlung (Vorschlag)

**Option A**, mit pragmatischem Einstieg:

- Rust-Kern (`core`, `world`) + CLI-Runner ab M1. Die Insel kann damit schon im Terminal 100 Jahre laufen und Kennzahlen ausgeben, bevor es eine Grafik gibt.
- Erste Darstellung als einfache Browser-Seite über WASM, die den Zustand liest und die Zeit steuert.
- Determinismus: eigener Seed-RNG (z. B. PCG) mit getrennten Strömen pro System, Weltzustand möglichst in Ganzzahlen/Festkomma, feste Iterationsreihenfolge.
- Speichern: Snapshot als versioniertes Binärformat (z. B. MessagePack) plus Ereignisjournal; SQLite erst, wenn Abfragen auf die Chronik nötig werden.
- Python darf parallel als Auswertungswerkzeug für Läufe dienen, nicht als Engine.

Falls schneller Fortschritt wichtiger ist als Langlebigkeit: **Option B**, mit klar gekapseltem Kern, damit ein späterer Umstieg auf Rust Modul für Modul möglich bleibt.

## Offene Fragen an Dennis

1. Wie wichtig ist dir, selbst im Code mitzuarbeiten, und in welcher Sprache fühlst du dich am wohlsten?
2. Soll das Glas im Browser laufen (auch von TheCave ausgeliefert) oder als Desktop-Anwendung?
3. Pixel-/Rasteroptik in 2D genügt für M1–M3, oder schwebt dir früh eine 3D-Darstellung vor?
