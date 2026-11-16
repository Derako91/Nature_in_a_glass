"""Temporärer Bildaustausch für die Darstellung, KEINE echte Fischfang-Aktivität."""
import argparse
import json
import shutil
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--out', type=Path, default=Path('/tmp/fish-preview-overlay'))
    out = parser.parse_args().out / 'people'
    out.mkdir(parents=True, exist_ok=True)
    people = ROOT / 'assets/nature/people'
    entry = next(a for a in json.loads((people / 'manifest.json').read_text())['assets']
                 if a['id'] == 'person' and a.get('state') == 'fish')
    names = []
    for source in entry['files']:
        name = source.replace('_fish_', '_gather_')
        shutil.copyfile(people / source, out / name)
        names.append(name)
    entry.update(state='gather', files=names)
    (out / 'manifest.json').write_text(json.dumps({'assets': [entry]}, indent=2) + '\n')
    print(f'Darstellungsprobe: --bilder {out.parent}; ersetzt alle erwachsenen Sammelbilder, auch fern vom Wasser.')


if __name__ == '__main__':
    main()
