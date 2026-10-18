"""Erwachsene Menschen als vorbereitende Pixelentwürfe, ohne Spielintegration."""
import json
import numpy as np
from PIL import Image, ImageDraw
import build_proof as b

OUT=b.OUT/'humans'
SIZE=(16,24)
PALETTES=[['40382F','C49A72','526854','91A17C','806447'],['40362F','C49A72','795649','B88D67','756349']]


def sprite(sex,variant,direction,state,frame):
    mask=Image.new('L',SIZE);d=ImageDraw.Draw(mask)
    def p(x,y):return (15-x if direction=='l' else x,y)
    def poly(points,c):d.polygon([p(x,y) for x,y in points],fill=c)
    def line(points,c,width=1):d.line([p(x,y) for x,y in points],fill=c,width=width)
    def box(rect,c):
        x0,y0,x1,y1=rect
        if direction=='l':x0,x1=15-x1,15-x0
        d.rectangle((x0,y0,x1,y1),fill=c)
    moving=state in ('walk','carry')
    feet=[(-2,2),(0,1),(2,-2),(1,0)][frame] if moving else (0,0)
    # Beide Grundfiguren nutzen dieselben Tätigkeiten, Größen und Werkzeuge.
    line([(6,15),(6,19),(6+feet[0],22)],1,2)
    line([(9,15),(9,19),(9+feet[1],22)],5,2)
    line([(6+feet[0],22),(7+feet[0],22)],1)
    line([(9+feet[1],22),(10+feet[1],22)],1)
    shoulder=4 if sex=='male' else 5
    poly([(shoulder,9),(9,8),(11,11),(10,16),(5,16),(4,12)],3)
    box((5,15,10,16),5)
    # Kopf in Dreiviertelansicht, mit seitlicher Nase und kleinem Kinn.
    poly([(6,2),(10,2),(11,4),(12,5),(11,6),(10,8),(7,8),(6,6)],2)
    poly([(6,2),(8,1),(10,2),(11,3),(8,3),(7,5),(6,5)],1)
    if sex=='female':
        poly([(5,3),(6,3),(6,8),(4,10),(3,8),(4,6)],1)
    else:line([(7,7),(9,8)],1)
    # Ein Auge; die andere Gesichtshälfte ist durch Perspektive verdeckt.
    line([(10,4),(10,4)],1)
    if state=='carry':
        box((10,11,14,16),5)
        line([(11,11),(14,11)],4)
        line([(10,10),(12,13),(13,13)],2,2)
        line([(5,10),(7,13),(10,14)],3,2)
        line([(10,14),(11,14)],2)
    elif state=='work':
        # Neutrale beidhändige Werkzeugbewegung; keine Berufs-/Simulationsregel.
        hand=[(11,8),(13,11),(12,15),(10,12)][frame]
        line([(5,10),(7,12),hand],3,2)
        line([(10,10),hand],2,2)
        end=[(13,3),(15,7),(15,18),(13,16)][frame]
        line([hand,end],5)
        line([(max(0,end[0]-2),end[1]),end],1,2)
    else:
        swing=[2,0,-2,-1][frame] if moving else 0
        line([(5,10),(4,13),(5-swing,16)],3,2)
        line([(5-swing,16)]*2,2)
        line([(10,10),(11,13),(10+swing,16)],3,2)
        line([(10+swing,16)]*2,2)
    tags=np.array(mask);rgba=np.zeros((24,16,4),dtype=np.uint8)
    palette=[b.rgb(c) for c in PALETTES[variant]]
    for y,x in zip(*np.where(tags>0)):
        if x>0 and y+1<24 and tags[y+1,x-1]==0:rgba[y+1,x-1]=(12,26,52,77)
    for y,x in zip(*np.where(tags>0)):
        tone=int(tags[y,x])-1
        if tone==2 and x>=8 and (y<13 or tags[y-1,x]==0):tone=3
        rgba[y,x]=palette[tone]+(255,)
    return Image.fromarray(rgba)


