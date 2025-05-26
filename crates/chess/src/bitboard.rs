use numext_fixed_uint::U256;

use crate::types::Dimensions;

use core::ops::*;

pub trait BitBoard:
    Sized
    + Clone
    + Eq
    + PartialEq
    + std::fmt::Debug
{
    fn zero() -> Self;
    fn one() -> Self;
    fn max_value() -> Self;
    fn min_value() -> Self;


    fn bit(&self, index: usize) -> Option<bool>;
    fn byte(&self, index: usize) -> Option<u8>;
    fn set_bit(&mut self, index: usize, value: bool)->bool;
    fn set_byte(&mut self, index: usize, value: u8)->bool;

    
    fn count_ones(&self) -> u32;
    fn count_zeros(&self) -> u32;
    fn leading_zeros(&self) -> u32;
    fn trailing_zeros(&self) -> u32;
    fn size_of() -> usize;

    fn overflowing_add(self, rhs: Self) -> (Self, bool);
    fn overflowing_sub(self, rhs: Self) -> (Self, bool);
    fn overflowing_mul(self, rhs: Self) -> (Self, bool);
    fn checked_add(self, rhs: Self) -> Option<Self>;
    fn checked_sub(self, rhs: Self) -> Option<Self>;
    fn checked_mul(self, rhs: Self) -> Option<Self>;

    
    /*fn overflowing_shl(self, n: u32) -> (Self, bool);
    fn overflowing_shr(self, n: u32) -> (Self, bool);
    fn checked_shl(self, n: u32) -> Option<Self>;
    fn checked_shr(self, n: u32) -> Option<Self>;*/


    fn is_zero(&self) -> bool;
    fn is_power_of_two(&self) -> bool;
    fn is_max(&self) -> bool;


    // Min/max
    fn lowest_one(&self) -> Option<usize>;
    fn highest_one(&self) -> Option<usize>;
}


#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BitBoard64(pub u64);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BitBoard256(pub U256);

impl BitBoard for BitBoard64 {
    fn zero() -> Self {
        BitBoard64(0)
    }

    fn one() -> Self {
        BitBoard64(1)
    }

    fn max_value() -> Self {
        BitBoard64(u64::MAX)
    }

    fn min_value() -> Self {
        BitBoard64(u64::MIN)
    }

    fn bit(&self, index: usize) -> Option<bool> {
        if index >= 64 {
            None
        } else {
            Some((self.0 & (1 << index)) != 0)
        }
    }
    

    fn byte(&self, index: usize) -> Option<u8> {
        if index < 8 {
            Some(((self.0 >> (index * 8)) & 0xFF) as u8)
        } else {
            None
        }
    }

    fn set_bit(&mut self, index: usize, value: bool) -> bool {
        if index >= 64 {
            return false;
        }
    
        if value {
            self.0 |= 1 << index;
        } else {
            self.0 &= !(1 << index);
        }
    
        true
    }
    
    fn set_byte(&mut self, index: usize, value: u8) -> bool {
        if index >= 8 {
            return false;
        }
    
        self.0 &= !(0xFF << (index * 8));
        self.0 |= (value as u64) << (index * 8);
    
        true
    }
    

    fn count_ones(&self) -> u32 {
        self.0.count_ones()
    }

    fn count_zeros(&self) -> u32 {
        self.0.count_zeros()
    }

    fn leading_zeros(&self) -> u32 {
        self.0.leading_zeros()
    }

    fn trailing_zeros(&self) -> u32 {
        self.0.trailing_zeros()
    }

    fn size_of() -> usize {
        64
    }

    fn overflowing_add(self, rhs: Self) -> (Self, bool) {
        let (res, overflow) = self.0.overflowing_add(rhs.0);
        (BitBoard64(res), overflow)
    }

    fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
        let (res, overflow) = self.0.overflowing_sub(rhs.0);
        (BitBoard64(res), overflow)
    }

    fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
        let (res, overflow) = self.0.overflowing_mul(rhs.0);
        (BitBoard64(res), overflow)
    }

    fn checked_add(self, rhs: Self) -> Option<Self> {
        self.0.checked_add(rhs.0).map(BitBoard64)
    }

    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(BitBoard64)
    }

    fn checked_mul(self, rhs: Self) -> Option<Self> {
        self.0.checked_mul(rhs.0).map(BitBoard64)
    }

    /*fn overflowing_shl(self, n: u32) -> (Self, bool) {
        let res = self.0 << n;
        (BitBoard64(res), false)
    }

    fn overflowing_shr(self, n: u32) -> (Self, bool) {
        let res = self.0 >> n;
        (BitBoard64(res), false)
    }

    fn checked_shl(self, n: u32) -> Option<Self> {
        if n >= 64 { None } else { Some(BitBoard64(self.0 << n)) }
    }

    fn checked_shr(self, n: u32) -> Option<Self> {
        if n >= 64 { None } else { Some(BitBoard64(self.0 >> n)) }
    }*/


    fn is_zero(&self) -> bool {
        self.0 == 0
    }

    fn is_power_of_two(&self) -> bool {
        self.0.is_power_of_two()
    }

    fn is_max(&self) -> bool {
        self.0 == u64::MAX
    }

    fn lowest_one(&self) -> Option<usize> {
        if self.0 == 0 { None } else { Some(self.0.trailing_zeros() as usize) }
    }

    fn highest_one(&self) -> Option<usize> {
        if self.0 == 0 { None } else { Some(63 - self.0.leading_zeros() as usize) }
    }
}

