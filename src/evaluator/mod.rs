use std::collections::HashMap;
use std::collections::hash_map::RandomState;

pub use crate::position::{Position,PieceSet,PieceType};
//centipawn scores
const KING_CP_SCORE:usize = 10000;
const PAWN_CP_SCORE:usize = 100;
const KNIGHT_CP_SCORE:usize = 300;
const BISHOP_CP_SCORE:usize = 350;
const ROOK_CP_SCORE:usize = 500;
const QUEEN_CP_SCORE:usize = 900;


pub struct Evaluator{
    piece_sq_table: HashMap<PieceType,Vec<usize>,RandomState>
}

impl Evaluator{
    pub fn new()->Self{
        Evaluator{piece_sq_table: HashMap::with_hasher(RandomState::new())}
    }

    pub fn calc_material_score(&mut self,piece_set: &PieceSet)->usize{
        let material_score = piece_set.king.bitboard.count_ones() as usize * KING_CP_SCORE + 
        piece_set.pawn.bitboard.count_ones() as usize * PAWN_CP_SCORE + 
        piece_set.queen.bitboard.count_ones() as usize * QUEEN_CP_SCORE + 
        piece_set.rook.bitboard.count_ones() as usize * ROOK_CP_SCORE + 
        piece_set.knight.bitboard.count_ones() as usize * KNIGHT_CP_SCORE + 
        piece_set.bishop.bitboard.count_ones() as usize * BISHOP_CP_SCORE;
        material_score
    }

    pub fn perform_evaluation(&mut self,position:&mut Position)->usize{
        let mut total_score = 0;
        for piece_set in position.pieces.iter(){
            if position.turn==piece_set.player { total_score += self.calc_material_score(piece_set);}
            else { total_score -= self.calc_material_score(piece_set);}
        }
        total_score
    }
}