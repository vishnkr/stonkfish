use std::fmt;

use crate::engine::bitboard::{to_row,to_col};

#[derive(PartialEq)]
pub struct Move(u32);

#[derive(PartialEq)]
pub enum MType{
    Quiet,
    Capture,
    Promote,
    KingSideCastle,
    QueenSideCastle,
    DoublePawnPush,
    EnPassant
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
                MType::KingSideCastle => 3u32,
                MType::QueenSideCastle => 4u32,
                MType::DoublePawnPush => 5u32,
                MType::EnPassant => 6u32
            }).into()
    }
    pub fn make_move(&self){
        println!("{}",format!("{:b}", self));
    }
    pub fn parse_from(&self)->usize{
        ((self.0 >>16) & 0xFF) as usize
    }
    pub fn parse_to(&self)->usize{
        ((self.0 >>8) & 0xFF) as usize
    }
    pub fn parse_mtype(&self)->usize{
        (self.0 & 0xFF) as usize
    }
}

impl fmt::Binary for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"{:#032b}",self.0)
    }
}

impl fmt::Debug for Move{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result{
        let mtype = match self.parse_mtype() {
            0=> "Quiet",
            1=> "Capture",
            2=> "Promote",
            3=> "KingSideCastle",
            4=> "QueenSideCastle",
            5=> "DoublePawnPush",
            6=> "En Passant",
            _ => "Invalid value"
        };
        let dest_pos = self.parse_to();
        let src_pos = self.parse_from();
        write!(f,"Move {} from {} to {}",mtype,src_pos,dest_pos)
    }
}

impl fmt::Display for Move{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result{
        let mtype = match self.parse_mtype() {
            0=> "Quiet",
            1=> "Capture",
            2=> "Promote",
            3=> "KingSideCastle",
            4=> "QueenSideCastle",
            5=> "DoublePawnPush",
            6=> "En Passant",
            _ => "Invalid value"
        };
        let dest_pos = self.parse_to();
        let src_pos = self.parse_from();
        let src_coords = (to_row(src_pos as u8),to_col(src_pos as u8));
        let dest_coords = (to_row(dest_pos as u8),to_col(dest_pos as u8));
        write!(f,"Move {} from {} (row-{}, col-{}) to {} (row-{}, col-{})",mtype,src_pos,src_coords.0,src_coords.1,dest_pos,dest_coords.0,dest_coords.1)
    }
}