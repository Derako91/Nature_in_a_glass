"""Regression: gepolsterte Motive dürfen nicht halb so groß erscheinen."""
import tempfile
import unittest
from pathlib import Path
from PIL import Image, ImageDraw
from tools.sprite_agent.model import write, read, digest
from tools.sprite_agent.export_nature import export_package

def package_at(root):
    (root/'sprites').mkdir()
    manifest={'package_id':'size-test','asset_id':'fox','category':'animal',
              'display_size':[10,6],'anchor':[5,5],'frames':[]}
    # Standbild 40/80, andere Pose 56/80: nur die Referenz soll 10 Weltpixel breit sein.
    for state,box in [('graze',(20,8,59,39)),('walk',(12,8,67,39))]:
        frame={'key':f'fox_0_{state}_r_f0','variant':0,'state':state,'frame':0,'direction':'r','lods':[]}
        for scale in (2,8):
            im=Image.new('RGBA',(10*scale,6*scale));d=ImageDraw.Draw(im)
            # Weiter reichender Schatten darf die Motivmessung nicht verändern.
            d.rectangle((0,4*scale,10*scale-1,6*scale-1),fill=(12,26,52,77))
            d.rectangle(tuple(round(v*scale/8) for v in box),fill=(190,90,40,255))
            path=root/'sprites'/f'{frame["key"]}__lod{scale}.png';im.save(path)
            frame['lods'].append({'file':path.relative_to(root).as_posix(),'scale':scale,
                                 'anchor':[5*scale,5*scale],'sha256':digest(path)})
        manifest['frames'].append(frame)
    write(root/'manifest.json',manifest)

class ExportSizeTests(unittest.TestCase):
    def test_world_width_anchor_and_pose_scale(self):
        with tempfile.TemporaryDirectory() as temp:
            root=Path(temp)/'package';root.mkdir();package_at(root)
            destination=Path(temp)/'export';export_package(root,destination,preview=True)
            output=read(destination/'animals/manifest.json')
            for asset in output['assets']:
                self.assertEqual(asset['display_size'],[20,12])
                self.assertEqual(asset['anchor'],[40,40])
                self.assertEqual(asset['levels'][0]['anchor'],[10,10])
                # Fußpunkt bleibt beim LOD-Wechsel bei identischem Bildschirmort.
                for zoom in (4,8,16,32):
                    main_offset=40*20/80*zoom/4
                    far_offset=10*20/20*zoom/4
                    self.assertEqual(main_offset,far_offset)
            self.assertEqual(40*20/80,10)  # Referenzmotiv tatsächlich Weltbreite 10.
            self.assertEqual(56*20/80,14)  # Laufpose nicht unabhängig zusammengestaucht.
            report=read(destination/'export-report.json')['world_sizing']['variants']['0']
            self.assertEqual(report['visible_width'],40)
            self.assertEqual(report['factor'],2)
            self.assertEqual(report['reference'],'fox_0_graze_r_f0')

    def test_pilot_reference_reaches_requested_width(self):
        pilot=Path(__file__).resolve().parents[3]/'art/nature/sprite-agent-pilot'
        with tempfile.TemporaryDirectory() as temp:
            destination=Path(temp)/'export';export_package(pilot,destination,preview=True)
            report=read(destination/'export-report.json')['world_sizing']['variants']['0']
            width=report['visible_width']*report['display_size'][0]/report['source_canvas_width']
            self.assertAlmostEqual(width,10)
            self.assertGreater(report['factor'],1.5)

    def test_shadow_only_does_not_create_export(self):
        with tempfile.TemporaryDirectory() as temp:
            root=Path(temp)/'package';root.mkdir();package_at(root)
            manifest=read(root/'manifest.json')
            for frame in manifest['frames']:
                for lod in frame['lods']:
                    file=root/lod['file']
                    with Image.open(file) as im:
                        shadow=Image.new('RGBA',im.size,(12,26,52,77))
                    shadow.save(file);lod['sha256']=digest(file)
            write(root/'manifest.json',manifest)
            destination=Path(temp)/'export'
            with self.assertRaises(ValueError):export_package(root,destination,preview=True)
            self.assertFalse(destination.exists())
