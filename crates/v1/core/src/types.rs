use serde::{Serialize,Deserialize};


#[derive(Debug,PartialEq,Clone,Serialize,Deserialize)]
pub struct Dimensions{
    pub files: u8,
    pub ranks: u8,
}


impl Dimensions {
    pub fn to_index(&self, file: u8, rank: u8) -> usize {
        (rank as usize) * (self.files as usize) + (file as usize)
    }

    pub fn from_index(&self, idx: usize) -> (u8, u8) {
        (
            (idx % self.files as usize) as u8,
            (idx / self.files as usize) as u8,
        )
    }

    pub fn size(&self) -> usize {
        self.ranks as usize * self.files as usize
    }

    pub fn largest(&self) -> u8 {
        self.ranks.max(self.files)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Color {
    White,
    Black,
}

impl Color{
    pub fn opposite(self)->Color{
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

pub const KSCW: u8 = 1 << 3;
pub const QSCW: u8 = 1 << 2;
pub const KSCB: u8 = 1 << 1;
pub const QSCB: u8 = 1 << 0;


#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct MoveOffset {
    pub dx: i8,
    pub dy: i8,
    pub repeat: bool, // true for slide, false for jump
}

#[derive(Debug, Clone)]
pub struct Piece{
    pub kind: PieceKind,
    pub color: Color
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum PieceKind{
    Standard(char),
    Custom{
        id: char,
        offsets: Vec<MoveOffset>,
    }
}



#[derive(Clone, Debug)]
pub enum GameOutcome {
    Win { winner: Color, reason: WinReason },
    Draw(DrawReason),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WinReason {
    Checkmate,
    Resignation,
    Stalemate,
    AllPiecesCaptured,
    NChecks(u8),
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DrawReason {
    Stalemate,
    ThreefoldRepetition,
    FiftyMoveRule,
    InsufficientMaterial,
    Agreement,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Dir {
    N, S, E, W, NE, NW, SE, SW,
}