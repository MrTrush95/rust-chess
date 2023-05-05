extern crate num;
#[macro_use]
extern crate num_derive;

use piece::Piece;
use ui::draw_board;

use crate::board::Board;

pub mod board;
pub mod color;
pub mod error;
pub mod piece;
pub mod ui;

fn main() {
    let mut board = Board::new();

    board.squares[0] = Some(Piece::new(piece::PieceType::Pawn, color::Color::White));

    draw_board(board);
}
