
pub mod bitboard;
pub mod evaluator;
pub mod move_generator;
pub mod position;
use position::{Position};

pub struct Engine{
    move_generator: move_generator::MoveGenerator,
    evaluator: evaluator::Evaluator,
    position: Position
}

impl Engine{
    pub fn new(fen:String)->Engine{
        let position : Position = Position::new(fen);
        Engine{
            move_generator: move_generator::MoveGenerator::new(position.dimensions.clone()),
            evaluator: evaluator::Evaluator::new(),
            position: position,
        }
    }

    pub fn get_best_move_depth(depth: usize){
        //perform move-gen, evaluate and quiescence search somewhere
    }
}
