"""Vollständige Vierbild-Zyklen und zwei Fellvarianten auf dem freigegebenen Raster."""
import json
import numpy as np
from PIL import Image, ImageDraw
import build_animal_proof as a

b=a.b
BASE_CONTOUR=a.contour
BASE_PALETTES=a.PALETTES.copy()
PALETTES_1={
 'deer':['694C36','C5A06F','DDBB8B','BBA37C'],
 'deer_young':['785839','BA9566','D8B783','C1AB7F'],
 'wolf':['515960','9CA5AE','BFCAD0','BCC3BD'],
 'fox':['71412D','C4753D','D58949','D4C6A0'],
 'rabbit':['6B604F','928467','BBAA86','D6C49E'],
}
FOX={
 'r':[['....k.','tbbbb.','tbbbbb','.k..k.'],
      ['....k.','.bbbb.','tbbbbb','tk.k..'],
      ['....k.','tbbbb.','tbbbbb','..k..k'],
      ['....k.','tbbbb.','.bbbbb','..kk..']],
 'l':[['.k....','.bbbbt','bbbbbt','.k..k.'],
      ['.k....','.bbbb.','bbbbbt','..k.kt'],
      ['.k....','.bbbbt','bbbbbt','k..k..'],
      ['.k....','.bbbbt','bbbbb.','..kk..']],
}
RABBIT={
 'r':[['.k.','bbb','tb.'],['.kk','bbb','t.b'],['k.k','bbb','.tb'],['kk.','bbb','tb.']],
 'l':[['.k.','bbb','.bt'],['kk.','bbb','b.t'],['k.k','bbb','bt.'],['.kk','bbb','.bt']],
}

def contour(species,direction,state,frame):
    w,h=a.SIZES[species]
    if species=='fox' and state!='rest':
        index=frame if state in ('walk','run') else 0
        rows=[list(r) for r in FOX[direction][index]]
        if state=='graze' and frame==1:
            rows[0]=list('......')
            rows[3][5 if direction=='r' else 0]='b'
        if state=='run':
            # Schweif wird beim Strecken waagerechter; rechte Lichtseite bleibt fest.
            tail=0 if direction=='r' else 5
            if frame in (1,3):rows[0][tail]='t'
        return rows
    if species=='rabbit' and state!='rest':
        index=frame if state in ('walk','run') else (0 if frame==0 else 3)
        rows=[list(r) for r in RABBIT[direction][index]]
        if state=='run' and frame in (1,3):
            rows[2]=list('t..' if direction=='r' else '..t')
        return rows
    if state not in ('walk','run'):return BASE_CONTOUR(species,direction,state,frame)
    rows=[list(r) for r in a.STAND[species][direction]]
    # Zwei Beinebenen: abwechselnd tragen, vorführen, gegenüberliegend tragen,
    # zurückführen. Der Körper und der Anker bleiben im Raster stehen.
    near=(2,w-3)
    strides=[(-1,1),(0,0),(1,-1),(0,0)] if state=='walk' else [(-1,1),(1,0),(1,-1),(0,-1)]
    offset_front,offset_back=strides[frame]
    if direction=='l':offset_front,offset_back=-offset_back,-offset_front
    rows[h-2]=list('.'*w);rows[h-1]=list('.'*w)
    for leg,root in enumerate(near):
        shade='s' if leg==0 else 'k'
        dx=offset_front if leg==0 else offset_back
        end=max(0,min(w-1,root+dx))
        rows[h-2][root]=shade
        if frame==3 and leg==0:
            rows[h-2][end]=shade  # vorgeführter Huf, noch nicht aufgesetzt
        else:rows[h-1][end]=shade
    return rows

a.contour=contour

def make(species,direction,state,frame,variant):
    a.PALETTES=BASE_PALETTES if variant==0 else PALETTES_1
    return a.make_sprite(species,direction,state,frame)

