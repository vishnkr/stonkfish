use std::{collections::HashMap};

use crate::engine::bitboard::Bitboard;

use super::{Color, piece::{Piece, PieceRepr}, Dimensions};


#[derive(Debug,PartialEq)]
pub struct PieceCollection{
    pub player:Color,
    // contains all kinds of pieces (regular+custom)
    pub pieces:HashMap<PieceRepr,Piece>,
    pub occupied: Bitboard,
}

impl PieceCollection{
    pub fn new(color:Color,dimensions:&Dimensions)->Self{
        let standard_pieces = vec!['k','q','r','b','n','p'];
        let mut pieces = HashMap::new();
        for piece_repr in standard_pieces{
            pieces.insert(piece_repr,Piece::new_piece(color, piece_repr, dimensions));
        }
        PieceCollection{
            player:color,
            pieces,
            occupied: Bitboard::zero(),
        }
    }

    pub fn get_king_mut(&self)->&Piece{
        self.pieces.get(&'k').unwrap()
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

}
