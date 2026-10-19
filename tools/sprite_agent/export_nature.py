"""Ausgabeadapter für gestaltung.md 16.6 / Nature in a Glass v0.7.2."""
import shutil
from pathlib import Path
from PIL import Image
from .model import read, write, digest

GROUPS={'animal':'animals','human':'people','plant':'vegetation','object':'objects','building':'buildings'}
DIRECTIONS={'se':'r','sw':'l','r':'r','l':'l'}

def calibrate_sizes(package, manifest, planned):
    """Motivbreite statt transparenter Leinwand als Weltmaß; pro Variante konstant."""
    result={}
    preferred='graze' if manifest['category']=='animal' else 'idle'
    for variant in sorted({frame['variant'] for _,_,_,frame in planned}):
        candidates=[p for p in planned if p[3]['variant']==variant]
        # Dieselbe Referenz für alle Zustände/Phasen der Variante. Nicht jedes
        # Bild einzeln aufblähen: ein ausgestreckter Schwanz darf weiter reichen.
        name,main,_,frame=min(candidates,key=lambda p:(
            p[3]['state']!=preferred,p[3]['frame']!=0,
            DIRECTIONS[p[3]['direction']]!='r',p[0]))
        with Image.open(package/main['file']) as source:
            if source.mode!='RGBA':raise ValueError('Motivkalibrierung benötigt RGBA: '+name)
            bbox=source.getchannel('A').point(lambda a:255 if a>128 else 0).getbbox()
            if bbox is None:raise ValueError('Kein deckendes Motiv für Weltmaß: '+name)
            visible_width=bbox[2]-bbox[0]
            factor=source.width/visible_width
            size=[n*factor for n in manifest['display_size']]
            result[variant]={'reference':name,'reference_file':main['file'],
                'alpha_threshold':128,'visible_bbox':list(bbox),
                'source_canvas_width':source.width,'visible_width':visible_width,
                'factor':factor,'display_size':size,
                'target_visible_width':manifest['display_size'][0]}
    return result

def export_package(package, destination, preview=False):
    package=Path(package).resolve();destination=Path(destination).resolve()
    if destination.exists():raise ValueError('Exportziel existiert bereits; keine Überschreibung.')
    if not preview and not (package/'approval.json').exists():
        raise ValueError('Ohne Freigabe nur ausdrücklich mit --preview exportieren.')
    manifest=read(package/'manifest.json')
    if not preview and read(package/'approval.json')['manifest_sha256']!=digest(package/'manifest.json'):
        raise ValueError('Freigegebenes Manifest wurde verändert.')
    planned=[]; skipped=[]; names=set()
    for frame in manifest['frames']:
        direction=DIRECTIONS.get(frame['direction'])
        if not direction:
            skipped.append(frame['key']);continue
        if manifest['category']=='plant':
            if direction!='r' or frame['frame']!=0:
                skipped.append(frame['key']);continue
            state='normal' if frame['state']=='idle' else frame['state']
            name=f"{manifest['asset_id']}_{frame['variant']}_{state}"
        else:
            name=f"{manifest['asset_id']}_{frame['variant']}_{frame['state']}_{direction}_f{frame['frame']}"
        if name in names:raise ValueError('Mehrere Ansichten würden dieselbe Engine-Datei überschreiben: '+name)
        names.add(name)
        main=max(frame['lods'],key=lambda l:l['scale'])
        far=next((l for l in frame['lods'] if l['scale']==2),None)
        if not far or main['scale']<=2:raise ValueError('Export benötigt eine Hauptfassung >2× und eine 2×-Fernfassung.')
        for lod in (main,far):
            source=(package/lod['file']).resolve()
            if not source.is_relative_to(package) or digest(source)!=lod['sha256']:
                raise ValueError('Ungültige oder veränderte Quelldatei: '+lod['file'])
        planned.append((name,main,far,frame))
    if not planned:raise ValueError('Keine unterstützte Ansicht für l/r vorhanden.')
    sizing=calibrate_sizes(package,manifest,planned)
    group=GROUPS[manifest['category']]
    target=destination/group;target.mkdir(parents=True,exist_ok=False)
    entries=[]
    for name,main,far,frame in planned:
        for suffix,lod in (('',main),('@fern',far)):
            shutil.copyfile(package/lod['file'],target/(name+suffix+'.png'))
        entries.append({'id':manifest['asset_id'],'files':[name+'.png'],
            'anchor':main['anchor'],'display_size':sizing[frame['variant']]['display_size'],
            'levels':[{'label':'@fern','files':[name+'@fern.png'],'anchor':far['anchor']}]})
    write(target/'manifest.json',{'assets':entries,'frame_aliases':{}})
    write(destination/'export-report.json',{'source_package':manifest['package_id'],
        'format':'nature-in-a-glass-0.7.2','preview_only':preview,'assets':len(planned),
        'world_sizing':{'method':'reference_visible_width_per_variant','variants':sizing,
            'note':'Weltmaß bezeichnet die deckende Breite des Referenzmotivs. Größeres display_size kompensiert transparenten Leinwandrand; Dateifußpunkte bleiben unverändert.'},
        'skipped_rear_views_or_unsupported_plant_frames':skipped,
        'direction_map':DIRECTIONS,
        'warning':'Dateinamen bilden angeforderte Richtungen ab, keine automatische Bestätigung ihrer Bildrichtigkeit. @fern ist zunächst ein technischer Kandidat, keine freigegebene eigene Fernzeichnung.'})
    (destination/'README.txt').write_text(
        'Prüfexport für nature-in-a-glass --bilder "'+str(destination)+'"\n'
        'Keine automatische Installation. Stil, Anatomie, Richtungen und Fernsilhouette prüfen.\n'
        'Hauptbild + @fern nach gestaltung.md 16.6; Zwischenstufen berechnet die Engine.\n',encoding='utf-8')
    return destination
