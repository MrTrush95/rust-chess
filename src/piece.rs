use core::fmt;

use crate::color::Color;

/// Piece are stored as u8 to preserve memory
/// bits 0..2 -> PieceType
/// bits 3-4 -> Color
#[derive(Clone, Copy, Debug)]
pub struct Piece(u8);

#[derive(Debug, FromPrimitive, PartialEq)]
pub enum PieceType {
    Pawn = 1,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

const PIECE_TYPE_MASK: u8 = 0b00111;
const COLOR_MASK: u8 = 0b11000;

impl Piece {
    pub fn new(piece_type: PieceType, color: Color) -> Piece {
        Piece(((color as u8) << 3) | piece_type as u8)
    }

    pub fn get_color(&self) -> Color {
        num::FromPrimitive::from_u8((self.0 & COLOR_MASK) >> 3).expect("color")
    }

    pub fn get_type(&self) -> PieceType {
        num::FromPrimitive::from_u8(self.0 & PIECE_TYPE_MASK).expect("piece_type")
    }

    pub fn is_color(&self, color: Color) -> bool {
        self.get_color() == color
    }

    pub fn is_type(&self, piece_type: PieceType) -> bool {
        self.get_type() == piece_type
    }

    pub fn to_index(&self) -> u8 {
        self.0
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut notation = match self.get_type() {
            PieceType::Pawn => String::from("p"),
            PieceType::Knight => String::from("n"),
            PieceType::Bishop => String::from("b"),
            PieceType::Rook => String::from("r"),
            PieceType::Queen => String::from("q"),
            PieceType::King => String::from("k"),
        };

        if self.is_color(Color::White) {
            notation = String::from(notation).to_ascii_uppercase();
        }

        write!(f, "{}", notation)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        let white_pawn = Piece::new(PieceType::Pawn, Color::White);
        let black_pawn = Piece::new(PieceType::Pawn, Color::Black);

        let white_queen = Piece::new(PieceType::Queen, Color::White);

        assert_eq!(white_pawn.to_index(), 0b01001);
        assert_eq!(black_pawn.to_index(), 0b10001);
        assert_eq!(white_queen.to_index(), 0b01101);
    }

    #[test]
    fn test_display() {
        let white_pawn = Piece::new(PieceType::Pawn, Color::White);
        let black_queen = Piece::new(PieceType::Queen, Color::Black);

        assert_eq!(format!("{}", white_pawn), "P");
        assert_eq!(format!("{}", black_queen), "q");
    }

    #[test]
    fn test_get_color() {
        let white_pawn = Piece::new(PieceType::Pawn, Color::White);
        let black_queen = Piece::new(PieceType::Queen, Color::Black);

        assert_eq!(white_pawn.get_color(), Color::White);
        assert_eq!(black_queen.get_color(), Color::Black);
    }

    #[test]
    fn test_get_type() {
        let white_pawn = Piece::new(PieceType::Pawn, Color::White);
        let black_queen = Piece::new(PieceType::Queen, Color::Black);

        assert_eq!(white_pawn.get_type(), PieceType::Pawn);
        assert_eq!(black_queen.get_type(), PieceType::Queen);
    }

    #[test]
    fn test_is_color() {
        let white_pawn = Piece::new(PieceType::Pawn, Color::White);
        let black_queen = Piece::new(PieceType::Queen, Color::Black);

        assert_eq!(white_pawn.is_color(Color::White), true);
        assert_eq!(black_queen.is_color(Color::White), false);
    }

    #[test]
    fn test_is_type() {
        let white_pawn = Piece::new(PieceType::Pawn, Color::White);
        let black_queen = Piece::new(PieceType::Queen, Color::Black);

        assert_eq!(white_pawn.is_type(PieceType::Pawn), true);
        assert_eq!(black_queen.is_type(PieceType::Pawn), false);
    }
}