def main():
    OUT.mkdir(parents=True,exist_ok=True)
    assets=[];images={};canonical={};aliases={};checks=[]
    for sex in ('male','female'):
        for state,count,ms in [('stand',1,None),('walk',4,200),('carry',4,200),('work',4,300)]:
            mapping=[]
            for variant in (0,1):
                for direction in ('l','r'):
                    frames=[]
                    for frame in range(count):
                        im=sprite(sex,variant,direction,state,frame)
                        name=f'human_{sex}_{variant}_{state}_{direction}_f{frame}.png'
                        identity=im.tobytes();file=canonical.get(identity)
                        if file is None:
                            file=name;canonical[identity]=file
                            im.save(OUT/file,icc_profile=b.PROFILE)
                        else:aliases[name]=file
                        images[(sex,variant,direction,state,frame)]=im;frames.append(im)
                        mapping.append(dict(variant=variant,direction=direction,frame=frame,file=file))
                        pixels=np.array(im)
                        colors={tuple(p[:3]) for p in pixels.reshape(-1,4) if p[3]}
                        assert len(colors)<=6 and im.size==SIZE
                        assert set(np.unique(pixels[:,:,3]))<={0,77,255}
                        checks.append(dict(frame_id=name,file=file,size=list(SIZE),colors=len(colors)))
                    if count==4:assert len({im.tobytes() for im in frames})==4
            assets.append(dict(id='human_'+sex,size=list(SIZE),display_size=[8,12],anchor=[8,22],variants=2,state=state,frames=count,frame_ms=ms,directions=['l','r'],files=list(dict.fromkeys(m['file'] for m in mapping)),frame_map=mapping))
    manifest=dict(version=1,status='preproduction_not_integrated',resolution_scale=2,assets=assets,frame_aliases=aliases)
    (OUT/'manifest.json').write_text(json.dumps(manifest,indent=2),encoding='utf-8')
    (OUT/'validation.json').write_text(json.dumps(checks,indent=2),encoding='utf-8')
    sheets=[]
    for tick in range(24):
        board=Image.new('RGB',(1450,840),b.rgb('0C1A34'));d=ImageDraw.Draw(board)
        d.text((30,22),'MENSCHEN / ERWACHSENE GRUNDFIGUREN',font=b.font(28,True),fill='#E5E4D4')
        d.text((30,65),'Männlich und weiblich · zwei Kleidungsvarianten · Darstellung hier 5× · Vorbereitung, noch kein Spielinhalt',font=b.font(18),fill='#B9C6BB')
        cols=[('stand','Stehen'),('walk','Gehen · 0,8 s'),('carry','Tragen · 0,8 s'),('work','Arbeiten · 1,2 s')]
        for c,(_,title) in enumerate(cols):d.text((200+c*310,115),title,font=b.font(21,True),fill='#DADAC4')
        for row,(sex,variant) in enumerate([(s,v) for s in ('male','female') for v in (0,1)]):
            y=180+row*155
            d.text((25,y+35),('Mann' if sex=='male' else 'Frau')+f' · {variant+1}',font=b.font(22),fill='#DADAC4')
            for c,(state,_) in enumerate(cols):
                frame=0 if state=='stand' else (tick//(3 if state=='work' else 2))%4
                for j,direction in enumerate(('l','r')):
                    im=images[(sex,variant,direction,state,frame)].resize((80,120),Image.Resampling.NEAREST)
                    board.paste(im,(205+c*310+j*135,y),im)
        sheets.append(board)
    sheets[0].save(b.OUT/'_humans-prototypes.png',icc_profile=b.PROFILE)
    sheets[0].save(b.OUT/'_humans-motion.gif',save_all=True,append_images=sheets[1:],duration=100,loop=0,disposal=2)
    # Größenvergleich bei Zoom 8: doppelt aufgelöste Assets in nativer Pixelgröße.
    zoom=Image.new('RGB',(1050,330),b.rgb('0C1A34'));d=ImageDraw.Draw(zoom)
    d.text((25,20),'MASSSTAB / ZOOM 8 UND VERGRÖSSERTE KONTROLLE',font=b.font(23,True),fill='#E5E4D4')
    examples=[('Mann',images[('male',0,'r','stand',0)]),('Frau',images[('female',0,'r','stand',0)]),('Hirsch',Image.open(b.OUT/'animals/deer_0_graze_r_f0.png').convert('RGBA')),('Jungbaum',Image.open(b.OUT/'vegetation/tree_young_0_normal.png').convert('RGBA'))]
    for i,(title,im) in enumerate(examples):
        x=45+i*250
        zoom.paste(im,(x,115-im.height),im)
        big=im.resize((im.width*4,im.height*4),Image.Resampling.NEAREST)
        zoom.paste(big,(x+70,235-big.height),big)
        d.text((x,270),title,font=b.font(21),fill='#DADAC4')
    zoom.save(b.OUT/'_humans-scale.png',icc_profile=b.PROFILE)
    gif=Image.open(b.OUT/'_humans-motion.gif');duration=0
    for i in range(gif.n_frames):
        gif.seek(i);assert gif.info['duration']%100==0
        duration+=gif.info['duration']
    assert duration==2400  # GIF fasst unveränderte Zwischenbilder zusammen.
    print(len(checks),'Bildzuordnungen,',len(canonical),'eindeutige PNGs; Maße, Farben, Alpha, Bewegungsphasen und GIF-Zeiten geprüft.')


if __name__=='__main__':main()
