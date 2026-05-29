use std::fmt;
use std::io;

#[derive(Debug)]
pub enum CliError {
    IoError(io::Error),
    GitError(String),
    InvalidSelection,
    CloneError(String),
    CleanupError(String),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CliError::IoError(e) => write!(f, "IO Error: {}", e),
            CliError::GitError(msg) => write!(f, "Git Error: {}", msg),
            CliError::InvalidSelection => write!(f, "Invalid selection made"),
            CliError::CloneError(msg) => write!(f, "Clone Error: {}", msg),
            CliError::CleanupError(msg) => write!(f, "Cleanup Error: {}", msg),
        }
    }
}

impl From<io::Error> for CliError {
    fn from(err: io::Error) -> Self {
        CliError::IoError(err)
    }
}

pub type Result<T> = std::result::Result<T, CliError>;
