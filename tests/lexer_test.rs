use atlas_lang::compiler::{
    elements::{keyword::Keyword, operator::Operator, token::Token},
    lexer::Lexer,
};

#[test]
fn parethesis_test() {
    let mut lex = Lexer::new(
        "var hola = 100

hola + (10+10)",
    );
    let tokens = lex.lex();

    assert_eq!(
        tokens,
        vec![
            Token::Keyword(Keyword::Var),
            Token::Identifier("hola".into()),
            Token::Operator(Operator::Assign),
            Token::Int32(100.into()),
            Token::NewLine,
            Token::NewLine,
            Token::Identifier("hola".into()),
            Token::Operator(Operator::Add),
            Token::StartParenthesis,
            Token::Int32(10.into()),
            Token::Operator(Operator::Add),
            Token::Int32(10.into()),
            Token::EndParenthesis,
            Token::EOF
        ]
    );
}

#[test]
fn brackets_test() {
    let mut lex = Lexer::new("[10+10]");
    let tokens = lex.lex();

    assert_eq!(
        tokens,
        vec![
            Token::StartBracket,
            Token::Int32(10.into()),
            Token::Operator(Operator::Add),
            Token::Int32(10.into()),
            Token::EndBracket,
            Token::EOF
        ]
    );
}
