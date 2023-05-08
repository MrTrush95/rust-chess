use console_engine::{ConsoleEngine, KeyCode, MouseButton};

use crate::{board::Board, chess_move::ChessMove, string::pad_right};

pub struct UI<'a> {
    board: &'a mut Board,
    engine: ConsoleEngine,
    selected_square: Option<u8>,
    message: String,
}

const BOARD_START_X: usize = 10;
const BOARD_START_Y: usize = 5;
const SQUARE_WIDTH: usize = 3;
const RANK_CHARS: [&str; 8] = ["8", "7", "6", "5", "4", "3", "2", "1"];
const FILE_CHARS: [&str; 8] = ["a", "b", "c", "d", "e", "f", "g", "h"];

impl UI<'_> {
    pub fn new(board: &mut Board, width: u32, height: u32) -> UI {
        let engine = console_engine::ConsoleEngine::init_fill_require(width, height, 12).unwrap();

        UI {
            board,
            engine,
            selected_square: None,
            message: String::new(),
        }
    }

    pub fn start_game(&mut self) {
        loop {
            self.engine.wait_frame();
            self.engine.clear_screen();

            if self.engine.is_key_pressed(KeyCode::Char('q')) {
                // if the user presses 'q' :
                break; // exits app
            }

            self.draw_message();
            self.draw_board();
            self.draw_coordinates();

            self.handle_click();

            self.engine.draw();
        }
    }

    fn draw_board(&mut self) {
        for file_index in 0..8 {
            for rank_index in 0..8 {
                let square_index = file_index * 8 + rank_index;

                self.engine.print(
                    (BOARD_START_X + rank_index * SQUARE_WIDTH) as i32,
                    (BOARD_START_Y + file_index) as i32,
                    match self.board.squares[square_index as usize] {
                        Some(piece) => pad_right(&piece.to_string(), SQUARE_WIDTH, ' '),
                        None => pad_right(".", SQUARE_WIDTH, ' '),
                    }
                    .as_str(),
                )
            }
        }
    }

    fn draw_coordinates(&mut self) {
        for (index, file_char) in FILE_CHARS.iter().enumerate() {
            self.engine.print(
                (BOARD_START_X + index * SQUARE_WIDTH) as i32,
                (BOARD_START_Y + 9) as i32,
                file_char,
            )
        }

        for (index, rank_char) in RANK_CHARS.iter().enumerate() {
            self.engine.print(
                (BOARD_START_X - SQUARE_WIDTH - 1) as i32,
                (BOARD_START_Y + index) as i32,
                rank_char,
            )
        }
    }

    fn draw_message(&mut self) {
        self.engine.print(
            BOARD_START_X as i32,
            (BOARD_START_Y - 1) as i32,
            &self.message,
        );
    }

    fn handle_click(&mut self) {
        let mouse_pos = self.engine.get_mouse_press(MouseButton::Left);
        if let Some((pos_x, pos_y)) = mouse_pos {
            let square = self.get_square_from_pos(pos_x, pos_y);

            match square {
                Some(square) => self.on_square_selected(square),
                None => (),
            }
        }
    }

    fn get_square_from_pos(&mut self, pos_x: u32, pos_y: u32) -> Option<u8> {
        if pos_x < BOARD_START_X as u32
            || pos_x > (BOARD_START_X + SQUARE_WIDTH * 8) as u32
            || pos_y < BOARD_START_Y as u32
            || pos_y > (BOARD_START_Y + 7) as u32
        {
            return None;
        }

        let rank = (pos_y as usize) - BOARD_START_Y;
        let file = ((pos_x as usize) - BOARD_START_X) / SQUARE_WIDTH;

        let square = (rank * 8 + file) as u8;

        Some(square)
    }

    fn on_square_selected(&mut self, square: u8) {
        match self.selected_square {
            Some(selected_square) => {
                let chess_move = ChessMove::new(selected_square, square);
                let result = self.board.try_make_move(chess_move);

                self.selected_square = None;
            }
            None => self.selected_square = Some(square),
        }
    }
}
