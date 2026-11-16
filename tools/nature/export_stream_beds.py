"""Nasse/trockene Bachstücke mit festen Anschlussquerschnitten exportieren."""
import hashlib
import json
from pathlib import Path

import numpy as np
from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parents[2]
ART = ROOT / 'art/nature/stream-beds-style-b'
WATER = ROOT / 'assets/nature/water'
CONFIG = ART / 'config.json'
ENDPOINTS = {'n': (0, 0), 'ne': (.5, 0), 'e': (1, 0), 'se': (1, .5),
             's': (1, 1), 'sw': (.5, 1), 'w': (0, 1), 'nw': (0, .5)}
OPPOSITE = {'n': 's', 'ne': 'sw', 'e': 'w', 'se': 'nw'}


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def geometry(direction, u, v):
    points = np.stack((u, v), axis=-1)
    relative = points - .5
    if direction == 'c':
        return np.sqrt((relative**2).sum(axis=-1)), np.zeros_like(u), np.zeros_like(u)
    end = np.array(ENDPOINTS[direction], dtype=float)
    vector = end - .5
    length = float(np.linalg.norm(vector))
    unit = vector / length
    normal = np.array([-unit[1], unit[0]])
    # Innen kleine natürliche Biegung, letzte 20 % exakt gerade. Die
    # Verlängerung über die Kante verhindert runde, verengte Endkappen.
    t = np.linspace(0, 1, 81)
    bend = .018 * np.sin(np.pi * np.clip(t / .8, 0, 1))**2
    path = .5 + t[:, None] * vector + bend[:, None] * normal
    path = np.vstack((path, end + unit * .04))
    distances = np.full(u.shape, np.inf)
    for a, b in zip(path[:-1], path[1:]):
        delta = b - a
        projection = np.clip(((points - a) * delta).sum(axis=-1) / float(delta @ delta), 0, 1)
        closest = a + projection[:, :, None] * delta
        distances = np.minimum(distances, ((points - closest)**2).sum(axis=-1))
    signed = (relative * normal).sum(axis=-1)
    # Feste Lichtseite im Bildschirmraum; Vorzeichenwechsel bei der
    # Gegenrichtung kompensiert sich, statt die Beleuchtung zu spiegeln.
    screen_normal = np.array([(normal[0] - normal[1]) * 2, normal[0] + normal[1]])
    screen_normal /= np.linalg.norm(screen_normal)
    light = float(screen_normal @ np.array([.70710678, -.70710678]))
    along = (relative * unit).sum(axis=-1) / length
    return np.sqrt(distances), signed * light, along


def material(source, row, col):
    from export_water_style_b import periodic
    box = (round(col * source.width / 3) + 4, round(row * source.height / 2) + 4,
           round((col + 1) * source.width / 3) - 4, round((row + 1) * source.height / 2) - 4)
    pixels = np.asarray(source.crop(box).resize((128, 128), Image.Resampling.LANCZOS)).astype(float)
    pixels = periodic(pixels)
    pixels[-1] = pixels[0]
    pixels[:, -1] = pixels[:, 0]
    return pixels


