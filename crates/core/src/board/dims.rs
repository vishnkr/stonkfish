
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Dimensions {
    pub width: u8,
    pub height: u8,
}

impl Dimensions {
    pub fn new(width: u8, height: u8) -> Self {
        assert!(width >= 5 && width <= 16, "Width must be between 5 and 16");
        assert!(height >= 5 && height <= 16, "Height must be between 5 and 16");
        Self { width, height }
    }
    
    pub fn standard() -> Self {
        Self::new(8, 8)
    }

    #[inline]
    pub fn num_squares(&self) -> u16 {
        (self.width as u16) * (self.height as u16)
    }
    
    /// files (width)
    #[inline]
    pub fn files(&self) -> u8 {
        self.width
    }
    
    /// ranks (height)
    #[inline]
    pub fn ranks(&self) -> u8 {
        self.height
    }

    #[inline]
    pub fn file_rank_to_square(&self, file: u8, rank: u8) -> u8 {
        debug_assert!(file < self.width, "File {} out of bounds (max {})", file, self.width);
        debug_assert!(rank < self.height, "Rank {} out of bounds (max {})", rank, self.height);
        rank * self.width + file
    }

    #[inline]
    pub fn square_to_file_rank(&self, sq: u8) -> (u8, u8) {
        let file = sq % self.width;
        let rank = sq / self.width;
        (file, rank)
    }
    
    #[inline]
    pub fn is_valid_file(&self, file: u8) -> bool {
        file < self.width
    }
    
    #[inline]
    pub fn is_valid_rank(&self, rank: u8) -> bool {
        rank < self.height
    }
}

