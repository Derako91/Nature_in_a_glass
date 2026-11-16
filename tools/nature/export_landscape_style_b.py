"""Stil-B-Landschaft: archivierte Material-/Pflanzenquellen proportional exportieren.

Weltmaß und Weltfußpunkte bleiben erhalten. Keine Zufallszahlen, keine Änderung
an Simulation oder Renderer. Aufruf aus dem Repository mit Pillow/NumPy/SciPy.
"""
import hashlib
import json
from pathlib import Path

import numpy as np
from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parents[2]
REVIEW = ROOT / 'art/nature/landscape-style-b'
ASSETS = ROOT / 'assets/nature'
GROUND = ['soil', 'sand', 'rock', 'grass_wet', 'grass_dry', 'burnt',
          'shrub_floor', 'forest_floor', 'silt']
WALLS = [('soil', 2), ('sand', 2), ('rock', 3)]
PLANTS = ['tree_young', 'tree_mid', 'tree_old', 'shrub', 'snag_standing', 'log_fallen']


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def source(name):
    return Image.open(REVIEW / 'sources' / (name + '.png')).convert('RGBA')


def material(name, col, row, columns, rows, target, variant, size):
    im = source(name).convert('RGB')
    box = (round(col * im.width / columns) + 4, round(row * im.height / rows) + 4,
           round((col + 1) * im.width / columns) - 4, round((row + 1) * im.height / rows) - 4)
    patch = np.asarray(im.crop(box).resize((128, 128), Image.Resampling.LANCZOS)).astype(float)
    # Gespiegelte Materialfortsetzung: wiederholbare Kanten ohne eine helle
    # oder dunkle Konturlinie. Varianten teilen dieselbe Randstruktur.
    patch = np.concatenate((patch, patch[:, ::-1]), axis=1)
    patch = np.concatenate((patch, patch[::-1]), axis=0)
    mean = patch.mean(axis=(0, 1))
    patch = np.array(target) + (patch - mean) * .65
    shifted = np.roll(patch, (variant * 53, variant * 79), axis=(0, 1))
    y, x = np.mgrid[0:256, 0:256]
    weight = (.85 * np.sin(np.pi * x / 255)**2 * np.sin(np.pi * y / 255)**2)[:, :, None]
    patch = np.clip(patch * (1 - weight) + shifted * weight, 0, 255).astype('uint8')
    patch[-1] = patch[0]
    patch[:, -1] = patch[:, 0]
    if rows == 3:
        # UV-Quadrat auf die feste 2:1-Feldraute abbilden, außerhalb Alpha 0.
        w, h = size
        yy, xx = np.mgrid[0:h, 0:w]
        u = (xx + .5 - w / 2) / w + (yy + .5) / h
        v = (yy + .5) / h - (xx + .5 - w / 2) / w
        mask = (u >= 0) & (u <= 1) & (v >= 0) & (v <= 1)
        # Dieselbe periodische Abtastung für sämtliche Varianten.
        ix = np.clip(np.rint(u * 255), 0, 255).astype(int)
        iy = np.clip(np.rint(v * 255), 0, 255).astype(int)
        data = np.zeros((h, w, 4), dtype='uint8')
        data[:, :, :3] = patch[iy, ix]
        data[:, :, 3] = mask * 255
        return Image.fromarray(data), patch
    image = Image.fromarray(patch).resize(size, Image.Resampling.LANCZOS).convert('RGBA')
    pixels = np.array(image)
    pixels[-1] = pixels[0]
    pixels[:, -1] = pixels[:, 0]
    return Image.fromarray(pixels), patch


def plant_cuts(name):
    im = source(name)
    cuts = []
    connected = None
    if name in ("snag_standing", "tree_mid", "tree_old"):
        # Diese Quelle hält das mathematische Raster nicht ein. Ganze
        # zusammenhängende Holzsilhouetten statt Wurzeln/Spitzen abschneiden.
        from export_people_style_b import split
        connected = split(REVIEW / "sources" / (name + ".png"), 4, 2)
    for row in range(2):
        for col in range(4):
            cell = im.crop((round(col * im.width / 4), round(row * im.height / 2),
                            round((col + 1) * im.width / 4), round((row + 1) * im.height / 2)))
            if connected is not None:
                cell = connected[row * 4 + col]
            alpha = np.asarray(cell)[:, :, 3]
            if max(alpha[0].max(), alpha[-1].max(), alpha[:, 0].max(), alpha[:, -1].max()) > 64:
                raise ValueError(f'Pflanze am Quellzellenrand: {name}, {row}/{col}')
            box = cell.getchannel('A').point(lambda a: 255 if a > 64 else 0).getbbox()
            if box is None:
                raise ValueError(f'Leere Pflanzenzelle: {name}')
            cut = cell.crop(box)
            opaque = np.asarray(cut)[:, :, 3] > 160
            ys, xs = np.where(opaque)
            bottom = int(ys.max()) + 1
            foot = float(np.median(xs[ys >= bottom - 5])) + .5
            if name == 'log_fallen':
                foot = cut.width / 2
            cuts.append((cut, (foot, bottom)))
    return cuts


