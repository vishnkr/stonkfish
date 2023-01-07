
mod engine;
use engine::{
    move_generation::{MoveGenerator,moves::Move},
    evaluation::Evaluator,
    position::Position,
    search::Search
    };

pub struct Engine{
    move_generator: MoveGenerator,
    evaluator: Evaluator,
    position: Position,
    search: Search
}

impl Engine{
    pub fn new(fen:String)->Engine{
        let position : Position = Position::new(fen);
        Engine{
            move_generator: MoveGenerator::new(position.dimensions.clone()),
            evaluator: Evaluator::new(),
            position: position,
            search: Search::new(),
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
