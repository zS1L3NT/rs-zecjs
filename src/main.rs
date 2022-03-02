mod error;
mod lexer;
mod opsult;

pub use error::Error;

fn main() {
    let x = lexer::token::Keyword::Boolean(lexer::token::Position { line: 1, column: 1 });
}
