use crate::board::{Dimensions, Square, BitBoard, BB};

/// Direction offsets for sliding pieces
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Direction {
    pub file_delta: i8,
    pub rank_delta: i8,
}

impl Direction {
    pub const NORTH: Direction = Direction { file_delta: 0, rank_delta: 1 };
    pub const SOUTH: Direction = Direction { file_delta: 0, rank_delta: -1 };
    pub const EAST: Direction = Direction { file_delta: 1, rank_delta: 0 };
    pub const WEST: Direction = Direction { file_delta: -1, rank_delta: 0 };
    pub const NORTHEAST: Direction = Direction { file_delta: 1, rank_delta: 1 };
    pub const NORTHWEST: Direction = Direction { file_delta: -1, rank_delta: 1 };
    pub const SOUTHEAST: Direction = Direction { file_delta: 1, rank_delta: -1 };
    pub const SOUTHWEST: Direction = Direction { file_delta: -1, rank_delta: -1 };
    
    pub const ROOK_DIRS: &'static [Direction] = &[
        Direction::NORTH,
        Direction::SOUTH,
        Direction::EAST,
        Direction::WEST,
    ];
    
    pub const BISHOP_DIRS: &'static [Direction] = &[
        Direction::NORTHEAST,
        Direction::NORTHWEST,
        Direction::SOUTHEAST,
        Direction::SOUTHWEST,
    ];
    
    pub const QUEEN_DIRS: &'static [Direction] = &[
        Direction::NORTH,
        Direction::SOUTH,
        Direction::EAST,
        Direction::WEST,
        Direction::NORTHEAST,
        Direction::NORTHWEST,
        Direction::SOUTHEAST,
        Direction::SOUTHWEST,
    ];
}


pub trait MovePattern {
    fn attacks_from(&self, sq: Square, dims: &Dimensions, occupied: BitBoard, friendly: BitBoard) -> BitBoard;
}

pub struct SlidingPattern {
    directions: Vec<Direction>,
    max_distance: Option<u8>, // None = unlimited
}

impl SlidingPattern {
    pub fn new(directions: Vec<Direction>) -> Self {
        Self {
            directions,
            max_distance: None,
        }
    }
    
    pub fn with_max_distance(directions: Vec<Direction>, max_distance: u8) -> Self {
        Self {
            directions,
            max_distance: Some(max_distance),
        }
    }
}

impl MovePattern for SlidingPattern {
    fn attacks_from(&self, sq: Square, dims: &Dimensions, occupied: BitBoard, friendly: BitBoard) -> BitBoard {
        let (file, rank) = sq.file_rank(dims);
        let mut attacks = BitBoard::empty();
        
        for &dir in &self.directions {
            let mut current_file = file as i8;
            let mut current_rank = rank as i8;
            let mut distance = 0;
            
            loop {
                current_file += dir.file_delta;
                current_rank += dir.rank_delta;
                
                if current_file < 0 || current_file >= dims.width as i8 ||
                   current_rank < 0 || current_rank >= dims.height as i8 {
                    break;
                }
                
                if let Some(max) = self.max_distance {
                    if distance >= max {
                        break;
                    }
                }
                
                let target_sq = Square::from_rank_file(
                    current_rank as u8,
                    current_file as u8,
                    dims,
                );
                
                if occupied.contains(target_sq) {
                    if !friendly.contains(target_sq) {
                        attacks = attacks.set(target_sq);
                    }
                    break;
                }
                
                attacks = attacks.set(target_sq);
                distance += 1;
            }
        }
        
        attacks
    }
}

pub struct JumpingPattern {
    offsets: Vec<Direction>,
}

impl JumpingPattern {
    pub fn new(offsets: Vec<Direction>) -> Self {
        Self { offsets }
    }
}

impl MovePattern for JumpingPattern {
    fn attacks_from(&self, sq: Square, dims: &Dimensions, _occupied: BitBoard, friendly: BitBoard) -> BitBoard {
        let (file, rank) = sq.file_rank(dims);
        let mut attacks = BitBoard::empty();
        
        for &offset in &self.offsets {
            let target_file = file as i8 + offset.file_delta;
            let target_rank = rank as i8 + offset.rank_delta;
            
            if target_file >= 0 && target_file < dims.width as i8 &&
               target_rank >= 0 && target_rank < dims.height as i8 {
                let target_sq = Square::from_rank_file(
                    target_rank as u8,
                    target_file as u8,
                    dims,
                );
                
                if !friendly.contains(target_sq) {
                    attacks = attacks.set(target_sq);
                }
            }
        }
        
        attacks
    }
}

