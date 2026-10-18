"""Fünf Erwachsene für Abschnitt 17: native 56×120-Pixelkonturen und Tätigkeiten."""
import json
import math
import numpy as np
from PIL import Image, ImageDraw
import build_proof as b

OUT=b.OUT/'people'
LOOKS=[
 dict(name='Mara',sex='female',build=18,hair='braid',colors=['61452F','D0A17B','716548','A69570','A54E3B']),
 dict(name='Tarek',sex='male',build=22,hair='short',colors=['332D29','AA7655','746A54','ABA082','4D7B91']),
 dict(name='Elin',sex='female',build=16,hair='long',colors=['B19A66','D9B38C','685B4B','A18C6A','7B567E']),
 dict(name='Borin',sex='male',build=24,hair='beard',colors=['554538','B98B67','6E7455','A3AA7D','B78E38']),
 dict(name='Nela',sex='female',build=20,hair='bun',colors=['393330','94684D','83654F','B39774','537E68']),
]
STATES=[('idle',2,800),('walk',4,200),('run',4,100),('gather',4,300),('eat',2,600),('drink',2,600),('sleep',1,None),('hurt',4,250)]


def person(appearance,state,direction,frame):
    look=LOOKS[appearance];w,h=56,120
    labels=Image.new('L',(w,h));d=ImageDraw.Draw(labels)
    def point(p):return (55-p[0] if direction=='l' else p[0],p[1])
    def poly(points,tone):d.polygon([point(p) for p in points],fill=tone)
    def ellipse(box,tone):
        x0,y0,x1,y1=box
        if direction=='l':x0,x1=55-x1,55-x0
        d.ellipse((x0,y0,x1,y1),fill=tone)
    def line(points,tone,width=1):d.line([point(p) for p in points],fill=tone,width=width)
    def limb(a,c,width,tone):
        dx,dy=c[0]-a[0],c[1]-a[1];length=max(1,math.hypot(dx,dy));nx=-dy/length*width/2;ny=dx/length*width/2
        poly([(round(a[0]+nx),round(a[1]+ny)),(round(c[0]+nx),round(c[1]+ny)),(round(c[0]-nx),round(c[1]-ny)),(round(a[0]-nx),round(a[1]-ny))],tone)
        r=max(1,width//2);ellipse((c[0]-r,c[1]-r,c[0]+r,c[1]+r),tone)
    def arm(shoulder,elbow,hand):
        limb(shoulder,elbow,6,3);limb(elbow,hand,4,2)
    hip=(27,72);neck=(29,32);head=(31,22)
    feet=[(21,112),(34,112)];knees=[(22,93),(33,93)]
    hands=[(18,65),(39,65)];elbows=[(16,51),(40,51)]
    if state in ('walk','run','hurt'):
        stride=[(-9,9),(0,-3),(9,-9),(-3,0)][frame]
        speed=1.5 if state=='run' else .55 if state=='hurt' else 1
        feet=[(21+round(stride[0]*speed),112),(34+round(stride[1]*speed),112-(5 if frame%2 else 0))]
        knees=[(21+round(stride[0]*.4),92),(34+round(stride[1]*.4),92)]
        if state=='run':neck=(34,34);head=(36,24)
        hands=[(18-round(stride[0]*.65),64),(38-round(stride[1]*.65),64)]
        if state=='hurt':
            neck=(32,36);head=(33,26);hands[1]=(30,58);elbows[1]=(42,50)
            feet[0]=(feet[0][0],110 if frame==1 else 112)
    elif state=='idle':
        head=(31+frame,22);hands[1]=(39,65-frame)
    elif state=='gather':
        bends=[12,24,32,20];bend=bends[frame]
        hip=(23,77);neck=(31+bend//3,40+bend);head=(36+bend//3,32+bend)
        knees=[(17,96),(35,95)];feet=[(17,112),(41,112)]
        elbows=[(29,72),(44,74+frame*3)];hands=[(32,91),(45,98+frame*2)]
    elif state=='eat':
        hip=(23,85);neck=(28,49);head=(30,39)
        knees=[(13,103),(40,98)];feet=[(23,112),(46,112)]
        elbows=[(15,72),(40,63)];hands=[(25,75),(36,47 if frame else 65)]
    elif state=='drink':
        hip=(23,84);neck=(32,62+frame*3);head=(39,56+frame*3)
        knees=[(18,109),(41,99)];feet=[(6,112),(46,112)]
        elbows=[(30,81),(46,79)];hands=[(38,99 if frame==0 else 67),(47,99 if frame==0 else 70)]
    elif state=='sleep':
        hip=(19,103);neck=(35,102);head=(44,99)
        knees=[(8,111),(17,111)];feet=[(26,113),(35,113)]
        elbows=[(34,110),(44,112)];hands=[(43,110),(49,109)]
    shoulder_width=look['build']
    shoulders=[(neck[0]-shoulder_width//2,neck[1]+9),(neck[0]+shoulder_width//2-2,neck[1]+10)]
    for i in range(2):
        start=(hip[0]+(-4 if i==0 else 4),hip[1])
        limb(start,knees[i],7,1);limb(knees[i],feet[i],5,1)
        line([feet[i],(min(53,feet[i][0]+4),feet[i][1])],1,3)
    arm(shoulders[0],elbows[0],hands[0])
    poly([shoulders[0],(neck[0]-3,neck[1]),(neck[0]+4,neck[1]),shoulders[1],(hip[0]+10,hip[1]+3),(hip[0]-9,hip[1]+3)],3)
    # Kleidungsfalten, feste Erkennungsfarbe an Gürtel und Schulterband.
    line([(hip[0]-9,hip[1]-3),(hip[0]+9,hip[1]-3)],5,4)
    line([(neck[0]-3,neck[1]+4),(hip[0]+5,hip[1]-8)],5,3)
    line([(hip[0]+5,hip[1]-9),(hip[0]+7,hip[1]-1)],4,2)
    limb(neck,(head[0]-2,head[1]+5),6,2)
    hx,hy=head
    ellipse((hx-6,hy-10,hx+6,hy+6),2)
    poly([(hx+4,hy-1),(min(54,hx+9),hy+1),(hx+5,hy+3)],2)
    poly([(hx-6,hy),(hx-7,hy-6),(hx-3,hy-11),(hx+3,hy-10),(hx+6,hy-6),(hx+1,hy-6),(hx-4,hy-3)],1)
    hair=look['hair']
    if hair=='braid':
        line([(hx-6,hy-1),(hx-8,hy+11),(hx-6,hy+21)],1,4)
        line([(hx-8,hy+10),(hx-5,hy+10)],5,2)
    elif hair=='long':poly([(hx-6,hy-6),(hx-10,hy+18),(hx-3,hy+16),(hx-4,hy-1)],1)
    elif hair=='bun':ellipse((hx-12,hy-7,hx-5,hy+1),1)
    elif hair=='beard':poly([(hx-3,hy+4),(hx+5,hy+4),(hx+3,hy+10),(hx-1,hy+9)],1)
    line([(hx+3,hy-2),(hx+3,hy-2)],1)
    arm(shoulders[1],elbows[1],hands[1])
    tags=np.array(labels);rgba=np.zeros((h,w,4),dtype=np.uint8)
    for y,x in zip(*np.where(tags>0)):
        if x>=4 and y+4<h and tags[y+4,x-4]==0:rgba[y+4,x-4]=(12,26,52,77)
    palette=[b.rgb(c) for c in look['colors']]
    for y,x in zip(*np.where(tags>0)):
        tone=int(tags[y,x])-1
        if tone==2 and x>neck[0]+2:tone=3
        rgba[y,x]=palette[tone]+(255,)
    return Image.fromarray(rgba)


def dead(appearance,direction):
    im=Image.new('RGBA',(120,40));d=ImageDraw.Draw(im)
    colors=[tuple(round(sum(b.rgb(c))/3*.65+v*.35) for v in b.rgb(c))+(255,) for c in LOOKS[appearance]['colors']]
    def p(x,y):return (119-x if direction=='l' else x,y)
    d.ellipse((10,27,110,36),fill=(12,26,52,77))
    d.line([p(17,28),p(49,22),p(80,23)],fill=colors[0],width=8)
    d.line([p(15,31),p(44,29),p(77,26)],fill=colors[0],width=6)
    d.polygon([p(51,18),p(82,15),p(94,20),p(84,30),p(53,28)],fill=colors[2])
    d.line([p(62,18),p(63,28)],fill=colors[4],width=3)
    hx=100 if direction=='r' else 19
    d.ellipse((hx-9,13,hx+7,26),fill=colors[1])
    d.line([p(92,16),p(103,14)],fill=colors[0],width=4)
    d.line([p(84,20),p(79,33),p(96,32)],fill=colors[1],width=4)
    return im


def main():
    OUT.mkdir(parents=True,exist_ok=True);unique={};aliases={};assets=[];sprites={};checks=[]
    def save(filename,im):
        identity=(im.size,im.tobytes());target=unique.get(identity)
        if target is None:target=filename;unique[identity]=target;im.save(OUT/target,icc_profile=b.PROFILE)
        else:aliases[filename]=target
        a=np.array(im);colors={tuple(p[:3]) for p in a.reshape(-1,4) if p[3]}
        assert len(colors)<=6 and set(np.unique(a[:,:,3]))<={0,77,255}
        checks.append(dict(frame_id=filename,file=target,size=list(im.size),colors=len(colors)))
        return target
    for state,count,ms in STATES:
        mapping=[]
        for appearance in range(5):
            for direction in ('l','r'):
                frames=[]
                for frame in range(count):
                    im=person(appearance,state,direction,frame);assert im.size==(56,120)
                    file=save(f'person_{appearance}_{state}_{direction}_f{frame}.png',im)
                    mapping.append(dict(appearance=appearance,direction=direction,frame=frame,file=file))
                    sprites[(appearance,state,direction,frame)]=im;frames.append(im)
                assert len({im.tobytes() for im in frames})==count
        assets.append(dict(id='person',state=state,size=[56,120],display_size=[7,15],anchor=[28,112],appearances=5,directions=['l','r'],frames=count,frame_ms=ms,files=list(dict.fromkeys(m['file'] for m in mapping)),frame_map=mapping))
    mapping=[]
    for a in range(5):
        for direction in ('l','r'):
            file=save(f'person_dead_{a}_{direction}_f0.png',dead(a,direction))
            mapping.append(dict(appearance=a,direction=direction,frame=0,file=file))
    assets.append(dict(id='person_dead',size=[120,40],display_size=[15,5],anchor=[60,20],frames=1,files=[m['file'] for m in mapping],frame_map=mapping))
    marker=Image.new('RGBA',(16,16));d=ImageDraw.Draw(marker);d.polygon([(8,1),(14,6),(8,14),(2,6)],fill=(255,255,255,255))
    file=save('person_marker_0.png',marker)
    assets.append(dict(id='person_marker',size=[16,16],display_size=[8,8],anchor=[8,14],frames=1,files=[file],tint='renderer_recognition_color'))
    manifest=dict(version=1,status='section_17_preproduction',base_tile=[64,32],appearances=[dict(id=i,gender=l['sex'],hair=l['hair'],recognition_color='#'+l['colors'][4]) for i,l in enumerate(LOOKS)],assets=assets,frame_aliases=aliases)
    (OUT/'manifest.json').write_text(json.dumps(manifest,indent=2),encoding='utf-8')
    (OUT/'validation.json').write_text(json.dumps(checks,indent=2),encoding='utf-8')
    board=Image.new('RGB',(1600,1080),b.rgb('0C1A34'));draw=ImageDraw.Draw(board)
    draw.text((30,20),'MENSCHEN / FÜNF FESTE AUSSEHEN / ABSCHNITT 17',font=b.font(29,True),fill='#E5E4D4')
    draw.text((30,66),'56 × 120 Pixel · eigene Nahansicht bis Zoom 32 · Vorbereitungsbilder, keine Spielaufnahme',font=b.font(20),fill='#B9C6BB')
    for col in range(5):
        x=35+col*310
        draw.text((x,116),f'Aussehen {col} · '+('Frau' if LOOKS[col]['sex']=='female' else 'Mann'),font=b.font(21,True),fill='#DADAC4')
        for row,(state,frame,label) in enumerate([('idle',0,'Stehen'),('walk',1,'Gehen')]):
            y=160+row*190;draw.text((x,y),label,font=b.font(18),fill='#B9C6BB')
            for j,direction in enumerate(('l','r')):
                im=sprites[(col,state,direction,frame)];board.paste(im,(x+35+j*110,y+32),im)
        for row,zoom in enumerate((4,8,32)):
            y=580+row*145;draw.text((x,y),f'Zoom {zoom}',font=b.font(18),fill='#B9C6BB')
            im=sprites[(col,'idle','r',0)];im=im.resize((round(56*zoom/32),round(120*zoom/32)),Image.Resampling.NEAREST)
            board.paste(im,(x+20,y+30),im)
            for k,path in enumerate(['animals/deer_0_graze_r_f0.png','vegetation/tree_young_0_normal.png']):
                ref=Image.open(b.OUT/path).convert('RGBA');ref=ref.resize((round(ref.width*zoom/8),round(ref.height*zoom/8)),Image.Resampling.NEAREST)
                board.paste(ref,(x+95+k*105,y+30+im.height-ref.height),ref)
    board.save(b.OUT/'_people-testsheet.png',icc_profile=b.PROFILE)
    contact=Image.new('RGB',(1560,920),b.rgb('0C1A34'));draw=ImageDraw.Draw(contact)
    draw.text((25,20),'TÄTIGKEITEN / ZOOM 32 · JEWEILS EINE PHASE',font=b.font(27,True),fill='#E5E4D4')
    labels=['Stehen','Gehen','Laufen','Sammeln','Essen','Trinken','Schlafen','Verletzt']
    for c,((state,n,ms),label) in enumerate(zip(STATES,labels)):
        x=25+c*192;draw.text((x,78),label,font=b.font(20),fill='#DADAC4')
        for a in range(5):
            im=sprites[(a,state,'r',min(2,n-1))];contact.paste(im,(x+45,125+a*155),im)
    contact.save(b.OUT/'_people-states.png',icc_profile=b.PROFILE)
    remains=Image.new('RGB',(900,390),b.rgb('0C1A34'));rd=ImageDraw.Draw(remains)
    rd.text((20,20),'VERSTORBENE UND ÜBERSICHTSZEICHEN',font=b.font(26,True),fill='#E5E4D4')
    for a in range(5):
        x=20+a*175;rd.text((x,75),f'Aussehen {a}',font=b.font(18),fill='#B9C6BB')
        for j,direction in enumerate(('l','r')):
            im=dead(a,direction);remains.paste(im,(x,115+j*70),im)
        tinted=Image.new('RGBA',(16,16),b.rgb(LOOKS[a]['colors'][4])+(255,))
        tinted.putalpha(marker.getchannel('A'));tinted=tinted.resize((48,48),Image.Resampling.NEAREST)
        remains.paste(tinted,(x+35,280),tinted)
    remains.save(b.OUT/'_people-remains.png',icc_profile=b.PROFILE)
    # Jede Tätigkeit hat eine eigene Vorschau mit ihrer tatsächlichen Schleifendauer.
    for state,n,ms in STATES:
        if n==1:continue
        frames=[]
        for f in range(n):
            im=Image.new('RGB',(900,250),b.rgb('0C1A34'));dr=ImageDraw.Draw(im)
            dr.text((20,15),f'{state} · {n} Bilder · {n*ms/1000:g} Sekunden',font=b.font(23,True),fill='#E5E4D4')
            for a in range(5):
                for j,direction in enumerate(('l','r')):
                    sp=sprites[(a,state,direction,f)];im.paste(sp,(20+a*175+j*75,90),sp)
            frames.append(im)
        frames[0].save(b.OUT/f'_people-{state}.gif',save_all=True,append_images=frames[1:],duration=ms,loop=0,disposal=2)
        gif=Image.open(b.OUT/f'_people-{state}.gif');assert gif.n_frames==n
        for i in range(n):gif.seek(i);assert gif.info['duration']==ms
    print(len(checks),'Bildzuordnungen,',len(unique),'eindeutige Dateien; fünf Aussehen, acht Zustände, Tote und Marker geprüft.')


if __name__=='__main__':main()
