
mod engine;
use crate::engine::{Engine};
use crate::engine::moves::{Move,MType};

fn main() {
    let engine: &Engine = &Engine::new("10/5p1k1/9/p2p1P3/5q3/P1PbN2p1/7P1/2Q3K2/10/10 w - - 1 44".to_string());
    let mv:&Move = &Move::new(4,255,MType::Capture);
    mv.make_move();
    println!("{} {} {}",mv.parse_from(),mv.parse_to(),mv.parse_mtype());
}
