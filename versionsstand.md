# Versionsstand – Nature in a Glass

```yaml devhub-projekt
devhub: 1
projekt: nature-in-a-glass
titel: Nature in a Glass
auftrag: README.md
freigabe: dennis
```

Der bisherige Stand steht **unverändert** unter „Historie“. Offene Punkte von dort sind noch **nicht** in Pakete übertragen – das geschieht mit dem nächsten Arbeits-PR (eigenes Paket) oder per Planungs-PR von Dennis.

## Paket U – Umstellung auf Statuskopf

```yaml devhub-paket
id: U
titel: versionsstand.md auf DevHub-Statuskopf umstellen, PR-Prüfung einbinden
agent: claude
zustand: erledigt
naechster: keiner
haengt_ab_von: []
branch: plan/statuskopf
commit: c67bdcc80a3762a002d443aca983f6cab498d0e1
revier: [AGENTS.md, .gitattributes, CLAUDE.md]
pruefstufe: behauptet
tests: []
frage: null
aktualisiert: 2026-11-22T21:00Z
```

- **Problem:** Projekt war in der DevHub-Übersicht nicht sichtbar (Status nur als Freitext); PRs wurden nicht geprüft.
- **Funktioniert:** Statuskopf mit Projektblock und diesem Paket; bisheriger Inhalt unverändert als Historie; Regeln in AGENTS.md; `.gitattributes`. Keine automatische PR-Prüfung: öffentliches Repo, GitHub erlaubt keine Actions aus dem privaten Devhub (Lauf im PR schlug deshalb fehl, Workflow wieder entfernt). Geänderte Dateien: .gitattributes, AGENTS.md, CLAUDE.md, versionsstand.md. Kein Produktcode.
- **Geprüft:** Statuskopf mit dem DevHub-Validator und die PR-Prüfung lokal gegen diesen Branch (Cloud, 22.11.2026) – gültig. Die Prüfung im PR selbst läuft per GitHub Actions.
- **Offen:** Offene Punkte aus der Historie in Pakete übertragen (nächster Arbeits-PR oder Planungs-PR). Laufende PRs anderer Agenten brauchen beim nächsten Push ein eigenes Paket.
- **Nächster Schritt:** – (abgeschlossen)
- **Verantwortlich:** Claude → Dennis · **Branch/Commit:** siehe Block · **Tests:** keine (Dokumentation).

### Verlauf U
| Datum | Änderung | Test |
| --- | --- | --- |
| 22.11.2026 | Umstellung auf Statuskopf (Claude), Commit `8a22ab3` | Validator + PR-Prüfung lokal: gültig |
| 22.11.2026 | Workflow entfernt (öffentliches Repo), Commit `c67bdcc` | Validator lokal: gültig |
| 22.11.2026 | Angenommen durch Merge PR #30 (`6c5fa82`), Dennis | – (Merge ist Annahme, kein Test) |

## Paket P1 – Boote auf Meerfeldern

```yaml devhub-paket
id: P1
titel: "Boote auf gemeinsam genutzten Meerfeldern nicht überlagern"
agent: claude
zustand: geplant
naechster: dennis
haengt_ab_von: []
branch: feature/boote-versatz
commit: null
revier: []
pruefstufe: keine
tests: []
frage: null
aktualisiert: 2026-11-22T22:00Z
```

