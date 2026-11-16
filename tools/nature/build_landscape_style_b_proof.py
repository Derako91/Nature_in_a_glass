"""Synthetischer Lesbarkeitstest mit Erwachsenen/Kind auf drei Böden."""
from PIL import Image,ImageDraw,ImageFont
from pathlib import Path
import json
r=Path(__file__).resolve().parents[2];a=r/'assets/nature';m=json.load(open(a/'manifest.json'));e={x['id']:x for x in m['assets']};p=json.load(open(a/'people/manifest.json'));font=ImageFont.truetype('DejaVuSans.ttf',15)
ids=['tree_young','tree_mid','tree_old','shrub','snag_standing','log_fallen'];grounds=['ground_soil','ground_grass_wet','ground_burnt'];out=Image.new('RGB',(1536,850),'#344032');d=ImageDraw.Draw(out);d.text((12,8),'Stil B / Weltmaß bei Zoom 16 / synthetischer Lesbarkeitstest, keine Spielaufnahme',font=font,fill='#eddfc4')
def paste(im,anchor,foot,factor):
 s=im.resize((round(im.width*factor),round(im.height*factor)),Image.Resampling.NEAREST);out.paste(s,(round(foot[0]-anchor[0]*factor),round(foot[1]-anchor[1]*factor)),s)
for row,g in enumerate(grounds):
 for col,name in enumerate(ids):
  x=col*256;y=40+row*270;entry=e[name];state=entry['states'][0 if row<2 else 1];d.text((x+6,y+4),name+' / '+state,font=font,fill='#eddfc4');d.text((x+6,y+23),g,font=font,fill='#bcb593');ground=Image.open(a/f'ground/{g}_0.png').convert('RGBA');out.paste(ground,(x,y+130),ground)
  im=Image.open(a/f'vegetation/{name}_0_{state}.png').convert('RGBA');paste(im,entry['anchor'],(x+92,y+195),entry['display_size'][0]*4/im.width)
  for id,look,direction,foot in [('person',0,'r',(x+177,y+208)),('person_child',2,'l',(x+209,y+215))]:
   entry=next(t for t in p['assets'] if t['id']==id and t['state']=='idle' and f'{id}_{look}_idle_{direction}_f0.png' in t['files']);im=Image.open(a/f'people/{id}_{look}_idle_{direction}_f0.png').convert('RGBA');paste(im,entry['anchor'],foot,entry['display_size'][0]*4/im.width)
out.save(r/'art/nature/landscape-style-b/readability.png')
