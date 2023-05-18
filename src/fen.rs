use std::collections::HashMap;

use crate::{
    board::Board,
    color::Color,
    piece::{Piece, PieceType},
};

pub struct Fen;

impl Fen {
    pub fn create_board(fen: String) -> Board {
        let mut squares: [Option<Piece>; 64] = [None; 64];
        let mut kings_pos: HashMap<Color, u8> = HashMap::new();
        let mut file_index = 0;
        let mut rank_index = 0;

        for fen_value in fen.chars() {
            match fen_value {
                'a'..='z' | 'A'..='Z' => {
                    let piece = Piece::from_notation(fen_value);
                    let square_index = file_index * 8 + rank_index;
                    if piece.is_type(PieceType::King) {
                        kings_pos.insert(piece.get_color(), 64 - square_index as u8);
                    }
                    squares[square_index] = Some(piece);
                    rank_index += 1;
                }
                '0'..='8' => {
                    let fen_value = fen_value as usize - 0x30;
                    rank_index += fen_value;
                }
                '/' => {
                    file_index += 1;
                    rank_index = 0;
                }
                _ => unreachable!(),
            }
        }

        Board::new(squares, kings_pos)
    }
}
