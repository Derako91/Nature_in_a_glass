"""Maßstabs-/Lesbarkeitsbögen auf echten Böden; keine Spielaufnahmen."""
import json
from pathlib import Path
from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parents[2]
ART = ROOT / 'art/nature/animals-style-b'
ASSETS = ROOT / 'assets/nature'
FONT = ImageFont.truetype('DejaVuSans.ttf', 18)


def main():
    manifest = json.loads((ASSETS / 'animals/manifest.json').read_text())
    lookup = {f: e for e in manifest['assets'] for f in e['files']}
    species = ['deer', 'deer_young', 'wolf', 'fox', 'rabbit']
    grounds = ['ground_soil', 'ground_grass_wet', 'ground_forest_floor']
    sheet = Image.new('RGB', (2560, 2220), '#344032')
    draw = ImageDraw.Draw(sheet)
    draw.text((12, 12), 'Stil B / echtes Weltmaß bei Zoom 8 und 32 / synthetischer Lesbarkeitstest', font=FONT, fill='#eadfc8')
    for ground_i, ground in enumerate(grounds):
        for zoom_i, zoom in enumerate((8, 32)):
            row = ground_i * 2 + zoom_i
            y = 60 + row * 360
            screen = zoom / 4
            tile = Image.open(ASSETS / f'ground/{ground}_0.png').convert('RGBA')
            tile = tile.resize((round(64 * screen), round(32 * screen)), Image.Resampling.NEAREST)
            for col, kind in enumerate(species):
                x = col * 512
                draw.text((x + 8, y + 4), f'{kind} / {ground} / Zoom {zoom}', font=FONT, fill='#eadfc8')
                sheet.paste(tile, (x + (512 - tile.width) // 2, y + 230 - tile.height // 2), tile)
                for variant, direction, sign in [(0, 'r', -1), (1, 'l', 1)]:
                    file = f'{kind}_{variant}_graze_{direction}_f0.png'
                    entry = lookup[file]
                    anchor = entry['anchor']
                    if zoom <= 8:
                        file = file.replace('.png', '@fern.png')
                        anchor = entry['levels'][0]['anchor']
                    image = Image.open(ASSETS / 'animals' / file).convert('RGBA')
                    factor = entry['display_size'][0] * screen / image.width
                    shown = image.resize((round(image.width * factor), round(image.height * factor)), Image.Resampling.NEAREST)
                    foot = (x + 256 + sign * 12 * screen, y + 230 + sign * screen)
                    sheet.paste(shown, (round(foot[0] - anchor[0] * factor), round(foot[1] - anchor[1] * factor)), shown)
    sheet.save(ART / 'readability.png')
    remains = Image.new('RGB', (1280, 1000), '#344032')
    draw = ImageDraw.Draw(remains)
    draw.text((12, 12), 'Kadaver / Nah und Fern im gleichen Weltmaß / keine Spielaufnahme', font=FONT, fill='#eadfc8')
    for row, (kind, variant) in enumerate([(k, v) for k in ['carcass_large', 'carcass_small'] for v in range(2)]):
        y = 60 + row * 230
        for col, direction in enumerate(('l', 'r')):
            for level_i, suffix in enumerate(('', '@fern')):
                file = f'{kind}_{variant}_rest_{direction}_f0{suffix}.png'
                draw.text((col * 640 + level_i * 320 + 8, y + 5), file[:-4], font=FONT, fill='#eadfc8')
                image = Image.open(ASSETS / 'animals' / file).convert('RGBA')
                factor = 2 if not suffix else 8
                image = image.resize((image.width * factor, image.height * factor), Image.Resampling.NEAREST)
                remains.paste(image, (col * 640 + level_i * 320 + 40, y + 70), image)
    remains.save(ART / 'carcasses-contact.png')
    tracks = Image.new('RGB', (1280, 950), '#344032')
    draw = ImageDraw.Draw(tracks)
    draw.text((12, 12), 'Dichtespuren auf Gras / volle Deckung, im Spiel nach Dichte / keine Spielaufnahme', font=FONT, fill='#eadfc8')
    ground = Image.open(ASSETS / 'ground/ground_grass_wet_0.png').convert('RGBA')
    for row, kind in enumerate(('deer', 'wolf', 'fox', 'rabbit')):
        for variant in range(4):
            x, y = variant * 320, 70 + row * 210
            draw.text((x + 8, y), f'{kind} / Variante {variant}', font=FONT, fill='#eadfc8')
            tracks.paste(ground, (x + 32, y + 35), ground)
            image = Image.open(ASSETS / f'animals/tracks_{kind}_{variant}.png').convert('RGBA')
            tracks.paste(image, (x + 32, y + 35), image)
    tracks.save(ART / 'tracks-contact.png')


if __name__ == '__main__':
    main()