def preview(sprites,kind):
    result=[]
    labels={'grass':'Gras','shrub':'Buschboden','forest':'Waldboden'}
    names={'deer':'Hirsch','deer_young':'Kalb','wolf':'Wolf','fox':'Fuchs','rabbit':'Kaninchen'}
    ground=a.panel_background(kind).crop((18,10,46,24)).resize((224,112),Image.Resampling.NEAREST)
    for tick in range(8):
        im=Image.new('RGB',(1400,860),b.rgb('0C1A34'));d=ImageDraw.Draw(im)
        d.text((34,24),'TIERBEWEGUNG / '+labels[kind].upper(),font=b.font(25,True),fill='#E5E4D4')
        d.text((34,63),'Zwei Fellvarianten je Feld · 8× vergrößert · Feste Fußpunkte und Licht von rechts oben',font=b.font(18),fill='#B2C0B9')
        columns=[('walk','l','Gehen links · 0,8 s'),('walk','r','Gehen rechts · 0,8 s'),('run','l','Laufen links · 0,4 s'),('run','r','Laufen rechts · 0,4 s')]
        for c,(_,_,title) in enumerate(columns):d.text((190+c*298,112),title,font=b.font(18,True),fill='#D1D3BB')
        for row,species in enumerate(a.SIZES):
            y=153+row*132
            d.text((30,y+39),names[species],font=b.font(20,True),fill='#D1D3BB')
            for c,(state,direction,_) in enumerate(columns):
                x=190+c*298
                im.paste(ground,(x,y),ground)
                frame=(tick//2)%4 if state=='walk' else tick%4
                for variant in (0,1):
                    sp=sprites[(species,state,direction,frame,variant)]
                    sp=sp.resize((sp.width*8,sp.height*8),Image.Resampling.NEAREST)
                    im.paste(sp,(x+40+variant*104-sp.width//2,y+80-sp.height+8),sp)
        result.append(im)
    result[0].save(b.ROOT/f'assets/nature/_animals-motion-{kind}.gif',save_all=True,append_images=result[1:],duration=100,loop=0,disposal=2)
    if kind=='grass':result[0].save(b.ROOT/'assets/nature/_animals-motion.png',icc_profile=b.PROFILE)

def main():
    a.OUT.mkdir(parents=True,exist_ok=True)
    sprites={};assets=[];checks=[];cycles=[];unique={};aliases={}
    backgrounds={kind:np.array(a.panel_background(kind))[12:20,28:37,:3].mean(axis=(0,1)) for kind in ['grass','shrub','forest']}
    weight=np.array([.2126,.7152,.0722])
    for species,size in a.SIZES.items():
        for state,n,ms in [('graze',2,600),('walk',4,200),('run',4,100),('rest',1,None)]:
            files=[];frame_map=[]
            for variant in (0,1):
                for direction in ('l','r'):
                    frames=[]
                    for frame in range(n):
                        im=make(species,direction,state,frame,variant)
                        filename=f'{species}_{variant}_{state}_{direction}_f{frame}.png'
                        identity=(im.size,im.tobytes())
                        canonical=unique.get(identity)
                        if canonical is None:
                            canonical=filename;unique[identity]=filename
                            im.save(a.OUT/filename,icc_profile=b.PROFILE)
                        else:
                            aliases[filename]=canonical
                            redundant=a.OUT/filename
                            if redundant.exists():
                                assert redundant.resolve().parent==a.OUT.resolve()
                                assert Image.open(redundant).convert('RGBA').tobytes()==im.tobytes()
                                redundant.unlink()
                        files.append(canonical);frames.append(im)
                        frame_map.append(dict(variant=variant,direction=direction,frame=frame,file=canonical))
                        sprites[(species,state,direction,frame,variant)]=im
                        pixels=np.array(im);opaque=pixels[pixels[:,:,3]==255,:3]
                        colors={tuple(p[:3]) for p in pixels.reshape(-1,4) if p[3]}
                        assert im.size==size and len(colors)<=6
                        assert set(np.unique(pixels[:,:,3]))<={0,77,255}
                        contrast={kind:round(float(np.mean(np.abs(opaque@weight-bg@weight))/(bg@weight)),3) for kind,bg in backgrounds.items()}
                        assert min(contrast.values())>=.15,(filename,contrast)
                        checks.append(dict(frame_id=filename,file=canonical,size=list(size),colors=len(colors),alpha_ok=True,srgb=True,mean_absolute_luma_contrast=contrast))
                    if n==4:
                        distinct=len({im.tobytes() for im in frames})
                        assert distinct==4,(species,state,direction,variant,'Doppelte Phase')
                        arrays=[np.array(im)[:,:,3]==255 for im in frames]
                        changes=[int(np.count_nonzero(arrays[i]!=arrays[(i+1)%4])) for i in range(4)]
                        # Die Naht 3->0 darf nicht stärker springen als die anderen Phasen.
                        assert changes[-1]<=max(changes[:3])+2,(species,state,changes)
                        cycles.append(dict(id=species,state=state,direction=direction,variant=variant,distinct_frames=distinct,loop_ms=n*ms,changed_silhouette_pixels=changes))
            assets.append(dict(id=species,group='animals',size=list(size),anchor=[size[0]//2,size[1]-1],variants=2,state=state,directions=['l','r'],frames=n,frame_ms=ms,status='animation_review',files=list(dict.fromkeys(files)),frame_map=frame_map))
    manifest=dict(version=1,status='animation_review',base_tile={'width':64,'height':32},missing=['carcasses','tracks'],frame_aliases=aliases,assets=assets)
    (a.OUT/'manifest.json').write_text(json.dumps(manifest,indent=2),encoding='utf-8')
    (a.OUT/'validation.json').write_text(json.dumps(dict(images=checks,cycles=cycles,silhouette_50_percent='Kleinformen verbessert; abschließende Erkennbarkeit im Spiel offen'),indent=2,ensure_ascii=False),encoding='utf-8')
    for kind in backgrounds:preview(sprites,kind)
    # Bestehendes Formenblatt bleibt mit aktuellen Konturen synchron.
    a.sheet({key[:4]:im for key,im in sprites.items() if key[4]==0},animated=True)
    film=Image.new('RGB',(1150,660),b.rgb('0C1A34'));d=ImageDraw.Draw(film)
    d.text((24,20),'PHASENPRÜFUNG · Gehen 0–3 / Laufen 0–3 · Richtung rechts',font=b.font(22,True),fill='#E5E4D4')
    for row,species in enumerate(a.SIZES):
        y=90+row*110
        d.text((16,y+30),species,font=b.font(17),fill='#ACBAB0')
        for col,(state,frame) in enumerate([(s,f) for s in ['walk','run'] for f in range(4)]):
            sp=sprites[(species,state,'r',frame,0)]
            sp=sp.resize((sp.width*8,sp.height*8),Image.Resampling.NEAREST)
            film.paste(sp,(155+col*120,y+70-sp.height),sp)
            d.text((160+col*120,y+78),str(frame),font=b.font(15),fill='#ACBAB0')
    film.save(b.ROOT/'assets/nature/_animals-frames.png',icc_profile=b.PROFILE)
    feed=[]
    for frame in range(2):
        im=Image.new('RGB',(1000,280),b.rgb('0C1A34'));d=ImageDraw.Draw(im)
        d.text((24,16),'Fresszustand · 2 Bilder / 1,2 Sekunden · 12×',font=b.font(22,True),fill='#E5E4D4')
        for i,species in enumerate(a.SIZES):
            sp=sprites[(species,'graze','r',frame,0)]
            sp=sp.resize((sp.width*12,sp.height*12),Image.Resampling.NEAREST)
            im.paste(sp,(30+i*195,195-sp.height),sp)
            d.text((30+i*195,221),species,font=b.font(18),fill='#AABAB5')
        feed.append(im)
    feed[0].save(b.ROOT/'assets/nature/_animals-graze.gif',save_all=True,append_images=feed[1:],duration=600,loop=0,disposal=2)
    print(len(checks),'Bildzuordnungen aus',len(unique),'eindeutigen PNGs,',len(cycles),'Vierbild-Zyklen und drei animierte Bodenprüfungen erstellt.')
    import build_animal_remains
    build_animal_remains.main()

if __name__=='__main__':main()
