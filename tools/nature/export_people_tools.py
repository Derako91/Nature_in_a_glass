"""Werkzeugfassungen proportional exportieren; Kalibrierung am unveränderten Erwachsenen."""
import copy
import hashlib
import json
import statistics
from pathlib import Path

import numpy as np
from PIL import Image, ImageDraw
from scipy import ndimage

ROOT = Path(__file__).resolve().parents[2]
ART = ROOT / 'art/nature/people-tools-style-b'
PEOPLE = ROOT / 'assets/nature/people'
STATES = {
    'fish_net':(4,300), 'gather_basket':(4,300),
    'eat_basket':(2,600), 'build_axe':(4,250),
}
BASE_FRAMES = {'idle':2,'fish':4,'gather':4,'eat':2,'build':4}


def ordinary_frames(look,state):
    return [Image.open(PEOPLE/f'person_{look}_{state}_{d}_f{f}.png').convert('RGBA')
            for d in ('r','l') for f in range(BASE_FRAMES[state])]



def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def opaque_box(image):
    return image.getchannel('A').point(lambda a:255 if a>128 else 0).getbbox()


def split_cells(path, rows):
    """Ganze Figuren UND abgesetzte Körbe/Netzteile erhalten; Sohlen separat messen."""
    source = Image.open(path).convert('RGBA')
    data = np.asarray(source)
    labels,count = ndimage.label(data[:,:,3] >= 160,np.ones((3,3)))
    sizes = np.bincount(labels.ravel())
    boxes = ndimage.find_objects(labels)
    candidates = sorted(((int(sizes[i]),i,b) for i,b in enumerate(boxes,1) if b is not None),reverse=True)
    main = candidates[:rows*4]
    if len(main) != rows*4 or min(c[0] for c in main) < statistics.median(c[0] for c in main)*.25:
        raise ValueError(f'Keine {rows*4} vollständigen Figuren in {path}')
    main.sort(key=lambda c:c[2][0].stop)
    ordered=[]
    for row in range(rows):
        ordered.extend(sorted(main[row*4:(row+1)*4],key=lambda c:c[2][1].start))
    selected = {label:{label} for _,label,_ in ordered}
    for area,label,(ys,xs) in candidates:
        if label in selected or area < 6:
            continue
        cx,cy=(xs.start+xs.stop)/2,(ys.start+ys.stop)/2
        def distance(item):
            by,bx=item[2]
            dx=max(bx.start-cx,0,cx-bx.stop)
            dy=max(by.start-cy,0,cy-by.stop)
            return dx*dx+dy*dy
        owner=min(ordered,key=distance)[1]
        selected[owner].add(label)
    result=[]
    for _,label,(ys,xs) in ordered:
        if xs.start == 0 or ys.start == 0 or xs.stop == source.width or ys.stop == source.height:
            raise ValueError(f'Figur berührt Quellbildrand: {path}')
        body = labels == label
        y,x=np.nonzero(body)
        lower=y>=y.max()-max(3,round((y.max()-y.min())*.05))
        foot_x=float(np.median(x[lower]));foot_y=float(y.max()+1)
        all_parts=np.isin(labels,list(selected[label]))
        py,px=np.nonzero(all_parts)
        box=(max(0,int(px.min())-4),max(0,int(py.min())-4),
             min(source.width,int(px.max())+5),min(source.height,int(py.max())+5))
        x0,y0,x1,y1=box
        mask=ndimage.binary_dilation(all_parts[y0:y1,x0:x1],iterations=4)
        cut=data[y0:y1,x0:x1].copy()
        cut[:,:,3]=np.where(mask & (cut[:,:,3]>=24),cut[:,:,3],0)
        result.append({'image':Image.fromarray(cut),'foot':(foot_x-x0,foot_y-y0),
                       'body_height':int(y.max()-y.min()+1),'source_box':list(box),
                       'detached_parts':len(selected[label])-1})
    return result