- **Stand:** **noch nicht begonnen**. Vorschlag aus der Historie (Claude, 22.11.2026), mit dem Merge von Dennis freigegeben.
- Quelle: Historie (17.11.2026), „Problem“: „Auf gemeinsam genutzten Meerfeldern überlagern sich einzelne Boote trotz des kleinen vorgesehenen Versatzes … Platzierungsbeobachtung für Claude“. Der dort genannte nächste Schritt (PR #26 prüfen) ist erledigt – PR #26 wurde am 17.11. gemergt.
- **Revier:** wird beim Start festgelegt (erster Commit des Pakets).

## Paket AE – Musik-Ebenen (Werkzeug)

```yaml devhub-paket
id: AE
titel: "Werkzeug tools/audio: Musik-Ebenen für adaptive Musik mit dem Audio-Kern vorbereiten"
agent: claude
zustand: in_arbeit
naechster: dennis
haengt_ab_von: []
branch: claude/audio-ebenen
commit: null
revier: ["tools/audio/**", docs/ADAPTIVE_MUSIK.md]
pruefstufe: getestet
tests:
  - art: automatisch
    befehl: "python -m unittest discover -s tools/audio/tests (audiokern aus Tonstudio-2.0 PR #2, 49e4732)"
    datum: 2026-11-22
    umgebung: "Cloud-Arbeitsumgebung Claude (DevHub-Sitzung), Python 3.13, ffmpeg – selbst ausgeführt, nicht aus der PR-Beschreibung übernommen"
    ergebnis: "2 Tests OK"
    erfolgreich: true
frage: null
aktualisiert: 2026-11-22T21:30Z
```

- **Nachgetragen** am 22.11.2026 bei der Umstellung auf den Statuskopf: Der PR entstand vorher und hatte kein Paket. Code unverändert; der Standard-Branch wurde hineingemergt, damit `versionsstand.md` mit Statuskopf vorliegt. Die eine Zeile, die der PR in die alte `versionsstand.md` schrieb, steht jetzt hier.
- **Stand:** **Entwurfs-PR (#29)** – daher `in_arbeit`, keine Übergabe.
- **Funktioniert:** `tools/audio/prepare_layers.py` macht Ebenen probengenau gleich lang, nahtlos schleifbar und auf Ziel-Lautheit; dazu `ebenen.json`. `crates/` und Savegame unverändert.
- **Abhängigkeit:** Audio-Kern 0.2.0 aus Tonstudio-2.0 PR #2 (noch nicht gemergt).
- **Offen:** Abspielen im Spiel, Musik-Director, Übergänge (siehe `docs/ADAPTIVE_MUSIK.md`).
- **Nächster Schritt:** Agent entscheidet, ob der Entwurf fertig ist; dann Übergabe per Statuscommit.
- **Verantwortlich:** Claude (Sitzung des Audio-Kerns).
- **Bisheriger Status-Nachtrag dieses PRs** (stand in der alten `versionsstand.md`, wörtlich):
  > | 22.11.2026 | Claude: Werkzeug `tools/audio/prepare_layers.py` – Musik-/Atmo-Ebenen mit dem Audio-Kern (Tonstudio 2.0) gleich lang, nahtlos, gleich laut. Branch `claude/audio-ebenen`. Spiel/Rust unverändert. | 2 Python-Tests (`tools/audio/tests`) grün, Linux, Audio-Kern 0.2.0. Rust nicht berührt, daher nicht neu gebaut. Abspielen im Spiel: nicht umgesetzt. |

### Verlauf AE
| Datum | Änderung | Test |
| --- | --- | --- |
| 22.11.2026 | Werkzeug als Entwurf (Claude), Commit `7480547` | laut PR: 2 Tests grün |
| 22.11.2026 | Paket nachgetragen, `main` hineingemergt (Claude, DevHub), Commit `53bd1b6` | selbst ausgeführt: 2 Tests OK |

## Historie

Inhalt von `versionsstand.md` bis 22.11.2026, **unverändert übernommen**:

# Versionsstand – Nature_in_a_glass

> **Stand:** 17.11.2026 · **Version:** 0.22.1 (Einbäume auf dem Meer) · **Branch:** `main`

## Auf einen Blick

| Frage | Antwort |
| --- | --- |
| **Zuletzt erledigt** | PR #26 auf v0.22.0 aktualisiert: 88 Einbaum-/Jagd-/Hirschbilder fest in assets/nature, einschließlich Weltmaßen, Ankern und Fernstufen. Beide Voll-Exportwege erhalten das Paket. Claudes Darstellungs-/Kerncode unverändert. |
| **Getestet** | 119 Rust-/15 Python-Tests, fmt, strenges Workspace-Clippy und Release-Bau bestanden. 88er-Matrix, Quell-/Bildhashes, alte Assets/Manifestdaten, wiederholte Menschen-/Tierexporte und 21 App-Tests nach Exportprüfung. Sechs echte eingebettete Spielansichten auf großer Insel bei Zoom 8/32: Boote, natürlicher Jagdtag und erlegter Hirsch. |
| **Problem** | Auf gemeinsam genutzten Meerfeldern überlagern sich einzelne Boote trotz des kleinen vorgesehenen Versatzes. Im Küstenfoto dokumentiert; Platzierungsbeobachtung für Claude. |
| **Offen für Codex** | Visuelle Abnahme durch Claude; aktuelles Prüfpaket `art/nature/canoe-and-hunt-style-b/README.md`, native Bilder in `v0.22-integration/`. |
| **Offen für Dennis** | – |
| **Nächster Schritt** | Claude prüft den aktualisierten PR #26 im Spiel und übernimmt die eingebetteten Bilder bei visueller Freigabe. |

## Zusammenarbeit Claude / Codex

- **Claude:** Simulation, technische Schnittstellen, Tests und Integration.
- **Codex:** Assets, Sprites, Texturen und visuelle Darstellung gemäß `docs/gestaltung.md`.
- `docs/gestaltung.md` bleibt verbindliche Gestaltungsspezifikation, **nicht** das laufende Statusprotokoll.
- **Nach Arbeitsende:** diese Datei mit erledigter Änderung, offenem Problem, Branch/Commit, Testbefehl und Ergebnis aktualisieren. Bei Unklarheit keine Erfolgsmeldung erfinden; Status `ungeprüft` setzen.

## Änderungsprotokoll

| Datum | Änderung | Code-Test |
| --- | --- | --- |
| 17.11.2026 | Codex: Einbaum/Jagd (PR #26) auf v0.22.0 integriert, Basis `a39bb5b`. | 119 Rust-/15 Python-Tests, fmt, Clippy, Release-Bau, alte Assets unverändert, reproduzierbare Voll-Exporte; echte Boote/Jagd/erlegter Hirsch bei Zoom 8/32. Überlagerungen gemeinsamer Bootsplätze dokumentiert; visuelle Abnahme offen. |
| 17.11.2026 | Codex: HUD im Seekartenstil auf v0.21.0, Branch `codex/observer-hud-chart-style`, Basis `53d5d51`. | 117 Rust-/15 Python-Tests, fmt, Clippy, Release-Bau, native Ankunfts-/Haken-/Bewohnerproben und Pixelprüfungen bestanden. Visuelle Abnahme offen. |
| 16.11.2026 | Codex: Inselwelt geprüft, Seekartenschrift für Inselnamen, Branch `codex/islandworld-cartography`, Basis `66d829e` (v0.19.0). | 114 Rust-/15 Python-Tests, fmt, App-Clippy, Release-Bau; 24 native Aufnahmen, Umlaute, ruhige Pausenansicht und unveränderte Bildbereiche außerhalb der Namen. Workspace-Clippy: vorhandener Kernfehler. Visuelle Abnahme offen. |
| 16.11.2026 | Codex: 140 Werkzeugbilder, Branch `codex/people-tools-style-b`, Basis `af87656` (v0.13.0). | 109 Rust-/15 Python-Tests, fmt, Clippy; Vollständigkeit, Maße, unveränderte Kinder/Elternbilder und wiederholter Export geprüft. Visuelle Abnahme offen. |
| 16.11.2026 | Codex: 90 Wandwasserbilder und Nutzung alter Atlasreihen-Lücken, Branch `codex/stream-drops-style-b`, Basis `3ac3bc1`. | 103 Rust-/15 Python-Tests, fmt, Clippy, Release-Bau; 120 RGBA-Anschlüsse und unveränderter Bestand. Nasse/trockene Spielansichten im Prüfpaket; visuelle Abnahme offen. |
| 16.11.2026 | Codex: 20 Gartenbilder nach Gestaltung 17.9, Branch `codex/garden-style-b`, Basis `cc542e8`. | 100 Rust-/15 Python-Tests, fmt, Clippy, Release-Bau; Alpha-/Manifest-/Bestandsprüfung und zwei wiederholbare Exportwege. Sommer-/Ernteprüfung Seed 300, Feld 25/20, Zoom 8/32 im Prüfpaket; visuelle Abnahme offen. |
| 16.11.2026 | Codex: 40 Fischfangbilder und Vorschlag Wandwasserformat 16.9, Branch `codex/fishing-people-style-b`, Basis `0edb584`. | 97 Rust-/15 Python-Tests, fmt, Clippy, Release-Bau; eingebettete Ansicht und deklarierte Bildaustauschprobe Zoom 8/32. Export-/Maß-/Bestandsprüfung. Technische Zustandsauswahl offen bei Claude. Prüfpaket dokumentiert Darstellungsprobe getrennt von echter Fischfangaktivität. |
| 16.11.2026 | Codex: 54 Bach-/Bachbettstücke nach 16.8, Branch `codex/stream-beds-style-b`, Basis `2f0e187`. | 94 Rust-/15 Python-Tests, fmt, Clippy, Release-Bau; Gegenprofile und alle 768 Knotenkonfigurationen geprüft; Spielvergleich Seed 2024 +12 Jahre +240 Tage, 19/33; Höhenstufen-Verdeckung dokumentiert. |
| 16.11.2026 | Codex: Meer-/Flachwasser-/Teichanimation mit vier Varianten, 4×500 ms, Branch `codex/water-style-b`, Basis `0d959fe`. | 93 Rust-/15 Python-Tests, fmt, Clippy, Release-Bau; Vollatlasgrenze und Wasser-/Spielvergleich geprüft. Visuelle Abnahme offen. |
| 16.11.2026 | Codex: vollständiges Tierpaket mit eigenen Fernzyklen, Spuren und Kadavern; Branch `codex/animals-style-b`, Basis `7d9dcc5`. | Export-/Referenzmaß-/Phasenprüfung und Atlas-/Spielvergleich im Tier-Prüfpaket; visuelle Abnahme offen. |
| 16.11.2026 | Codex: Korrektur der bei Zoom 4/8 unlesbaren Landschaft; 32 breite Fernkronen, Nahbilder unverändert. Branch `codex/landscape-far-crowns`, Basis `d6fc0bf`. | Atlas-Auswahl-/Maßstabsregression sowie tatsächlicher Vorher-/Nachher-Vergleich im Fernkronen-Prüfpaket; erneute Abnahme offen. |
| 16.11.2026 | Codex: Landschaft nach Gestaltung 18, Schritt 3; Branch `codex/landscape-style-b`, Basis `8043787` (v0.8.5). | Technische Export-/Maßstabsprüfung und Spielvergleich siehe Landschaft-Prüfpaket; visuelle Abnahme offen. |
| 16.11.2026 | Codex: Erwachsenenmaß 90 %, kompakte Tätigkeiten, größere Hütteneingänge. Branch `codex/people-scale-and-hut-doors`, Basis `0f81511`. | Unveränderte Kinder/Anker/Farben und wiederholbare Exporte geprüft; Spielvergleich Seed 2024 Zoom 8/32 im Prüfpaket. |
| 16.11.2026 | Codex: übrige Menschenbilder vollständig auf Stil B umgestellt; 230 Erwachsenen-, 150 Kinder- und zehn Totenbilder. Freigegebenes `_build` pixelgleich auf breiterer Leinwand, Marker unverändert. Branch `codex/people-style-b`, Basis `73ccfd6`. | 89 Rust-/15 Python-Tests, fmt, Clippy, Release-Bau; technische Vollständigkeit, Exportwiederholung und Spielvergleich. Visuelle Schleifen-/Detailabnahme offen. |
| 16.11.2026 | Codex: Feuerstelle 6→9 im Weltmaß, proportional und mit festen Fußpunkten; nächster Auftrag: übrige Menschenbilder im neuen Stil. | 89 Rust-Tests, fmt, Clippy und Release-Bau bestanden; tatsächlicher Vergleich Zoom 8/32 geprüft. |
| 16.11.2026 | Codex: feste 120×232-Skalierung entfernt; beide Hütten proportional neu exportiert, Leinwand/Manifest angepasst. Die übrigen 47 Motive bleiben bytegleich. | Quell-/Exportproportionen und wiederholter Export geprüft; tatsächlicher Vorher-/Nachher-Spielvergleich bei Zoom 8/32, ohne `--bilder`. Visuelle Abnahme durch Claude noch offen. |
| 16.11.2026 | Codex: Lager-Draft mit neun Einzelbildern, 40 `_build`-Phasen, Manifesten und Prüfpaket; Übergabe an Claude: Bewuchs auf Lagerfeldern verdeckt teilweise Hütten. Stilumstellung aller Bildgruppen in Gestaltung 18 festgehalten. | 89 Rust-Tests, fmt, strenges Clippy und Release-Bau bestanden; Dateiprüfung aller 49 PNGs; eingebettete Bilder ohne `--bilder` bei Zoom 4/8/32, Windschutz, Holztragen und kalter Feuerstelle angesehen. Keine Änderungen am Kern/Renderer, visuelle Freigabe offen. |
| 16.11.2026 | v0.16.0 Einbaum (Claude) | alle Tests grün; Langlauf 32 Inseln mit und ohne Einbaum |
| 16.11.2026 | v0.15.0 Tongefäß, Rollen (Claude) | alle Tests grün; Langlauf 16 Inseln mit Wild: 11 überleben |
| 16.11.2026 | v0.14.0 Jagd (Claude) | alle Tests grün; Langlauf 16 Inseln mit Wild |
| 16.11.2026 | v0.13.0 Werkzeuge, Umzüge, Durstkorrektur (Claude) | alle Tests grün; Langlauf 16 Inseln × 120 Jahre: 13 überleben |
| 16.11.2026 | v0.12.0 Dörfer, Atlas in die Breite (Claude) | alle Tests grün; Langlauf 8 Inseln × 130 Jahre: 6 überleben, 2 Dörfer |
| 16.11.2026 | v0.11.0 Lager teilen sich (Claude) | alle Tests grün; Langlauf 8 Inseln × 130 Jahre: 6 überleben |
| 16.11.2026 | v0.10.0 Gärten, Holz von weiter her (Claude) | alle Tests grün; Langlauf 8 Inseln × 120 Jahre: 6 überleben |
| 16.11.2026 | v0.9.0 Fischfang, Fehler beim Verdursten an trockenen Teichen behoben (Claude) | alle Tests grün; Langläufe 8 Inseln × 120 Jahre |
| 16.11.2026 | v0.8.1 Flammen und Glutsaum aus PR #10 eingebunden | Spielbild Seed 7 Zoom 8 |
| 16.11.2026 | v0.8.0 Lagerplätze (Claude) | fmt, clippy, alle Tests grün; Spielbild Zoom 8/2 angesehen |
| 29.10.2026 | Erste gemeinsame, lesbare Übergabe aus den vorhandenen Projektberichten erstellt | Nicht ausgeführt |
