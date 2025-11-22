mod piece_sq;

pub use crate::position::{Position,piece_collection::PieceCollection};
pub use crate::move_generation::att_table::SlideDirection;
use crate::evaluation::piece_sq::PieceSquareTables;

//centipawn scores
const KING_CP_SCORE:isize = 20000;
const PAWN_CP_SCORE:isize = 100;
const KNIGHT_CP_SCORE:isize = 320;
const BISHOP_CP_SCORE:isize = 330;
const ROOK_CP_SCORE:isize = 500;
const QUEEN_CP_SCORE:isize = 900;
const DIAGONAL_SCORE:isize = 90;

pub struct Evaluator{
    piece_sq_table: PieceSquareTables
}

impl Evaluator{
    pub fn new(position:& Position)->Self{
        Evaluator{piece_sq_table: PieceSquareTables::new(position)}
    }
  
    pub fn evaluate(&mut self,position:&Position)->isize{
        //TODO: add positional eval score
       self.get_material_eval_score(position)
    }
    
    pub fn calc_material_score(&mut self,piece_set: &PieceCollection)->isize{
        let material_score = 0;
        /*piece_set.king.bitboard.count_ones() as isize * KING_CP_SCORE + 
        piece_set.pawn.bitboard.count_ones() as isize * PAWN_CP_SCORE + 
        piece_set.queen.bitboard.count_ones() as isize * QUEEN_CP_SCORE + 
        piece_set.rook.bitboard.count_ones() as isize * ROOK_CP_SCORE + 
        piece_set.knight.bitboard.count_ones() as isize * KNIGHT_CP_SCORE + 
        piece_set.bishop.bitboard.count_ones() as isize * BISHOP_CP_SCORE;*/
        material_score
    }

    pub fn get_material_eval_score(&mut self,position:&Position)->isize{
        let mut total_score = 0;
        for pieces in &position.piece_collections{
            println!("ts {}",self.calc_material_score(&pieces));
            if position.turn ==pieces.player { total_score += self.calc_material_score(&pieces);}
            else { total_score -= self.calc_material_score(&pieces);}
        }
        total_score
    }

    pub fn calc_custom_material_value(&mut self,piece_repr:char,
        jump_offsets:Vec<Vec<i8>>,
        slide_dirs: &[SlideDirection],
    )->isize{
        let material_score = 0;
        material_score
    }
}

/*
8x8
0,  0,  0,  0,  0,  0,  0,  0,
50, 50, 50, 50, 50, 50, 50, 50,
10, 10, 20, 30, 30, 20, 10, 10,
 5,  5, 10, 25, 25, 10,  5,  5,
 0,  0,  0, 20, 20,  0,  0,  0,
 5, -5,-10,  0,  0,-10, -5,  5,
 5, 10, 10,-20,-20, 10, 10,  5,
 0,  0,  0,  0,  0,  0,  0,  0

 9x9
 0....

 5,+5,+0,-30,+0,+30,+0,-5
 0....
*/