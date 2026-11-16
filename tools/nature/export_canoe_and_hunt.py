"""Einbaum-/Jagdentwurf proportional exportieren; noch keine produktive Einbettung."""
import json
import statistics
from pathlib import Path

import numpy as np
from PIL import Image, ImageDraw
from export_people_tools import split_cells, ordinary_frames, opaque_box, sha
from export_people_style_b import split

ROOT = Path(__file__).resolve().parents[2]
ART = ROOT / 'art/nature/canoe-and-hunt-style-b'
OUTPUT = ART / 'sprites'
KINDS = {'hunt': ('hunt', (144, 120), (72, 112), (16.2, 13.5), 250),
         'boat': ('fish_boat', (224, 144), (112, 128), (25.2, 16.2), 300)}


def place(cut, factor, size, anchor, origin):
    scaled = cut.resize((round(cut.width * factor), round(cut.height * factor)), Image.Resampling.LANCZOS)
    x, y = (round(anchor[i] - origin[i] * factor) for i in range(2))
    if x < 0 or y < 0 or x + scaled.width > size[0] or y + scaled.height > size[1]:
        raise ValueError(f'Quellpose überschreitet Leinwand: {scaled.size}, {x,y}, {size}')
    im = Image.new('RGBA', size)
    im.alpha_composite(scaled, (x, y))
    return im


