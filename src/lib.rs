
pub mod engine;
use engine::{
    move_generation::{MoveGenerator,moves::*, generate_legal_moves},
    evaluation::{Evaluator},
    position::{Position},
    transposition::*,
    stats::*
    };

pub struct Engine{
    pub move_generator: MoveGenerator,
    pub evaluator: Evaluator,
    pub position: Position,
    transposition_table: TranspositionTable,
    pub stats: Stats
}


const TEST_TT_SIZE:usize = 254288000;

impl Engine{
    pub fn new(fen:String)->Engine{
        let position : Position = Position::new(fen);
        Engine{
            move_generator: MoveGenerator::new(position.dimensions.clone(),position.get_jump_offets()),
            evaluator: Evaluator {  },//Evaluator::new(&position),
            position: position,
            transposition_table: TranspositionTable::new(TEST_TT_SIZE),
            stats: Stats::new()
        }
    }

    pub fn alphabeta(&mut self,depth:u8,mut alpha:isize, beta:isize)->isize{
        if depth==0{
            return 0 // self.evaluator.evaluate(&mut self.position) //(quiescense here later)
        }

        let cur_best_move: Move = Move::encode_move(0,0,MType::None,None);
        let best_score = 0;
        let old_alpha = alpha;

        self.stats.search_stats.node_count+=1;
        let turn = self.position.turn;
        let in_check = self.move_generator.is_king_under_check(&mut self.position,turn);
        let mut legal_move_count = 0;

        let move_list = self.move_generator.generate_pseudolegal_moves(&mut self.position);
        for (_,mv) in move_list.enumerate(){

            if !self.move_generator.is_legal_move(&mut self.position,&mv){
                continue
            }
            
            self.position.make_move(&mv);
            let score = -self.alphabeta(depth-1,-beta,-alpha);
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
            self.transposition_table.insert(TableEntry { 
                key: self.position.get_zobrist_hash(), 
                node_type: NodeType::Exact, 
                score: best_score, 
                depth: depth, 
                best_move: best_move 
            });
        }
        alpha
    }

    pub fn get_best_move_depth(&mut self,depth: u8)->Option<Move>{
        for _ in 1..depth+1{
            self.alphabeta(
                depth, 
                isize::MIN, 
                isize::MAX
            );
        }
        let best_move: Option<Move> = match self.transposition_table.lookup(self.position.get_zobrist_hash()){
            Some(x)=> Some(x.best_move.to_owned()),
            None=> None
        };
        best_move
        
    }

    pub fn perft(&mut self,depth: u32,current_depth:u32)->u64{
        if depth==0{
            return 1;
        }
        let mut nodes = 0;
        let moves = generate_legal_moves(&self.move_generator, &mut self.position);//time_it!("Legal move generation",{ generate_legal_moves(&self.move_generator, &mut self.position) });
        self.stats.move_gen_stats.moves_generated += moves.len();
        //println!("{}({})","---|".repeat(current_depth as usize),current_depth);
        for mv in moves{
            self.stats.move_gen_stats.update_move_type_count(&mv);
            /*println!("{} - depth {}",mv.to_algebraic_notation(
                self.move_generator.dimensions.height,
                self.position.turn,
                &self.position.piece_collections[self.position.turn as usize]
            ),depth);*/
            self.position.make_move(&mv);
            let count = self.perft(depth-1,current_depth+1);
            *self.stats.move_gen_stats.moves_per_depth.entry(current_depth).or_insert(0) += 1;
            nodes+=count;
            self.position.unmake_move(&mv);
        }
        //println!("{}({})","---|".repeat(current_depth as usize),current_depth);
        return nodes
    }

    pub fn perft_divide(&mut self, depth:u32)->u64{
        let mut total = 0;
        if depth == 0{
            return 1;
        }
        let moves= generate_legal_moves(&self.move_generator, &mut self.position);
        for mv in moves{
            self.position.make_move(&mv);
            let count = self.perft(depth-1,1);
            self.position.unmake_move(&mv);
            total+=count;
            println!("{} - {}",mv.to_algebraic_notation(
                self.move_generator.dimensions.height,
                self.position.turn,
                &self.position.piece_collections[self.position.turn as usize]
            ),count);
        }
        println!("Found {} moves",total);
        total
    }
}
