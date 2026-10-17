"""Tierstudien über lokales ComfyUI. Rohbilder bleiben gemäß Abschnitt 9.1 lokal."""
import json
import time
import urllib.request
from urllib.parse import urlencode
from generate_sources import ROOT, request

OUT=ROOT/'art/nature/animals'
SPECIES={
    'deer':'one adult red deer stag, slender tan brown body, four thin legs with hooves, small simple antlers, narrow head and long neck',
    'wolf':'one grey wolf, lean low elongated body, four paws, short pointed ears, straight hanging tail, long muzzle',
    'fox':'one red fox, small slender rusty red body, pointed ears and muzzle, large bushy tail with pale cream tip',
    'rabbit':'one small wild brown rabbit, compact oval body, two long upright ears, small pale tail, natural proportions',
}

def main():
    (OUT/'raw').mkdir(parents=True,exist_ok=True)
    for i,(species,subject) in enumerate(SPECIES.items()):
        for j,direction in enumerate(['l','r']):
            name=species+'_'+direction
            if (OUT/'raw'/(name+'.png')).exists():continue
            facing='left' if direction=='l' else 'right'
            text=('Original naturalistic pixel painting for an early 1990s strategy game. '+subject+'. '
                  'Single whole animal, standing still, all feet visible. Facing '+facing+', head on the '+facing+' side of the image. '
                  'Elevated three quarter side view, viewed slightly from above, warm earthy colors, '
                  'small clear silhouette, light from upper RIGHT regardless of facing, left side shaded, '
                  'isolated on a plain pale ivory background. Full body centered with wide empty margins. '
                  'Restrained hand painted texture, realistic animal anatomy, no background scenery.')
            wf={
                '1':{'class_type':'CheckpointLoaderSimple','inputs':{'ckpt_name':'sd_xl_base_1.0.safetensors'}},
                '2':{'class_type':'CLIPTextEncode','inputs':{'text':text,'clip':['1',1]}},
                '3':{'class_type':'CLIPTextEncode','inputs':{'text':'cute, cartoon, big eyes, smile, anthropomorphic, clothes, collar, person, forest, plants, ground, platform, many animals, herd, collage, sprite sheet, text, letters, black outline, photorealistic, 3d render, extra legs, cropped body','clip':['1',1]}},
                '4':{'class_type':'EmptyLatentImage','inputs':{'width':768,'height':768,'batch_size':1}},
                '5':{'class_type':'KSampler','inputs':{'model':['1',0],'positive':['2',0],'negative':['3',0],'latent_image':['4',0],'seed':2026101900+i*2+j,'steps':24,'cfg':6.5,'sampler_name':'dpmpp_2m','scheduler':'karras','denoise':1.0}},
                '6':{'class_type':'VAEDecode','inputs':{'samples':['5',0],'vae':['1',2]}},
                '7':{'class_type':'SaveImage','inputs':{'images':['6',0],'filename_prefix':'nature_in_a_glass/animals/'+name}},
            }
            (OUT/(name+'.api.json')).write_text(json.dumps(wf,indent=2),encoding='utf-8')
            pid=request('/prompt',{'prompt':wf,'client_id':'nature-animals-01'})['prompt_id']
            print('START',name,pid,flush=True)
            deadline=time.monotonic()+600
            while time.monotonic()<deadline:
                h=request('/history/'+pid)
                if pid in h:
                    entry=h[pid]
                    (OUT/(name+'.history.json')).write_text(json.dumps(entry,indent=2),encoding='utf-8')
                    if entry.get('status',{}).get('status_str')=='error':raise RuntimeError(entry['status'])
                    pic=entry['outputs']['7']['images'][0]
                    with urllib.request.urlopen('http://127.0.0.1:8188/view?'+urlencode(pic)) as r:
                        (OUT/'raw'/(name+'.png')).write_bytes(r.read())
                    print('DONE',name,flush=True)
                    break
                time.sleep(3)
            else:raise TimeoutError(pid)

if __name__=='__main__':main()
