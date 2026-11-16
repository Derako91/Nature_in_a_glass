"""Vollständiger Stil-B-Tiersatz mit eigenen Fernbildern, Spuren und Kadavern."""
import hashlib
import json
from pathlib import Path

import numpy as np
from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parents[2]
ART = ROOT / 'art/nature/animals-style-b'
ANIMALS = ROOT / 'assets/nature/animals'
SIZES = {'deer': (14, 12), 'deer_young': (9, 8), 'wolf': (13, 8),
         'fox': (10, 6), 'rabbit': (6, 5)}
STATES = [('graze', 2, 600), ('walk', 4, 200), ('run', 4, 100), ('rest', 1, None)]
CELLS = [('graze', 'r', 0), ('graze', 'r', 1), ('graze', 'l', 0), ('graze', 'l', 1)]
CELLS += [(s, d, f) for s in ('walk', 'run') for d in ('r', 'l') for f in range(4)]
CELLS += [('rest', 'r', 0), ('rest', 'l', 0)]


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def box(image):
    return image.getchannel('A').point(lambda a: 255 if a > 128 else 0).getbbox()


def feet(cut, state=''):
    a = np.asarray(cut)[:, :, 3]
    ys, xs = np.where(a > 160)
    y = int(ys.max()) + 1
    # Mitte der Standfläche statt Schwerpunkt des Fells; Schwanz kann ein
    # Fressbild verbreitern, darf den Fußpunkt aber nicht nach hinten ziehen.
    x = float(np.median(xs[ys >= y - 4])) + .5
    if state == 'rest':
        x = cut.width / 2
    return x, y


def place(cut, size, anchor, foot, factor):
    image = Image.new('RGBA', size)
    scaled = cut.resize((max(1, round(cut.width * factor)), max(1, round(cut.height * factor))), Image.Resampling.LANCZOS)
    position = (round(anchor[0] - foot[0] * factor), round(anchor[1] - foot[1] * factor))
    if position[0] < 0 or position[1] < 0 or position[0] + scaled.width > size[0] or position[1] + scaled.height > size[1]:
        raise ValueError(f'Motiv passt nicht proportional: {size} / {position} / {scaled.size}')
    image.alpha_composite(scaled, position)
    return image


