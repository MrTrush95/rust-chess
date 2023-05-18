use std::collections::HashMap;

use crate::{
    chess_move::ChessMove,
    color::Color,
    error::Error,
    fen::Fen,
    move_generator::MoveGenerator,
    piece::{Piece, PieceType},
};

#[derive(Clone, Debug)]
pub struct Board {
    pub all_moves: Vec<ChessMove>,
    pub side_to_move: Color,
    pub squares: [Option<Piece>; 64],
    pub kings_pos: HashMap<Color, u8>,
}

impl Board {
    pub fn new(squares: [Option<Piece>; 64], kings_pos: HashMap<Color, u8>) -> Board {
        Board {
            all_moves: vec![],
            side_to_move: Color::White,
            squares,
            kings_pos,
        }
    }

    pub fn default() -> Board {
        Fen::create_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_owned())
    }

    pub fn try_make_move(&mut self, user_move: ChessMove) -> Result<(), Error> {
        // Get all the legal move possible in the current position
        let legal_moves = MoveGenerator::new(&self, false).moves;

        // Check for the validity of the current move
        if !legal_moves.iter().any(|&m| m == user_move) {
            return Err(Error::InvalidMove);
        }

        self.make_move(user_move);

        Ok(())
    }

    pub fn make_move(&mut self, chess_move: ChessMove) {
        let start_square = chess_move.get_start_index() as usize;
        let target_square = chess_move.get_target_index() as usize;

        let piece = self.squares[start_square].unwrap();

        if piece.is_type(PieceType::King) {
            self.kings_pos
                .insert(self.side_to_move, target_square as u8);
        }

        self.squares[start_square] = None;
        self.squares[target_square] = Some(piece);

        self.all_moves.push(chess_move);

        self.side_to_move = match self.side_to_move {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    pub fn get_king_position(&self) -> u8 {
        let value = self
            .kings_pos
            .get(&self.side_to_move)
            .expect("King does not exist");

        *value
    }
}
