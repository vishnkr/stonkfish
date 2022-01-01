use crate::evaluator::*;
mod game;
mod position;
mod evaluator;
use crate::position::Position;
pub const WHITE:u8 =0;
pub const BLACK:u8=1;

pub struct Engine{
    pub position: Position,
    pub evaluator: Evaluator
}

impl Engine{
    pub fn new()->Engine{
        Engine{position: Position::new(), evaluator: Evaluator::new()}
    }


}
