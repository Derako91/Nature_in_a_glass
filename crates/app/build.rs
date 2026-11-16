//! Bettet die Natur-Assets (PNG und Manifeste) in das Programm ein, damit der
//! Download ohne zusätzlichen Ordner auskommt. Prüfblätter (`_*.png`) und
//! Entwürfe bleiben draußen. Jeder Bildordner darf ein eigenes `manifest.json`
//! haben; alle werden eingebettet.

use std::fs;
use std::path::Path;

const GROUPS: [&str; 8] = [
    "ground",
    "walls",
    "water",
    "vegetation",
    "animals",
    "people",
    "objects",
    "buildings",
];

fn main() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../assets/nature");
    println!("cargo:rerun-if-changed={}", root.display());
    let mut entries = Vec::new();
    for g in GROUPS {
        let dir = root.join(g);
        println!("cargo:rerun-if-changed={}", dir.display());
        let Ok(rd) = fs::read_dir(&dir) else { continue };
        let mut files: Vec<_> = rd
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.extension().is_some_and(|x| x == "png"))
            .collect();
        files.sort();
        for f in files {
            let name = f.file_stem().unwrap().to_string_lossy().to_string();
            entries.push(format!(
                "    ({:?}, {:?}, include_bytes!({:?})),",
                name,
                g,
                f.display().to_string()
            ));
        }
    }
    // Manifeste: das Hauptmanifest und je Ordner ein eigenes, falls vorhanden
    // (z. B. `animals/`, `people/`, `buildings/`, `objects/`).
    let mut manifests = Vec::new();
    for p in std::iter::once("manifest.json".to_string())
        .chain(GROUPS.iter().map(|g| format!("{g}/manifest.json")))
    {
        let f = root.join(&p);
        println!("cargo:rerun-if-changed={}", f.display());
        if f.exists() {
            manifests.push(format!("    include_str!({:?}),", f.display().to_string()));
        }
    }
    let code = format!(
        "/// Eingebettete Bilder: (Dateiname ohne Endung, Ordner, PNG-Daten).\n\
         pub static IMAGES: &[(&str, &str, &[u8])] = &[\n{}\n];\n\
         /// Alle eingebetteten Manifeste.\n\
         pub static MANIFESTS: &[&str] = &[\n{}\n];\n",
        entries.join("\n"),
        manifests.join("\n"),
    );
    let out = Path::new(&std::env::var("OUT_DIR").unwrap()).join("assets.rs");
    fs::write(out, code).unwrap();
}
