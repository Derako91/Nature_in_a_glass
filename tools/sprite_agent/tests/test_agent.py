import copy
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch
from PIL import Image, ImageDraw
from tools.sprite_agent.model import PROFILE, load_profile, plan, validate, frames, write, read
from tools.sprite_agent.imaging import cutout, fit_frames, make_lod
from tools.sprite_agent.runner import produce, approve
from tools.sprite_agent.review import validate_package

def fixture(path):
    im=Image.new('RGB',(128,128),'white');d=ImageDraw.Draw(im)
    d.rectangle((30,40,90,80),fill='#B75D2C')
    d.rectangle((38,80,44,96),fill='#50382E')
    d.rectangle((75,80,82,96),fill='#50382E')
    d.rectangle((40,50,55,60),fill='#F4EFE2')
    im.save(path)

class FakeComfy:
    calls=0
    def __init__(self,url):pass
    def capabilities(self):return {'nodes':['IPAdapterUnifiedLoader','IPAdapterAdvanced'], 'checkpoints':['sd_xl_base_1.0.safetensors']}
    def upload(self,path):return 'ref.png'
    def request(self,path,data):
        FakeComfy.calls+=1
        return {'prompt_id':str(FakeComfy.calls)}
    def collect(self,pid,target,history_path):
        fixture(target);write(history_path,{'mock':True})

class AgentTests(unittest.TestCase):
    def setUp(self):
        self.profile=load_profile(PROFILE)
        self.job=plan('Erstelle einen Fuchs mit vier Blickrichtungen.',self.profile)[0]

    def test_four_directions_and_engine_units(self):
        self.assertEqual(len(list(frames(self.job))),4)
        self.assertEqual(self.job['anchor'],[5,5])

    def test_batch_is_explicit_catalog_selection(self):
        jobs=plan('Erstelle drei neue Tiere für Nature in a Glass.',self.profile)
        self.assertEqual([j['id'] for j in jobs],['fox','wolf','rabbit'])

    def test_all_requested_categories(self):
        for noun,kind in [('Mensch','human'),('Hütte','building'),('Korb','object'),('Baum','plant')]:
            self.assertEqual(plan('Erstelle '+noun,self.profile)[0]['category'],kind)

    def test_unknown_subject_and_vague_animation_rejected(self):
        for text in ('Erstelle einen Drachen','Erstelle Fuchs Animation'):
            with self.assertRaises(ValueError):plan(text,self.profile)

    def test_bad_lod_and_path_rejected(self):
        for field,value in [('id','../../escape'),('anchor',[100,5]),('display_size',[float('nan'),6])]:
            bad=copy.deepcopy(self.job);bad[field]=value
            with self.assertRaises(ValueError):validate(bad)
        bad=copy.deepcopy(self.job);bad['lods'][1]['scale']=1
        with self.assertRaises(ValueError):validate(bad)

    def test_multiview_img2img_rejected(self):
        self.job['strategy']='img2img'
        with self.assertRaises(ValueError):validate(self.job)

    def test_cutout_preserves_enclosed_light_fur(self):
        with tempfile.TemporaryDirectory() as tmp:
            path=Path(tmp)/'source.png';fixture(path)
            im,info=cutout(path)
            self.assertEqual(im.getpixel((15,15))[3],255)
            self.assertEqual(info['method'],'border_connected_color_key')
            self.assertEqual(im.getpixel((0,im.height-1))[3],0)

    def test_empty_image_fails(self):
        with tempfile.TemporaryDirectory() as tmp:
            path=Path(tmp)/'empty.png';Image.new('RGBA',(32,32)).save(path)
            with self.assertRaises(ValueError):cutout(path)

    @patch('tools.sprite_agent.runner.Comfy',FakeComfy)
    def test_end_to_end_resume_and_approval_guards(self):
        FakeComfy.calls=0
        with tempfile.TemporaryDirectory() as tmp:
            base=Path(tmp);jobfile=base/'job.json';write(jobfile,self.job)
            folder=produce(jobfile,base/'out','mock')
            self.assertEqual(FakeComfy.calls,4)
            report=read(folder/'validation.json')
            self.assertEqual(report['technical_status'],'passed')
            self.assertEqual(report['visual_status'],'pending_human_review')
            self.assertFalse(report['approved'])
            self.assertEqual(report['file_count'],16)
            self.assertTrue((folder/'review.html').exists())
            produce(jobfile,base/'out','mock',resume=folder)
            self.assertEqual(FakeComfy.calls,4)
            with self.assertRaises(ValueError):approve(folder,'Tester','Nur Technik',False)
            manifest=read(folder/'manifest.json')
            incomplete=copy.deepcopy(manifest);incomplete['frames'][0]['lods'].pop()
            self.assertEqual(validate_package(folder,incomplete,list(frames(self.job)),persist=False)['technical_status'],'failed')
            one=folder/manifest['frames'][0]['lods'][0]['file']
            original=one.read_bytes();one.unlink()
            bad=validate_package(folder,manifest,list(frames(self.job)),persist=False)
            self.assertEqual(bad['technical_status'],'failed')
            one.write_bytes(original)
            approve(folder,'Testfixture','Synthetischer Test, keine echte Freigabe',True)
            with self.assertRaises(ValueError):produce(jobfile,base/'out','mock',resume=folder)
            with self.assertRaises(ValueError):approve(folder,'Tester','Nochmals',True)

    @patch('tools.sprite_agent.runner.Comfy',FakeComfy)
    def test_budget_fails_before_gpu_submission(self):
        with tempfile.TemporaryDirectory() as tmp:
            self.job['budget']=1;path=Path(tmp)/'job.json';write(path,self.job)
            FakeComfy.calls=0
            with self.assertRaises(ValueError):produce(path,Path(tmp)/'out','mock')
            self.assertEqual(FakeComfy.calls,0)

    @patch('tools.sprite_agent.runner.Comfy',FakeComfy)
    def test_other_categories_export_without_renderer_assumptions(self):
        with tempfile.TemporaryDirectory() as tmp:
            for noun in ('Mensch','Hütte','Korb','Baum'):
                job=plan('Erstelle '+noun,self.profile)[0]
                job['directions']=['se']
                path=Path(tmp)/(job['id']+'.json');write(path,job)
                folder=produce(path,Path(tmp)/'out','mock')
                self.assertEqual(read(folder/'validation.json')['technical_status'],'passed')
                self.assertEqual(read(folder/'manifest.json')['display_size'],job['display_size'])

    def test_ambiguous_submission_is_not_resent(self):
        class LostReply(FakeComfy):
            def request(self,path,data):
                FakeComfy.calls+=1
                raise OSError('Verbindung nach Annahme unterbrochen')
        FakeComfy.calls=0
        with tempfile.TemporaryDirectory() as tmp, patch('tools.sprite_agent.runner.Comfy',LostReply):
            path=Path(tmp)/'job.json';write(path,self.job)
            out=Path(tmp)/'out'
            with self.assertRaises(OSError):produce(path,out,'mock')
            folder=next(p for p in out.iterdir() if p.is_dir())
            with self.assertRaises(RuntimeError):produce(path,out,'mock',resume=folder)
            self.assertEqual(FakeComfy.calls,1)

if __name__=='__main__':unittest.main()
