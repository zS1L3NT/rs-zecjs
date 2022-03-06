use crate::{lexer::token::Token, opsult::Opsult, Error};

pub struct KeywordLexer {
    js: String,
}

impl KeywordLexer {
    fn new(js: String) -> Self {
        Self { js }
    }

    fn lex(&self) -> Opsult<Vec<Token>, Error> {
        let mut tokens = vec![];

        Opsult::Some(tokens)
    }
}
