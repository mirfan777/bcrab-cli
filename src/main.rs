use bcrab_cli::{handle_create_new, handle_make_resources, handle_make_dto, ui};
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
    /// Generate boilerplate code
    Make {
        #[command(subcommand)]
        subcommand: MakeSubcommand,
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

#[derive(Subcommand)]
enum MakeSubcommand {
    /// Generate a complete resource with DTO, Service, Repository, Entity, and Handlers
    Resources {
        /// Name of the resource (e.g., Admin, Product, Order)
        name: String,
    },
    /// Generate only DTO for a resource
    Dto {
        /// Name of the resource (e.g., Admin, Product, Order)
        name: String,
    },
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Create { subcommand } => match subcommand {
            CreateSubcommand::New { name } => handle_create_new(name),
        },
        Commands::Make { subcommand } => match subcommand {
            MakeSubcommand::Resources { name } => handle_make_resources(name),
            MakeSubcommand::Dto { name } => handle_make_dto(name),
        },
    };

    if let Err(e) = result {
        ui::print_error(&e.to_string());
        std::process::exit(1);
    }
}