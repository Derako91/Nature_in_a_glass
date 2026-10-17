"""Pixelredaktion der ComfyUI-Tierstudien für 3–9 Pixel große Spielsprites.

Runde 1 enthält Fressbilder, Ruhe und je eine Geh-/Laufpose, keine Laufzyklen.
Die Richtungen besitzen getrennte Konturen und dieselbe Lichtrichtung.
"""
import json
from pathlib import Path
import numpy as np
from PIL import Image, ImageDraw
import build_proof as b

OUT=b.ROOT/'assets/nature/animals'
SIZES={'deer':(9,8),'deer_young':(6,6),'wolf':(8,5),'fox':(6,4),'rabbit':(3,3)}
PALETTES={
    'deer':['765133','B07A44','D2A268','C8B28A'],
    'deer_young':['88613D','BD935E','D6B782','C8B28A'],
    'wolf':['57616B','9AA0A6','C1C5C5','C9C9B8'],
    'fox':['78432D','D2682C','E89C57','DED1AF'],
    'rabbit':['796A52','A79879','C8B898','D9CBAF'],
}
# b: Fell mit ortsfester Beleuchtung; k: Glieder/Ohren; t: heller Schwanz.
STAND={
 'deer':{
  'r':['.....k.k.','......k..','......bb.','.bbbb.bb.','bbbbbbb..','.bbbbb...','..k..k...','..k..k...'],
  'l':['.k.k.....','..k......','.bb......','.bb.bbbb.','..bbbbbbb','...bbbbb.','...k..k..','...k..k..']},
 'deer_young':{
  'r':['....k.','....bb','.bbbbb','bbbbb.','.k..k.','.k..k.'],
  'l':['.k....','bb....','bbbbb.','.bbbbb','.k..k.','.k..k.']},
 'wolf':{
  'r':['.....k..','.bbbbbbb','bbbbbbb.','.k..k...','..k..k..'],
  'l':['..k.....','bbbbbbb.','.bbbbbbb','...k..k.','..k..k..']},
 'fox':{
  'r':['....k.','tbbbbb','bbbbb.','..k.k.'],
  'l':['.k....','bbbbbt','.bbbbb','.k.k..']},
 'rabbit':{
  'r':['k.k','bbb','tbb'],
  'l':['k.k','bbb','bbt']},
}
REST={
 'deer':{'r':['.........','.........','.....k.k.','......k..','......bb.','.bbbbbbb.','bbbbbbbb.','.bbbbbb..'],
         'l':['.........','.........','.k.k.....','..k......','.bb......','.bbbbbbb.','.bbbbbbbb','..bbbbbb.']},
 'deer_young':{'r':['......','......','....k.','.bbbbb','bbbbbb','.bbbb.'],
               'l':['......','......','.k....','bbbbb.','bbbbbb','.bbbb.']},
 'wolf':{'r':['........','........','.....k..','.bbbbbbb','bbbbbbb.'],
         'l':['........','........','..k.....','bbbbbbb.','.bbbbbbb']},
 'fox':{'r':['......','....k.','.bbbbb','tbbbb.'],
        'l':['......','.k....','bbbbb.','.bbbbt']},
 'rabbit':{'r':['...','.bb','tbb'],'l':['...','bb.','bbt']},
}

def contour(species,direction,state,frame):
    rows=[list(row) for row in (REST if state=='rest' else STAND)[species][direction]]
    w,h=SIZES[species]
    if state in ('walk','run'):
        if species=='rabbit':
            rows=[list(r) for r in (['.kk','bbb','tb.'] if direction=='r' else ['kk.','bbb','.bt'])]
            if state=='run':rows[0]=list('k.k')
        else:
            # Unterschiedliche Last- und Streckpose; noch keine Animationsframes.
            rows[h-1]=list('.'*w)
            feet=([1,w-2] if state=='run' else [2,w-3])
            if direction=='l' and state=='walk':feet=[1,w-3]
            for x in feet:rows[h-1][x]='k'
    if state=='graze' and frame==1:
        if species in ('deer','deer_young'):
            headx=w-2 if direction=='r' else 1
            # Kopf und Hals senken, Körper bleibt am festen Fußpunkt.
            for y in range(0,h-3):
                for x in range(headx-1,headx+2):
                    if 0<=x<w:rows[y][x]='.'
            rows[h-3][headx]='b';rows[h-2][headx]='b'
            if headx+1<w:rows[h-2][headx+1]='b'
            if species=='deer':rows[h-4][headx]='k'
        elif species in ('wolf','fox'):
            headx=w-1 if direction=='r' else 0
            rows[0]=list('.'*w)
            rows[h-2][headx]='b'
        else:
            rows[0]=list('.k.' if direction=='r' else 'k..')
    # Gestreckte Füße bleiben mit dem Körper verbunden.
    for x,c in enumerate(rows[h-1]):
        if c!='.' and not any(rows[h-2][n]!='.' for n in range(max(0,x-1),min(w,x+2))):
            rows[h-2][x]='k'
    return rows

