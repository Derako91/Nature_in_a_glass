"""Echte Spiel-/Entwurfsbeobachter-Aufnahmen auf einer großen Insel; Metadaten je Bild."""
import argparse
import os
from pathlib import Path
import subprocess
import time
import json
from PIL import Image

ROOT=Path(__file__).resolve().parents[2]
ART=ROOT/'art/nature/canoe-and-hunt-style-b'


def capture(binary,preview,display):
    env=dict(os.environ,DISPLAY=display,LIBGL_ALWAYS_SOFTWARE='1')
    cases=[('coast','47,84')]+([('hunt-gallery','44,77')] if preview else [])
    records=[]
    for label,select in cases:
        for zoom in (8,32):
            name=('preview-' if preview else 'baseline-')+label+f'-zoom{zoom}'
            args=['--seed','5','--groesse','gross','--years','10','--tiere','hirsche',
                  '--menschen','--danach','30','--select',select,'--zoom',str(zoom)]
            if preview:args+=['--bilder',str(ART/'sprites')]
            log=Path('/tmp')/(name+'.log')
            with log.open('w') as stream:
                p=subprocess.Popen([str(binary),*args],cwd=ROOT,env=env,stdout=stream,stderr=stream)
                try:
                    deadline=time.monotonic()+180
                    while time.monotonic()<deadline:
                        if p.poll() is not None:raise RuntimeError(f'{name}: Programm beendet; siehe {log}')
                        with __import__('tempfile').TemporaryDirectory(prefix='codex-capture-') as temp:
                            shot=Path(temp)/'screen.png'
                            subprocess.run(['import','-window','root',str(shot)],env=env,check=True,stderr=subprocess.DEVNULL)
                            im=Image.open(shot).convert('RGB')
                            if len(im.crop((0,0,1440,65)).getcolors(1000000) or [])>100:
                                time.sleep(.5)
                                subprocess.run(['import','-window','root',str(ART/(name+'.png'))],env=env,check=True,stderr=subprocess.DEVNULL)
                                records.append(dict(image=name+'.png',args=args,preview_only=preview,
                                    description='Vorläufige Meerplatzierung; Person8/Hirsch als Bildgalerie' if preview else 'Unverändertes v0.18.1-Spiel'))
                                print(name,flush=True);break
                        time.sleep(.3)
                    else:raise RuntimeError(f'{name}: keine Spielansicht innerhalb 180s')
                finally:
                    p.terminate();p.wait(timeout=10)
    (ART/('preview-captures.json' if preview else 'baseline-captures.json')).write_text(json.dumps(records,ensure_ascii=False,indent=2)+'\n')


if __name__=='__main__':
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('binary',type=Path)
    parser.add_argument('--preview',action='store_true')
    parser.add_argument('--display',default=':100')
    args=parser.parse_args()
    capture(args.binary.resolve(),args.preview,args.display)
