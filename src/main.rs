extern crate num;
#[macro_use]
extern crate num_derive;

use crate::board::Board;

pub mod board;
pub mod color;
pub mod error;
pub mod piece;

fn main() {
    let board = Board::new();

    println!("This is my board {:?}", board);
}
