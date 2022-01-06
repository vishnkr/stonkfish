use crate::engine::moves::{Move,MType};
use crate::engine::position;
use std::vec::Vec;
pub struct MoveGenerator{
    id: usize
}


impl MoveGenerator{
    pub fn new()->MoveGenerator{
        MoveGenerator{id:0}
    }

    pub fn generate_moves(&self,cur_position:&mut position::Position)->Vec<Move>{
        let moves = Vec::new();
        let piece_set: &position::PieceSet = &cur_position.pieces[cur_position.turn as usize];
        moves
    }
}