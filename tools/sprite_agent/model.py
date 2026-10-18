"""Aufträge, Stilprofile und unveränderliche Paketpfade."""
import copy
import hashlib
import json
import re
import math
import os
from uuid import uuid4
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
PROFILE = ROOT / 'tools/sprite_agent/profiles/nature.json'

def read(path):
    return json.loads(Path(path).read_text(encoding='utf-8-sig'))

def write(path, value):
    path=Path(path)
    temp=path.with_name(path.name+'.'+uuid4().hex+'.tmp')
    try:
        with temp.open('x',encoding='utf-8') as stream:
            json.dump(value,stream,ensure_ascii=False,indent=2,allow_nan=False)
            stream.write('\n');stream.flush();os.fsync(stream.fileno())
        temp.replace(path)
    finally:
        temp.unlink(missing_ok=True)

def digest(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()

def safe_id(value):
    if not isinstance(value, str) or not re.fullmatch(r'[a-z][a-z0-9_-]{0,79}', value):
        raise ValueError('ID muss aus kleinen ASCII-Buchstaben, Ziffern, _ oder - bestehen.')
    return value

def resolve(profile, value):
    return (Path(profile['_base']) / value).resolve()

def load_profile(path):
    path = Path(path).resolve()
    p = read(path)
    p['_base'] = str(path.parent)
    p['_file'] = str(path)
    return p

def validate(job):
    safe_id(job['id'])
    if job['category'] not in ('animal','human','building','plant','object'):
        raise ValueError('Unbekannte Asset-Kategorie.')
    for key in ('display_size','anchor'):
        if len(job[key]) != 2 or not all(isinstance(n,(int,float)) and math.isfinite(n) and n >= 0 for n in job[key]):
            raise ValueError('Ungültige Geometrie: '+key)
    if not all(0 < n <= 200 for n in job['display_size']):
        raise ValueError('Ungültige Darstellungsgröße.')
    if any(a > s for a,s in zip(job['anchor'],job['display_size'])):
        raise ValueError('Anker außerhalb des Bildes.')
    if not job['directions'] or len(set(job['directions'])) != len(job['directions']):
        raise ValueError('Richtungen fehlen oder sind doppelt.')
    if any(d not in ('se','sw','ne','nw','r','l') for d in job['directions']):
        raise ValueError('Richtung unbekannt.')
    if not job['states']:
        raise ValueError('Zustände fehlen.')
    for state, config in job['states'].items():
        safe_id(state)
        if not 1 <= config['frames'] <= 16 or config['frame_ms'] <= 0:
            raise ValueError('Ungültiger Animationszyklus.')
        if len(config['poses']) != config['frames']:
            raise ValueError('Für jedes Bild ist eine konkrete Pose nötig; keine kopierten Scheinanimationen.')
    if not 1 <= job['variants'] <= 4 or not 1 <= job['budget'] <= 64:
        raise ValueError('Budget/Varianten außerhalb der Grenzen.')
    if job['strategy'] not in ('auto','reference','img2img','text'):
        raise ValueError('Unbekannte Workflow-Strategie.')
    if job['strategy'] in ('reference','img2img') and not job.get('reference'):
        raise ValueError('Referenz fehlt.')
    if job['strategy']=='img2img' and len(job['directions'])>1:
        raise ValueError('Bild-zu-Bild ist nur für dieselbe Ansicht geeignet; für mehrere Ansichten reference nutzen.')
    if not job['subject'].strip():
        raise ValueError('Motivbeschreibung fehlt.')
    if not job['lods'] or len({l['id'] for l in job['lods']})!=len(job['lods']):
        raise ValueError('LOD-Liste fehlt oder hat doppelte IDs.')
    ordered=sorted(job['lods'],key=lambda l:l['scale'])
    if len({l['scale'] for l in ordered})!=len(ordered):raise ValueError('Doppelte LOD-Skalierung.')
    if any(b['zoom_min']<=a['zoom_min'] for a,b in zip(ordered,ordered[1:])):
        raise ValueError('LOD-Zoomgrenzen müssen mit der Auflösung wachsen.')
    for lod in ordered:
        safe_id(lod['id'])
        if lod['scale'] not in (1,2,4,8) or not 0<=lod['zoom_min']<=32:
            raise ValueError('Ungültige LOD-Skalierung/Zoomgrenze.')
        if any(round(n*lod['scale'])<2 or round(n*lod['scale'])>=2000 for n in job['display_size']):
            raise ValueError('LOD-Bildmaß außerhalb des zulässigen Bereichs.')
    if ordered[0]['zoom_min']>4:raise ValueError('LOD für Zoom 4 fehlt.')
    if any(job[k] not in range(256,1025,64) for k in ('width','height')):
        raise ValueError('Generierung benötigt Maße 256–1024, Vielfache von 64.')
    if not 1<=job['steps']<=50 or not 0<job['denoise']<=1 or not 0<=job['reference_weight']<=2:
        raise ValueError('Samplingparameter außerhalb der Grenzen.')
    if not 1<=job['background_tolerance']<=80:raise ValueError('Ungültige Freistellschwelle.')
    return job

def plan(text, profile, category=None):
    """Bewusst begrenzter lokaler Parser; keine erfundenen Maße für unbekannte Motive."""
    lowered = text.casefold()
    found = []
    for name, spec in profile['catalog'].items():
        if any(re.search(r'\b'+re.escape(a)+r'\b',lowered) for a in spec['aliases']):
            found.append((name,spec))
    if not found and re.search(r'\b(drei|3)\b.*\btiere\b',lowered):
        found=[(n,profile['catalog'][n]) for n in profile['animal_batch']]
    if not found:
        raise ValueError('Motiv nicht im Profilkatalog. Einmal Motiv, Kategorie, Weltmaß und Anker als JSON ergänzen; siehe README. Der Agent rät keine Weltmaße.')
    jobs=[]
    for name,spec in found:
        if category and spec['category']!=category:
            raise ValueError('Kategorie widerspricht dem Profilkatalog.')
        j=copy.deepcopy(profile['defaults'])
        j.update({k:v for k,v in spec.items() if k not in ('aliases',)})
        profile_path=Path(profile['_file'])
        portable=str(profile_path.relative_to(ROOT)).replace('\\','/') if profile_path.is_relative_to(ROOT) else str(profile_path)
        j.update(id=name,request=text,profile=portable)
        if re.search(r'\b(vier|4)\b.*(richtung|ansicht)',lowered):
            j['directions']=['se','sw','ne','nw']
        if any(w in lowered for w in ('animation','laufen','gehen')):
            raise ValueError('Animationsauftrag benötigt Zustände und konkrete Phasen im JSON-Auftrag; Freitext darf keine Vollständigkeit vortäuschen.')
        jobs.append(validate(j))
    return jobs

def frames(job):
    for variant in range(job['variants']):
        for state, timing in job['states'].items():
            for direction in job['directions']:
                for frame,pose in enumerate(timing['poses']):
                    yield {'key':f"{job['id']}_{variant}_{state}_{direction}_f{frame}",
                           'variant':variant,'state':state,'direction':direction,
                           'frame':frame,'frame_ms':timing['frame_ms'],'pose':pose}
