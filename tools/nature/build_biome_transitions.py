"""Nachbarschaftsabhängige Biomentwürfe aus vorhandenen ComfyUI-Materialien.

Fest gespeicherte Testfelder, keine neue Simulation. Materialmischung in Welt-
koordinaten verhindert sichtbare Wiederholungen an jeder 64×32-Feldgrenze.
"""
import json
import math
import numpy as np
from PIL import Image, ImageDraw
import build_proof as b
import build_biome_proof as old

OUT=b.ROOT/'assets/nature/transitions'
BASE=b.ROOT/'assets/nature'
SEED=20261017

def smooth(a,b,x):
    t=max(0,min(1,(x-a)/(b-a)))
    return t*t*(3-2*t)

def noise(x,y,s=0):
    h=(int(x)*374761393+int(y)*668265263+(SEED+s)*1442695041)&0xffffffff
    h=((h^(h>>13))*1274126177)&0xffffffff
    return ((h^(h>>16))&65535)/65535

def positions(x,y):
    result=[]
    for i in range(200):
        dx=round((noise(x,y,2*i+7)-.5)*42)
        dy=round((noise(x,y,2*i+8)-.5)*18)
        if abs(dx)/2+abs(dy)>11:continue
        if any((dx-a)**2+(dy-c)**2<36 for a,c in result):continue
        result.append((dx,dy))
        if len(result)==6:return result
    raise RuntimeError('Keine sechs gültigen Fußpunkte')

def make_fields(n,kind=None):
    fields=[]
    for y in range(n):
        row=[]
        for x in range(n):
            if kind is None:
                moisture=.92-.78*x/(n-1)
                forest=.9*(1-smooth(2,6,x))*(1-smooth(5,9,y))
                forest*=.72+.28*noise(x,y,19)
                if (x-2)**2+(y-3)**2<1.5:forest=.15
                shrub=.76*smooth(5,8,x)*(1-smooth(9,11,x))*smooth(2,5,y)
                rock=smooth(7,10,x)*(1-smooth(2,5,y))
                water=max(0,290-60*((x-3.8)**2+(y-8.0)**2))
                grass=(.93-.7*forest-.48*shrub)*(1-.9*rock)
                height=6+round(rock*4)*5
                if water>0:moisture=max(moisture,.98)
            else:
                p=next(p for name,t,c,co,p in old.PROFILES if name==kind)
                moisture=p['moisture'];forest=p['forest'];shrub=p['shrub'];grass=p['grass'];rock=0;water=0;height=6
                patch=math.sin(x*1.7+y*.6)*math.cos(y*1.3-x*.4)
                forest=max(0,forest+patch*.28) if forest else 0
                shrub=max(0,min(1,shrub+patch*.2))
                grass=max(0,min(1,grass+patch*.16))
                moisture=max(0,min(1,moisture+patch*.1))
                if kind=='forest' and (x-2)**2+(y-2)**2<1.8:
                    forest=.08;grass=.75;shrub=.22
                if kind=='meadow':forest=.3 if (x,y) in [(0,1),(1,0),(4,3)] else .02
                if kind=='scrub':forest=.15 if (x,y) in [(1,2),(4,1)] else 0
                if kind=='wetland':
                    water=max(0,290-55*((x-2.2)**2+(y-2.1)**2))
                    forest=.35 if x in (0,4) else .06
                if kind=='highland':
                    rock=.65+.35*smooth(0,3,4-x+y*.2)
                    ridge=max(0,1-((x-1.1)/3.3)**2-((y-2.0)/3.4)**2)
                    height=6+round(ridge*4)*5
                    grass=.32*(1-smooth(1,4,4-x+y*.2))
                    forest=0;shrub=.13*(1-rock)
            age=75 if forest>.5 else 35 if forest>.2 else 10
            if water>150:forest=shrub=grass=0
            if rock>.7:forest=0
            row.append(dict(x=x,y=y,soil_moisture_fraction=round(moisture,4),grass=round(grass,4),forest=round(min(1,forest),4),
                            shrub=round(shrub,4),forest_age=age,surface_water=round(water,2),rock=round(rock,4),height_px=height,flow=0))
        fields.append(row)
    # Expliziter zusammenhängender Fließpfad; alle Bachpixel bleiben in diesen Feldern.
    if kind is None:
        route=[(2,3),(2,4),(3,4),(3,5),(3,6),(4,6),(4,7),(4,8)]
    elif kind=='meadow':route=[(0,0),(0,1),(1,1),(1,2),(2,2),(2,3),(3,3),(3,4)]
    else:route=[]
    for i,(x,y) in enumerate(route):
        fields[y][x]['flow']=85
        fields[y][x]['upstream']=list(route[i-1]) if i else None
        fields[y][x]['downstream']=list(route[i+1]) if i+1<len(route) else None
    return fields

