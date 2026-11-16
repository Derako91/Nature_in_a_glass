"""Vier generierte Feuerphasen technisch aufbereiten, ohne Spielassets zu ersetzen."""
import argparse
import hashlib
import json
from pathlib import Path
from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT/'art/nature/fire-review'

def build(source, kind='small'):
    global OUT
    config = {
        'small': ('fire-review','fire_flame_small',(40,64),(10,16),'Niedriges Feuer'),
        'large': ('fire-large-review','fire_flame_large',(64,112),(16,28),'Großes Feuer'),
        'smoke': ('smoke-review','smoke',(96,128),(24,32),'Rauch'),
        'ground': ('fire-ground-review','fire_ground',(256,128),(64,32),'Glutsaum'),
    }
    folder,asset,size,world,label=config[kind]
    OUT=ROOT/'art/nature'/folder
    w,h=size; anchor=(w//2,h-2) if kind!='ground' else (w//2,h//2)
    if OUT.exists():
        raise ValueError('Prüfverzeichnis existiert bereits; keine Überschreibung.')
    src=Image.open(source).convert('RGBA')
    cuts=[]
    for i in range(4):
        cell=src.crop((round(i*src.width/4),0,round((i+1)*src.width/4),src.height))
        box=cell.getchannel('A').point(lambda a:255 if a>16 else 0).getbbox()
        if box is None: raise ValueError('Leere Phase')
        cuts.append(cell.crop(box))
    factor=min((w-4)/max(im.width for im in cuts),(h-4)/max(im.height for im in cuts))
    OUT.mkdir(parents=True)
    icc=Image.open(ROOT/'assets/nature/ground/ground_soil_0.png').info.get('icc_profile')
    frames=[];entries=[];brightness=[]
    for i,cut in enumerate(cuts):
        cut=cut.resize((max(1,round(cut.width*factor)),max(1,round(cut.height*factor))),Image.Resampling.LANCZOS)
        im=Image.new('RGBA',size)
        y=h-2-cut.height if kind!='ground' else (h-cut.height)//2
        im.alpha_composite(cut,((w-cut.width)//2,y))
        if kind=='smoke':
            im.putalpha(im.getchannel('A').point(lambda a:round(a*.55)))
        if kind=='ground':
            # Bodenüberlagerung auf die verbindliche 2:1-Raute begrenzen.
            mask=Image.new('L',size); ImageDraw.Draw(mask).polygon([(w//2,0),(w-1,h//2),(w//2,h-1),(0,h//2)],fill=255)
            from PIL import ImageChops
            im.putalpha(ImageChops.multiply(im.getchannel('A'),mask))
        name=f'{asset}_0_f{i}.png';im.save(OUT/name,icc_profile=icc)
        frames.append(im)
        pixels=list(im.getdata())
        brightness.append(sum((.2126*r+.7152*g+.0722*b)*a/255 for r,g,b,a in pixels))
        entries.append({'id':asset,'files':[name],'anchor':anchor,
                        'display_size':world,'frame_duration_ms':200})
    raw_brightness=brightness.copy()
    alpha_factors=[1.0]*4
    if kind=='smoke':
        # Dichtere Phasen nur abdämpfen, damit die Wolke nicht aufblitzt.
        floor=min(brightness)
        for i,im in enumerate(frames):
            alpha_factors[i]=floor/brightness[i]
            im.putalpha(im.getchannel('A').point(lambda a:round(a*alpha_factors[i])))
            im.save(OUT/f'{asset}_0_f{i}.png',icc_profile=icc)
            brightness[i]=sum((.2126*r+.7152*g+.0722*b)*a/255 for r,g,b,a in im.getdata())
    spread=(max(brightness)-min(brightness))/min(brightness)
    (OUT/'manifest.json').write_text(json.dumps({'assets':entries,'frame_aliases':{},
        'status':'review_only_not_installed','loop_ms':800},indent=2)+'\n',encoding='utf-8')
    report={'format':'RGBA8','size':size,'frames':4,'loop_ms':800,
        'anchor':anchor,'display_size':world,
        'source_sha256':hashlib.sha256(Path(source).read_bytes()).hexdigest(),
        'common_scale':factor,'integrated_luminance':brightness,
        'raw_integrated_luminance':raw_brightness,'alpha_factors':alpha_factors,
        'max_min_luminance_change':spread,'brightness_limit_25_percent':spread<=.25,
        'visual_review':'pending','game_review':'pending','producer':'built-in image_gen'}
    (OUT/'validation.json').write_text(json.dumps(report,indent=2)+'\n',encoding='utf-8')
    font=lambda n:ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',n)
    sheet=Image.new('RGB',(1000,700),'#19292b');d=ImageDraw.Draw(sheet)
    d.text((24,15),label+' – vier Phasen / 0,8 Sekunden',font=font(27),fill='#eee5ce')
    d.text((24,57),'Prüfentwurf, keine Spielaufnahme. Gemeinsamer Maßstab und Fußpunkt.',font=font(18),fill='#c2cdc7')
    for i,im in enumerate(frames):
        panel=Image.new('RGBA',(220,290),'#35432e')
        preview=im.resize((round(w*min(190/w,256/h)),round(h*min(190/w,256/h))),Image.Resampling.NEAREST)
        panel.alpha_composite(preview,((220-preview.width)//2,(280-preview.height)//2))
        sheet.paste(panel.convert('RGB'),(24+i*240,100))
        d.text((24+i*240,400),f'Phase {i+1}',font=font(18),fill='#eee5ce')
    grounds=['grass_wet','shrub_floor','forest_floor'];gif=[]
    for fi,im in enumerate(frames):
        strip=Image.new('RGBA',(960,210),'#283b2d')
        for c,g in enumerate(grounds):
            tile=Image.open(ROOT/f'assets/nature/ground/ground_{g}_0.png').convert('RGBA').resize((256,128),Image.Resampling.NEAREST)
            strip.alpha_composite(tile,(c*320+32,70))
            strip.alpha_composite(im,(c*320+160-anchor[0],130-anchor[1]))
        gif.append(strip.convert('RGB'))
    sheet.paste(gif[0],(24,450))
    d.text((24,666),'Unten: Zoom 16 auf Gras, Busch- und Waldboden. Animation als separate GIF-Vorschau.',font=font(17),fill='#c2cdc7')
    sheet.save(OUT/'comparison.png',icc_profile=icc)
    gif[0].save(OUT/'animation.gif',save_all=True,append_images=gif[1:],duration=200,loop=0)
    print(json.dumps(report))

if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('source',type=Path)
    p.add_argument('--kind',choices=['small','large','ground','smoke'],default='small')
    args=p.parse_args();build(args.source,args.kind)
