"""Ruhige Stil-B-Wasserrauten: vier Varianten, vier kohärente 500-ms-Phasen."""
import hashlib
import json
from pathlib import Path

import numpy as np
from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parents[2]
ART = ROOT / 'art/nature/water-style-b'
WATER = ROOT / 'assets/nature/water'
CONFIG = ART / 'config.json'
KINDS = ['water_deep', 'water_shallow', 'water_pond']


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def periodic(pixels):
    # Periodisch-plus-glatt zerlegen: gleicht Randdiskontinuitäten aus, ohne
    # die Wellen durch Spiegelkacheln in regelmäßige X-Formen zu verwandeln.
    h, w, _ = pixels.shape
    boundary = np.zeros_like(pixels)
    boundary[0] = pixels[-1] - pixels[0]
    boundary[-1] = -boundary[0]
    boundary[:, 0] += pixels[:, -1] - pixels[:, 0]
    boundary[:, -1] -= pixels[:, -1] - pixels[:, 0]
    yy, xx = np.mgrid[0:h, 0:w]
    denominator = 2 * np.cos(2 * np.pi * xx / w) + 2 * np.cos(2 * np.pi * yy / h) - 4
    denominator[0, 0] = 1
    smooth = np.fft.ifft2(np.fft.fft2(boundary, axes=(0, 1)) / denominator[:, :, None], axes=(0, 1)).real
    return pixels - smooth


def offset(pixels, shift, axis=1):
    low = int(np.floor(shift))
    fraction = shift - low
    return np.roll(pixels, low, axis=axis) * (1 - fraction) + np.roll(pixels, low + 1, axis=axis) * fraction


def project(texture):
    h, w = 64, 128
    yy, xx = np.mgrid[0:h, 0:w]
    u = (xx + .5 - w / 2) / w + (yy + .5) / h
    v = (yy + .5) / h - (xx + .5 - w / 2) / w
    mask = (u >= 0) & (u <= 1) & (v >= 0) & (v <= 1)
    data = np.zeros((h, w, 4), dtype='uint8')
    data[:, :, :3] = texture[np.clip(np.rint(v * 255), 0, 255).astype(int), np.clip(np.rint(u * 255), 0, 255).astype(int)]
    data[:, :, 3] = mask * 255
    data[~mask] = 0
    return Image.fromarray(data)


