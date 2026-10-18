"""Gebirgsgewässer: nachbarabhängige Prüfbelegungen und ruhige Wasseranimation."""
import json
import numpy as np
from PIL import Image, ImageDraw
import build_biome_transitions as land

b = land.b
OUT = b.ROOT / 'assets/nature/mountain-water'
COLORS = ['397DB5', '438AC9', '4E96E6', '619FE2', '76ACDD', '86B8DF']
PALETTE = [b.rgb(c) + (255,) for c in COLORS]


def waterfall(width, direction, frame):
    im = Image.new('RGBA', (16, 32))
    left = (16 - width) // 2
    for y in range(29):
        for x in range(left, left + width):
            tone = 1 if x == left else 2 if x < 8 else 3
            if (y - frame * 2 + x * 3) % 13 < 3:
                tone = min(5, tone + 1)
            if direction == 'left' and x < 8:
                tone = max(0, tone - 1)
            im.putpixel((x, y), PALETTE[tone])
    d = ImageDraw.Draw(im)
    d.ellipse((max(0, left - 2), 27, min(15, left + width + 1), 31), fill=PALETTE[3])
    d.line((left, 29, left + width - 1, 29), fill=PALETTE[4], width=1)
    return im


def stream_strip(frame):
    im = Image.new('RGBA', (16, 32))
    for y in range(32):
        for x in range(5, 11):
            tone = 1 if x == 5 else 2 if x < 9 else 3
            if (y - frame * 2 + x) % 8 < 2:
                tone += 1
            im.putpixel((x, y), PALETTE[tone])
    return im


def water_overlay(t, x, y, frame):
    if t['flow'] < 60:return Image.new('RGBA',(64,32))
    points = []
    for key in ('upstream', 'downstream'):
        p = t.get(key)
        points.append((32 + (p[0]-x-p[1]+y)*16, 16 + (p[0]-x+p[1]-y)*8) if p else (32,16))
    start, end = points
    curve = []
    for i in range(49):
        f = i / 48
        curve.append(tuple(round((1-f)**2*start[j] + 2*f*(1-f)*(32,16)[j] + f*f*end[j]) for j in (0,1)))
    width = t['visual_width']
    im = Image.new('RGBA', (64,32));d = ImageDraw.Draw(im)
    # Feuchter Fels und Erdboden wechseln mit dem vorhandenen Felsanteil.
    rock=max(0,min(1,t['rock']))
    bank=tuple(round(a*rock+c*(1-rock)) for a,c in zip(b.rgb('676B66'),b.rgb('675E42'))) + (255,)
    d.line(curve, fill=bank, width=width+4, joint='curve')
    d.line(curve, fill=PALETTE[1], width=width, joint='curve')
    d.line(curve, fill=PALETTE[2], width=max(1,width-2), joint='curve')
    for i in range(3+frame,45,9):
        d.line(curve[i:i+3], fill=PALETTE[3], width=1)
    alpha = np.array(im.getchannel('A'));alpha[np.array(b.diamond_mask())==0] = 0
    im.putalpha(Image.fromarray(alpha))
    return im


def fixture(kind):
    n=8;fields=land.make_fields(n,'meadow')
    for y,row in enumerate(fields):
        for x,t in enumerate(row):
            distance=x+y
            rock=1-land.smooth(3,10,distance)
            t.update(rock=rock, height_px=48 if distance<3 else 32 if distance<6 else 18 if distance<9 else 6,
                     grass=.8*(1-rock),shrub=.18*(1-rock),forest=0,forest_age=35,
                     soil_moisture_fraction=.62+.25*(1-rock),surface_water=0,flow=0)
            t.pop('upstream',None);t.pop('downstream',None)
            if kind=='forest' and distance>7:
                t.update(forest=.48,forest_age=75,grass=.25,shrub=.12)
            if kind=='wetland' and (x-6.2)**2+(y-6.1)**2<2.4:
                t.update(surface_water=260,grass=0,shrub=0)
    route=[(0,0),(1,0),(1,1),(2,1),(2,2),(3,2),(3,3),(4,3),(4,4),(5,4),(5,5),(6,5),(6,6),(7,6)]
    for i,(x,y) in enumerate(route):
        t=fields[y][x]
        width=3 if kind=='stream' else 7
        t.update(flow=85 if kind=='stream' else 240,visual_width=width,
                 upstream=list(route[i-1]) if i else None,
                 downstream=list(route[i+1]) if i+1<len(route) else None)
    return fields


