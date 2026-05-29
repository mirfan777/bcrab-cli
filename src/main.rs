use bcrab_cli::{handle_create_new, ui};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "boil")]
#[command(about = "Rust CLI for publishing cargo projects", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new project from template
    Create {
        #[command(subcommand)]
        subcommand: CreateSubcommand,
    },
}

#[derive(Subcommand)]
enum CreateSubcommand {
    /// Create new app from template
    New {
        /// Name of the app to create
        name: String,
    },
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Create { subcommand } => match subcommand {
            CreateSubcommand::New { name } => handle_create_new(name),
        },
    };

    if let Err(e) = result {
        ui::print_error(&e.to_string());
        std::process::exit(1);
    }
}