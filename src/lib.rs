
mod engine;
use engine::{
    move_generation::{MoveGenerator,moves::*},
    evaluation::Evaluator,
    position::Position,
    transposition::*,
    };

pub struct Engine{
    move_generator: MoveGenerator,
    evaluator: Evaluator,
    position: Position,
    transposition_table: TranspositionTable,
    search_stats: SearchStats
}

pub struct SearchStats{
    pub node_count: u32
}
const TEST_SIZE:usize = 1000000;

impl Engine{
    pub fn new(fen:String)->Engine{
        let position : Position = Position::new(fen);
        Engine{
            move_generator: MoveGenerator::new(position.dimensions.clone()),
            evaluator: Evaluator::new(),
            position: position,
            transposition_table: TranspositionTable::new(TEST_SIZE),
            search_stats: SearchStats { node_count: 0 }
        }
    }

    pub fn alphabeta(&mut self,depth:u8,mut alpha:isize,mut beta:isize)->isize{
        if depth==0{
            return self.evaluator.evaluate(&mut self.position) //(quiescense here later)
        }

        let mut cur_best_move: Move = Move::new(0,0,MType::None,None);
        let best_score = 0;
        let old_alpha = alpha;

        self.search_stats.node_count+=1;

        let in_check = self.move_generator.is_king_under_check(&mut self.position);
        let mut legal_move_count = 0;

        let move_list = self.move_generator.generate_pseudolegal_moves(&mut self.position);
        for (i,mv) in move_list.enumerate(){

            if !self.move_generator.is_legal_move(&mut self.position,&mv){
                continue
            }
            
            self.position.make_move(&mv);
            self.position.switch_turn();
            let mut score = -self.alphabeta(depth-1,-beta,-alpha);
            self.position.switch_turn();
            self.position.unmake_move(&mv);
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
            self.transposition_table.insert(self.position.get_zobrist_hash(), TableEntry { key: self.position.get_zobrist_hash(), node_type: NodeType::Exact, score: best_score, depth: depth, best_move: best_move })
        }
        alpha
    }

    pub fn get_best_move_depth(&mut self,depth: u8)->Option<Move>{
        for di in 1..depth+1{
            self.alphabeta(
                depth, 
                isize::MIN, 
                isize::MAX
            );
        }
        let best_move: Option<Move> = match self.transposition_table.get_entry(self.position.get_zobrist_hash()){
            Some(x)=> Some(x.best_move.to_owned()),
            None=> None
        };
        best_move
        
    }
}
