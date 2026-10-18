# Hinweise für die Arbeit an diesem Repository

## Zusammenarbeit

- Dennis ist Projektinhaber und Spieldesigner, programmiert aber nicht. Den Code schreibt Claude. Erklärungen an Dennis: auf Deutsch, ohne Fachjargon, mit Fokus darauf, was er im Spiel sieht.
- Spielbare Stände kommen als Download (GitHub Releases), nicht als Bauanleitung.
- **Arbeitsteilung:** Claude ist für Simulation und Technik zuständig (Wasser, Flussläufe, Tiere, Kern, Tests, Werkzeuge). Die **Gestaltung** – Texturen, Sprites, Tiles, Bildaufbereitung (z. B. aus ComfyUI) und deren Darstellung – übernimmt **Codex** nach `docs/gestaltung.md`. Claude ändert dort nur technische Schnittstellen (z. B. `crates/core/src/appearance.rs`, Simulationswerte), keine visuellen Vorgaben.
- **Codex-Pull-Requests** darf Claude selbst übernehmen (Squash), wenn die GitHub-Prüfungen grün sind und Claude den Stand im Spiel angesehen hat (Bildschirmfotos, z. B. Zoom 2 und 8). Freigabe von Dennis am 18.10.2026. Danach Version anheben und Dennis kurz berichten.
- Designgrundlagen: `docs/PRINCIPLES.md` (nicht verhandelbar), `docs/ROADMAP.md`, Entscheidungen in `decisions/`. Größere technische Weichenstellungen als neues ADR festhalten.

## Sprache

- Oberfläche, Chronik-Texte, Doku und Code-Kommentare: Deutsch.
- Bezeichner im Code: Englisch.

## Aufbau

- `crates/core` – Simulation. Darf keine Grafik-, Netzwerk- oder KI-Abhängigkeiten bekommen.
- `crates/app` – Desktop-Beobachter (eframe/egui 0.37; `App::ui`, `egui::Panel::left/right/top`).
- `crates/cli` – `nig-sim`, headless.

## Determinismus (wichtig)

- Im Kern nur `+ − × ÷`, `sqrt`, `abs`, `floor`, `min/max`. Keine `sin`, `cos`, `exp`, `ln`, `powf`, `powi` – deren Ergebnisse können sich zwischen Plattformen unterscheiden. Für Darstellung in `crates/app` ist alles erlaubt.
- Zufall nur über `rng::Rng` mit eigenem Strom pro Teilsystem (`Rng::stream(seed, "name")`).
- Keine `HashMap`-Iteration im Kern, feste Iterationsreihenfolge.
- Eingriffe des Beobachters nur über `Intervention`, damit sie in der Chronik landen. Niemals Bewohner direkt steuern (Prinzip 1).
- Ändert sich die Simulation absichtlich, schlägt `cross_platform_reference_hash` fehl. Dann neuen Wert mit `cargo run --release -p nig-sim -- --seed 42 --years 20` ermitteln und im Test eintragen.
- Spielstand-Struktur geändert → `SAVE_FORMAT_VERSION` erhöhen.

## Vor jedem Commit

```
cargo fmt --all
cargo clippy --release --all-targets -- -D warnings
cargo test --release --workspace
```

Oberfläche: Dennis möchte ein ruhiges Fenster – nichts darf beim Laufen der Zeit verrutschen. Leisten und Spalte haben feste Größen, Abschnitte immer dieselbe Zeilenzahl, wechselnde Zahlen mit fester Breite, nur die Chronik scrollt. Tag/Nacht nur bei 1 Std/s.

Visuelle Änderungen am Beobachter prüfen: `xvfb-run -s "-screen 0 1440x900x24"` + `import -window root` (ImageMagick); Startoptionen `--seed`, `--years`, `--view karte`, `--layer`, `--speed`, `--select x,y` helfen dabei; Stabilität mit `compare -metric AE -crop …` zwischen zwei Bildern prüfen.

## Release

Version in `Cargo.toml` (`workspace.package.version`) anheben und auf `main` pushen. `.github/workflows/release.yml` erkennt die neue Version, baut Windows/macOS/Linux und legt Release samt Tag `vX.Y.Z` selbst an. Tags nicht manuell pushen (der Git-Proxy der Cloud-Sitzungen lässt Tag-Pushes nicht durch).
