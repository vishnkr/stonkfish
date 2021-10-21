use primitive_types::U256;
pub type Bitboard = U256;

pub fn to_pos(x:u8,y:u8) -> usize{
    ((x*16)+y).into()
}

pub fn to_string(bitboard:&Bitboard){
    println!("reached");
    let mut bb_string = String::new().to_owned();
    for i in 0..16{
        for j in 0..16{
            let index = to_pos(i,j);
            if bitboard.bit(index){
                bb_string.push_str("1 ");
            } else{
                bb_string.push_str("- " );
            }
        }
        bb_string.push_str("\n");
    }
    println!("{}",bitboard);
    println!("{}",bb_string);
}
