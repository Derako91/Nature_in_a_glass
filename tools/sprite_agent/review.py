"""Technische Abnahme getrennt von menschlicher Bildbewertung."""
import html
import math
import numpy as np
from PIL import Image, ImageDraw, ImageFont
from .model import write, digest, read
from .imaging import save

def font(size):
    for name in ('C:/Windows/Fonts/segoeui.ttf','DejaVuSans.ttf'):
        try:return ImageFont.truetype(name,size)
        except OSError:pass
    return ImageFont.load_default()

def validate_package(root, manifest, expected, persist=True):
    errors=[]; warnings=[]; hashes={}
    if len(manifest['frames'])!=len(expected):
        errors.append('Unvollständige Anzahl Frames.')
    if {f['key'] for f in manifest['frames']}!={f['key'] for f in expected}:
        errors.append('Frame-Zuordnung unvollständig oder falsch.')
    job=read(root/'job.json')
    expected_lods={l['id']:l for l in job['lods']}
    expected_frames={f['key']:f for f in expected}
    if manifest['display_size']!=job['display_size'] or manifest['anchor']!=job['anchor']:
        errors.append('Weltmaß/Anker weicht vom Auftrag ab.')
    lod_metrics=[]
    for frame in manifest['frames']:
        if frame['key'] in expected_frames:
            for field in ('state','variant','direction','frame','frame_ms'):
                if frame[field]!=expected_frames[frame['key']][field]:errors.append('Frame-Metadaten falsch: '+frame['key'])
        if len(frame['lods'])!=len(expected_lods) or {l['id'] for l in frame['lods']}!=set(expected_lods):
            errors.append('LOD-Zuordnung unvollständig: '+frame['key'])
        for lod in frame['lods']:
            schema=expected_lods.get(lod['id'])
            if schema is None or lod['scale']!=schema['scale'] or lod['zoom_min']!=schema['zoom_min']:
                errors.append('LOD-Schema falsch: '+lod['file']);continue
            if lod['size']!=[round(n*schema['scale']) for n in job['display_size']]:
                errors.append('LOD-Maße widersprechen Weltmaß: '+lod['file'])
            path=root/lod['file']
            if not path.resolve().is_relative_to(root.resolve()):
                errors.append('Dateipfad verlässt Paket: '+lod['file']);continue
            if not path.is_file():
                errors.append('Datei fehlt: '+lod['file']);continue
            with Image.open(path) as im:
                if im.format!='PNG' or im.mode!='RGBA' or list(im.size)!=lod['size']:
                    errors.append('Format/Maße falsch: '+lod['file'])
                else:
                    a=np.array(im.getchannel('A'))
                    if not (a==0).any() or not (a>128).any():
                        errors.append('Transparenz oder sichtbares Motiv fehlt: '+lod['file'])
                    if a[0].max()>128 or a[-1].max()>128 or a[:,0].max()>128 or a[:,-1].max()>128:
                        warnings.append('Motiv am Bildrand: '+lod['file'])
                    if lod['scale']<=2 and (root/frame['lods'][-1]['file']).is_file():
                        high=Image.open(root/frame['lods'][-1]['file']).convert('RGBA')
                        nearest=np.array(high.resize(im.size,Image.Resampling.NEAREST).getchannel('A'))>128
                        filtered=a>128
                        union=(nearest|filtered).sum()
                        iou=float((nearest&filtered).sum()/union) if union else 0
                        lod_metrics.append({'file':lod['file'],'nearest_vs_filtered_silhouette_iou':round(iou,3),
                                            'opaque_pixels':int(filtered.sum()),'semantic_readability':'not_measured'})
                        if iou<.7 or filtered.sum()<25:
                            warnings.append('Kleine Silhouette instabil/zu wenig Pixel: eigene LOD zur Sichtprüfung vorschlagen: '+lod['file'])
                if not im.info.get('icc_profile'):
                    errors.append('sRGB-Profil fehlt: '+lod['file'])
            if any(abs(a/lod['scale']-b)>0.001 for a,b in zip(lod['anchor'],manifest['anchor'])):
                errors.append('LOD-Fußpunkt driftet: '+lod['file'])
            h=digest(path)
            if h in hashes:
                warnings.append('Identische Bilddaten: '+lod['file']+' / '+hashes[h])
            hashes[h]=lod['file']
    report={'technical_status':'passed' if not errors else 'failed','errors':errors,
        'warnings':warnings,'visual_status':'pending_human_review','approved':False,
        'required_review':['Stil B getroffen?','Gleiche Figur in allen Ansichten?',
            'Blickrichtungen und Anatomie korrekt?','Fußpunkt tatsächlich zwischen den Füßen?',
            'Helle Fellteile und Kanten sauber freigestellt?',
            'Ohren, Beine und Schwanz bei Zoom 4/8 lesbar?',
            'Bei Animation: plausible Bewegung und kein Größen-/Positionsspringen?'],
        'lod_action':'Kleine Fassungen sind technische Vereinfachungskandidaten. Bei unklarer Silhouette eigene gezeichnete LOD zur Prüfung anfordern.',
        'file_count':sum(len(f['lods']) for f in manifest['frames']), 'lod_metrics':lod_metrics}
    if persist:write(root/'validation.json',report)
    return report

