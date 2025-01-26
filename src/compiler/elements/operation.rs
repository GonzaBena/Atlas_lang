use crate::compiler::error::parse_error::ParseError;

use super::{operator::Operator, token::Token};

#[derive(Debug, Clone, PartialEq)]
pub struct Operation {
    pub(crate) operator: Operator,
    pub(crate) left: Box<Token>,
    pub(crate) right: Box<Token>,
}

impl Operation {
    /// Create a new Operation
    ///
    /// # Example
    /// ```ignore
    /// let operator  = Operator::Assign;
    /// let left      = Token::Void;
    /// let right     = Token::Void;
    ///
    /// let operation = Operation::new(operator, left, right);
    /// ```
    pub fn new(operator: Operator, left: Token, right: Token) -> Operation {
        Operation {
            operator,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    /// Resolve the operation by evaluating the left and right tokens and applying the operator.
    ///
    /// # Errors
    ///
    /// Returns a `ParseError` if resolving the left or right tokens fails, or if the operator execution fails.
    ///
    /// # Example
    /// ```ignore
    /// let mut operation = Operation::new(Operator::Add, Token::Number(1), Token::Number(2));
    /// let result = operation.resolve();
    /// assert_eq!(result, Ok(Token::Number(3)));
    /// ```
    pub fn resolve(&mut self) -> Result<Token, ParseError> {
        let left = self.left.clone().resolve()?;
        let right = self.right.clone().resolve()?;
        let result = self.operator.execute(left, right)?;
        Ok(result)
    }
}
