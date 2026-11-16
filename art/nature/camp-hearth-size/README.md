# Größere Lagerfeuerstelle

Weltbreite 9 statt 6 für brennende, kalte und verfallene Feuerstelle. Beide
Achsen werden gleich um Faktor 1,5 vergrößert. PNGs, Fußpunkte, Animationstakt
und feste Lagerposition bleiben unverändert; geändert sind Manifest und
Erstexport-Metadaten.

Vorher/nachher im eingebetteten Spiel bei Zoom 8/32 angesehen: Seed 2024,
zehn Jahre Natur, Menschen, weitere drei Jahre, Auswahl 19/34. Die Feuerstelle
ist neben Menschen besser erkennbar. Die Sortierung nach Fußpunkt gilt weiter;
ein Mensch kann je nach Standort weiterhin einen Teil der Feuerstelle verdecken.

```sh
nature-in-a-glass --seed 2024 --years 10 --menschen --danach 3 --select 19,34 --zoom 32
```

Basis: 73ccfd6 / v0.8.3. Formatprüfung, strenges Clippy, Release-Bau und
89 Rust-Tests bestanden. Vorher-/Nachher-Aufnahmen liegen in diesem Ordner.
Keine Änderungen an Renderer oder Simulation.
