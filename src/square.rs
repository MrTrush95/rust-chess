use crate::{file::File, rank::Rank};

pub struct Square(u8);

impl Square {
    pub fn new(index: u8) -> Square {
        Square(index)
    }

    pub fn is_valid(index: i8) -> bool {
        index >= 0 && index <= 63
    }

    pub fn get_rank(&self) -> Rank {
        Rank::from_square(self.0)
    }

    pub fn get_file(&self) -> File {
        File::from_square(self.0)
    }

    pub fn to_notation(&self) -> String {
        let rank_notation = self.get_rank().to_notation();
        let file_notation = self.get_file().to_notation();

        format!("{}{}", file_notation, rank_notation)
    }
}
