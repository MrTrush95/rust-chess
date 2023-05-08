use crate::{
    board::Board,
    chess_move::ChessMove,
    color::Color,
    file::File,
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

        let forward_move: i32 = 8;
        // TODO: It would be great if we could have a square
        //       struct that contains those two other struct.
        let rank = Rank::from_square(square_index);
        let file = File::from_square(square_index);
        let offset: i32 = if piece.is_color(Color::White) { -1 } else { 1 };

        let mut quiet_moves: Vec<i32> = vec![forward_move * offset];
        let mut attack_moves: Vec<i32> = vec![7, 9];

        let space_ahead_occupied =
            board.squares[(square_index as i32 + (forward_move * offset)) as usize].is_some();

        if rank.is_pawn_rank(piece.get_color()) && !space_ahead_occupied {
            quiet_moves.push(forward_move * 2 * offset);
        }

        if !file.is_first_file() {
            attack_moves.push(7 * offset);
        }

        if !file.is_last_file() {
            attack_moves.push(9 * offset);
        }

        for quiet_move in quiet_moves.iter() {
            let space_occupied =
                board.squares[(square_index as i32 + *quiet_move) as usize].is_some();

            if !space_occupied {
                moves.push(ChessMove::new(
                    square_index,
                    (square_index as i32 + *quiet_move) as u8,
                ));
            }
        }

        for attack_move in attack_moves.iter() {
            let space_occupied =
                board.squares[(square_index as i32 + *attack_move) as usize].is_some();

            if space_occupied {
                moves.push(ChessMove::new(
                    square_index,
                    (square_index as i32 + *attack_move) as u8,
                ));
            }
        }

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
