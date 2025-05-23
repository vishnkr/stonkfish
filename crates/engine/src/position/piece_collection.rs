use std::{collections::HashMap};

use crate::bitboard::Bitboard;

use super::{Color, piece::{Piece, PieceRepr}};


#[derive(Debug,PartialEq)]
pub struct PieceCollection{
    pub player:Color,
    // contains all kinds of pieces (regular+custom)
    pub pieces:HashMap<PieceRepr,Piece>,
    pub occupied: Bitboard,
}

impl PieceCollection{
    pub fn new(color:Color)->Self{
        let pieces = HashMap::new();
        PieceCollection{
            player:color,
            pieces,
            occupied: Bitboard::zero(),
        }
    }

    pub fn get_king(&self)->&Piece{
        let king_repr: char = match self.player{
            Color::BLACK => 'k',
            Color::WHITE => 'K',
        };
        self.pieces.get(&king_repr).unwrap()
    }

    pub fn get_mut_piece_from_sq(&mut self,loc:usize)->Option<&mut Piece>{
        for (_,piece) in self.pieces.iter_mut(){
            if piece.bitboard.bit(loc).unwrap(){
                return Some(piece);
            }
        }
        None
    }

    pub fn get_piece_from_sq(&self,loc:usize)->Option<&Piece>{
        for (_,piece) in self.pieces.iter(){
            if piece.bitboard.bit(loc).unwrap(){
                return Some(piece);
            }
        }
        None
    }

    pub fn has_piece_at(&self,loc:usize)->bool{
        return self.occupied.bit(loc).unwrap();
    }

}
