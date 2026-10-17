"""Zweite Stilrunde: wärmere Pixelfarben, kompakte Kronen, feiner Boden."""
import numpy as np
from PIL import Image, ImageDraw
import build_proof as b

# Materialvorlagen und Totholz stammen unverändert aus Runde 1 (comfyui/),
# nur die vier Vegetationsmotive wurden neu erzeugt (comfyui-v2/).
b.SRC=b.ROOT/'art/nature/comfyui'
V2=b.ROOT/'art/nature/comfyui-v2'
b.SHEET_TITLE='Warme Pixel. Kleine, lebendige Natur.'
b.SHEET_SUBTITLE='Gestaltungs-Prüfblatt 02  /  Richtung Siedler I & II  /  Lokales ComfyUI · SDXL'
b.HOLE_MODULUS=79
original_sprite=b.sprite

def texture(source,size,color):
    im=Image.open(b.SRC/(source+'.png')).convert('L').resize(size,Image.Resampling.BOX)
    a=np.array(im)
    idx=np.searchsorted(np.quantile(a,[.10,.28,.56,.79,.95]),a)
    base=np.array(b.rgb(color),dtype=float)
    colors=[]
    for f,warm in zip([.84,.92,1,1.055,1.11,1.17],[-3,-2,0,3,6,9]):
        c=np.clip(base*f+np.array([warm,warm*.45,-warm*.8]),1,244).astype(np.uint8)
        colors.append(c)
    colors[2]=np.array(b.rgb(color),dtype=np.uint8)
    if source=='water': colors=b.palette(color,(.88,.94,1,1.035,1.07,1.11))
    return Image.fromarray(np.array(colors,dtype=np.uint8)[idx]).convert('RGBA')

def sprite(name,size,color):
    if name=='snag_standing':
        return original_sprite(name,size,color)
    im=Image.open(V2/(name+'.png')).convert('RGB')
    a=np.array(im).astype(int)
    green=(a[:,:,1]>a[:,:,0]*.90)&(a[:,:,1]>a[:,:,2]*1.12)&(a[:,:,0]<205)
    brown=(a[:,:,0]>a[:,:,2]*1.2)&(a[:,:,1]>a[:,:,2]*1.1)&(a[:,:,0]<170)
    keep=green|brown
    if name=='shrub': keep=green
    # Weißen Außenbereich und vereinzelte KI-Pixel außerhalb des Hauptmotivs verwerfen.
    from scipy.ndimage import label
    labeled,count=label(keep)
    counts=np.bincount(labeled.ravel()); counts[0]=0
    keep=labeled==int(np.argmax(counts))
    ys,xs=np.where(keep)
    if name!='shrub':
        # Bodensockel unterhalb des Stamms gehören nicht zum Spielsprite.
        low=int(ys.min()+.86*(ys.max()-ys.min()))
        center=int((xs.min()+xs.max())/2)
        radius=max(2,int((xs.max()-xs.min())*.11))
        keep[low:,:center-radius]=False
        keep[low:,center+radius:]=False
        ys,xs=np.where(keep)
    box=(int(xs.min()),int(ys.min()),int(xs.max()+1),int(ys.max()+1))
    cropped=im.crop(box)
    mask=Image.fromarray((keep*255).astype('uint8')).crop(box)
    w,h=size
    target_h={'tree_young':12,'tree_mid':18,'tree_old':24,'shrub':6}[name]
    # Die Dateimaße bleiben fest; der Inhalt wird gedrungener und am Fuß verankert.
    dims=(w-2,target_h)
    cropped=cropped.resize(dims,Image.Resampling.BOX)
    mask=mask.resize(dims,Image.Resampling.BOX).point(lambda v:255 if v>=100 else 0)
    a=np.array(cropped).astype(float)
    lum=a[:,:,0]*.30+a[:,:,1]*.59+a[:,:,2]*.11
    valid=lum[np.array(mask)>0]
    idx=np.searchsorted(np.quantile(valid,[.20,.48,.78]),lum)
    palettes={
        'tree_young':['305E30','46843A','70964A','A1AE65'],
        'tree_mid':['294D2A','466C32','698743','98A35A'],
        'tree_old':['1E502A','355F2B','597B38','89954C'],
        'shrub':['46572E','637438','76823C','A2A55C'],
    }
    out=Image.new('RGBA',size)
    d=ImageDraw.Draw(out)
    d.ellipse((0,h-3,w//2+1,h-1),fill=b.rgb('0C1A34')+(77,))
    trunk=b.rgb('7A6854')+(255,)
    if name!='shrub': d.line((w//2,h-7,w//2,h-1),fill=trunk,width=1 if w<15 else 2)
    offset=h-target_h-1
    for y in range(target_h):
        for x in range(w-2):
            if mask.getpixel((x,y)):
                c=b.rgb(palettes[name][idx[y,x]])+(255,)
                if a[y,x,0]>a[y,x,1]*1.03: c=trunk
                out.putpixel((x+1,y+offset),c)
    b.save(name+'_0_normal','vegetation',out,[w//2,h-1])

b.texture=texture
b.sprite=sprite

if __name__=='__main__':
    b.main()
    # Vergleich mit identischer Geometrie, damit die Stiländerung sichtbar bleibt.
    canvas=Image.new('RGB',(1600,650),b.rgb('0C1A34'))
    d=ImageDraw.Draw(canvas)
    for i,(path,title) in enumerate([(b.ROOT/'art/nature/proof-01/_island.png','01 · Erster Entwurf'),(b.OUT/'_island.png','02 · Richtung Siedler I & II')]):
        d.text((i*800+50,45),title,font=b.font(28,True),fill='#E5E4D4')
        im=Image.open(path).resize((784,476),Image.Resampling.NEAREST)
        canvas.paste(im,(i*800+8,130),im)
    canvas.save(b.OUT/'_comparison-v2.png',icc_profile=b.PROFILE)
