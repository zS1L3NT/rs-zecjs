use std::fmt::Display;

use crate::lexer::token::Position;

#[derive(Debug, Clone)]
pub struct Error {
    pub msg: String,
    pub pos: Position,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} (line {} column {})",
            self.msg, self.pos.line, self.pos.column
        )
    }
}