def export():
    sources=json.loads((ART/'sources.json').read_text())['sources']
    assert {(s['kind'],s['look']) for s in sources} == {(k,l) for k in ('net','basket','axe') for l in range(5)}
    assert len(sources)==15
    manifest_path=PEOPLE/'manifest.json'
    manifest=json.loads(manifest_path.read_text())
    output,records,scales,issues={ },[],[],[]
    for source in sources:
        path=ART/source['file'];assert sha(path)==source['sha256'],path
        kind,look=source['kind'],source['look']
        cells=split_cells(path,4 if kind=='basket' else 3)
        old_idle=ordinary_frames(look,'idle')
        target=statistics.median(opaque_box(i)[3]-opaque_box(i)[1] for i in old_idle)
        reference=statistics.median(c['body_height'] for c in cells[:4])
        factor=target/reference
        scales.append({'kind':kind,'look':look,'factor':factor,'old_idle_height':target,
                       'source_standing_height':reference,'calibration_cells':4})
        mapping=[]
        if kind=='basket':
            mapping=[('gather_basket','r',range(4,8)),('gather_basket','l',range(8,12)),
                     ('eat_basket','r',range(12,14)),('eat_basket','l',range(14,16))]
        else:
            state='fish_net' if kind=='net' else 'build_axe'
            mapping=[(state,'r',range(4,8)),(state,'l',range(8,12))]
        for state,direction,indices in mapping:
            base=state.rsplit('_',1)[0]
            parents=ordinary_frames(look,base)
            max_width=max(opaque_box(i)[2]-opaque_box(i)[0] for i in parents)
            max_height=max(opaque_box(i)[3]-opaque_box(i)[1] for i in parents)
            for frame,index in enumerate(indices):
                cell=cells[index];cut=cell['image'];fx,fy=cell['foot']
                size=(round(cut.width*factor),round(cut.height*factor))
                x,y=round(56-fx*factor),round(112-fy*factor)
                name=f'person_{look}_{state}_{direction}_f{frame}'
                if x<0 or y<0 or x+size[0]>112 or y+size[1]>120:
                    issues.append({'file':name,'issue':'Leinwand überschritten','position':[x,y],'scaled_size':size})
                    continue
                image=Image.new('RGBA',(112,120))
                image.alpha_composite(cut.resize(size,Image.Resampling.LANCZOS),(x,y))
                b=opaque_box(image);width,height=b[2]-b[0],b[3]-b[1]
                if width>max_width+1:
                    issues.append({'file':name,'issue':'Zu breit','actual':width,'parent_limit':max_width})
                if height>max_height+4:
                    issues.append({'file':name,'issue':'Zu hoch','actual':height,'parent_limit':max_height})
                if b[3]>114:
                    issues.append({'file':name,'issue':'Werkzeug unter den Sohlen','bottom':b[3]})
                output[name]=image
                records.append({'file':name+'.png','look':look,'state':state,'direction':direction,'frame':frame,
                                'size':[112,120],'anchor':[56,112],'display_size':[12.6,13.5],
                                'opaque_size':[width,height],'parent_max_width':max_width,'parent_max_height':max_height,
                                'source_box':cell['source_box'],'source_factor':factor,'detached_parts':cell['detached_parts']})
    if issues:
        (ART/'issues.json').write_text(json.dumps(issues,ensure_ascii=False,indent=2)+'\n')
        raise ValueError(f'{len(issues)} Quellposen überschreiten die vereinbarten Maße, siehe issues.json')
    assert len(output)==140
    for name,image in output.items():image.save(PEOPLE/(name+'.png'))
    for record in records:record['sha256']=sha(PEOPLE/record['file'])
    for state,(count,ms) in STATES.items():
        base=state.rsplit('_',1)[0]
        entry=copy.deepcopy(next(e for e in manifest['assets'] if e['id']=='person' and e.get('state')==base))
        entry.update(state=state,frames=count,frame_ms=ms,status='tools_style_b_draft',
                     files=[r['file'] for r in records if r['state']==state])
        manifest['assets']=[e for e in manifest['assets'] if not(e['id']=='person' and e.get('state')==state)]
        manifest['assets'].append(entry)
    manifest_path.write_text(json.dumps(manifest,ensure_ascii=False,indent=2)+'\n')
    (ART/'export.json').write_text(json.dumps({'files':records,'scales':scales,'visual_approval':'pending',
        'proportional_scaling':True,'unchanged_foot_points':True,'mirrored_directions':False},ensure_ascii=False,indent=2)+'\n')
    (ART/'issues.json').unlink(missing_ok=True)
    proofs(output)
    print('140 Werkzeugbilder, vorhandene Erwachsenenmaße/Fußpunkte, keine Kinderbilder.')


def proofs(images):
    for state,(count,ms) in STATES.items():
        frames=[]
        for frame in range(count):
            board=Image.new('RGBA',(5*180,2*160),'#354032');draw=ImageDraw.Draw(board)
            for look in range(5):
                for row,direction in enumerate(('r','l')):
                    im=images[f'person_{look}_{state}_{direction}_f{frame}']
                    board.alpha_composite(im,(look*180+34,row*160+24))
                    draw.text((look*180+5,row*160+5),f'{look} / {direction} / f{frame}',fill='white')
            frames.append(board.convert('RGB'))
        frames[0].save(ART/(state+'-contact.png'))
        frames[0].save(ART/(state+'.gif'),save_all=True,append_images=frames[1:],duration=ms,loop=0,disposal=2)


if __name__ == '__main__':
    export()
