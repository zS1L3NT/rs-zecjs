use super::Position;

pub enum Literal {
    String(String, Position),
    Number(f64, Position),
    Boolean(bool, Position),
}
