"""Fernbildsatz und Unverändertheit des vollständigen Nahbildpakets prüfen."""
import hashlib
import io
import json
import subprocess
from pathlib import Path

import numpy as np
from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
ART = ROOT / 'art/nature/landscape-far-crowns'
ASSETS = ROOT / 'assets/nature'


def original(commit, path):
    return subprocess.check_output(['git', 'show', commit + ':' + str(path)], cwd=ROOT)


def silhouette_area(image):
    # Farbunabhängig: zählt auch olivgelbe Blätter, schmale Äste und
    # halbtransparente Kanten mit ihrer tatsächlichen Deckung.
    return float(np.asarray(image.convert('RGBA'))[:, :, 3].astype(float).sum() / 255)


def main():
    cfg = json.loads((ART / 'config.json').read_text())
    commit = cfg['base_commit']
    baseline = json.loads((ART / 'base-manifest.json').read_text())
    assert (ART / 'base-manifest.json').read_bytes() == original(commit, 'assets/nature/manifest.json')
    current = json.loads((ASSETS / 'manifest.json').read_text())
    before = {e['id']: e for e in baseline['assets']}
    records = json.loads((ART / 'export.json').read_text())['files']
    assert len(records) == 32
    measured = {}
    for entry in current['assets']:
        old = before[entry['id']]
        plain = {k: v for k, v in entry.items() if k != 'levels'}
        assert plain == {k: v for k, v in old.items() if k != 'levels'}, entry['id']
        if entry['id'] not in ('tree_young', 'tree_mid', 'tree_old', 'shrub'):
            assert entry == old
            continue
        level = next(l for l in entry['levels'] if l['label'] == '@fern')
        assert len(level['files']) == 8
        assert level['anchor'] == [v / 4 for v in entry['anchor']]
        measurements = []
        for file in level['files']:
            image = Image.open(ASSETS / file)
            assert image.mode == 'RGBA'
            assert list(image.size) == [v // 4 for v in entry['size']]
            assert hashlib.sha256((ASSETS / file).read_bytes()).hexdigest() == next(r['sha256'] for r in records if r['file'] == file)
            if '_normal@' in file:
                mainfile = file.replace('@fern', '')
                near = Image.open(ASSETS / mainfile)
                legacy = Image.open(io.BytesIO(original('8043787', 'assets/nature/' + mainfile)))
                measurements.append(dict(file=file, fern=silhouette_area(image),
                                         thin_pr17=silhouette_area(near) / 16,
                                         legacy=silhouette_area(legacy)))
        measured[entry['id']] = measurements
    tracked = subprocess.check_output(['git', 'ls-tree', '-r', '--name-only', commit, 'assets/nature'], cwd=ROOT, text=True).splitlines()
    count = 0
    for file in tracked:
        if file.endswith('.png'):
            assert (ROOT / file).read_bytes() == original(commit, file), file
            count += 1
    value = dict(status='passed',far_images=32,all_existing_png_byte_identical=count,
                 only_far_manifest_levels_added=True,world_sizes_and_anchors_unchanged=True,
                 silhouette_area_at_two_pixels_per_world_unit=measured,visual_approval='pending')
    (ART / 'validation.json').write_text(json.dumps(value, indent=2) + '\n')
    print(f'32 Fernbilder geprüft; alle {count} vorhandenen PNGs bytegleich.')
    for kind, rows in measured.items():
        sums = {key: sum(r[key] for r in rows) for key in ('fern', 'thin_pr17', 'legacy')}
        print(kind, 'Silhouettenfläche gegenüber PR17:', round(sums['fern'] / sums['thin_pr17'], 2),
              'gegenüber altem Satz:', round(sums['fern'] / sums['legacy'], 2))


if __name__ == '__main__':
    main()
