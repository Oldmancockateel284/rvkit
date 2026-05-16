use crate::boards;
use std::fs;
use std::path::Path;

pub fn run(board_name: &str, name: &str) {
    let board = boards::get(board_name).unwrap_or_else(|| {
        eprintln!(
            "Board '{}' is not supported. Run 'rvkit boards' to see available boards.",
            board_name
        );
        std::process::exit(1);
    });

    println!("Creating project '{}' for board '{}'...", name, board.name);

    let root = Path::new(name);
    if root.exists() {
        eprintln!("Error: directory '{}' already exists.", name);
        std::process::exit(1);
    }

    fs::create_dir_all(root.join("src")).expect("Failed to create src/");
    fs::create_dir_all(root.join("linker")).expect("Failed to create linker/");

    fs::write(
        root.join("linker").join(format!("{}.ld", board.name)),
        board.linker_script,
    )
    .expect("Failed to write linker script");

    let toml = format!(
        "[project]\nname = \"{}\"\nboard = \"{}\"\n",
        name, board.name
    );
    fs::write(root.join("rvkit.toml"), toml).expect("Failed to write rvkit.toml");

    let main_zig = r#"const std = @import("std");

    export fn _start() callconv(.c) noreturn {
        main();
        while (true) {}
    }

    pub fn main() void {
    // Your bare metal code here
    }
    "#;

    fs::write(root.join("src/main.zig"), main_zig).expect("Failed to write main.zig");

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

        exe.setLinkerScript(b.path("linker/{}.ld"));

        b.installArtifact(exe);
    }}
    "#,
        board.cpu_arch, name, board.name
    );
    fs::write(root.join("build.zig"), build_zig).expect("Failed to write build.zig");

    println!("✓ Project '{}' created for board '{}'", name, board.name);
    println!("  → cd {}", name);
    println!("  → rvkit build");
}
