"""Zweite Stilrunde: einzelne Naturmotive mit geführter ComfyUI-Bildvariation."""
import json
import time
import urllib.request
from urllib.parse import urlencode
from pathlib import Path
from PIL import Image, ImageDraw
from generate_sources import request, ROOT

OUT=ROOT/'art/nature/comfyui-v2'
INPUT=Path('L:/Comfy-Desktop/ComfyUI-Shared/input')

def guide(name):
    im=Image.new('RGB',(512,768),'#f3f1e8')
    d=ImageDraw.Draw(im)
    if name=='shrub':
        shapes=[(105,400,285,555),(230,370,390,555),(170,345,315,540)]
        colors=['#485b2d','#82934a','#687c38']
    else:
        d.polygon([(241,600),(247,380),(270,380),(276,600),(286,612),(231,612)],fill='#806645')
        if name=='tree_young':
            shapes=[(172,265,330,445),(190,200,310,365),(215,155,288,300)]
        elif name=='tree_mid':
            shapes=[(105,230,295,470),(220,205,410,465),(163,145,355,395)]
        else:
            shapes=[(70,240,275,485),(220,245,440,480),(122,145,330,390),(232,155,404,390)]
        colors=['#31512a','#729046','#557639','#7e994c']
    for box,c in zip(shapes,colors):
        d.ellipse(box,fill=c)
    # Gegenständliche eigene Vorlagen statt abstrakter Kronen als Bildführung.
    source,crop={
        'tree_young':('tree_young',(48,20,214,277)),
        'tree_mid':('tree_young',(485,20,741,290)),
        'tree_old':('tree_mid',(20,10,744,650)),
        'shrub':('shrub',(113,545,320,622)),
    }[name]
    obj=Image.open(ROOT/'art/nature/comfyui'/(source+'.png')).convert('RGB').crop(crop)
    obj.thumbnail((390,550))
    im=Image.new('RGB',(512,768),'#f3f1e8')
    im.paste(obj,((512-obj.width)//2,610-obj.height))
    im.save(OUT/(name+'.guide.png'))
    im.save(INPUT/('nig_v2_'+name+'.png'))

def main():
    OUT.mkdir(parents=True,exist_ok=True)
    for i,name in enumerate(['tree_young','tree_mid','tree_old','shrub']):
        guide(name)
        description={
            'tree_young':'one small young sapling, slim pointed light green crown, thin short trunk',
            'tree_mid':'one medium tree with rounded compact green foliage crown, short visible trunk',
            'tree_old':'one old tree with broad rounded irregular dark olive green foliage crown, short sturdy trunk',
            'shrub':'one small low spreading shrub with three rounded olive green foliage clumps, no visible trunk',
        }[name]
        prompt=('A pixel art painting of '+description+'. Entire full tree including woody trunk and many leaves fully visible, centered on a plain ivory background. '
                'Detailed hand painted pixel art for an early 1990s DOS village simulation. Naturalistic miniature, hundreds of tiny irregular leaf clusters, '
                'rich stippled leafy texture, dithered leaf highlights, warm earthy greens, yellow olive highlights upper right, deep green shadows lower left, '
                'subtle volume, fine pixel texture, original artwork, simple compact silhouette, elevated three quarter view. '
                'Preserve the single object composition and its placement from the reference.')
        wf={
            '1':{'class_type':'CheckpointLoaderSimple','inputs':{'ckpt_name':'sd_xl_base_1.0.safetensors'}},
            '2':{'class_type':'CLIPTextEncode','inputs':{'text':prompt,'clip':['1',1]}},
            '3':{'class_type':'CLIPTextEncode','inputs':{'text':'single leaf, leaf closeup, macro, multiple trees, sprite sheet, grid, collage, forest, landscape, ground, base, platform, grass, flowers, text, border, glossy, 3d render, black outlines, cartoon, geometric, flat color, abstract, flat illustration, logo, icon, smooth vector','clip':['1',1]}},
            '4':{'class_type':'LoadImage','inputs':{'image':'nig_v2_'+name+'.png'}},
            '8':{'class_type':'VAEEncode','inputs':{'pixels':['4',0],'vae':['1',2]}},
            '5':{'class_type':'KSampler','inputs':{'model':['1',0],'positive':['2',0],'negative':['3',0],'latent_image':['8',0],'seed':2026101800+i,'steps':32,'cfg':7,'sampler_name':'dpmpp_2m','scheduler':'karras','denoise':.52}},
            '6':{'class_type':'VAEDecode','inputs':{'samples':['5',0],'vae':['1',2]}},
            '7':{'class_type':'SaveImage','inputs':{'images':['6',0],'filename_prefix':'nature_in_a_glass/v2/'+name}}
        }
        (OUT/(name+'.api.json')).write_text(json.dumps(wf,indent=2),encoding='utf-8')
        result=request('/prompt',{'prompt':wf,'client_id':'nature-proof-v2'})
        pid=result['prompt_id']
        print('START',name,pid,flush=True)
        deadline=time.monotonic()+600
        while time.monotonic()<deadline:
            history=request('/history/'+pid)
            if pid in history:
                entry=history[pid]
                (OUT/(name+'.history.json')).write_text(json.dumps(entry,indent=2),encoding='utf-8')
                if entry.get('status',{}).get('status_str')=='error': raise RuntimeError(entry['status'])
                pic=entry['outputs']['7']['images'][0]
                with urllib.request.urlopen('http://127.0.0.1:8188/view?'+urlencode(pic)) as r:
                    (OUT/(name+'.png')).write_bytes(r.read())
                print('DONE',name,flush=True)
                break
            time.sleep(3)
        else: raise TimeoutError(pid)

if __name__=='__main__': main()
