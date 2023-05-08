extern crate num;
#[macro_use]
extern crate num_derive;

use ui::UI;

use crate::board::Board;

pub mod board;
pub mod chess_move;
pub mod color;
pub mod error;
pub mod fen;
pub mod file;
pub mod move_generator;
pub mod piece;
pub mod rank;
pub mod square;
pub mod string;
pub mod ui;

fn main() {
    let mut board = Board::default();

    let mut ui = UI::new(&mut board, 40, 20);

    ui.start_game();
}
