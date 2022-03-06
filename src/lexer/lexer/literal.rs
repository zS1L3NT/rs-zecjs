use crate::{error::Error, lexer::token::Token, opsult::Opsult};

pub struct LiteralLexer {
    js: String,
}

impl LiteralLexer {
    fn new(js: String) -> Self {
        Self { js }
    }

    fn lex(&self) -> Opsult<Vec<Token>, Error> {
        let mut tokens = vec![];

        Opsult::Some(tokens)
    }
}
