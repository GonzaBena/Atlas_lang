use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Keyword {
    Let,
    Const,
    If,
    Else,
    While,
    For,
    Func,
    Return,
    Break,
    Continue,
    True,
    False,
    Null,
}

impl Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl Keyword {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "let" => Some(Keyword::Let),
            "const" => Some(Keyword::Const),
            "if" => Some(Keyword::If),
            "else" => Some(Keyword::Else),
            "while" => Some(Keyword::While),
            "for" => Some(Keyword::For),
            "func" => Some(Keyword::Func),
            "return" => Some(Keyword::Return),
            "break" => Some(Keyword::Break),
            "continue" => Some(Keyword::Continue),
            "true" => Some(Keyword::True),
            "false" => Some(Keyword::False),
            "null" => Some(Keyword::Null),
            _ => None,
        }
    }

    pub fn is_valid(s: &str) -> bool {
        Keyword::from_str(s).is_some()
    }

    pub fn to_str(&self) -> &str {
        match self {
            Keyword::Let => "let",
            Keyword::Const => "const",
            Keyword::If => "if",
            Keyword::Else => "else",
            Keyword::While => "while",
            Keyword::For => "for",
            Keyword::Func => "func",
            Keyword::Return => "return",
            Keyword::Break => "break",
            Keyword::Continue => "continue",
            Keyword::True => "true",
            Keyword::False => "false",
            Keyword::Null => "null",
        }
    }

    pub fn to_string(&self) -> String {
        self.to_str().to_string()
    }
}
