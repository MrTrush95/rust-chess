use crate::{
    chess_move::ChessMove, color::Color, error::Error, fen::Fen, move_generator::MoveGenerator,
    piece::Piece,
};

#[derive(Debug)]
pub struct Board {
    pub side_to_move: Color,
    pub squares: [Option<Piece>; 64],
}

impl Board {
    pub fn new(squares: [Option<Piece>; 64]) -> Board {
        Board {
            side_to_move: Color::White,
            squares,
        }
    }

    pub fn default() -> Board {
        Fen::create_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_owned())
    }

    pub fn try_make_move(&self, start: u8, target: u8) -> Result<(), Error> {
        // Get all the legal move possible in the current position
        let move_generator = MoveGenerator::new(&self);
        let legal_moves = move_generator.moves;

        println!("All Legal Moves: {:?}", legal_moves);

        let try_move = ChessMove::new(start, target);

        // 2. Check for the validity of the current move

        // 3. If it is valid, proceed forward and make the move

        // 4. Otherwise, return Error::InvalidMove
        Ok(())
    }
}
