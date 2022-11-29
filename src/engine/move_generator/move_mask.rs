use crate::engine::position::*;
use crate::engine::bitboard::{Bitboard,to_pos};
// contains bitboard with all possible moves for a piece which can be iterated to get a list of moves
pub struct MoveMask{
    pub bitboard: Bitboard,
    pub src: usize,
    pub piece_type: PieceType,
    //opponent_bb : Bitboard,
}