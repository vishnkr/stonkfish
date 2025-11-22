use crate::{
    board::{Dimensions, Square, BitBoard, BB},
    piece::{PieceKind, Color},
    movegen::patterns::{MovePattern, SlidingPattern, JumpingPattern, Direction},
};

pub struct StandardPatterns;

impl StandardPatterns {
    pub fn pattern_for(kind: PieceKind) -> Box<dyn MovePattern> {
        match kind {
            PieceKind::Pawn => {
                Box::new(JumpingPattern::new(vec![]))
            }
            PieceKind::Knight => {
                Box::new(JumpingPattern::new(vec![
                    Direction { file_delta: 1, rank_delta: 2 },
                    Direction { file_delta: 2, rank_delta: 1 },
                    Direction { file_delta: -1, rank_delta: 2 },
                    Direction { file_delta: -2, rank_delta: 1 },
                    Direction { file_delta: 1, rank_delta: -2 },
                    Direction { file_delta: 2, rank_delta: -1 },
                    Direction { file_delta: -1, rank_delta: -2 },
                    Direction { file_delta: -2, rank_delta: -1 },
                ]))
            }
            PieceKind::Bishop => {
                Box::new(SlidingPattern::new(Direction::BISHOP_DIRS.to_vec()))
            }
            PieceKind::Rook => {
                Box::new(SlidingPattern::new(Direction::ROOK_DIRS.to_vec()))
            }
            PieceKind::Queen => {
                Box::new(SlidingPattern::new(Direction::QUEEN_DIRS.to_vec()))
            }
            PieceKind::King => {
                Box::new(JumpingPattern::new(vec![
                    Direction::NORTH,
                    Direction::SOUTH,
                    Direction::EAST,
                    Direction::WEST,
                    Direction::NORTHEAST,
                    Direction::NORTHWEST,
                    Direction::SOUTHEAST,
                    Direction::SOUTHWEST,
                ]))
            }
            PieceKind::Custom(_) => {
                Box::new(JumpingPattern::new(vec![]))
            }
        }
    }
    
    pub fn pawn_attacks(sq: Square, color: Color, dims: &Dimensions, friendly: BitBoard) -> BitBoard {
        let (file, rank) = sq.file_rank(dims);
        let mut attacks = BitBoard::empty();
        
        let forward = match color {
            Color::White => 1,
            Color::Black => -1,
        };
        
        // Pawns attack diagonally forward
        for file_delta in [-1, 1] {
            let target_file = file as i8 + file_delta;
            let target_rank = rank as i8 + forward;
            
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
    
    pub fn pawn_pushes(sq: Square, color: Color, dims: &Dimensions, occupied: BitBoard) -> BitBoard {
        let (file, rank) = sq.file_rank(dims);
        let mut pushes = BitBoard::empty();
        
        let forward = match color {
            Color::White => 1,
            Color::Black => -1,
        };
        
        // Single push
        let target_rank = rank as i8 + forward;
        if target_rank >= 0 && target_rank < dims.height as i8 {
            let target_sq = Square::from_rank_file(
                target_rank as u8,
                file,
                dims,
            );
            
            if !occupied.contains(target_sq) {
                pushes = pushes.set(target_sq);
                
                // Double push from starting rank
                let starting_rank = match color {
                    Color::White => 1,
                    Color::Black => dims.height - 2,
                };
                
                if rank == starting_rank {
                    let double_rank = rank as i8 + forward * 2;
                    if double_rank >= 0 && double_rank < dims.height as i8 {
                        let double_sq = Square::from_rank_file(
                            double_rank as u8,
                            file,
                            dims,
                        );
                        if !occupied.contains(double_sq) {
                            pushes = pushes.set(double_sq);
                        }
                    }
                }
            }
        }
        
        pushes
    }
}

