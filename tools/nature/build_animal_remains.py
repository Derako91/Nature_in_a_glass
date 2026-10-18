"""Kadaver und Dichtespuren im vorhandenen Pixelraster, ohne Simulationslogik."""
import json
from PIL import Image, ImageDraw
import build_animal_proof as a

b = a.b
OUT = a.OUT
SHADOW = (12, 26, 52, 77)
CONTOURS = {
    'carcass_large': {
        'l': ['.........', '.bb.bbbb.', 'bbbbbbbb.', '..k..k...'],
        'r': ['.........', '.bbbb.bb.', '.bbbbbbbb', '...k..k..'],
    },
    'carcass_small': {'l': ['bbb.', '.kb.'], 'r': ['.bbb', '.bk.']},
}
PALETTES = [('493B36', '706055', '998774'), ('434345', '686665', '96918A')]
TRACKS = {
    'deer': [(0, 0, 0), (2, 0, 0), (0, 1, 1), (2, 1, 1)],
    'wolf': [(0, 0, 1), (2, 0, 1), (1, 1, 0), (1, 2, 0)],
    'fox': [(1, 0, 1), (0, 1, 0), (1, 1, 0)],
    'rabbit': [(0, 0, 0), (2, 0, 0), (1, 2, 1), (3, 2, 1)],
}


