"""Rastert ComfyUI-Vorlagen ein und baut das Prüfblatt; keine Spielintegration."""
import json
import math
from pathlib import Path

import numpy as np
from PIL import Image, ImageDraw, ImageFont, ImageCms

ROOT = Path(__file__).resolve().parents[2]
SRC = ROOT / 'art/nature/comfyui'
OUT = ROOT / 'assets/nature'
# Vorhandenes sRGB-Profil wiederverwenden: ein neu erzeugtes ICC-Profil trägt
# einen Zeitstempel und würde sonst unveränderte PNGs bei jedem Build ändern.
_profile_source = OUT / 'ground/ground_soil_0.png'
PROFILE = (Image.open(_profile_source).info.get('icc_profile') if _profile_source.exists() else None)
if not PROFILE:
    PROFILE = ImageCms.ImageCmsProfile(ImageCms.createProfile('sRGB')).tobytes()
ANCHORS = {'soil':'967E58','sand':'DED09A','rock':'8A847C','grass_wet':'68A444',
           'grass_dry':'BAAC5C','burnt':'302A26','shrub_floor':'76823C',
           'forest_floor':'7A6854','shallow':'4A96C4','deep':'122E68',
           'pond':'3A76BA','stream':'4E96E6'}
ASSETS = []
IMAGES = {}
SHEET_TITLE='Eine kleine Welt. Lesbare Natur.'
SHEET_SUBTITLE='Gestaltungs-Prüfblatt 01  /  Lokales ComfyUI · SDXL  /  Entwurf zur Abnahme'
HOLE_MODULUS=19

def rgb(h):
    return tuple(bytes.fromhex(h))

def palette(h, factors=(.72,.86,1,1.10,1.19,1.28)):
    return [tuple(min(244,round(c*f)) for c in rgb(h)) for f in factors]

def save(name, group, im, anchor, **extra):
    path = OUT / group / (name + '.png')
    path.parent.mkdir(parents=True,exist_ok=True)
    im.save(path, icc_profile=PROFILE)
    IMAGES[name] = im
    ASSETS.append(dict(id=name.rsplit('_0',1)[0], group=group, size=list(im.size),
                       anchor=anchor, variants=1, states=['normal'], frames=1,
                       files=[str(path.relative_to(OUT)).replace('\\','/')], **extra))

def texture(source, size, color):
    im = Image.open(SRC / (source+'.png')).convert('L').resize(size,Image.Resampling.BOX)
    a = np.array(im)
    levels = np.quantile(a,[.08,.24,.60,.85,.96])
    idx = np.searchsorted(levels,a)
    tones=palette(color,(.94,.97,1,1.025,1.05,1.075))
    return Image.fromarray(np.array(tones,dtype=np.uint8)[idx], 'RGB').convert('RGBA')

def diamond_mask():
    a = np.zeros((32,64),dtype=np.uint8)
    for y in range(32):
        for x in range(64):
            if abs((x+.5-32)/32)+abs((y+.5-16)/16)<=1:
                a[y,x]=255
    return Image.fromarray(a)

def build_tiles():
    mask = diamond_mask()
    for name,source in [('soil','soil'),('sand','soil'),('rock','rock'),('grass_wet','grass'),
                        ('grass_dry','grass'),('burnt','burnt'),('shrub_floor','grass'),('forest_floor','soil')]:
        # Periodisches Material ohne wechselnde Außenkante; die Randfarbe ist überall identisch.
        square = texture(source,(32,32),ANCHORS[name])
        data = np.array(square)
        data[0,:,:3] = data[-1,:,:3] = rgb(ANCHORS[name])
        data[:,0,:3] = data[:,-1,:3] = rgb(ANCHORS[name])
        tile = Image.new('RGBA',(64,32))
        for y in range(32):
            for x in range(64):
                if mask.getpixel((x,y)):
                    u = (x-32)/2+y
                    v = y-(x-32)/2
                    c = tuple(data[min(31,max(0,int(v))),min(31,max(0,int(u)))])
                    if name in ('grass_wet','grass_dry','burnt','shrub_floor','forest_floor'):
                        # Deterministische kleine Lücken, keine Halbtransparenz.
                        cutoff = 3 if name=='grass_dry' else 1
                        if (x*17+y*29+x*y*3)%HOLE_MODULUS < cutoff:
                            c = (0,0,0,0)
                    tile.putpixel((x,y),c)
        save('ground_'+name+'_0','ground',tile,[32,16])
    for name,source in [('soil','soil'),('sand','soil'),('rock','rock')]:
        wall=texture(source,(32,32),ANCHORS[name])
        a=np.array(wall)
        a[-1]=a[0]
        save('wall_'+name+'_0','walls',Image.fromarray(a),[16,16])
    for name in ('shallow','deep','pond'):
        im=texture('water',(64,32),ANCHORS[name])
        im.putalpha(mask)
        save('water_'+name+'_0','water',im,[32,16],preview_static=True)
    im=texture('water',(16,32),ANCHORS['stream'])
    save('water_stream_0','water',im,[8,16],preview_static=True)

