"""Karten- und Küstenprüfung der Inselwelt; unveränderte CLI, native Aufnahmen."""
import argparse
import json
import os
from pathlib import Path
import subprocess
import tempfile
import time
from PIL import Image

ROOT=Path(__file__).resolve().parents[2]
ART=ROOT/'art/nature/islandworld-cartography'


def capture(binary,label,display,extra_only):
    env=dict(os.environ,DISPLAY=display,LIBGL_ALWAYS_SOFTWARE='1')
    cases=[] if extra_only else [(f'{view}-zoom{zoom}',view,zoom,None) for view in ('karte','iso') for zoom in (1,2,8)]
    cases += [('karte-all','karte',.45,None),('iso-all','iso',.4,None),
              ('karte-focused-zoom2','karte',2,'105,76'),
              ('karte-umlaut-zoom8','karte',8,'58,38'),
              ('karte-long-name-zoom8','karte',8,'137,57'),
              ('iso-coast-zoom8','iso',8,'86,58')]
    meta=ART/(label+'-captures.json')
    records=json.loads(meta.read_text()) if extra_only and meta.exists() else []
    for name,view,zoom,select in cases:
        full=label+'-'+name
        args=['--seed','2024','--groesse','inselwelt','--years','3','--zoom',str(zoom)]
        if view=='karte':args+=['--view','karte']
        if select:args+=['--select',select]
        with (Path('/tmp')/('islandworld-'+full+'.log')).open('w') as log:
            p=subprocess.Popen([str(binary),*args],cwd=ROOT,env=env,stdout=log,stderr=log)
            try:
                deadline=time.monotonic()+100
                with tempfile.TemporaryDirectory(prefix='codex-island-review-') as temp:
                    shot=Path(temp)/'screen.png'
                    while time.monotonic()<deadline:
                        if p.poll() is not None:raise RuntimeError(full+' exited')
                        subprocess.run(['import','-window','root',str(shot)],env=env,check=True,stderr=subprocess.DEVNULL)
                        im=Image.open(shot).convert('RGB')
                        if len(im.crop((0,0,1440,65)).getcolors(1000000) or [])>100:
                            time.sleep(.5)
                            subprocess.run(['import','-window','root',str(ART/(full+'.png'))],env=env,check=True,stderr=subprocess.DEVNULL)
                            record=dict(image=full+'.png',args=args)
                            if name=='karte-all':
                                time.sleep(1)
                                subprocess.run(['import','-window','root',str(shot)],env=env,check=True,stderr=subprocess.DEVNULL)
                                first=Image.open(ART/(full+'.png')).convert('RGB').crop((0,66,1044,875))
                                second=Image.open(shot).convert('RGB').crop((0,66,1044,875))
                                first_bytes,second_bytes=first.tobytes(),second.tobytes()
                                record['paused_view_changed_pixels']=sum(first_bytes[i:i+3]!=second_bytes[i:i+3] for i in range(0,len(first_bytes),3))
                            records.append(record);print(full,flush=True);break
                        time.sleep(.25)
                    else:raise RuntimeError(full+' not ready')
            finally:p.terminate();p.wait(timeout=10)
    meta.write_text(json.dumps(records,ensure_ascii=False,indent=2)+'\n')


if __name__=='__main__':
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('binary',type=Path)
    parser.add_argument('label',choices=['before','after'])
    parser.add_argument('--display',default=':100')
    parser.add_argument('--extra-only',action='store_true')
    args=parser.parse_args();ART.mkdir(parents=True,exist_ok=True)
    capture(args.binary.resolve(),args.label,args.display,args.extra_only)
