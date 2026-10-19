"""Prüfexport eines generierten Fernentwurfs; keine Installation in assets/."""
import argparse
import json
import hashlib
import math
from pathlib import Path
from PIL import Image, ImageDraw, ImageFont, ImageCms

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'art/nature/fox-far-review'
MAIN = ROOT / 'art/nature/fox-ab/fox_0_graze_r_f0.png'
NAME = 'fox_0_graze_r_f0'
ICC = ImageCms.ImageCmsProfile(ImageCms.createProfile('sRGB')).tobytes()

def halve(im):
    """Vorschau nach atlas.rs::halve v0.7.4, kein Ersatz für Spielprüfung."""
    out = Image.new('RGBA', ((im.width+1)//2, (im.height+1)//2))
    for y in range(out.height):
        for x in range(out.width):
            ps = [im.getpixel((min(x*2+dx, im.width-1), min(y*2+dy, im.height-1)))
                  for dy in range(2) for dx in range(2)]
            a = sum(p[3]/255 for p in ps)
            rgb = [math.floor(sum(p[c]*p[3]/255 for p in ps)/a+0.5) if a else 0 for c in range(3)]
            alpha = math.floor(max(0, min(1, (a/4-0.25)*2))*255+0.5)
            out.putpixel((x,y), (*rgb, alpha))
    return out

def build(source=None, export=None):
    OUT.mkdir(parents=True, exist_ok=True)
    target = OUT / (NAME+'@fern.png')
    if source:
        if target.exists():
            raise ValueError('Entwurf existiert bereits; keine automatische Überschreibung.')
        src = Image.open(source).convert('RGBA')
        bbox = src.getchannel('A').point(lambda a:255 if a>128 else 0).getbbox()
        if bbox is None: raise ValueError('Leeres Motiv')
        cut = src.crop(bbox)
        cut.thumbnail((19,10), Image.Resampling.LANCZOS)
        # Kanten des absichtlich gröberen Entwurfs technisch stabilisieren.
        cut.putalpha(cut.getchannel('A').point(lambda a:255 if a>=128 else 0))
        far = Image.new('RGBA',(20,12))
        far.alpha_composite(cut, ((20-cut.width)//2,10-cut.height))
        far.save(target, icc_profile=ICC)
        (OUT/'provenance.json').write_text(json.dumps({
            'method':'built-in image_gen, reference-guided simplified redesign',
            'source_sha256':hashlib.sha256(Path(source).read_bytes()).hexdigest(),
            'reference':'art/nature/fox-ab/fox-b-imagegen-source.png',
            'source_bbox':bbox,'fit':[19,10],'alpha_threshold':128,
            'direction':'r','state':'graze','frame':0,
            'status':'candidate_not_approved'},indent=2)+'\n',encoding='utf-8')
    far = Image.open(target).convert('RGBA')
    main = Image.open(MAIN).convert('RGBA')
    auto40 = halve(main); auto20 = halve(auto40); auto10 = halve(auto20)
    font = lambda n:ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',n)
    sheet = Image.new('RGB',(1200,1000),'#18262b'); d=ImageDraw.Draw(sheet)
    d.text((24,15),'Fuchs B – Fernentwurf',font=font(28),fill='#eee7d5')
    d.text((24,55),'Links: berechnete Stufen | rechts: eigener Fernentwurf. Synthetisch, keine Spielaufnahme.',font=font(18),fill='#bed0cc')
    grounds = ['grass_wet','shrub_floor','forest_floor']
    for col, ground in enumerate(grounds):
        x=24+col*392
        d.text((x,95),['Gras','Buschboden','Waldboden'][col],font=font(20),fill='#eee7d5')
        tile=Image.open(ROOT/f'assets/nature/ground/ground_{ground}_0.png').convert('RGBA')
        for row,zoom in enumerate((4,8,16,32)):
            y=130+row*174
            d.text((x,y),f'Zoom {zoom}',font=font(17),fill='#bed0cc')
            # Regel: feinste Stufe, die höchstens 1,5-fach schrumpft.
            for k,levels in enumerate(([main,auto40,auto20,auto10],[main,auto40,far])):
                chosen=next((im for im in levels if im.width<=10*zoom/4*1.5),levels[-1])
                panel=Image.new('RGBA',(182,140),'#344532')
                t=tile.resize((16*zoom,8*zoom),Image.Resampling.NEAREST)
                panel.alpha_composite(t,((182-t.width)//2,(140-t.height)//2))
                shown=chosen.resize((round(10*zoom/4),round(6*zoom/4)),Image.Resampling.NEAREST)
                panel.alpha_composite(shown,(91-round(5*zoom/4),85-round(5*zoom/4)))
                sheet.paste(panel.convert('RGB'),(x+k*190,y+24))
    # Stark vergrößerte technische Ansicht für die Pixelkontrolle.
    sheet.paste(auto20.resize((160,96),Image.Resampling.NEAREST),(24,858))
    sheet.paste(far.resize((160,96),Image.Resampling.NEAREST),(220,858))
    d.text((410,867),'20 × 12 px, 8-fach gezeigt: automatisch / Entwurf',font=font(18),fill='#eee7d5')
    d.text((410,900),'Ab Zoom 16 verwenden beide Spalten das gleiche feine Bild.',font=font(17),fill='#bed0cc')
    d.text((410,931),'Freigabe und Prüfung im Spiel stehen aus.',font=font(17),fill='#bed0cc')
    sheet.save(OUT/'comparison.png',icc_profile=ICC)
    assert far.size==(20,12) and far.getextrema()[3]==(0,255)
    assert main.size == (80,48)
    assert tuple(v/far.width for v in (10,10)) == tuple(v/main.width for v in (40,40))
    report={'technical':'passed','visual':'pending_user_review','game_review':'pending',
            'size':[20,12],'anchor':[10,10],'display_size':[10,6],
            'opaque_bbox':far.getchannel('A').getbbox(),
            'scope':'one right-facing reference frame; no animation or left view',
            'comparison':'Python approximation of v0.7.4 LOD; not an engine screenshot'}
    (OUT/'validation.json').write_text(json.dumps(report,indent=2)+'\n',encoding='utf-8')
    if export:
        dest=Path(export)/'animals';dest.mkdir(parents=True,exist_ok=False)
        # Unverändertes Nahbild nur im ignorierten lokalen Prüfexport duplizieren.
        (dest/(NAME+'.png')).write_bytes(MAIN.read_bytes())
        (dest/(NAME+'@fern.png')).write_bytes(target.read_bytes())
        (dest/'manifest.json').write_text(json.dumps({'assets':[{
            'id':'fox','files':[NAME+'.png'],'anchor':[40,40],'display_size':[10,6],
            'levels':[{'label':'@fern','files':[NAME+'@fern.png'],'anchor':[10,10]}]
        }],'frame_aliases':{}},indent=2)+'\n',encoding='utf-8')
    print(json.dumps(report))

if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--source',type=Path);p.add_argument('--export',type=Path)
    args=p.parse_args();build(args.source,args.export)
