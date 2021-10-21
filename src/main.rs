
mod utils;

use utils::bitboard::{Bitboard,to_string};

fn main() {
    let bb = Bitboard::from(2147483646);//String::from("8/5p1k/8/p2p1P2/5q2/P1PbN2p/7P/2Q3K1 w - - 1 44");
    to_string(&bb);
}
