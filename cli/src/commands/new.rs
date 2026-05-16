use crate::boards;
use std::fs;

pub fn run(board_name: &str, name: &str) {
    let board = boards::get(board_name).unwrap_or_else(|| {
        eprintln!(
            "Board '{}' is not supported. Run 'rvkit boards' to see available boards.",
            board_name
        );
        std::process::exit(1);
    });

    println!("Creating project '{}' for board '{}'...", name, board.name);

    let cwd = std::env::current_dir().unwrap_or_else(|_| {
        eprintln!("Error: cannot determine current directory.");
        std::process::exit(1);
    });
    let root = cwd.join(name);
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

    let main_zig = "const std = @import(\"std\");\n\nexport fn _start() callconv(.c) noreturn {\n    main();\n    while (true) {}\n}\n\nfn main() void {\n    // Your bare metal code here\n}\n";

    fs::write(root.join("src/main.zig"), main_zig).expect("Failed to write main.zig");

    let build_zig = format!(
        "const std = @import(\"std\");\n\npub fn build(b: *std.Build) void {{\n    const target = b.resolveTargetQuery(.{{\n        .cpu_arch = .{},\n        .os_tag = .freestanding,\n        .abi = .none,\n    }});\n\n    const exe = b.addExecutable(.{{\n        .name = \"{}\",\n        .root_module = b.createModule(.{{\n            .root_source_file = b.path(\"src/main.zig\"),\n            .target = target,\n            .optimize = .ReleaseSmall,\n        }}),\n    }});\n\n    exe.setLinkerScript(b.path(\"linker/{}.ld\"));\n\n    b.installArtifact(exe);\n}}\n",
        board.cpu_arch, name, board.name
    );
    fs::write(root.join("build.zig"), build_zig).expect("Failed to write build.zig");

    println!("✓ Project '{}' created for board '{}'", name, board.name);
    println!("  → cd {}", name);
    println!("  → rvkit build");
}
