"""Technischer Einzelbildexport generierter Lager-/Bau-Prüfbögen (17.8).

Keine Zeichnung oder Spiegelung: Die Motive und beide Menschenrichtungen
stammen aus image_gen. Dieses Werkzeug trennt Rasterzellen, skaliert sie auf
die vereinbarten Dateimaße und schreibt Fußpunkte und Prüfnachweise.
"""
import argparse
import hashlib
import json
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parents[2]
REVIEW = ROOT / "art/nature/camp-review"
BUILDINGS = ROOT / "assets/nature/buildings"
PEOPLE = ROOT / "assets/nature/people"
FONT = "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf"


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def cells(path, cols, rows):
    source = Image.open(path).convert("RGBA")
    result = []
    for row in range(rows):
        for col in range(cols):
            box = (round(col * source.width / cols),
                   round(row * source.height / rows),
                   round((col + 1) * source.width / cols),
                   round((row + 1) * source.height / rows))
            cell = source.crop(box)
            # Nur eine technische Begrenzung: transparente RGB-Reste gehören
            # nicht zum Motiv, schwache Kontakt-Schatten bleiben im Ausschnitt.
            bounds = cell.getchannel("A").point(lambda a: 255 if a > 16 else 0).getbbox()
            if bounds is None:
                raise ValueError(f"Leere Rasterzelle {row}/{col} in {path}")
            result.append((cell.crop(bounds), box, bounds))
    return result


def place(cut, size, anchor, factor=None, fit=None):
    if fit:
        target = fit
    else:
        factor = factor or min((size[0] - 4) / cut.width,
                               (size[1] - 8) / cut.height)
        target = (max(1, round(cut.width * factor)),
                  max(1, round(cut.height * factor)))
    scaled = cut.resize(target, Image.Resampling.LANCZOS)
    image = Image.new("RGBA", size)
    image.alpha_composite(scaled, (round(anchor[0] - target[0] / 2),
                                   anchor[1] - target[1]))
    return image


