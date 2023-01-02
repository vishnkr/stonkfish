
pub mod bitboard;
pub mod evaluation;
pub mod move_generation;
pub mod position;
pub mod search;
use crate::engine::position::{Position};
use crate::engine::search::*;
use self::move_generation::moves::Move;


pub struct Engine{
    move_generator: move_generation::MoveGenerator,
    evaluator: evaluation::Evaluator,
    position: Position,
    search: search::Search
}

impl Engine{
    pub fn new(fen:String)->Engine{
        let position : Position = Position::new(fen);
        Engine{
            move_generator: move_generation::MoveGenerator::new(position.dimensions.clone()),
            evaluator: evaluation::Evaluator::new(),
            position: position,
            search: search::Search::new(),
        }
    }

    pub fn get_best_move_depth(&mut self,depth: u8)->Option<Move>{
        for di in 1..depth+1{
            self.search.alphabeta(
                &mut self.position, 
                &mut self.move_generator,
                &mut self.evaluator, 
                depth, 
                isize::MIN, 
                isize::MAX
            );
        }
        let best_move: Option<Move> = match self.search.transposition_table.get_entry(self.position.get_zobrist_hash()){
            Some(x)=> Some(x.best_move.to_owned()),
            None=> None
        };
        best_move
        
    }
}
