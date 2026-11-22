"""Prüfstand für prepare_layers.py – braucht Audio-Kern und ffmpeg, sonst übersprungen.

    python -m unittest discover -s tools/audio/tests -v
"""
from __future__ import annotations

import hashlib
import importlib.util
import json
import pathlib
import subprocess
import sys
import tempfile
import unittest

import numpy as np

try:
    import audiokern as ak
except ImportError:
    ak = None

MODUL = pathlib.Path(__file__).resolve().parents[1] / "prepare_layers.py"


def _laden():
    spec = importlib.util.spec_from_file_location("prepare_layers", MODUL)
    m = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = m
    spec.loader.exec_module(m)
    return m


def _ffmpeg(*args):
    subprocess.run(["ffmpeg", "-hide_banner", "-nostdin", "-loglevel", "error", "-y", *args], check=True)


def _hash(p):
    return hashlib.sha256(p.read_bytes()).hexdigest()


@unittest.skipIf(ak is None, "Audio-Kern nicht installiert")
class EbenenTests(unittest.TestCase):
    def setUp(self):
        self.mod = _laden()
        self.tmp = tempfile.TemporaryDirectory()
        self.basis = pathlib.Path(self.tmp.name)
        roh = self.basis / "roh"
        roh.mkdir()
        # Wind: Rauschen, Stereo, 44,1 kHz; Motiv: Ton mit Schwebung, Mono, MP3 – absichtlich ungleiche Formate
        _ffmpeg("-f", "lavfi", "-i", "anoisesrc=color=brown:amplitude=0.4:duration=14:seed=5:sample_rate=44100",
                "-ac", "2", str(roh / "wind.flac"))
        _ffmpeg("-f", "lavfi", "-i", "aevalsrc=0.3*sin(2*PI*220*t)*(0.6+0.4*sin(2*PI*0.37*t)):s=48000:d=12",
                str(roh / "motiv.mp3"))
        self.liste = self.basis / "ebenen.json"
        self.liste.write_text(json.dumps({"ziel": "assets/audio/insel", "laenge_s": 8.0, "ueberblendung_s": 1.5,
                                          "lufs": -20, "ebenen": [
                                              {"name": "wind", "datei": "roh/wind.flac", "von": 2.0, "lufs": -26},
                                              {"name": "menschen", "datei": "roh/motiv.mp3"}]}), encoding="utf-8")

    def tearDown(self):
        self.tmp.cleanup()

    def test_gleich_lang_nahtlos_und_gleich_laut(self):
        quellen = {p: _hash(p) for p in (self.basis / "roh").iterdir()}
        index = self.mod.vorbereiten(self.liste)
        ziel = self.basis / "assets/audio/insel"
        self.assertEqual(json.loads((ziel / "ebenen.json").read_text(encoding="utf-8")), index)
        self.assertEqual(index["proben"], 8 * 48000)
        for eintrag, soll in zip(index["ebenen"], (-26.0, -20.0)):
            klang = ak.importieren(ziel / eintrag["datei"])
            self.assertEqual(klang.daten.shape[0], index["proben"], eintrag["name"])
            self.assertAlmostEqual(eintrag["lufs"], soll, delta=0.2)
            self.assertLessEqual(eintrag["spitze_dbtp"], -1.0 + 0.15)
            # Naht: Sprung Ende→Anfang nicht größer als die größten Sprünge im Inneren
            d = klang.daten
            innen = np.abs(np.diff(d, axis=0)).max()
            self.assertLessEqual(np.abs(d[0] - d[-1]).max(), innen * 1.05, eintrag["name"])
        self.assertEqual({p: _hash(p) for p in (self.basis / "roh").iterdir()}, quellen)

    def test_nichts_ueberschreiben_und_fehler_vor_dem_schreiben(self):
        self.mod.vorbereiten(self.liste)
        with self.assertRaises(FileExistsError):
            self.mod.vorbereiten(self.liste)
        self.assertEqual(self.mod.main([str(self.liste), "--ueberschreiben"]), 0)
        plan = json.loads(self.liste.read_text(encoding="utf-8"))
        plan["ziel"], plan["laenge_s"] = "neu", 11.0  # Wind ab 2 s: 2 + 11 + 1,5 > 14 s
        self.liste.write_text(json.dumps(plan), encoding="utf-8")
        with self.assertRaises(ak.AudioFehler):
            self.mod.vorbereiten(self.liste)
        self.assertFalse((self.basis / "neu").exists())
        plan["laenge_s"], plan["ebenen"][0]["name"] = 4.0, "Wind!"
        self.liste.write_text(json.dumps(plan), encoding="utf-8")
        with self.assertRaises(ValueError):
            self.mod.vorbereiten(self.liste)


if __name__ == "__main__":
    unittest.main()
