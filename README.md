# reflex
A simple `flex`-like lexing/tokenizing library written in Rust

Workflow:

- Define a `Token` enum with all possible tokens that implements `Clone`
- Create a `Ruleset` with `Ruleset::<Token>::new()`
- Add token rules with `add_rule()`, `add_simple()`, and `add_noop()`
- Call `lex()` with the string to be tokenized
- Use the resultant lazy iterator however you like

Example code for tokenizing a simple calculator language:

```rust
extern crate reflex;

use std::io;
use std::fmt;
use reflex::{Ruleset, lex};

#[derive(Clone)]
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

    let mut ruleset: Ruleset<Token> = Ruleset::new();
    ruleset.add_rule(r"-?[0-9]*\.?[0-9]+", |token| Token::Number(token.parse().unwrap_or(0.0)));
    ruleset.add_simple(r"\+", Token::OpAdd);
    ruleset.add_simple(r"-", Token::OpSub);
    ruleset.add_simple(r"\*", Token::OpMul);
    ruleset.add_simple(r"/", Token::OpDiv);
    ruleset.add_simple(r"\^", Token::OpPow);
    ruleset.add_noop(r"(?s)\s");

    for token in lex(&ruleset, program) {
        println!("{}", token.unwrap());
    }
}
```