def export():
    cfg = json.loads(CONFIG.read_text())
    path = ART / cfg['source']['file']
    if sha(path) != cfg['source']['sha256']:
        raise ValueError('Bachmaterialquelle geändert')
    source = Image.open(path).convert('RGB')
    water_path = ART / cfg['water_source']['file']
    if sha(water_path) != cfg['water_source']['sha256']:
        raise ValueError('Wasser-Materialquelle geändert')
    water_material = material(Image.open(water_path).convert('RGB'), 0, 2)
    yy, xx = np.mgrid[0:64, 0:128]
    u = (xx + .5 - 64) / 128 + (yy + .5) / 64
    v = (yy + .5) / 64 - (xx + .5 - 64) / 128
    iu = np.rint((u % 1) * 127).astype(int)
    iv = np.rint((v % 1) * 127).astype(int)
    assets, records, images = [], [], {}
    shapes = {d: geometry(d, u, v) for d in ['c'] + list(ENDPOINTS)}
    for state in ('wet', 'dry'):
        for strength in range(3):
            bed_radius = cfg['bed_radii_uv'][strength]
            water_radius = cfg['water_radii_uv'][strength]
            patch = material(source, 1 if state == 'wet' else 0, strength)
            texture = patch[iv, iu]
            painted_water = np.array(cfg['water_rgb']) + (water_material[iv, iu] - water_material.mean(axis=(0, 1))) * .45
            mean = patch.mean(axis=(0, 1))
            files = []
            for direction in ['c'] + list(ENDPOINTS):
                distance, shade, along = shapes[direction]
                soft = .006
                radial = np.sqrt((u - .5)**2 + (v - .5)**2)
                junction_radius = 1.5 * (bed_radius + soft)
                bed_distance = distance * bed_radius / junction_radius if direction == 'c' else distance
                edge_fade = np.clip((along - .86) / .10, 0, 1)
                natural_radius = bed_radius * (1 + .08 * np.sin(u * 71 + v * 43) * (1 - edge_fade))
                alpha = np.clip((natural_radius + soft - bed_distance) / (2 * soft), 0, 1)
                # Gleicher Querschnitt am Ende jedes Gegenstücks. Das
                # Ausblenden feiner Körnung auf den letzten Pixeln vermeidet
                # sichtbare Textursprünge an Kanten/Ecken und bei Mündungen.
                endpoint = np.clip((along - .86) / .10, 0, 1)[:, :, None]
                color = texture * (1 - endpoint) + mean * endpoint
                color *= (1 + .07 * np.clip(shade / bed_radius, -1, 1))[:, :, None]
                channel = np.clip((water_radius + .004 - distance) / .008, 0, 1)
                if state == 'wet':
                    water_color = painted_water * (1 - endpoint) + np.array(cfg['water_rgb']) * endpoint
                    color = color * (1 - channel[:, :, None]) + water_color * channel[:, :, None]
                else:
                    color *= (1 - .12 * channel)[:, :, None]
                if direction != 'c':
                    # Im gemeinsamen Knoten nur das Wasser/Innere übermalen.
                    # Sonst legt die Bank des späteren Halbstücks einen
                    # grauen Damm über einen zuvor gezeichneten Wasserarm.
                    banks = np.clip((radial - junction_radius) / (2 * soft), 0, 1)
                    alpha *= channel + (1 - channel) * banks
                    inner = water_color if state == 'wet' else texture * .88
                    color = color * banks[:, :, None] + inner * (1 - banks[:, :, None])
                data = np.zeros((64, 128, 4), dtype='uint8')
                data[:, :, :3] = np.clip(np.rint(color), 0, 255).astype('uint8')
                data[:, :, 3] = np.rint(alpha * 255).astype('uint8')
                data[data[:, :, 3] == 0] = 0
                image = Image.fromarray(data)
                file = f'stream_{state}_{strength}_{direction}.png'
                image.save(WATER / file)
                images[file[:-4]] = image
                files.append(file)
                records.append(dict(file=file, state=state, strength=strength, part=direction,
                                    size=[128, 64], anchor=[64, 32], sha256=sha(WATER / file)))
            assets.append(dict(id=f'stream_{state}_{strength}', group='water', size=[128, 64],
                               anchor=[64, 32], display_size=[64, 32], resolution_scale=2,
                               state=state, strength=strength, frames=1, files=files,
                               parts=['c'] + list(ENDPOINTS)))
    manifest_path = WATER / 'manifest.json'
    # Gemeinsames Wassermanifest: andere Familien bei Wiederholung erhalten.
    previous = json.loads(manifest_path.read_text()) if manifest_path.exists() else dict(version=1, assets=[])
    # Vorhandene Reihenfolge und andere Familien (z. B. stream_drop) erhalten.
    replacements = {e['id']: e for e in assets}
    previous['assets'] = [replacements.pop(e['id'], e) for e in previous['assets']]
    previous['assets'].extend(replacements.values())
    previous['status'] = 'stream_beds_style_b_draft'
    manifest_path.write_text(json.dumps(previous, ensure_ascii=False, indent=2) + '\n')
    (ART / 'export.json').write_text(json.dumps(dict(files=records, visual_approval='pending',
        bed_radii_uv=cfg['bed_radii_uv'], water_radii_uv=cfg['water_radii_uv'],
        endpoint_extension_uv=.04, straight_end_fraction=.20), indent=2) + '\n')
    proof(images)
    print('54 Bach-/Bachbettstücke mit gemeinsamen Anschlussbreiten exportiert.')


