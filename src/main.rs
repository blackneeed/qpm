use qpm::args::{Args, Commands, Parser};

fn main() {
    let cli = Args::parse();

    match &cli.command {
        Commands::Install { package } => {}
    }
}
