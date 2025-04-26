use qpm::args::{Args, Commands, Parser, RepoCommands};

fn main() {
    let cli = Args::parse();

    match &cli.command {
        Commands::Install { package } => {}
        Commands::Repo { command } => match command {
            RepoCommands::Add { name, url } => {}
            RepoCommands::Remove { name } => {}
            RepoCommands::List => {}
            RepoCommands::Update => {}
        },
    }
}
