mod boards;
mod commands;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rvkit")]
#[command(version)]
#[command(about = "Bare metal Zig, without the bare metal pain.")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a new project for a target board
    New {
        #[arg(long, short)]
        board: String,
        name: String,
    },
    /// Build the project
    Build,
    /// Flash the firmware onto the board
    Flash,
    /// Serial monitor
    Monitor {
        #[arg(long, short, default_value = "/dev/ttyUSB0")]
        port: String,
        #[arg(long, short, default_value = "115200")]
        baud: u32,
    },
    /// List supported boards
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
