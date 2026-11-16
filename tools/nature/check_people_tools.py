"""Matrix, Proportionen, Quellen und unveränderten Bildbestand gegen die Auftragsbasis prüfen."""
import hashlib
import json
import subprocess
from pathlib import Path
from PIL import Image, ImageOps
from export_people_tools import ART, PEOPLE, ROOT, STATES, ordinary_frames, opaque_box, sha


def check():
    registry = json.loads((ART / 'sources.json').read_text())
    for source in registry['sources']:
        assert sha(ART / source['file']) == source['sha256']
    manifest = json.loads((PEOPLE / 'manifest.json').read_text())
    expected = set()
    for state, (count, ms) in STATES.items():
        entry = next(e for e in manifest['assets'] if e.get('state') == state)
        parent = next(e for e in manifest['assets'] if e.get('state') == state.rsplit('_', 1)[0])
        for key in set(parent) - {'state', 'frames', 'frame_ms', 'files', 'status'}:
            assert entry[key] == parent[key], (state, key)
        assert entry['frames'] == count and entry['frame_ms'] == ms
        files = set()
        for look in range(5):
            parents = ordinary_frames(look, state.rsplit('_', 1)[0])
            max_width = max(opaque_box(im)[2] - opaque_box(im)[0] for im in parents)
            max_height = max(opaque_box(im)[3] - opaque_box(im)[1] for im in parents)
            for direction in ('l', 'r'):
                phases = set()
                for frame in range(count):
                    name = f'person_{look}_{state}_{direction}_f{frame}.png'
                    im = Image.open(PEOPLE / name).convert('RGBA')
                    assert im.size == (112, 120)
                    box = opaque_box(im)
                    assert box and box[2]-box[0] <= max_width+1 and box[3]-box[1] <= max_height+4
                    assert box[3] <= 114
                    phases.add(im.tobytes()); files.add(name)
                    opposite = Image.open(PEOPLE / name.replace(f'_{direction}_', f'_{"r" if direction == "l" else "l"}_')).convert('RGBA')
                    assert im.tobytes() != ImageOps.mirror(opposite).tobytes(), name
                assert len(phases) == count
        assert set(entry['files']) == files
        expected |= files
    assert len(expected) == 140
    old_manifest = json.loads(subprocess.check_output(['git', 'show', f"{registry['base_commit']}:assets/nature/people/manifest.json"], cwd=ROOT))
    without_new = dict(manifest, assets=[e for e in manifest['assets'] if e.get('state') not in STATES])
    assert without_new == old_manifest
    tree = subprocess.check_output(['git', 'ls-tree', '-r', registry['base_commit'], 'assets/nature'], cwd=ROOT, text=True)
    unchanged = 0
    for line in tree.splitlines():
        meta, file = line.split('\t')
        if not file.endswith('.png'): continue
        data = (ROOT / file).read_bytes()
        blob = hashlib.sha1(b'blob ' + str(len(data)).encode() + b'\0' + data).hexdigest()
        assert blob == meta.split()[2], file
        unchanged += 1
    result = {'new_frames': len(expected), 'parent_pngs_unchanged': unchanged,
              'base_commit': registry['base_commit'], 'matrix_and_metadata': 'passed',
              'directions_not_mirrored': True, 'width_tolerance_px': 1,
              'height_tolerance_px': 4, 'visual_approval': 'pending'}
    (ART / 'validation.json').write_text(json.dumps(result, indent=2) + '\n')
    print(json.dumps(result))


if __name__ == '__main__':
    check()
