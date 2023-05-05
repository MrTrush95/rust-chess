use crate::{board::Board, piece::Piece};

pub struct Fen;

impl Fen {
    pub fn create_board(fen: String) -> Board {
        let mut squares: [Option<Piece>; 64] = [None; 64];
        let mut file_index = 0;
        let mut rank_index = 0;

        for fen_value in fen.chars() {
            match fen_value {
                'a'..='z' | 'A'..='Z' => {
                    squares[file_index * 8 + rank_index] = Some(Piece::from_notation(fen_value));
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

        Board::new(squares)
    }
}