def proof(images):
    font = ImageFont.truetype('DejaVuSans.ttf', 17)
    sheet = Image.new('RGB', (1440, 60 + 6 * 210), '#344032')
    draw = ImageDraw.Draw(sheet)
    draw.text((10, 12), 'Bachstücke / vergrößerter Kontaktbogen, keine Spielaufnahme', font=font, fill='#eadfc8')
    for row, (state, strength) in enumerate([(s, n) for s in ('wet', 'dry') for n in range(3)]):
        for col, direction in enumerate(['c'] + list(ENDPOINTS)):
            x, y = col * 160, 60 + row * 210
            draw.text((x + 5, y + 3), f'{state} {strength} {direction}', font=font, fill='#eadfc8')
            image = images[f'stream_{state}_{strength}_{direction}'].resize((256, 128), Image.Resampling.NEAREST)
            # Spalten überlappen nicht: zur Prüfung jeden Quellausschnitt
            # genau einmal und ohne Begrenzung auf das Motiv vergrößern.
            image = image.resize((152, 76), Image.Resampling.NEAREST)
            sheet.paste(image, (x + 4, y + 55), image)
    sheet.save(ART / 'parts-contact.png')
    joins = Image.new('RGB', (1600, 60 + 6 * 300), '#344032')
    draw = ImageDraw.Draw(joins)
    draw.text((12, 12), 'Gegenstücke auf gleicher Höhe / synthetische Anschlussprobe', font=font, fill='#eadfc8')
    directions = ['n', 'ne', 'e', 'se']
    offsets = {'n': (0, -64), 'ne': (64, -32), 'e': (128, 0), 'se': (64, 32)}
    ground = Image.open(ROOT / 'assets/nature/ground/ground_soil_0.png').convert('RGBA').resize((128, 64), Image.Resampling.LANCZOS)
    for row, (state, strength) in enumerate([(s, n) for s in ('wet', 'dry') for n in range(3)]):
        for col, direction in enumerate(directions):
            panel = Image.new('RGBA', (320, 230), (52, 64, 50, 255))
            a = (50, 90)
            delta = offsets[direction]
            b = (a[0] + delta[0], a[1] + delta[1])
            for at in (a, b):panel.alpha_composite(ground, at)
            for at, part in [(a, 'c'), (b, 'c'), (a, direction), (b, OPPOSITE[direction])]:
                panel.alpha_composite(images[f'stream_{state}_{strength}_{part}'], at)
            x, y = col * 400, 60 + row * 300
            draw.text((x + 8, y + 5), f'{state} {strength} / {direction}-{OPPOSITE[direction]}', font=font, fill='#eadfc8')
            shown = panel.convert('RGB').resize((384, 276), Image.Resampling.NEAREST)
            joins.paste(shown, (x + 8, y + 22))
    joins.save(ART / 'joins-contact.png')
    junctions = Image.new('RGB', (1800, 60 + 6 * 210), '#344032')
    draw = ImageDraw.Draw(junctions)
    draw.text((12, 12), 'Mittelstück plus Halbstücke / echte Überlagerungsreihenfolge / keine Spielaufnahme', font=font, fill='#eadfc8')
    configurations = [('n', 's'), ('ne', 'se'), ('n', 'e'), ('n', 'se', 'sw'), ('ne', 'e', 's', 'nw'), tuple(ENDPOINTS)]
    ground = Image.open(ROOT / 'assets/nature/ground/ground_grass_wet_0.png').convert('RGBA')
    for row, (state, strength) in enumerate([(s, n) for s in ('wet', 'dry') for n in range(3)]):
        for col, connected in enumerate(configurations):
            x, y = col * 300, 60 + row * 210
            draw.text((x + 6, y + 5), f'{state} {strength} / ' + ','.join(connected), font=font, fill='#eadfc8')
            combined = images[f'stream_{state}_{strength}_c'].copy()
            for d in ENDPOINTS:
                if d in connected:
                    combined.alpha_composite(images[f'stream_{state}_{strength}_{d}'])
            shown = combined.resize((256, 128), Image.Resampling.NEAREST)
            junctions.paste(ground, (x + 22, y + 38), ground)
            junctions.paste(shown, (x + 22, y + 38), shown)
    junctions.save(ART / 'junctions-contact.png')


if __name__ == '__main__':
    export()
