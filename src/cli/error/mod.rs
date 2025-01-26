use thiserror::Error;

/// This error covers all about CLI
#[derive(Debug, Error)]
pub enum CLIError {
    #[error("The route doesn't exists or it's a directory.")]
    InvalidPath,

    #[error("The extension have to be .atlas or .atl.")]
    InvalidExtension,
}
