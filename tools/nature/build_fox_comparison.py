"""Technischer Export der freigegebenen Bildreferenz, keine neue Zeichnung."""
import hashlib
import json
from pathlib import Path
from PIL import Image, ImageDraw, ImageFont, ImageCms

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'art/nature/fox-ab'
SOURCE = OUT / 'fox-b-imagegen-source.png'
OLD = ROOT / 'assets/nature/animals/fox_0_graze_r_f0.png'
ICC = ImageCms.ImageCmsProfile(ImageCms.createProfile('sRGB')).tobytes()

def font(n):
    return ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf', n)

def main():
    src = Image.open(SOURCE).convert('RGBA')
    # Unsichtbare Generierungsreste außerhalb des Motivs nicht mitskalieren.
    bbox = src.getchannel('A').point(lambda a: 255 if a > 8 else 0).getbbox()
    cropped = src.crop(bbox)
    # Der blaugraue Schlagschatten wird mit 30 % Deckkraft exportiert.
    pixels = cropped.load()
    for y in range(cropped.height):
        for x in range(cropped.width):
            r,g,b,a = pixels[x,y]
            if b > r and b > g and r < 80:
                pixels[x,y] = (12,26,52,round(a*0.3))
    cropped.thumbnail((76, 40), Image.Resampling.LANCZOS)
    sprite = Image.new('RGBA', (80, 48))
    sprite.alpha_composite(cropped, ((80-cropped.width)//2, 40-cropped.height))
    target = OUT / 'fox_0_graze_r_f0.png'
    sprite.save(target, icc_profile=ICC)
    old = Image.open(OLD).convert('RGBA')
    manifest = {'status':'single_frame_review_not_installed', 'assets':[
        {'id':'fox','variant':0,'state':'graze','direction':'r','frame':0,
         'file':target.name,'size':[80,48],'display_size':[10,6],
         'anchor':[40,40],'resolution_scale':8}],
        'reference':SOURCE.name,'baseline':'assets/nature/animals/'+OLD.name,
        'note':'Ein Standbild. Erst nach Freigabe alle Bilder derselben Art konsistent umstellen.'}
    (OUT/'manifest.json').write_text(json.dumps(manifest,indent=2,ensure_ascii=False)+'\n',encoding='utf-8')
    sheet = Image.new('RGB',(1320,1050),'#102132')
    d = ImageDraw.Draw(sheet)
    d.text((30,20),'Fuchs: Projektstand A / neuer Stil B',font=font(30),fill='#ECE3CC')
    d.text((30,65),'Synthetischer Vergleich – keine Spielaufnahme. Gleiches Weltmaß und gleicher Fußpunkt.',font=font(19),fill='#B7C5C6')
    grounds = [('Gras','ground_grass_wet'),('Buschboden','ground_shrub_floor'),('Waldboden','ground_forest_floor')]
    for col,(label,name) in enumerate(grounds):
        x=30+col*430
        d.text((x,112),label,font=font(23),fill='#ECE3CC')
        ground=Image.open(ROOT/f'assets/nature/ground/{name}_0.png').convert('RGBA')
        for row,zoom in enumerate((4,8,16,32)):
            y=155+row*200
            d.text((x,y),f'Zoom {zoom} – A links / B rechts',font=font(17),fill='#B7C5C6')
            tile=ground.resize((int(16*zoom),int(8*zoom)),Image.Resampling.NEAREST)
            for k,(im,anchor) in enumerate(((old,(10,10)),(sprite,(40,40)))):
                panel=Image.new('RGBA',(195,160),'#26372E')
                panel.alpha_composite(tile,((195-tile.width)//2,(160-tile.height)//2))
                scale=10*zoom/4/im.width
                displayed=im.resize((round(im.width*scale),round(im.height*scale)),Image.Resampling.NEAREST)
                panel.alpha_composite(displayed,(98-round(anchor[0]*scale),92-round(anchor[1]*scale)))
                sheet.paste(panel.convert('RGB'),(x+k*205,y+30))
    d.text((30,1000),'B: Bildgenerator-Referenz, technisch verkleinert. A: bestehender Zeichenexport aus Runde 09.',font=font(18),fill='#B7C5C6')
    sheet.save(OUT/'comparison.png',icc_profile=ICC)
    assert sprite.size==(80,48) and sprite.mode=='RGBA'
    assert (40/80,40/80)==(10/20,10/20)
    report={'source_size':src.size,'source_bbox':bbox,'output_size':sprite.size,
            'visible_rgb_colors':len({p[:3] for p in sprite.getdata() if p[3]}),
            'alpha_levels':len(set(sprite.getchannel('A').getdata())),
            'world_size':[10,6],'world_anchor':[5,5],
            'source_sha256':hashlib.sha256(SOURCE.read_bytes()).hexdigest(),
            'baseline_sha256':hashlib.sha256(OLD.read_bytes()).hexdigest(),
            'game_review':'pending','animation':'not_in_scope'}
    (OUT/'validation.json').write_text(json.dumps(report,indent=2)+'\n',encoding='utf-8')
    print(json.dumps(report,indent=2))

if __name__=='__main__':
    main()
