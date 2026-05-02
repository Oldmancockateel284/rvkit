# rvkit — Cahier des Charges v2

> _"Bare metal Zig, without the bare metal pain."_

**CLI Rust + Framework Embedded Zig pour RISC-V**

Open & Hack · Karagure · Mars 2026

---

## 1. Problème à résoudre

Le développement bare metal en Zig sur architecture RISC-V implique une configuration manuelle fastidieuse : adressage mémoire, registres périphériques, linker scripts, toolchain — chaque board nécessite une mise en place spécifique et chronophage.

Il n'existe aujourd'hui aucun équivalent à Arduino ou PlatformIO pour l'écosystème Zig/RISC-V. Le développeur embedded Zig doit :

- Configurer manuellement les linker scripts et memory maps pour chaque chip
- Écrire le startup code et le vecteur d'interruptions from scratch
- Comprendre les registres MMIO spécifiques à chaque famille de microcontrôleurs
- Gérer le build system, la cross-compilation, et le flashage à la main
- Réécrire ses drivers de zéro à chaque nouveau projet ou changement de board

**rvkit résout ce problème** en fournissant un CLI (Rust) qui orchestre le workflow de développement, couplé à un framework Zig (HAL, BSP, drivers) qui abstrait les spécificités hardware. L'objectif : permettre au développeur de démarrer un projet embedded Zig fonctionnel en moins de 2 minutes.

---

## 2. Utilisateurs cibles

rvkit cible principalement trois profils :

- **Makers et hobbyistes** — qui veulent prototyper sur des boards RISC-V abordables (CH32V003 à ~0.10€, ESP32-C3) sans se battre contre la toolchain.
- **Étudiants en électronique/embarqué** — qui découvrent le bare-metal et ont besoin d'un point d'entrée accessible, similaire à l'expérience Arduino.
- **Startups hardware / petites structures** — qui cherchent une alternative moderne au C pour le prototypage rapide sur RISC-V, sans investir dans une infrastructure de développement lourde.

Le marché professionnel embarqué traditionnel (C/C++) n'est pas la cible prioritaire, bien que rvkit puisse séduire des profils curieux de Zig cherchant une alternative moderne au C.

---

## 3. Architecture dual-langage

rvkit suit le même modèle qu'Arduino : l'IDE Arduino est en Java, mais les projets sont en C/C++. Ici, **l'outil (CLI) est en Rust, les projets sont en Zig.**

| Composant | Langage | Rôle |
|---|---|---|
| CLI (Layer 5) | **Rust** | Orchestration : new, build, flash, monitor, pkg. Seul composant en Rust. |
| HAL (Layer 3) | **Zig** | Interfaces abstraites pour les périphériques : GPIO, UART, SPI, I2C, Timer, ADC, PWM. |
| BSP (Layer 2) | **Zig** | Board Support Packages : implémentation du HAL pour chaque chip (registres, clock, linker). |
| Drivers (Layer 4) | **Zig** | Drivers de périphériques externes (SSD1306, WS2812, BME280...) portables via le HAL. |
| Chip (Layer 1) | **Zig** | Définitions des registres MMIO, auto-générées depuis les fichiers SVD. |

Le CLI Rust invoque le compilateur Zig pour builder les projets utilisateurs. Le framework Zig est distribué comme dépendance Zig que le `build.zig` du projet importe.

---

## 4. Fonctionnalités

### 4.1 CLI (Rust) — Commandes

Priorisation MoSCoW :

| Priorité | Commande | Description | Statut |
|---|---|---|---|
| **Must** | `rvkit new --board <t> <n>` | Génère un projet Zig préconfiguré pour la board cible | ✓ Implémenté |
| **Must** | `rvkit build` | Compile le projet via `zig build` | ✓ Implémenté |
| **Must** | `rvkit flash` | Flashe le firmware (wlink / esptool selon board) | ✓ Implémenté |
| **Must** | `rvkit monitor` | Moniteur série intégré (UART) | ○ À faire |
| **Should** | `rvkit boards` | Liste les boards supportées | ✓ Implémenté |
| **Should** | Config `rvkit.toml` | Fichier de configuration projet (board, flash port, baud) | ✓ Implémenté |
| **Could** | `rvkit pkg` | Gestionnaire de packages (drivers, BSP tiers) | ○ Roadmap |
| **Could** | TUI interactif | Interface TUI complète (ratatui) pour toutes les commandes | ○ Roadmap |

