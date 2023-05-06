use crate::{
    board::Board,
    chess_move::ChessMove,
    color::Color,
    piece::{Piece, PieceType},
    rank::Rank,
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
                Some(piece) => self.generate_pieces_moves(board, piece, square_index as u8),
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
        square_index: u8,
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
        square_index: u8,
    ) -> Vec<ChessMove> {
        let mut moves: Vec<ChessMove> = vec![];

        let forward_move: i8 = 8;
        let forward_move_two: i8 = 16;
        let rank = Rank::from_square(square_index);
        let direction: i8 = if piece.is_color(Color::White) { -1 } else { 1 };

        // Handle basic move
        let space_occupied =
            board.squares[(square_index as i8 + (forward_move * direction)) as usize].is_some();

        if !space_occupied {
            moves.push(ChessMove::new(
                square_index,
                (square_index as i8 + (forward_move * direction)) as u8,
            ));
        }

        // Handle moving by two squares on start row
        let space_occupied =
            board.squares[(square_index as i8 + (forward_move_two * direction)) as usize].is_some();

        if rank.is_pawn_rank(piece.get_color()) && !space_occupied {
            moves.push(ChessMove::new(
                square_index,
                (square_index as i8 + (forward_move_two * direction)) as u8,
            ));
        }

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
        square_index: u8,
    ) -> Vec<ChessMove> {
        let moves: Vec<ChessMove> = vec![];

        moves
    }

    fn generate_bishop_moves(
        &self,
        board: &Board,
        piece: &Piece,
        square_index: u8,
    ) -> Vec<ChessMove> {
        let moves: Vec<ChessMove> = vec![];

        moves
    }

    fn generate_rook_moves(
        &self,
        board: &Board,
        piece: &Piece,
        square_index: u8,
    ) -> Vec<ChessMove> {
        let moves: Vec<ChessMove> = vec![];

        moves
    }

    fn generate_queen_moves(
        &self,
        board: &Board,
        piece: &Piece,
        square_index: u8,
    ) -> Vec<ChessMove> {
        let moves: Vec<ChessMove> = vec![];

        moves
    }

    fn generate_king_moves(
        &self,
        board: &Board,
        piece: &Piece,
        square_index: u8,
    ) -> Vec<ChessMove> {
        let moves: Vec<ChessMove> = vec![];

        moves
    }
}