def contact_sheet(root, manifest, backgrounds, reference=None):
    # Seiten statt unbegrenzt hoher Einzelbilder bei größeren Aufträgen.
    panel_w=max(80,math.ceil(manifest['display_size'][0]*8)+8)
    panel_h=max(125,math.ceil(manifest['display_size'][1]*8)+16)
    column_w=3*(panel_w+2)+19
    row_h=panel_h+45
    pages=[]
    for start in range(0,len(manifest['frames']),8):
        frames=manifest['frames'][start:start+8]
        sheet=Image.new('RGB',(200+4*column_w,130+len(frames)*row_h),'#132638'); d=ImageDraw.Draw(sheet)
        d.text((20,12),'Sprite-Agent / '+manifest['asset_id']+' / Freigabe offen',font=font(25),fill='#EEE3CB')
        d.text((20,48),'Synthetische Vorschau – technische Prüfung ist keine gestalterische Freigabe.',font=font(17),fill='#B6C8C8')
        for col,zoom in enumerate((4,8,16,32)):
            d.text((200+col*column_w,84),f'Zoom {zoom}',font=font(19),fill='#EEE3CB')
        for row,frame in enumerate(frames):
            y=125+row*row_h
            d.text((15,y+10),frame['direction']+' / '+frame['state'],font=font(17),fill='#EEE3CB')
            d.text((15,y+36),f"V{frame['variant']} / Bild {frame['frame']}",font=font(15),fill='#B6C8C8')
            for col,zoom in enumerate((4,8,16,32)):
                lod=max((l for l in frame['lods'] if l['zoom_min']<=zoom),key=lambda l:l['zoom_min'])
                sprite=Image.open(root/lod['file']).convert('RGBA')
                factor=zoom/4/lod['scale']
                sprite=sprite.resize(tuple(max(1,round(s*factor)) for s in sprite.size),Image.Resampling.NEAREST)
                for bgindex,bg in enumerate(backgrounds):
                    panel=Image.new('RGBA',(panel_w,panel_h),'#2E3F30')
                    tile=bg.resize((max(1,round(16*zoom)),max(1,round(8*zoom))),Image.Resampling.NEAREST)
                    panel.alpha_composite(tile,((panel_w-tile.width)//2,(panel_h-tile.height)//2))
                    # Platz nach oberem/unterem Ankeranteil, auch für Gebäude.
                    foot_y=max(75,math.ceil(manifest['anchor'][1]*8)+4)
                    panel.alpha_composite(sprite,(panel_w//2-round(lod['anchor'][0]*factor),foot_y-round(lod['anchor'][1]*factor)))
                    sheet.paste(panel.convert('RGB'),(185+col*column_w+bgindex*(panel_w+2),y))
                d.text((185+col*column_w,y+panel_h+4),lod['id']+' · Untergrund 1 / 2 / 3',font=font(13),fill='#B6C8C8')
        name=f'contact-{start//8+1:02}.png';save(sheet,root/name);pages.append(name)
    # Gesonderte große Silhouetten- und Referenzansicht macht Fehlrichtungen sichtbar.
    count=len(manifest['frames'])+(1 if reference else 0)
    overview=Image.new('RGB',(1000,60+math.ceil(count/4)*210),'#DDD6C4');d=ImageDraw.Draw(overview)
    d.text((15,12),'Identität / Richtung / Silhouette – vergrößerte Prüfansicht',font=font(22),fill='#25332D')
    items=[]
    if reference:
        items.append(('Freigegebene Stilreferenz',Image.open(reference).convert('RGBA')))
    for f in manifest['frames']:
        items.append((f['key'],Image.open(root/f['lods'][-1]['file']).convert('RGBA')))
    for i,(label,im) in enumerate(items):
        x=(i%4)*250;y=60+(i//4)*210
        bbox=im.getchannel('A').point(lambda a:255 if a>32 else 0).getbbox()
        if bbox: im=im.crop(bbox)
        im.thumbnail((225,155),Image.Resampling.NEAREST)
        # Kleine Exporte zur Kontrolle sichtbar hochziehen, ohne dies als Spielzoom auszugeben.
        k=min(225/im.width,155/im.height)
        im=im.resize((round(im.width*k),round(im.height*k)),Image.Resampling.NEAREST)
        overview.paste(im,(x+(250-im.width)//2,y),im)
        d.text((x+5,y+165),label[:34],font=font(13),fill='#25332D')
    save(overview,root/'identity.png')
    return pages

def html_review(root, manifest, report, pages):
    images=''.join(f'<figure><img src="{name}" alt="Kontaktbogen"></figure>' for name in ['identity.png']+pages)
    warnings=''.join('<li>'+html.escape(w)+'</li>' for w in report['warnings']+report['required_review'])
    assessment=''
    if (root/'assessment.json').exists():
        assessment='<p><strong>Gestalterische Vorprüfung: '+html.escape(read(root/'assessment.json')['production_recommendation'])+'</strong> – <a href="assessment.json">Befunde</a>. Dies ist keine Freigabe von Dennis.</p>'
    text=f'''<!doctype html><html lang="de"><meta charset="utf-8"><title>Sprite-Freigabe</title>
<style>body{{background:#132638;color:#eee3cb;font:18px system-ui;max-width:1300px;margin:2em auto}}img{{max-width:100%}}a{{color:#9cdde4}}figure{{margin:1em 0}}code{{background:#294050}}</style>
<h1>{html.escape(manifest['asset_id'])}: Freigabe offen</h1><p>Technik: {report['technical_status']}. Menschliche Freigabe: offen. Nichts wurde ins Spiel übernommen.</p>{assessment}
<p><a href="manifest.json">Manifest</a> · <a href="validation.json">Prüfbericht</a> · <a href="job.json">Auftrag</a></p>
<ul>{warnings}</ul>{images}<p>Auswahl und Änderungen an Dennis zurückgeben. Eine Freigabe wird separat über den Befehl approve protokolliert; diese Seite behauptet keine Zustimmung.</p></html>'''
    (root/'review.html').write_text(text,encoding='utf-8')
