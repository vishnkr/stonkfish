use super::dims::Dimensions;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Square(pub u16);

impl Square {
    #[inline]
    pub fn idx(self) -> u16 {
        self.0
    }
    
    #[inline]
    pub fn from_rank_file(rank: u8, file: u8, dims: &Dimensions) -> Self {
        Self(dims.file_rank_to_square(file, rank) as u16)
    }
    
    #[inline]
    pub fn file_rank(self, dims: &Dimensions) -> (u8, u8) {
        dims.square_to_file_rank(self.0 as u8)
    }

    pub fn to_string(self, dims: &Dimensions) -> String {
        let (rank, file) = self.file_rank(dims);
        let file_char = (b'a' + file as u8) as char;
        format!("{file_char}{}", rank + 1)
    }
}