def sample(fields,u,v,key):
    n=len(fields);u=max(0,min(n-1,u));v=max(0,min(n-1,v))
    x=int(u);y=int(v);xx=min(n-1,x+1);yy=min(n-1,y+1);fx=u-x;fy=v-y
    return (fields[y][x][key]*(1-fx)+fields[y][xx][key]*fx)*(1-fy)+(fields[yy][x][key]*(1-fx)+fields[yy][xx][key]*fx)*fy

def load_materials():
    sources={
        'soil':('art/nature/comfyui/soil.png','967E58'),
        'dry':('art/nature/comfyui-biomes/dry_soil.png','AA895A'),
        'damp':('art/nature/comfyui-biomes/wet_soil.png','675E42'),
        'rock':('art/nature/comfyui-biomes/highland_rock.png','8A847C'),
        'wetgrass':('art/nature/comfyui/grass.png','68A444'),
        'drygrass':('art/nature/comfyui/grass.png','BAAC5C'),
        'water':('art/nature/comfyui/water.png','3A76BA'),
        'floor':('art/nature/comfyui/soil.png','6E6945'),
    }
    out={}
    for name,(path,color) in sources.items():
        # Große vorhandene Vorlage einmal als zusammenhängende Textur lesen.
        src=np.array(Image.open(b.ROOT/path).convert('L').resize((256,256),Image.Resampling.BOX))
        levels=np.quantile(src,[.1,.28,.58,.82,.95]);idx=np.searchsorted(levels,src)
        tones=b.palette(color,(.84,.93,1,1.04,1.09,1.15))
        if name=='water':tones=b.palette(color,(.90,.95,1,1.025,1.05,1.08))
        out[name]=np.array(tones,dtype=np.uint8)[idx]
    return out

