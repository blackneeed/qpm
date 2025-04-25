pub use clap::Parser;
use clap::Subcommand;

#[derive(Parser)]
#[command(name = "qpm")]
#[command(about = "qpm - A simple package manager for Linux (and Windows)")]
#[command(version = "0.1.0")]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Install a package")]
    Install {
        #[arg(help = "Name of the package to install")]
        package: String,
    },
}
