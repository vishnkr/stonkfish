
pub mod bitboard;
pub mod evaluation;
pub mod move_generation;
pub mod position;
use position::{Position};

pub struct Engine{
    move_generator: move_generation::MoveGenerator,
    evaluator: evaluation::Evaluator,
    position: Position
}

impl Engine{
    pub fn new(fen:String)->Engine{
        let position : Position = Position::new(fen);
        Engine{
            move_generator: move_generation::MoveGenerator::new(position.dimensions.clone()),
            evaluator: evaluation::Evaluator::new(),
            position: position,
        }
    }

    pub fn get_best_move_depth(depth: usize){
        //perform move-gen, evaluate and quiescence search somewhere
    }
}
