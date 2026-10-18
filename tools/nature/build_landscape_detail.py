"""Runde 10: Landschaftsmaterialien und Pflanzen mit echter doppelter Auflösung."""
import io
import json
import math
import subprocess
import numpy as np
from PIL import Image, ImageDraw
from scipy.ndimage import gaussian_filter
import build_proof as b

BASELINE='2babb42'
BASE=b.OUT
COLORS=b.ANCHORS


def original(path):
    return Image.open(io.BytesIO(subprocess.check_output(['git','show',f'{BASELINE}:assets/nature/{path}'],cwd=b.ROOT))).convert('RGBA')


def baseline_manifest():
    return json.loads(subprocess.check_output(['git','show',f'{BASELINE}:assets/nature/manifest.json'],cwd=b.ROOT))


def material(name,size,group,variant=0):
    key=name.replace('ground_','').replace('wall_','').replace('water_','')
    source='water' if group=='water' else 'grass' if key in ('grass_wet','grass_dry','shrub_floor') else 'rock' if key=='rock' else 'burnt' if key=='burnt' else 'soil'
    im=Image.open(b.SRC/(source+'.png')).convert('L')
    if source in ('grass','soil'):
        cx,cy=im.width//2,im.height//2
        im=im.crop((cx-64,cy-64,cx+64,cy+64))
    im=im.resize((64,64),Image.Resampling.BOX)
    data=np.roll(np.array(im).astype(float),(variant*13,variant*19),axis=(0,1))
    if source in ('grass','soil'):
        data=data-gaussian_filter(data,3)+128
    tones=b.palette(COLORS[key],(.84,.93,1,1.045,1.09,1.14))
    if key=='sand':tones=b.palette(COLORS[key],(.94,.97,1,1.02,1.04,1.06))
    if source=='grass':tones=b.palette(COLORS[key],(.92,.97,1,1.025,1.05,1.075))
    if group=='water':tones=b.palette(COLORS[key],(.90,.95,1,1.025,1.05,1.08))
    indices=np.searchsorted(np.quantile(data,[.12,.3,.58,.8,.94]),data)
    # Kleine, materialabhängige Formen sind echte zusätzliche Pixel, kein Upscaling.
    pattern=Image.fromarray(indices.astype('uint8'),'L');d=ImageDraw.Draw(pattern)
    for i in range(58):
        x=3+(i*17+variant*7)%57;y=3+(i*23+variant*11)%57
        if key.startswith('grass'):
            d.line([(x-1,y+2),(x,y),(x+1,y+1),(x+2,y-1)],fill=3+i%2,width=1)
        elif key=='rock' and i%4==0:
            d.line([(x-2,y),(x+1,y-1),(x+4,y+1)],fill=1,width=1)
            d.line([(x-1,y+1),(x+2,y)],fill=4,width=1)
        elif key in ('soil','forest_floor','shrub_floor','burnt') and i%2==0:
            d.line([(x,y),(x+1,y+1)],fill=1+i%4)
        elif group=='water' and i%5==0:
            d.line([(x-3,y),(x+2,y)],fill=3,width=1)
    a=np.array(pattern)
    a[0,:]=a[-1,:]=a[:,0]=a[:,-1]=2
    w,h=size;out=Image.new('RGBA',size)
    diamond=group=='ground' or (group=='water' and key!='stream')
    for y in range(h):
        for x in range(w):
            if diamond:
                u=(x+.5-w/2)/w+(y+.5)/h
                v=(y+.5)/h-(x+.5-w/2)/w
                if not (0<=u<=1 and 0<=v<=1):continue
                xx=min(63,int(u*64));yy=min(63,int(v*64))
            else:xx=x%64;yy=y%64
            tone=int(a[yy,xx])
            out.putpixel((x,y),tones[tone]+(255,))
    if group=='walls':
        p=np.array(out);p[-1]=p[0];out=Image.fromarray(p)
    return out


