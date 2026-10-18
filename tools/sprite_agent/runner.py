"""CLI-Aufträge → lokales ComfyUI → LOD-Paket → menschliche Freigabe."""
import argparse
import json
import shutil
import sys
import time
from contextlib import contextmanager
from pathlib import Path
from uuid import uuid4
from PIL import Image

from .model import ROOT, PROFILE, read, write, digest, resolve, load_profile, validate, plan, frames, safe_id
from .comfy import Comfy, workflow
from .imaging import reference_canvas, cutout, fit_frames, make_lod, save
from .review import validate_package, contact_sheet, html_review

@contextmanager
def exclusive(path):
    """Betriebssystem-Lock: wird auch bei Prozessabbruch freigegeben."""
    path=Path(path);path.parent.mkdir(parents=True,exist_ok=True)
    with path.open('a+b') as lock:
        if path.stat().st_size==0:lock.write(b'0');lock.flush()
        lock.seek(0)
        if sys.platform=='win32':
            import msvcrt
            try:msvcrt.locking(lock.fileno(),msvcrt.LK_NBLCK,1)
            except OSError:raise RuntimeError('Ein anderer Lauf bearbeitet diesen Ausgabeordner.')
        else:
            import fcntl
            fcntl.flock(lock.fileno(),fcntl.LOCK_EX|fcntl.LOCK_NB)
        try:yield
        finally:
            lock.seek(0)
            if sys.platform=='win32':msvcrt.locking(lock.fileno(),msvcrt.LK_UNLCK,1)
            else:fcntl.flock(lock.fileno(),fcntl.LOCK_UN)

def produce(job_path, output_root, url, resume=None):
    lockroot=Path(resume).resolve().parent if resume else Path(output_root).resolve()
    with exclusive(lockroot/'.sprite-agent.lock'):
        return _produce(job_path,output_root,url,resume)

def _produce(job_path, output_root, url, resume=None):
    job=validate(read(job_path))
    profile=load_profile(job.get('profile',PROFILE))
    style_path=resolve(profile,profile['style_document'])
    style=style_path.read_text(encoding='utf-8')
    reference=resolve(profile,job['reference']) if job.get('reference') else None
    if reference and not reference.is_file():raise ValueError('Referenz fehlt: '+str(reference))
    required=list(frames(job))
    if len(required)>job['budget']:
        raise ValueError(f"Auftrag benötigt {len(required)} Bilder, Budget ist {job['budget']}. Vor Generierung Auftrag eingrenzen.")
    snapshot={'job':job,'style_sha256':digest(style_path),
              'reference_sha256':digest(reference) if reference else None,
              'profile':{k:v for k,v in profile.items() if not k.startswith('_')}}
    if resume:
        folder=Path(resume).resolve()
        if (folder/'approval.json').exists():raise ValueError('Freigegebenes Paket ist unveränderlich.')
        if read(folder/'snapshot.json')!=snapshot:raise ValueError('Auftrag/Stil/Referenz geändert; neuer Lauf erforderlich.')
    else:
        folder=Path(output_root).resolve()/(job['id']+'-'+time.strftime('%Y%m%d-%H%M%S')+'-'+uuid4().hex[:6])
        folder.mkdir(parents=True,exist_ok=False)
        write(folder/'snapshot.json',snapshot)
        write(folder/'job.json',job)
        (folder/'style-source.md').write_text(style,encoding='utf-8')
    for name in ('raw','workflows','sprites'): (folder/name).mkdir(exist_ok=True)
    print('PAKET '+str(folder),flush=True)
    client=Comfy(url)
    capabilities=client.capabilities()
    if profile['checkpoint'] not in capabilities['checkpoints']:
        raise ValueError('Checkpoint nicht installiert: '+profile['checkpoint'])
    needs_ref=job['strategy']=='reference' or job['strategy']=='auto' and reference
    if needs_ref and not {'IPAdapterUnifiedLoader','IPAdapterAdvanced'}.issubset(capabilities['nodes']):
        raise ValueError('Referenz-Workflow nicht verfügbar; kein stiller Text-only-Rückfall.')
    write(folder/'environment.json',capabilities)
    journal_path=folder/'journal.json'
    journal=read(journal_path) if journal_path.exists() else {'calls':{},'budget':job['budget']}
    uploaded=None
    if reference:
        canvas=reference_canvas(reference,(job['width'],job['height']))
        canvas.save(folder/'reference-input.png')
        uploaded=client.upload(folder/'reference-input.png')
    for index,frame in enumerate(required):
        key=frame['key'];raw=folder/'raw'/(key+'.png')
        record=journal['calls'].get(key)
        if record and record.get('sha256'):
            if not raw.exists() or digest(raw)!=record['sha256']:
                raise ValueError('Gesicherte Rohdatei fehlt oder wurde verändert: '+key)
            continue
        if not record:
            if len(journal['calls'])>=job['budget']:raise ValueError('Generierungsbudget ausgeschöpft.')
            graph,strategy=workflow(job,profile,frame,job['seed']+index,uploaded)
            write(folder/'workflows'/(key+'.api.json'),graph)
            # Vor POST buchen: bei Verbindungsabbruch nicht blind erneut senden.
            record={'state':'submitting','strategy':strategy,'seed':job['seed']+index}
            journal['calls'][key]=record;write(journal_path,journal)
            response=client.request('/prompt',{'prompt':graph,'client_id':folder.name})
            record.update(prompt_id=response['prompt_id'],state='queued');write(journal_path,journal)
        if not record.get('prompt_id'):
            raise RuntimeError('Unklarer Versandstatus; ComfyUI-Historie prüfen. Keine automatische Doppelgenerierung: '+key)
        print('GENERIERUNG '+key+' / '+record['strategy'],flush=True)
        client.collect(record['prompt_id'],raw,folder/'workflows'/(key+'.history.json'))
        record.update(state='complete',sha256=digest(raw));write(journal_path,journal)
    return prepare(folder,job,profile,reference,snapshot,required)

