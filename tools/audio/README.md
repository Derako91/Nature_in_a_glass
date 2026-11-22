# Musik-Ebenen vorbereiten (Audio-Kern)

Werkzeug für die geplante adaptive Musik ([docs/ADAPTIVE_MUSIK.md](../../docs/ADAPTIVE_MUSIK.md)).
`prepare_layers.py` macht aus Rohaufnahmen Ebenen, die ein späterer
Musik-Director synchron übereinanderlegen und überblenden kann:

- alle Ebenen **probengenau gleich lang** (48 kHz),
- **nahtlos schleifbar**: das Material direkt nach dem Ausschnitt wird über den
  Anfang geblendet, am Wiederholpunkt knackt nichts,
- auf ihre **Ziel-Lautheit** gebracht (LUFS, True Peak ≤ −1 dBTP),
- dazu `ebenen.json` mit Länge, Rate und Messwerten.

Gerechnet wird mit dem Audio-Kern aus
[Tonstudio 2.0](https://github.com/Derako91/Tonstudio-2.0) – nur im Werkzeug.
Das Spiel selbst (`crates/core`, `crates/app`) spielt noch keinen Ton und ist
unverändert; das Abspielen kommt mit der geplanten Audio-Runtime.

```
pip install "audiokern @ git+https://github.com/Derako91/Tonstudio-2.0"
python tools/audio/prepare_layers.py meine-ebenen.json
python -m unittest discover -s tools/audio/tests -v
```

Format der Liste: Kopf von `prepare_layers.py`. Bei Musik die Länge auf ganze
Takte legen, sonst stolpert der Rhythmus am Wiederholpunkt – das Werkzeug
kennt kein Tempo. Rechte an der verwendeten Musik vorher klären; erzeugte
Audiodateien nicht ungeprüft einchecken.