def export():
    cfg = json.loads((REVIEW / 'config.json').read_text())
    for s in cfg['sources']:
        if sha(REVIEW / s['file']) != s['sha256']:
            raise ValueError(f'Quelle verändert: {s["file"]}')
    manifest = json.loads((REVIEW / 'base-manifest.json').read_text())
    entries = {e['id']: e for e in manifest['assets']}
    records = []
    periodic = {}

    def save(image, file, entry, **details):
        image.save(ASSETS / file)
        records.append(dict(file=file, size=list(image.size), sha256=sha(ASSETS / file),
                            anchor=entry['anchor'], display_size=entry['display_size'], **details))

    for i, key in enumerate(GROUND):
        name = 'ground_' + key
        entry = entries.get(name)
        if entry is None:
            entry = dict(id=name, group='ground', states=['normal'], frames=1)
            manifest['assets'].append(entry)
        entry.update(size=[256, 128], anchor=[128, 64], display_size=[64, 32], variants=4,
                     resolution_scale=4, files=[f'ground/{name}_{v}.png' for v in range(4)])
        for variant, file in enumerate(entry['files']):
            image, uv = material('materials', i % 3, i // 3, 3, 3, cfg['means'][name], variant, (256, 128))
            periodic[file] = uv
            save(image, file, entry)
    for i, (key, count) in enumerate(WALLS):
        name = 'wall_' + key
        entry = entries[name]
        entry.update(size=[128, 128], anchor=[64, 64], resolution_scale=4, variants=count,
                     files=[f'walls/{name}_{v}.png' for v in range(count)])
        for variant, file in enumerate(entry['files']):
            image, uv = material('walls', i, 0, 3, 1, cfg['means'][name], variant, (128, 128))
            periodic[file] = uv
            save(image, file, entry)
    for name in PLANTS:
        entry = entries.get(name)
        if entry is None:
            entry = dict(id=name, group='vegetation', display_size=[16, 5], frames=1)
            manifest['assets'].append(entry)
            size, anchor = (128, 40), (64, 32)
        else:
            size = tuple(x * 4 for x in entry['size'])
            anchor = tuple(x * 4 for x in entry['anchor'])
        states = ['normal', 'charred' if name in ('snag_standing', 'log_fallen') else 'stressed']
        entry.update(size=list(size), anchor=list(anchor), resolution_scale=8, variants=4,
                     states=states, files=[])
        cuts = plant_cuts(name)
        for variant in range(4):
            factors = []
            for n in (variant, variant + 4):
                cut, (fx, fy) = cuts[n]
                factors.append(min((anchor[0] - 3) / fx, (size[0] - anchor[0] - 3) / (cut.width - fx),
                                   (anchor[1] - 3) / fy, (size[1] - anchor[1] - 2) / max(1, cut.height - fy)))
            factor = min(factors)
            for row, state in enumerate(states):
                cut, (fx, fy) = cuts[variant + row * 4]
                image = Image.new('RGBA', size)
                scaled = cut.resize((max(1, round(cut.width * factor)), max(1, round(cut.height * factor))), Image.Resampling.LANCZOS)
                image.alpha_composite(scaled, (round(anchor[0] - fx * factor), round(anchor[1] - fy * factor)))
                file = f'vegetation/{name}_{variant}_{state}.png'
                entry['files'].append(file)
                save(image, file, entry, proportional_factor=factor, source_foot=[fx, fy])
    manifest['status'] = 'landscape_style_b_draft'
    (ASSETS / 'manifest.json').write_text(json.dumps(manifest, ensure_ascii=False, indent=2) + '\n')
    # Techniknachweis vor Isometrie: periodische UV-Ränder und gemeinsame
    # Materialränder zwischen Varianten prüfen, nicht nur den PNG-Dateityp.
    for file, uv in periodic.items():
        assert np.array_equal(uv[0], uv[-1]) and np.array_equal(uv[:, 0], uv[:, -1]), file
    for prefix, count in [(f'ground/ground_{k}', 4) for k in GROUND] + [(f'walls/wall_{k}', n) for k, n in WALLS]:
        first = periodic[prefix + '_0.png']
        for variant in range(1, count):
            uv = periodic[f'{prefix}_{variant}.png']
            assert np.array_equal(first[0], uv[0]) and np.array_equal(first[:, 0], uv[:, 0]), prefix
    (REVIEW / 'export.json').write_text(json.dumps(dict(files=records, periodic_edges=True,
            variant_edges_identical=True, visual_approval='pending'), indent=2) + '\n')
    contact_sheet(records)
    print(f'{len(records)} Landschaftsbilder exportiert, Weltmaße erhalten, Materialkanten geprüft.')


def contact_sheet(records):
    font = ImageFont.truetype('DejaVuSans.ttf', 17)
    for group in ('ground', 'walls', 'vegetation'):
        rows = [r for r in records if r['file'].startswith(group + '/')]
        columns = 4
        sheet = Image.new('RGB', (1200, 50 + ((len(rows) + 3) // 4) * 290), '#384333')
        draw = ImageDraw.Draw(sheet)
        draw.text((12, 10), 'Stil B / ' + group + ' / Kontaktbogen, keine Spielaufnahme', font=font, fill='#eadfc8')
        for i, record in enumerate(rows):
            x, y = (i % columns) * 300, 50 + (i // columns) * 290
            name = Path(record['file']).stem
            draw.text((x + 8, y + 5), name, font=font, fill='#eadfc8')
            image = Image.open(ASSETS / record['file']).convert('RGBA')
            factor = min(270 / image.width, 245 / image.height)
            image = image.resize((round(image.width * factor), round(image.height * factor)), Image.Resampling.NEAREST)
            sheet.paste(image, (x + (300 - image.width) // 2, y + 35 + (245 - image.height) // 2), image)
        sheet.save(REVIEW / (group + '-contact.png'))


if __name__ == '__main__':
    export()
