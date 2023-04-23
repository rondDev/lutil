use clap::{Parser, Args, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    ExtendedRegexp {
        pattern: String,
    },
    FixedStrings {
        string: String,
    },
    BasicRegexp {
        pattern: String,
    },
    PerlRegexp {
        pattern: String,
    },
    RegExp {
        pattern: String,
    },
    File {
        path: String,
    },
    IgnoreCase {
        boolean: bool,
    },
}

fn main() {
    let args = Cli::parse();
    
    match args.command {
        Commands::ExtendedRegexp { pattern } => todo!(),
        Commands::FixedStrings { string } => todo!(),
        Commands::BasicRegexp { pattern } => todo!(),
        Commands::PerlRegexp { pattern } => todo!(),
        Commands::RegExp { pattern } => todo!(),
        Commands::File { path } => todo!(),
        Commands::IgnoreCase { boolean } => todo!(),

    }
}
