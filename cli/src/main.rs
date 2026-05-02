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
    Monitor {
        #[arg(long, short, default_value = "/dev/ttyUSB0")]
        port: String,
        #[arg(long, short, default_value = "115200")]
        baud: u32,
    },
    /// Liste les boards supportées
    Boards,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::New { board, name } => commands::new::run(&board, &name),
        Commands::Boards => commands::boards::run(),
        Commands::Build => commands::build::run(),
        Commands::Flash => commands::flash::run(),
        Commands::Monitor { port, baud } => commands::monitor::run(&port, baud),
    }
}
