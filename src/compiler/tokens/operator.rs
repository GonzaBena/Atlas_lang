#[derive(Debug, PartialEq, Clone, PartialOrd)] 
#[allow(dead_code)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    DivInt,
    Mod,
    Pow,
    Asign,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
    Equal,
    NotEqual,
}

impl Operator {
    pub fn to_string(&self) -> String {
        match self {
            Operator::Add => "+".to_string(),
            Operator::Sub => "-".to_string(),
            Operator::Mul => "*".to_string(),
            Operator::Div => "/".to_string(),
            Operator::DivInt => "//".to_string(),
            Operator::Mod => "%".to_string(),
            Operator::Pow => "**".to_string(),
            Operator::Greater => ">".to_string(),
            Operator::Less => "<".to_string(),
            Operator::GreaterEqual => ">=".to_string(),
            Operator::LessEqual => "<=".to_string(),
            Operator::Equal => "==".to_string(),
            Operator::NotEqual => "!=".to_string(),
            Operator::Asign => "=".to_string(),
        }
    }
}