def carcass(kind, direction, variant):
    rows = CONTOURS[kind][direction]
    w, h = len(rows[0]), len(rows)
    im = Image.new('RGBA', (w, h))
    palette = [b.rgb(c) + (255,) for c in PALETTES[variant]]
    for y, row in enumerate(rows):
        for x, c in enumerate(row):
            if c != '.' and x > 0 and y + 1 < h and rows[y + 1][x - 1] == '.':
                im.putpixel((x - 1, y + 1), SHADOW)
    for y, row in enumerate(rows):
        for x, c in enumerate(row):
            if c != '.':
                # Beide Konturen werden unabhängig beleuchtet: rechts oben hell.
                tone = 0 if c == 'k' else (2 if y <= 1 and x >= w // 2 else 1)
                im.putpixel((x, y), palette[tone])
    return im


def tracks(species):
    im = Image.new('RGBA', (64, 32))
    palette = [(83, 70, 48, 255), (137, 122, 82, 255), (165, 154, 108, 255)]
    # Lockere, feste Gruppen ohne Bewegung im Zeitraffer.
    for i, (x, y) in enumerate([(16, 15), (24, 12), (32, 16), (40, 13)]):
        for dx, dy, tone in TRACKS[species]:
            im.putpixel((x + dx, y + dy + i % 2), palette[tone])
    if species in ('deer', 'rabbit'):
        for x, y in [(24, 21), (35, 9), (44, 18)]:
            im.putpixel((x, y), palette[2])
            im.putpixel((x + 1, y - 1), palette[1])
    return im


def main():
    assets, images, checks = [], {}, []
    for kind in CONTOURS:
        mapping = []
        for variant in (0, 1):
            for direction in ('l', 'r'):
                filename = f'{kind}_{variant}_rest_{direction}_f0.png'
                im = carcass(kind, direction, variant)
                images[filename] = im
                mapping.append(dict(variant=variant, direction=direction, frame=0, file=filename))
        assets.append(dict(id=kind, group='animals', size=list(im.size), anchor=[im.width // 2, im.height // 2], variants=2, state='rest', frames=1, frame_ms=None, directions=['l', 'r'], files=[m['file'] for m in mapping], frame_map=mapping, status='design_review'))
    for species in TRACKS:
        filename = f'tracks_{species}.png'
        images[filename] = tracks(species)
        assets.append(dict(id=f'tracks_{species}', group='animals', size=[64, 32], anchor=[32, 16], files=[filename], opacity='renderer_density', status='design_review'))
    diamond = b.diamond_mask()
    for filename, im in images.items():
        colors = {p[:3] for p in im.getdata() if p[3]}
        alpha = {p[3] for p in im.getdata()}
        is_track = filename.startswith('tracks_')
        assert len(colors) <= (3 if is_track else 6)
        assert alpha <= ({0, 255} if is_track else {0, 77, 255})
        if is_track:
            assert im.size == (64, 32)
            assert all(not im.getpixel((x, y))[3] or diamond.getpixel((x, y)) for y in range(32) for x in range(64))
            assert sum(p[3] > 0 for p in im.getdata()) < 60
        else:
            assert im.size == ((9, 4) if filename.startswith('carcass_large') else (4, 2))
        im.save(OUT / filename, icc_profile=b.PROFILE)
        checks.append(dict(file=filename, size=list(im.size), colors=len(colors), alpha=sorted(alpha), srgb=True))
    assert len({(im.size, im.tobytes()) for im in images.values()}) == len(images)
    manifest_path = OUT / 'manifest.json'
    manifest = json.loads(manifest_path.read_text(encoding='utf-8'))
    ids = {entry['id'] for entry in assets}
    manifest['assets'] = [entry for entry in manifest['assets'] if entry['id'] not in ids] + assets
    manifest['missing'] = [item for item in manifest.get('missing', []) if item not in ('carcasses', 'tracks')]
    manifest['integration_status'] = 'Spielintegration und Erkennbarkeit bei halber Größe offen'
    manifest_path.write_text(json.dumps(manifest, indent=2, ensure_ascii=False), encoding='utf-8')
    (OUT / 'remains-validation.json').write_text(json.dumps(checks, indent=2), encoding='utf-8')
    sheet = Image.new('RGB', (1350, 940), b.rgb('0C1A34'))
    d = ImageDraw.Draw(sheet)
    d.text((32, 24), 'KADAVER UND SPUREN / RUNDE 06', font=b.font(29, True), fill='#E5E4D4')
    d.text((32, 68), 'Pixelentwürfe · Licht rechts oben · keine Spielintegration', font=b.font(20), fill='#B9C6BB')
    for i, (filename, im) in enumerate(list(images.items())[:8]):
        x, y = 35 + (i % 4) * 330, 125 + (i // 4) * 125
        sp = im.resize((im.width * 12, im.height * 12), Image.Resampling.NEAREST)
        sheet.paste(sp, (x + 70, y + 5), sp)
        d.text((x, y + 65), ('Groß' if i < 4 else 'Klein') + f' · Variante {(i % 4) // 2} · ' + ('links' if i % 2 == 0 else 'rechts'), font=b.font(18), fill='#B9C6BB')
    names = ['Hirsch', 'Wolf', 'Fuchs', 'Kaninchen']
    for col, species in enumerate(TRACKS):
        x = 35 + col * 330
        d.text((x, 388), names[col] + ' · Spuren 4×', font=b.font(22, True), fill='#E5E4D4')
        for row, kind in enumerate(['grass', 'shrub', 'forest']):
            bg = a.panel_background(kind).convert('RGBA')
            overlay = images[f'tracks_{species}.png'].copy()
            overlay.putalpha(overlay.getchannel('A').point(lambda p: round(p * .65)))
            bg.alpha_composite(overlay)
            for index, size in enumerate(('carcass_large', 'carcass_small')):
                sp = images[f'{size}_0_rest_r_f0.png']
                bg.alpha_composite(sp, (25 + index * 14, 23 - sp.height))
            bg = bg.resize((256, 128), Image.Resampling.NEAREST)
            sheet.paste(bg, (x, 430 + row * 145), bg)
    d.text((32, 885), 'Spuren hier mit 65 % Deckkraft · Gras / Buschboden / Waldboden · Dichte später vom Renderer', font=b.font(20), fill='#B9C6BB')
    sheet.save(b.ROOT / 'assets/nature/_animals-remains.png', icc_profile=b.PROFILE)
    print(f'{len(images)} eindeutige PNGs: 8 Kadaver und 4 Spurenfelder; Maße, Paletten, Alpha und Rautengrenzen geprüft.')


if __name__ == '__main__':
    main()
