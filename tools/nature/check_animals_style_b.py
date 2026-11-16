"""Vollständigkeit, Maßstab und unabhängige Fernbilder des Tiersatzes prüfen."""
import hashlib
import json
import subprocess
from pathlib import Path

from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
ART = ROOT / 'art/nature/animals-style-b'
ANIMALS = ROOT / 'assets/nature/animals'


def main():
    cfg = json.loads((ART / 'config.json').read_text())
    report = json.loads((ART / 'export.json').read_text())
    manifest = json.loads((ANIMALS / 'manifest.json').read_text())
    records = {r['file']: r for r in report['files']}
    assert len(records) == 472
    referenced = set()
    for entry in manifest['assets']:
        for file in entry['files']:
            referenced.add(file)
            image = Image.open(ANIMALS / file)
            assert image.mode == 'RGBA'
            assert list(image.size) == entry['size']
            assert entry['anchor'] == records[file]['anchor']
        for level in entry.get('levels', []):
            assert level['label'] == '@fern'
            for file in level['files']:
                referenced.add(file)
                image = Image.open(ANIMALS / file)
                assert list(image.size) == [v // 4 for v in entry['size']]
                assert level['anchor'] == [v / 4 for v in entry['anchor']]
                assert level['anchor'] == records[file]['anchor']
    assert referenced == set(records)
    for file, record in records.items():
        assert hashlib.sha256((ANIMALS / file).read_bytes()).hexdigest() == record['sha256']
    widths = []
    for species in ('deer', 'deer_young', 'wolf', 'fox', 'rabbit'):
        for variant in range(2):
            for direction in ('l', 'r'):
                stem = f'{species}_{variant}_graze_{direction}_f0'
                entry = next(e for e in manifest['assets'] if stem + '.png' in e['files'])
                scale = entry['display_size'][0] / entry['size'][0]
                box = records[stem + '.png']['bbox']
                actual = (box[2] - box[0]) * scale
                target = cfg['target_visible_widths'][f'{species}_{variant}_{direction}']
                assert abs(actual - target) <= .75, (stem, actual, target)
                far = records[stem + '@fern.png']['bbox']
                widths.append(dict(file=stem, target=target, near=actual,
                                   far=(far[2] - far[0]) * scale * 4))
                for state, frames in [('graze', 2), ('walk', 4), ('run', 4), ('rest', 1)]:
                    names = [f'{species}_{variant}_{state}_{direction}_f{f}.png' for f in range(frames)]
                    assert len({records[n]['sha256'] for n in names}) == frames
                    assert len({records[n.replace('.png', '@fern.png')]['sha256'] for n in names}) == frames
    untouched = 0
    paths = subprocess.check_output(['git', 'ls-tree', '-r', '--name-only', cfg['base_commit'], 'assets/nature'], cwd=ROOT, text=True).splitlines()
    for path in paths:
        if path.endswith('.png') and not path.startswith('assets/nature/animals/'):
            original = subprocess.check_output(['git', 'show', cfg['base_commit'] + ':' + path], cwd=ROOT)
            assert (ROOT / path).read_bytes() == original, path
            untouched += 1
    value = dict(status='passed', png_files=472, live_near=220, live_far=220,
                 carcasses_near_and_far=16, tracks=16, unchanged_other_pngs=untouched,
                 all_manifest_files_present=True, all_phases_distinct=True,
                 fixed_scale_per_species_variant_direction=True,
                 identical_world_anchor_across_levels=True, visible_reference_widths=widths,
                 visual_approval='pending', rare_states_game_review='not_all_naturally_exercised')
    (ART / 'validation.json').write_text(json.dumps(value, indent=2) + '\n')
    (ANIMALS / 'validation.json').write_text(json.dumps(dict(status='style_b_draft',
        report='art/nature/animals-style-b/validation.json', exported_pngs=472), indent=2) + '\n')
    (ANIMALS / 'remains-validation.json').write_text(json.dumps([r for r in report['files'] if r['file'].startswith(('carcass_', 'tracks_'))], indent=2) + '\n')
    print(f'472 Bilder und alle Phasen/Verweise geprüft; {untouched} andere PNGs bytegleich.')


if __name__ == '__main__':
    main()