### 4.2 Framework Zig — Couches

**Layer 1 — Chip (Register Definitions)**
Définitions des registres MMIO pour chaque chip, auto-générées à partir des fichiers SVD. Aucune logique — juste une carte mémoire typée du hardware. Le projet `regz` (Zig) peut servir de base pour la génération.

**Layer 2 — BSP (Board Support Packages)**
Chaque BSP encapsule tout ce qui est spécifique à un board : linker script, configuration mémoire, vecteur d'interruptions, startup code, configuration de clock. Le BSP implémente les interfaces HAL pour les périphériques de son chip.

**Layer 3 — HAL (Hardware Abstraction Layer)**
Le cœur du framework. Définit des interfaces comptime Zig pour chaque type de périphérique : GPIO, UART, SPI, I2C, Timer, ADC, PWM. Les drivers programment contre ces interfaces, pas contre un chip spécifique. L'abstraction est zero-cost grâce au duck typing comptime de Zig (même pattern que `std.io.Reader`/`Writer`).

**Layer 4 — Drivers**
Drivers de périphériques externes (écrans, capteurs, LEDs, modules radio). Écrits en Zig pur, ils ne dépendent que du HAL. Un driver SSD1306 écrit avec l'interface I2C du HAL fonctionne sur n'importe quel board qui implémente cette interface. C'est la couche principale de contribution communautaire.

### 4.3 Expérience utilisateur cible

```bash
$ rvkit new --board ch32v003 my_blinky
  ✓ Created project 'my_blinky'
  ✓ Board: CH32V003 (QingKe V2A, 48MHz, 2K RAM / 16K Flash)

$ cd my_blinky && cat src/main.zig
```

```zig
const rvkit = @import("rvkit");
const board = rvkit.board;

pub fn main() void {
    const led = board.gpio(.pc13, .{ .direction = .output });
    while (true) {
        led.toggle();
        rvkit.delay_ms(500);
    }
}
```

```bash
$ rvkit build
  ✓ Compiling for ch32v003 (riscv32-none-elf)
  ✓ Binary: 1.2 KB

$ rvkit flash
  ✓ Flashing via WCH-Link...
  ✓ Done!
```

**Promesse :** 3 commandes, 10 lignes de code, et ta LED clignote. Pas de linker script à écrire. Pas de startup code à copier. Pas de Makefile cryptique.

---

## 5. Boards supportées

### 5.1 Boards v1

| Board | Architecture | Flash tool | Notes |
|---|---|---|---|
| **CH32V003** | RISC-V 32bit (QingKe V2A) | wlink (WCH-LinkE) | Ultra low-cost (~0.10€), cible maker principale |
| **ESP32-C3** | RISC-V 32bit (RV32IMC) | esptool | WiFi/BLE intégré, très populaire |

**Architecture extensible :** chaque board est un BSP indépendant. On peut en ajouter un nouveau sans modifier le core. La communauté peut contribuer des BSP pour d'autres boards RISC-V (GD32VF103, BL602, etc.).

### 5.2 Boards hors scope v1

- Toute board non RISC-V (ARM, Xtensa, AVR)
- Les boards RISC-V exotiques ou peu répandues dans la communauté maker
- L'ESP-WROOM-32 (Xtensa, pas RISC-V) — potentiellement supporté en v2+ si rvkit s'ouvre au-delà du RISC-V

---

## 6. Structure du projet

### 6.1 Repository rvkit

Le repository contient deux parties distinctes : le CLI Rust et le framework Zig.

