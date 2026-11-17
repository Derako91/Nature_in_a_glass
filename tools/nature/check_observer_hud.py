"""Native HUD-Ruhe und unveränderte Steuerfunktionen gegen v0.21.0 prüfen."""
import json
from pathlib import Path
import re
import subprocess
import numpy as np
from PIL import Image

ROOT=Path(__file__).resolve().parents[2]
ART=ROOT/'art/nature/observer-hud'
BASE='53d5d51'


def pixels(name):
    image=Image.open(ART/(name+'.png')).convert('RGB')
    assert image.size==(1440,900)
    return np.asarray(image)


def changed(a,b,rect):
    x0,y0,x1,y1=rect
    return int(np.count_nonzero(np.any(a[y0:y1,x0:x1]!=b[y0:y1,x0:x1],axis=2)))


def methods(source):
    matches=list(re.finditer(r'^    fn ([a-z_]+)\(',source,re.M))
    return {m.group(1):source[m.start():matches[i+1].start() if i+1<len(matches) else len(source)]
            for i,m in enumerate(matches)}


def check():
    a,b=pixels('after-initial'),pixels('after-long-date')
    mask=np.zeros(a.shape[:2],bool);mask[:32,158:530]=True;mask[:32,1150:]=True
    top=np.zeros(mask.shape,bool);top[:66,:]=True
    toolbar=int(np.count_nonzero(np.any(a!=b,axis=2)&top&~mask))
    assert toolbar==0,toolbar
    keys=changed(a,pixels('after-residents'),(1048,92,1231,365))
    assert keys==0,keys
    quiet=changed(pixels('quiet-a'),pixels('quiet-b'),(0,0,1440,900))
    assert quiet==0,quiet
    adult,child=pixels('interaction-resident-adult'),pixels('interaction-resident-child')
    title=changed(adult,child,(16,80,776,130))
    separator=changed(adult,child,(245,277,765,278))
    assert title==0 and separator==0,(title,separator)
    deceased=pixels('interaction-deceased')
    long_title=changed(pixels('after-long-register'),deceased,(16,80,776,130))
    long_separator=changed(pixels('after-long-register'),deceased,(245,277,765,278))
    assert long_title==0 and long_separator==0,(long_title,long_separator)
    restored_speed=changed(pixels('interaction-before-landing'),pixels('interaction-landed'),(710,6,777,29))
    assert restored_speed==0,restored_speed
    old=subprocess.check_output(['git','show',BASE+':crates/app/src/main.rs'],cwd=ROOT,text=True)
    new=(ROOT/'crates/app/src/main.rs').read_text()
    before,after=methods(old),methods(new)
    preserved=['advance','check_important_events','fresh','sync_pending','replace_sim',
        'save_path','save','load','last_event_id','take_from_clock','give_to_clock',
        'season_name','time_text','boat_countdown','status_bar','terrain_buttons',
        'animal_row','people_row']
    for name in preserved:
        assert re.sub(r'\s+','',before[name])==re.sub(r'\s+','',after[name]),name
    # Bilder, Kern und Wasser-/Küstenzeichnung gehören nicht zur HUD-Änderung.
    assert not subprocess.check_output(['git','diff',BASE,'--','crates/core','assets/nature','crates/app/src/view.rs'],cwd=ROOT)
    result=dict(base_commit=BASE,static_toolbar_changed_pixels=toolbar,
        fixed_statistics_keys_changed_pixels=keys,paused_screen_changed_pixels=quiet,
        resident_title_frame_changed_pixels=title,resident_biography_separator_changed_pixels=separator,
        deceased_title_frame_changed_pixels=long_title,deceased_biography_separator_changed_pixels=long_separator,
        restored_speed_button_changed_pixels=restored_speed,
        unchanged_control_methods=preserved,visual_approval='pending')
    (ART/'validation.json').write_text(json.dumps(result,ensure_ascii=False,indent=2)+'\n')
    print(json.dumps(result,ensure_ascii=False))


if __name__=='__main__':check()