def fall_draw(frame):
    def draw(canvas,fields,t,x,y,cx,cy):
        if t['flow']<60:return
        p=t.get('downstream')
        if not p:return
        target=fields[p[1]][p[0]]
        drop=t['height_px']-target['height_px']
        if drop<=0 or target['flow']<60:return
        dx,dy=p[0]-x,p[1]-y
        ex=cx+(dx-dy)*16;ey=cy+(dx+dy)*8
        direction='left' if dy else 'right'
        width=t['visual_width']
        sprite=waterfall(width,direction,frame).resize((16,drop+4),Image.Resampling.NEAREST)
        canvas.alpha_composite(sprite,(ex-8,ey-1))
    return draw


def main():
    OUT.mkdir(parents=True,exist_ok=True)
    assets=[];validation=[];identities=set()
    for name,width in [('stream',3),('river',7)]:
        for direction in ('left','right'):
            frames=[waterfall(width,direction,f) for f in range(4)]
            filenames=[]
            for f,im in enumerate(frames):
                filename=f'waterfall_{name}_{direction}_f{f}.png'
                im.save(OUT/filename,icc_profile=b.PROFILE);filenames.append(filename)
                assert im.tobytes() not in identities
                identities.add(im.tobytes())
            assets.append(dict(id=f'waterfall_{name}_{direction}',size=[16,32],anchor=[8,0],files=filenames,frames=4,frame_ms=400,role='vertical_drop',nominal_drop_px=28))
            validate_frames(frames,filenames,validation)
    frames=[stream_strip(f) for f in range(4)]
    filenames=[]
    for f,im in enumerate(frames):
        filename=f'water_stream_f{f}.png';im.save(OUT/filename,icc_profile=b.PROFILE);filenames.append(filename)
        assert im.tobytes() not in identities;identities.add(im.tobytes())
    assets.append(dict(id='water_stream',size=[16,32],anchor=[8,16],frames=4,frame_ms=400,files=filenames,longitudinal_repeat_px=8))
    validate_frames(frames,filenames,validation)
    materials=land.load_materials()
    validate_connections()
    sheets=[]
    kinds=[('stream','Gebirgsbach → Wiese'),('river','Gebirgsfluss → Wiese'),('forest','Fels → Waldboden'),('wetland','Fels → Feuchtgebiet')]
    for frame in range(4):
        board=Image.new('RGB',(1820,1420),b.rgb('0C1A34'));d=ImageDraw.Draw(board)
        d.text((40,24),'GEBIRGSWASSER / RUNDE 08',font=b.font(27,True),fill='#E5E4D4')
        d.text((40,70),'Bäche · Flüsse · Wasserfälle · Übergänge zwischen Fels und bewachsenen Böden',font=b.font(21),fill='#B9C6BB')
        for i,(kind,title) in enumerate(kinds):
            fields=fixture(kind)
            scene=land.render(fields,materials,lambda t,x,y:water_overlay(t,x,y,frame),fall_draw(frame))
            x=25+(i%2)*900;y=140+(i//2)*610
            d.text((x+16,y),title,font=b.font(26,True),fill='#E5E4D4')
            zoom=scene.resize((840,534),Image.Resampling.NEAREST)
            board.paste(zoom,(x,y+44),zoom)
            if frame==0:
                scene.save(OUT/f'{kind}-scene.png',icc_profile=b.PROFILE)
                (OUT/f'{kind}-layout.json').write_text(json.dumps(fields,indent=2),encoding='utf-8')
        d.text((40,1370),'Feste Prüfbelegungen · Animation 4 × 400 ms · noch keine Spielintegration',font=b.font(21),fill='#B9C6BB')
        sheets.append(board)
    sheets[0].save(b.ROOT/'assets/nature/_mountain-water.png',icc_profile=b.PROFILE)
    sheets[0].save(b.ROOT/'assets/nature/_mountain-water.gif',save_all=True,append_images=sheets[1:],duration=400,loop=0,disposal=2)
    manifest=dict(version=1,status='design_review',assets=assets,fixtures=[k for k,t in kinds],
                  rules='Wasser nur auf durchflossenen Testfeldern; Fall nur zum durchflossenen, tieferen Nachbarn.',
                  integration='visual_width und height_px sind Prüfwerte, keine neuen Simulationsschwellen; Höhen und Breite später vom Renderer ableiten.')
    (OUT/'manifest.json').write_text(json.dumps(manifest,indent=2,ensure_ascii=False),encoding='utf-8')
    (OUT/'validation.json').write_text(json.dumps(validation,indent=2),encoding='utf-8')
    gif=Image.open(b.ROOT/'assets/nature/_mountain-water.gif')
    assert gif.n_frames==4
    for f in range(4):gif.seek(f);assert gif.info['duration']==400
    print('20 Wasser-PNGs, vier zusammenhängende Prüfbelegungen, vier Animationsphasen; Paletten, Transparenz und Helligkeitsruhe geprüft.')


def validate_connections():
    fields=fixture('river')
    for row in fields:
        for t in row:
            if t['flow']<60:continue
            p=t.get('downstream')
            if p:
                target=fields[p[1]][p[0]]
                assert target['upstream']==[t['x'],t['y']]
                assert abs(p[0]-t['x'])+abs(p[1]-t['y'])==1
                assert t['height_px']>=target['height_px']
            im=water_overlay(t,t['x'],t['y'],0)
            assert np.all(np.array(im)[:,:,3][np.array(b.diamond_mask())==0]==0)
    t=fields[0][0].copy();t['flow']=0
    assert water_overlay(t,0,0,0).getbbox() is None
    # Keine Kaskade ohne Gefälle oder ohne Durchfluss.
    t=fields[1][1].copy();p=t['downstream']
    t['height_px']=fields[p[1]][p[0]]['height_px']
    canvas=Image.new('RGBA',(160,160))
    fall_draw(0)(canvas,fields,t,1,1,80,60)
    assert canvas.getbbox() is None
    t['height_px']+=20;t['flow']=0
    fall_draw(0)(canvas,fields,t,1,1,80,60)
    assert canvas.getbbox() is None
    t['flow']=240
    fall_draw(0)(canvas,fields,t,1,1,80,60)
    assert canvas.getbbox() is not None


def validate_frames(frames,files,report):
    arrays=[np.array(im) for im in frames]
    opaque=arrays[0][:,:,3]>0
    for im,pixels in zip(frames,arrays):
        assert im.size==(16,32)
        assert len({tuple(p[:3]) for p in pixels.reshape(-1,4) if p[3]})<=6
        assert set(np.unique(pixels[:,:,3]))<={0,255}
        assert np.array_equal(pixels[:,:,3]>0,opaque)
    weights=np.array([.2126,.7152,.0722])
    light=np.stack([p[:,:,:3]@weights for p in arrays])[:,opaque]
    variation=(light.max(axis=0)-light.min(axis=0))/light.min(axis=0)
    assert variation.max()<=.15,float(variation.max())
    assert len({im.tobytes() for im in frames})==4
    report.append(dict(files=files,max_pixel_luminance_variation=float(variation.max()),constant_silhouette=True,frames=4,frame_ms=400))


if __name__=='__main__':main()
