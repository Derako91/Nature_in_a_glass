"""Native HUD-Prüfaufnahmen bei 1440×900, Xvfb :101; Programmdatei als erstes Argument."""
import os,subprocess,time,sys,json
from pathlib import Path
from PIL import Image
root=Path(__file__).resolve().parents[2];out=root/'art/nature/observer-hud';out.mkdir(parents=True,exist_ok=True)
env=dict(os.environ,DISPLAY=':101',XDG_RUNTIME_DIR='/workspace/.nature-tools/runtime',XDG_CACHE_HOME='/workspace/.nature-tools/cache',LIBGL_ALWAYS_SOFTWARE='1')
cases=[('initial',['--seed','2024']),('long-date',['--seed','2024','--years','9','--days','364']),('residents',['--seed','2024','--years','10','--menschen','--danach','3','--bewohner']),('archipelago',['--seed','2024','--groesse','inselwelt','--years','3'])]
if len(sys.argv)>3 and sys.argv[3]=='--long-register':
 cases=[('long-register',['--seed','5','--years','10','--menschen','--danach','30','--bewohner'])]
if len(sys.argv)>3 and sys.argv[3]=='--residents-only':
 cases=[c for c in cases if c[0]=='residents']
records=[]
for name,args in cases:
 label=sys.argv[2]+'-'+name
 with open('/tmp/hud-'+label+'.log','w') as log:
  p=subprocess.Popen([sys.argv[1],*args],cwd=root,env=env,stdout=log,stderr=log)
  try:
   deadline=time.monotonic()+100
   while time.monotonic()<deadline:
    if p.poll() is not None:raise RuntimeError(label+' exited')
    subprocess.run(['import','-window','root','/tmp/hud-shot.png'],env=env,check=True,stderr=subprocess.DEVNULL)
    im=Image.open('/tmp/hud-shot.png').convert('RGB')
    if len(im.crop((0,0,1440,65)).getcolors(1000000) or [])>100:
     time.sleep(.6);subprocess.run(['import','-window','root',str(out/(label+'.png'))],env=env,check=True,stderr=subprocess.DEVNULL)
     print(label,flush=True);records.append(dict(image=label+'.png',args=args));break
    time.sleep(.3)
   else:raise RuntimeError('not ready')
  finally:p.terminate();p.wait(timeout=10)
meta=out/(sys.argv[2]+'-captures.json')
if len(sys.argv)>3 and meta.exists():
 records=[r for r in json.loads(meta.read_text()) if r['image'] not in {n['image'] for n in records}]+records
meta.write_text(json.dumps(records,indent=2)+'\n')
