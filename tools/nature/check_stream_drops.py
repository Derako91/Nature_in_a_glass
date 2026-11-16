"""90 Wandwasserteile, Kachel-/Kappenanschlüsse und Bestandsbilder prüfen."""
import hashlib
import json
import subprocess

import numpy as np
from PIL import Image
from export_stream_drops import ART, WATER, ROOT, sha


def check():
    report = json.loads((ART/'export.json').read_text())
    assert len(report['files']) == 90
    expected = set()
    luma_checks = []
    joins = 0
    for state in ('wet','dry'):
        for strength in range(3):
            for side in ('left','right'):
                for frame in range(4 if state=='wet' else 1):
                    parts = {}
                    for part in ('head','body','foot'):
                        name = f'stream_drop_{state}_{strength}_{side}_{part}_f{frame}.png'
                        expected.add(name)
                        im = Image.open(WATER/name)
                        h = 64 if part=='body' else 16
                        assert im.mode == 'RGBA' and im.size == (32,h), name
                        row = next(r for r in report['files'] if r['file']==name)
                        assert sha(WATER/name) == row['sha256']
                        data = np.asarray(im)
                        parts[part] = data
                        assert np.all(data[:,:,3] > 0) and np.all(data[:,:,3] <= 255)
                        if state=='dry':
                            wet = np.asarray(Image.open(WATER/name.replace('_dry_','_wet_')))
                            assert np.array_equal(data[:,:,3],wet[:,:,3])
                    assert np.array_equal(parts['head'][-1],parts['body'][0])
                    assert np.array_equal(parts['body'][-1],parts['body'][0])
                    assert np.array_equal(parts['foot'][0],parts['body'][-1])
                    assert np.array_equal(parts['head'][0],parts['foot'][-1])
                    joins += 4
                if state=='wet':
                    for part in ('head','body','foot'):
                        phases = [np.asarray(Image.open(WATER/f'stream_drop_wet_{strength}_{side}_{part}_f{f}.png')) for f in range(4)]
                        assert len({p.tobytes() for p in phases}) == 4
                        assert all(np.array_equal(p[:,:,3],phases[0][:,:,3]) for p in phases)
                        assert all(np.array_equal(p[:,[0,1,-2,-1]],phases[0][:,[0,1,-2,-1]]) for p in phases)
                        assert all(np.array_equal(p[0],phases[0][0]) and np.array_equal(p[-1],phases[0][-1]) for p in phases)
                        mask = phases[0][:,:,3] > 128
                        lumas = [float((p[:,:,:3].astype(float) @ np.array([.2126,.7152,.0722]))[mask].mean()) for p in phases]
                        spread = (max(lumas)-min(lumas))/np.mean(lumas)
                        assert spread < .15
                        luma_checks.append({'strength':strength,'side':side,'part':part,'mean_luma':lumas,'spread':float(spread)})
    assert {p.name for p in WATER.glob('stream_drop_*.png')} == expected
    manifest = json.loads((WATER/'manifest.json').read_text())
    entries = [a for a in manifest['assets'] if a['id'].startswith('stream_drop_')]
    assert len(entries) == 36 and {f for a in entries for f in a['files']} == expected
    for a in entries:
        h = 64 if a['id'].endswith('_body') else 16
        assert a['size'] == [32,h] and a['anchor'] == [16,0] and a['display_size'] == [8,h/4]
        assert a['frames'] == (4 if '_wet_' in a['id'] else 1)
    base = json.loads((ART/'config.json').read_text())['base_commit']
    tree = subprocess.check_output(['git','ls-tree','-r',base,'assets/nature'],cwd=ROOT,text=True)
    old_count = 0
    for row in tree.splitlines():
        meta,name = row.split('\t',1)
        if name.endswith('.png'):
            data = (ROOT/name).read_bytes()
            assert hashlib.sha1(f'blob {len(data)}\0'.encode()+data).hexdigest() == meta.split()[2], name
            old_count += 1
    old_manifest = json.loads(subprocess.check_output(['git','show',f'{base}:assets/nature/water/manifest.json'],cwd=ROOT))
    assert [a for a in manifest['assets'] if not a['id'].startswith('stream_drop_')] == old_manifest['assets']
    result = {'status':'passed','png_files':90,'wet_frames':72,'dry_frames':18,'matching_rgba_profiles':joins,
              'unique_animation_phases':True,'unchanged_parent_pngs':old_count,'luma_checks':luma_checks,
              'maximum_luma_spread':max(r['spread'] for r in luma_checks),'visual_approval':'pending'}
    (ART/'validation.json').write_text(json.dumps(result,ensure_ascii=False,indent=2)+'\n')
    print(f'90 Wandwasserbilder, {joins} exakte RGBA-Anschlüsse, {old_count} unveränderte Eltern-PNGs.')


if __name__ == '__main__':
    check()
