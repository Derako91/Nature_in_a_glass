"""Native Vergleichsbilder, Bildruhe und Schriftabdeckung des Inselwelt-Prüflaufs auswerten."""
import hashlib
import json
from pathlib import Path
import subprocess
import numpy as np
from PIL import Image, ImageDraw

ROOT=Path(__file__).resolve().parents[2]
ART=ROOT/'art/nature/islandworld-cartography'
NAMES=[('Sonnenwerder',129,32),('Lerchenland',105,76),('Möweneiland',58,38),
       ('Bernsteinwerder',137,57),('Robbeninsel',41,94),('Heideinsel',69,127)]


def check():
    for label in ('before','after'):
        records=json.loads((ART/(label+'-captures.json')).read_text())
        assert len(records)==12
        for record in records:
            assert Image.open(ART/record['image']).size==(1440,900)
        stable=next(r for r in records if r['image']==label+'-karte-all.png')
        assert stable['paused_view_changed_pixels']==0
    font=ROOT/'assets/fonts/IMFeENit28P.ttf'
    font_hash=hashlib.sha256(font.read_bytes()).hexdigest()
    assert font_hash=='47cd75dce54b1f2e0831359d22d5e688f519d68ae45706b664fd310fd0e3ccf7'
    charset=subprocess.check_output(['fc-query','--format','%{charset}',str(font)],text=True)
    covered=set()
    for token in charset.split():
        span=token.split('-');lo=int(span[0],16);hi=int(span[-1],16)
        covered.update(range(lo,hi+1))
    assert all(ord(c) in covered for c in ''.join(n for n,_,_ in NAMES)+'ÄÖÜäöüß')
    before=np.asarray(Image.open(ART/'before-karte-all.png').convert('RGB'))
    after=np.asarray(Image.open(ART/'after-karte-all.png').convert('RGB'))
    mask=np.zeros(before.shape[:2],dtype=bool)
    board=Image.new('RGB',(600,45+len(NAMES)*62),'#17242d');draw=ImageDraw.Draw(board)
    draw.text((15,12),'Bisher                          Seekarten-Schrift (native Ausschnitte)',fill='white')
    for row,(name,x,y) in enumerate(NAMES):
        cx=162+(x+.5)*4.5;cy=122+(y+.5)*4.5
        box=(round(cx-110),round(cy-26),round(cx+110),round(cy+26))
        mask[box[1]:box[3],box[0]:box[2]]=True
        for col,phase in enumerate(('before','after')):
            crop=Image.open(ART/(phase+'-karte-all.png')).crop(box).convert('RGB')
            board.paste(crop,(15+col*290,40+row*62))
    board.save(ART/'name-details-comparison.png')
    diff=np.any(before!=after,axis=2)
    outside=int(np.count_nonzero(diff & ~mask))
    result=dict(base_commit='66d829e',screenshots=24,seed=2024,world_size=[160,160],
        requested_zooms=[1,2,8],extra_overview_zooms=[.4,.45],
        font_sha256=font_hash,german_glyphs='passed',paused_view_changed_pixels=0,
        changed_pixels_outside_six_label_regions=outside,visual_approval='pending')
    (ART/'validation.json').write_text(json.dumps(result,ensure_ascii=False,indent=2)+'\n')
    print(json.dumps(result,ensure_ascii=False))


if __name__=='__main__':check()
