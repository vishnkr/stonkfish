use crate::engine::{
    position::*,
    evaluation::{Evaluator},
    move_generation::{MoveGenerator,moves::*}
};
use self::transposition::*;

pub mod transposition;

pub struct Search{
    transposition_table: TranspositionTable,
}

const TEST_SIZE:usize = 1000000;

impl Search{
    pub fn new()->Self{
        Search{
            transposition_table: TranspositionTable::new(TEST_SIZE)
        }
    }
    pub fn minimax(&mut self,position:&mut Position,move_generator:&mut MoveGenerator, evaluator: &mut Evaluator,depth:u8,alpha:isize,beta:isize)->isize{
        let mut node_count = 0;
        if depth==0{
            return evaluator.evaluate(position) //(quiescense here later)
        }

        let mut best_move: Move = Move::new(0,0,MType::None);
        let score = 0;
        node_count+=1;
        let move_list = move_generator.generate_pseudolegal_moves(position);
        for (i,mv) in move_list.enumerate(){

            if !move_generator.is_legal_move(position,&mv){
                continue
            }
            position.make_move(&mv);
            let mut eval = self.minimax(position,move_generator,evaluator,depth-1,alpha,beta);

        }
        score
    }
}