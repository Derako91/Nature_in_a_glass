"""Runde 09: neue Pixelkonturen für die Nahansicht, gemäß Gestaltung 4.3/15.3."""
import io
import json
import subprocess
import numpy as np
from PIL import Image, ImageDraw
import build_animal_proof as old

b=old.b
OUT=old.OUT
SIZES={'deer':(28,24),'deer_young':(18,16),'wolf':(26,16),'fox':(20,12),'rabbit':(12,10)}
NAMES={'deer':'Hirsch','deer_young':'Kalb','wolf':'Wolf','fox':'Fuchs','rabbit':'Kaninchen'}
PALETTES={
 'deer':[['49372B','765133','B07A44','D2A268','DAC3A0'],['49382D','735B40','AF8D60','D3B380','E0CDAB']],
 'deer_young':[['574332','88613D','BD935E','D6B782','E5D4AF'],['574633','806344','B19B70','D5BD8C','E6D8B8']],
 'wolf':[['37444E','57616B','9AA0A6','BCC4CB','D5D7CF'],['3F484D','657078','929A9E','B7C1C2','D7D8CC']],
 'fox':[['4D382C','78432D','D2682C','E89C57','DED1AF'],['4D382C','82543A','BE783C','DDA665','E7D6B4']],
 'rabbit':[['4B4339','796A52','A79879','C8B898','DED0AF'],['4B453D','756D5C','9E9986','C3BEA3','E0D9BC']],
}
BASELINE='2babb42'


