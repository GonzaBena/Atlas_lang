use atlas_lang::compiler::{elements::token::Token, lexer::Lexer, parser::Parser, Variable};

#[test]
fn assignation_test() {
    let mut lex = Lexer::new(
        "
    var hola = 10
    ",
    );
    let tokens = lex.lex();
    let mut parser = Parser::new(tokens, None);
    let _ = parser.parse().unwrap();

    let result = vec![(
        "hola",
        Variable::new(
            "hola".to_string(),
            "Int32".to_string(),
            Token::Int32(10.into()),
            0,
        ),
    )];
    let tuple = parser.get_variables();

    for (key, val) in tuple {
        assert_eq!(key, result[0].0);
        assert_eq!(val, result[0].1);
    }
}

#[test]
fn add_test() {
    let mut lex = Lexer::new(
        "
    var hola = 10
    hola + 10
    ",
    );
    let tokens = lex.lex();
    let mut parser = Parser::new(tokens, None);
    let parse = parser.parse().unwrap();

    assert_eq!(parse, vec![Token::Int32(20.into())])
}

#[test]
fn add_assign_test() {
    let mut lex = Lexer::new(
        "
    var hola = 10
    hola += 10
    ",
    );
    let tokens = lex.lex();
    let mut parser = Parser::new(tokens, None);
    let parse = parser.parse().unwrap();

    assert_eq!(parse, vec![]);
    let result = vec![(
        "hola",
        Variable::new(
            "hola".to_string(),
            "Int32".to_string(),
            Token::Int32(20.into()),
            0,
        ),
    )];
    let tuple = parser.get_variables();

    for (key, val) in tuple {
        assert_eq!(key, result[0].0);
        assert_eq!(val, result[0].1);
    }
}

#[test]
fn sub_test() {
    let mut lex = Lexer::new(
        "
    var hola = 10
    hola - 10
    ",
    );
    let tokens = lex.lex();
    let mut parser = Parser::new(tokens, None);
    let parse = parser.parse().unwrap();

    assert_eq!(parse, vec![Token::Int32(0.into())])
}

#[test]
fn sub_assign_test() {
    let mut lex = Lexer::new(
        "
    var hola = 10
    hola -= 10
    ",
    );
    let tokens = lex.lex();
    let mut parser = Parser::new(tokens, None);
    let parse = parser.parse().unwrap();

    assert_eq!(parse, vec![]);
    let result = vec![(
        "hola",
        Variable::new(
            "hola".to_string(),
            "Int32".to_string(),
            Token::Int32(0.into()),
            0,
        ),
    )];
    let tuple = parser.get_variables();

    for (key, val) in tuple {
        assert_eq!(key, result[0].0);
        assert_eq!(val, result[0].1);
    }
}

#[test]
fn mul_test() {
    let mut lex = Lexer::new(
        "
    var hola = 10
    hola * 5
    ",
    );
    let tokens = lex.lex();
    let mut parser = Parser::new(tokens, None);
    let parse = parser.parse().unwrap();

    assert_eq!(parse, vec![Token::Int32(50.into())])
}

#[test]
fn mul_assign_test() {
    let mut lex = Lexer::new(
        "
    var hola = 10
    hola *= 5
    ",
    );
    let tokens = lex.lex();
    let mut parser = Parser::new(tokens, None);
    let parse = parser.parse().unwrap();

    assert_eq!(parse, vec![]);
    let result = vec![(
        "hola",
        Variable::new(
            "hola".to_string(),
            "Int32".to_string(),
            Token::Int32(50.into()),
            0,
        ),
    )];
    let tuple = parser.get_variables();

    for (key, val) in tuple {
        assert_eq!(key, result[0].0);
        assert_eq!(val, result[0].1);
    }
}

#[test]
fn div_test() {
    let mut lex = Lexer::new(
        "
    var hola = 10
    hola / 2
    ",
    );
    let tokens = lex.lex();
    let mut parser = Parser::new(tokens, None);
    let parse = parser.parse().unwrap();

    assert_eq!(parse, vec![Token::Int32(5.into())])
}

