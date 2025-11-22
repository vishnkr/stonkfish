use crate::board::{Dimensions, Square};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveType {
    Quiet = 0,
    Capture = 1,
    Promotion = 2,
    EnPassant = 3,
    Castling = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move(u32);

impl Move {
    pub fn new(src: Square, dst: Square, move_type: MoveType, flags: u8) -> Self {
        let v = (src.0 as u32)
            | ((dst.0 as u32) << 8)
            | ((move_type as u32) << 16)
            | ((flags as u32) << 24);
        Self(v)
    }

    pub fn src(self) -> Square {
        Square((self.0 & 0xFF) as u16)
    }

    pub fn dst(self) -> Square {
        Square(((self.0 >> 8) & 0xFF) as u16)
    }

    pub fn kind(self) -> MoveType {
        match ((self.0 >> 16) & 0xFF) as u8 {
            0 => MoveType::Quiet,
            1 => MoveType::Capture,
            2 => MoveType::Promotion,
            3 => MoveType::EnPassant,
            4 => MoveType::Castling,
            _ => unreachable!(),
        }
    }

    pub fn flags(self) -> u8 {
        ((self.0 >> 24) & 0xFF) as u8
    }

    pub fn debug_string(self, dims: &Dimensions) -> String {
        format!(
            "{} -> {} ({:?})",
            self.src().to_string(dims),
            self.dst().to_string(dims),
            self.kind(),
        )
    }

    pub fn to_uci(self, dims: &Dimensions) -> String {
        let mut s = format!(
            "{}{}",
            self.src().to_string(dims),
            self.dst().to_string(dims)
        );

        if self.kind() == MoveType::Promotion {
            s.push('q');
        }

        s
    }
}
