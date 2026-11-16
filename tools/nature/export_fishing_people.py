"""Eigenständiger proportionaler _fish-Export; bestehende Menschen bleiben gleich."""
import hashlib
import json
from pathlib import Path

import numpy as np
from PIL import Image, ImageDraw
from export_people_style_b import split

ROOT = Path(__file__).resolve().parents[2]
REVIEW = ROOT / 'art/nature/fishing-people-style-b'
PEOPLE = ROOT / 'assets/nature/people'


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def foot(cut):
    a = np.asarray(cut)[:, :, 3] > 128
    ys, xs = np.nonzero(a)
    # Breite Sohlen bestimmen den Fußpunkt; die dünne Speerspitze zählt nicht.
    lower = ys >= ys.max() - max(3, round((ys.max() - ys.min()) * .06))
    return float(np.median(xs[lower])), float(ys.max() + 1), int(ys.min())


def export():
    cfg = json.loads((REVIEW / 'sources.json').read_text())
    report = []
    board = Image.new('RGBA', (4 * 240, 10 * 160), '#354032')
    draw = ImageDraw.Draw(board)
    for source in cfg['sources']:
        path = REVIEW / source['file']
        assert sha(path) == source['sha256'], path
        cuts = split(path, 4, 2)
        look = source['look']
        reference = Image.open(PEOPLE / f'person_{look}_idle_r_f0.png').convert('RGBA')
        b = reference.getchannel('A').point(lambda a: 255 if a > 128 else 0).getbbox()
        target = b[3] - b[1]
        factor = target / max(foot(c)[1] - foot(c)[2] for c in cuts)
        for i, cut in enumerate(cuts):
            direction, frame = ('r' if i < 4 else 'l'), i % 4
            fx, fy, head = foot(cut)
            scaled = cut.resize((round(cut.width * factor), round(cut.height * factor)), Image.Resampling.LANCZOS)
            x, y = round(56 - fx * factor), round(112 - fy * factor)
            assert x >= 2 and y >= 2 and x + scaled.width <= 110 and y + scaled.height <= 118, (source, i)
            image = Image.new('RGBA', (112, 120))
            image.alpha_composite(scaled, (x, y))
            name = f'person_{look}_fish_{direction}_f{frame}.png'
            image.save(PEOPLE / name)
            report.append({'file': name, 'sha256': sha(PEOPLE / name), 'size': [112, 120],
                           'anchor': [56, 112], 'display_size': [12.6, 13.5],
                           'source_factor': factor, 'source_foot': [fx, fy],
                           'body_world_height': round((fy - head) * factor * .1125, 3)})
            col, row = frame, look * 2 + (direction == 'l')
            board.alpha_composite(image, (col * 240 + 40, row * 160 + 30))
            draw.text((col * 240 + 8, row * 160 + 6), f'{look} / {direction} / f{frame}', fill='white')
    manifest_path = PEOPLE / 'manifest.json'
    manifest = json.loads(manifest_path.read_text())
    manifest['assets'] = [a for a in manifest['assets'] if not (a['id'] == 'person' and a.get('state') == 'fish')]
    manifest['assets'].append({'id': 'person', 'state': 'fish', 'size': [112, 120],
        'display_size': [12.6, 13.5], 'anchor': [56, 112], 'appearances': 5,
        'directions': ['l', 'r'], 'frames': 4, 'frame_ms': 300,
        'status': 'draft_pending_engine_state',
        'files': [r['file'] for r in report]})
    manifest_path.write_text(json.dumps(manifest, ensure_ascii=False, indent=2) + '\n')
    (REVIEW / 'export.json').write_text(json.dumps({'files': report, 'visual_approval': 'pending',
        'engine_selection': 'pending: fish activity and person_frames mapping by Claude',
        'mirrored_directions': False}, ensure_ascii=False, indent=2) + '\n')
    board.convert('RGB').save(REVIEW / 'contact.png')
    print('40 eigenständige _fish-Bilder exportiert; Kinder und bestehende Zustände unverändert.')


if __name__ == '__main__':
    export()
