use crate::{lexer::token::Token, opsult::Opsult, Error};

pub struct SymbolLexer {
    js: String,
}

impl SymbolLexer {
    fn new(js: String) -> Self {
        Self { js }
    }

    fn lex(&self) -> Opsult<Vec<Token>, Error> {
        let mut tokens = vec![];

        Opsult::Some(tokens)
    }
}
