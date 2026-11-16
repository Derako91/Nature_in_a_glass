"""Vollständigkeit, gemeinsame Beetmaske und unveränderten Bestand prüfen."""
import hashlib
import json
import subprocess
from pathlib import Path

import numpy as np
from PIL import Image
from export_gardens import ART, GROUND, ROOT, STAGES, sha


def check():
    report = json.loads((ART / 'export.json').read_text())
    manifest = json.loads((GROUND / 'manifest.json').read_text())
    assert len(report['files']) == 20
    expected = {f'garden_{s}_{v}.png' for s in STAGES for v in range(4)}
    assert {p.name for p in GROUND.glob('garden_*.png')} == expected
    reference = None
    unique, colors = set(), {}
    for stage in STAGES:
        entry = next(a for a in manifest['assets'] if a['id'] == f'garden_{stage}')
        assert entry['size'] == [128, 64] and entry['anchor'] == [64, 32]
        assert entry['display_size'] == [64, 32] and entry['variants'] == 4
        assert set(entry['files']) == {f'garden_{stage}_{v}.png' for v in range(4)}
        greens, means = [], []
        for name in entry['files']:
            image = Image.open(GROUND / name)
            assert image.mode == 'RGBA' and image.size == (128, 64)
            data = np.asarray(image)
            alpha = data[:, :, 3]
            if reference is None:
                reference = alpha
            assert np.array_equal(alpha, reference), name
            assert alpha[32,64] == 255 and alpha[0,0] == alpha[0,-1] == alpha[-1,0] == alpha[-1,-1] == 0
            y, x = np.mgrid[:64,:128]
            outside = np.abs((x+.5-64)/64) + np.abs((y+.5-32)/32) > 1
            assert not np.any(alpha[outside]), name
            assert np.all(data[alpha == 0] == 0), name
            record = next(r for r in report['files'] if r['file'] == name)
            digest = sha(GROUND / name)
            assert digest == record['sha256'] and digest not in unique
            unique.add(digest)
            rgb = data[alpha > 128,:3].astype(float)
            greens.append(float(np.mean((rgb[:,1] > rgb[:,0]*1.03) & (rgb[:,1] > rgb[:,2]*1.25))))
            means.append(rgb.mean(axis=0).round(2).tolist())
        colors[stage] = {'green_fractions': greens, 'mean_rgb': means}
    cfg = json.loads((ART / 'config.json').read_text())
    tree = subprocess.check_output(['git','ls-tree','-r',cfg['base_commit'],'assets/nature'],cwd=ROOT,text=True)
    count = 0
    for row in tree.splitlines():
        meta, name = row.split('\t',1)
        if not name.endswith('.png'):
            continue
        data = (ROOT / name).read_bytes()
        assert hashlib.sha1(f'blob {len(data)}\0'.encode()+data).hexdigest() == meta.split()[2], name
        count += 1
    result = {'status': 'passed', 'images': 20, 'stages': STAGES, 'variants': 4,
              'common_alpha_mask': True, 'outside_diamond_transparent': True,
              'unchanged_parent_pngs': count, 'stage_color_measurements': colors,
              'visual_approval': 'pending'}
    (ART / 'validation.json').write_text(json.dumps(result,ensure_ascii=False,indent=2)+'\n')
    print(f'20 Gartenbilder, gemeinsame Raute/Beetmaske; alle {count} Eltern-PNGs unverändert.')


if __name__ == '__main__':
    check()
