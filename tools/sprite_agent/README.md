# Lokaler Sprite-Agent v1

Ein kleiner auftragsgesteuerter Produzent, kein weiterer Server und kein
autonomer Art Director. Python steuert das vorhandene ComfyUI, verarbeitet
Bilder und erstellt Freigabepakete. Keine Cloud-Bildgenerierung, kein neues
Modell heruntergeladen, keine Änderungen am Renderer. Der erste lokale
Fuchstest zeigt ausdrücklich noch **keine** gleichwertige, konsistente
Reproduktion des freigegebenen Stils B; siehe Praxistest unter `art/nature/sprite-agent-pilot/`.

## Einmalige Einrichtung auf diesem Rechner

1. Das vorhandene ComfyUI starten (lokal `http://127.0.0.1:8188`). Das normale
   Desktop-Profil kennt bereits die gemeinsamen Modelle unter
   `L:/Comfy-Desktop/ComfyUI-Shared/models`.
2. Python ist bereits vorhanden:
   `L:/Comfy-Desktop/ComfyUI-Installs/ComfyUI/ComfyUI/.venv/Scripts/python.exe`.
   Pillow, NumPy und SciPy sind dort installiert. Auf einem anderen Rechner
   diese drei Pakete aus `requirements.txt` in einer eigenen Umgebung installieren.
3. `python -m tools.sprite_agent doctor` prüft API, Modelle, Referenz-Nodes und
   GPU. Nötig: SDXL Base, für referenzgeführte Aufträge zusätzlich
   IPAdapterUnifiedLoader/Advanced, IP-Adapter PLUS SDXL und CLIP-ViT-H.
   Ohne Referenz-Nodes gibt es keinen stillen Rückfall auf Textgenerierung.
4. Stilprofil `profiles/nature.json` beschreibt Ankerfamilie, Licht, Referenzen,
   Kategorien, Weltmaße und Standardaufträge. `gestaltung.md` bleibt die
   menschliche Vorgabe. Ihr vollständiger Inhalt und Hash werden je Lauf
   archiviert; die kurze maschinenlesbare Zusammenfassung liegt im Profil.
   Änderungen an der Bildsprache erfordern eine bewusste Profilanpassung.

`python` in den Beispielen bedeutet diese konfigurierte Umgebung. In PowerShell
ist der direkte Aufruf ohne Umgebungswechsel einfacher:

```powershell
& './tools/sprite_agent/sprite-agent.ps1' 'Erstelle einen Fuchs mit vier Blickrichtungen im Stil von Nature in a Glass.'
& './tools/sprite_agent/sprite-agent.ps1' 'Erstelle drei neue Tiere für Nature in a Glass.'
```

Der zweite Auftrag wählt transparent die drei Profilmotive Fuchs, Wolf und
Kaninchen. Er erfindet keine unbekannten Arten. Neue Motive werden **einmal**
im Katalog ergänzt, danach genügt ihr Name im Auftrag. Es handelt sich um
einen bewusst kleinen regelbasierten Auftragsparser, nicht um ein Sprachmodell.
Bekannte Motive: Fuchs, Wolf, Kaninchen, Mensch, Hütte, Korb, Baum. Gebäudemaße
und Gegenstandsmaße sind vorläufige Produktionsprofile, keine Engine-Freigabe.
Freie semantische Zusatzwünsche werden nicht vollständig interpretiert; dafür
den konkreten JSON-Auftrag bearbeiten. Unbekannte Motive und vage
Animationsaufträge werden abgewiesen statt falsch vervollständigt.

## Ablauf und Budget

`request` plant und produziert bis zum lokalen Prüfblatt ohne weitere
Codex-Steuerung. Default: höchstens acht Bilder insgesamt, je Bild 24 Schritte
mit SDXL auf 768×768, alle GPU-Aufträge seriell. Keine Schleife bis ein Bild
angeblich schön genug ist. Größere Aufträge müssen bewusst aufgeteilt werden.

```text
python -m tools.sprite_agent plan "Erstelle einen Fuchs mit vier Blickrichtungen" --out runs/mein-auftrag
python -m tools.sprite_agent run runs/mein-auftrag/fox.json
python -m tools.sprite_agent run runs/mein-auftrag/fox.json --resume runs/sprite-agent/fox-...
python -m tools.sprite_agent prepare runs/sprite-agent/fox-...
```

`prepare` wiederholt nur die lokale Aufbereitung, ohne GPU oder neue Bilder.
Der Ausgabepfad erscheint direkt beim Start. Jeder Lauf hat eine eindeutige ID.
Die JSON-Snapshots, Seeds, tatsächlich gesendeten API-Workflows und Historien
machen ihn nachvollziehbar. `journal.json` speichert Prompt-IDs vor dem Warten:
Nach einem Timeout wird derselbe Prompt weiter abgefragt. Ist beim Absenden
unklar, ob ComfyUI den Auftrag angenommen hat, stoppt der Agent zur Prüfung,
statt doppelt zu generieren. Das Budget zählt gebuchte Aufträge einschließlich
Fehlschlägen; automatisches endloses Wiederholen gibt es nicht.

## Freistellung und Detailstufen

