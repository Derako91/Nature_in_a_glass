"""Native HUD-Prüfaufnahmen bei 1440×900, Xvfb :101; Programmdatei als erstes Argument."""
import ctypes as C, os,subprocess,time,json,sys
from pathlib import Path
from PIL import Image
root=Path(__file__).resolve().parents[2];out=root/'art/nature/observer-hud'
env=dict(os.environ,DISPLAY=':101',XDG_RUNTIME_DIR='/workspace/.nature-tools/runtime',XDG_CACHE_HOME='/workspace/.nature-tools/cache',LIBGL_ALWAYS_SOFTWARE='1')
x=C.CDLL('libX11.so.6');t=C.CDLL('libXtst.so.6')
x.XOpenDisplay.argtypes=[C.c_char_p];x.XOpenDisplay.restype=C.c_void_p
d=x.XOpenDisplay(b':101');assert d
x.XFlush.argtypes=[C.c_void_p]
t.XTestFakeMotionEvent.argtypes=[C.c_void_p,C.c_int,C.c_int,C.c_int,C.c_ulong]
t.XTestFakeButtonEvent.argtypes=[C.c_void_p,C.c_uint,C.c_int,C.c_ulong]
def click(px,py):
 t.XTestFakeMotionEvent(d,0,px,py,0);t.XTestFakeButtonEvent(d,1,1,0);x.XFlush(d);time.sleep(.08);t.XTestFakeButtonEvent(d,1,0,0);x.XFlush(d);time.sleep(.2)
def shot(name):
 t.XTestFakeMotionEvent(d,0,1010,860,0);x.XFlush(d);time.sleep(.15)
 subprocess.run(['import','-window','root',str(out/(name+'.png'))],env=env,check=True)
def start(args):
 log=open('/tmp/hud-interaction-'+str(time.monotonic_ns())+'.log','w')
 p=subprocess.Popen([sys.argv[1],*args],cwd=root,env=env,stdout=log,stderr=log)
 deadline=time.monotonic()+90
 while time.monotonic()<deadline:
  if p.poll() is not None:raise RuntimeError('game exited')
  subprocess.run(['import','-window','root','/tmp/hud-ready.png'],env=env,check=True,stderr=subprocess.DEVNULL)
  if len(Image.open('/tmp/hud-ready.png').convert('RGB').crop((0,0,1440,65)).getcolors(1000000) or [])>100:
   time.sleep(.5);return p
  time.sleep(.3)
 raise RuntimeError('no game')
def run():
 records=[]
 p=start(['--seed','2024'])
 try:
  shot('interaction-before-landing');click(1250,18);shot('interaction-rushing')
  deadline=time.monotonic()+90
  while time.monotonic()<deadline:
   subprocess.run(['import','-window','root','/tmp/hud-live.png'],env=env,check=True)
   Image.open('/tmp/hud-live.png').crop((0,875,1440,900)).save('/tmp/hud-status.png')
   status=subprocess.check_output(['tesseract','/tmp/hud-status.png','stdout','--psm','7'],stderr=subprocess.DEVNULL,text=True).lower()
   if 'gelandet' in status:
    shot('interaction-landed');break
   time.sleep(.5)
  else:raise RuntimeError('Keine Landung beobachtet')
  records.append('Echten Bootszähler ab Jahr1 angeklickt; Landestatus per OCR beobachtet; normales Tempo wiederhergestellt')
 finally:p.terminate();p.wait(timeout=10)
 p=start(['--seed','2024'])
 try:
  click(995,42);click(1220,42);time.sleep(.8);shot('interaction-auto-boat-off')
  click(995,42);click(1220,42);time.sleep(.8);shot('interaction-auto-boat-on');records.append('Haken aus/an und jeweils Neue Insel; reservierten Zählerplatz vergleichen')
 finally:p.terminate();p.wait(timeout=10)
 p=start(['--seed','2024','--years','10','--menschen','--danach','3','--bewohner'])
 try:
  shot('interaction-resident-adult');click(80,260);shot('interaction-resident-child');records.append('Erwachsene und Kind ausgewählt; Fenster und Metadatenplätze vergleichen')
 finally:p.terminate();p.wait(timeout=10)
 (out/'interaction-notes.json').write_text(json.dumps(records,ensure_ascii=False,indent=2)+'\n')
 print('Interaktionsaufnahmen fertig')


def extra():
 p=start(['--seed','2024','--groesse','inselwelt','--years','3'])
 try:
  shot('quiet-a');time.sleep(1);shot('quiet-b')
 finally:p.terminate();p.wait(timeout=10)
 p=start(['--seed','5','--years','10','--menschen','--danach','30','--bewohner'])
 try:
  t.XTestFakeMotionEvent(d,0,100,390,0);x.XFlush(d)
  for _ in range(14):
   t.XTestFakeButtonEvent(d,5,1,0);t.XTestFakeButtonEvent(d,5,0,0);x.XFlush(d);time.sleep(.06)
  shot('interaction-register-bottom')
  click(80,450);shot('interaction-deceased')
 finally:p.terminate();p.wait(timeout=10)


if __name__=='__main__':
 if len(sys.argv)>2 and sys.argv[2]=='--extra':extra()
 else:run()
