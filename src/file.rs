use std::unreachable;

pub struct File(u8);

impl File {
    pub fn from_square(square: u8) -> File {
        File(square % 8)
    }

    pub fn get_index(&self) -> u8 {
        self.0
    }

    pub fn is_first_file(&self) -> bool {
        self.0 == 0
    }

    pub fn is_last_file(&self) -> bool {
        self.0 == 7
    }

    pub fn to_notation(&self) -> char {
        match self.0 {
            0 => 'a',
            1 => 'b',
            2 => 'c',
            3 => 'd',
            4 => 'e',
            5 => 'f',
            6 => 'g',
            7 => 'h',
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_file() {
        for rank_index in 0..8 {
            for file_index in 0..8 {
                let square = rank_index * 8 + file_index;

                assert_eq!(File::from_square(square).get_index(), file_index);
            }
        }
    }

    #[test]
    fn test_is_first_file() {
        let a1_file = File::from_square(0);
        let c1_file = File::from_square(58);

        assert_eq!(a1_file.is_first_file(), true);
        assert_eq!(c1_file.is_first_file(), false);
    }

    #[test]
    fn test_is_last_file() {
        let h1_file = File::from_square(63);
        let c1_file = File::from_square(58);

        assert_eq!(h1_file.is_last_file(), true);
        assert_eq!(c1_file.is_first_file(), false);
    }
}
