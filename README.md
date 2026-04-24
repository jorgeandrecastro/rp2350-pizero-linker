[![Crates.io](https://img.shields.io/crates/v/rp2350-pizero-linker.svg)](https://crates.io/crates/rp2350-pizero-linker)
[![License: GPL-2.0-or-later](https://img.shields.io/badge/License-GPL_2.0_or_later-blue.svg)](https://opensource.org/licenses/GPL-2.0)
[![docs.rs](https://docs.rs/rp2350-pizero-linker/badge.svg)](https://docs.rs/rp2350-pizero-linker)

# 🦅 rp2350-pizero-linker

Support de linkage et de démarrage sans configuration pour la **Waveshare RP2350-PiZero** en Rust.

Arrêtez de vous battre avec `memory.x` et les linker scripts personnalisés. `rp2350-pizero-linker` fournit une façon automatisée et transparente de démarrer vos applications `no_std` Rust sur la **RP2350-PiZero** avec la cartographie mémoire correcte et les blocs de démarrage requis.

> ⚠️ Ce crate est spécifiquement calibré pour la **Waveshare RP2350-PiZero** :
> **16 MB de Flash** (W25Q128JVS) et **520 KB de RAM** (RP2350**B**).
> Il n'est pas compatible avec une Pico 2 standard (4 MB Flash, 512 KB RAM).

----

## 🚀 Fonctionnalités

* **Zéro configuration :** Injecte automatiquement le bon `memory.x` pour la RP2350-PiZero.
* **Prêt au démarrage :** Inclut le `.start_block` et `.boot_info` requis par le BootROM du RP2350.
* **Compatible picotool :** Mappe `.bi_entries` pour que `picotool` puisse identifier votre binaire.
* **Transparent :** Utilise `INSERT AFTER` pour fonctionner de manière transparente avec `cortex-m-rt`.
* **Optimisé pour la carte :** Préconfiguré pour 16 MB de Flash et 520 KB de RAM (RP2350B).

----

## 📦 Installation

Ajoutez ceci à votre `Cargo.toml` :

```toml
[dependencies]
rp2350-pizero-linker = "0.1.0"
```

----

## 🛠 Utilisation

Dans votre `main.rs`, importez simplement le crate pour activer l'automatisation du linker :

```rust
#![no_std]
#![no_main]

// 🦅 La ligne magique : gère la cartographie mémoire et les blocs de démarrage
use rp2350_pizero_linker as _;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    // Votre code RP2350-PiZero ici...
    loop {}
}
```

**`.cargo/config.toml` recommandé**

```toml
[build]
target = "thumbv8m.main-none-eabihf"

[target.'cfg(all(target_arch = "arm", target_os = "none"))']
runner = "elf2uf2-rs -d"

rustflags = [
  "-C", "linker=flip-link",
  "-C", "link-arg=-Tlink.x",
  
  "-C", "link-arg=--nmagic",
   
]
```

----

## 🚀 Automatisation du flashage

Pour ceux qui préfèrent un contrôle manuel ou ont des problèmes de permissions avec les runners directs, voici un script `flash.sh` qui automatise la compilation, la conversion en UF2 et le flashage via `picotool`.

**📄 flash.sh**

Créez un fichier `flash.sh` à la racine de votre projet :

```bash
#!/bin/bash
export PATH="$HOME/.cargo/bin:$PATH"

# 1. Compilation
cargo build --release || exit 1

# 2. Conversion 
# On utilise le nom 'andrew-pico2' au lieu de 'iabrain'
picotool uf2 convert -t elf target/thumbv8m.main-none-eabihf/release/Yournameproject Yournameapp.uf2 --family rp2350-arm-s

# 3. Flash
# picotool a besoin d'un accès USB, sudo est souvent nécessaire ici
sudo picotool load Yournameapp.uf2 -x
```

en cas de soucis admin wsl , si la usb est branche sur le port 1
**🛠️ Comment l'utiliser :**

Rendre le script exécutable : `chmod +x flash.sh`

Lancer : `./flash.sh`

> **Note :** Ce script nécessite `picotool`. Le flag `--family rp2350-arm-s` est **obligatoire** pour la nouvelle architecture RP2350.

----

## 🗺 Cartographie mémoire

| Région | Adresse    | Taille  | Détail                          |
|--------|------------|---------|---------------------------------|
| FLASH  | 0x10000000 | 16 MB   | Flash externe W25Q128JVS (QSPI) |
| RAM    | 0x20000000 | 520 KB  | SRAM interne RP2350B (8 banques)|

----

## 🛡 Licence

Ce projet est sous licence **GPL-2.0-or-later**.
Protection de l'infrastructure open-source pour la communauté.

----

## 🦅 À propos

Développé par Jorge Andre Castro.
Adapté pour la Waveshare RP2350-PiZero avce 16 MB Flash, 520 KB RAM, RP2350B.