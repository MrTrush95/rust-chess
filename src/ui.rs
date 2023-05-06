use std::io;

use crate::board::Board;

pub fn start_game(board: &mut Board) {
    loop {
        // clear_board();

        draw_board(&board);

        println!("Enter the move coordinates: ");

        let mut buf = String::new();
        io::stdin()
            .read_line(&mut buf)
            .expect("Could not parse user input");

        let result = board.try_make_move(48, 32);
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