```
rvkit/
├── Cargo.toml                  # Projet Rust (CLI)
├── src/                        # CLI Rust
│   ├── main.rs
│   ├── boards.rs
│   └── commands/
│       ├── new.rs
│       ├── build.rs
│       ├── flash.rs
│       ├── monitor.rs
│       └── boards.rs
│
├── framework/                  # Framework Zig (Layers 1–4)
│   ├── build.zig
│   ├── hal/
│   │   ├── hal.zig             #   Export public
│   │   ├── gpio.zig
│   │   ├── uart.zig
│   │   ├── spi.zig
│   │   ├── i2c.zig
│   │   ├── timer.zig
│   │   ├── adc.zig
│   │   └── pwm.zig
│   ├── core/
│   │   ├── mmio.zig
│   │   ├── interrupt.zig
│   │   └── clock.zig
│   ├── bsp/
│   │   ├── ch32v003/
│   │   │   ├── chip.zig
│   │   │   ├── gpio.zig
│   │   │   ├── uart.zig
│   │   │   ├── clock.zig
│   │   │   ├── linker.ld
│   │   │   └── board.zig
│   │   └── esp32_c3/
│   │       ├── chip.zig
│   │       ├── gpio.zig
│   │       ├── uart.zig
│   │       ├── clock.zig
│   │       ├── linker.ld
│   │       └── board.zig
│   └── drivers/
│       ├── display/ssd1306.zig
│       ├── led/ws2812.zig
│       └── sensor/bme280.zig
│
├── linker/                     # Linker scripts (embarqués par le CLI)
│   ├── ch322v003.ld
│   └── esp32c3.ld
│
├── templates/                  # Templates générés par rvkit new
│   ├── blinky/
│   └── hello_uart/
│
├── examples/
├── docs/
├── LICENSE                     # MIT
└── README.md
```

### 6.2 Projet généré par `rvkit new`

Voici ce que l'utilisateur obtient après `rvkit new --board ch32v003 my_project` :

```
my_project/
├── rvkit.toml              # Config board + flash
├── build.zig               # Build préconfiguré (importe le framework)
├── src/
│   └── main.zig            # Code utilisateur
└── linker/
    └── ch32v003.ld         # Linker script (généré, ne pas éditer)
```

### 6.3 rvkit.toml

```toml
[project]
name = "my_project"
board = "ch32v003"

[flash]
tool = "wlink"
port = "/dev/ttyUSB0"
baud_rate = 115200

[dependencies]
# drivers tiers (futur)
# ssd1306 = { version = "0.1.0" }
```

---

## 7. Contraintes techniques

### 7.1 OS supportés

| OS | Support | Notes |
|---|---|---|
| **Linux** | Cible principale | Priorité de développement et de CI |
| macOS | Best-effort | Pas de CI dédiée en v1 |
| Windows | Best-effort | Pas de CI dédiée en v1 |

### 7.2 Dépendances

| Dépendance | Langage | Rôle | Type |
|---|---|---|---|
| Rust stable | Rust | Compilation du CLI rvkit | Obligatoire |
| Zig | Zig | Compilation des projets embedded | Obligatoire |
| wlink | Rust | Flash CH32V003 via WCH-LinkE | Requis pour CH32V003 |
| esptool | Python | Flash ESP32-C3 | Requis pour ESP32-C3 |
| clap | Rust crate | Parsing des commandes CLI | Dépendance interne |
| serde + toml | Rust crate | Parsing rvkit.toml | Dépendance interne |
| serialport | Rust crate | Communication série (monitor) | Dépendance interne |

**Philosophie :** zéro dépendance surprise. rvkit vérifie au démarrage que les outils nécessaires sont installés et guide l'utilisateur si quelque chose manque. Rust stable uniquement, pas de nightly.

---

## 8. Hors scope v1

### Ce que rvkit n'est PAS

