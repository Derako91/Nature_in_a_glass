"""Entwurfs-Matrix, Maßstäbe, Quellen, Manifest und unveränderten Produktivbestand prüfen."""
import hashlib
import gzip
import json
import subprocess
from PIL import Image, ImageOps
from export_canoe_and_hunt import ROOT, ART, OUTPUT, KINDS, opaque_box, sha


def check():
    cfg=json.loads((ART/'sources.json').read_text())
    report=json.loads((ART/'export.json').read_text())
    for s in cfg['people']+[cfg['deer']]:assert sha(ART/s['file'])==s['sha256']
    expected=set()
    for kind,(state,size,anchor,display,ms) in KINDS.items():
        manifest=json.loads((OUTPUT/'people/manifest.json').read_text())
        entry=next(e for e in manifest['assets'] if e['state']==state)
        assert tuple(entry['size'])==size and tuple(entry['anchor'])==anchor
        assert tuple(entry['display_size'])==display and entry['frame_ms']==ms
        files=set()
        for look in range(5):
            for direction in ('r','l'):
                phases=set()
                for frame in range(4):
                    name=f'person_{look}_{state}_{direction}_f{frame}.png'
                    im=Image.open(OUTPUT/'people'/name).convert('RGBA')
                    assert im.size==size
                    box=opaque_box(im)
                    assert box and box[0]>0 and box[1]>0 and box[2]<size[0] and box[3]<size[1]
                    assert im.getpixel((0,0))[3]==0
                    other=Image.open(OUTPUT/'people'/name.replace(f'_{direction}_',f'_{"l" if direction=="r" else "r"}_')).convert('RGBA')
                    assert im.tobytes()!=ImageOps.mirror(other).tobytes(),name
                    phases.add(im.tobytes());files.add(name)
                assert len(phases)==4,(look,state,direction)
        assert set(entry['files'])==files
        expected|={('people',f) for f in files}
    for variant in range(2):
        for direction in ('l','r'):
            for far in ('','@fern'):
                name=f'deer_hunted_{variant}_rest_{direction}_f0{far}.png'
                im=Image.open(OUTPUT/'animals'/name).convert('RGBA')
                assert im.size==((48,32) if far else (192,128))
                assert opaque_box(im) and im.getpixel((0,0))[3]==0
                expected.add(('animals',name))
    actual={(p.parent.name,p.name) for p in OUTPUT.rglob('*.png')}
    assert actual==expected and len(actual)==88
    for r in report['files']:
        assert sha(OUTPUT/r['group']/r['file'])==r['sha256']
        assert sha(ROOT/'assets/nature'/r['group']/r['file'])==r['sha256']
    for group in ('people','animals'):
        published=json.loads((ROOT/'assets/nature'/group/'manifest.json').read_text())
        for entry in json.loads((OUTPUT/group/'manifest.json').read_text())['assets']:
            assert entry in published['assets']
    for s in report['scales']:assert abs(s['factor']*s['reference_height']-s['target_height'])<1e-9
    tree=subprocess.check_output(['git','ls-tree','-r','a39bb5b','assets/nature'],cwd=ROOT,text=True)
    unchanged=0
    for line in tree.splitlines():
        meta,name=line.split('\t')
        if name in ('assets/nature/people/manifest.json','assets/nature/animals/manifest.json'):
            parent=json.loads(subprocess.check_output(['git','show','a39bb5b:'+name],cwd=ROOT))
            current=json.loads((ROOT/name).read_text())
            current['assets']=[e for e in current['assets'] if not(e.get('id')=='deer_hunted' or (e.get('id')=='person' and e.get('state') in ('hunt','fish_boat')))]
            assert current==parent,name
            continue
        data=(ROOT/name).read_bytes()
        blob=hashlib.sha1(b'blob '+str(len(data)).encode()+b'\0'+data).hexdigest()
        assert blob==meta.split()[2],name
        unchanged+=1
    integration=ART/'v0.22-integration'
    cases=json.loads((integration/'cases.json').read_text())
    captures=json.loads((integration/'captures.json').read_text())
    expected_photos={c['name']+f'-zoom{z}.png' for c in cases for z in (8,32)}
    assert {c['image'] for c in captures}==expected_photos and len(captures)==6
    for c in captures:
        assert c['embedded'] and not c['preview'] and '--bilder' not in c['args']
        assert Image.open(integration/c['image']).size==(1440,900)
        if c['image'].startswith('hunt-'):assert c['save'] is None
    saved=json.load(gzip.open(integration/'deer.json.gz','rt'))
    assert saved['engine_version']=='0.22.0'
    assert any(c.get('hunted') and c['species']=='Deer' and c['meat']==0 and c['days_left']==2 for c in saved['simulation']['animals']['carcasses'])
    assert not subprocess.check_output(['git','diff','a39bb5b','--','crates/core','crates/app/src/view.rs','crates/app/src/main.rs'],cwd=ROOT)
    result=dict(base_commit='a39bb5b',new_embedded_pngs=88,
        production_assets_unchanged=unchanged,native_screenshots=6,deer_save_hunted=True,embedded=True,source_hashes='passed',
        matrix_and_manifests='passed',phase_uniqueness='passed',
        standing_calibration='passed',directions_not_mirrored=True,visual_approval='pending')
    (ART/'validation.json').write_text(json.dumps(result,indent=2)+'\n')
    print(json.dumps(result))


if __name__=='__main__':check()
