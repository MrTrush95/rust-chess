use rstest::rstest;
use std::assert_eq;

use rust_chess::{board::Board, move_generator::MoveGenerator};

#[rstest]
#[case(1, 20)]
#[case(2, 400)]
// #[case(3, 8902)]
fn it_generate_valid_moves(#[case] depth: i32, #[case] nodes: u64) {
    let board = Board::default();
    let total_moves = perft(board, depth);

    assert_eq!(total_moves, nodes);
}

fn perft(board: Board, depth: i32) -> u64 {
    let mut move_count = 0;
    let move_generator = MoveGenerator::new(&board, false);

    if depth == 1 {
        return move_generator.moves.len() as u64;
    }

    for chess_move in move_generator.moves.iter() {
        let mut clone_board = board.clone();

        clone_board
            .try_make_move(*chess_move)
            .expect("Invalid Board Move");

        move_count = move_count + perft(clone_board, depth - 1);
    }

    move_count
}
