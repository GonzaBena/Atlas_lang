use thiserror::Error;

use crate::compiler::elements::token::Token;

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum ParseError {
    #[error("SyntaxError: {0}")]
    SyntaxError(String),

    #[error("UndefinedVariable: {0}")]
    UndefinedVariable(String),

    #[error("Unexpected token at position {position}: {token:?}")]
    UnexpectedToken { position: usize, token: Token },

    #[error("Unexpected token at the EOF")]
    UndefinedEOF,

    #[error("UndefinedVariable: {0}")]
    UndefinedFunction(String),

    #[error("UndefinedType: {0}")]
    UndefinedType(String),

    #[error("InvalidType: {0}")]
    InvalidType(String),

    #[error("TypeError: {0}")]
    TypeError(String),

    #[error("DefinedVariable: {0}")]
    DefinedVariable(String),

    #[error("DefinedFunction: {0}")]
    DefinedFunction(String),

    #[error("FunctionExecution: {0}")]
    FunctionExecution(String),

    #[error("InvalidTypeConvertion: {0}")]
    InvalidTypeConvertion(String),

    #[error("InvalidOperation: You can't '{operation}' a {type1} with a '{type2}'.")]
    InvalidOperation {
        operation: String,
        type1: String,
        type2: String,
    },
}

// impl fmt::Display for ParseError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Self::UndefinedEOF => write!(
//                 f,
//                 "The End of File couldn't be defined. \nThe file could be damaged or corrupted. "
//             ),
//             Self::SyntaxError(txt) => write!(f, "SyntaxError: {txt}"),
//             Self::InvalidType(txt) => {
//                 write!(f, "InvalidTypeError: The type '{txt}' doesn't exists.")
//             }
//             Self::DefinedVariable(txt) => write!(
//                 f,
//                 "DefinedVariableError: The variable '{txt}' already was defined."
//             ),

//             Self::DefinedFunction(txt) => write!(
//                 f,
//                 "DefinedFunctionError: The function '{txt}' already was defined."
//             ),
//             v => write!(f, "{v}"),
//         }
//     }
// }
