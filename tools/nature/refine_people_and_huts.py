"""Erwachsenenmaß, kompakte Tätigkeiten und größere Hüttentüren nach Auftrag.

Die Bildgenerator-Quellen sind unter art/nature/people-hut-adjustments archiviert.
Hütten erhalten ausschließlich einen registrierten Türbereich; ihr Alpha und
alle Pixel außerhalb dieses Bereichs bleiben gleich.
"""
import argparse
import hashlib
import json
import statistics
from pathlib import Path

import numpy as np
from PIL import Image, ImageDraw, ImageFilter
from scipy import ndimage

ROOT = Path(__file__).resolve().parents[2]
REVIEW = ROOT / "art/nature/people-hut-adjustments"
PEOPLE = ROOT / "assets/nature/people"
BUILDINGS = ROOT / "assets/nature/buildings"
CONFIG = REVIEW / "config.json"


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def config():
    value = json.loads(CONFIG.read_text())
    for source in value["sources"]:
        if sha(REVIEW / source["file"]) != source["sha256"]:
            raise ValueError(f"Geänderte Quelle: {source['file']}")
    return value


def opaque_box(image):
    return image.getchannel("A").point(lambda a: 255 if a > 128 else 0).getbbox()


def apply_people(splitter=None, placer=None):
    if not CONFIG.exists():
        return
    if splitter is None:
        from export_people_style_b import split, place
        splitter, placer = split, place
    cfg = config()
    manifest_path = PEOPLE / "manifest.json"
    manifest = json.loads(manifest_path.read_text())
    scale = cfg["adult_scale"]
    # Absolut vom bekannten Grundmaß setzen: Wiederholung schrumpft nicht weiter.
    for entry in manifest["assets"]:
        if entry["id"] == "person":
            entry["display_size"] = [14 * scale, 15 * scale]
        elif entry["id"] == "person_dead":
            entry["display_size"] = [15 * scale, 5 * scale]
    rows = []
    for source in cfg["sources"]:
        if source["kind"] != "compact":
            continue
        look = source["look"]
        cuts = splitter(REVIEW / source["file"], 4, 5)
        hands = next(s for s in cfg["sources"] if s["kind"] == "hands" and s["look"] == look)
        hand_cuts = splitter(REVIEW / hands["file"], 4, 5)
        for direction, refs, gather, eat, drink in [
            ("r", (0, 2), range(4, 8), (12, 13), (16, 17)),
            ("l", (1, 3), range(8, 12), (14, 15), (18, 19)),
        ]:
            idle = Image.open(PEOPLE / f"person_{look}_idle_{direction}_f0.png").convert("RGBA")
            box = opaque_box(idle)
            height = box[3] - box[1]
            reference = statistics.median(cuts[i].height - 8 for i in refs)
            factor = height / reference
            selected = [cuts[i] for i in (*gather, *eat, *drink)]
            max_width = max(opaque_box(c)[2] - opaque_box(c)[0] for c in selected)
            pixel_world = 14 * scale / 112
            factor = min(factor, cfg["gather_max_world_width"] / pixel_world / max_width)
            for state, indices in (("gather", gather), ("eat", eat), ("drink", drink)):
                state_cuts = cuts if state == "gather" else hand_cuts
                state_factor = factor
                if state != "gather":
                    state_factor = height / statistics.median(hand_cuts[i].height - 8 for i in refs)
                    width = max(opaque_box(hand_cuts[i])[2] - opaque_box(hand_cuts[i])[0] for i in (*eat, *drink))
                    state_factor = min(state_factor, cfg["gather_max_world_width"] / pixel_world / width)
                for frame, index in enumerate(indices):
                    path = PEOPLE / f"person_{look}_{state}_{direction}_f{frame}.png"
                    icc = Image.open(path).info.get("icc_profile")
                    image = placer(state_cuts[index], (112, 120), (56, 112), state_factor)
                    image.save(path, icc_profile=icc)
                    box = opaque_box(image)
                    rows.append({"file": path.name, "state": state, "look": look,
                                 "direction": direction, "frame": frame,
                                 "opaque_size": [box[2] - box[0], box[3] - box[1]],
                                 "world_width": (box[2] - box[0]) * pixel_world,
                                 "factor": state_factor, "sha256": sha(path)})
    manifest_path.write_text(json.dumps(manifest, ensure_ascii=False, indent=2) + "\n")
    # Exportnachweise der bestehenden Menschen-Pipeline aktuell halten.
    report_path = ROOT / "art/nature/people-style-b/export.json"
    if report_path.exists():
        report = json.loads(report_path.read_text())
        for record in report["files"]:
            path = PEOPLE / record["file"]
            image = Image.open(path)
            record.update(sha256=sha(path), bbox=image.getchannel("A").getbbox())
        report["adult_scale"] = scale
        report["compact_refinement"] = str(CONFIG.relative_to(ROOT))
        report_path.write_text(json.dumps(report, ensure_ascii=False, indent=2) + "\n")
    (REVIEW / "people-adjustment.json").write_text(json.dumps({
        "adult_scale": scale, "children": "unchanged", "anchors": "unchanged",
        "frames": rows,
    }, indent=2) + "\n")