#[test]
fn div_assign_test() {
    let mut lex = Lexer::new(
        "
    var hola = 10
    hola /= 2
    ",
    );
    let tokens = lex.lex();
    let mut parser = Parser::new(tokens, None);
    let parse = parser.parse().unwrap();

    assert_eq!(parse, vec![]);
    let result = vec![(
        "hola",
        Variable::new(
            "hola".to_string(),
            "Int32".to_string(),
            Token::Int32(5.into()),
            0,
        ),
    )];
    let tuple = parser.get_variables();

    for (key, val) in tuple {
        assert_eq!(key, result[0].0);
        assert_eq!(val, result[0].1);
    }
}

#[test]
fn int_div_test() {
    let mut lex = Lexer::new(
        "
    var hola = 10
    hola // 3
    ",
    );
    let tokens = lex.lex();
    let mut parser = Parser::new(tokens, None);
    let parse = parser.parse().unwrap();

    assert_eq!(parse, vec![Token::Int32((3).into())])
}

#[test]
fn int_div_assign_test() {
    let mut lex = Lexer::new(
        "
    var hola = 10
    hola //= 3
    ",
    );
    let tokens = lex.lex();
    let mut parser = Parser::new(tokens, None);
    let parse = parser.parse().unwrap();

    assert_eq!(parse, vec![]);
    let result = vec![(
        "hola",
        Variable::new(
            "hola".to_string(),
            "Int32".to_string(),
            Token::Int32(3.into()),
            0,
        ),
    )];
    let tuple = parser.get_variables();

    for (key, val) in tuple {
        assert_eq!(key, result[0].0);
        assert_eq!(val, result[0].1);
    }
}

#[test]
fn modulo_test() {
    let mut lex = Lexer::new(
        "
    var hola = 10
    hola % 3
    ",
    );
    let tokens = lex.lex();
    let mut parser = Parser::new(tokens, None);
    let parse = parser.parse().unwrap();

    assert_eq!(parse, vec![Token::Int32(1.into())])
}

#[test]
fn modulo_assign_test() {
    let mut lex = Lexer::new(
        "
    var hola = 10
    hola %= 3
    ",
    );
    let tokens = lex.lex();
    let mut parser = Parser::new(tokens, None);
    let parse = parser.parse().unwrap();

    assert_eq!(parse, vec![]);
    let result = vec![(
        "hola",
        Variable::new(
            "hola".to_string(),
            "Int32".to_string(),
            Token::Int32(1.into()),
            0,
        ),
    )];
    let tuple = parser.get_variables();

    for (key, val) in tuple {
        assert_eq!(key, result[0].0);
        assert_eq!(val, result[0].1);
    }
}

#[test]
fn power_test() {
    let mut lex = Lexer::new(
        "
    var hola = 2
    hola ** 3
    ",
    );
    let tokens = lex.lex();
    for token in tokens
        .split(|x| *x == Token::NewLine)
        .filter(|x| !x.is_empty() && *x != [Token::EOF])
    {
        println!("tokens: {token:?}",);
    }
    let mut parser = Parser::new(tokens, None);
    let parse = parser.parse().unwrap();

    assert_eq!(parse, vec![Token::Int32(8.into())])
}

#[test]
fn power_assign_test() {
    let mut lex = Lexer::new(
        "
    var hola = 2
    hola **= 3
    ",
    );
    let tokens = lex.lex();
    for token in tokens
        .split(|x| *x == Token::NewLine)
        .filter(|x| !x.is_empty() && *x != [Token::EOF])
    {
        println!("tokens: {token:?}",);
    }
    let mut parser = Parser::new(tokens, None);
    let parse = parser.parse().unwrap();

    assert_eq!(parse, vec![]);
    let result = vec![(
        "hola",
        Variable::new(
            "hola".to_string(),
            "Int32".to_string(),
            Token::Int32(8.into()),
            0,
        ),
    )];
    let tuple = parser.get_variables();

    for (key, val) in tuple {
        assert_eq!(key, result[0].0);
        assert_eq!(val, result[0].1);
    }
}
