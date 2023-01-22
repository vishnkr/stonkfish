    use crate::engine::{
    position::*,
    evaluation::{Evaluator},
    move_generation::{MoveGenerator,moves::*}
};
use self::transposition::*;

pub mod transposition;

pub struct Search{
    pub transposition_table: TranspositionTable,
    pub node_count: u32,
}

const TEST_SIZE:usize = 1000000;

impl Search{
    pub fn new()->Self{
        Search{
            transposition_table: TranspositionTable::new(TEST_SIZE),
            node_count: 0
        }
    }
    pub fn alphabeta(&mut self,position:&mut Position,move_generator:&mut MoveGenerator, evaluator: &mut Evaluator,depth:u8,mut alpha:isize,mut beta:isize)->isize{
        if depth==0{
            return evaluator.evaluate(position) //(quiescense here later)
        }

        let mut cur_best_move: Move = Move::new(0,0,MType::None,None);
        let best_score = 0;
        let old_alpha = alpha;

        self.node_count+=1;

        let in_check = move_generator.is_king_under_check(position);
        let mut legal_move_count = 0;

        let move_list = move_generator.generate_pseudolegal_moves(position);
        for (i,mv) in move_list.enumerate(){

            if !move_generator.is_legal_move(position,&mv){
                continue
            }
            
            position.make_move(&mv);
            position.switch_turn();
            let mut score = -self.alphabeta(position,move_generator,evaluator,depth-1,-beta,-alpha);
            position.switch_turn();
            position.unmake_move(&mv);
            legal_move_count+=1;
            if score > alpha {
                alpha = score;
            }
            if score >= beta { return beta } // fail hard beta cutoff
        }

        if legal_move_count == 0{
            if in_check{
                return isize::MIN;
            }
            return 0
        }
        // better move found
        if old_alpha!=alpha{
            let best_move = cur_best_move;
            self.transposition_table.insert(position.get_zobrist_hash(), TableEntry { key: position.get_zobrist_hash(), node_type: NodeType::Exact, score: best_score, depth: depth, best_move: best_move })
        }
        alpha
    }

}