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
    #[command(about = "Manage repos")]
    Repo {
        #[command(subcommand)]
        command: RepoCommands,
    },
}

#[derive(Subcommand)]
pub enum RepoCommands {
    #[command(about = "Add a new repo")]
    Add {
        #[arg(help = "Name of the repo")]
        name: String,
        #[arg(help = "URL of the repo")]
        url: String,
    },
    #[command(about = "List all repos")]
    List,
    #[command(about = "Remove a repo")]
    Remove {
        #[arg(help = "Name of the repo to remove")]
        name: String,
    },
    #[command(about = "Update a repos")]
    Update,
}