- ❌ **Pas un IDE** — l'utilisateur garde son éditeur (Neovim, Zed, VS Code). rvkit ne remplace pas ZLS.
- ❌ **Pas un gestionnaire de packages Zig** — c'est le rôle du package manager Zig natif. `rvkit pkg` gèrera les drivers/BSP tiers, pas les dépendances Zig génériques.
- ❌ **Pas un simulateur/émulateur RISC-V** — on flash du vrai hardware.
- ❌ **Pas un outil de debug avancé** — GDB et OpenOCD restent des outils séparés en v1.
- ❌ **Pas un concurrent d'Embassy** — Embassy est un framework Rust/ARM avec un async executor. rvkit est un framework Zig/RISC-V sans runtime.

---

## 9. Positionnement concurrentiel

| | rvkit | Arduino | Embassy | ESP-IDF |
|---|---|---|---|---|
| Langage outil | Rust | Java | Rust | CMake/Python |
| Langage projets | Zig | C/C++ | Rust | C |
| Architecture | RISC-V first | AVR / ARM | ARM first | Xtensa / RV |
| Runtime | Aucun | Minimal | Async executor | FreeRTOS |
| HAL portable | Oui (comptime) | Oui (C++) | Oui (traits) | Non (ESP only) |
| Licence | MIT | LGPL | Apache 2.0 | Apache 2.0 |

**Différenciation clé :** rvkit occupe un espace vierge — il n'existe aucun framework Zig embedded unifié ciblant RISC-V. C'est le « premier arrivé » sur ce créneau.

---

## 10. Roadmap

### Phase 1 — Fondations (Q1–Q2 2026)

**Objectif :** un blinky qui compile sur CH32V003 et ESP32-C3 avec la même interface HAL, sans changer le `main.zig`.

- Créer le dossier `framework/` avec la structure HAL / BSP
- Extraire l'interface HAL GPIO depuis le code CH32V003 existant
- Implémenter le BSP ESP32-C3 avec les mêmes interfaces
- Extraire l'interface HAL UART (déjà debuggé sur CH32V003)
- Valider la portabilité cross-board
- Documentation de base + README sérieux

### Phase 2 — Écosystème (Q3–Q4 2026)

**Objectif :** un framework complet avec tous les périphériques de base et les premiers drivers.

- Ajouter SPI, I2C, Timer, ADC, PWM au HAL
- Premiers drivers : SSD1306 (OLED), WS2812 (NeoPixel), BME280
- Implémenter `rvkit monitor` (terminal série via ratatui)
- Système de templates : `rvkit new --template hello_uart`
- Tooling SVD → Zig pour faciliter l'ajout de nouveaux chips

### Phase 3 — Communauté (2027)

**Objectif :** un écosystème ouvert où la communauté peut contribuer sans toucher au cœur.

- Système de packages (`rvkit pkg install` / `publish`)
- Registry de packages communautaires
- Support de boards RISC-V supplémentaires (GD32VF103, BL602, etc.)
- Exploration du support ARM / Xtensa
- TUI interactif complet
- Site web + documentation en ligne

---

## 11. Modèle business

rvkit suit un modèle **open-core** :

- **Open-source (MIT) :** le CLI, le framework, le HAL, les BSP et les drivers de base. Tout ce qui est nécessaire pour développer.
- **Payant :** boards custom Open & Hack (hardware RISC-V propriétaire), support étendu, BSP privés pour clients industriels, formations.

L'objectif long terme est de créer un écosystème viable autour de rvkit où Open & Hack vend le hardware et les services, tandis que le software reste libre et communautaire.

---

## 12. Récapitulatif stack technique

| Couche | Technologie | Langage |
|---|---|---|
| CLI / Commandes | clap + serde + toml | Rust stable |
| TUI Monitor | ratatui + serialport | Rust stable |
| Flash CH32V003 | wlink | Rust (externe) |
| Flash ESP32-C3 | esptool | Python (externe) |
| Framework / HAL | Interfaces comptime | Zig |
| BSP | MMIO + linker scripts | Zig |
| Drivers | Via interfaces HAL | Zig |
| Config | rvkit.toml | TOML |

---

_rvkit — Bare metal Zig, without the bare metal pain._
_Open & Hack · MIT License · Karagure_
_Document créé le 10/03/2026 — v2.0_