def prepare(folder,job,profile,reference,snapshot,required):
    # Ab hier rein lokale Aufbereitung. Neue Zustände werden nie durch Kopien ersetzt.
    if (folder/'approval.json').exists():raise ValueError('Freigegebenes Paket ist unveränderlich.')
    journal=read(folder/'journal.json')
    for frame in required:
        raw=folder/'raw'/(frame['key']+'.png')
        expected_hash=journal['calls'].get(frame['key'],{}).get('sha256')
        if not raw.is_file() or not expected_hash or digest(raw)!=expected_hash:
            raise ValueError('Rohbild fehlt oder weicht vom Generierungsjournal ab: '+frame['key'])
    if reference:
        target=folder/'reference.png'
        if reference.resolve()!=target.resolve():shutil.copyfile(reference,target)
        reference=target
    cuts=[]; extraction=[]
    for frame in required:
        im,info=cutout(folder/'raw'/(frame['key']+'.png'),job['background_tolerance'])
        cuts.append(im);extraction.append({'key':frame['key'],**info})
    high=fit_frames(cuts,job)
    manifest={'schema_version':1,'package_id':folder.name,'asset_id':job['id'],
        'category':job['category'],'status':'pending_human_review','display_size':job['display_size'],
        'anchor':job['anchor'],'anchor_method':'bottom_center_heuristic_requires_review',
        'direction_convention':'screen_diagonal_se_sw_ne_nw','lighting':'upper_right_fixed',
        'style_sha256':snapshot['style_sha256'],'frames':[]}
    for frame,im in zip(required,high):
        entry={**frame,'lods':[]}
        for lod in sorted(job['lods'],key=lambda l:l['scale']):
            output,method=make_lod(im,job,lod)
            name='sprites/'+frame['key']+'__'+lod['id']+'.png'
            save(output,folder/name)
            entry['lods'].append({**lod,'file':name,'size':list(output.size),
                'anchor':[x*lod['scale'] for x in job['anchor']],
                'method':method,'sha256':digest(folder/name)})
        manifest['frames'].append(entry)
    write(folder/'manifest.json',manifest)
    write(folder/'extraction.json',extraction)
    report=validate_package(folder,manifest,required)
    report['warnings'].extend(w for e in extraction for w in e['warnings'])
    if not reference:report['warnings'].append('Keine Identitätsreferenz: Konsistenz über Ansichten nicht gesichert.')
    write(folder/'validation.json',report)
    backgrounds=[Image.open(resolve(profile,p)).convert('RGBA') for p in profile['backgrounds']]
    pages=contact_sheet(folder,manifest,backgrounds,reference)
    html_review(folder,manifest,report,pages)
    # Archiv liegt daneben und enthält weder sich selbst noch Daten anderer Läufe.
    shutil.make_archive(str(folder), 'zip', root_dir=folder)
    print('TECHNIK '+report['technical_status']+' / GESTALTUNG pending_human_review',flush=True)
    print('ANSICHT '+str(folder/'review.html'),flush=True)
    return folder

def approve(folder, reviewer, notes, visual_checked):
    folder=Path(folder).resolve()
    with exclusive(folder.parent/'.sprite-agent.lock'):
        return _approve(folder,reviewer,notes,visual_checked)

