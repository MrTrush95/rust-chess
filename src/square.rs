use crate::{file::File, rank::Rank};

pub struct Square(u8);

impl Square {
    pub fn new(index: u8) -> Square {
        Square(index)
    }

    pub fn get_rank(&self) -> Rank {
        Rank::from_square(self.0)
    }

    pub fn get_file(&self) -> File {
        File::from_square(self.0)
    }
}
