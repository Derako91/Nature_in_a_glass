"""Fünf Gartenstadien × vier Varianten auf ganzen Bodenrauten exportieren."""
import hashlib
import json
from pathlib import Path

import numpy as np
from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parents[2]
ART = ROOT / 'art/nature/garden-style-b'
GROUND = ROOT / 'assets/nature/ground'
STAGES = ['bare', 'sprout', 'growing', 'ripe', 'stubble']


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def project(texture, cfg):
    # Einheitliche Grundraute, innen ein Beet mit schmalem weichem Grasübergang.
    # Dieselbe UV-Abbildung und Beetmaske für sämtliche Zustände/Varianten.
    yy, xx = np.mgrid[0:64, 0:128]
    u = (xx + .5 - 64) / 128 + (yy + .5) / 64
    v = (yy + .5) / 64 - (xx + .5 - 64) / 128
    extent = cfg["bed_fraction"]
    u, v = (u - .5) / extent + .5, (v - .5) / extent + .5
    edge = np.minimum.reduce([u, v, 1-u, 1-v])
    alpha = np.clip(edge / cfg["edge_feather_uv"], 0, 1)
    ix = np.clip(np.rint(u * 255), 0, 255).astype(int)
    iy = np.clip(np.rint(v * 255), 0, 255).astype(int)
    rgba = np.zeros((64, 128, 4), dtype=np.uint8)
    rgba[:, :, :3] = np.asarray(texture)[iy, ix]
    rgba[:, :, 3] = np.rint(alpha * 255).astype(np.uint8)
    rgba[alpha == 0] = 0
    return Image.fromarray(rgba)


def export():
    cfg = json.loads((ART / 'config.json').read_text())
    path = ART / cfg['source']['file']
    assert sha(path) == cfg['source']['sha256'], 'Gartenquelle geändert'
    source = Image.open(path).convert('RGB')
    assert cfg['rows'] == STAGES and cfg['columns'] == 4
    assert abs((source.width/4)/(source.height/5)-1) < .02, 'Materialzellen müssen quadratisch sein'
    entries, records = [], []
    board = Image.new('RGBA', (4*300, 5*185), '#354032')
    draw = ImageDraw.Draw(board)
    for row, stage in enumerate(STAGES):
        files = []
        for variant in range(4):
            # Flache Materialquadrate abtasten, anschließend ausdrücklich auf
            # die isometrische Bodenraute projizieren; keine Sprite-Streckung.
            box = (round(variant*source.width/4)+3, round(row*source.height/5)+3,
                   round((variant+1)*source.width/4)-3, round((row+1)*source.height/5)-3)
            texture = source.crop(box).resize((256, 256), Image.Resampling.LANCZOS)
            image = project(texture, cfg)
            name = f'garden_{stage}_{variant}.png'
            image.save(GROUND / name)
            files.append(name)
            records.append({'file': name, 'sha256': sha(GROUND / name), 'source_box': box,
                            'size': [128, 64], 'anchor': [64, 32], 'display_size': [64, 32]})
            x, y = variant*300, row*185
            draw.text((x+12, y+8), f'{stage} / {variant}', fill='white')
            board.alpha_composite(image.resize((256, 128), Image.Resampling.NEAREST), (x+22, y+35))
        entries.append({'id': f'garden_{stage}', 'group': 'ground', 'size': [128, 64],
                        'anchor': [64, 32], 'display_size': [64, 32], 'variants': 4,
                        'resolution_scale': 2, 'frames': 1, 'status': 'garden_style_b_draft', 'files': files})
    path = GROUND / 'manifest.json'
    manifest = json.loads(path.read_text()) if path.exists() else {'version': 1, 'assets': []}
    manifest['assets'] = [a for a in manifest['assets'] if not a['id'].startswith('garden_')] + entries
    path.write_text(json.dumps(manifest, ensure_ascii=False, indent=2)+'\n')
    (ART / 'export.json').write_text(json.dumps({'files': records, 'visual_approval': 'pending'}, indent=2)+'\n')
    board.convert('RGB').save(ART / 'contact.png')
    print('20 Gartenbilder, ganze 128×64-Raute, Weltmaß 64×32, gemeinsame Beetmaske.')


if __name__ == '__main__':
    export()
