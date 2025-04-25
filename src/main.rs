use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "qpm")]
#[command(about = "qpm - A simple package manager for Linux (and Windows)")]
#[command(version = "0.1.0")]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Install a package")]
    Install {
        #[arg(help = "Name of the package to install")]
        package: String,
    },

    #[command(about = "Show qpm information")]
    Info { package: String },
}

fn main() {
    let cli = Args::parse();

    match &cli.command {
        Commands::Install { package } => {}
        Commands::Info { package } => {}
    }
}
