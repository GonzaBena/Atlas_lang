use std::{error::Error, fmt};

#[derive(Debug)]
pub enum CLIError {
    InvalidPath,
    InvalidExtension,
}

impl fmt::Display for CLIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPath => write!(f, "The route doesn't exists or it's a directory."),
            Self::InvalidExtension => write!(f, "The extension have to be .atlas or .atl."),
            // v => write!(f, "CLIError: {v}"),
        }
    }
}

impl Error for CLIError {}