def entrance_box(image, variant):
    pixels = np.asarray(image.convert("RGBA"))
    mask = ((pixels[:, :, 0] < 65) & (pixels[:, :, 1] < 40)
            & (pixels[:, :, 2] < 30) & (pixels[:, :, 3] > 128))
    labels, _ = ndimage.label(mask)
    sizes = np.bincount(labels.ravel())
    candidates = []
    for label, (ys, xs) in enumerate(ndimage.find_objects(labels), 1):
        center_x = (xs.start + xs.stop) / 2 / image.width
        center_y = (ys.start + ys.stop) / 2 / image.height
        if center_y > 0.4 and ((variant == 0 and center_x > 0.45)
                              or (variant == 1 and center_x < 0.55)):
            candidates.append((sizes[label], (xs.start, ys.start, xs.stop, ys.stop)))
    if not candidates:
        raise ValueError("Keine dunkle Türöffnung gefunden")
    return max(candidates)[1]


def apply_hut_doors():
    if not CONFIG.exists():
        return
    cfg = config()
    records = []
    for variant, target in [(0, (102, 95, 138, 167)), (1, (25, 77, 61, 149))]:
        original_path = REVIEW / f"sources/hut-original-{variant}.png"
        original = Image.open(original_path).convert("RGBA")
        path = BUILDINGS / f"camp_hut_{variant}.png"
        current = Image.open(path).convert("RGBA")
        if current.size != original.size or current.getchannel("A").tobytes() != original.getchannel("A").tobytes():
            raise ValueError("Hüttenkontur geändert; Türkorrektur muss neu abgestimmt werden.")
        source = next(s for s in cfg["sources"] if s["kind"] == "door" and s["variant"] == variant)
        generated = Image.open(REVIEW / source["file"]).convert("RGBA")
        box = entrance_box(generated, variant)
        sx = (box[2] - box[0]) / (target[2] - target[0])
        sy = (box[3] - box[1]) / (target[3] - target[1])
        # Nur den generierten Türbereich auf sein vereinbartes Maß registrieren;
        # die vorhandene Hütte wird weder skaliert noch durch die neue ersetzt.
        registered = generated.transform(original.size, Image.Transform.AFFINE,
            (sx, 0, box[0] - sx * target[0], 0, sy, box[1] - sy * target[1]),
            Image.Resampling.BICUBIC)
        region = (target[0] - 8, target[1] - 8, target[2] + 8, target[3] + 8)
        mask = Image.new("L", original.size)
        ImageDraw.Draw(mask).rectangle(region, fill=255)
        # Kleine weiche Naht nur im umliegenden Holz, die Öffnung selbst bleibt
        # vollständig im generierten Bereich. Alpha kommt immer vom Original.
        mask = mask.filter(ImageFilter.GaussianBlur(1.2))
        result = Image.composite(registered, original, mask)
        result.putalpha(original.getchannel("A"))
        result.save(path, icc_profile=Image.open(original_path).info.get("icc_profile"))
        records.append({"file": str(path.relative_to(ROOT)), "size": result.size,
                        "registered_opening": target, "opening_world": [4.5, 9],
                        "original_sha256": sha(original_path), "sha256": sha(path),
                        "alpha_unchanged": result.getchannel("A").tobytes() == original.getchannel("A").tobytes(),
                        "changed_region": mask.getbbox()})
    report_path = ROOT / "art/nature/camp-review/export.json"
    if report_path.exists():
        report = json.loads(report_path.read_text())
        for record in report["files"]:
            path = ROOT / record["file"]
            image = Image.open(path)
            record.update(sha256=sha(path), size=list(image.size),
                          bbox=image.getchannel("A").getbbox())
        report["door_refinement"] = str(CONFIG.relative_to(ROOT))
        report_path.write_text(json.dumps(report, ensure_ascii=False, indent=2) + "\n")
    (REVIEW / "door-adjustment.json").write_text(json.dumps(records, indent=2) + "\n")


def main():
    apply_people()
    apply_hut_doors()
    from export_people_style_b import boards_and_animations
    images = {p.stem: Image.open(p).convert("RGBA") for p in PEOPLE.glob("*.png")}
    boards_and_animations(images)
    print("Erwachsene 90 %, kompakte Tätigkeiten und Türöffnungen angewandt.")


if __name__ == "__main__":
    argparse.ArgumentParser(description=__doc__).parse_args()
    main()
