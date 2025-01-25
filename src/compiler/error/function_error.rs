use thiserror::Error;

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum FunctionError {
    /// Indicate that the file doesn't correctly finish
    #[error("The End of File couldn't be defined. \nThe file could be damaged or corrupted.")]
    UndefinedEOF,

    #[error("InvalidNumberOfArgs: {0}")]
    InvalidNumberOfArgs(String),

    #[error("ExecutionError: {0}")]
    ExecutionError(String),

    #[error("DifferentReturnType: {0}")]
    DifferentReturnType(String),
}

// impl fmt::Display for FunctionError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Self::UndefinedEOF => write!(
//                 f,
//                 "The End of File couldn't be defined. \nThe file could be damaged or corrupted. "
//             ),
//             v => write!(f, "{v}"),
//         }
//     }
// }

// impl Error for FunctionError {}
