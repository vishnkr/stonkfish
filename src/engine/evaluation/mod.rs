mod piece_sq;

use std::collections::HashMap;
use arrayvec::ArrayVec;
pub use crate::engine::position::{Position,piece::PieceSet,piece::PieceType};
pub use crate::engine::move_generation::att_table::SlideDirection;

use self::piece_sq::PieceSquareTables;
//centipawn scores
const KING_CP_SCORE:isize = 10000;
pub const PAWN_CP_SCORE:isize = 100;
const KNIGHT_CP_SCORE:isize = 300;
const BISHOP_CP_SCORE:isize = 350;
const ROOK_CP_SCORE:isize = 500;
const QUEEN_CP_SCORE:isize = 900;
const DIAGONAL_SCORE:isize = 90;

pub struct Evaluator{
    piece_sq_table: PieceSquareTables
}

impl Evaluator{
    pub fn new(position: &Position)->Self{
        Evaluator{piece_sq_table: PieceSquareTables::new(position)}
    }

    pub fn evaluate(&mut self,position:&mut Position)->isize{
        //TODO: add positional eval score
        self.get_material_eval_score(position)
    }

    pub fn calc_material_score(&mut self,piece_set: &PieceSet)->isize{
        let material_score = piece_set.king.bitboard.count_ones() as isize * KING_CP_SCORE + 
        piece_set.pawn.bitboard.count_ones() as isize * PAWN_CP_SCORE + 
        piece_set.queen.bitboard.count_ones() as isize * QUEEN_CP_SCORE + 
        piece_set.rook.bitboard.count_ones() as isize * ROOK_CP_SCORE + 
        piece_set.knight.bitboard.count_ones() as isize * KNIGHT_CP_SCORE + 
        piece_set.bishop.bitboard.count_ones() as isize * BISHOP_CP_SCORE;
        material_score
    }

    pub fn get_material_eval_score(&mut self,position:&mut Position)->isize{
        let mut total_score = 0;
        for piece_set in position.pieces.iter(){
            println!("ts {}",self.calc_material_score(piece_set));
            if position.turn as u8 ==piece_set.player { total_score += self.calc_material_score(piece_set);}
            else { total_score -= self.calc_material_score(piece_set);}
        }
        total_score
    }

    pub fn calc_custom_material_value(&mut self,piece_repr:char,
        jump_offsets:ArrayVec<ArrayVec<i8,0>,0>,
        slide_dirs: &[SlideDirection],
    )->isize{
        let material_score = 0;
        material_score
    }
}