def sprite(species,direction,state,frame,variant):
    w,h=SIZES[species]
    labels=Image.new('L',(w,h));d=ImageDraw.Draw(labels)
    # Konturen in einem anatomischen Koordinatensystem; Beleuchtung erst danach.
    def xy(p):
        x,y=p
        return (w-1-x if direction=='l' else x,y)
    def poly(points,tone=2):d.polygon([xy(p) for p in points],fill=tone)
    def line(points,tone=1,width=1):d.line([xy(p) for p in points],fill=tone,width=width)
    def ellipse(box,tone=2):
        x0,y0,x1,y1=box
        if direction=='l':x0,x1=w-1-x1,w-1-x0
        d.ellipse((x0,y0,x1,y1),fill=tone)
    rest=state=='rest';low=state=='graze' and frame==1
    moving=state in ('walk','run')
    offsets=[(-2,2),(0,-1),(2,-2),(-1,0)]
    front,back=offsets[frame] if moving else (0,0)
    if state=='run':front*=2;back*=2
    if species in ('deer','deer_young'):
        adult=species=='deer'
        # Kalb hat kurze Schnauze, große Ohren, Flecken und deutlich kürzere Beine.
        if adult:
            body=(5,10 if not rest else 16,20,16 if not rest else 21)
            hip,shoulder,foot=8,18,22
            neck=[(17,13),(18,8),(21,6),(24,8),(22,15)]
            head=(20,6,26,9)
        else:
            body=(2,6 if not rest else 10,12,11 if not rest else 14)
            hip,shoulder,foot=4,11,14
            neck=[(10,9),(11,4),(14,3),(15,7),(13,11)]
            head=(12,3,17,6)
        if not rest:
            for root,offset,tone in [(hip+1,-back,1),(shoulder+1,-front,1),(hip,back,2),(shoulder,front,2)]:
                end=max(1,min(w-2,root+offset))
                lift=2 if moving and frame%2==1 and tone==1 else 0
                line([(root,body[3]-2),(root,foot-4),(end,foot-lift)],tone,1)
                line([(end,foot-lift),(min(w-1,end+1),foot-lift)],1)
        ellipse(body)
        if rest:
            line([(hip,foot),(shoulder+2,foot)],1)
            neck=[(shoulder-1,body[1]+2),(shoulder,body[1]-4),(shoulder+3,body[1]-5),(shoulder+4,body[1]),(shoulder+2,body[3])]
            head=(shoulder+1,body[1]-5,min(w-1,shoulder+6),body[1]-2)
        if low:
            neck=[(shoulder-1,body[1]+2),(shoulder+3,body[1]+3),(w-3,foot-2),(w-5,foot-1),(shoulder,body[3])]
            head=(w-6,foot-3,w-1,foot-1)
        poly(neck);ellipse(head)
        hx,hy,_,_=head
        poly([(hx+1,hy),(hx-1,max(0,hy-2)),(hx+3,hy)],3)
        if adult and low:
            line([(hx+1,hy),(hx,hy-2),(hx-3,hy-3)],1)
            line([(hx,hy-2),(hx+1,hy-4)],3)
        if adult and not low:
            for root in (hx+1,hx+3):
                line([(root,hy),(root-1,max(0,hy-3)),(root-2,max(0,hy-5))],1)
                line([(root-1,max(0,hy-3)),(root+1,max(0,hy-4))],3)
        if not adult:
            for p in [(5,7),(8,8),(5,9)]:
                if not rest:line([p,p],4)
        line([(head[2]-1,head[1]+1)]*2,1)
        line([(head[2],head[3]-1)]*2,1)
        line([(body[0],body[1]+2),(body[0]-1,body[1]+4)],4)
    elif species in ('wolf','fox'):
        fox=species=='fox'
        hip,shoulder=(7,13) if fox else (8,19)
        top=5 if fox else 5
        bottom=8 if fox else 11
        foot=h-2
        if rest:top+=2;bottom=foot
        if not rest:
            for root,offset,tone in [(hip+1,-back,1),(shoulder+1,-front,1),(hip,back,2),(shoulder,front,2)]:
                end=max(2,min(w-2,root+offset))
                lift=1 if moving and frame%2 else 0
                line([(root,bottom-1),(end,foot-lift)],tone)
                line([(end,foot-lift),(min(w-1,end+1),foot-lift)],1)
        if fox:
            # Breiter Schweif mit heller Spitze, klar vom geraden Wolfsschwanz getrennt.
            dy=-1 if state=='run' and frame%2 else 0
            poly([(8,top+2),(5,top+dy),(1,top+dy-1),(0,top+dy+2),(4,bottom+1),(8,bottom)],2)
            poly([(1,top+dy-1),(0,top+dy+2),(3,bottom),(3,top+dy)],4)
        else:line([(hip-2,top+2),(3,bottom),(1,bottom+1)],1,2)
        ellipse((hip-2,top,shoulder+2,bottom))
        if low:
            head=(shoulder+1,bottom-1,w-1,foot)
            poly([(shoulder,top+1),(w-3,bottom),(w-2,foot),(shoulder,bottom)],2)
        else:
            head=(shoulder+1,top-2,w-2,top+2)
            poly([(shoulder-1,top+1),(shoulder+2,top-2),(w-3,top),(shoulder+2,bottom)],2)
        ellipse(head)
        hx,hy,_,_=head
        poly([(hx,hy+1),(hx,max(0,hy-3)),(hx+2,hy)],1)
        poly([(hx+2,hy+1),(hx+3,max(0,hy-2)),(hx+4,hy+2)],2)
        poly([(head[2]-2,head[3]-1),(w-1,head[3]-1),(w-2,head[3]+1),(head[2]-3,head[3])],4)
        line([(w-1,head[3]-1)]*2,1)
        line([(hx+2,hy+1)]*2,1)
        if fox:line([(shoulder+1,top+2),(shoulder+1,bottom-1)],4)
    else:
        # Kaninchen: aufrechter Hinterkörper, zwei lange Ohren, kurze Vorderpfoten.
        hop=1 if state=='run' and frame in (1,3) else 0
        ellipse((2,4-hop,8,8-hop))
        ellipse((1,6-hop,3,8-hop),4)
        ellipse((7,4 if not low else 6,11,7 if not low else 8))
        if not rest:
            lean=(frame%2) if moving else 0
            line([(8,5),(7+lean,1)],2,2)
            line([(10,5),(10-lean,0)],3)
        else:line([(8,5),(5,3)],2,2)
        line([(9,5 if not low else 7)]*2,1)
        line([(4,8-hop),(6+(front//2 if moving else 0),8)],1)
        line([(9,7),(min(11,9+(back//2 if moving else 0)),8)],2)
    raw=np.array(labels)
    rgba=np.zeros((h,w,4),dtype=np.uint8)
    palette=[b.rgb(c) for c in PALETTES[species][variant]]
    # Feste Schattenrichtung, unabhängig von Blickrichtung und Bewegung.
    for y,x in zip(*np.where(raw>0)):
        if x>0 and y+1<h and raw[y+1,x-1]==0:rgba[y+1,x-1]=(12,26,52,77)
    for y,x in zip(*np.where(raw>0)):
        tag=int(raw[y,x]);tone=tag
        if tag==2:
            # Rechts oben beleuchtet; Bauch und linke Kontur erhalten die dunkle Fellfarbe.
            topedge=y==0 or raw[y-1,x]==0
            bottomedge=y==h-1 or raw[y+1,x]==0
            tone=3 if topedge and x>=w//3 else 1 if bottomedge else 2
        if tag==1:tone=0
        rgba[y,x]=tuple(palette[tone])+(255,)
    return Image.fromarray(rgba)


def baseline(species):
    path=f'assets/nature/animals/{species}_0_graze_r_f0.png'
    data=subprocess.check_output(['git','show',f'{BASELINE}:{path}'],cwd=b.ROOT)
    return Image.open(io.BytesIO(data)).convert('RGBA')


def zoom_sheet(sprites):
    im=Image.new('RGB',(1500,920),b.rgb('0C1A34'));d=ImageDraw.Draw(im)
    d.text((30,24),'TIERE / NEUE KONTUREN FÜR DIE NAHANSICHT',font=b.font(28,True),fill='#E5E4D4')
    d.text((30,70),'Links: tatsächliche Bildschirmgrößen · rechts: Detailvergleich 6× · keine Spielaufnahme',font=b.font(19),fill='#B9C6BB')
    for x,label in [(200,'Zoom 2'),(380,'Zoom 4'),(560,'Zoom 8'),(790,'Bisher vergrößert'),(1120,'Neu gezeichnet')]:
        d.text((x,112),label,font=b.font(19,True),fill='#DADAC4')
    for row,species in enumerate(SIZES):
        y=165+row*145;sp=sprites[(species,'graze','r',0,0)];oldsp=baseline(species)
        d.text((30,y+35),NAMES[species],font=b.font(23,True),fill='#DADAC4')
        for col,zoom in enumerate((2,4,8)):
            x=200+col*180;tile=old.panel_background('grass').resize((round(64*zoom/4),round(32*zoom/4)),Image.Resampling.NEAREST)
            im.paste(tile,(x,y+20),tile)
            size=(max(1,round(sp.width*zoom/8)),max(1,round(sp.height*zoom/8)))
            small=sp.resize(size,Image.Resampling.NEAREST)
            im.paste(small,(x+tile.width//2-small.width//2,y+20+tile.height//2-small.height+1),small)
        prior_size=(sp.width,round(oldsp.height*sp.width/oldsp.width))
        for x,source in [(790,oldsp.resize(prior_size,Image.Resampling.NEAREST)),(1120,sp)]:
            big=source.resize((source.width*6,source.height*6),Image.Resampling.NEAREST)
            im.paste(big,(x,y+125-big.height),big)
    im.save(b.ROOT/'assets/nature/_animals-detail-comparison.png',icc_profile=b.PROFILE)


def motion_sheet(sprites):
    frames=[]
    for tick in range(8):
        im=Image.new('RGB',(1500,940),b.rgb('0C1A34'));d=ImageDraw.Draw(im)
        d.text((25,20),'NAHANSICHT / BEWEGUNG UND FELLVARIANTEN · 4×',font=b.font(27,True),fill='#E5E4D4')
        columns=[('walk','l'),('walk','r'),('run','l'),('run','r')]
        for c,(state,direction) in enumerate(columns):
            d.text((190+c*320,75),('Gehen' if state=='walk' else 'Laufen')+' · '+direction,font=b.font(20),fill='#B9C6BB')
        for row,species in enumerate(SIZES):
            y=125+row*157
            d.text((25,y+35),NAMES[species],font=b.font(23),fill='#DADAC4')
            for c,(state,direction) in enumerate(columns):
                for variant in (0,1):
                    sp=sprites[(species,state,direction,(tick//2 if state=='walk' else tick)%4,variant)]
                    sp=sp.resize((sp.width*4,sp.height*4),Image.Resampling.NEAREST)
                    im.paste(sp,(190+c*320+variant*140,y+100-sp.height),sp)
        frames.append(im)
    frames[0].save(b.ROOT/'assets/nature/_animals-detail-motion.gif',save_all=True,append_images=frames[1:],duration=100,loop=0,disposal=2)


def state_sheet(sprites):
    im=Image.new('RGB',(1500,980),b.rgb('0C1A34'));d=ImageDraw.Draw(im)
    d.text((28,24),'HALTUNGEN / BEIDE RICHTUNGEN · 4×',font=b.font(28,True),fill='#E5E4D4')
    columns=[('graze',0,'Aufrecht'),('graze',1,'Fressen'),('rest',0,'Ruhen'),('run',1,'Laufen')]
    for c,(_,__,title) in enumerate(columns):d.text((180+c*325,88),title,font=b.font(22),fill='#B9C6BB')
    for row,species in enumerate(SIZES):
        y=145+row*160
        d.text((25,y+45),NAMES[species],font=b.font(21,True),fill='#DADAC4')
        for c,(state,frame,_) in enumerate(columns):
            for j,direction in enumerate(('l','r')):
                sp=sprites[(species,state,direction,frame,0)]
                sp=sp.resize((sp.width*4,sp.height*4),Image.Resampling.NEAREST)
                im.paste(sp,(180+c*325+j*145,y+110-sp.height),sp)
    im.save(b.ROOT/'assets/nature/_animals-detail-states.png',icc_profile=b.PROFILE)


def main():
    old_manifest=json.loads((OUT/'manifest.json').read_text(encoding='utf-8'))
    old_files={f for entry in old_manifest['assets'] if entry['id'] in SIZES for f in entry['files']}
    sprites={};assets=[];checks=[];unique={};aliases={};cycles=[]
    backgrounds={k:np.array(old.panel_background(k))[12:20,28:37,:3].mean(axis=(0,1)) for k in ('grass','shrub','forest')}
    weights=np.array([.2126,.7152,.0722])
    for species,size in SIZES.items():
        for state,count,ms in [('graze',2,600),('walk',4,200),('run',4,100),('rest',1,None)]:
            mapping=[]
            for variant in (0,1):
                for direction in ('l','r'):
                    sequence=[]
                    for frame in range(count):
                        im=sprite(species,direction,state,frame,variant)
                        filename=f'{species}_{variant}_{state}_{direction}_f{frame}.png'
                        identity=(im.size,im.tobytes());canonical=unique.get(identity)
                        if canonical is None:
                            canonical=filename;unique[identity]=filename
                            im.save(OUT/filename,icc_profile=b.PROFILE)
                        else:aliases[filename]=canonical
                        sprites[(species,state,direction,frame,variant)]=im;sequence.append(im)
                        mapping.append(dict(variant=variant,direction=direction,frame=frame,file=canonical))
                        pixels=np.array(im);opaque=pixels[pixels[:,:,3]==255,:3]
                        colors={tuple(p[:3]) for p in pixels.reshape(-1,4) if p[3]}
                        assert im.size==size and len(colors)<=6
                        assert set(np.unique(pixels[:,:,3]))<={0,77,255}
                        contrast={k:round(float(np.abs(opaque@weights-bg@weights).mean()/(bg@weights)),3) for k,bg in backgrounds.items()}
                        assert min(contrast.values())>=.15,(filename,contrast)
                        checks.append(dict(frame_id=filename,file=canonical,size=list(size),colors=len(colors),contrast=contrast))
                    if count==4:
                        assert len({im.tobytes() for im in sequence})==4,(species,state,direction)
                        masks=[np.array(im)[:,:,3]==255 for im in sequence]
                        changes=[int(np.count_nonzero(masks[i]!=masks[(i+1)%4])) for i in range(4)]
                        cycles.append(dict(id=species,state=state,variant=variant,direction=direction,loop_ms=count*ms,changed_silhouette_pixels=changes))
            assets.append(dict(id=species,group='animals',size=list(size),display_size=[v/2 for v in size],anchor=[size[0]//2,size[1]-2],variants=2,state=state,directions=['l','r'],frames=count,frame_ms=ms,status='detail_review',files=list(dict.fromkeys(m['file'] for m in mapping)),frame_map=mapping))
    canonical_files=set(unique.values())
    for filename in old_files|set(aliases):
        if filename not in canonical_files:
            path=OUT/filename
            assert path.resolve().parent==OUT.resolve()
            if path.exists():path.unlink()
    # Andere Gruppen bleiben erhalten; alte Alias-Ziele für Tiere werden ersetzt.
    retained=[entry for entry in old_manifest['assets'] if entry['id'] not in SIZES]
    manifest=dict(version=2,status='detail_review',base_tile={'width':64,'height':32},missing=old_manifest.get('missing',[]),frame_aliases=aliases,assets=assets+retained)
    (OUT/'manifest.json').write_text(json.dumps(manifest,indent=2,ensure_ascii=False),encoding='utf-8')
    (OUT/'validation.json').write_text(json.dumps(dict(images=checks,cycles=cycles,zoom_review=[2,4,8],in_game_review='Noch ausstehend; maßstabsgetreues Prüfblatt ist keine Spielaufnahme'),indent=2,ensure_ascii=False),encoding='utf-8')
    zoom_sheet(sprites);motion_sheet(sprites);state_sheet(sprites)
    print(f'{len(checks)} Bildzuordnungen, {len(unique)} eindeutige PNGs, {len(cycles)} Vierbildzyklen in neuer Auflösung geprüft.')


if __name__=='__main__':main()
