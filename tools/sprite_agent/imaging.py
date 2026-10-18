"""Freistellung, gemeinsamer Maßstab und LOD-Export (Pillow/NumPy/SciPy)."""
from io import BytesIO
import numpy as np
from PIL import Image, ImageCms, ImageDraw
from scipy import ndimage

# Stabiles, eingebettetes sRGB-Profil ohne pro Lauf wechselnden Zeitstempel.
def srgb():
    from .model import ROOT
    existing=ROOT/'art/nature/fox-ab/fox_0_graze_r_f0.png'
    if existing.exists():
        with Image.open(existing) as im:
            if im.info.get('icc_profile'):
                return im.info['icc_profile']
    return ImageCms.ImageCmsProfile(ImageCms.createProfile('sRGB')).tobytes()

def reference_canvas(path, size):
    im=Image.open(path).convert('RGBA')
    bbox=im.getchannel('A').point(lambda a:255 if a>8 else 0).getbbox()
    if not bbox:
        raise ValueError('Leere Referenz.')
    im=im.crop(bbox)
    im.thumbnail((round(size[0]*.8),round(size[1]*.8)),Image.Resampling.LANCZOS)
    canvas=Image.new('RGBA',size,'white')
    canvas.alpha_composite(im,((size[0]-im.width)//2,(size[1]-im.height)//2))
    return canvas.convert('RGB')

def cutout(path, tolerance=32):
    im=Image.open(path).convert('RGBA')
    a=np.asarray(im).copy()
    warnings=[]
    if a[:,:,3].min()<8:
        method='existing_alpha'
    else:
        # Nur vom Rand erreichbarer Hintergrund verschwindet. Cremefarbene
        # Fellinseln im Motiv bleiben erhalten; Randkontakt muss geprüft werden.
        rgb=a[:,:,:3].astype(float)
        border=np.concatenate((rgb[0],rgb[-1],rgb[:,0],rgb[:,-1]))
        color=np.median(border,axis=0)
        distance=np.max(np.abs(rgb-color),axis=2)
        # SDXL erzeugt teils einen weißen Saum auf neutralem Hintergrund.
        # Nur sehr helle neutrale Pixel als weiteren Hintergrund zulassen.
        neutral_halo=(rgb.min(axis=2)>242)&(rgb.max(axis=2)-rgb.min(axis=2)<10)
        candidates=(distance<tolerance)|neutral_halo
        seeds=np.zeros(candidates.shape,bool)
        seeds[0]=candidates[0]; seeds[-1]=candidates[-1]
        seeds[:,0]=candidates[:,0]; seeds[:,-1]=candidates[:,-1]
        bg=ndimage.binary_propagation(seeds,mask=candidates)
        a[bg,3]=0
        if np.percentile(np.max(np.abs(border-color),axis=1),90)>tolerance:
            warnings.append('Hintergrund ungleichmäßig: Freistellung manuell prüfen.')
        method='border_connected_color_key'
    mask=a[:,:,3]>32
    labels,count=ndimage.label(mask)
    sizes=np.bincount(labels.ravel()); sizes[0]=0
    if not sizes.any():
        raise ValueError('Freistellung ergab ein leeres Motiv.')
    if sum(sizes>max(sizes)*.02)>1:
        warnings.append('Mehrere getrennte Motivteile: Anatomie, Einzelmotiv und Freistellung prüfen.')
    largest=int(sizes.argmax())
    removed=int(mask.sum()-sizes[largest])
    if removed:
        # Auftrag verlangt genau ein zusammenhängendes Motiv, keinen Bildrand-
        # Strich und keine Hintergrundinseln. Verluste bleiben im Bericht sichtbar.
        a[(labels!=largest),3]=0
        warnings.append(f'{removed} Pixel in getrennten Komponenten entfernt; auf verlorene Motivteile prüfen.')
        mask=labels==largest
    bbox=Image.fromarray(np.uint8(mask)*255).getbbox()
    if bbox[0]<=1 or bbox[1]<=1 or bbox[2]>=im.width-1 or bbox[3]>=im.height-1:
        warnings.append('Motiv berührt Quellrand: mögliches Abschneiden.')
    if mask.mean()>.8:
        warnings.append('Sehr große Vordergrundfläche: Hintergrundentfernung möglicherweise unvollständig.')
    return Image.fromarray(a).crop(bbox),{'method':method,'bbox':list(bbox),'removed_component_pixels':removed,'warnings':warnings}

def fit_frames(cutouts, job):
    """Ein Skalierungsfaktor für alle Frames; keine unabhängige Normalisierung."""
    scale=max(l['scale'] for l in job['lods'])
    w,h=[round(x*scale) for x in job['display_size']]
    ax,ay=[round(x*scale) for x in job['anchor']]
    maxw=max(im.width for im in cutouts)
    maxh=max(im.height for im in cutouts)
    # Mindestens ein Pixel Rand bereits im kleinsten LOD, nicht nur in 8×.
    margin=max(2,scale)
    factor=min((2*min(ax,w-ax)-2*margin)/maxw,(ay-margin)/maxh)
    if factor<=0:
        raise ValueError('Anker lässt keinen Platz für das Motiv.')
    result=[]
    for im in cutouts:
        obj=im.resize((max(1,round(im.width*factor)),max(1,round(im.height*factor))),Image.Resampling.LANCZOS)
        target=Image.new('RGBA',(w,h))
        # Heuristischer Fußpunkt: untere Mitte des freigestellten Motivs.
        # Im Bericht ausdrücklich zur visuellen Kontrolle markiert.
        d=ImageDraw.Draw(target)
        d.ellipse((ax-obj.width*.45-2,ay-2,ax+obj.width*.3,ay+2),fill=(12,26,52,77))
        target.alpha_composite(obj,(ax-obj.width//2,ay-obj.height))
        result.append(target)
    return result

def make_lod(high, job, lod):
    size=tuple(round(n*lod['scale']) for n in job['display_size'])
    if size==high.size:
        return high.copy(),'reference_downsample'
    im=high.resize(size,Image.Resampling.LANCZOS)
    # Resampling kann selbst transparente Ecken mit Alpha 1–7 füllen.
    # Solche unsichtbaren Fransen entfernen, gerade bei 5×5-Gegenständen.
    im.putalpha(im.getchannel('A').point(lambda a:0 if a<8 else a))
    if lod['scale']<=2:
        # Weniger Farbflächen als technische Vorvereinfachung. Keine Behauptung,
        # Ohren/Schwanz würden dadurch semantisch korrekt nachgezeichnet.
        alpha=im.getchannel('A')
        rgb=im.convert('RGB').quantize(colors=24,method=Image.Quantize.MEDIANCUT,dither=Image.Dither.NONE).convert('RGBA')
        rgb.putalpha(alpha)
        return rgb,'filtered_palette_candidate_requires_review'
    return im,'filtered_downsample'

def save(im,path):
    im.save(path,icc_profile=srgb())
