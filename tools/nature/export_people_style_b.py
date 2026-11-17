"""Proportionaler Export aller Menschenzustände aus generierten Stil-B-Bögen.

Die Quellen sind archiviert. Keine gespiegelten Richtungen, keine erfundenen
Animationsphasen und keine nichtproportionale Größenkorrektur.
"""
import argparse
import hashlib
import json
import statistics
from pathlib import Path

import numpy as np
from PIL import Image, ImageDraw, ImageFont
from scipy import ndimage

ROOT = Path(__file__).resolve().parents[2]
REVIEW = ROOT / "art/nature/people-style-b"
PEOPLE = ROOT / "assets/nature/people"
ADULT = [("idle", 2, 800), ("walk", 4, 200), ("run", 4, 100),
         ("gather", 4, 300), ("eat", 2, 600), ("drink", 2, 600),
         ("sleep", 1, None), ("hurt", 4, 250)]
CHILD = [("idle", 2, 800), ("walk", 4, 200), ("gather", 4, 300),
         ("eat", 2, 600), ("drink", 2, 600), ("sleep", 1, None)]
ADULT_CELLS = [(state, f) for state, count, _ in ADULT for f in range(count)] + [("dead", 0)]
CHILD_CELLS = (
    [("idle", d, f) for d in ("r", "l") for f in range(2)]
    + [("walk", d, f) for d in ("r", "l") for f in range(4)]
    + [("gather", d, f) for d in ("r", "l") for f in range(4)]
    + [("eat", d, f) for d in ("r", "l") for f in range(2)]
    + [("drink", d, f) for d in ("r", "l") for f in range(2)]
    + [("sleep", d, 0) for d in ("r", "l")]
)


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def load_font(size):
    for name in ("DejaVuSans.ttf", "C:/Windows/Fonts/segoeui.ttf"):
        try:
            return ImageFont.truetype(name, size)
        except OSError:
            pass
    return ImageFont.load_default()


def split(path, cols, rows):
    image = Image.open(path).convert("RGBA")
    pixels = np.asarray(image)
    labels, count = ndimage.label(pixels[:, :, 3] >= 160, np.ones((3, 3)))
    sizes = np.bincount(labels.ravel())
    objects = ndimage.find_objects(labels)
    parts = sorted(((int(sizes[label]), label, box) for label, box in enumerate(objects, 1)
                    if box is not None), reverse=True)[:cols * rows]
    if len(parts) != cols * rows or min(p[0] for p in parts) < statistics.median(p[0] for p in parts) * 0.2:
        raise ValueError(f"Keine {cols * rows} vollständigen Figuren in {path}")
    # Der Generator hält Zeilen/Spalten semantisch ein, aber die Füße stehen
    # nicht exakt auf mathematischen Rastergrenzen. Ganze zusammenhängende
    # Figuren suchen, nach Bodenhöhe gruppieren, dann links→rechts ordnen.
    # So trennt keine starre Rasterkante versehentlich Kopf oder Füße ab.
    parts.sort(key=lambda part: part[2][0].stop)
    cuts = []
    for start in range(0, len(parts), cols):
        row = sorted(parts[start:start + cols], key=lambda part: part[2][1].start)
        for _, label, (ys, xs) in row:
            if xs.start == 0 or ys.start == 0 or xs.stop == image.width or ys.stop == image.height:
                raise ValueError(f"Figur berührt Quellbildrand: {path}")
            box = (max(0, xs.start - 4), max(0, ys.start - 4),
                   min(image.width, xs.stop + 4), min(image.height, ys.stop + 4))
            x0, y0, x1, y1 = box
            data = pixels[y0:y1, x0:x1].copy()
            mask = ndimage.binary_dilation(labels[y0:y1, x0:x1] == label, iterations=4)
            data[:, :, 3] = np.where(mask & (data[:, :, 3] >= 24), data[:, :, 3], 0)
            cut = Image.fromarray(data)
            cuts.append(cut)
    return cuts


