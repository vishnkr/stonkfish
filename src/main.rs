
mod utils;
mod position;
use utils::bitboard::{Bitboard,to_string};
use position::{Position};
const WHITE:u8 =0;
const BLACK:u8=1;

fn main() {
    //let bb = Bitboard::from(2147483646);//String::from("8/5p1k/8/p2p1P2/5q2/P1PbN2p/7P/2Q3K1 w - - 1 44");
    let bb_string = String::from("10/5p1k1/9/p2p1P3/5q3/P1PbN2p1/7P1/2Q3K2/10/10 w - - 1 44");//to_string(&bb);
    let pos:Position = Position::load_from_fen(bb_string);
    println!("{} {}",pos.dimensions.width,pos.dimensions.height)
}
