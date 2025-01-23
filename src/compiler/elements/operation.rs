use crate::compiler::error::parse_error::ParseError;

use super::{operator::Operator, token::Token};

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub struct Operation {
    pub(crate) operator: Operator,
    pub(crate) left: Box<Token>,
    pub(crate) right: Box<Token>,
}

// impl PartialEq for Operation<'_> {
//     fn eq(&self, other: &Self) -> bool {
//         self.operator == other.operator && self.left == other.left && self.right == other.right
//     }
// }

#[allow(dead_code)]
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

    pub fn resolve(&mut self) -> Result<Token, ParseError> {
        let left = self.left.clone().resolve()?;
        let right = self.right.clone().resolve()?;
        let result = self.operator.execute(left, right);
        Ok(result)
    }
}