def tile(fields,x,y,materials):
    im=Image.new('RGBA',(64,32));mask=b.diamond_mask();t=fields[y][x]
    for py in range(32):
        for px in range(64):
            if not mask.getpixel((px,py)):continue
            # Die Oberflächenkoordinaten treffen an gemeinsamen Kanten exakt zusammen.
            u=(px+.5-32)/64+(py+.5)/32
            v=(py+.5)/32-(px+.5-32)/64
            wx=x+u;wy=y+v;gx=int(wx*32);gy=int(wy*32)
            s=sample(fields,wx-.5,wy-.5,'soil_moisture_fraction')
            grass=sample(fields,wx-.5,wy-.5,'grass') if t['grass']>0 else 0
            forest=sample(fields,wx-.5,wy-.5,'forest')
            rock=sample(fields,wx-.5,wy-.5,'rock')
            # Feste räumliche Lücken statt Alpha-Fransen oder zeitlichem Zufall.
            coarse=noise(gx//4,gy//4,81)*.6+noise(gx//2,gy//2,37)*.25+noise(gx,gy,13)*.15
            name='dry' if s<.35 else 'damp' if s>.78 else 'soil'
            if forest>.35 and coarse<forest*.8:name='floor'
            if coarse<grass:
                dry_amount=1-smooth(.18,.62,s)
                name='drygrass' if noise(gx//3,gy//3,122)<dry_amount else 'wetgrass'
            if coarse<rock:name='rock'
            if t['surface_water']>150:
                water=sample(fields,wx-.5,wy-.5,'surface_water')
                shore=water+(noise(gx//2,gy//2,49)-.5)*18
                # Landfelder bleiben grundsätzlich wasserfrei.
                if shore>150:name='water'
                else:name='damp'
            c=tuple(materials[name][gy%256,gx%256])
            im.putpixel((px,py),c+(255,))
    return im

def tree_count(f):
    return 0 if f<.05 else 1 if f<.2 else 2 if f<.4 else 3 if f<.6 else 4 if f<.7 else 5 if f<.8 else 6

def stream_overlay(t,x,y):
    """Quadratische Kurve mit festen Kantenanschlüssen, auf die Raute begrenzt."""
    center=(32,16)
    endpoints=[]
    for key in ['upstream','downstream']:
        p=t.get(key)
        if p:
            dx=p[0]-x;dy=p[1]-y
            endpoints.append((32+(dx-dy)*16,16+(dx+dy)*8))
        else:endpoints.append(center)
    start,end=endpoints
    points=[]
    for i in range(33):
        f=i/32
        points.append(tuple(round((1-f)**2*start[j]+2*f*(1-f)*center[j]+f*f*end[j]) for j in (0,1)))
    overlay=Image.new('RGBA',(64,32));draw=ImageDraw.Draw(overlay)
    for color,width in [('675E42',5),('4E96E6',3),('87B4C9',1)]:
        draw.line(points,fill=b.rgb(color)+(255,),width=width,joint='curve')
    alpha=np.array(overlay.getchannel('A'))
    alpha[np.array(b.diamond_mask())==0]=0
    overlay.putalpha(Image.fromarray(alpha))
    return overlay

def render(fields,materials):
    n=len(fields);canvas=Image.new('RGBA',(n*64+48,n*32+100));d=ImageDraw.Draw(canvas)
    origin=canvas.width//2
    for s in range(2*n-1):
        for y in range(n):
            x=s-y
            if not 0<=x<n:continue
            t=fields[y][x];height=t['height_px'];cx=origin+(x-y)*32;cy=60+(x+y)*16-height
            wall=b.rgb('8A847C' if t['rock']>.5 else '967E58')
            left=fields[y+1][x]['height_px'] if y+1<n else 0
            right=fields[y][x+1]['height_px'] if x+1<n else 0
            if height>left:d.polygon([(cx-32,cy),(cx,cy+16),(cx,cy+16+height-left),(cx-32,cy+height-left)],fill=tuple(round(c*.56) for c in wall))
            if height>right:d.polygon([(cx,cy+16),(cx+32,cy),(cx+32,cy+height-right),(cx,cy+16+height-right)],fill=tuple(round(c*.72) for c in wall))
            canvas.alpha_composite(tile(fields,x,y,materials),(cx-32,cy-16))
            if t['flow']>=60 and t['surface_water']<=150:
                canvas.alpha_composite(stream_overlay(t,x,y),(cx-32,cy-16))
            if t['surface_water']>150:continue
            spots=positions(x,y);sprites=[]
            stage='young' if t['forest_age']<15 else 'mid' if t['forest_age']<60 else 'old'
            for i,(dx,dy) in enumerate(spots[:tree_count(t['forest'])]):
                sprites.append((dy,dx,old.original('vegetation','tree_'+stage+'_0_normal')))
            for dx,dy in spots[::-1][:old.count_shrub(t['shrub'])]:
                sprites.append((dy,dx,old.original('vegetation','shrub_0_normal')))
            for dy,dx,sp in sorted(sprites,key=lambda p:p[0]):canvas.alpha_composite(sp,(cx+dx-sp.width//2,cy+dy-sp.height+1))
    return canvas

def masks():
    result=[];diamond=b.diamond_mask()
    for edge in ['n','e','s','w']:
        for variant in range(3):
            im=Image.new('RGBA',(64,32))
            for y in range(32):
                for x in range(64):
                    if not diamond.getpixel((x,y)):continue
                    u=(x+.5-32)/64+(y+.5)/32;v=(y+.5)/32-(x+.5-32)/64
                    distance={'n':v,'e':1-u,'s':1-v,'w':u}[edge]
                    coverage=1-smooth(.08,.66,distance)
                    if noise(x,y,400+variant)<coverage:im.putpixel((x,y),(184,188,156,255))
            path=OUT/f'edge_{edge}_{variant}.png';im.save(path,icc_profile=b.PROFILE)
            a=np.array(im)
            assert set(np.unique(a[:,:,3]))<={0,255}
            assert np.all(a[:,:,3][np.array(diamond)==0]==0)
            result.append(dict(file=path.name,size=[64,32],anchor=[32,16],role='coverage_mask',edge=edge,variant=variant,colors=1))
    return result

def main():
    OUT.mkdir(parents=True,exist_ok=True);materials=load_materials();mask_info=masks()
    fields=make_fields(12);scene=render(fields,materials)
    scene.save(OUT/'connected-landscape.png',icc_profile=b.PROFILE)
    (OUT/'connected-landscape.json').write_text(json.dumps(fields,indent=2),encoding='utf-8')
    board=Image.new('RGB',(1800,1250),b.rgb('0C1A34'));d=ImageDraw.Draw(board)
    d.text((52,30),'NATURE IN A GLASS / BIOME / RUNDE 07',font=b.font(22,True),fill='#91A99D')
    d.text((50,70),'Eine Landschaft statt einzelner Felder.',font=b.font(40,True),fill='#E5E4D4')
    d.text((52,128),'Waldsäume · Lichtungen · Feuchte Ufer · Trockene Grasinseln · Gestufte Felspartien',font=b.font(22),fill='#B9C6BB')
    zoom=scene.resize((scene.width*2,scene.height*2),Image.Resampling.NEAREST)
    board.paste(zoom,((1800-zoom.width)//2,200),zoom)
    d.text((52,1170),'12 × 12 Testfelder · 64 × 32 px · feste Vegetationspositionen · noch keine Spielintegration',font=b.font(20),fill='#A4B6BB')
    board.save(BASE/'_biome-transitions.png',icc_profile=b.PROFILE)
    board=Image.new('RGB',(2280,1390),b.rgb('0C1A34'));d=ImageDraw.Draw(board)
    d.text((48,34),'NATURE IN A GLASS / AUSGEARBEITETE BIOME',font=b.font(22,True),fill='#91A99D')
    d.text((46,76),'Mehr Charakter. Weniger Raster.',font=b.font(40,True),fill='#E5E4D4')
    captions=['Wald mit Lichtung und lockerem Saum','Wieseninseln und geschwungener Bach','Buschgruppen statt gleichmäßiger Verteilung','Trockene Graspolster mit offenen Bodenlücken','Feuchter Ufersaum ohne starre Wasserrauten','Zusammenhängende Stufen und karge Grasnischen']
    for i,(name,title,_,__,profile) in enumerate(old.PROFILES):
        x=32+(i%3)*752;y=158+(i//3)*592
        d.rounded_rectangle((x,y,x+728,y+570),radius=12,fill='#14283A')
        d.text((x+22,y+20),title,font=b.font(26,True),fill='#E5E4D4')
        f=make_fields(5,name);im=render(f,materials)
        im.save(OUT/(name+'.png'),icc_profile=b.PROFILE)
        (OUT/(name+'.json')).write_text(json.dumps(f,indent=2),encoding='utf-8')
        im=im.resize((im.width*2,im.height*2),Image.Resampling.NEAREST)
        board.paste(im,(x-4,y+42),im)
        d.text((x+22,y+529),captions[i],font=b.font(19),fill='#B9C6BB')
    board.save(BASE/'_biomes-refined.png',icc_profile=b.PROFILE)
    report=dict(version=1,status='design_prototype',seed=SEED,base_tile=[64,32],masks=mask_info,
                sources='Vorhandene lokale ComfyUI-Materialvorlagen; keine neuen Rohbildkopien',
                sampling='Weltkoordinaten, nachbarabhängige Bodenanteile, feste räumliche Pixelmasken',
                water_rule='Wasserpixel nur auf Feldern mit surface_water > 150; Bachpixel nur bei flow >= 60')
    (OUT/'manifest.json').write_text(json.dumps(report,indent=2,ensure_ascii=False),encoding='utf-8')
    assert all(len(positions(x,y))==6 for x in range(12) for y in range(12))
    print('Zwölf Übergangsmasken, sechs überarbeitete Biome und eine zusammenhängende Prüflandschaft erstellt.')

if __name__=='__main__':main()
