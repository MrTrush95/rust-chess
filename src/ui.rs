use crate::board::Board;

pub fn draw_board(board: Board) {
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
