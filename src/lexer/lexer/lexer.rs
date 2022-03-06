use crate::{error::Error, lexer::token::Token, opsult::Opsult};

pub struct Lexer {
    js: String,
}

impl Lexer {
    fn new(js: String) -> Self {
        Self { js }
    }

    fn lex(&self) -> Opsult<Vec<Token>, Error> {
        let mut tokens = vec![];

        Opsult::Some(tokens)
    }
}
