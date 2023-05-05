pub struct ChessMove(u16);

pub const START_MASK: u16 = 0b0000000000111111;
pub const TARGET_MASK: u16 = 0b0000111111000000;

impl ChessMove {
    pub fn new(start: u16, target: u16) -> ChessMove {
        ChessMove((start | (target << 6)) as u16)
    }

    pub fn get_start_index(&self) -> u8 {
        (&self.0 & START_MASK) as u8
    }

    pub fn get_target_index(&self) -> u8 {
        ((&self.0 & TARGET_MASK) >> 6) as u8
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_chess_move() {
        let start: u16 = 8;
        let target: u16 = 16;

        let chess_move = ChessMove::new(start, target);

        assert_eq!(chess_move.get_start_index(), start as u8);
        assert_eq!(chess_move.get_target_index(), target as u8);
    }
}
