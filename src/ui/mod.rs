pub mod button;

pub const SQUARE_SIZE: u16 = 50; // button.rs

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Square(chess::Square),
}

#[derive(Debug, Clone, Copy)]
pub enum SquareType {
    Piece,
    Moveable,
    None,
}
