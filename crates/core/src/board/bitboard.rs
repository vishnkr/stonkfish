use super::square::Square;
use super::dims::Dimensions;

pub trait BB: Copy + Sized {
    fn empty() -> Self;
    fn full() -> Self;
    fn from_square(sq: Square) -> Self;
    fn clear(self, sq: Square) -> Self;
    fn set(self, sq: Square) -> Self;
    fn contains(self, sq: Square) -> bool;
    fn pop_lsb(&mut self) -> Option<Square>;
    fn count(self) -> u32;
    fn is_empty(self) -> bool;
    fn union(self, other: Self) -> Self;
    fn intersect(self, other: Self) -> Self;
    fn difference(self, other: Self) -> Self;
}

/// BitBoard64 - u64 implementation for boards up to 64 squares
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitBoard64(pub u64);

impl BB for BitBoard64 {
    #[inline] 
    fn empty() -> Self { 
        BitBoard64(0) 
    }
    
    #[inline] 
    fn full() -> Self { 
        BitBoard64(u64::MAX) 
    }

    #[inline]
    fn from_square(sq: Square) -> Self {
        BitBoard64(1u64 << sq.0)
    }

    #[inline]
    fn clear(self, sq: Square) -> Self {
        BitBoard64(self.0 & !(1u64 << sq.0))
    }

    #[inline]
    fn set(self, sq: Square) -> Self {
        BitBoard64(self.0 | (1u64 << sq.0))
    }

    #[inline]
    fn contains(self, sq: Square) -> bool {
        (self.0 >> sq.0) & 1 == 1
    }

    #[inline]
    fn pop_lsb(&mut self) -> Option<Square> {
        if self.0 == 0 { 
            return None; 
        }
        let lsb = self.0.trailing_zeros() as u16;
        self.0 &= self.0 - 1;
        Some(Square(lsb))
    }
    
    #[inline]
    fn count(self) -> u32 {
        self.0.count_ones()
    }
    
    #[inline]
    fn is_empty(self) -> bool {
        self.0 == 0
    }
    
    #[inline]
    fn union(self, other: Self) -> Self {
        BitBoard64(self.0 | other.0)
    }
    
    #[inline]
    fn intersect(self, other: Self) -> Self {
        BitBoard64(self.0 & other.0)
    }
    
    #[inline]
    fn difference(self, other: Self) -> Self {
        BitBoard64(self.0 & !other.0)
    }
}

/// Simple U256 implementation using [u64; 4]
/// This allows us to support boards up to 256 squares
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct U256([u64; 4]);

impl U256 {
    fn zero() -> Self {
        U256([0, 0, 0, 0])
    }
    
    fn max() -> Self {
        U256([u64::MAX, u64::MAX, u64::MAX, u64::MAX])
    }
    
    fn from_u64(val: u64) -> Self {
        U256([val, 0, 0, 0])
    }
    
    fn is_zero(&self) -> bool {
        self.0.iter().all(|&w| w == 0)
    }
    
    fn shl(&self, bits: u16) -> Self {
        if bits >= 256 {
            return Self::zero();
        }
        
        let word_shift = (bits / 64) as usize;
        let bit_shift = bits % 64;
        
        if word_shift >= 4 {
            return Self::zero();
        }
        
        let mut result = [0u64; 4];
        
        if bit_shift == 0 {
            // word shift
            for i in word_shift..4 {
                result[i - word_shift] = self.0[i];
            }
        } else {
            // bit shift across words
            for i in word_shift..4 {
                let src_idx = i;
                let dst_idx = i - word_shift;
                
                if src_idx < 4 {
                    result[dst_idx] |= self.0[src_idx] << bit_shift;
                }
                
                if src_idx + 1 < 4 && bit_shift > 0 {
                    result[dst_idx] |= self.0[src_idx + 1] >> (64 - bit_shift);
                }
            }
        }
        
        U256(result)
    }
    
    fn bitwise_and(&self, other: &Self) -> Self {
        U256([
            self.0[0] & other.0[0],
            self.0[1] & other.0[1],
            self.0[2] & other.0[2],
            self.0[3] & other.0[3],
        ])
    }
    
