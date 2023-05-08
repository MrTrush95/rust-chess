use std::io;

use crate::{board::Board, chess_move::ChessMove};

pub fn start_game(board: &mut Board) {
    loop {
        clear_board();

        draw_board(&board);

        println!("Enter the move coordinates: ");

        let mut user_move = String::new();
        io::stdin()
            .read_line(&mut user_move)
            .expect("Could not parse user input");

        let user_move = parse_user_move(user_move.trim().to_owned());

        let result = board.try_make_move(user_move);
    }
}

pub fn draw_board(board: &Board) {
    for file in 0..8 {
        draw_current_file(file);
        for rank in 0..8 {
            let index = file * 8 + rank;
            draw_square(index, &board)
        }
        draw_next_rank();
    }
    draw_ranks();
}

pub fn clear_board() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn draw_square(index: usize, board: &Board) {
    print!(
        "{}",
        match board.squares[index] {
            Some(piece) => format!("{}  ", piece.to_string()),
            None => String::from(".  "),
        }
    )
}

fn draw_current_file(file: usize) {
    print!("{}   ", 8 - file)
}

fn draw_next_rank() {
    println!();
}

fn draw_ranks() {
    println!("\n    a  b  c  d  e  f  g  h");
}

fn parse_user_move(user_move: String) -> ChessMove {
    // TODO: Refactor this to a better implementation, this
    //       is temporary and use only for testing purpose.
    let mut moves = user_move.split_whitespace().into_iter();

    let start = parse_coord(moves.next().unwrap());
    let target = parse_coord(moves.next().unwrap());

    ChessMove::new(start, target)
}

fn parse_coord(coord: &str) -> u8 {
    // TODO: Refactor this to a better implementation, this
    //       is temporary and use only for testing purpose.
    let letters = String::from("abcdefgh");
    let mut chars = coord.chars();

    let col = letters.find(chars.next().unwrap()).unwrap();
    let row = 8 - chars.next().unwrap().to_digit(10).unwrap() as usize;

    (row * 8 + col) as u8
}
