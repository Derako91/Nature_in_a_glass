"""Fischfangbilder einschließlich unveränderter bestehender Figuren prüfen."""
import hashlib
import json
import subprocess
from pathlib import Path
from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
REVIEW = ROOT / 'art/nature/fishing-people-style-b'
PEOPLE = ROOT / 'assets/nature/people'


def check():
    report = json.loads((REVIEW / 'export.json').read_text())
    assert len(report['files']) == 40
    for look in range(5):
        dirs = []
        for direction in ('r', 'l'):
            phases = []
            for frame in range(4):
                path = PEOPLE / f'person_{look}_fish_{direction}_f{frame}.png'
                im = Image.open(path)
                assert im.mode == 'RGBA' and im.size == (112, 120)
                bbox = im.getchannel('A').getbbox()
                assert bbox and bbox[0] > 0 and bbox[1] > 0 and bbox[2] < 112 and bbox[3] < 120
                phases.append(im.tobytes())
                row = next(r for r in report['files'] if r['file'] == path.name)
                assert hashlib.sha256(path.read_bytes()).hexdigest() == row['sha256']
                reference = Image.open(PEOPLE / f'person_{look}_idle_r_f0.png').getchannel('A').point(lambda a: 255 if a > 128 else 0).getbbox()
                standing = (reference[3] - reference[1]) * .1125
                assert .8 * standing <= row['body_world_height'] <= standing + .001, row
            assert len(set(phases)) == 4
            dirs.append(phases)
        for a, b in zip(*dirs):
            assert Image.frombytes('RGBA', (112, 120), a).transpose(Image.Transpose.FLIP_LEFT_RIGHT).tobytes() != b
    base = '0edb584'
    previous = subprocess.check_output(['git', 'ls-tree', '-r', '--name-only', base, 'assets/nature'], cwd=ROOT, text=True).splitlines()
    count = 0
    for name in previous:
        if not name.endswith('.png'):
            continue
        old = subprocess.check_output(['git', 'show', f'{base}:{name}'], cwd=ROOT)
        assert old == (ROOT / name).read_bytes(), name
        count += 1
    manifest = json.loads((PEOPLE / 'manifest.json').read_text())
    original_manifest = json.loads(subprocess.check_output(['git', 'show', f'{base}:assets/nature/people/manifest.json'], cwd=ROOT))
    without_fish = dict(manifest)
    without_fish['assets'] = [a for a in manifest['assets'] if not (a['id'] == 'person' and a.get('state') == 'fish')]
    assert without_fish == original_manifest, 'Bestehende Anker, Farben oder Zustände geändert'
    entry = next(a for a in manifest['assets'] if a['id'] == 'person' and a.get('state') == 'fish')
    assert len(entry['files']) == 40 and entry['anchor'] == [56, 112] and entry['frame_ms'] == 300
    result = {'status': 'passed', 'new_frames': 40, 'unchanged_parent_pngs': count,
              'unique_phases': True, 'separately_drawn_directions': True,
              'engine_state_selection': 'pending Claude', 'visual_approval': 'pending'}
    (REVIEW / 'validation.json').write_text(json.dumps(result, ensure_ascii=False, indent=2) + '\n')
    print(f'40 Fischfangbilder und {count} unveränderte Eltern-PNGs geprüft.')


if __name__ == '__main__':
    check()
