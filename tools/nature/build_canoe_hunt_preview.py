"""Nur zur Bildabnahme: temporärer Beobachter, keine Änderung an Simulation oder Produktivdateien."""
import argparse
import difflib
import os
from pathlib import Path
import shutil
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[2]
ART = ROOT / 'art/nature/canoe-and-hunt-style-b'


def build(output):
    view = ROOT/'crates/app/src/view.rs'
    main = ROOT/'crates/app/src/main.rs'
    originals = {view:view.read_bytes(), main:main.read_bytes()}
    v = originals[view].decode()
    needle = '    let persons = ap::person_sprites(&sim.people, world);'
    assert v.count(needle) == 1
    v = v.replace(needle, '''    // ENTWURFSPROBE: nur lokale Renderkopien, keine Bewohnersteuerung.
    let mut persons = ap::person_sprites(&sim.people, world);
    for p in &mut persons {
        if p.activity == "fish" && p.tool == Some("boat") {
            if let Some(sea) = world.neighbors(p.tile as usize).find(|&j| world.tiles[j].is_ocean()) {
                p.tile = sea as u32;
                p.prev_tile = p.tile;
            }
        }
        // Bildgalerie: Person 8 zeigt Jagdbilder, keine echte Jagdbehauptung.
        if p.id == 8 && !p.child {
            p.activity = "hunt";
            p.tool = None;
        }
    }''')
    v = v.replace('"hurt" | "build" => (4, 250.0),','"hurt" | "build" | "hunt" => (4, 250.0),')
    needle = '                    Thing::Body(look, dir) => {'
    assert v.count(needle) == 1
    # Ein dekoratives Hirsch-Prüfmotiv neben der ausgewählten Jagdfigur.
    insertion = '''                        if p.id == 8 {
                            let n = "deer_hunted_0_rest_r_f0";
                            if let Some(pk) = atlas.pick(n, n, 21.6, scale) {
                                let sample = foot + Vec2::new(18.0 * scale, 2.0 * scale);
                                tex_rect(&mut mesh, pk.rect(sample), &pk.entry, tint);
                            }
                        }
'''
    before, after = v.split(needle)
    close = before.rfind('                    }')
    assert close != -1
    v = before[:close] + insertion + before[close:] + needle + after
    m = originals[main].decode()
    needle = '        self.take_from_clock();'
    assert m.count(needle) == 1
    m = m.replace(needle, '''        egui::Area::new(egui::Id::unique("codex_art_preview"))
            .fixed_pos(egui::pos2(12.0, 72.0))
            .order(egui::Order::Foreground)
            .show(ui.ctx(), |ui| {
                egui::Frame::new().fill(Color32::from_rgb(45, 30, 10)).show(ui, |ui| {
                    ui.label(RichText::new("ENTWURFSPROBE: Meerplatzierung vorläufig; Jagdfigur/Hirsch nur Bildgalerie")
                        .color(Color32::from_rgb(255, 225, 130)));
                });
            });
        self.take_from_clock();''')
    changed = {view:v,main:m}
    patch = ''.join(''.join(difflib.unified_diff(originals[p].decode().splitlines(True),s.splitlines(True),
                    fromfile='a/'+str(p.relative_to(ROOT)),tofile='b/'+str(p.relative_to(ROOT)))) for p,s in changed.items())
    (ART/'review-preview.patch').write_text(patch)
    executable=ROOT/'target/release'/('nature-in-a-glass.exe' if os.name=='nt' else 'nature-in-a-glass')
    with tempfile.TemporaryDirectory(prefix='codex-art-baseline-') as temp:
        backup=Path(temp)/executable.name
        if executable.exists():shutil.copy2(executable,backup)
        try:
            for p,s in changed.items():p.write_text(s)
            subprocess.run(['cargo','build','--release','-p','nature-in-a-glass'],cwd=ROOT,check=True)
            output.parent.mkdir(parents=True,exist_ok=True)
            shutil.copy2(executable,output)
        finally:
            for p,data in originals.items():p.write_bytes(data)
            if backup.exists():shutil.copy2(backup,executable)
    assert all(p.read_bytes()==data for p,data in originals.items())
    print(f'Entwurfsbeobachter: {output}; Produktivdateien und vorheriges Programm zurückgesetzt.')


if __name__ == '__main__':
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output',type=Path,required=True)
    build(parser.parse_args().output.resolve())
