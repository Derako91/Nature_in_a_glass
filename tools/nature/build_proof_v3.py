"""Gezielte Naturkorrektur: dunkle geschlossene Altkronen und feiner Sand."""
import numpy as np
from PIL import Image, ImageDraw
import build_proof_v2 as v2

b=v2.b
base_texture=v2.texture
base_sprite=v2.sprite
b.SHEET_TITLE='Ruhiger Boden. Klarer alter Wald.'
b.SHEET_SUBTITLE='Gestaltungs-Prüfblatt 03  /  Richtung Siedler I & II  /  Naturkorrekturen'

def texture(source,size,color):
    if color!='DED09A':return base_texture(source,size,color)
    src=np.array(Image.open(b.SRC/'soil.png').convert('L'))
    yy,xx=np.indices((size[1],size[0]))
    # Kleine Materialdetails aus derselben Quelle, ohne die großen Bodenwellen.
    values=src[(yy*11+163)%src.shape[0],(xx*13+211)%src.shape[1]]
    idx=np.searchsorted(np.quantile(values,[.12,.3,.6,.84,.96]),values)
    colors=b.palette(color,(.94,.97,1,1.02,1.04,1.06))
    return Image.fromarray(np.array(colors,dtype=np.uint8)[idx]).convert('RGBA')

def sprite(name,size,color):
    if name!='tree_old':return base_sprite(name,size,color)
    # Eigenständige Pixelkontur; das Blattmuster stammt aus der ComfyUI-Vorlage.
    src=Image.open(v2.V2/'tree_old.png').convert('L').crop((60,260,450,510)).resize((18,20),Image.Resampling.BOX)
    lum=np.array(src)
    levels=np.quantile(lum,[.25,.55,.84])
    idx=np.searchsorted(levels,lum)
    colors=[b.rgb(h) for h in ['173D25','1E502A','396331','607A3B']]
    im=Image.new('RGBA',(18,30));d=ImageDraw.Draw(im)
    d.ellipse((1,27,10,29),fill=b.rgb('0C1A34')+(77,))
    d.line((9,19,9,29),fill=b.rgb('7A6854')+(255,),width=2)
    mask=Image.new('L',(18,30));m=ImageDraw.Draw(mask)
    m.ellipse((1,8,12,23),fill=255);m.ellipse((7,7,17,22),fill=255);m.ellipse((4,4,14,18),fill=255)
    for y in range(4,24):
        for x in range(18):
            if mask.getpixel((x,y)):
                tone=int(idx[y-4,x])
                if x<5:tone=min(tone,1)
                if x>11 and y<14:tone=max(tone,2)
                im.putpixel((x,y),colors[tone]+(255,))
    b.save(name+'_0_normal','vegetation',im,[9,29])

b.texture=texture;b.sprite=sprite

if __name__=='__main__':b.main()
