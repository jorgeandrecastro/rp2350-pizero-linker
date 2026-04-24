#![no_std]
#![forbid(unsafe_code)]

//! # rp2350-pizero-linker
//!
//! Infrastructure de linkage zero-config pour la **Waveshare RP2350-PiZero**.
//!
//! ## Rôle de ce crate
//!
//! Ce crate ne contient **aucun code exécutable**. Son unique responsabilité
//! est d'injecter automatiquement le linker script `memory.x` adapté à la
//! RP2350-PiZero via son `build.rs`, au moment de la compilation.
//!
//! ## Ce qui est injecté
//!
//! | Région | Adresse      | Taille  | Détail                           |
//! |--------|--------------|---------|----------------------------------|
//! | FLASH  | `0x10000000` | 16 MB   | Flash externe W25Q128JVS (QSPI)  |
//! | RAM    | `0x20000000` | 520 KB  | SRAM interne RP2350B (8 banques) |
//!
//! Les sections suivantes sont également configurées :
//! - `.start_block` — bloc de démarrage requis par le BootROM du RP2350
//! - `.boot_info`   — informations de boot embarquées dans le bloc de démarrage
//! - `.bi_entries`  — métadonnées pour `picotool`
//!
//! ## Utilisation
//!
//! ```toml
//! # Cargo.toml
//! [dependencies]
//! rp2350-pizero-linker = "0.1.0"
//! ```
//!
//! ```rust,no_run
//! #![no_std]
//! #![no_main]
//!
//! // La ligne magique active le linkage et le boot block
//! use rp2350_pizero_linker as _;
//! ```
//!
//! ## ⚠️ Compatibilité
//!
//! Cette crate est **exclusivement** calibré pour la Waveshare RP2350-PiZero
//! (16 MB Flash, 520 KB RAM, RP2350**B**).
//! Il n'est **pas** compatible avec une Raspberry Pi Pico 2 standard
//! (4 MB Flash, 512 KB RAM, RP2350A).