def place(cut, size, anchor, factor):
    target = (max(1, round(cut.width * factor)), max(1, round(cut.height * factor)))
    if target[0] > size[0] - 4 or target[1] > anchor[1] - 4:
        raise ValueError(f"Motiv passt nicht proportional auf Leinwand: {target} / {size}")
    image = Image.new("RGBA", size)
    image.alpha_composite(cut.resize(target, Image.Resampling.LANCZOS),
                          (round(anchor[0] - target[0] / 2), anchor[1] - target[1]))
    return image


def export():
    source_manifest = json.loads((REVIEW / "sources.json").read_text())
    adult, child = {}, {}
    for source in source_manifest["sources"]:
        path = REVIEW / source["file"]
        if sha(path) != source["sha256"]:
            raise ValueError(f"Geänderter Quellbogen: {path}")
        if source["kind"] == "adult":
            adult[source["look"], source["dir"]] = split(path, 4, 6)
        else:
            child[source["look"]] = split(path, 8, 4)
    manifest_path = PEOPLE / "manifest.json"
    manifest = json.loads(manifest_path.read_text())
    icc = Image.open(PEOPLE / "person_0_build_r_f0.png").info.get("icc_profile")
    images, files, scales = {}, [], []

    def save(name, image):
        path = PEOPLE / (name + ".png")
        image.save(path, icc_profile=icc)
        images[name] = image
        files.append({"file": path.name, "sha256": sha(path), "size": list(image.size),
                      "bbox": image.getchannel("A").getbbox()})

    for look in range(5):
        # An der bereits freigegebenen Holztragehöhe ausrichten. Doppelte
        # transparente Breite + doppeltes display_size lassen 8 px/Welteinheit
        # und damit die sichtbare Größe unverändert, schaffen aber Platz für
        # ausgestreckte Arme/Beine und liegende Posen.
        build = [Image.open(PEOPLE / f"person_{look}_build_{d}_f{f}.png").convert("RGBA")
                 for d in ("l", "r") for f in range(4)]
        heights = [im.getchannel("A").point(lambda a: 255 if a > 128 else 0).getbbox()
                   for im in build]
        height = statistics.median(b[3] - b[1] for b in heights)
        for direction in ("l", "r"):
            cuts = adult[look, direction][:23]
            reference = statistics.median(c.height - 8 for c in cuts[:2])
            # Separat erzeugte Richtungen können im Quellbogen etwas größer
            # sein. Jede Richtung an dieselbe Holztragehöhe kalibrieren; der
            # Faktor bleibt dann für alle Tätigkeiten und Phasen unverändert.
            factor = min(height / reference, 108 / max(c.width for c in cuts),
                         108 / max(c.height for c in cuts))
            scales.append({"look": look, "direction": direction, "kind": "adult",
                           "factor": factor, "build_reference_height": height})
            for index, (state, frame) in enumerate(ADULT_CELLS):
                cut = adult[look, direction][index]
                if state == "dead":
                    f = min(112 / cut.width, 32 / cut.height)
                    size = (round(cut.width * f), round(cut.height * f))
                    image = Image.new("RGBA", (120, 40))
                    image.alpha_composite(cut.resize(size, Image.Resampling.LANCZOS),
                                          (60 - size[0] // 2, 20 - size[1] // 2))
                    save(f"person_dead_{look}_{direction}_f0", image)
                else:
                    save(f"person_{look}_{state}_{direction}_f{frame}",
                         place(cut, (112, 120), (56, 112), factor))
        for direction in ("l", "r"):
            for frame in range(4):
                name = f"person_{look}_build_{direction}_f{frame}"
                old = Image.open(PEOPLE / (name + ".png")).convert("RGBA")
                if old.size == (56, 120):
                    padded = Image.new("RGBA", (112, 120))
                    padded.paste(old, (28, 0))
                elif old.size == (112, 120):
                    padded = old
                else:
                    raise ValueError(f"Unerwartetes Holztrageformat: {old.size}")
                save(name, padded)
        cuts = child[look][:30]
        reference = statistics.median(c.height - 8 for c in cuts[:4])
        factor = min(height * 0.7 / reference, 76 / max(c.width for c in cuts),
                     72 / max(c.height for c in cuts))
        scales.append({"look": look, "kind": "child", "factor": factor})
        for index, (state, direction, frame) in enumerate(CHILD_CELLS):
            save(f"person_child_{look}_{state}_{direction}_f{frame}",
                 place(cuts[index], (80, 80), (40, 76), factor))

    entries = []
    for kind, states, size, world, anchor in [
        ("person", ADULT + [("build", 4, 250)], [112, 120], [14, 15], [56, 112]),
        ("person_child", CHILD, [80, 80], [10, 10], [40, 76]),
    ]:
        for state, count, ms in states:
            mapping = [{"appearance": look, "direction": d, "frame": f,
                        "file": f"{kind}_{look}_{state}_{d}_f{f}.png"}
                       for look in range(5) for d in ("l", "r") for f in range(count)]
            entries.append({"id": kind, "state": state, "size": size, "display_size": world,
                            "anchor": anchor, "appearances": 5, "directions": ["l", "r"],
                            "frames": count, "frame_ms": ms, "status": "style_b_draft",
                            "files": [m["file"] for m in mapping], "frame_map": mapping})
    entries.append({"id": "person_dead", "size": [120, 40], "display_size": [15, 5],
                    "anchor": [60, 20], "frames": 1,
                    "files": [f"person_dead_{look}_{d}_f0.png" for look in range(5) for d in ("l", "r")]})
    entries.extend(a for a in manifest["assets"] if a["id"] == "person_marker")
    manifest.update(status="style_b_draft", assets=entries, frame_aliases={})
    manifest_path.write_text(json.dumps(manifest, ensure_ascii=False, indent=2) + "\n")
    for kind, states in (("person", ADULT + [("build", 4, 250)]), ("person_child", CHILD)):
        for state, count, _ in states:
            for look in range(5):
                for d in ("l", "r"):
                    frames = [images[f"{kind}_{look}_{state}_{d}_f{f}"].tobytes() for f in range(count)]
                    if len(set(frames)) != count:
                        raise ValueError(f"Identische Phasen: {kind}/{look}/{state}/{d}")
    report = {"producer": "image_gen; proportionaler technischer Rasterexport",
              "new_adult_frames": 230, "new_child_frames": 150, "new_dead_frames": 10,
              "build_frames_padded_without_resizing": 40, "marker_unchanged": True,
              "scales": scales, "files": files, "mirrored_directions": False,
              "visual_approval": "pending", "game_review": "pending"}
    (REVIEW / "export.json").write_text(json.dumps(report, ensure_ascii=False, indent=2) + "\n")
    from refine_people_and_huts import CONFIG, apply_people
    if CONFIG.exists():
        apply_people(split, place)
        images = {name: Image.open(PEOPLE / (name + ".png")).convert("RGBA") for name in images}
    boards_and_animations(images)
    from export_fishing_people import REVIEW as FISH_REVIEW, export as export_fishing
    if (FISH_REVIEW / "sources.json").exists():
        export_fishing()
    from export_people_tools import ART as TOOLS_REVIEW, export as export_tools
    if (TOOLS_REVIEW / "sources.json").exists():
        export_tools()
    from export_canoe_and_hunt import ART as CANOE_REVIEW, export as export_canoe
    if (CANOE_REVIEW / "sources.json").exists():
        export_canoe()
    print("390 neue Tätigkeits-/Kinder-/Totenbilder, 40 Holztragebilder nur gepolstert.")


def boards_and_animations(images):
    font = load_font
    for kind, states in (("person", ADULT + [("build", 4, 250), ("dead", 1, None)]), ("person_child", CHILD)):
        board = Image.new("RGB", (1640, 90 + len(states) * 150), "#29372d")
        draw = ImageDraw.Draw(board)
        draw.text((20, 12), "Stil B / " + kind + " / synthetischer Kontaktbogen, keine Spielaufnahme", font=font(20), fill="#eee2ca")
        for look in range(5):
            x = 16 + look * 324
            draw.text((x, 50), f"Aussehen {look} – links / rechts", font=font(15), fill="#dccba7")
            for row, (state, count, ms) in enumerate(states):
                y = 85 + row * 150
                draw.text((x, y), state, font=font(15), fill="#dccba7")
                for index, d in enumerate(("l", "r")):
                    name = f"person_dead_{look}_{d}_f0" if state == "dead" else f"{kind}_{look}_{state}_{d}_f0"
                    image = images[name]
                    board.paste(image, (x + 20 + index * 145, y + 22), image)
        board.save(REVIEW / (kind + "-contact.png"))
        for state, count, ms in states:
            if count < 2:
                continue
            frames = []
            for frame in range(count):
                strip = Image.new("RGB", (1160, 160), "#48533d")
                draw = ImageDraw.Draw(strip)
                draw.text((8, 4), f"{kind} / {state} – fünf Aussehen, beide Richtungen", font=font(15), fill="#eee2ca")
                for look in range(5):
                    for index, d in enumerate(("l", "r")):
                        image = images[f"{kind}_{look}_{state}_{d}_f{frame}"]
                        strip.paste(image, (look * 232 + index * 114, 28), image)
                frames.append(strip)
            frames[0].save(REVIEW / f"{kind}-{state}.gif", save_all=True, append_images=frames[1:], duration=ms, loop=0)
    scale_sheet(images)


def scale_sheet(images):
    """Maßvergleich; die tatsächliche LOD-Darstellung wird im Spiel geprüft."""
    font = load_font
    sheet = Image.new("RGB", (1720, 940), "#30402f")
    draw = ImageDraw.Draw(sheet)
    manifest = json.loads((PEOPLE / "manifest.json").read_text())
    adult_width = next(a["display_size"][0] for a in manifest["assets"]
                       if a["id"] == "person" and a["state"] == "idle")
    draw.text((20, 16), "Menschen Stil B / synthetischer Maßvergleich – tatsächliche Spielaufnahmen separat", font=font(22), fill="#eee2ca")
    for row, zoom in enumerate((4, 8, 32)):
        y = 80 + row * 260
        draw.text((20, y), f"Zoom {zoom}", font=font(20), fill="#eee2ca")
        ground = Image.open(ROOT / "assets/nature/ground/ground_grass_wet_0.png").convert("RGBA")
        foreground = []
        for look in range(5):
            x = 280 + look * 280
            draw.text((x, y), f"Aussehen {look}: idle / walk · l/r", font=font(13), fill="#d6c9ae")
            for index, (state, direction) in enumerate((s, d) for s in ("idle", "walk") for d in ("l", "r")):
                foot = (x + 30 + index * 68, y + 190)
                tile = ground.resize((16 * zoom, 8 * zoom), Image.Resampling.NEAREST)
                sheet.paste(tile, (foot[0] - tile.width // 2, foot[1] - tile.height // 2), tile)
                image = images[f"person_{look}_{state}_{direction}_f0"]
                factor = adult_width / image.width * zoom / 4
                shown = image.resize((round(image.width * factor), round(image.height * factor)), Image.Resampling.NEAREST)
                foreground.append((shown, (foot[0] - round(56 * factor), foot[1] - round(112 * factor))))
        # Alle Böden vor sämtlichen Figuren zeichnen; große Rauten dürfen
        # keine zuvor gezeichnete Nachbarfigur im Prüfblatt überdecken.
        for shown, position in foreground:
            sheet.paste(shown, position, shown)
        # Vergleichsmotive in der tatsächlichen Weltbreite, nicht automatisch
        # auf dieselbe Pixelhöhe wie die Menschen gebracht.
        for index, (file, world_width, anchor) in enumerate([
            ("animals/deer_0_graze_r_f0.png", 14, (14, 22)),
            ("vegetation/tree_young_0_normal.png", 10, None),
        ]):
            path = ROOT / "assets/nature" / file
            image = Image.open(path).convert("RGBA")
            factor = world_width * zoom / 4 / image.width
            shown = image.resize((max(1, round(image.width * factor)), max(1, round(image.height * factor))), Image.Resampling.NEAREST)
            anchor = anchor or (image.width // 2, image.height - 2)
            foot = (65 + index * 120, y + 190)
            sheet.paste(shown, (foot[0] - round(anchor[0] * factor), foot[1] - round(anchor[1] * factor)), shown)
    sheet.save(ROOT / "assets/nature/_people-testsheet.png")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.parse_args()
    export()
