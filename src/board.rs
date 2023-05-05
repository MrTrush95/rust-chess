use crate::{color::Color, error::Error, piece::Piece};

#[derive(Debug)]
pub struct Board {
    side_to_move: Color,
    pub squares: [Option<Piece>; 64],
}

impl Board {
    pub fn new() -> Board {
        Board {
            side_to_move: Color::White,
            squares: [None; 64],
        }
    }

    pub fn try_make_move() -> Result<(), Error> {
        // 1. Get all the legal move possible in the current position

        // 2. Check for the validity of the current move

        // 3. If it is valid, proceed forward and make the move

        // 4. Otherwise, return Error::InvalidMove
        Ok(())
    }
}