def export():
    cfg = json.loads(CONFIG.read_text())
    source_path = ART / cfg['source']['file']
    if sha(source_path) != cfg['source']['sha256']:
        raise ValueError('Wasserquelle geändert')
    source = Image.open(source_path).convert('RGB')
    manifest_path = ROOT / 'assets/nature/manifest.json'
    manifest = json.loads(manifest_path.read_text())
    yy, xx = np.mgrid[0:256, 0:256]
    weight = (np.sin(np.pi * xx / 255)**2 * np.sin(np.pi * yy / 255)**2)[:, :, None]
    records, templates, checks = [], {}, []
    for col, kind in enumerate(KINDS):
        materials = []
        for row in range(2):
            box = (round(col * source.width / 3) + 4, round(row * source.height / 2) + 4,
                   round((col + 1) * source.width / 3) - 4, round((row + 1) * source.height / 2) - 4)
            patch = np.asarray(source.crop(box).resize((256, 256), Image.Resampling.LANCZOS)).astype(float)
            patch = periodic(patch)
            patch = np.array(cfg['mean_colors'][kind]) + (patch - patch.mean(axis=(0, 1))) * .7
            materials.append(patch)
        frames_by_variant = []
        for variant in range(4):
            base = materials[0]
            alternate = materials[variant % 2]
            if variant >= 2:
                alternate = np.roll(materials[1], (variant * 19, variant * 31), axis=(0, 1))
            phases = []
            for frame, (step_x, step_y) in enumerate(((0, 0), (1, .5), (2, 0), (1, -.5))):
                shift_x = step_x * cfg['motion_uv_pixels'][kind]
                shift_y = step_y * cfg['motion_uv_pixels'][kind]
                # Jede Phase bewegt dieselben gemalten Wellen nur geringfügig.
                # Kein unabhängiges Neuwürfeln der Struktur und kein Aufblitzen.
                moved_base = offset(offset(base, shift_x), shift_y, axis=0)
                moved_alternate = offset(offset(alternate, shift_x), shift_y, axis=0)
                texture = moved_base * (1 - weight) + moved_alternate * weight
                texture = np.clip(np.rint(texture), 0, 255).astype('uint8')
                texture[-1] = texture[0]
                texture[:, -1] = texture[:, 0]
                templates[(kind, variant, frame)] = texture
                image = project(texture)
                path = WATER / f'{kind}_{variant}_f{frame}.png'
                image.save(path)
                phases.append(image)
                records.append(dict(file=path.name, kind=kind, variant=variant, frame=frame,
                                    sha256=sha(path), size=image.size, anchor=[64, 32]))
            frames_by_variant.append(phases)
            alpha = np.asarray(phases[0])[:, :, 3] > 0
            lumas = []
            for phase in phases:
                rgb = np.asarray(phase)[:, :, :3].astype(float)
                lumas.append(float((rgb @ np.array([.2126, .7152, .0722]))[alpha].mean()))
            spread = (max(lumas) - min(lumas)) / np.mean(lumas)
            assert spread < .15, (kind, variant, spread)
            difference = max(float(np.abs(np.asarray(phases[f]).astype(float) - np.asarray(phases[(f + 1) % 4]).astype(float)).mean()) for f in range(4))
            assert difference > 0, (kind, variant)
            checks.append(dict(kind=kind, variant=variant, mean_luma=lumas,
                               luma_spread=spread, maximum_mean_channel_delta=difference))
        # Standbild und Phase 0 sind identisch, auch für ältere Betrachter.
        frames_by_variant[0][0].save(WATER / f'{kind}_0.png')
        entry = next(e for e in manifest['assets'] if e['id'] == kind)
        entry.update(size=[128, 64], anchor=[64, 32], display_size=[64, 32],
                     resolution_scale=2, variants=4, frames=4, frame_ms=500,
                     preview_static=False, status='water_style_b_draft',
                     files=[f'water/{kind}_0.png'] + [f'water/{kind}_{v}_f{f}.png' for v in range(4) for f in range(4)])
        for frame in range(4):
            reference = templates[(kind, 0, frame)]
            for variant in range(4):
                uv = templates[(kind, variant, frame)]
                assert np.array_equal(uv[0], uv[-1]) and np.array_equal(uv[:, 0], uv[:, -1])
                assert np.array_equal(uv[0], reference[0]) and np.array_equal(uv[:, 0], reference[:, 0])
        proof(kind, frames_by_variant)
    manifest_path.write_text(json.dumps(manifest, ensure_ascii=False, indent=2) + '\n')
    (ART / 'export.json').write_text(json.dumps(dict(files=records, checks=checks,
        periodic_edges=True, variant_edges_identical_per_phase=True, frame_ms=500,
        loop_ms=2000, flow='small_periodic_texture_displacement_not_simulated_current',
        visual_approval='pending'), indent=2) + '\n')
    from export_stream_beds import CONFIG as STREAM_CONFIG, export as export_streams
    if STREAM_CONFIG.exists():
        export_streams()
    from export_stream_drops import ART as DROP_ART, export as export_drops
    if (DROP_ART / "config.json").exists():
        export_drops()
    print('48 Wasserphasen + 3 Standbilder, vier Varianten, kohärente 2-Sekunden-Schleifen exportiert.')


def proof(kind, variants):
    font = ImageFont.truetype('DejaVuSans.ttf', 18)
    sheet = Image.new('RGB', (1120, 680), '#344032')
    draw = ImageDraw.Draw(sheet)
    draw.text((12, 12), kind + ' / Varianten und Phasen / synthetische Materialprobe', font=font, fill='#eadfc8')
    for variant, phases in enumerate(variants):
        for frame, image in enumerate(phases):
            x, y = frame * 280, 50 + variant * 155
            draw.text((x + 8, y), f'Variante {variant} / Phase {frame}', font=font, fill='#eadfc8')
            shown = image.resize((256, 128), Image.Resampling.NEAREST)
            sheet.paste(shown, (x + 12, y + 24), shown)
    sheet.save(ART / (kind + '-contact.png'))
    frames = []
    for frame in range(4):
        panel = Image.new('RGB', (1024, 600), '#344032')
        for variant in range(4):
            image = variants[variant][frame].resize((512, 256), Image.Resampling.NEAREST)
            panel.paste(image, ((variant % 2) * 512, (variant // 2) * 300), image)
        frames.append(panel)
    frames[0].save(ART / (kind + '.gif'), save_all=True, append_images=frames[1:], duration=500, loop=0)


if __name__ == '__main__':
    export()