Vorhandenes Alpha wird übernommen. Andernfalls entfernt eine vom Bildrand
ausgehende Farbauswahl den gleichmäßigen Hintergrund. Die größte zusammenhängende
Figur bleibt erhalten; entfernte Teile werden gemeldet. Das ist keine universelle
KI-Segmentierung: Schatten, helle Fellränder, Requisiten und voneinander getrennte
Objektteile brauchen eine Sichtprüfung. Keine unbemerkte Installation eines
weiteren Freistellmodells.

Ein gemeinsamer Skalierungsfaktor für sämtliche Bilder verhindert unabhängiges
Aufblasen kleiner Posen. Fußpunkte sind zunächst die untere Motivmitte und werden
als Heuristik gekennzeichnet. Sie bleiben über alle LOD-Stufen in denselben
Weltkoordinaten. Bei unbekannter Anatomie kann der Agent die Korrektheit nicht
automatisch bestätigen.

LOD1/2/4/8 enthalten 1×/2×/4×/8× die Weltmaße. Kleine Fassungen sind geglättet
und auf 24 Farben vereinfacht. Das ist eine technische Vereinfachung, **keine
semantische Neuzeichnung** von Ohren und Schwanz. Ein Silhouettenvergleich sowie
die Zahl verbleibender Pixel markieren kritische Fälle zur Prüfung. Bei Bedarf
wird eine eigene gezeichnete kleine Fassung vorgeschlagen. Kontaktbögen zeigen
drei Untergründe bei Zoom 4/8/16/32, außerdem eine vergrößerte Identitätsansicht.

## Freigabe

`review.html` öffnet lokal ohne Dienst oder Internet. `validation.json` trennt
`technical_status` von `visual_status`. Technik kann bestanden sein, während
Richtung, Stil oder Anatomie falsch sind. Nichts wird automatisch ins Spiel
kopiert. Die Manifestdatei bleibt bis zur menschlichen Prüfung ein Vorschlag.

Nur nach Dennis' ausdrücklicher Auswahl:

```text
python -m tools.sprite_agent approve PAKET --reviewer Dennis --notes "Auswahl und geprüfte Ansichten" --visual-checked
```

Das protokolliert Zustimmung und Manifest-Hash in `approval.json`, installiert
aber nichts. Bereits freigegebene Pakete können nicht erneut aufbereitet oder
überschrieben werden. Änderungen sind immer neue Läufe. Der Befehl ist kein
Beweis einer tatsächlichen menschlichen Prüfung – der aufrufende Assistent
darf ihn nur nach dieser Freigabe verwenden.

## Weitere Kategorien, Ansichten und Animation

Die Verarbeitung ist für Tiere, Menschen, Gebäude, Pflanzen und Objekte gleich.
Profil/Katalog bestimmen Motiv, Maße und Zustände. `se/sw/ne/nw` bezeichnen
Bildschirmdiagonalen; sie sind nicht das heutige `l/r` der Engine.
Für eine Animation im JSON `states` mit `frames`, `frame_ms` und genau einer
konkreten `poses`-Beschreibung je Frame anlegen. Jeder Frame wird wirklich
generiert; keine gespiegelten oder kopierten Scheinphasen. Eine solche Folge
wird technisch vorbereitet, ist aber mit SDXL/IPAdapter noch keine garantiert
konsistente Animation. Der erste Praxistest enthält daher vier Standansichten,
keinen als fertig ausgegebenen Bewegungszyklus.

`strategy: auto` nutzt mit Referenz den IP-Adapter-Workflow, sonst den bereits
verwendeten SDXL-Textworkflow. `img2img` nutzt den vorhandenen VAEEncode-Aufbau
für Variationen derselben Ansicht und wird bei mehreren Richtungen abgewiesen.
Das ist eine bewusste fachliche Auswahl statt eines großen Workflow-Katalogs.

## Erweiterung und Engine-Abstimmung

- `model.py`: Aufträge, Profile, Validierung.
- `comfy.py`: einziger Bildgenerierungsadapter; später austauschbar, kein
  ComfyUI-Typ im Paketmanifest.
- `imaging.py`: Freistellung, gemeinsamer Maßstab, LOD.
- `review.py`: technische Prüfungen und visuelle Vorlagen.
- `runner.py`: Budget, Journal, Wiederaufnahme, Freigabeschutz.

Andere Spiele nutzen `--profile DATEI.json`. Referenzen und Hintergründe werden
relativ zum Profil aufgelöst. Eine 3D-Pipeline wird nicht gebaut. Für eine spätere
andere Ausgabeart kann ein eigener Exportadapter ergänzt werden, ohne den
Auftrags- oder Freigabeablauf zu ersetzen.

Die genaue Übergabe an Claude steht in `docs/sprite-package-contract.md`.
Das Format ist noch nicht von ihm bestätigt; es gibt keine Rendereränderung.

Tests: `python -m unittest discover -s tools/sprite_agent/tests -v`.
Die Tests nutzen synthetische Bildfixtures und einen falschen Comfy-Adapter;
sie prüfen Abläufe und Fehler, keine künstlerische Qualität. Der echte lokale
Praxistest wird getrennt dokumentiert.
