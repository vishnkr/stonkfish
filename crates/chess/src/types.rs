use serde::{Serialize,Deserialize};
use std::collections::HashMap;

use crate::bitboard::BitBoard;


#[derive(Debug,PartialEq,Clone,Serialize,Deserialize)]
pub struct Dimensions{
    pub height: u8,
    pub width: u8
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Color {
    White,
    Black,
}

pub const KSCW: u8 = 1 << 3;
pub const QSCW: u8 = 1 << 2;
pub const KSCB: u8 = 1 << 1;
pub const QSCB: u8 = 1 << 0;



pub struct Position {
    pub files: u8,
    pub ranks: u8,
    pub fen: String,
    pub largest_dimension: u8,
    pub white_to_move: bool,
    pub castling: u8,
    pub en_passant: Option<usize>,
    pub halfmove_clock: u32,
    pub fullmove_number: u32,

    pub pieces: HashMap<char, Box<dyn BitBoard>>,
    pub walls: Box<dyn BitBoard>,
    pub occupied: Box<dyn BitBoard>,
    pub color_bitboards: HashMap<Color, Box<dyn BitBoard>>,
    pub custom_piece_rules: HashMap<char, Vec<MovePattern>>,
}
