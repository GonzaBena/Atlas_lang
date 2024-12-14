use crate::compiler::error::parse_error::ParseError;

use super::{operator::Operator, token::Token};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Operation<'a> {
    operator: Operator,
    left: Box<Token<'a>>,
    right: Box<Token<'a>>,
}

#[allow(dead_code)]
impl<'a> Operation<'a> {
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
    pub fn new(operator: Operator, left: Token<'a>, right: Token<'a>) -> Operation<'a> {
        Operation {
            operator,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub fn resolve(&mut self) -> Result<Token<'a>, ParseError<'a>> {
        let left = self.left.clone().resolve()?;
        let right = self.right.clone().resolve()?;
        Ok(self.operator.execute(left, right))
        // Ok(Token::EOF)
    }
}
