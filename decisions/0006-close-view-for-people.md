# ADR 0006 – Eigene Nahansicht für die Menschen

**Status:** angenommen (2026-10-18)

## Kontext

Ein Feld ist 50 × 50 m groß. Schon die Tiere sind gegenüber diesem Maßstab stark vergrößert, damit man sie erkennt. Für die Bewohner (v0.6) gab es zwei Wege: (A) Menschen als kleine Figuren im heutigen Zoombereich (bis 8), oder (B) eine eigene Nahansicht, in der man einzelne Menschen und später Dörfer im Detail sieht.

## Entscheidung

Dennis hat B gewählt.

- Die Kamera zoomt mit den Menschen bis 32 (bisher 8).
- Menschen werden – wie Tiere – vergrößert dargestellt (Darstellungsmaß 7 × 15 im Grundmaß), aber in achtfacher Auflösung gezeichnet, damit sie bei Zoom 32 scharf sind. Der Renderer skaliert jedes Bild auf sein Darstellungsmaß (wie bei Tieren und Pflanzen, `display_size`).
- Die Landschaft bekommt später eine Runde in vierfacher Auflösung; bis dahin ist sie ganz nah gepixelt.
- Vorgaben: `docs/gestaltung.md`, Abschnitte 4.3 und 17.

## Konsequenzen

- Der Bildatlas wächst deutlich. Bei hochaufgelösten Bildern, die weit herausgezoomt stark verkleinert werden, braucht der Beobachter verkleinerte Fassungen (Mipmaps) mit Abstand zwischen den Bildern im Atlas, sonst flimmert es. Das wird mit v0.6 eingebaut.
- Die Weltgröße und die Simulation bleiben unverändert; die Nahansicht ist reine Darstellung.
