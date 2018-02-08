# reflex
A simple `flex`-like lexing/tokenizing library written in Rust

Current workflow:

- Define a `Token` enum with all possible tokens that implements `Copy` and `Clone`
- Initialize a `Lexer` with `reflex::Lexer::<Token>::new()`
- Add token rules with `add_rule()`, `add_simple()`, and `add_noop()`
- Call `Lexer::lex()` with the string to be tokenized
- Use the resultant vector of `Token`s however you like

Example code for parsing a simple calculator language:

```rust
extern crate reflex;

use std::io;
use std::fmt;

#[derive(Copy, Clone)]
enum Token {
    Number(f64),
    OpAdd,
    OpSub,
    OpMul,
    OpDiv,
    OpPow,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Token::Number(n) => n.to_string(),
            Token::OpAdd     => "+".to_string(),
            Token::OpSub     => "-".to_string(),
            Token::OpMul     => "*".to_string(),
            Token::OpDiv     => "/".to_string(),
            Token::OpPow     => "^".to_string(),
        }.to_string())
    }
}

fn main() {
    let mut program = String::new();
    io::stdin().read_line(&mut program).unwrap();
    
    let mut lexer = reflex::Lexer::<Token>::new();
    lexer.add_rule(r"-?[0-9]*\.?[0-9]+", Box::new(|tok: &str| Token::Number(tok.parse().unwrap())));
    lexer.add_simple(r"\+", Token::OpAdd);
    lexer.add_simple(r"-", Token::OpSub);
    lexer.add_simple(r"\*", Token::OpMul);
    lexer.add_simple(r"/", Token::OpDiv);
    lexer.add_simple(r"\^", Token::OpPow);
    lexer.add_noop(r"(?s).");
    let tokens = lexer.lex(program.as_str());

    for token in tokens.iter() {
        println!("{}", token);
    }
}
```

Planned workflow:

- Define your `Token` enum as above
- Create as many `Ruleset`s as you like using `add_rule()`, `add_simple()`, and `add_noop()`
- Call `reflex::lex()` with a ruleset and a string slice
- Use the resulting lazy iterator however you like
