//! build.rs  Script de compilation pour rp2350-pizero-linker
//!
//! Ce fichier est exécuté automatiquement par Cargo AVANT la compilation
//! du crate. Son rôle : copier le linker script `memory.x` dans le
//! répertoire de sortie (OUT_DIR) afin que l'éditeur de liens (ld) puisse
//! le trouver au moment du linkage.
//!
//! Pourquoi OUT_DIR et pas la racine du projet ?
//! Cargo impose que les artefacts générés à la volée soient placés dans
//! OUT_DIR. C'est ce répertoire qui est ajouté au chemin de recherche de
//! l'éditeur de liens via `cargo:rustc-link-search`.

use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    // Récupère le répertoire de sortie fourni par Cargo (ex: target/thumbv8m…/release/build/…)
    // La variable OUT_DIR est toujours définie par Cargo — unwrap() sans risque ici.
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    // Copie memory.x (embarqué dans le binaire du build script via include_bytes!)
    // dans OUT_DIR. C'est ce fichier que cortex-m-rt et l'éditeur de liens liront
    // pour connaître la cartographie mémoire de la carte (Flash 16MB, RAM 520KB).
    fs::write(out.join("memory.x"), include_bytes!("memory.x")).unwrap();

    // Indique à Cargo d'ajouter OUT_DIR au chemin de recherche des linker scripts.
    // Sans cette ligne, l'éditeur de liens ne trouverait pas memory.x même s'il
    // est bien présent dans OUT_DIR.
    println!("cargo:rustc-link-search={}", out.display());

    // Invalide le cache du build script si l'un de ces fichiers change.
    // Sans ces directives, Cargo ne relancerait pas build.rs après une
    // modification de memory.x, et l'ancien script resterait en cache.
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=memory.x");
}