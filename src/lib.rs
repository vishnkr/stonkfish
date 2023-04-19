
pub mod engine;
use engine::{
    move_generation::{MoveGenerator,moves::*, generate_legal_moves},
    evaluation::{Evaluator, PieceType},
    position::{Position,Color},
    transposition::*,
    stats::*, bitboard::sq_to_notation
    };

use crate::engine::move_generation::moves;

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
            move_generator: MoveGenerator::new(position.dimensions.clone()),
            evaluator: Evaluator::new(&position),
            position: position,
            transposition_table: TranspositionTable::new(TEST_TT_SIZE),
            stats: Stats::new()
        }
    }

    pub fn alphabeta(&mut self,depth:u8,mut alpha:isize,mut beta:isize)->isize{
        if depth==0{
            return self.evaluator.evaluate(&mut self.position) //(quiescense here later)
        }

        let mut cur_best_move: Move = Move::encode_move(0,0,MType::None,None);
        let best_score = 0;
        let old_alpha = alpha;

        self.stats.search_stats.node_count+=1;

        let in_check = self.move_generator.is_king_under_check(&mut self.position);
        let mut legal_move_count = 0;

        let move_list = self.move_generator.generate_pseudolegal_moves(&mut self.position);
        for (i,mv) in move_list.enumerate(){

            if !self.move_generator.is_legal_move(&mut self.position,&mv){
                continue
            }
            
            self.position.make_move(&mv);
            let mut score = -self.alphabeta(depth-1,-beta,-alpha);
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
        for di in 1..depth+1{
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
        if (depth==0){
            return 1;
        }
        let mut nodes = 0;
        let mut moves_per_depth = vec![0; depth as usize + 1];
        let moves = time_it!("Legal move generation",{ generate_legal_moves(&self.move_generator, &mut self.position) });
        self.stats.move_gen_stats.moves_generated += moves.len();
        println!("{}({})","---|".repeat(current_depth as usize),current_depth);
        for mv in moves{
            self.stats.move_gen_stats.update_move_type_count(&mv);
            println!("{} - depth {}",self.to_algebraic_notation(&mv, self.position.turn),depth);
            self.position.make_move(&mv);
            let count = self.perft(depth-1,current_depth+1);
            *self.stats.move_gen_stats.moves_per_depth.entry(current_depth).or_insert(0) += 1;
            nodes+=count;
            self.position.unmake_move(&mv);
        }
        println!("{}({})","---|".repeat(current_depth as usize),current_depth);
        return nodes
    }
    pub fn to_algebraic_notation(&self,mv:&Move,color:Color)->String{
        let (src,dest,mtype) = (mv.get_src_square(), mv.get_dest_square(),mv.get_mtype().unwrap());
        let piece = self.position.pieces[color as usize].get_piece_from_sq(src.into()).unwrap();
        let rows = self.move_generator.dimensions.height;
        if piece.piece_type == PieceType::Pawn{
            return format!("{}{}",
            sq_to_notation(src,rows)
            ,sq_to_notation(dest,rows));
        } else {
            let mut piece_type_str = piece.piece_repr.to_string();
            if color == Color::WHITE{ piece_type_str = piece_type_str.to_ascii_uppercase()}
            match mtype{
                MType::Quiet=>{ format!("{}{}{}",piece_type_str,sq_to_notation(src,rows),sq_to_notation(dest,rows)) },
                MType::KingsideCastle => "O-O".to_string(),
                MType::QueensideCastle => "O-O-O".to_string(),
                MType::Capture=>{format!("{}{}x{}",piece_type_str,sq_to_notation(src,rows),sq_to_notation(dest,rows))},
                //capture promos will be formated as promo for now
                MType::Promote =>{format!("{}{}={}",piece_type_str,sq_to_notation(src,rows),sq_to_notation(dest,rows))},
                _=> "".to_string()
            }
        }

    }
}
