mod lexer;

fn main() {
    let x = lexer::token::Keyword::Boolean(lexer::token::Position { line: 1, column: 1 });
}