def export(camp_source, right_source, left_source):
    BUILDINGS.mkdir(parents=True, exist_ok=True)
    REVIEW.mkdir(parents=True, exist_ok=True)
    if (BUILDINGS / "manifest.json").exists():
        raise ValueError("Gebäudemanifest existiert; nicht automatisch überschreiben.")
    icc = Image.open(ROOT / "assets/nature/ground/ground_soil_0.png").info.get("icc_profile")
    camp = cells(camp_source, 3, 3)
    # Breiten aus 17.8, Dateifeinheit 8×. Hütten bewusst 30 Welteinheiten
    # hoch (Menschen: 15). Beide Varianten erhalten dasselbe Hüllmaß.
    specs = [
        ("camp_hearth", (48, 32), (6, 4), (24, 30)),
        ("camp_windbreak", (96, 112), (12, 14), (48, 108)),
        ("camp_hut_0", (128, 240), (16, 30), (64, 236)),
        ("camp_hut_1", (128, 240), (16, 30), (64, 236)),
        ("camp_ruin", (48, 40), (6, 5), (24, 38)),
    ]
    images, entries, records = {}, [], []

    def save(name, image, folder):
        path = folder / (name + ".png")
        if path.exists():
            raise ValueError(f"Datei existiert bereits: {path}")
        image.save(path, icc_profile=icc)
        images[name] = image
        records.append({"file": str(path.relative_to(ROOT)), "sha256": digest(path),
                        "size": list(image.size), "mode": image.mode,
                        "bbox": image.getchannel("A").getbbox()})

    for index, (name, size, world, anchor) in enumerate(specs):
        fit = (120, 232) if name.startswith("camp_hut_") else None
        image = place(camp[index][0], size, anchor, fit=fit)
        save(name, image, BUILDINGS)
        entries.append({"id": name, "files": [name + ".png"], "size": size,
                        "display_size": world, "anchor": anchor})

    # Derselbe Steinring in allen Phasen, damit der Boden nicht mitflackert.
    # Die vier unterschiedlichen Flammen stammen aus dem bereits eingebauten
    # image_gen-Feuerpaket (PR #10), nicht aus kopierten Scheinphasen.
    fire_sources = [ROOT / f"assets/nature/objects/fire_flame_small_0_f{i}.png" for i in range(4)]
    for frame, source in enumerate(fire_sources):
        name = f"camp_fire_f{frame}"
        image = Image.new("RGBA", (48, 56))
        image.alpha_composite(images["camp_hearth"], (0, 22))
        flame = Image.open(source).convert("RGBA").resize((20, 32), Image.Resampling.LANCZOS)
        image.alpha_composite(flame, (14, 10))
        save(name, image, BUILDINGS)
    entries.append({"id": "camp_fire", "files": [f"camp_fire_f{i}.png" for i in range(4)],
                    "size": [48, 56], "display_size": [6, 7], "anchor": [24, 52],
                    "frames": 4, "frame_ms": 200})
    (BUILDINGS / "manifest.json").write_text(json.dumps({
        "version": 1, "status": "draft_visual_review", "base_tile": [64, 32],
        "assets": entries, "frame_aliases": {},
        "note": "Zeittakt kommt aus dem Beobachter; frame_ms dokumentiert 17.8."
    }, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")

    directions = {"r": cells(right_source, 4, 5), "l": cells(left_source, 4, 5)}
    mapping = []
    for look in range(5):
        # Ein Skalierungsfaktor je Aussehen, über alle Phasen/Richtungen.
        group = [directions[d][look * 4 + f][0] for d in ("l", "r") for f in range(4)]
        factor = min(52 / max(c.width for c in group), 104 / max(c.height for c in group))
        for direction in ("l", "r"):
            for frame in range(4):
                name = f"person_{look}_build_{direction}_f{frame}"
                cut = directions[direction][look * 4 + frame][0]
                save(name, place(cut, (56, 120), (28, 112), factor=factor), PEOPLE)
                mapping.append({"appearance": look, "direction": direction,
                                "frame": frame, "file": name + ".png"})
    manifest_path = PEOPLE / "manifest.json"
    manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
    if any(entry.get("state") == "build" for entry in manifest["assets"]):
        raise ValueError("Menschenmanifest enthält bereits build.")
    manifest["assets"].append({"id": "person", "state": "build",
        "size": [56, 120], "display_size": [7, 15], "anchor": [28, 112],
        "appearances": 5, "directions": ["l", "r"], "frames": 4, "frame_ms": 250,
        "status": "draft_visual_review", "files": [m["file"] for m in mapping],
        "frame_map": mapping})
    manifest_path.write_text(json.dumps(manifest, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")

    font = lambda size: ImageFont.truetype(FONT, size)
    board = Image.new("RGB", (1500, 1110), "#202b29")
    draw = ImageDraw.Draw(board)
    draw.text((24, 15), "Lagerplätze – Einzelbild-Draft / Stil B", font=font(26), fill="#f1e4c9")
    draw.text((24, 52), "Synthetischer Kontaktbogen. Spielprüfung als separate Aufnahmen.", font=font(17), fill="#c0cabc")
    for index, (name, image) in enumerate((n, im) for n, im in images.items() if n.startswith("camp_")):
        x, y = 24 + index % 5 * 294, 100 + index // 5 * 300
        panel = Image.new("RGBA", (276, 260), "#48533d")
        factor = min(2, 248 / image.width, 244 / image.height)
        preview = image.resize((round(image.width * factor), round(image.height * factor)), Image.Resampling.NEAREST)
        panel.alpha_composite(preview, ((276 - preview.width) // 2, 250 - preview.height))
        board.paste(panel.convert("RGB"), (x, y))
        draw.text((x, y + 264), name, font=font(16), fill="#f1e4c9")
    for look in range(5):
        x = 30 + look * 294
        draw.text((x, 735), f"Aussehen {look} / Holz tragen", font=font(17), fill="#f1e4c9")
        for index, direction in enumerate(("l", "r")):
            im = images[f"person_{look}_build_{direction}_f0"].resize((112, 240), Image.Resampling.NEAREST)
            board.paste(im, (x + index * 130, 780), im)
    board.save(REVIEW / "einzelbilder.png", icc_profile=icc)

    animation = []
    for frame in range(4):
        strip = Image.new("RGB", (1200, 310), "#48533d")
        draw = ImageDraw.Draw(strip)
        for look in range(5):
            for index, direction in enumerate(("l", "r")):
                im = images[f"person_{look}_build_{direction}_f{frame}"].resize((112, 240), Image.Resampling.NEAREST)
                strip.paste(im, (look * 240 + index * 118, 35), im)
        draw.text((16, 8), "Build: fünf Aussehen, l/r, vier Phasen à 250 ms", font=font(18), fill="#f1e4c9")
        animation.append(strip)
    animation[0].save(REVIEW / "build.gif", save_all=True, append_images=animation[1:], duration=250, loop=0)
    animation = []
    for frame in range(4):
        panel = Image.new("RGB", (288, 336), "#48533d")
        im = images[f"camp_fire_f{frame}"].resize((288, 336), Image.Resampling.NEAREST)
        panel.paste(im, (0, 0), im)
        animation.append(panel)
    animation[0].save(REVIEW / "camp-fire.gif", save_all=True, append_images=animation[1:], duration=200, loop=0)
    report = {"producer": "image_gen; technischer Rasterexport mit Pillow",
        "sources": [{"name": name, "sha256": digest(path)} for name, path in
                    [("camp", camp_source), ("people_right", right_source), ("people_left", left_source)]],
        "mirrored_frames": False, "building_count": 9, "build_frame_count": 40,
        "hut_export": "Hüllmaß beider Hütten auf 120×232 px normiert; Leinwand 128×240, Weltmaß 16×30",
        "fire_source_files": [{"file": str(p.relative_to(ROOT)), "sha256": digest(p)} for p in fire_sources],
        "fire_export": "Fester kalter Steinring + vier vorhandene generierte Flammen, einheitlich 20×32 px; Ring unverändert.",
        "files": records,
        "visual_approval": "pending", "game_review": "pending"}
    (REVIEW / "export.json").write_text(json.dumps(report, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print("Exportiert: 9 Lagerbilder, 40 Bauphasen, Manifeste und Prüfbögen.")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("camp", type=Path)
    parser.add_argument("right", type=Path)
    parser.add_argument("left", type=Path)
    args = parser.parse_args()
    export(args.camp, args.right, args.left)
