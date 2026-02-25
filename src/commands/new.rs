use crate::boards;
use std::fs;
use std::path::Path;

pub fn run(board_name: &str, name: &str) {
    let board = boards::get(board_name).unwrap_or_else(|| {
        eprintln!(
            "Board '{}' non supportée. Lance 'rvkit boards' pour voir les boards disponibles.",
            board_name
        );
        std::process::exit(1);
    });

    println!(
        "Création du projet '{}' pour la board '{}'...",
        name, board.name
    );

    let root = Path::new(name);
    if root.exists() {
        eprintln!("Erreur : le dossier '{}' existe déjà.", name);
        std::process::exit(1);
    }

    fs::create_dir_all(root.join("src")).expect("Impossible de créer src/");
    fs::create_dir_all(root.join("linker")).expect("Impossible de créer linker/");

    // Copie le linker script de la board
    fs::write(
        root.join("linker").join(format!("{}.ld", board.name)),
        board.linker_script,
    )
    .expect("Impossible d'écrire le linker script");

    // Génère rvkit.toml
    let toml = format!(
        "[project]\nname = \"{}\"\nboard = \"{}\"\n",
        name, board.name
    );
    fs::write(root.join("rvkit.toml"), toml).expect("Impossible d'écrire rvkit.toml");

    // Génère src/main.zig
    let main_zig = r#"const std = @import("std");

pub fn main() void {
    // Ton code bare metal ici
}
"#;
    fs::write(root.join("src/main.zig"), main_zig).expect("Impossible d'écrire main.zig");

    // Génère build.zig
    let build_zig = format!(
        r#"const std = @import("std");

pub fn build(b: *std.Build) void {{
    const target = b.resolveTargetQuery(.{{
        .cpu_arch = .{},
        .os_tag = .freestanding,
        .abi = .none,
    }});

    const exe = b.addExecutable(.{{
        .name = "{}",
        .root_module = b.createModule(.{{
            
            .root_source_file = b.path("src/main.zig"),
            .target = target,
            .optimize = .ReleaseSmall,
        }}),
    }});

    b.installArtifact(exe);
}}
"#,
        board.cpu_arch, name
    );
    fs::write(root.join("build.zig"), build_zig).expect("Impossible d'écrire build.zig");

    println!("✓ Projet '{}' créé pour la board '{}'", name, board.name);
    println!("  → cd {}", name);
    println!("  → rvkit build");
}
