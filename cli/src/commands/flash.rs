use serde::Deserialize;
use std::fs;
use std::process::Command;

#[derive(Deserialize)]
struct Config {
    project: ProjectConfig,
}

#[derive(Deserialize)]
struct ProjectConfig {
    name: String,
    board: String,
}

pub fn run() {
    let toml_content = fs::read_to_string("rvkit.toml").unwrap_or_else(|_| {
        eprintln!("Error: rvkit.toml not found. Are you inside an rvkit project?");
        std::process::exit(1);
    });

    let config: Config = toml::from_str(&toml_content).unwrap_or_else(|_| {
        eprintln!("Error: rvkit.toml is invalid.");
        std::process::exit(1);
    });

    let board = crate::boards::get(&config.project.board).unwrap_or_else(|| {
        eprintln!("Board '{}' is not supported.", config.project.board);
        std::process::exit(1);
    });

    let binary = format!("zig-out/bin/{}", config.project.name);

    println!("Flashing '{}' onto board '{}'...", binary, board.name);

    let status = match board.flash_tool {
        "wlink" => Command::new("wlink").args(["flash", &binary]).status(),
        "esptool" => Command::new("esptool.py")
            .args(["write_flash", "0x0", &binary])
            .status(),
        _ => {
            eprintln!("Flash tool '{}' is not supported.", board.flash_tool);
            std::process::exit(1);
        }
    };

    match status {
        Ok(s) if s.success() => println!("✓ Flash succeeded!"),
        Ok(_) => {
            eprintln!("✗ Flash failed.");
            std::process::exit(1);
        }
        Err(_) => {
            eprintln!("Error: '{}' not found.", board.flash_tool);
            std::process::exit(1);
        }
    }
}
