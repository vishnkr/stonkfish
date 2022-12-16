
use std::{fmt::{self, write}};
use crate::engine::{bitboard::*, position::PieceType};

#[derive(PartialEq)]
pub struct Move(u32);

#[derive(PartialEq)]
pub enum MType{
    Quiet,
    Capture,
    Promote,
    // refers to castling on the right since custom positions could have king placed wherever
    KingsideCastle,
    // refers to castling on the left
    QueensideCastle,
    EnPassant
}

impl fmt::Display for MType{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result{
        fmt::Debug::fmt(self,f)
    }
}

impl fmt::Debug for MType{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result{
        write!(f, "{:?}",self)
    }
}

//encoding based on chessprogramming wiki - https://www.chessprogramming.org/Encoding_Moves

impl Move{
    pub fn new(src:u8,dest:u8,mtype:MType)->Move{
        Move(
            (((0 | (src as u32))<< 16u32) | (dest as u32) << 8u32)| 
            match mtype {
                MType::Quiet => 0,
                MType::Capture => 1u32,
                MType::Promote => 2u32,
                MType::KingsideCastle => 3u32,
                MType::QueensideCastle => 4u32,
                MType::EnPassant => 5u32
            }).into()
    }
    pub fn make_move(&self){
        println!("{}",format!("{:b}", self));
    }
    pub fn parse_from(&self)->u8{
        ((self.0 >>16) & 0xFF) as u8
    }
    pub fn parse_to(&self)->u8{
        ((self.0 >>8) & 0xFF) as u8
    }
    pub fn parse_mtype(&self)->Option<MType>{
        let mtype = (self.0 & 0xFF) as u32;
        match mtype {
            0 => Some(MType::Quiet),
            1u32 => Some(MType::Capture),
            2u32 => Some(MType::Promote),
            3u32 => Some(MType::KingsideCastle),
            4u32 => Some(MType::QueensideCastle),
            5u32 => Some(MType::EnPassant),
            _ => None
        }
    }
}

// contains bitboard with all possible moves for a piece which can be iterated to get a list of moves
pub struct MoveMask{
    pub bitboard: Bitboard,
    pub src: u8,
    pub piece_type: PieceType,
    pub opponent:Bitboard
}

impl Iterator for MoveMask{
    type Item = Move;

    fn next(&mut self)->Option<Self::Item>{
        let dest_pos = match self.bitboard.lowest_one(){
            Some(x) => x,
            None=> return None
        };
        self.bitboard.set_bit(dest_pos,false);
;        let mut mtype = MType::Quiet;
        if self.opponent.bit(dest_pos).unwrap(){
            mtype = MType::Capture;
        }
        Some(Move::new(self.src, dest_pos as u8, mtype))
    }
}

impl fmt::Binary for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"{:#032b}",self.0)
    }
}

impl fmt::Debug for Move{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result{
        let mtype = self.parse_mtype().unwrap().to_string();
        let dest_pos = self.parse_to();
        let src_pos = self.parse_from();
        write!(f,"Move {} from {} to {}",mtype,src_pos,dest_pos)
    }
}

impl fmt::Display for Move{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result{
        let mtype = self.parse_mtype().unwrap().to_string();
        let dest_pos = self.parse_to();
        let src_pos = self.parse_from();
        let src_pos = self.parse_from();
        let src_coords = (to_row(src_pos as u8),to_col(src_pos as u8));
        let dest_coords = (to_row(dest_pos as u8),to_col(dest_pos as u8));
        write!(f,"Move {} from {} (row-{}, col-{}) to {} (row-{}, col-{})",mtype,src_pos,src_coords.0,src_coords.1,dest_pos,dest_coords.0,dest_coords.1)
    }
}