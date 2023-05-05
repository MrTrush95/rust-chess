use crate::{
    board::Board,
    chess_move::ChessMove,
    piece::{Piece, PieceType},
};

pub struct MoveGenerator {
    pub moves: Vec<ChessMove>,
}

impl MoveGenerator {
    pub fn new(board: &Board) -> MoveGenerator {
        let mut move_generator = MoveGenerator { moves: vec![] };

        move_generator.generate_legal_moves(board);

        move_generator
    }

    fn generate_legal_moves(&mut self, board: &Board) {
        for (square_index, square) in board.squares.iter().enumerate() {
            let chess_moves = match square {
                Some(piece) => self.generate_pieces_moves(board, piece, square_index),
                None => None,
            };

            if chess_moves.is_some() {
                self.moves.append(&mut chess_moves.unwrap());
            }
        }
    }

    fn generate_pieces_moves(
        &self,
        board: &Board,
        piece: &Piece,
        square_index: usize,
    ) -> Option<Vec<ChessMove>> {
        if piece.get_color() != board.side_to_move {
            return None;
        }

        let moves = match piece.get_type() {
            PieceType::Pawn => self.generate_pawn_moves(board, piece, square_index),
            PieceType::Knight => self.generate_knight_moves(board, piece, square_index),
            PieceType::Bishop => self.generate_bishop_moves(board, piece, square_index),
            PieceType::Rook => self.generate_rook_moves(board, piece, square_index),
            PieceType::Queen => self.generate_queen_moves(board, piece, square_index),
            PieceType::King => self.generate_king_moves(board, piece, square_index),
        };

        Some(moves)
    }

    fn generate_pawn_moves(
        &self,
        board: &Board,
        piece: &Piece,
        square_index: usize,
    ) -> Vec<ChessMove> {
        let moves: Vec<ChessMove> = vec![];

        // 1. Handle basic move
        // 2. Handle moving by two squares on start row
        // 3. Handle attacks
        // 4. Handle en-passant
        // 5. Handle promotion
        // 6. Handle King Safety

        moves
    }

    fn generate_knight_moves(
        &self,
        board: &Board,
        piece: &Piece,
        square_index: usize,
    ) -> Vec<ChessMove> {
        let moves: Vec<ChessMove> = vec![];

        moves
    }

    fn generate_bishop_moves(
        &self,
        board: &Board,
        piece: &Piece,
        square_index: usize,
    ) -> Vec<ChessMove> {
        let moves: Vec<ChessMove> = vec![];

        moves
    }

    fn generate_rook_moves(
        &self,
        board: &Board,
        piece: &Piece,
        square_index: usize,
    ) -> Vec<ChessMove> {
        let moves: Vec<ChessMove> = vec![];

        moves
    }

    fn generate_queen_moves(
        &self,
        board: &Board,
        piece: &Piece,
        square_index: usize,
    ) -> Vec<ChessMove> {
        let moves: Vec<ChessMove> = vec![];

        moves
    }

    fn generate_king_moves(
        &self,
        board: &Board,
        piece: &Piece,
        square_index: usize,
    ) -> Vec<ChessMove> {
        let moves: Vec<ChessMove> = vec![];

        moves
    }
}
