"""Sechs Biomentwürfe mit expliziten Testwerten, keine Änderung der Simulation."""
import json
from pathlib import Path
import numpy as np
from PIL import Image, ImageDraw
import build_proof as b

ROOT=b.ROOT
OUT=ROOT/'assets/nature/biomes'
SOURCES=ROOT/'art/nature/comfyui-biomes'
BASE=ROOT/'assets/nature'
POINTS=[(-6,-5),(8,-4),(-15,1),(0,2),(15,2),(-6,7)]
PROFILES=[
    ('forest','01  Feuchter Wald','Dunkler Waldboden · geschlossene Kronen','Viel Wald, hohe Bodenfeuchte, wenig Gras',dict(moisture=.85,grass=.25,shrub=.2,forest=.88,age=75,elevation=35)),
    ('meadow','02  Wiesenland','Offene grüne Flächen · kleine Baumgruppen','Viel Gras, mittlere Feuchte, lockerer Wald',dict(moisture=.6,grass=.9,shrub=.1,forest=.13,age=35,elevation=20)),
    ('scrub','03  Trockenes Buschland','Olivgrüne Büsche · warmer offener Boden','Viel Busch, wenig Wald, geringe Feuchte',dict(moisture=.25,grass=.3,shrub=.82,forest=.07,age=25,elevation=45)),
    ('steppe','04  Steppe','Strohfarbenes Gras · weite baumarme Flächen','Trocken, Graslücken, vereinzelte Büsche',dict(moisture=.12,grass=.62,shrub=.15,forest=0,age=0,elevation=25)),
    ('wetland','05  Feuchtgebiet','Dunkle Ufer · offene Wasserflächen','Nasser Boden, Senkenwasser, Wald am Rand',dict(moisture=.96,grass=.72,shrub=.28,forest=.28,age=45,elevation=8)),
    ('highland','06  Felsiges Bergland','Graue Schichten · karger gestufter Bewuchs','Hoch gelegen, dünner Boden, kaum Bäume',dict(moisture=.32,grass=.15,shrub=.08,forest=0,age=0,elevation=250)),
]
IMAGES={}

def original(group,name):
    key=(group,name)
    if key not in IMAGES:
        im=Image.open(BASE/group/(name+'.png')).convert('RGBA')
        manifest=json.loads((BASE/'manifest.json').read_text(encoding='utf-8'))
        relative=f'{group}/{name}.png'
        entry=next((a for a in manifest['assets'] if relative in a['files']),None)
        if entry and 'display_size' in entry:
            im=im.resize(tuple(entry['display_size']),Image.Resampling.NEAREST)
        IMAGES[key]=im
    return IMAGES[key]

def material(name,color):
    src=Image.open(SOURCES/(name+'.png')).convert('L').resize((32,32),Image.Resampling.BOX)
    a=np.array(src)
    idx=np.searchsorted(np.quantile(a,[.12,.3,.56,.8,.94]),a)
    colors=b.palette(color,(.84,.93,1,1.06,1.12,1.18))
    square=np.array(colors,dtype=np.uint8)[idx]
    square[0]=square[-1]=b.rgb(color)
    square[:,0]=square[:,-1]=b.rgb(color)
    im=Image.new('RGBA',(64,32))
    mask=b.diamond_mask()
    for y in range(32):
        for x in range(64):
            if mask.getpixel((x,y)):
                u=min(31,max(0,int((x-32)/2+y)))
                v=min(31,max(0,int(y-(x-32)/2)))
                im.putpixel((x,y),tuple(square[v,u])+(255,))
    im.save(OUT/(name+'_0.png'),icc_profile=b.PROFILE)
    return im

def layer(canvas,im,cx,cy,opacity=1):
    im=im.copy()
    if opacity<1: im.putalpha(im.getchannel('A').point(lambda v:round(v*opacity)))
    canvas.alpha_composite(im,(cx-32,cy-16))

def count_forest(f):
    if f<.05:return 0
    if f<.2:return 1
    if f<.4:return 2
    if f<.6:return 3
    if f<.8:return 5
    return 6

def count_shrub(s):
    if s<.1:return 0
    if s<.3:return 1
    if s<.6:return 2
    if s<.8:return 3
    return 4

