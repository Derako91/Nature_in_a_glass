"""v0.22: Echte eingebettete Einbaum-/Jagdansichten, ohne Vorschau oder --bilder."""
import argparse
import os
from pathlib import Path
import subprocess
import time
import json
import tempfile
import shutil
import gzip
import ctypes as C
from PIL import Image

ROOT=Path(__file__).resolve().parents[2]
ART=ROOT/'art/nature/canoe-and-hunt-style-b/v0.22-integration'


def capture(binary,display,only):
    env=dict(os.environ,DISPLAY=display,LIBGL_ALWAYS_SOFTWARE='1')
    cases=json.loads((ART/'cases.json').read_text())
    if only:cases=[c for c in cases if c['name']==only]
    meta=ART/'captures.json'
    records=json.loads(meta.read_text()) if meta.exists() else []
    for case in cases:
        for zoom in (8,32):
            name=case['name']+f'-zoom{zoom}'
            args=case['args']+['--zoom',str(zoom)]
            assert '--bilder' not in args
            private=tempfile.TemporaryDirectory(prefix='canoe22-game-')
            executable=binary
            if 'save' in case:
                executable=Path(private.name)/binary.name
                shutil.copy2(binary,executable)
                folder=Path(private.name)/'spielstaende';folder.mkdir()
                with gzip.open(ART/case['save'],'rb') as source:
                    (folder/'welt.json').write_bytes(source.read())
            with (Path('/tmp')/('canoe22-'+name+'.log')).open('w') as log:
                p=subprocess.Popen([str(executable),*args],cwd=ROOT,env=env,stdout=log,stderr=log)
                try:
                    deadline=time.monotonic()+180
                    with tempfile.TemporaryDirectory() as temp:
                        shot=Path(temp)/'shot.png'
                        while time.monotonic()<deadline:
                            if p.poll() is not None:raise RuntimeError(name+' exited')
                            subprocess.run(['import','-window','root',str(shot)],env=env,check=True,stderr=subprocess.DEVNULL)
                            im=Image.open(shot).convert('RGB')
                            if len(im.crop((0,0,1440,65)).getcolors(1000000) or [])>100:
                                time.sleep(.5)
                                if 'save' in case:
                                    x=C.CDLL('libX11.so.6');xt=C.CDLL('libXtst.so.6')
                                    x.XOpenDisplay.argtypes=[C.c_char_p];x.XOpenDisplay.restype=C.c_void_p
                                    d=x.XOpenDisplay(display.encode());assert d
                                    x.XFlush.argtypes=[C.c_void_p]
                                    xt.XTestFakeMotionEvent.argtypes=[C.c_void_p,C.c_int,C.c_int,C.c_int,C.c_ulong]
                                    xt.XTestFakeButtonEvent.argtypes=[C.c_void_p,C.c_uint,C.c_int,C.c_ulong]
                                    xt.XTestFakeMotionEvent(d,0,740,42,0)
                                    xt.XTestFakeButtonEvent(d,1,1,0);x.XFlush(d);time.sleep(.08)
                                    xt.XTestFakeButtonEvent(d,1,0,0);x.XFlush(d)
                                    load_deadline=time.monotonic()+30
                                    while time.monotonic()<load_deadline:
                                        time.sleep(.25)
                                        subprocess.run(['import','-window','root',str(shot)],env=env,check=True,stderr=subprocess.DEVNULL)
                                        status=Path(temp)/'status.png'
                                        Image.open(shot).crop((0,875,1440,900)).save(status)
                                        text=subprocess.check_output(['tesseract',str(status),'stdout','--psm','7'],stderr=subprocess.DEVNULL,text=True)
                                        if 'geladen' in text.lower():break
                                    else:raise RuntimeError('Spielstand nicht geladen')
                                    xt.XTestFakeMotionEvent(d,0,1010,860,0);x.XFlush(d)
                                    time.sleep(.3)
                                subprocess.run(['import','-window','root',str(ART/(name+'.png'))],env=env,check=True,stderr=subprocess.DEVNULL)
                                records=[r for r in records if r['image']!=name+'.png']
                                records.append(dict(image=name+'.png',args=args,embedded=True,preview=False,meaning=case['meaning'],save=case.get('save')))
                                print(name,flush=True);break
                            time.sleep(.3)
                        else:raise RuntimeError('no game frame')
                finally:p.terminate();p.wait(timeout=10);private.cleanup()
    meta.write_text(json.dumps(records,ensure_ascii=False,indent=2)+'\n')


if __name__=='__main__':
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('binary',type=Path)
    parser.add_argument('--display',default=':102')
    parser.add_argument('--only')
    args=parser.parse_args();capture(args.binary.resolve(),args.display,args.only)
