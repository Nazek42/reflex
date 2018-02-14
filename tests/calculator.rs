extern crate reflex;

use reflex::{Ruleset, lex};

#[derive(Copy, Clone, PartialEq, Debug)]
enum Token {
    Number(f64),
    OpAdd,
    OpSub,
    OpMul,
    OpDiv,
    OpPow,
    ParenL,
    ParenR,
}

#[test]
fn calculator() {
    let program = "(3.45 + 3) * (-7 - .8) ^ (1.3 / -.5)".to_string();
    let expected = vec![
        Token::ParenL, Token::Number(3.45), Token::OpAdd, Token::Number(3.0), Token::ParenR,
        Token::OpMul, Token::ParenL, Token::Number(-7.0), Token::OpSub, Token::Number(0.8), Token::ParenR,
        Token::OpPow, Token::ParenL, Token::Number(1.3), Token::OpDiv, Token::Number(-0.5), Token::ParenR
    ];

    let mut ruleset: Ruleset<Token> = Ruleset::new();
    ruleset.add_rule(r"-?[0-9]*\.?[0-9]+", |token| Token::Number(token.parse().unwrap_or(0.0)));
    ruleset.add_simple(r"\+", Token::OpAdd);
    ruleset.add_simple(r"-", Token::OpSub);
    ruleset.add_simple(r"\*", Token::OpMul);
    ruleset.add_simple(r"/", Token::OpDiv);
    ruleset.add_simple(r"\^", Token::OpPow);
    ruleset.add_simple(r"\(", Token::ParenL);
    ruleset.add_simple(r"\)", Token::ParenR);
    ruleset.add_noop(r"(?s)\s");
    let mut i = 0;
    for token in lex(&ruleset, program) {
        assert_eq!(token.unwrap(), expected[i]);
        i += 1;
    }
}