def export():
    cfg = json.loads((ART / 'sources.json').read_text())
    assert len(cfg['people']) == 10
    assert {(s['kind'],s['look']) for s in cfg['people']} == {(k,l) for k in KINDS for l in range(5)}
    images, records, scales = {}, [], []
    for source in cfg['people']:
        path = ART / source['file']; assert sha(path) == source['sha256']
        if source['kind'] == 'boat':
            from export_fishing_people import foot
            cells = []
            for cut in split(path, 2, 6):
                fx, fy, head = foot(cut)
                cells.append(dict(image=cut, foot=(fx,fy), body_height=fy-head, source_box=None))
        else:
            cells = split_cells(path, 3)
        kind, look = source['kind'], source['look']
        state, size, anchor, display, ms = KINDS[kind]
        parents = ordinary_frames(look, 'idle')
        target = statistics.median(opaque_box(im)[3]-opaque_box(im)[1] for im in parents)
        reference = statistics.median(c['body_height'] for c in cells[:4])
        factor = target/reference
        scales.append(dict(kind=kind,look=look,factor=factor,reference_height=reference,target_height=target))
        for i, cell in enumerate(cells[4:]):
            direction, frame = ('r' if i < 4 else 'l'), i % 4
            cut = cell['image']
            origin = cell['foot']
            if kind == 'boat':
                # Der Anker liegt mittig unter dem geschlossenen Rumpf,
                # nicht unter den unsichtbaren Füßen der sitzenden Person.
                data = np.asarray(cut)
                ys,xs = np.nonzero(data[:,:,3] > 160)
                lower = ys >= ys.max()-max(2,round((ys.max()-ys.min())*.04))
                origin = (float(np.median(xs[lower])), float(ys.max()+1))
            im = place(cut, factor, size, anchor, origin)
            name = f'person_{look}_{state}_{direction}_f{frame}'
            images[('people', name)] = im
            box = opaque_box(im)
            records.append(dict(group='people',file=name+'.png',kind=kind,look=look,
                                direction=direction,frame=frame,size=size,anchor=anchor,display_size=display,
                                source_box=cell['source_box'],factor=factor,opaque_box=box))
    source = cfg['deer']; path = ART/source['file']; assert sha(path) == source['sha256']
    cuts = split(path, 2, 2)
    # Fester gemeinsamer Maßstab: gesamtes Motiv einschließlich Speer ca. 17 Welteinheiten.
    factor = 152 / statistics.median(opaque_box(c)[2]-opaque_box(c)[0] for c in cuts)
    for i,cut in enumerate(cuts):
        variant,direction = i//2, ('r' if i%2 == 0 else 'l')
        b = opaque_box(cut)
        origin = ((b[0]+b[2])/2, (b[1]+b[3])/2)
        im = place(cut, factor, (192,128), (96,64), origin)
        name=f'deer_hunted_{variant}_rest_{direction}_f0'
        for far in (False,True):
            sprite = im.resize((48,32),Image.Resampling.LANCZOS) if far else im
            suffix='@fern' if far else ''
            images[('animals',name+suffix)] = sprite
            records.append(dict(group='animals',file=name+suffix+'.png',kind='deer',variant=variant,
                                direction=direction,frame=0,size=sprite.size,anchor=[24,16] if far else [96,64],
                                display_size=[21.6,14.4],factor=factor,opaque_box=opaque_box(sprite)))
    assert len(images)==88
    for (group,name),im in images.items():
        folder=OUTPUT/group;folder.mkdir(parents=True,exist_ok=True);im.save(folder/(name+'.png'))
    for record in records:record['sha256']=sha(OUTPUT/record['group']/record['file'])
    people_manifest=dict(version=1,status='draft_interface_pending',assets=[])
    parent=json.loads((ROOT/'assets/nature/people/manifest.json').read_text())
    people_manifest['appearances']=parent['appearances']
    for kind,(state,size,anchor,display,ms) in KINDS.items():
        people_manifest['assets'].append(dict(id='person',group='people',state=state,variants=5,frames=4,
            frame_ms=ms,directions=['l','r'],size=size,anchor=anchor,display_size=display,
            files=[r['file'] for r in records if r['kind']==kind]))
    (OUTPUT/'people/manifest.json').write_text(json.dumps(people_manifest,ensure_ascii=False,indent=2)+'\n')
    deer_manifest=dict(version=1,status='draft_interface_pending',assets=[dict(id='deer_hunted',group='animals',
        state='rest',variants=2,frames=1,frame_ms=None,directions=['l','r'],size=[192,128],anchor=[96,64],
        display_size=[21.6,14.4],files=[r['file'] for r in records if r['kind']=='deer' and '@fern' not in r['file']],
        levels=[dict(label='@fern',anchor=[24,16],
                  files=[r['file'] for r in records if '@fern' in r['file']])])])
    (OUTPUT/'animals/manifest.json').write_text(json.dumps(deer_manifest,ensure_ascii=False,indent=2)+'\n')
    (ART/'export.json').write_text(json.dumps(dict(base_commit=cfg['base_commit'],embedded=False,
        files=records,scales=scales,visual_approval='pending'),ensure_ascii=False,indent=2)+'\n')
    proofs(images)
    print('88 Entwurfsbilder in art/: 40 Jagd, 40 Einbaum, 4 erlegte Hirsche + 4 Fernbilder.')


def proofs(images):
    for kind,(state,size,anchor,display,ms) in KINDS.items():
        frames=[]
        for frame in range(4):
            board=Image.new('RGB',(5*260,2*185),'#354032' if kind=='hunt' else '#376d85')
            draw=ImageDraw.Draw(board)
            for look in range(5):
                for row,direction in enumerate(('r','l')):
                    im=images[('people',f'person_{look}_{state}_{direction}_f{frame}')]
                    board.paste(im,(look*260+(260-size[0])//2,row*185+30),im)
                    draw.text((look*260+8,row*185+8),f'{look} / {direction} / f{frame}',fill='white')
            frames.append(board)
        frames[0].save(ART/(kind+'-contact.png'))
        frames[0].save(ART/(kind+'.gif'),save_all=True,append_images=frames[1:],duration=ms,loop=0,disposal=2)
    board=Image.new('RGB',(4*230,165),'#354032');draw=ImageDraw.Draw(board)
    for i,((group,name),im) in enumerate((key,value) for key,value in images.items() if key[0]=='animals' and '@fern' not in key[1]):
        board.paste(im,(i*230+18,20),im);draw.text((i*230+8,5),name,fill='white')
    board.save(ART/'deer-contact.png')


if __name__ == '__main__':
    export()
