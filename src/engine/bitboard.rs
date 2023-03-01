use std::ops::{BitAnd,BitAndAssign,BitOr,BitOrAssign,Not,BitXor,BitXorAssign, Shl,ShlAssign, Shr,ShrAssign};
extern crate numext_fixed_uint;
use numext_fixed_uint::U256;

pub type Bitboard256 = U256;
pub type Bitboard64 = u64;

pub enum BitboardType {
    Bitboard256 = 256,
    Bitboard64 = 64
}

trait BitShifts: Sized + Shl<u16, Output=Self> + Shr<u16, Output=Self> + ShlAssign<u16> + ShrAssign<u16> + Shl<u32, Output=Self> + ShlAssign<u32> + Shr<u32, Output=Self> + ShrAssign<u32>{}
impl<T> BitShifts for T where T: Shl<u16, Output=Self> + Shr<u16, Output=Self> + ShlAssign<u16> + ShrAssign<u16> + Shl<u32, Output=Self> + ShlAssign<u32> + Shr<u32, Output=Self> + ShrAssign<u32>{}
pub trait Bitboard: BitShifts + Clone + BitAnd<Output=Self> + BitAndAssign + BitOr<Output=Self> + BitOrAssign + Not<Output=Self> + BitXor<Output=Self> + BitXorAssign {
    const BBTYPE: BitboardType;
    fn zero()->Self;
    fn is_zero(&self)->bool;
    fn bit(&self, index: usize) -> Option<bool>;
    fn set_bit(&mut self,index: usize, value: bool) -> bool;
    fn count_ones(&self) -> u32;
    fn lowest_one(&self) -> Option<usize>; 
    fn from_u16(value:u16)->Self;
}

/* 
So I'm working on a chess engine that should work on regular chess board sizes and also larger boards upto 16x16 for which I am using 64 bit and 256 bit integers as bitboards respectively. 
Since most of the logic is similar for both bitboard types, I grouped the common methods into a trait and used a generic type argument implementing this trait. The issue is that even though both u64 and the crate containing the U256 struct have std::ops traits for all bit operations implemented, I'm unable to perform bit operations on the generic T type. Would it be possible for both types to use their original implementation for BitOr, BitAnd, etc.? If not what's the best to solve this?

```
pub type Bitboard256 = U256;
pub type Bitboard64 = u64;
pub trait Bitboard:{
    const BBTYPE: BitboardType;
    fn zero()->Self;
    fn is_zero(&self)->bool;
    fn bit(&self, index: usize) -> Option<bool>;
    fn set_bit(&mut self,index: usize, value: bool) -> bool;
    fn count_ones(&self) -> u32;
    fn lowest_one(&self) -> Option<usize>;
 
}
```
I would like to do something like this which seems to work if used a zero value of bitboard64 or bitboard256 instead of T
```
fn random_fn<T:Bitboard>(){
    let bitboard1 = T::zero();
    let bitboard2 = T::zero();
    let bitboard3 = &bitboard1 | &bitboard2;
}

```

*/
//impl<T> Bitboard for T where T: Sized + BitAnd + BitAndAssign + BitOr + BitOrAssign + Not + BitXor + BitXorAssign{} 
impl Bitboard for Bitboard256{
    const BBTYPE: BitboardType = BitboardType::Bitboard256;

    fn zero() -> Self { Bitboard256::zero() }
    fn is_zero(&self) -> bool { self.is_zero()}
    fn bit(&self, index: usize) -> Option<bool> { self.bit(index) }

    fn set_bit(&mut self, index: usize, value: bool) -> bool { self.set_bit(index, value) }

    fn count_ones(&self) -> u32 { self.count_ones() }

    fn lowest_one(&self) -> Option<usize> { self.lowest_one() }

    fn from_u16(value:u16) -> Self{ Bitboard256::zero() | value}
}

impl Bitboard for Bitboard64 {
    const BBTYPE: BitboardType = BitboardType::Bitboard64;

    fn zero() -> Self { 0u64 }
    fn is_zero(&self) -> bool { *self==0 }
    fn bit(&self, index: usize) -> Option<bool> {
        if index >= 64 {
            return None;
        }
        Some((*self & (1u64 << index)) != 0)
    }

    fn set_bit(&mut self, index: usize, value: bool) -> bool {
        if index >= 64 {
            return false;
        }
        if value {
            *self |= 1u64 << index;
        } else {
            *self &= !(1u64 << index);
        }
        true
    }

    fn count_ones(&self) -> u32 { self.count_ones() }

    fn lowest_one(&self) -> Option<usize> {
        if *self == 0 {
            return None;
        }
        Some(self.trailing_zeros() as usize)
    }
    fn from_u16(value:u16) -> Self{ value as u64}
}


pub fn to_pos(x:u8,y:u8,bbtype:BitboardType) -> usize{
    let rows = match bbtype{
        BitboardType::Bitboard256 => 16,
        BitboardType::Bitboard64 => 8,
    };
    ((x*rows)+y).into()
}

pub fn to_col(pos:u8,bbtype:BitboardType)->u8{
    match bbtype{
        BitboardType::Bitboard256 => pos%16,
        BitboardType::Bitboard64 => pos%8,
    }
}

pub fn to_row(pos:u8,bbtype:BitboardType)->u8{
    match bbtype{
        BitboardType::Bitboard256 => pos/16,
        BitboardType::Bitboard64 => pos/8,
    }
}

pub fn display_bitboard<T:Bitboard>(bitboard:&T)->String{
    let x = match T::BBTYPE{
        BitboardType::Bitboard256 => 16,
        BitboardType::Bitboard64 => 8,
    };
    let mut bb_string = String::new().to_owned();
    bb_string.push_str("  1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 \n");
    for i in 0..x{
        bb_string.push_str(&(((i+1)%10).to_string()+" "));
        for j in 0..x{
            let index = to_pos(i,j,T::BBTYPE);
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

pub fn display_bitboard_with_board_desc<T:Bitboard>(bitboard:&T,str:&str)->String{
    let bitboard_str = display_bitboard(bitboard);
    let mut new_str = str.to_string();
    new_str.push_str("\n");
    new_str.push_str(&bitboard_str);
    println!("{}",new_str);
    new_str

}