def plant(name,size,variant=0):
    w,h=size;out=Image.new('RGBA',size);d=ImageDraw.Draw(out)
    shadow=b.rgb('0C1A34')+(77,);trunk=b.rgb('7A6854')+(255,)
    d.ellipse((1,h-5,w//2+3,h-1),fill=shadow)
    if name=='snag_standing':
        # Gesplitterte Spitze und freiliegende Seitenäste.
        colors=[b.rgb(c)+(255,) for c in ['493F35','665744','7A6854','9E8969','B19C7B']]
        d.polygon([(w//2-2,h-2),(w//2-1,3),(w//2+1,1),(w//2+2,h-2)],fill=colors[2])
        for i in range(4):
            y=10+i*7+variant%3;side=-1 if (i+variant)%2 else 1
            d.line([(w//2,y+3),(w//2+side*4,y),(w//2+side*4,y-4)],fill=colors[1 if side<0 else 3],width=1)
        d.line((w//2+1,5,w//2+1,h-3),fill=colors[3],width=1)
        return out
    palettes={
        'tree_old':['173D25','1E502A','396331','607A3B'],
        'tree_mid':['294D2A','466C32','698743','98A35A'],
        'tree_young':['305E30','46843A','70964A','A1AE65'],
        'shrub':['46572E','637438','76823C','A2A55C']}
    colors=[b.rgb(c)+(255,) for c in palettes[name]]
    is_shrub=name=='shrub'
    top=2 if is_shrub else 4
    bottom=h-3 if is_shrub else int(h*.78)
    if not is_shrub:
        d.line((w//2,top+4,w//2,h-2),fill=trunk,width=2 if w<30 else 3)
        for side in (-1,1):d.line([(w//2,h-8),(w//2+side*4,h-14)],fill=trunk,width=1)
    mask=Image.new('L',size);md=ImageDraw.Draw(mask)
    centers=[(.28,.66,.25),(.70,.60,.27),(.50,.33,.28),(.25,.37,.18),(.74,.35,.17)]
    if is_shrub:centers=[(.25,.60,.25),(.48,.45,.29),(.72,.58,.24)]
    lobes=[]
    for fx,fy,fr in centers:
        cx=int(w*fx);cy=top+int((bottom-top)*fy)+variant%2;rx=max(3,int(w*fr)+(variant==2));ry=max(3,int((bottom-top)*fr*1.3))
        box=(max(1,cx-rx),max(top,cy-ry),min(w-2,cx+rx),min(bottom,cy+ry))
        md.ellipse(box,fill=255);lobes.append((cx,cy,rx,ry))
    source=Image.open(b.ROOT/'art/nature/comfyui-v2'/f'{name}.png').convert('L').resize(size,Image.Resampling.BOX)
    source=np.array(source)
    for y in range(top,bottom+1):
        for x in range(w):
            if not mask.getpixel((x,y)):continue
            # Blattgruppen mit klaren Lichtflächen statt zufällig wechselnder Kontur.
            fine=(((x//2+variant)*13+(y//2)*7+(x//2)*(y//2))%17)/17
            lit=.55*x/w+.25*(1-y/h)+.20*source[y,x]/255
            tone=max(0,min(3,int(lit*4 + (fine-.5)*1.6)))
            out.putpixel((x,y),colors[tone])
    return out


def scene(images,detail):
    n=6;canvas=Image.new('RGBA',(820,500));origin=410
    for s in range(11):
        for y in range(n):
            x=s-y
            if not 0<=x<n:continue
            cx=origin+(x-y)*64;cy=90+(x+y)*32
            if x>3 and y<3:key='ground/ground_rock_0.png'
            elif (x,y) in [(3,4),(3,5),(4,4),(4,5)]:key='water/water_pond_0.png'
            else:key='ground/ground_grass_wet_0.png'
            if detail and key.startswith('ground/'):
                key=key.replace('_0.png',f'_{(x+2*y)%4}.png')
            tile=images[key]
            if not detail:tile=tile.resize((128,64),Image.Resampling.NEAREST)
            canvas.alpha_composite(tile,(cx-64,cy-32))
            if x<3 and y<4:
                name='tree_old' if x<2 else 'tree_mid'
            elif x==4 and y==1:name='snag_standing'
            elif x<2 and y>=4:name='shrub'
            elif x==5 and y==5:name='tree_young'
            else:continue
            variant=(x+2*y)%4 if detail else 0
            sp=images[f'vegetation/{name}_{variant}_normal.png']
            if not detail:sp=sp.resize((sp.width*2,sp.height*2),Image.Resampling.NEAREST)
            canvas.alpha_composite(sp,(cx-sp.width//2,cy-sp.height+2))
    return canvas


def main():
    old_manifest=baseline_manifest();images={};old_images={};checks=[]
    for entry in old_manifest['assets']:
        old_size=entry['size'];size=[v*2 for v in old_size]
        source_file=entry['files'][0]
        old_images[source_file]=original(source_file)
        variants=4 if entry['group'] in ('ground','vegetation') else 1
        entry['files']=[];entry['variants']=variants
        for variant in range(variants):
            filename=source_file.replace('_0',f'_{variant}')
            im=plant(entry['id'],tuple(size),variant) if entry['group']=='vegetation' else material(entry['id'],tuple(size),entry['group'],variant)
            colors={tuple(p[:3]) for p in np.array(im).reshape(-1,4) if p[3]}
            assert len(colors)<=6,(filename,len(colors))
            assert set(np.unique(np.array(im)[:,:,3]))<={0,77,255}
            assert im.size==tuple(size)
            assert im.tobytes()!=old_images[source_file].resize(tuple(size),Image.Resampling.NEAREST).tobytes()
            im.save(BASE/filename,icc_profile=b.PROFILE);images[filename]=im
            checks.append(dict(file=filename,size=size,colors=len(colors),new_detail=True))
            entry['files'].append(filename)
        entry['display_size']=old_size;entry['size']=size;entry['anchor']=[v*2 for v in entry['anchor']];entry['resolution_scale']=2
    old_manifest['status']='landscape_detail_review'
    (BASE/'manifest.json').write_text(json.dumps(old_manifest,indent=2,ensure_ascii=False),encoding='utf-8')
    (BASE/'landscape-detail-validation.json').write_text(json.dumps(checks,indent=2),encoding='utf-8')
    board=Image.new('RGB',(1800,1180),b.rgb('0C1A34'));d=ImageDraw.Draw(board)
    d.text((35,25),'LANDSCHAFT / ECHTE DETAILS FÜR DIE NAHANSICHT',font=b.font(30,True),fill='#E5E4D4')
    d.text((35,75),'Gleiche Weltgröße · neue Materialstrukturen, Blattgruppen und Äste · keine Spielaufnahme',font=b.font(21),fill='#B9C6BB')
    for x,detail,title in [(30,False,'Bisher · bei Zoom 8 vergrößert'),(930,True,'Neu · bei Zoom 8 volle Auflösung')]:
        d.text((x,140),title,font=b.font(25,True),fill='#E5E4D4')
        im=scene(images if detail else old_images,detail);board.paste(im,(x,180),im)
    keys=['ground/ground_rock_0.png','ground/ground_grass_wet_0.png','vegetation/tree_old_0_normal.png','vegetation/snag_standing_0_normal.png']
    for i,(key,title) in enumerate(zip(keys,['Felsstruktur','Grasbüschel','Baumkrone','Totholz / Äste'])):
        x=40+i*440;d.text((x,725),title,font=b.font(23,True),fill='#E5E4D4')
        im=images[key];im=im.resize((im.width*3,im.height*3),Image.Resampling.NEAREST)
        board.paste(im,(x,785),im)
    d.text((35,1110),'Doppelte Pixelauflösung · vier Boden-/Pflanzenvarianten · sechs Farben je Asset · gleicher Maßstab',font=b.font(21),fill='#B9C6BB')
    board.save(BASE/'_landscape-detail-comparison.png',icc_profile=b.PROFILE)
    zooms=Image.new('RGB',(1600,650),b.rgb('0C1A34'));z=ImageDraw.Draw(zooms)
    z.text((28,22),'LANDSCHAFT / ZOOM 2, 4 UND 8 · tatsächliche Rastergrößen',font=b.font(27,True),fill='#E5E4D4')
    full=scene(images,True)
    for x,zoom in [(25,2),(275,4),(740,8)]:
        z.text((x,90),f'Zoom {zoom}',font=b.font(23),fill='#B9C6BB')
        view=full.resize((round(full.width*zoom/8),round(full.height*zoom/8)),Image.Resampling.NEAREST)
        zooms.paste(view,(x,130),view)
    zooms.save(BASE/'_landscape-detail-zoom.png',icc_profile=b.PROFILE)
    print(len(checks),'Landschaftsassets in doppelter Auflösung erstellt und geprüft.')


if __name__=='__main__':main()
