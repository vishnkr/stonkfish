use std::fmt;

#[derive(Debug,PartialEq)]
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