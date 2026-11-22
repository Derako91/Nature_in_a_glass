# Nature in a Glass – adaptive Musik und Klangatmosphäre

> **Idee / späterer Ausbau, 22.11.2026.** In Nature in a Glass derzeit **nicht implementiert**. Diese Dokumentation ist kein Auftrag, die Engine oder den bestehenden Audiopfad sofort umzubauen. TheCave-weites Zielbild: [DevHub – modulare Medienarchitektur](https://github.com/Derako91/Devhub/blob/main/docs/modulare-medienarchitektur.md).

## Künstlerische Idee

Nature in a Glass ist ein **Beobachtungsspiel**: Die Welt verändert sich durch Wasser, Pflanzen, Wetter, Tiere, Menschen und später Dörfer. Die Musik soll sich **unaufdringlich mitentwickeln**, nicht alle paar Sekunden einen neuen kompletten Song starten. Nach langer Beobachtung kann die Insel musikalisch anders klingen als zu Beginn.

Beispiele, ausdrücklich **keine fest verdrahteten Regeln**:

| Beobachteter Zustand | Mögliche musikalische Ebene |
| --- | --- |
| Junge oder unbewohnte Insel | Luftige Ambient-Flächen, vereinzelte Töne, Wind/Wasser als Klangatmosphäre |
| Vegetation wächst, Ökosystem stabilisiert sich | Leichte Flöten, behutsame Streicher, warme Harmonien |
| Ankunft von Menschen | Ein neues, zurückhaltendes musikalisches Motiv |
| Lager/Dorf entstehen | Weitere rhythmische oder melodische Ebene, ohne triumphalische Wertung |
| Nacht / Jahreszeiten / Wetterwechsel | Leisere oder anders instrumentierte Variante |
| Dürre, Feuer, Verlust, Rückkehr der Natur | Passende Spannungs-/Auflösungsvarianten, keine permanente Alarmmusik |

Die Musik folgt **tatsächlichen Zuständen und Ereignissen**; sie suggeriert keinen Fortschritt, den die Simulation nicht erzeugt hat. Ein scheiterndes Ökosystem darf ebenso ruhig begleitet oder in Stille belassen werden.

## Modulare Architektur

1. **Simulation (Rust `crates/core`)** erzeugt bestehende deterministische Daten: Weltzustand, Zeit, Saison, Vegetation, Wetter, Wasser, Menschen und Ereignisse. **Keine Audioengine, keine KI-Musikgenerierung und keine Audiokosten im Simulations-Tick.**
2. **Musik-Director (geplant, außerhalb des Simulationskerns)** liest gelegentlich Zustands-Snapshots/Ereignisse, ordnet ihnen musikalische Zustände/Gewichte zu und entscheidet über Übergänge. Er verändert niemals Simulation, Seed, Eventjournal oder Savegame-Semantik.
3. **Audio Core Runtime (geplant)** spielt vorhandene, passend produzierte Audio-Layer/Loops ab, kontrolliert Pegel, Crossfades, Loop-Grenzen und Latenz. Die aktuelle Python-`audiokern`-0.1 ist ein **Production-/Offline-Baustein**, kein eingebauter Echtzeitplayer.
4. **Tonstudio 2.0** erzeugt/arrangiert Themen, Atmosphären, Stems und Varianten – per Komposition, Sampler oder optionaler Musik-KI. Fertige Assets werden versioniert bereitgestellt.
5. **Kükii** kann später den Musik-Director und die Audio-Runtime konsumieren. Nature in a Glass ist heute eine eigene Rust-App: eine direkte, optionale Audio-Anbindung ist bis dahin genauso möglich. Keine sofortige Kükii-Abhängigkeit.

## Vorschlag für die erste technische Abnahme (nicht umgesetzt)

- Zwei sauber loopende musikalische Layer aus echten Dateien, gleiche BPM/Tonart/Taktstruktur oder bewusst tempo-unabhängige Ambient-Layer.
- Eine begrenzte Zustandsänderung (z. B. unbewohnte Insel → Menschen angekommen) löst **einmalig** einen sanften Layer-Crossfade aus.
- Keine hörbaren Klicks; kein Versatz zwischen synchronen Layern; kein lautes Springen bei schnellem Zeitraffer.
- **Hysterese und Cooldown** für wechselhafte Zustände: keine musikalischen Sprünge bei jedem Wetter-/Pflanzenwert.
- Simulationsgeschwindigkeit und Audio-Zeit sind entkoppelt: 10 Jahre/s bedeuten nicht, dass Musik beschleunigt wird oder Tausende Ereignisse abgespielt werden.
- Lautstärke, Musik-an/aus, Geräusche separat und ein **stiller Modus**; Spiel bleibt vollständig ohne Sound nutzbar.
- Ausfall/Fehlen des Audio-Core-Runtime-Adapters darf keinen Simulationstakt anhalten. Ein einfacher bestehender Player kann bis dahin genügen.
- Reproduzierbarer Test mit festen Welt-Snapshots/Ereignissen, um Übergangsauswahl zu prüfen; Audioqualität zusätzlich im Hörtest bewerten.

## Später mögliche Erweiterungen

- Layer-Gruppen für Tag/Nacht, Wetter, Vegetation und menschliche Siedlungen, aber vorsichtige Begrenzung gleichzeitiger Stimmen.
- Leitmotive, die wiederkehren, sich harmonisch/klanglich mit der Insel wandeln und gelegentlich vollständig verstummen.
- Räumliche Geräusche (Wind, Wald, Wasser, Tiere) zusätzlich zur Musik; die Audio-Runtime kann gemeinsame Mix-Funktionen bereitstellen.
- Asset-Referenzen und standardisierte Audio-Events nach dem TheCave-Integrationsstandard; keine Pflicht zu Online-KI oder einem laufenden Homeserver.

## Abgrenzung und Stand

**Jetzt:** Idee dokumentiert, keine Änderung an `crates/core`, `crates/app`, `crates/cli` oder dem Savegame-Format.
**Vor einer Umsetzung:** bestehendes Audio-Backend im Rust-Client, technische Kükii-Schnittstelle, Rechte der verwendeten Musik und CPU-/Latenzbudget prüfen.

Ziel: **Die Welt klingt anders, weil sie anders geworden ist – ohne ihre Selbstständigkeit zu verlieren.**