impl BitBoard for BitBoard256{
    fn zero() -> Self {
        BitBoard256(U256::zero())
    }

    fn one() -> Self {
        BitBoard256(U256::one())
    }

    fn max_value() -> Self {
        BitBoard256(U256::max_value())
    }

    fn min_value() -> Self {
        BitBoard256(U256::min_value())
    }


    fn bit(&self, index: usize) -> Option<bool> {
        self.0.bit(index)
    }

    fn byte(&self, index: usize) -> Option<u8> {
        self.0.byte(index)
    }

    fn set_bit(&mut self, index: usize, value: bool)  -> bool{
        self.0.set_bit(index, value)
    }

    fn set_byte(&mut self, index: usize, value: u8) ->bool{
        self.0.set_byte(index, value)
    }

    fn count_ones(&self) -> u32 {
        self.0.count_ones()
    }

    fn count_zeros(&self) -> u32 {
        self.0.count_zeros()
    }

    fn leading_zeros(&self) -> u32 {
        self.0.leading_zeros()
    }

    fn trailing_zeros(&self) -> u32 {
        self.0.trailing_zeros()
    }

    fn size_of() -> usize {
        256
    }

    fn overflowing_add(self, rhs: Self) -> (Self, bool) {
        let (res, overflow) = self.0.overflowing_add(&rhs.0);
        (BitBoard256(res), overflow)
    }

    fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
        let (res, overflow) = self.0.overflowing_sub(&rhs.0);
        (BitBoard256(res), overflow)
    }

    fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
        let (res, overflow) = self.0.overflowing_mul(&rhs.0);
        (BitBoard256(res), overflow)
    }

    fn checked_add(self, rhs: Self) -> Option<Self> {
        self.0.checked_add(&rhs.0).map(BitBoard256)
    }

    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(&rhs.0).map(BitBoard256)
    }

    fn checked_mul(self, rhs: Self) -> Option<Self> {
        self.0.checked_mul(&rhs.0).map(BitBoard256)
    }

    /*fn overflowing_shl(self, n: u32) -> (Self, bool) {
        let res = self.0 << n;
        (BitBoard256(res), false)
    }

    fn overflowing_shr(self, n: u32) -> (Self, bool) {
        let res = self.0 >> n;
        (BitBoard256(res), false)
    }

    fn checked_shl(self, rhs: u128) -> Option<Self> {
        self.0.checked_shl(rhs)
    }

    fn checked_shr(self, rhs: u128) -> Option<Self> {
        self.0.checked_shr(rhs)
    }*/

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    fn is_power_of_two(&self) -> bool {
        self.0.is_power_of_two()
    }

    fn is_max(&self) -> bool {
        self.0.is_max()
    }

    fn lowest_one(&self) -> Option<usize> {
        self.0.lowest_one()
    }

    fn highest_one(&self) -> Option<usize> {
        self.0.highest_one()
    }
}



pub fn to_pos(x:u8,y:u8) -> usize{
    ((x*16)+y).into()
}

pub fn add_u8_i8(x:u8,y:i8)->Option<u8>{
    let res = x as i16 + y as i16;
    if res<0 { return None}
    Some(res  as u8)
}

pub fn to_col(pos:u8)->u8{
    pos%16
}

pub fn to_row(pos:u8)->u8{
    pos/16
}


pub fn sq_to_notation(pos:u8,rows:u8)->String{
    let (row,col) = (to_row(pos),to_col(pos));
    /* Since bitboard is in little endian, LSB is a8 equivalent(black's lower left)
        So first rank in the bitboard is black's -> notation wise first rank is white's
     */
    let row_char =  rows-row ; 
    let col_char = (b'a' + col) as char;
    format!("{}{}", col_char, row_char)
}

pub fn display_bitboard<T: BitBoard>(bitboard: &T)->String{
    let mut bb_string = String::new().to_owned();
    bb_string.push_str("  1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 \n");
    for i in 0..16{
        bb_string.push_str(&(((i+1)%10).to_string()+" "));
        for j in 0..16{
            let index = to_pos(i,j);
            if bitboard.bit(index).unwrap_or(false){
                bb_string.push_str("1 ");
            } else{
                bb_string.push_str("- " );
            }
        }
        bb_string.push_str("\n");
    }
    bb_string
}

pub fn display_bitboard_with_board_desc<T: BitBoard>(bitboard: &T, desc: &str)->String{
    let bitboard_str = display_bitboard(bitboard);
    let mut new_str = desc.to_string();
    new_str.push_str("\n");
    new_str.push_str(&bitboard_str);
    println!("{}",new_str);
    new_str

}

pub struct SizeDependentBitboards<T: BitBoard> {
    pub full_bitboard: T,
    pub row_boundary: T,
    pub col_boundary: T,
}

impl<T: BitBoard> SizeDependentBitboards<T> {
    pub fn new(dimensions: &Dimensions) -> Self {
        let mut full_bitboard = T::zero();
        let mut row_boundary = T::zero();
        let mut col_boundary = T::zero();

        for i in 0..dimensions.height{
            for j in 0..dimensions.width{
                full_bitboard.set_bit(to_pos(i,j),true);
                if i==0||i==dimensions.height-1{
                    row_boundary.set_bit(to_pos(i,j), true);
                }
                if j==0||i==dimensions.width-1{
                    col_boundary.set_bit(to_pos(i,j), true);
                }
            }
        }
        SizeDependentBitboards { full_bitboard, row_boundary , col_boundary}
    }
}