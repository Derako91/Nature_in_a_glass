"""Wandwasser nach 16.9: gemalte Materialien, feste Profile, ruhige vier Phasen."""
import hashlib
import json
from pathlib import Path

import numpy as np
from PIL import Image, ImageDraw
from export_water_style_b import periodic
from export_stream_beds import material

ROOT = Path(__file__).resolve().parents[2]
ART = ROOT / 'art/nature/stream-drops-style-b'
WATER = ROOT / 'assets/nature/water'


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def patch(source, col, row, size):
    w, h = source.size
    box = (round(col*w/3)+4, round(row*h/2)+4,
           round((col+1)*w/3)-4, round((row+1)*h/2)-4)
    return periodic(np.asarray(source.crop(box).resize(size, Image.Resampling.LANCZOS)).astype(float))


def rgba(color, alpha):
    data = np.empty((*color.shape[:2],4),dtype=np.uint8)
    data[:,:,:3] = np.clip(np.rint(color),0,255).astype(np.uint8)
    data[:,:,3] = np.rint(alpha*255).astype(np.uint8)
    data[data[:,:,3] == 0] = 0
    return Image.fromarray(data)


def export():
    cfg = json.loads((ART/'config.json').read_text())
    path = ART/cfg['source']['file']
    assert sha(path) == cfg['source']['sha256'], 'Fallquelle geändert'
    source = Image.open(path).convert('RGB')
    assert cfg['body_size'] == [32,64] and cfg['cap_size'] == [32,16] and cfg['frame_ms'] == 500
    bed_cfg_path = ART/cfg['bed_config']
    bed_cfg = json.loads(bed_cfg_path.read_text())
    bed_source_path = bed_cfg_path.parent/bed_cfg['source']['file']
    assert sha(bed_source_path) == bed_cfg['source']['sha256']
    bed_source = Image.open(bed_source_path).convert('RGB')
    water_rgb = np.array(bed_cfg['water_rgb'],dtype=float)
    x = (np.arange(32)+.5)/16-1
    water_texture = patch(source,2,0,(32,64))
    foam_texture = patch(source,2,1,(32,16))
    entries, records, images = [], [], {}
    for state in ('wet','dry'):
        for strength in range(3):
            ratio = bed_cfg['water_radii_uv'][strength]/bed_cfg['bed_radii_uv'][strength]
            channel = np.clip((ratio+.05-np.abs(x))/.10,0,1)[:,None]
            mean = material(bed_source,1 if state=='wet' else 0,strength).mean(axis=(0,1))
            stream_bank = mean[None,:]*(1+.07*x[:,None])
            stream_profile = (stream_bank*(1-channel)+water_rgb*channel if state=='wet'
                              else stream_bank*(1-.12*channel))
            for side in ('left','right'):
                bank = patch(source,0 if side=='left' else 1,0 if state=='wet' else 1,(32,64))
                bank_mean = mean*(.80 if side=='left' else .93)
                bank_profile = bank_mean[None,:]*(1+.07*x[:,None])
                body_profile = (bank_profile*(1-channel)+water_rgb*channel if state=='wet'
                                else bank_profile*(1-.25*channel))
                for part in ('head','body','foot'):
                    height = 64 if part=='body' else 16
                    t = np.linspace(0,1,height)[:,None,None]
                    envelope = np.sin(np.pi*t)**2
                    # Unregelmäßige Ränder innen, unveränderte Anschlussprofile.
                    wobble = .025*np.sin(np.arange(height)[:,None]*.65+strength+(0 if side=='left' else 1))*envelope[:,:,0]
                    alpha = np.clip((1-np.abs(x)[None,:]+wobble)/.10,0,1)
                    names = []
                    for frame in range(4 if state=='wet' else 1):
                        if part=='body':
                            base = np.broadcast_to(body_profile,(height,32,3)).copy()
                            bank_detail = (bank-bank.mean(axis=(0,1)))*.65
                            detail = bank_detail*(1-channel[None,:,:])
                            if state=='wet':
                                moved = np.roll(water_texture,frame*16,axis=0)
                                detail += (moved-moved.mean(axis=(0,1)))*.65*channel[None,:,:]
                                luma = moved @ np.array([.2126,.7152,.0722])
                                threshold = np.quantile(luma,.85)
                                glint = np.clip((luma-threshold)/max(1,luma.max()-threshold),0,1)
                                detail += glint[:,:,None]*np.array([110,85,60])*channel[None,:,:]
                            else:
                                detail *= 1-.12*channel[None,:,:]
                            color = base+detail*envelope
                            color[0] = color[-1] = body_profile
                        else:
                            start,end = ((stream_profile,body_profile) if part=='head'
                                         else (body_profile,stream_profile))
                            color = start[None,:,:]*(1-t)+end[None,:,:]*t
                            detail = (bank[:16]-bank[:16].mean(axis=(0,1)))*.25*(1-channel[None,:,:])
                            if state=='wet':
                                moved = np.roll(foam_texture,frame*4,axis=0)
                                # Heller Umbruch/kleiner Aufprallsaum, keine weiße Lichtwand.
                                detail += ((moved-moved.mean(axis=(0,1)))*.28
                                           +(14 if part=='head' else 30))*channel[None,:,:]
                            else:
                                detail *= 1-.12*channel[None,:,:]
                            color += detail*envelope
                            color[0],color[-1] = start,end
                        image = rgba(color,alpha)
                        name = f'stream_drop_{state}_{strength}_{side}_{part}_f{frame}'
                        path = WATER/(name+'.png')
                        image.save(path)
                        images[name] = image
                        names.append(path.name)
                        records.append({'file':path.name,'sha256':sha(path),'size':[32,height],
                                        'anchor':[16,0],'display_size':[8,height/4],
                                        'water_bed_ratio':ratio})
                    entries.append({'id':f'stream_drop_{state}_{strength}_{side}_{part}',
                        'group':'water','size':[32,height],'anchor':[16,0],
                        'display_size':[8,height/4],'resolution_scale':4,
                        'frames':4 if state=='wet' else 1,'frame_ms':500 if state=='wet' else None,
                        'status':'stream_drop_style_b_draft','files':names})
    path = WATER/'manifest.json'
    manifest = json.loads(path.read_text())
    manifest['assets'] = [a for a in manifest['assets'] if not a['id'].startswith('stream_drop_')]+entries
    path.write_text(json.dumps(manifest,ensure_ascii=False,indent=2)+'\n')
    (ART/'export.json').write_text(json.dumps({'files':records,'visual_approval':'pending'},indent=2)+'\n')
    proofs(images)
    print('90 Wandwasserbilder: 72 nass (4×500 ms), 18 trocken, feste Kopf/Rinne/Fuß-Profile.')


def proofs(images):
    frames = []
    for frame in range(4):
        sheet = Image.new('RGBA',(1200,800),'#514b40')
        draw = ImageDraw.Draw(sheet)
        for row,state in enumerate(('wet','dry')):
            for strength in range(3):
                for side_index,side in enumerate(('left','right')):
                    col = strength*2+side_index
                    draw.text((col*200+10,row*400+8),f'{state} / {strength} / {side}',fill='white')
                    f = frame if state=='wet' else 0
                    prefix = f'stream_drop_{state}_{strength}_{side}_'
                    stack = Image.new('RGBA',(32,160))
                    y = 0
                    for part in ('head','body','body','foot'):
                        im = images[prefix+part+f'_f{f}'];stack.alpha_composite(im,(0,y));y+=im.height
                    sheet.alpha_composite(stack.resize((64,320),Image.Resampling.NEAREST),(col*200+68,row*400+40))
        frames.append(sheet.convert('RGB'))
    frames[0].save(ART/'joins-contact.png')
    frames[0].save(ART/'animation.gif',save_all=True,append_images=frames[1:],duration=500,loop=0,disposal=2)


if __name__ == '__main__':
    export()
