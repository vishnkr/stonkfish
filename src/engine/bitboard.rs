use numext_fixed_uint::U256;

use super::position::Dimensions;

pub type Bitboard = U256;

pub fn to_pos(x:u8,y:u8) -> usize{
    //println!("x {} y {}",x,y);
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

pub fn display_bitboard(bitboard:&Bitboard)->String{
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

pub fn display_bitboard_with_board_desc(bitboard:&Bitboard,str:&str)->String{
    let bitboard_str = display_bitboard(bitboard);
    let mut new_str = str.to_string();
    new_str.push_str("\n");
    new_str.push_str(&bitboard_str);
    println!("{}",new_str);
    new_str

}

pub struct SizeDependentBitboards{
    pub full_bitboard:Bitboard,
    pub row_boundary:Bitboard,
    pub col_boundary:Bitboard
}

impl SizeDependentBitboards{
    pub fn new(dimensions:&Dimensions)->Self{
        let mut full_bitboard = Bitboard::zero();
        let mut row_boundary = Bitboard::zero();
        let mut col_boundary = Bitboard::zero();
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