def make_sprite(species,direction,state,frame):
    w,h=SIZES[species]
    im=Image.new('RGBA',(w,h));d=ImageDraw.Draw(im)
    # Ein bis zwei Pixel nach links, im verfügbaren Sprite-Rechteck.
    d.line((max(0,w//2-2),h-1,w//2,h-1),fill=b.rgb('0C1A34')+(77,))
    colors=[b.rgb(c) for c in PALETTES[species]]
    for y,row in enumerate(contour(species,direction,state,frame)):
        assert len(row)==w,(species,direction,row)
        for x,c in enumerate(row):
            if c=='.':continue
            if c=='s':tone=0
            elif c=='k':tone=0 if x<w/2 else 1
            elif c=='t':tone=3
            else:
                tone=2 if x>=w*.5 and y<h*.60 else 1
                if x<w*.3 and y>=h*.4:tone=0
            im.putpixel((x,y),colors[tone]+(255,))
    return im

def panel_background(kind):
    im=Image.open(b.ROOT/'assets/nature/ground/ground_soil_0.png').convert('RGBA')
    name={'grass':'ground_grass_wet_0','shrub':'ground_shrub_floor_0','forest':'ground_forest_floor_0'}[kind]
    im.alpha_composite(Image.open(b.ROOT/'assets/nature/ground'/(name+'.png')).convert('RGBA'))
    return im

def sheet(sprites,animated=False):
    im=Image.new('RGB',(1800,1690),b.rgb('0C1A34'));d=ImageDraw.Draw(im)
    d.text((48,28),'NATURE IN A GLASS / TIERE / PRÜFBLATT '+('02' if animated else '01'),font=b.font(21,True),fill='#91A99D')
    d.text((46,65),'Kleine Tiere. Klare Unterschiede.',font=b.font(40,True),fill='#E5E4D4')
    d.text((48,122),'Vierbild-Zyklen und zwei Fellvarianten · Hier Variante 0, Bild 0 · Licht von rechts oben' if animated else 'Fressbilder und Ruhe · Geh- und Laufposen als Entwurf · Beide Richtungen mit Licht von rechts oben',font=b.font(20),fill='#B5C1BE')
    names={'deer':'Hirsch','deer_young':'Kalb','wolf':'Wolf','fox':'Fuchs','rabbit':'Kaninchen'}
    states=[('graze','Stehen / fressen'),('walk','Gehen · Pose'),('run','Laufen · Pose'),('rest','Ruhen')]
    for j,(state,title) in enumerate(states):d.text((260+j*370,182),title,font=b.font(22,True),fill='#C8D0BA')
    for i,species in enumerate(SIZES):
        top=224+i*163
        d.rounded_rectangle((34,top,1764,top+148),radius=10,fill='#14283A')
        d.text((52,top+20),names[species],font=b.font(23,True),fill='#E5E4D4')
        w,h=SIZES[species]
        d.text((52,top+54),f'{w} × {h} px',font=b.font(18),fill='#9CAFA9')
        d.text((52,top+83),'Ansicht: 10×',font=b.font(16),fill='#9CAFA9')
        for j,(state,title) in enumerate(states):
            for k,direction in enumerate(['l','r']):
                x=260+j*370+k*160
                sprite=sprites[(species,state,direction,0)].resize((w*10,h*10),Image.Resampling.NEAREST)
                im.paste(sprite,(x,top+112-h*10),sprite)
                d.text((x,top+121),'links' if direction=='l' else 'rechts',font=b.font(15),fill='#9CAFA9')
    d.text((48,1068),'IM GELÄNDE · 4× vergrößert, Jungbaum als Größenvergleich',font=b.font(23,True),fill='#E5E4D4')
    for j,(kind,label) in enumerate([('grass','Gras'),('shrub','Buschboden'),('forest','Waldboden')]):
        strip=Image.new('RGBA',(140,64))
        for x in (4,68):strip.alpha_composite(panel_background(kind),(x,22))
        for k,species in enumerate(SIZES):
            x=17+k*21
            sp=sprites[(species,'graze','r',0)]
            strip.alpha_composite(sp,(x-sp.width//2,40-sp.height+1))
        young=Image.open(b.ROOT/'assets/nature/vegetation/tree_young_0_normal.png')
        strip.alpha_composite(young,(119,37-young.height))
        view=strip.resize((560,256),Image.Resampling.NEAREST)
        im.paste(view,(46+j*580,1120),view)
        d.text((48+j*580,1380),label,font=b.font(20,True),fill='#BCC6B2')
    d.text((48,1440),'SILHOUETTEN · 50 % der Spielgröße, zur Prüfung 10× dargestellt',font=b.font(20,True),fill='#BCC6B2')
    for i,species in enumerate(SIZES):
        sp=sprites[(species,'graze','r',0)]
        mask=sp.getchannel('A').point(lambda v:255 if v==255 else 0)
        small=mask.resize((max(1,round(sp.width/2)),max(1,round(sp.height/2))),Image.Resampling.NEAREST)
        sil=Image.new('RGBA',small.size,'#D2D6C2');sil.putalpha(small)
        sil=sil.resize((sil.width*10,sil.height*10),Image.Resampling.NEAREST)
        x=55+i*340;im.paste(sil,(x,1540-sil.height),sil)
        d.text((x,1555),names[species],font=b.font(18),fill='#AABAB5')
    d.text((48,1632),'Entwurf ohne Spielintegration · Kaninchen bei 3 × 3 px nur als Kleinform lesbar · '+('Bewegung in den GIF-Prüfblättern' if animated else 'Laufzyklen folgen nach Formabnahme'),font=b.font(18),fill='#AABAB5')
    im.save(b.ROOT/'assets/nature/_animals-testsheet.png',icc_profile=b.PROFILE)

def main():
    OUT.mkdir(parents=True,exist_ok=True)
    sprites={};assets=[];report=[]
    for species,size in SIZES.items():
        for state in ['graze','walk','run','rest']:
            frames=2 if state=='graze' else 1
            files=[]
            for direction in ['l','r']:
                for frame in range(frames):
                    im=make_sprite(species,direction,state,frame)
                    filename=f'{species}_0_{state}_{direction}_f{frame}.png'
                    im.save(OUT/filename,icc_profile=b.PROFILE)
                    files.append(filename);sprites[(species,state,direction,frame)]=im
                    a=np.array(im);opaque=a[a[:,:,3]==255][:,:3]
                    colors={tuple(p[:3]) for p in a.reshape(-1,4) if p[3]}
                    assert len(colors)<=6 and im.size==size
                    assert set(np.unique(a[:,:,3]))<={0,77,255}
                    # Numerische Kontrastprobe am selben Waldboden-Ausschnitt.
                    bg=np.array(panel_background('forest'))[12:20,28:37,:3].mean(axis=(0,1))
                    weights=np.array([.2126,.7152,.0722])
                    background_luma=float(bg@weights)
                    contrast=float(np.mean(np.abs(opaque@weights-background_luma))/background_luma)
                    assert contrast>=.15,(filename,contrast)
                    report.append(dict(file=filename,size=list(size),colors=len(colors),alpha_ok=True,srgb=True,
                                       forest_mean_absolute_luma_contrast=round(contrast,3),
                                       silhouette_50_percent='requires_visual_review'))
            assets.append(dict(id=species,group='animals',size=list(size),anchor=[size[0]//2,size[1]-1],variants=1,
                               state=state,directions=['l','r'],frames=frames,frame_ms=600 if state=='graze' else None,
                               status='pose_only' if state in ['walk','run'] else 'prototype',files=files))
    manifest=dict(version=1,status='form_proof_pending_review',base_tile={'width':64,'height':32},
                  missing=['variant_1','walk_frames_1_to_3','run_frames_1_to_3','carcasses','tracks'],assets=assets)
    (OUT/'manifest.json').write_text(json.dumps(manifest,indent=2,ensure_ascii=False),encoding='utf-8')
    (OUT/'validation.json').write_text(json.dumps(report,indent=2),encoding='utf-8')
    sheet(sprites)
    frames=[]
    for f in range(2):
        preview=Image.new('RGB',(1000,280),b.rgb('0C1A34'));d=ImageDraw.Draw(preview)
        d.text((24,16),'Fresszustand · 2 Bilder / 1,2 Sekunden · 12×',font=b.font(22,True),fill='#E5E4D4')
        for i,species in enumerate(SIZES):
            sp=sprites[(species,'graze','r',f)]
            sp=sp.resize((sp.width*12,sp.height*12),Image.Resampling.NEAREST)
            preview.paste(sp,(30+i*195,195-sp.height),sp)
            d.text((30+i*195,221),species,font=b.font(18),fill='#AABAB5')
        frames.append(preview)
    frames[0].save(b.ROOT/'assets/nature/_animals-graze.gif',save_all=True,append_images=frames[1:],duration=600,loop=0,disposal=2)
    print(len(report),'Tierbilder mit exakten Maßen, harter Alpha-Kante und maximal sechs Farben geprüft.')

if __name__=='__main__':main()