def scene(name,profile,materials):
    im=Image.new('RGBA',(280,180))
    d=ImageDraw.Draw(im)
    cells=[]
    for s in range(7):
        for y in range(4):
            x=s-y
            if not 0<=x<4:continue
            t=dict(profile)
            noise=((x*37+y*53+19)%17)/16
            t['forest']=max(0,min(1,t['forest']+(noise-.5)*.22)) if t['forest'] else 0
            t['shrub']=max(0,min(1,t['shrub']+(noise-.5)*.15))
            t['grass']=max(0,min(1,t['grass']+(noise-.5)*.16))
            t['surface_water']=0
            t['flow']=0
            if name=='wetland' and (x,y) in [(1,1),(2,1),(1,2),(2,2),(3,2)]:
                t.update(surface_water=230,forest=0,shrub=0,grass=0)
            if name=='meadow' and x==0:t['flow']=70
            height=6
            if name=='highland':height=6+((3-x)+(3-y))*4
            cx=140+(x-y)*32
            cy=51+(x+y)*16-height
            base=materials['dry_soil'] if name in ['scrub','steppe'] else materials['wet_soil'] if name in ['forest','wetland'] else original('ground','ground_soil_0')
            wall_color='967E58'
            if name=='highland':
                base=materials['highland_rock'];wall_color='8A847C'
            d.polygon([(cx-32,cy),(cx,cy+16),(cx,cy+16+height),(cx-32,cy+height)],fill=tuple(int(c*.56) for c in b.rgb(wall_color)))
            d.polygon([(cx,cy+16),(cx+32,cy),(cx+32,cy+height),(cx,cy+16+height)],fill=tuple(int(c*.72) for c in b.rgb(wall_color)))
            layer(im,base,cx,cy)
            if t['surface_water']>150:
                layer(im,original('water','water_pond_0'),cx,cy)
            else:
                if name=='forest':layer(im,original('ground','ground_forest_floor_0'),cx,cy,.7)
                grass='ground_grass_dry_0' if t['moisture']<.35 else 'ground_grass_wet_0'
                layer(im,original('ground',grass),cx,cy,t['grass'])
                if t['flow']>=60:
                    d.line([(cx+16,cy-8),(cx,cy),(cx-16,cy+8)],fill=b.rgb('4E96E6'),width=3)
                sprites=[]
                for dx,dy in POINTS[:count_shrub(t['shrub'])]:
                    sprites.append((dy,dx,original('vegetation','shrub_0_normal')))
                age='young' if t['age']<15 else 'mid' if t['age']<60 else 'old'
                for dx,dy in POINTS[:count_forest(t['forest'])]:
                    sprites.append((dy,dx,original('vegetation','tree_'+age+'_0_normal')))
                for dy,dx,sprite in sorted(sprites,key=lambda v:v[0]):
                    im.alpha_composite(sprite,(cx+dx-sprite.width//2,cy+dy-sprite.height+1))
            cells.append(dict(x=x,y=y,**t))
    im.save(OUT/(name+'.png'),icc_profile=b.PROFILE)
    (OUT/(name+'.json')).write_text(json.dumps(cells,indent=2),encoding='utf-8')
    return im

def main():
    OUT.mkdir(parents=True,exist_ok=True)
    materials={name:material(name,color) for name,color in [('dry_soil','AA895A'),('wet_soil','675E42'),('highland_rock','8A847C')]}
    sheet=Image.new('RGB',(1920,1210),b.rgb('0C1A34'))
    d=ImageDraw.Draw(sheet)
    d.text((54,30),'NATURE IN A GLASS  /  BIOME',font=b.font(20,True),fill='#91A99D')
    d.text((52,67),'Sechs Landschaften. Eine gemeinsame Bildsprache.',font=b.font(36,True),fill='#E5E4D4')
    d.text((54,119),'Richtung Siedler I & II  ·  Materialvorlagen aus lokalem ComfyUI  ·  Gestaltungsentwürfe',font=b.font(20),fill='#ACBAB7')
    for i,(name,title,caption,conditions,profile) in enumerate(PROFILES):
        x=42+(i%3)*626;y=176+(i//3)*484
        d.rounded_rectangle((x,y,x+606,y+462),radius=14,fill='#14283A')
        d.text((x+22,y+17),title,font=b.font(25,True),fill='#E5E4D4')
        patch=scene(name,profile,materials).resize((560,360),Image.Resampling.NEAREST)
        sheet.paste(patch,(x+23,y+51),patch)
        d.text((x+22,y+405),caption,font=b.font(18),fill='#C2C7AC')
        d.text((x+22,y+432),conditions,font=b.font(16),fill='#95AAA9')
    d.text((54,1164),'Prüfszenen mit festen Testwerten · 64 × 32 px je Feld · Noch keine Biome-Zuordnung im Spiel',font=b.font(18),fill='#A4B6BB')
    sheet.save(BASE/'_biomes-testsheet.png',icc_profile=b.PROFILE)
    report=[]
    for name in materials:
        p=OUT/(name+'_0.png');im=Image.open(p);a=np.array(im)
        colors={tuple(v[:3]) for v in a.reshape(-1,4) if v[3]}
        assert im.size==(64,32) and im.mode=='RGBA' and len(colors)<=6
        assert np.all(a[:,:,3][np.array(b.diamond_mask())==0]==0)
        report.append(dict(file=p.name,size=list(im.size),colors=len(colors),srgb=bool(im.info.get('icc_profile')),rgba=True))
    (OUT/'manifest.json').write_text(json.dumps(dict(version=1,status='biome_concept_pending_review',base_tile=[64,32],materials=report,profiles=[dict(id=n,title=t,values=p) for n,t,c,co,p in PROFILES]),indent=2,ensure_ascii=False),encoding='utf-8')
    print('Sechs Landschaftsentwürfe und drei geprüfte Material-Prototypen erstellt.')

if __name__=='__main__':main()
