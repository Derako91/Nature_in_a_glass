"""ComfyUI-Adapter: vorhandene SDXL-Graphen, optionale Referenzbindung."""
import json
import time
import urllib.request
from urllib.parse import urlencode, urlsplit
from uuid import uuid4

from .model import write

DIRECTIONS = {
    'se':'facing screen RIGHT and toward viewer, head at right, elevated three-quarter front-side view',
    'sw':'facing screen LEFT and toward viewer, head at left, elevated three-quarter front-side view',
    'ne':'facing screen RIGHT and AWAY from viewer, head at upper right, visible back and rump, elevated rear three-quarter view',
    'nw':'facing screen LEFT and AWAY from viewer, head at upper left, visible back and rump, elevated rear three-quarter view',
    'r':'facing screen RIGHT toward viewer, elevated three-quarter view',
    'l':'facing screen LEFT toward viewer, elevated three-quarter view',
}

class Comfy:
    def __init__(self, url='http://127.0.0.1:8188'):
        parts=urlsplit(url)
        if parts.scheme not in ('http','https') or not parts.hostname:
            raise ValueError('Ungültiger ComfyUI-Endpunkt.')
        self.url=url.rstrip('/')

    def request(self, path, data=None):
        req=urllib.request.Request(self.url+path,
            None if data is None else json.dumps(data).encode(),
            {'Content-Type':'application/json'})
        with urllib.request.urlopen(req,timeout=30) as response:
            return json.load(response)

    def upload(self, path):
        boundary='sprite'+uuid4().hex
        filename=uuid4().hex+'.png'
        body=(f'--{boundary}\r\nContent-Disposition: form-data; name="image"; filename="{filename}"\r\nContent-Type: image/png\r\n\r\n'.encode()
              +path.read_bytes()+f'\r\n--{boundary}--\r\n'.encode())
        req=urllib.request.Request(self.url+'/upload/image',body,
            {'Content-Type':f'multipart/form-data; boundary={boundary}'})
        with urllib.request.urlopen(req,timeout=30) as response:
            result=json.load(response)
        return (result.get('subfolder','')+'/'+result['name']).lstrip('/')

    def capabilities(self):
        info=self.request('/object_info')
        names=info.get('CheckpointLoaderSimple',{}).get('input',{}).get('required',{}).get('ckpt_name',[[]])[0]
        return {'nodes':sorted(info), 'checkpoints':names,
                'stats':self.request('/system_stats')}

    def collect(self, prompt_id, target, history_path, timeout=600):
        deadline=time.monotonic()+timeout
        while time.monotonic()<deadline:
            history=self.request('/history/'+prompt_id)
            if prompt_id in history:
                entry=history[prompt_id]
                write(history_path,entry)
                if entry.get('status',{}).get('status_str')=='error':
                    raise RuntimeError('ComfyUI-Ausführung fehlgeschlagen; siehe '+str(history_path))
                pictures=entry.get('outputs',{}).get('7',{}).get('images',[])
                if not pictures:
                    raise RuntimeError('Keine Bildausgabe im abgeschlossenen ComfyUI-Auftrag.')
                with urllib.request.urlopen(self.url+'/view?'+urlencode(pictures[0]),timeout=30) as response:
                    target.write_bytes(response.read())
                return
            time.sleep(2)
        raise TimeoutError('Zeitlimit. Prompt '+prompt_id+' bleibt im Journal; erneuter Lauf wartet weiter, statt erneut zu generieren.')

def workflow(job, profile, frame, seed, uploaded_reference=None):
    strategy=job['strategy']
    if strategy=='auto':
        strategy='reference' if uploaded_reference else 'text'
    text=' '.join((job['subject'],frame['pose'],DIRECTIONS[frame['direction']],profile['style_prompt'],
                   'One single subject only. Keep the same anatomy, colors and markings across views.'))
    wf={
        '1':{'class_type':'CheckpointLoaderSimple','inputs':{'ckpt_name':profile['checkpoint']}},
        '2':{'class_type':'CLIPTextEncode','inputs':{'text':text,'clip':['1',1]}},
        '3':{'class_type':'CLIPTextEncode','inputs':{'text':profile['negative_prompt'],'clip':['1',1]}},
        '4':{'class_type':'EmptyLatentImage','inputs':{'width':job['width'],'height':job['height'],'batch_size':1}},
        '5':{'class_type':'KSampler','inputs':{'model':['1',0],'positive':['2',0],'negative':['3',0],
             'latent_image':['4',0],'seed':seed,'steps':job['steps'],'cfg':job['cfg'],
             'sampler_name':'dpmpp_2m','scheduler':'karras','denoise':1.0}},
        '6':{'class_type':'VAEDecode','inputs':{'samples':['5',0],'vae':['1',2]}},
        '7':{'class_type':'SaveImage','inputs':{'images':['6',0],'filename_prefix':'sprite_agent/'+frame['key']}}
    }
    if strategy in ('reference','img2img'):
        if not uploaded_reference:
            raise ValueError('Referenzbild fehlt.')
        wf['8']={'class_type':'LoadImage','inputs':{'image':uploaded_reference}}
    if strategy=='reference':
        wf['9']={'class_type':'IPAdapterUnifiedLoader','inputs':{'model':['1',0],'preset':'PLUS (high strength)'}}
        wf['10']={'class_type':'IPAdapterAdvanced','inputs':{'model':['9',0],'ipadapter':['9',1],
            'image':['8',0],'weight':job['reference_weight'],'weight_type':'linear',
            'combine_embeds':'concat','start_at':0.0,'end_at':0.85,'embeds_scaling':'V only'}}
        wf['5']['inputs']['model']=['10',0]
    if strategy=='img2img':
        wf['9']={'class_type':'VAEEncode','inputs':{'pixels':['8',0],'vae':['1',2]}}
        wf['5']['inputs'].update(latent_image=['9',0],denoise=job['denoise'])
        del wf['4']
    return wf,strategy
