use numext_fixed_uint::U256;

pub type Bitboard = U256;

pub fn to_pos(x:u8,y:u8) -> usize{
    ((x*16)+y).into()
}

pub fn to_col(pos:u8)->u8{
    pos%16
}

pub fn to_row(pos:u8)->u8{
    pos/16
}

pub fn to_string(bitboard:&Bitboard)->String{
    let mut bb_string = String::new().to_owned();
    
    for i in 0..16{
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
    println!("a16-h16");
    println!("{}",bb_string);
    println!("a1-h1\n");
    bb_string
}
