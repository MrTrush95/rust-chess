use crate::color::Color;

pub struct Rank(u8);

impl Rank {
    pub fn from_square(square: u8) -> Rank {
        Rank(7 - (square / 8))
    }

    pub fn get_index(&self) -> u8 {
        self.0
    }

    pub fn is_piece_rank(&self, color: Color) -> bool {
        match color {
            Color::White => self.0 == 0,
            Color::Black => self.0 == 7,
        }
    }

    pub fn is_pawn_rank(&self, color: Color) -> bool {
        match color {
            Color::White => self.0 == 1,
            Color::Black => self.0 == 6,
        }
    }

    pub fn to_notation(&self) -> char {
        match self.0 {
            0 => '8',
            1 => '7',
            2 => '6',
            3 => '5',
            4 => '4',
            5 => '3',
            6 => '2',
            7 => '1',
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rank() {
        for rank_index in 0..8 {
            for file_index in 0..8 {
                let square = rank_index * 8 + file_index;

                assert_eq!(Rank::from_square(square).get_index(), 7 - rank_index);
            }
        }
    }

    #[test]
    fn test_is_piece_rank() {
        let c1_rank = Rank::from_square(58);
        let c8_rank = Rank::from_square(2);

        let c2_rank = Rank::from_square(50);
        let c7_rank = Rank::from_square(10);

        assert_eq!(c1_rank.is_piece_rank(Color::White), true);
        assert_eq!(c8_rank.is_piece_rank(Color::Black), true);

        assert_eq!(c1_rank.is_piece_rank(Color::Black), false);
        assert_eq!(c8_rank.is_piece_rank(Color::White), false);

        assert_eq!(c2_rank.is_piece_rank(Color::White), false);
        assert_eq!(c7_rank.is_piece_rank(Color::Black), false);
    }

    #[test]
    fn test_is_pawn_rank() {
        let c2_rank = Rank::from_square(50);
        let c7_rank = Rank::from_square(10);

        assert_eq!(c2_rank.is_pawn_rank(Color::White), true);
        assert_eq!(c7_rank.is_pawn_rank(Color::Black), true);
    }
}
