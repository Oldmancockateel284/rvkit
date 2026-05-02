use serde::Deserialize;
use serialport::available_ports;
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
        eprintln!("Erreur : rvkit.toml introuvable. Es-tu dans un projet rvkit ?");
        std::process::exit(1);
    });

    let config: Config = toml::from_str(&toml_content).unwrap_or_else(|_| {
        eprintln!("Erreur : rvkit.toml invalide.");
        std::process::exit(1);
    });

    let board = crate::boards::get(&config.project.board).unwrap_or_else(|| {
        eprintln!("Board '{}' non supportée.", config.project.board);
        std::process::exit(1);
    });

    let binary = format!("zig-out/bin/{}", config.project.name);

    println!("Flash de '{}' sur la board '{}'...", binary, board.name);

    let status = match board.flash_tool {
        "wlink" => Command::new("wlink").args(["flash", &binary]).status(),
        "esptool" => Command::new("esptool.py")
            .args(["write_flash", "0x0", &binary])
            .status(),
        _ => {
            eprintln!("Outil de flash '{}' non supporté.", board.flash_tool);
            std::process::exit(1);
        }
    };

    match status {
        Ok(s) if s.success() => println!("✓ Flash réussi !"),
        Ok(_) => {
            eprintln!("✗ Flash échoué.");
            std::process::exit(1);
        }
        Err(_) => {
            eprintln!("Erreur : '{}' introuvable.", board.flash_tool);
            std::process::exit(1);
        }
    }
}
