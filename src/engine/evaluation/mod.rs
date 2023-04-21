mod piece_sq;

pub use crate::engine::position::{Position};
pub use crate::engine::move_generation::att_table::SlideDirection;


//centipawn scores
const _KING_CP_SCORE:isize = 10000;
pub const PAWN_CP_SCORE:isize = 100;
const _KNIGHT_CP_SCORE:isize = 300;
const _BISHOP_CP_SCORE:isize = 350;
const _ROOK_CP_SCORE:isize = 500;
const _QUEEN_CP_SCORE:isize = 900;
const _DIAGONAL_SCORE:isize = 90;

pub struct Evaluator{
    //piece_sq_table: PieceSquareTables
}

impl Evaluator{
    pub fn new()->Self{
        //Evaluator{piece_sq_table: PieceSquareTables::new(position)}
        Evaluator { }//piece_sq_table: () }
    }
    pub fn evaluate()->usize{ 0 }
    //pub fn evaluate(&mut self,position:&mut Position)->isize{
        //TODO: add positional eval score
       // self.get_material_eval_score(position)
    //}
    /* 
    pub fn calc_material_score(&mut self,piece_set: &PieceCollection)->isize{
        let material_score = 0;piece_set.king.bitboard.count_ones() as isize * KING_CP_SCORE + 
        piece_set.pawn.bitboard.count_ones() as isize * PAWN_CP_SCORE + 
        piece_set.queen.bitboard.count_ones() as isize * QUEEN_CP_SCORE + 
        piece_set.rook.bitboard.count_ones() as isize * ROOK_CP_SCORE + 
        piece_set.knight.bitboard.count_ones() as isize * KNIGHT_CP_SCORE + 
        piece_set.bishop.bitboard.count_ones() as isize * BISHOP_CP_SCORE;
        material_score
    }

    pub fn get_material_eval_score(&mut self,position:&mut Position)->isize{
        let mut total_score = 0;
        for pieces in position.piece_collections{
            println!("ts {}",self.calc_material_score(&pieces));
            if position.turn ==pieces.player { total_score += self.calc_material_score(&pieces);}
            else { total_score -= self.calc_material_score(&pieces);}
        }
        total_score
    }

    pub fn calc_custom_material_value(&mut self,piece_repr:char,
        jump_offsets:ArrayVec<ArrayVec<i8,0>,0>,
        slide_dirs: &[SlideDirection],
    )->isize{
        let material_score = 0;
        material_score
    }*/
}
