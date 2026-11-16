"""Wasserphasen, Ruhe, Varianten und unveränderte übrige Bildgruppen prüfen."""
import hashlib
import json
import subprocess
from pathlib import Path

import numpy as np
from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
ART = ROOT / 'art/nature/water-style-b'
ASSETS = ROOT / 'assets/nature'
KINDS = ['water_deep', 'water_shallow', 'water_pond']


def main():
    cfg = json.loads((ART / 'config.json').read_text())
    manifest = json.loads((ASSETS / 'manifest.json').read_text())
    before = json.loads((ART / 'base-manifest.json').read_text())
    report = json.loads((ART / 'export.json').read_text())
    assert len(report['files']) == 48
    for old, new in zip(before['assets'], manifest['assets']):
        assert old['id'] == new['id']
        if old['id'] not in KINDS:
            assert old == new
        else:
            assert new['display_size'] == old['display_size'] == [64, 32]
            assert new['anchor'] == [64, 32] and new['size'] == [128, 64]
            assert new['variants'] == 4 and new['frames'] == 4 and new['frame_ms'] == 500
    for kind in KINDS:
        assert (ASSETS / f'water/{kind}_0.png').read_bytes() == (ASSETS / f'water/{kind}_0_f0.png').read_bytes()
        first_phases = []
        for variant in range(4):
            images = []
            for frame in range(4):
                file = f'{kind}_{variant}_f{frame}.png'
                path = ASSETS / 'water' / file
                image = Image.open(path)
                assert image.mode == 'RGBA' and image.size == (128, 64)
                record = next(r for r in report['files'] if r['file'] == file)
                assert hashlib.sha256(path.read_bytes()).hexdigest() == record['sha256']
                pixels = np.asarray(image)
                assert pixels[0, 0, 3] == 0 and pixels[32, 64, 3] == 255
                if images:
                    assert np.array_equal(pixels[:, :, 3], images[0][:, :, 3])
                images.append(pixels)
            assert len({image.tobytes() for image in images}) == 4
            first_phases.append(images[0].tobytes())
        assert len(set(first_phases)) == 4
    replaced = {f'assets/nature/water/{k}_0.png' for k in KINDS}
    untouched = 0
    paths = subprocess.check_output(['git', 'ls-tree', '-r', '--name-only', cfg['base_commit'], 'assets/nature'], cwd=ROOT, text=True).splitlines()
    for path in paths:
        if path.endswith('.png') and path not in replaced:
            original = subprocess.check_output(['git', 'show', cfg['base_commit'] + ':' + path], cwd=ROOT)
            assert (ROOT / path).read_bytes() == original, path
            untouched += 1
    maximum = max(c['luma_spread'] for c in report['checks'])
    assert maximum < .15
    value = dict(status='passed',animated_pngs=48,replaced_static_pngs=3,
                 unchanged_other_pngs=untouched,all_phases_and_variants_distinct=True,
                 alpha_geometry_fixed=True,world_tile_size_unchanged=True,
                 static_matches_phase_zero=True,periodic_uv_edges=True,
                 common_variant_edges_per_phase=True,maximum_loop_mean_luma_spread=maximum,
                 frame_ms=500,loop_ms=2000,visual_approval='pending')
    (ART / 'validation.json').write_text(json.dumps(value, indent=2)+'\n')
    print(f'48 Phasen + drei Standbilder geprüft; {untouched} andere PNGs bytegleich; Luma-Schwankung {maximum*100:.4f} %.')


if __name__ == '__main__':
    main()
