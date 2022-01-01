
mod game;
mod position;
use crate::game::evaluator;
use crate::position::{Position};
pub const WHITE:u8 =0;
pub const BLACK:u8=1;

fn main() {
    let pos:&mut Position = &mut Position::load_from_fen("10/5p1k1/9/p2p1P3/5q3/P1PbN2p1/7P1/2Q3K2/10/10 w - - 1 44".to_string());
    // let nmove = Move::new(13,2,MType::Regular);Move,MType,e
    let evaluator:&mut evaluator::Evaluator = &mut evaluator::Evaluator::new();
    println!("eval score {}",evaluator.perform_evaluation(pos) as f32 /evaluator::PAWN_CP_SCORE as f32);
}
