use std::fmt;

#[derive(Debug,PartialEq)]
pub struct Move(u32);

#[derive(PartialEq)]
pub enum MType{
    Regular,
    Capture,
    Promote,
    Castle
}

impl Move{
    pub fn new(src:u8,dest:u8,mtype:MType)->Move{
        Move(
            (((0 | (src as u32))<< 16u32) | (dest as u32) << 8u32)| 
            match mtype {
                MType::Regular => {0},
                MType::Capture => {1u32},
                MType::Promote => {2u32},
                MType::Castle => {3u32},
            }).into()
    }
}

impl fmt::Binary for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"{:#032b}",self.0)
    }
}