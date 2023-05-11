use crate::{
    board::Board,
    chess_move::ChessMove,
    color::Color,
    piece::{Piece, PieceType},
    square::Square,
};

pub struct MoveGenerator<'a> {
    board: &'a Board,
    pub moves: Vec<ChessMove>,
}

pub const DIAGONAL_OFFSETS: [i8; 4] = [-7, 7, -9, 9];
pub const ORTHOGONAL_OFFSETS: [i8; 4] = [-1, 1, -8, 8];

impl MoveGenerator<'_> {
    pub fn new(board: &Board) -> MoveGenerator {
        let mut move_generator = MoveGenerator {
            board,
            moves: vec![],
        };

        move_generator.generate_legal_moves();

        move_generator
    }

    fn generate_legal_moves(&mut self) {
        for (square_index, square) in self.board.squares.iter().enumerate() {
            match square {
                Some(piece) => self.generate_pieces_moves(piece, square_index as u8),
                None => (),
            };
        }
    }

    fn generate_pieces_moves(&mut self, piece: &Piece, square_index: u8) {
        if piece.get_color() != self.board.side_to_move {
            return;
        }

        match piece.get_type() {
            PieceType::Pawn => self.generate_pawn_moves(piece, square_index),
            PieceType::Knight => self.generate_knight_moves(piece, square_index),
            PieceType::Bishop => self.generate_bishop_moves(piece, square_index),
            PieceType::Rook => self.generate_rook_moves(piece, square_index),
            PieceType::Queen => self.generate_queen_moves(piece, square_index),
            PieceType::King => self.generate_king_moves(piece, square_index),
        };
    }

    fn generate_pawn_moves(&mut self, piece: &Piece, square_index: u8) {
        let forward_move: i32 = 8;
        let square = Square::new(square_index);
        let offset: i32 = if piece.is_color(Color::White) { -1 } else { 1 };

        let mut quiet_moves: Vec<i32> = vec![forward_move * offset];
        let mut attack_moves: Vec<i32> = vec![];

        let space_ahead_occupied =
            self.board.squares[(square_index as i32 + (forward_move * offset)) as usize].is_some();

        if square.get_rank().is_pawn_rank(piece.get_color()) && !space_ahead_occupied {
            quiet_moves.push(forward_move * 2 * offset);
        }

        if !square.get_file().is_first_file() {
            attack_moves.push(7 * offset);
        }

        if !square.get_file().is_last_file() {
            attack_moves.push(9 * offset);
        }

        for quiet_move in quiet_moves.iter() {
            let space_occupied =
                self.board.squares[(square_index as i32 + *quiet_move) as usize].is_some();

            if !space_occupied {
                self.moves.push(ChessMove::new(
                    square_index,
                    (square_index as i32 + *quiet_move) as u8,
                ));
            }
        }

        for attack_move in attack_moves.iter() {
            let target_square = self.board.squares[(square_index as i32 + *attack_move) as usize];

            match target_square {
                Some(target_piece) => {
                    if !target_piece.is_color(piece.get_color()) {
                        self.moves.push(ChessMove::new(
                            square_index,
                            (square_index as i32 + *attack_move) as u8,
                        ));
                    }
                }
                None => (),
            }
        }

        // TODO: Handle en-passant
        // TODO: Handle promotion
        // TODO: Handle King Safety
    }

    fn generate_knight_moves(&self, piece: &Piece, square_index: u8) {}

    fn generate_bishop_moves(&mut self, piece: &Piece, square_index: u8) {
        self.generate_sliding_moves(piece, square_index, DIAGONAL_OFFSETS);
    }

    fn generate_rook_moves(&mut self, piece: &Piece, square_index: u8) {
        self.generate_sliding_moves(piece, square_index, ORTHOGONAL_OFFSETS);
    }

    fn generate_queen_moves(&mut self, piece: &Piece, square_index: u8) {
        self.generate_sliding_moves(piece, square_index, ORTHOGONAL_OFFSETS);
        self.generate_sliding_moves(piece, square_index, DIAGONAL_OFFSETS);
    }

    fn generate_king_moves(&self, piece: &Piece, square_index: u8) {}

    fn generate_sliding_moves(&mut self, piece: &Piece, square_index: u8, offsets: [i8; 4]) {
        for offset in offsets.iter() {
            for next in 0..8 {
                let target_square_index = (square_index as i8) + offset * (next + 1);

                if !Square::is_valid(target_square_index) {
                    break;
                }

                match self.board.squares[target_square_index as usize] {
                    Some(target_piece) => {
                        if !target_piece.is_color(piece.get_color()) {
                            self.add_move(square_index, target_square_index as u8);
                        }
                        break;
                    }
                    None => {
                        self.add_move(square_index, target_square_index as u8);
                    }
                }
            }
        }
    }

    fn add_move(&mut self, start: u8, target: u8) {
        self.moves.push(ChessMove::new(start, target));
    }
}
