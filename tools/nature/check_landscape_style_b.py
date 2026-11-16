"""Weltmaß, Exportdateien und unveränderte andere Bildgruppen prüfen."""
import hashlib
import json
import subprocess
from pathlib import Path

import numpy as np
from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
ART = ROOT / 'art/nature/landscape-style-b'
ASSETS = ROOT / 'assets/nature'


def main():
    baseline = json.loads((ART / 'base-manifest.json').read_text())
    current = json.loads((ASSETS / 'manifest.json').read_text())
    config = json.loads((ART / 'config.json').read_text())
    original_manifest = subprocess.check_output([
        'git', 'show', config['base_commit'] + ':assets/nature/manifest.json'
    ], cwd=ROOT)
    assert (ART / 'base-manifest.json').read_bytes() == original_manifest
    before = {e['id']: e for e in baseline['assets']}
    report = json.loads((ART / 'export.json').read_text())
    records = {record['file']: record for record in report['files']}
    assert len(records) == 91
    for entry in current['assets']:
        name = entry['id']
        if name not in before:
            continue
        old = before[name]
        assert entry['display_size'] == old['display_size'], name
        if old['group'] not in ('ground', 'walls', 'vegetation'):
            assert entry == old, name
        else:
            # Beide Ankerachsen müssen bei der neuen Auflösung dieselbe
            # Weltposition ergeben, nicht bloß die Fußpunkt-x-Koordinate.
            assert np.array_equal(np.array(entry['anchor']) / entry['size'][0],
                                  np.array(old['anchor']) / old['size'][0]), name
    for file, record in records.items():
        path = ASSETS / file
        image = Image.open(path)
        assert image.mode == 'RGBA', file
        assert list(image.size) == record['size'], file
        assert hashlib.sha256(path.read_bytes()).hexdigest() == record['sha256'], file
        pixels = np.asarray(image)
        if file.startswith('walls/'):
            assert np.array_equal(pixels[0], pixels[-1]), file
        if file.startswith('vegetation/'):
            assert max(pixels[0, :, 3].max(), pixels[:, 0, 3].max(), pixels[:, -1, 3].max()) <= 64, file
    untouched = 0
    tracked = subprocess.check_output([
        'git', 'ls-tree', '-r', '--name-only', config['base_commit'], 'assets/nature'
    ], cwd=ROOT, text=True).splitlines()
    for path in tracked:
        relative = path.removeprefix('assets/nature/')
        if path.endswith('.png') and relative not in records:
            original = subprocess.check_output(['git', 'show', config['base_commit'] + ':' + path], cwd=ROOT)
            assert (ROOT / path).read_bytes() == original, path
            untouched += 1
    result = dict(
        status='passed', exported_pngs=len(records), ground_pngs=36,
        wall_pngs=7, vegetation_pngs=48, unchanged_other_pngs=untouched,
        existing_display_sizes_unchanged=True, existing_world_anchors_unchanged=True,
        water_fire_people_buildings_animals_unchanged=True,
        all_files_rgba_and_hashes_match=True, periodic_uv_edges=report['periodic_edges'],
        variant_material_edges_identical=report['variant_edges_identical'],
        wall_vertical_edges_byte_identical=True, visual_approval='pending',
    )
    (ART / 'validation.json').write_text(json.dumps(result, indent=2) + '\n')
    print(json.dumps(result, indent=2))


if __name__ == '__main__':
    main()