    fn bitwise_or(&self, other: &Self) -> Self {
        U256([
            self.0[0] | other.0[0],
            self.0[1] | other.0[1],
            self.0[2] | other.0[2],
            self.0[3] | other.0[3],
        ])
    }
    
    fn bitwise_not(&self) -> Self {
        U256([
            !self.0[0],
            !self.0[1],
            !self.0[2],
            !self.0[3],
        ])
    }
    
    fn trailing_zeros(&self) -> Option<u16> {
        for (word_idx, &word) in self.0.iter().enumerate() {
            if word != 0 {
                return Some((word_idx * 64) as u16 + word.trailing_zeros() as u16);
            }
        }
        None
    }
    
    fn count_ones(&self) -> u32 {
        self.0.iter().map(|w| w.count_ones()).sum()
    }
}

impl std::ops::BitAnd for U256 {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        self.bitwise_and(&other)
    }
}

impl std::ops::BitOr for U256 {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        self.bitwise_or(&other)
    }
}

impl std::ops::Not for U256 {
    type Output = Self;
    fn not(self) -> Self {
        self.bitwise_not()
    }
}

impl std::ops::Shl<u16> for U256 {
    type Output = Self;
    fn shl(self, bits: u16) -> Self {
        U256::shl(&self, bits)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitBoard256(U256);

impl BB for BitBoard256 {
    #[inline]
    fn empty() -> Self {
        BitBoard256(U256::zero())
    }
    
    #[inline]
    fn full() -> Self {
        BitBoard256(U256::max())
    }
    
    #[inline]
    fn from_square(sq: Square) -> Self {
        let sq_idx = sq.0;
        let one = U256::from_u64(1);
        BitBoard256(one << sq_idx)
    }
    
    #[inline]
    fn clear(self, sq: Square) -> Self {
        let sq_bb = Self::from_square(sq);
        BitBoard256(self.0 & !sq_bb.0)
    }
    
    #[inline]
    fn set(self, sq: Square) -> Self {
        let sq_bb = Self::from_square(sq);
        BitBoard256(self.0 | sq_bb.0)
    }
    
    #[inline]
    fn contains(self, sq: Square) -> bool {
        let sq_bb = Self::from_square(sq);
        !(self.0 & sq_bb.0).is_zero()
    }
    
    #[inline]
    fn pop_lsb(&mut self) -> Option<Square> {
        if self.0.is_zero() {
            return None;
        }
        
        if let Some(bit_pos) = self.0.trailing_zeros() {
            // Clear the bit
            let one = U256::from_u64(1);
            let mask = !(one << bit_pos);
            self.0 = self.0 & mask;
            return Some(Square(bit_pos));
        }
        None
    }
    
    #[inline]
    fn count(self) -> u32 {
        self.0.count_ones()
    }
    
    #[inline]
    fn is_empty(self) -> bool {
        self.0.is_zero()
    }
    
    #[inline]
    fn union(self, other: Self) -> Self {
        BitBoard256(self.0 | other.0)
    }
    
    #[inline]
    fn intersect(self, other: Self) -> Self {
        BitBoard256(self.0 & other.0)
    }
    
    #[inline]
    fn difference(self, other: Self) -> Self {
        BitBoard256(self.0 & !other.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitBoard {
    Small(BitBoard64),
    Large(BitBoard256),
}

impl BitBoard {
    pub fn for_dims(dims: &Dimensions) -> Self {
        if dims.num_squares() <= 64 {
            BitBoard::Small(BitBoard64::empty())
        } else {
            BitBoard::Large(BitBoard256::empty())
        }
    }
    
    pub fn empty_for_dims(dims: &Dimensions) -> Self {
        Self::for_dims(dims)
    }
    
    pub fn full_for_dims(dims: &Dimensions) -> Self {
        if dims.num_squares() <= 64 {
            BitBoard::Small(BitBoard64::full())
        } else {
            BitBoard::Large(BitBoard256::full())
        }
    }
}

impl BB for BitBoard {
    fn empty() -> Self {

        BitBoard::Small(BitBoard64::empty())
    }
    
    fn full() -> Self {
        BitBoard::Small(BitBoard64::full())
    }
    
    fn from_square(sq: Square) -> Self {
        BitBoard::Small(BitBoard64::from_square(sq))
    }
    
    fn clear(self, sq: Square) -> Self {
        match self {
            BitBoard::Small(bb) => BitBoard::Small(bb.clear(sq)),
            BitBoard::Large(bb) => BitBoard::Large(bb.clear(sq)),
        }
    }
    
    fn set(self, sq: Square) -> Self {
        match self {
            BitBoard::Small(bb) => BitBoard::Small(bb.set(sq)),
            BitBoard::Large(bb) => BitBoard::Large(bb.set(sq)),
        }
    }
    
    fn contains(self, sq: Square) -> bool {
        match self {
            BitBoard::Small(bb) => bb.contains(sq),
            BitBoard::Large(bb) => bb.contains(sq),
        }
    }
    
    fn pop_lsb(&mut self) -> Option<Square> {
        match self {
            BitBoard::Small(bb) => {
                let result = bb.pop_lsb();
                *self = BitBoard::Small(*bb);
                result
            }
            BitBoard::Large(bb) => {
                let result = bb.pop_lsb();
                *self = BitBoard::Large(*bb);
                result
            }
        }
    }
    
    fn count(self) -> u32 {
        match self {
            BitBoard::Small(bb) => bb.count(),
            BitBoard::Large(bb) => bb.count(),
        }
    }
    
    fn is_empty(self) -> bool {
        match self {
            BitBoard::Small(bb) => bb.is_empty(),
            BitBoard::Large(bb) => bb.is_empty(),
        }
    }
    
    fn union(self, other: Self) -> Self {
        match (self, other) {
            (BitBoard::Small(a), BitBoard::Small(b)) => BitBoard::Small(a.union(b)),
            (BitBoard::Large(a), BitBoard::Large(b)) => BitBoard::Large(a.union(b)),
            _ => panic!("Cannot union different bitboard types"),
        }
    }
    
    fn intersect(self, other: Self) -> Self {
        match (self, other) {
            (BitBoard::Small(a), BitBoard::Small(b)) => BitBoard::Small(a.intersect(b)),
            (BitBoard::Large(a), BitBoard::Large(b)) => BitBoard::Large(a.intersect(b)),
            _ => panic!("Cannot intersect different bitboard types"),
        }
    }
    
    fn difference(self, other: Self) -> Self {
        match (self, other) {
            (BitBoard::Small(a), BitBoard::Small(b)) => BitBoard::Small(a.difference(b)),
            (BitBoard::Large(a), BitBoard::Large(b)) => BitBoard::Large(a.difference(b)),
            _ => panic!("Cannot difference different bitboard types"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::{Dimensions, Square};

    #[test]
    fn bitboard64_basics() {
        let dims = Dimensions::new(8, 8);
        let sq = Square(dims.file_rank_to_square(0, 0) as u16);
        let bb = BitBoard64::from_square(sq);
        assert!(bb.contains(sq));
    }
    
    #[test]
    fn bitboard_enum_small() {
        let dims = Dimensions::new(8, 8);
        let sq = Square::from_rank_file(0, 0, &dims);
        let bb = BitBoard::for_dims(&dims);
        let bb = bb.set(sq);
        assert!(bb.contains(sq));
        assert_eq!(bb.count(), 1);
    }
    
    #[test]
    fn bitboard_enum_large() {
        let dims = Dimensions::new(10, 10);
        let sq = Square::from_rank_file(0, 0, &dims);
        let bb = BitBoard::for_dims(&dims);
        let bb = bb.set(sq);
        assert!(bb.contains(sq));
        assert_eq!(bb.count(), 1);
    }
    
    #[test]
    fn bitboard256_basics() {
        let dims = Dimensions::new(10, 10);
        let sq1 = Square::from_rank_file(0, 0, &dims);
        let sq2 = Square::from_rank_file(5, 5, &dims);
        
        let bb1 = BitBoard256::from_square(sq1);
        let bb2 = BitBoard256::from_square(sq2);
        let combined = bb1.union(bb2);
        
        assert!(combined.contains(sq1));
        assert!(combined.contains(sq2));
        assert_eq!(combined.count(), 2);
    }
}
