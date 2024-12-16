use crate::compiler::error::parse_error::ParseError;

use super::{operator::Operator, token::Token};

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub struct Operation<'a> {
    pub(crate) operator: Operator,
    pub(crate) left: Box<Token<'a>>,
    pub(crate) right: Box<Token<'a>>,
}

// impl PartialEq for Operation<'_> {
//     fn eq(&self, other: &Self) -> bool {
//         self.operator == other.operator && self.left == other.left && self.right == other.right
//     }
// }

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
    }
}
