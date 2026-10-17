"""Lokale ComfyUI-Vorlagen für das erste Gestaltungs-Prüfblatt."""
import json
import time
import urllib.request
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'art/nature/comfyui'
BASE = 'http://127.0.0.1:8188'
STYLE = ('Original isometric strategy game art, warm hand painted miniature, restrained natural colors, '
         'simple readable shapes, light from upper right, darker left side, orthographic view. ')
SUBJECTS = {
    'tree_young': 'One single young slender tree, pointed narrow light green crown, visible thin brown trunk, entire tree centered with generous margin, isolated on pure white background, no ground, no grass, no other objects.',
    'tree_mid': 'One single medium mature tree, compact oval muted green crown, visible brown trunk, entire tree centered with generous margin, isolated on pure white background, no ground, no grass, no other objects.',
    'tree_old': 'One single old broad deciduous tree, wide closed dark forest green canopy, asymmetrical rounded foliage masses, visible sturdy short brown trunk, entire tree centered with generous margin, isolated on pure white background, no ground, no grass, no other objects.',
    'shrub': 'One single low wide rounded olive green bush, three connected foliage clumps, no tall trunk, entire bush centered with generous margin, isolated on pure white background, no ground, no grass, no other objects.',
    'snag_standing': 'One single dead standing tree trunk, slender weathered grey brown snag with two short broken bare branches, no foliage, entire trunk centered with generous margin, isolated on pure white background, no ground, no other objects.',
    'soil': 'Flat overhead seamless texture of fine warm brown bare earth, subtle small granular brush marks, uniform lighting, no objects, no stones, no grass, no perspective, fills entire image edge to edge.',
    'rock': 'Flat overhead seamless texture of grey weathered bedrock, quiet broad angular strata and fine cracks, uniform lighting, no loose stones, no objects, no plants, no perspective, fills entire image edge to edge.',
    'grass': 'Flat overhead seamless texture of short muted meadow grass, small fine brush strokes, soft olive and sage greens, uniform lighting, no flowers, no rocks, no objects, no perspective, fills entire image edge to edge.',
    'water': 'Flat overhead seamless texture of calm blue water, extremely subtle soft ripples, broad quiet blue surface, no white highlights, no foam, no shore, no objects, no perspective, fills entire image edge to edge.',
    'burnt': 'Flat overhead seamless texture of dark brown charcoal scorched earth with sparse small grey ash flecks, no fire, no objects, no stones, no perspective, fills entire image edge to edge.',
}

def request(path, data=None):
    payload = None if data is None else json.dumps(data).encode()
    req = urllib.request.Request(BASE + path, payload, {'Content-Type': 'application/json'})
    with urllib.request.urlopen(req, timeout=30) as r:
        return json.load(r)

def main():
    OUT.mkdir(parents=True, exist_ok=True)
    for i, (name, subject) in enumerate(SUBJECTS.items()):
        if (OUT / (name + '.png')).exists():
            continue
        workflow = {
            '1': {'class_type': 'CheckpointLoaderSimple', 'inputs': {'ckpt_name': 'sd_xl_base_1.0.safetensors'}},
            '2': {'class_type': 'CLIPTextEncode', 'inputs': {'text': STYLE + subject, 'clip': ['1', 1]}},
            '3': {'class_type': 'CLIPTextEncode', 'inputs': {'text': 'photograph, photorealistic, 3d render, glossy plastic, cartoon, black outline, text, letters, watermark, logo, grid, multiple panels, frame, dramatic lighting, harsh shadow, glowing, flowers, animals, buildings', 'clip': ['1', 1]}},
            '4': {'class_type': 'EmptyLatentImage', 'inputs': {'width': 768, 'height': 768, 'batch_size': 1}},
            '5': {'class_type': 'KSampler', 'inputs': {'model': ['1', 0], 'positive': ['2', 0], 'negative': ['3', 0], 'latent_image': ['4', 0], 'seed': 2026101700 + i, 'steps': 24, 'cfg': 6.5, 'sampler_name': 'dpmpp_2m', 'scheduler': 'karras', 'denoise': 1.0}},
            '6': {'class_type': 'VAEDecode', 'inputs': {'samples': ['5', 0], 'vae': ['1', 2]}},
            '7': {'class_type': 'SaveImage', 'inputs': {'images': ['6', 0], 'filename_prefix': 'nature_in_a_glass/' + name}},
        }
        (OUT / (name + '.api.json')).write_text(json.dumps(workflow, indent=2), encoding='utf-8')
        result = request('/prompt', {'prompt': workflow, 'client_id': 'nature-design-20261017'})
        prompt_id = result['prompt_id']
        print('START', name, prompt_id, flush=True)
        deadline = time.monotonic() + 600
        while time.monotonic() < deadline:
            history = request('/history/' + prompt_id)
            if prompt_id in history:
                entry = history[prompt_id]
                (OUT / (name + '.history.json')).write_text(json.dumps(entry, indent=2), encoding='utf-8')
                if entry.get('status', {}).get('status_str') == 'error':
                    raise RuntimeError(entry['status'])
                pic = entry['outputs']['7']['images'][0]
                from urllib.parse import urlencode
                with urllib.request.urlopen(BASE + '/view?' + urlencode(pic), timeout=30) as r:
                    (OUT / (name + '.png')).write_bytes(r.read())
                print('DONE', name, flush=True)
                break
            time.sleep(3)
        else:
            raise TimeoutError(prompt_id)

if __name__ == '__main__':
    main()