def _approve(folder,reviewer,notes,visual_checked):
    if not visual_checked or not reviewer.strip() or not notes.strip():
        raise ValueError('Freigabe benötigt menschliche Sichtprüfung, Prüfer und Notiz.')
    if (folder/'approval.json').exists():raise ValueError('Bereits freigegeben; keine Überschreibung.')
    manifest=read(folder/'manifest.json')
    report=validate_package(folder,manifest,list(frames(read(folder/'job.json'))),persist=False)
    if report['technical_status']!='passed':raise ValueError('Technische Prüfung fehlgeschlagen.')
    for frame in manifest['frames']:
        for lod in frame['lods']:
            if digest(folder/lod['file'])!=lod['sha256']:raise ValueError('Datei nach Export verändert.')
    # Kein Installieren, keine Änderung an freigegebenen Spielassets.
    record={'reviewer':reviewer,'notes':notes,'visual_checked':True,
            'manifest_sha256':digest(folder/'manifest.json'),'approved_at':time.strftime('%Y-%m-%dT%H:%M:%S%z')}
    with (folder/'approval.json').open('x',encoding='utf-8') as f:json.dump(record,f,ensure_ascii=False,indent=2)
    print('Freigabe protokolliert; Engine-Import bleibt separater Schritt.')

def main():
    p=argparse.ArgumentParser(description='Lokaler Sprite-Agent: Aufträge, ComfyUI, LOD-Pakete, Freigabe.')
    subs=p.add_subparsers(dest='command',required=True)
    d=subs.add_parser('doctor');d.add_argument('--url',default='http://127.0.0.1:8188')
    q=subs.add_parser('plan');q.add_argument('request');q.add_argument('--profile',default=str(PROFILE));q.add_argument('--out',required=True)
    s=subs.add_parser('request');s.add_argument('request');s.add_argument('--profile',default=str(PROFILE));s.add_argument('--output',default=str(ROOT/'runs/sprite-agent'));s.add_argument('--url',default='http://127.0.0.1:8188');s.add_argument('--max-images',type=int,default=8)
    r=subs.add_parser('run');r.add_argument('job');r.add_argument('--output',default=str(ROOT/'runs/sprite-agent'));r.add_argument('--url',default='http://127.0.0.1:8188');r.add_argument('--resume')
    t=subs.add_parser('prepare');t.add_argument('package')
    e=subs.add_parser('export-engine');e.add_argument('package');e.add_argument('--out',required=True);e.add_argument('--preview',action='store_true')
    a=subs.add_parser('approve');a.add_argument('package');a.add_argument('--reviewer',required=True);a.add_argument('--notes',required=True);a.add_argument('--visual-checked',action='store_true')
    args=p.parse_args()
    try:
        if args.command=='doctor':
            cap=Comfy(args.url).capabilities()
            print(json.dumps({'checkpoints':cap['checkpoints'],'reference_workflow':{'IPAdapterAdvanced','IPAdapterUnifiedLoader'}.issubset(cap['nodes']),'devices':cap['stats'].get('devices',[])},indent=2))
        elif args.command=='plan':
            jobs=plan(args.request,load_profile(args.profile))
            out=Path(args.out);out.mkdir(parents=True,exist_ok=True)
            # Nichts teilweise überschreiben, auch bei mehreren Aufträgen.
            if any((out/(j['id']+'.json')).exists() for j in jobs):raise ValueError('Auftragsdatei existiert; neues Zielverzeichnis verwenden.')
            for j in jobs:write(out/(j['id']+'.json'),j)
            print('\n'.join(str(out/(j['id']+'.json')) for j in jobs))
        elif args.command=='run':produce(args.job,args.output,args.url,args.resume)
        elif args.command=='prepare':
            folder=Path(args.package).resolve();snapshot=read(folder/'snapshot.json');job=validate(snapshot['job'])
            profile=snapshot['profile'];profile['_base']=str(Path(job['profile']).resolve().parent)
            reference=folder/'reference.png'
            if not reference.exists():reference=resolve(profile,job['reference']) if job.get('reference') else None
            with exclusive(folder.parent/'.sprite-agent.lock'):
                prepare(folder,job,profile,reference,snapshot,list(frames(job)))
        elif args.command=='request':
            jobs=plan(args.request,load_profile(args.profile))
            total=sum(len(list(frames(j))) for j in jobs)
            if not 1<=args.max_images<=64 or total>args.max_images:
                raise ValueError(f'Gesamtauftrag benötigt {total} Bilder; Limit {args.max_images}. Auftrag eingrenzen oder Limit ausdrücklich setzen.')
            jobroot=Path(args.output)/('auftrag-'+time.strftime('%Y%m%d-%H%M%S')+'-'+uuid4().hex[:6])
            jobroot.mkdir(parents=True,exist_ok=False)
            for j in jobs:
                jobfile=jobroot/(j['id']+'.json');write(jobfile,j)
                produce(jobfile,args.output,args.url)
        elif args.command=='approve':approve(args.package,args.reviewer,args.notes,args.visual_checked)
        elif args.command=='export-engine':
            from .export_nature import export_package
            print(export_package(args.package,args.out,args.preview))
    except (ValueError,RuntimeError,OSError,KeyError) as exc:
        print('ABBRUCH: '+str(exc),file=sys.stderr);sys.exit(1)
