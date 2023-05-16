use crate::{
    chess_move::ChessMove, color::Color, error::Error, fen::Fen, move_generator::MoveGenerator,
    piece::Piece,
};

#[derive(Clone, Debug)]
pub struct Board {
    pub all_moves: Vec<ChessMove>,
    pub side_to_move: Color,
    pub squares: [Option<Piece>; 64],
}

impl Board {
    pub fn new(squares: [Option<Piece>; 64]) -> Board {
        Board {
            all_moves: vec![],
            side_to_move: Color::White,
            squares,
        }
    }

    pub fn default() -> Board {
        Fen::create_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_owned())
    }

    pub fn try_make_move(&mut self, user_move: ChessMove) -> Result<(), Error> {
        // Get all the legal move possible in the current position
        let legal_moves = MoveGenerator::new(&self).moves;

        // Check for the validity of the current move
        if !legal_moves.iter().any(|&m| m == user_move) {
            return Err(Error::InvalidMove);
        }

        self.make_move(user_move);

        Ok(())
    }

    fn make_move(&mut self, chess_move: ChessMove) {
        let start_square = chess_move.get_start_index() as usize;
        let target_square = chess_move.get_target_index() as usize;

        let piece = self.squares[start_square];

        self.squares[start_square] = None;
        self.squares[target_square] = Some(piece.unwrap());

        self.all_moves.push(chess_move);

        self.side_to_move = match self.side_to_move {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}