def export():
    from export_people_style_b import split
    cfg = json.loads((ART / 'config.json').read_text())
    for source in cfg['sources']:
        if sha(ART / source['file']) != source['sha256']:
            raise ValueError(f'Geänderte Quelle: {source["file"]}')
    sources = {s['id']: ART / s['file'] for s in cfg['sources']}
    manifest = json.loads((ART / 'base-manifest.json').read_text())
    records, scales, images = [], {}, {}
    for species, (world_w, world_h) in SIZES.items():
        size = (world_w * (24 if species == "rabbit" else 16), world_h * 16)
        anchor = (size[0] // 2, (world_h * 2 - 1) * 8)
        for variant in range(2):
            near = split(sources[f'{species}-{variant}-near'], 4, 6)
            far = split(sources[f'{species}-{variant}-far'], 4, 6)
            for direction, refs in [('r', (0, 22)), ('l', (2, 23))]:
                indices = [i for i, (_, d, _) in enumerate(CELLS) if d == direction]
                near_feet = {i: feet(near[i], CELLS[i][0]) for i in indices}
                reference_width = box(near[refs[0]])[2] - box(near[refs[0]])[0]
                factor = cfg["target_visible_widths"][f"{species}_{variant}_{direction}"] * 8 / reference_width
                for i in indices:
                    fx, fy = near_feet[i]
                    factor = min(factor, (anchor[0] - 3) / fx,
                                 (size[0] - anchor[0] - 3) / (near[i].width - fx),
                                 (anchor[1] - 3) / fy,
                                 (size[1] - anchor[1] - 2) / max(1, near[i].height - fy))
                far_reference = box(far[refs[0]])[2] - box(far[refs[0]])[0]
                far_factor = factor * reference_width / far_reference / 4
                far_feet = {i: feet(far[i], CELLS[i][0]) for i in indices}
                far_size = (size[0] // 4, size[1] // 4)
                far_anchor = (anchor[0] / 4, anchor[1] / 4)
                for i in indices:
                    fx, fy = far_feet[i]
                    far_factor = min(far_factor, (far_anchor[0] - 1) / fx,
                                     (far_size[0] - far_anchor[0] - 1) / (far[i].width - fx),
                                     (far_anchor[1] - 1) / fy,
                                     (far_size[1] - far_anchor[1]) / max(1, far[i].height - fy))
                scales[f'{species}_{variant}_{direction}'] = dict(near=factor, far=far_factor)
                for i in indices:
                    state, d, frame = CELLS[i]
                    stem = f'{species}_{variant}_{state}_{d}_f{frame}'
                    for label, cut, dims, anc, ft, scale in [
                        ('', near[i], size, anchor, near_feet[i], factor),
                        ('@fern', far[i], far_size, far_anchor, far_feet[i], far_factor),
                    ]:
                        image = place(cut, dims, anc, ft, scale)
                        file = stem + label + '.png'
                        image.save(ANIMALS / file)
                        images[stem + label] = image
                        records.append(dict(file=file, size=dims, anchor=anc,
                                            bbox=box(image), sha256=sha(ANIMALS / file)))
        for entry in [e for e in manifest['assets'] if e['id'] == species]:
            entry.update(size=size, anchor=anchor, display_size=[world_w * 2, world_h * 2],
                         resolution_scale=8, status='style_b_draft')
            entry['levels'] = [dict(label='@fern', anchor=[v / 4 for v in anchor],
                                    files=[f.replace('.png', '@fern.png') for f in entry['files']])]
    export_remains(manifest, sources, records, images, split)
    export_tracks(manifest, sources, records, images)
    trim_sprites(manifest, records, images)
    manifest['status'] = 'style_b_draft'
    (ANIMALS / 'manifest.json').write_text(json.dumps(manifest, ensure_ascii=False, indent=2) + '\n')
    (ART / 'export.json').write_text(json.dumps(dict(files=records, scales=scales, mirrored=False,
            visual_approval='pending'), indent=2) + '\n')
    boards(images)
    print(f'{len(records)} Tier-/Fern-/Kadaver-/Spurenbilder exportiert.')


def trim_sprites(manifest, records, images):
    # Ein gemeinsamer Ausschnitt pro Art, über beide Fellvarianten, Richtungen,
    # Zustände und LODs. Entfernt ausschließlich Alpha-0-Ränder; gleiche
    # Weltposition jedes sichtbaren Pixels bleibt durch den Ankerabzug erhalten.
    for species in SIZES:
        entries = [e for e in manifest['assets'] if e['id'] == species]
        files = [f for e in entries for f in e['files']]
        bounds = []
        for file in files:
            for suffix, multiplier in [('', 1), ('@fern', 4)]:
                name = file[:-4] + suffix
                bounds.append(tuple(v * multiplier for v in images[name].getchannel('A').getbbox()))
        width, height = entries[0]['size']
        crop = (max(0, min(b[0] for b in bounds) // 4 * 4 - 4),
                max(0, min(b[1] for b in bounds) // 4 * 4 - 4),
                min(width, (max(b[2] for b in bounds) + 3) // 4 * 4 + 4),
                min(height, (max(b[3] for b in bounds) + 3) // 4 * 4 + 4))
        old_anchor = entries[0]['anchor']
        anchor = (old_anchor[0] - crop[0], old_anchor[1] - crop[1])
        dims = (crop[2] - crop[0], crop[3] - crop[1])
        for file in files:
            for suffix, divisor in [('', 1), ('@fern', 4)]:
                name = file[:-4] + suffix
                original = images[name]
                cropped = original.crop(tuple(v // divisor for v in crop))
                # Vollständige Pixelgleichheit nach Rückeinsetzen nachweisen.
                restored = Image.new('RGBA', original.size)
                restored.paste(cropped, (crop[0] // divisor, crop[1] // divisor))
                assert restored.tobytes() == original.tobytes(), name
                cropped.save(ANIMALS / (name + '.png'))
                images[name] = cropped
                record = next(r for r in records if r['file'] == name + '.png')
                record.update(size=cropped.size, anchor=[v / divisor for v in anchor],
                              bbox=box(cropped), sha256=sha(ANIMALS / (name + '.png')),
                              transparent_crop=[v // divisor for v in crop])
        for entry in entries:
            entry.update(size=dims, anchor=anchor, display_size=[v / 8 for v in dims])
            entry['levels'][0]['anchor'] = [v / 4 for v in anchor]


def export_remains(manifest, sources, records, images, split):
    near = split(sources['remains-near'], 4, 2)
    far = split(sources['remains-far'], 4, 2)
    for kind, start, world in [('carcass_large', 0, (14, 6)), ('carcass_small', 4, (7, 3))]:
        entry = next(e for e in manifest['assets'] if e['id'] == kind)
        # Kadaver werden um ihre Körpermitte verankert, wie bisher.
        size = (world[0] * 8, world[1] * 8)
        anchor = (size[0] / 2, size[1] / 2)
        for index, (variant, direction) in enumerate([(0, 'l'), (0, 'r'), (1, 'l'), (1, 'r')]):
            for label, cut, divisor in [('', near[start + index], 1), ('@fern', far[start + index], 4)]:
                dims = (size[0] // divisor, size[1] // divisor)
                image = Image.new('RGBA', dims)
                factor = min((dims[0] - 2) / cut.width, (dims[1] - 2) / cut.height)
                shown = cut.resize((max(1, round(cut.width * factor)), max(1, round(cut.height * factor))), Image.Resampling.LANCZOS)
                image.alpha_composite(shown, ((dims[0] - shown.width) // 2, (dims[1] - shown.height) // 2))
                stem = f'{kind}_{variant}_rest_{direction}_f0' + label
                image.save(ANIMALS / (stem + '.png'))
                images[stem] = image
                records.append(dict(file=stem + '.png', size=dims, anchor=[v / divisor for v in anchor],
                                    bbox=box(image), sha256=sha(ANIMALS / (stem + '.png'))))
        entry.update(size=size, anchor=anchor, display_size=world, resolution_scale=8)
        entry['levels'] = [dict(label='@fern', anchor=[v / 4 for v in anchor],
                                files=[f.replace('.png', '@fern.png') for f in entry['files']])]


def export_tracks(manifest, sources, records, images):
    source = Image.open(sources['tracks']).convert('RGBA')
    for col, species in enumerate(('deer', 'wolf', 'fox', 'rabbit')):
        cut = source.crop((round(col * source.width / 4), 0, round((col + 1) * source.width / 4), source.height))
        if (np.asarray(cut)[:, :, 3] > 64).mean() > .35:
            raise ValueError('Spurenquelle enthält zu viel deckenden Hintergrund')
        bounds = cut.getchannel('A').point(lambda a: 255 if a > 64 else 0).getbbox()
        if bounds is None:
            raise ValueError('Leere Spurenvorlage')
        cut = cut.crop(bounds).resize((128, 128), Image.Resampling.LANCZOS)
        # UV-Feld auf dieselbe feste Raute wie alle Dichteüberlagerungen.
        data = np.asarray(cut)
        yy, xx = np.mgrid[0:128, 0:256]
        u = (xx + .5 - 128) / 256 + (yy + .5) / 128
        v = (yy + .5) / 128 - (xx + .5 - 128) / 256
        mask = (u >= 0) & (u <= 1) & (v >= 0) & (v <= 1)
        entry = next(e for e in manifest['assets'] if e['id'] == 'tracks_' + species)
        files = []
        for variant in range(4):
            patch = np.roll(data, (variant * 11, variant * 17), axis=(0, 1))
            output = patch[np.clip((v * 127).astype(int), 0, 127), np.clip((u * 127).astype(int), 0, 127)].copy()
            output[:, :, 3] = np.where(mask, output[:, :, 3], 0)
            image = Image.fromarray(output)
            file = f'tracks_{species}_{variant}.png'
            image.save(ANIMALS / file)
            images[file[:-4]] = image
            files.append(file)
            records.append(dict(file=file, size=image.size, anchor=[128, 64], bbox=box(image), sha256=sha(ANIMALS / file)))
        entry.update(size=[256, 128], anchor=[128, 64], display_size=[64, 32], variants=4, files=files)


def boards(images):
    font = ImageFont.truetype('DejaVuSans.ttf', 17)
    for species in SIZES:
        sheet = Image.new('RGB', (1440, 80 + 8 * 220), '#344032')
        draw = ImageDraw.Draw(sheet)
        draw.text((10, 10), species + ' / Nah und Fern / synthetischer Kontaktbogen', font=font, fill='#eadfc8')
        row = 0
        for variant in range(2):
            for state, count, duration in STATES:
                y = 70 + row * 220
                draw.text((10, y), f'Variante {variant} / {state}', font=font, fill='#eadfc8')
                for direction_i, direction in enumerate(('l', 'r')):
                    for frame in range(count):
                        stem = f'{species}_{variant}_{state}_{direction}_f{frame}'
                        for level_i, label in enumerate(('', '@fern')):
                            image = images[stem + label]
                            image = image.crop(image.getchannel('A').point(lambda a: 255 if a > 24 else 0).getbbox())
                            factor = min(145 / image.width, 80 / image.height)
                            shown = image.resize((max(1, round(image.width * factor)), max(1, round(image.height * factor))), Image.Resampling.NEAREST)
                            x = 170 + direction_i * 625 + frame * 150
                            sheet.paste(shown, (x, y + 30 + level_i * 85), shown)
                row += 1
        sheet.save(ART / (species + '-contact.png'))
        for state, count, duration in STATES:
            if duration is None:
                continue
            frames = []
            for frame in range(count):
                panel = Image.new('RGB', (960, 512), '#344032')
                for variant in range(2):
                    for d_index, direction in enumerate(('l', 'r')):
                        for l_index, label in enumerate(('', '@fern')):
                            im = images[f'{species}_{variant}_{state}_{direction}_f{frame}' + label]
                            shown = im.resize((im.width * (1 if not label else 4), im.height * (1 if not label else 4)), Image.Resampling.NEAREST)
                            panel.paste(shown, (variant * 480 + d_index * 230, l_index * 256), shown)
                frames.append(panel)
            frames[0].save(ART / f'{species}-{state}.gif', save_all=True, append_images=frames[1:], duration=duration, loop=0)


if __name__ == '__main__':
    export()
