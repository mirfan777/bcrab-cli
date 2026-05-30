pub mod cli;
pub mod error;
pub mod models;
pub mod repository;
pub mod templates;
pub mod ui;
pub mod generator;

pub use cli::handle_create_new;
pub use generator::{handle_make_resources, handle_make_dto};
pub use error::{CliError, Result};
