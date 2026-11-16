"""Eigene breite Baum-/Buschkronen für Zoom 8 und darunter exportieren."""
import hashlib
import json
from pathlib import Path

import numpy as np
from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parents[2]
ART = ROOT / 'art/nature/landscape-far-crowns'
ASSETS = ROOT / 'assets/nature'
CONFIG = ART / 'config.json'
KINDS = ['tree_young', 'tree_mid', 'tree_old', 'shrub']


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def export():
    from export_people_style_b import split
    cfg = json.loads(CONFIG.read_text())
    manifest_path = ASSETS / 'manifest.json'
    manifest = json.loads(manifest_path.read_text())
    records = []
    for name in KINDS:
        source = next(s for s in cfg['sources'] if s['id'] == name)
        path = ART / source['file']
        if sha(path) != source['sha256']:
            raise ValueError(f'Geänderte Fernquelle: {path}')
        cuts = split(path, 4, 2)
        entry = next(e for e in manifest['assets'] if e['id'] == name)
        size = tuple(v // 4 for v in entry['size'])
        anchor = tuple(v / 4 for v in entry['anchor'])
        files = []
        for variant in range(4):
            pair = [cuts[variant], cuts[variant + 4]]
            feet = []
            factors = []
            for cut in pair:
                a = np.asarray(cut)[:, :, 3]
                ys, xs = np.where(a > 160)
                fy = int(ys.max()) + 1
                fx = float(np.median(xs[ys >= fy - 4])) + .5
                if name == 'shrub':
                    fx = cut.width / 2
                feet.append((fx, fy))
                factors.append(min((anchor[0] - 1) / fx,
                                   (size[0] - anchor[0] - 1) / (cut.width - fx),
                                   (anchor[1] - 1) / fy,
                                   (size[1] - anchor[1] - 1) / max(1, cut.height - fy)))
            factor = min(factors)
            for state, cut, (fx, fy) in zip(('normal', 'stressed'), pair, feet):
                image = Image.new('RGBA', size)
                scaled = cut.resize((max(1, round(cut.width * factor)),
                                     max(1, round(cut.height * factor))), Image.Resampling.LANCZOS)
                image.alpha_composite(scaled, (round(anchor[0] - fx * factor), round(anchor[1] - fy * factor)))
                file = f'vegetation/{name}_{variant}_{state}@fern.png'
                image.save(ASSETS / file)
                files.append(file)
                alpha = np.asarray(image)[:, :, 3]
                box = image.getchannel('A').point(lambda a: 255 if a > 128 else 0).getbbox()
                records.append(dict(file=file, size=size, anchor=anchor, sha256=sha(ASSETS / file),
                                    opaque_bbox=box, alpha_area=float(alpha.sum() / 255),
                                    factor=factor))
        levels = [l for l in entry.get('levels', []) if l['label'] != '@fern']
        levels.append(dict(label='@fern', files=files, anchor=anchor))
        entry['levels'] = levels
    manifest_path.write_text(json.dumps(manifest, ensure_ascii=False, indent=2) + '\n')
    (ART / 'export.json').write_text(json.dumps(dict(files=records, main_images='unchanged'), indent=2) + '\n')
    board(records)
    print('32 eigene Fernbilder exportiert; Nahbilder, Weltmaß und Fußpunkte unverändert.')


def board(records):
    font = ImageFont.truetype('DejaVuSans.ttf', 17)
    image = Image.new('RGB', (1200, 50 + 8 * 260), '#384333')
    draw = ImageDraw.Draw(image)
    draw.text((10, 12), 'Eigene Fernkronen / vergrößerter Kontaktbogen, keine Spielaufnahme', font=font, fill='#eadfc8')
    for i, r in enumerate(records):
        x, y = (i % 4) * 300, 50 + (i // 4) * 260
        draw.text((x + 8, y + 8), Path(r['file']).stem, font=font, fill='#eadfc8')
        sprite = Image.open(ASSETS / r['file']).convert('RGBA')
        factor = min(6, 220 / sprite.height)
        sprite = sprite.resize((round(sprite.width * factor), round(sprite.height * factor)), Image.Resampling.NEAREST)
        image.paste(sprite, (x + (300 - sprite.width) // 2, y + 35 + (220 - sprite.height) // 2), sprite)
    image.save(ART / 'fern-contact.png')


if __name__ == '__main__':
    export()
