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
            let chess_moves = match square {
                Some(piece) => self.generate_pieces_moves(piece, square_index as u8),
                None => None,
            };

            if chess_moves.is_some() {
                self.moves.append(&mut chess_moves.unwrap());
            }
        }
    }

    fn generate_pieces_moves(&self, piece: &Piece, square_index: u8) -> Option<Vec<ChessMove>> {
        if piece.get_color() != self.board.side_to_move {
            return None;
        }

        let moves = match piece.get_type() {
            PieceType::Pawn => self.generate_pawn_moves(piece, square_index),
            PieceType::Knight => self.generate_knight_moves(piece, square_index),
            PieceType::Bishop => self.generate_bishop_moves(piece, square_index),
            PieceType::Rook => self.generate_rook_moves(piece, square_index),
            PieceType::Queen => self.generate_queen_moves(piece, square_index),
            PieceType::King => self.generate_king_moves(piece, square_index),
        };

        Some(moves)
    }

    fn generate_pawn_moves(&self, piece: &Piece, square_index: u8) -> Vec<ChessMove> {
        let mut moves: Vec<ChessMove> = vec![];

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
                moves.push(ChessMove::new(
                    square_index,
                    (square_index as i32 + *quiet_move) as u8,
                ));
            }
        }

        for attack_move in attack_moves.iter() {
            let space_occupied =
                self.board.squares[(square_index as i32 + *attack_move) as usize].is_some();

            if space_occupied {
                moves.push(ChessMove::new(
                    square_index,
                    (square_index as i32 + *attack_move) as u8,
                ));
            }
        }

        // TODO: Handle en-passant
        // TODO: Handle promotion
        // TODO: Handle King Safety

        moves
    }

    fn generate_knight_moves(&self, piece: &Piece, square_index: u8) -> Vec<ChessMove> {
        let moves: Vec<ChessMove> = vec![];

        moves
    }

    fn generate_bishop_moves(&self, piece: &Piece, square_index: u8) -> Vec<ChessMove> {
        let moves: Vec<ChessMove> = vec![];

        moves
    }

    fn generate_rook_moves(&self, piece: &Piece, square_index: u8) -> Vec<ChessMove> {
        let moves: Vec<ChessMove> = vec![];

        moves
    }

    fn generate_queen_moves(&self, piece: &Piece, square_index: u8) -> Vec<ChessMove> {
        let moves: Vec<ChessMove> = vec![];

        moves
    }

    fn generate_king_moves(&self, piece: &Piece, square_index: u8) -> Vec<ChessMove> {
        let moves: Vec<ChessMove> = vec![];

        moves
    }
}
