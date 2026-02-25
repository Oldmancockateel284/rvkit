mod boards;
mod commands;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rvkit")]
#[command(about = "Bare metal Zig, without the bare metal pain.")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Génère un nouveau projet pour une board cible
    New {
        #[arg(long, short)]
        board: String,
        name: String,
    },
    /// Compile le projet
    Build,
    /// Flashe le firmware sur la board
    Flash,
    /// Moniteur série
    Monitor,
    /// Liste les boards supportées
    Boards,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::New { board, name } => commands::new::run(&board, &name),
        // Commands::Build => println!("Build..."),
        Commands::Flash => println!("Flash..."),
        Commands::Monitor => println!("Monitor..."),
        Commands::Boards => commands::boards::run(),
        Commands::Build => commands::build::run(),
    }
}