def sprite(name, size, color):
    src=Image.open(SRC/(name+'.png')).convert('RGB')
    # Nach Sichtprüfung ausgewählte Einzelmotive; SDXL lieferte teils Motivbögen.
    crops={'tree_young':(48,20,214,277),'tree_mid':(20,10,744,650),
           'tree_old':(244,241,437,443),'shrub':(113,545,320,622),
           'snag_standing':(100,30,710,650)}
    src=src.crop(crops[name])
    a=np.array(src)
    # Weißer Hintergrund der KI-Vorlage; graue Bodenschatten werden ausgeschlossen.
    chroma=a.max(axis=2).astype(int)-a.min(axis=2).astype(int)
    keep=(a.min(axis=2)<180)&(chroma>18)
    if name=='tree_young':
        keep[215:,:47]=False
        keep[215:,117:]=False
    if name=='tree_mid':
        keep[540:,:250]=False
        keep[540:,490:]=False
    if name=='shrub':
        keep &= a[:,:,1]>a[:,:,0]*.94
    if name=='tree_old':
        isolate=Image.new('L',src.size)
        outline=[(330,242),(380,259),(408,280),(436,320),(429,357),
                 (395,390),(365,405),(364,443),(327,443),(326,405),
                 (285,388),(244,353),(247,314),(286,270)]
        ImageDraw.Draw(isolate).polygon([(x-244,y-241) for x,y in outline],fill=255)
        keep &= np.array(isolate)>0
    if name=='snag_standing':
        keep=(a.min(axis=2)<155)&(chroma>7)
    ys,xs=np.where(keep)
    box=(int(xs.min()),int(ys.min()),int(xs.max()+1),int(ys.max()+1))
    mask=Image.fromarray((keep*255).astype('uint8')).crop(box)
    src=src.crop(box)
    w,h=size
    # Zwei Pixel unten bleiben für den im Sprite eingebetteten Schlagschatten.
    src=src.resize((w-2,h-2),Image.Resampling.BOX)
    mask=mask.resize(src.size,Image.Resampling.BOX).point(lambda v:255 if v>=135 else 0)
    a=np.array(src)
    lum=np.mean(a,axis=2)
    valid=lum[np.array(mask)>0]
    thresholds=np.quantile(valid,[.22,.52,.8])
    p=palette(color,(.63,.82,1,1.17))
    out=Image.new('RGBA',size)
    d=ImageDraw.Draw(out)
    d.ellipse((0,h-3,w//2+1,h-1), fill=rgb('0C1A34')+(77,))
    idx=np.searchsorted(thresholds,lum)
    for y in range(h-2):
        for x in range(w-2):
            if mask.getpixel((x,y)):
                c=p[idx[y,x]]
                if name!='snag_standing' and a[y,x,0]>a[y,x,1]*1.03:
                    c=rgb('7A6854')
                out.putpixel((x+1,y),c+(255,))
    # Expliziter Stammfuß; Lage und Maße ändern sich beim Zeichnen nie.
    d=ImageDraw.Draw(out)
    d.line((w//2,h-4,w//2,h-1),fill=rgb('7A6854')+(255,),width=1)
    if name=='tree_old':
        d.line((w//2,h-9,w//2,h-1),fill=rgb('7A6854')+(255,),width=1)
    if name=='snag_standing':
        d.line((w//2-1,3,w//2-1,h-1),fill=rgb('7A6854')+(255,),width=2)
    save(name+'_0_normal','vegetation',out,[w//2,h-1])

def paste_at(canvas,name,x,y):
    im=IMAGES[name]
    asset=next(a for a in ASSETS if a['files'][0].endswith('/'+name+'.png'))
    ax,ay=asset['anchor']
    canvas.alpha_composite(im,(int(x-ax),int(y-ay)))

def build_scene():
    scene=Image.new('RGBA',(560,340))
    d=ImageDraw.Draw(scene)
    layout=[]
    # Stabile Abstände >= 6 px, alle Fußpunkte mindestens 4 px vom Rautenrand.
    points=[(-6,-5),(8,-4),(-15,1),(0,2),(15,2),(-6,7)]
    for s in range(15):
        for y in range(8):
            x=s-y
            if not 0<=x<8: continue
            elev=0 if x in (0,7) or y in (0,7) else 6
            if y==1 and x in (2,3,4): elev=12
            cx=280+(x-y)*32
            cy=43+(x+y)*16-elev
            tile='grass_wet'
            trees=0
            age=None
            if elev==0: tile='shallow'
            elif (x,y) in ((5,5),(5,6)): tile='pond'
            elif x==1 or y==6: tile='sand'
            elif y==1 and x in (2,3,4): tile='rock'
            elif x==6 and y in (2,3): tile='grass_dry'
            elif (x,y)==(6,4): tile='burnt'
            elif (x,y)==(6,5): tile='soil'
            elif (x,y)==(5,6): tile='pond'
            if y in (2,3,4) and x in (2,3,4):
                age={2:'young',3:'mid',4:'old'}[y]
                trees={2:1,3:3,4:6}[x]
            # Seitenwände werden aus denselben Materialvorlagen abgeleitet.
            if elev:
                base=rgb(ANCHORS.get(tile,ANCHORS['soil']))
                d.polygon([(cx-32,cy),(cx,cy+16),(cx,cy+16+elev),(cx-32,cy+elev)],fill=tuple(round(c*.56) for c in base))
                d.polygon([(cx,cy+16),(cx+32,cy),(cx+32,cy+elev),(cx,cy+16+elev)],fill=tuple(round(c*.72) for c in base))
            if tile in ('shallow','pond'):
                paste_at(scene,'water_'+tile+'_0',cx,cy)
            else:
                paste_at(scene,'ground_soil_0',cx,cy)
                paste_at(scene,'ground_'+tile+'_0',cx,cy)
                if trees:
                    floor=IMAGES['ground_forest_floor_0'].copy()
                    floor.putalpha(floor.getchannel('A').point(lambda v: round(v*.25)))
                    scene.alpha_composite(floor,(cx-32,cy-16))
                if (x,y) in ((5,2),(5,3)):
                    floor=IMAGES['ground_shrub_floor_0'].copy()
                    floor.putalpha(floor.getchannel('A').point(lambda v: round(v*.5)))
                    scene.alpha_composite(floor,(cx-32,cy-16))
            # Zusammenhängender Bach durch den westlichen Inselteil.
            if x==1 and 1<=y<=6:
                d.line([(cx+16,cy-8),(cx,cy),(cx-16,cy+8)],fill=rgb('4E96E6'),width=3)
                d.line([(cx+16,cy-8),(cx,cy),(cx-16,cy+8)],fill=rgb('78B3EA'),width=1)
            for dx,dy in sorted(points[:trees],key=lambda p:p[1]):
                paste_at(scene,'tree_'+age+'_0_normal',cx+dx,cy+dy)
            if (x,y) in ((5,2),(5,3)):
                for dx,dy in points[:(2 if y==2 else 4)]:
                    paste_at(scene,'shrub_0_normal',cx+dx,cy+dy)
            if (x,y) in ((6,4),(6,5)):
                for dx,dy in points[:3]:
                    paste_at(scene,'snag_standing_0_normal',cx+dx,cy+dy)
            layout.append(dict(x=x,y=y,tile=tile,trees=trees,age=age))
    (OUT/'proof-layout.json').write_text(json.dumps(layout,indent=2),encoding='utf-8')
    scene.save(OUT/'_island.png',icc_profile=PROFILE)
    return scene

def font(size,bold=False):
    return ImageFont.truetype('C:/Windows/Fonts/'+('segoeuib.ttf' if bold else 'segoeui.ttf'),size)

def sheet(scene):
    im=Image.new('RGB',(1600,1120),rgb('0C1A34'))
    d=ImageDraw.Draw(im)
    d.text((64,35),'NATURE IN A GLASS',font=font(20,True),fill='#8FAAA3')
    d.text((62,66),SHEET_TITLE,font=font(40,True),fill='#E5E4D4')
    d.text((64,123),SHEET_SUBTITLE,font=font(19),fill='#A4B6BB')
    im.paste(scene.resize((1120,680),Image.Resampling.NEAREST),(20,170),scene.resize((1120,680),Image.Resampling.NEAREST))
    d.rounded_rectangle((1160,200,1536,776),radius=16,fill='#15283C')
    d.text((1184,224),'8 × 8 Felder',font=font(25,True),fill='#E5E4D4')
    lines=['64 × 32 px je Raute','Ansicht hier: 2× vergrößert','Licht von oben rechts','', 'Waldmatrix in der Insel:', 'hinten → vorne: jung / mittel / alt','je Reihe: 1 / 3 / 6 Bäume','', 'Bach, Teich und Strand','Feuchtes und trockenes Gras','Busch, Totholz und Brandspur','Fels und gestufte Erdkanten','', 'Prüfszene mit festen Testwerten.','Noch keine Spielintegration.']
    for i,line in enumerate(lines):
        d.text((1184,270+i*30),line,font=font(17),fill='#ABBAB8')
    d.line((64,840,1536,840),fill='#294153',width=1)
    d.text((64,861),'FORM & DICHTE',font=font(16,True),fill='#8FAAA3')
    for k,(name,title) in enumerate([('young','Jungwald'),('mid','Wald'),('old','Altwald')]):
        base=70+k*355
        d.text((base,895),title,font=font(20,True),fill='#E5E4D4')
        for j,count in enumerate([1,3,6]):
            tile=Image.new('RGBA',(72,70))
            paste_at(tile,'ground_soil_0',36,47)
            paste_at(tile,'ground_grass_wet_0',36,47)
            for dx,dy in [(-6,-5),(8,-4),(-15,1),(0,2),(15,2),(-6,7)][:count]:
                paste_at(tile,'tree_'+name+'_0_normal',36+dx,47+dy)
            scaled=tile.resize((108,105),Image.Resampling.NEAREST)
            im.paste(scaled,(base+j*108,928),scaled)
            d.text((base+j*108+40,1040),str(count),font=font(17),fill='#A4B6BB')
    d.text((1184,895),'SILHOUETTEN · 50 %',font=font(16,True),fill='#8FAAA3')
    for k,name in enumerate(['tree_young','tree_mid','tree_old','shrub','snag_standing']):
        a=IMAGES[name+'_0_normal']
        tiny=a.resize((max(1,a.width//2),max(1,a.height//2)),Image.Resampling.NEAREST)
        sil=Image.new('RGBA',tiny.size,'#DADCC7')
        sil.putalpha(tiny.getchannel('A').point(lambda v:255 if v==255 else 0))
        sil=sil.resize((sil.width*3,sil.height*3),Image.Resampling.NEAREST)
        im.paste(sil,(1186+k*62,978-sil.height),sil)
    d.text((1184,1003),'Halbiert, danach 3× zur Prüfung.',font=font(16),fill='#A4B6BB')
    im.save(OUT/'_testsheet.png',icc_profile=PROFILE)

def main():
    OUT.mkdir(parents=True,exist_ok=True)
    build_tiles()
    for name,size,color in [('tree_young',(10,14),'46843A'),('tree_mid',(14,22),'346E32'),
                            ('tree_old',(18,30),'1E502A'),('shrub',(10,8),'76823C'),
                            ('snag_standing',(6,22),'7A6854')]:
        sprite(name,size,color)
    manifest=dict(version=1,status='proof_pending_review',base_tile=dict(width=64,height=32),assets=ASSETS)
    (OUT/'manifest.json').write_text(json.dumps(manifest,indent=2,ensure_ascii=False),encoding='utf-8')
    sheet(build_scene())
    checks=[]
    for asset in ASSETS:
        im=Image.open(OUT/asset['files'][0])
        pixels=np.array(im).reshape(-1,4)
        colors={tuple(p[:3]) for p in pixels if p[3]}
        check=dict(file=asset['files'][0],size=list(im.size),rgba=im.mode=='RGBA',
                   colors=len(colors),palette_ok=len(colors)<=6,srgb=bool(im.info.get('icc_profile')),
                   alpha_values=sorted(int(v) for v in np.unique(pixels[:,3])))
        assert check['palette_ok'] and check['rgba'] and check['srgb'],check
        assert list(im.size)==asset['size']
        checks.append(check)
    (OUT/'validation.json').write_text(json.dumps(checks,indent=2),encoding='utf-8')
    print('Prüfblatt und',len(ASSETS),'Prototyp-Assets erstellt; Maße, RGBA, sRGB und Palette geprüft.')

if __name__=='__main__':
    main()
