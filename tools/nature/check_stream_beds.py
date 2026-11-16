"""Anschlussprofile, vollständige Bachstücke und unveränderte übrige Bilder."""
import hashlib
import json
import subprocess
from pathlib import Path

import numpy as np
from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
ART = ROOT / 'art/nature/stream-beds-style-b'
WATER = ROOT / 'assets/nature/water'


def main():
    from export_stream_beds import ENDPOINTS, OPPOSITE, geometry
    cfg = json.loads((ART / 'config.json').read_text())
    report = json.loads((ART / 'export.json').read_text())
    manifest = json.loads((WATER / 'manifest.json').read_text())
    files = {f for e in manifest['assets'] if e['id'].startswith('stream_') for f in e['files']}
    assert len(files) == len(report['files']) == 54
    images = {}
    for row in report['files']:
        file = row['file']
        assert file in files
        image = Image.open(WATER / file)
        assert image.mode == 'RGBA' and image.size == (128, 64)
        assert sha(WATER / file) == row['sha256']
        data = np.asarray(image).astype(int)
        assert data[32, 64, 3] > (128 if row['part'] == 'c' else 0)
        assert data[0, 0, 3] == data[-1, -1, 3] == 0
        images[file[:-4]] = data
    checks = []
    for strength in range(3):
        for part in ['c'] + list(ENDPOINTS):
            assert np.array_equal(images[f'stream_wet_{strength}_{part}'][:, :, 3],
                                  images[f'stream_dry_{strength}_{part}'][:, :, 3])
        for state in ('wet', 'dry'):
            for direction, opposite in OPPOSITE.items():
                a = images[f'stream_{state}_{strength}_{direction}']
                b = images[f'stream_{state}_{strength}_{opposite}']
                if direction == 'n':
                    pa, pb = a[0], b[-1]
                elif direction == 'e':
                    pa, pb = a[:, -1], b[:, 0]
                else:
                    xs = np.arange(78, 115)
                    if direction == 'ne':
                        ys = np.floor(16 + (xs - 96) / 2).astype(int)
                        pa, pb = a[ys, xs], b[ys + 32, xs - 64]
                    else:
                        ys = np.floor(48 - (xs - 96) / 2).astype(int)
                        pa, pb = a[ys, xs], b[ys - 32, xs - 64]
                # Farben dürfen nur um die letzte Rundungsstufe abweichen;
                # Alpha-Querschnitte bleiben pixelgenau deckungsgleich.
                delta = int(np.abs(pa - pb).max())
                alpha_delta = int(np.abs(pa[:, 3] - pb[:, 3]).max())
                assert alpha_delta <= 1, (state, strength, direction, alpha_delta)
                assert delta <= 2, (state, strength, direction, delta)
                assert np.array_equal(pa[:, 3] > 128, pb[:, 3] > 128)
                # Zusätzlich exakter geometrischer Querschnitt, unabhängig
                # von der Treppenlage des 2:1-Pixelrasters.
                end = np.array(ENDPOINTS[direction], float)
                vector = end - .5
                vector /= np.linalg.norm(vector)
                normal = np.array([-vector[1], vector[0]])
                offsets = np.linspace(-.18, .18, 1001)
                p = end[:, None] + normal[:, None] * offsets
                q = np.array(ENDPOINTS[opposite])[:, None] + normal[:, None] * offsets
                da = geometry(direction, p[0][None, :], p[1][None, :])[0]
                db = geometry(opposite, q[0][None, :], q[1][None, :])[0]
                assert np.allclose(da, db, atol=1e-10)
                checks.append(dict(state=state, strength=strength, pair=[direction, opposite],
                                   maximum_rgba_rounding_delta=delta,alpha_delta=alpha_delta,
                                   opaque_width_samples=int((pa[:, 3] > 128).sum())))
    # Alle 256 Richtungsbelegungen je Stärke im tatsächlichen Malablauf
    # zusammensetzen. Jede Mündung muss mit dem Wasser in der Mitte verbunden
    # bleiben; ein späterer Uferstreifen darf keinen grauen Querdamm erzeugen.
    from scipy import ndimage
    directions = list(ENDPOINTS)
    pixels_ends = {'n': (64, 0), 'ne': (96, 16), 'e': (127, 32), 'se': (96, 48),
                   's': (64, 63), 'sw': (32, 48), 'w': (0, 32), 'nw': (32, 16)}
    for strength in range(3):
        parts = {d: Image.open(WATER / f'stream_wet_{strength}_{d}.png').convert('RGBA')
                 for d in ['c'] + directions}
        for bits in range(256):
            composite = parts['c'].copy()
            for i, d in enumerate(directions):
                if bits & (1 << i):
                    composite.alpha_composite(parts[d])
            a = np.asarray(composite).astype(int)
            labels, _ = ndimage.label((a[:, :, 2] > a[:, :, 0] + 5) & (a[:, :, 3] > 48), np.ones((3, 3)))
            sizes = np.bincount(labels.ravel())
            sizes[0] = 0
            root = int(sizes.argmax())
            assert root and (labels[30:34, 62:66] == root).any(), (strength, bits, 'Mitte')
            for i, d in enumerate(directions):
                if bits & (1 << i):
                    x, y = pixels_ends[d]
                    assert (labels[max(0, y-1):min(64, y+2), max(0, x-1):min(128, x+2)] == root).any(), (strength, bits, d)
    untouched = 0
    paths = subprocess.check_output(['git','ls-tree','-r','--name-only',cfg['base_commit'],'assets/nature'],cwd=ROOT,text=True).splitlines()
    for file in paths:
        if file.endswith('.png'):
            assert (ROOT / file).read_bytes() == subprocess.check_output(['git','show',cfg['base_commit']+':'+file],cwd=ROOT),file
            untouched += 1
    value = dict(status='passed',png_files=54,whole_tile_size=[128,64],world_size=[64,32],
                 unchanged_parent_pngs=untouched,wet_dry_same_bed_width=True,
                 exact_geometric_end_sections=True,pixel_endpoint_checks=checks,
                 connected_water_junction_configurations=768,
                 visual_approval='pending')
    (ART/'validation.json').write_text(json.dumps(value,indent=2)+'\n')
    print(f'54 Bachstücke, 24 Gegenprofile und alle {untouched} Eltern-PNGs geprüft.')


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


if __name__ == '__main__':
    main()
