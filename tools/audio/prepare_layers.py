#!/usr/bin/env python3
"""Musik- und Atmo-Ebenen für die adaptive Musik vorbereiten (docs/ADAPTIVE_MUSIK.md).

Rechnet mit dem Audio-Kern aus Tonstudio 2.0 – nur hier im Werkzeug, nicht im Spiel und nicht in `crates/core`.
Alle Ebenen einer Liste werden **probengenau gleich lang**, **nahtlos schleifbar** (kein Knacken am Wiederholpunkt)
und auf ihre Ziel-Lautheit gebracht, damit ein späterer Musik-Director sie synchron überblenden kann.

    python tools/audio/prepare_layers.py ebenen.json [--ueberschreiben]

Liste:
    {"ziel": "assets/audio/insel",           # relativ zur Liste
     "laenge_s": 32.0,                        # Schleifenlänge aller Ebenen (bei Musik: ganze Takte)
     "ueberblendung_s": 2.0,                  # Naht: so viel Material wird nach dem Ausschnitt gebraucht
     "lufs": -20.0,                           # Vorgabe je Ebene, einzeln änderbar
     "ebenen": [{"name": "wind", "datei": "roh/wind.flac", "von": 3.0, "lufs": -26},
                {"name": "menschen", "datei": "roh/motiv.wav"}]}

Ergebnis: je Ebene `<name>.wav` (48 kHz, 24 bit) und `ebenen.json` mit Länge, Rate und Messwerten.
Quelldateien werden nur gelesen, vorhandene Ziele nur mit `--ueberschreiben` ersetzt.
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path

try:
    import audiokern as ak
except ImportError:  # pragma: no cover
    sys.exit("Audio-Kern fehlt: pip install 'audiokern @ git+https://github.com/Derako91/Tonstudio-2.0'")

FORMAT = "nig-musik-ebenen/1"
RATE = 48000
SPITZE_DBTP = -1.0


def vorbereiten(liste: Path, *, ueberschreiben: bool = False) -> dict:
    liste = Path(liste)
    plan = json.loads(liste.read_text(encoding="utf-8"))
    basis = liste.parent
    ziel = basis / plan.get("ziel", "assets/audio")
    laenge, ueber = float(plan["laenge_s"]), float(plan.get("ueberblendung_s", 2.0))
    ebenen = plan.get("ebenen") or []
    namen = [e.get("name", "") for e in ebenen]
    if not ebenen or len(set(namen)) != len(namen) or not all(re.fullmatch(r"[a-z0-9_-]+", n) for n in namen):
        raise ValueError("Ebenen brauchen eindeutige Namen aus a–z, 0–9, _ und -")

    fertig = []
    for e in ebenen:  # erst alles bauen, dann schreiben: ein Fehler hinterlässt nichts Halbes
        quelle = ak.importieren(basis / e["datei"], rate=RATE)
        schleife = ak.nahtlose_schleife(quelle, laenge, von=float(e.get("von", 0.0)), ueberblendung=ueber)
        a = ak.angleichen(schleife, float(e.get("lufs", plan.get("lufs", -20.0))), hoechste_spitze_dbtp=SPITZE_DBTP)
        fertig.append((e, a))
    pfade = [ziel / f"{e['name']}.wav" for e, _ in fertig] + [ziel / "ebenen.json"]
    vorhanden = [str(p) for p in pfade if p.exists()]
    if vorhanden and not ueberschreiben:
        raise FileExistsError("existiert bereits (mit --ueberschreiben ersetzen): " + ", ".join(vorhanden))

    index = {"format": FORMAT, "rate": RATE, "proben": round(laenge * RATE), "laenge_s": laenge,
             "ueberblendung_s": ueber, "ebenen": []}
    for e, a in fertig:
        ak.exportieren(a.klang, ziel / f"{e['name']}.wav", ueberschreiben=ueberschreiben)
        nachher = ak.lautheit(a.klang)
        index["ebenen"].append({"name": e["name"], "datei": f"{e['name']}.wav", "quelle": e["datei"],
                                "lufs": nachher.lufs, "spitze_dbtp": nachher.spitze_dbtp,
                                "spitze_begrenzt": a.spitze_begrenzt})
    (ziel / "ebenen.json").write_text(json.dumps(index, ensure_ascii=False, indent=1), encoding="utf-8")
    return index


def main(argv=None) -> int:
    p = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    p.add_argument("liste", type=Path)
    p.add_argument("--ueberschreiben", action="store_true")
    a = p.parse_args(argv)
    print(json.dumps(vorbereiten(a.liste, ueberschreiben=a.ueberschreiben), ensure_ascii=False, indent=1))
    return 0


if __name__ == "__main__":
    sys.exit(main())
