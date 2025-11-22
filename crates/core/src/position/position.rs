use crate::{
    board::{Dimensions, Square, BitBoard, BB},
    piece::{PieceKind, Color, Piece},
    moves::*,
};
use std::collections::HashMap;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CastlingRights(u8);

impl CastlingRights {
    pub const WHITE_KINGSIDE: u8 = 0b0001;
    pub const WHITE_QUEENSIDE: u8 = 0b0010;
    pub const BLACK_KINGSIDE: u8 = 0b0100;
    pub const BLACK_QUEENSIDE: u8 = 0b1000;
    
    pub fn new() -> Self {
        Self(0)
    }
    
    pub fn has_white_kingside(&self) -> bool {
        (self.0 & Self::WHITE_KINGSIDE) != 0
    }
    
    pub fn has_white_queenside(&self) -> bool {
        (self.0 & Self::WHITE_QUEENSIDE) != 0
    }
    
    pub fn has_black_kingside(&self) -> bool {
        (self.0 & Self::BLACK_KINGSIDE) != 0
    }
    
    pub fn has_black_queenside(&self) -> bool {
        (self.0 & Self::BLACK_QUEENSIDE) != 0
    }
    
    pub fn set_white_kingside(&mut self, value: bool) {
        if value {
            self.0 |= Self::WHITE_KINGSIDE;
        } else {
            self.0 &= !Self::WHITE_KINGSIDE;
        }
    }
    
    pub fn set_white_queenside(&mut self, value: bool) {
        if value {
            self.0 |= Self::WHITE_QUEENSIDE;
        } else {
            self.0 &= !Self::WHITE_QUEENSIDE;
        }
    }
    
    pub fn set_black_kingside(&mut self, value: bool) {
        if value {
            self.0 |= Self::BLACK_KINGSIDE;
        } else {
            self.0 &= !Self::BLACK_KINGSIDE;
        }
    }
    
    pub fn set_black_queenside(&mut self, value: bool) {
        if value {
            self.0 |= Self::BLACK_QUEENSIDE;
        } else {
            self.0 &= !Self::BLACK_QUEENSIDE;
        }
    }
    
    pub fn as_u8(&self) -> u8 {
        self.0
    }
}

#[derive(Debug,Clone)]
pub struct StateSnapshot {
    pub captured: Option<Piece>,
    pub castling_rights: CastlingRights,
    pub ep_square: Option<Square>,
    pub halfmove_clock: u16,
    pub fullmove_number: u16,
}


#[derive(Debug, Clone)]
pub struct Position {
    pub dims: Dimensions,
    pub side_to_move: Color,
        pub pieces: HashMap<PieceKind, BitBoard>,
    
    /// Occupancy bitboards [white, black]
    pub occ: [BitBoard; 2],
    
    pub all: BitBoard,
    pub castling_rights: CastlingRights,
    pub ep_square: Option<Square>,
    pub halfmove_clock: u16,
    pub fullmove_number: u16,
    pub history: Vec<StateSnapshot>,

}

impl Position {
    pub fn new_empty(dims: Dimensions) -> Self {
        Self {
            dims,
            side_to_move: Color::White,
            pieces: HashMap::new(),
            occ: [BitBoard::empty_for_dims(&dims), BitBoard::empty_for_dims(&dims)],
            all: BitBoard::empty_for_dims(&dims),
            castling_rights: CastlingRights::new(),
            ep_square: None,
            halfmove_clock: 0,
            fullmove_number: 1,
            history: vec![],
        }
    }
    
    pub fn set_piece(&mut self, sq: Square, piece: Piece) {
        self.remove_piece(sq);
        
        let bb = self.pieces.entry(piece.kind).or_insert_with(|| BitBoard::empty_for_dims(&self.dims));
        *bb = bb.set(sq);

        let idx = piece.color as usize;
        self.occ[idx] = self.occ[idx].set(sq);

        self.all = self.all.set(sq);
    }

    pub fn remove_piece(&mut self, sq: Square) -> Option<Piece> {
        if !self.all.contains(sq) {
            return None;
        }
        
        let mut found_kind = None;
        for (&kind, bb) in self.pieces.iter_mut() {
            if bb.contains(sq) {
                *bb = bb.clear(sq);
                found_kind = Some(kind);
                break;
            }
        }
        
        if let Some(kind) = found_kind {
            let color = if self.occ[Color::White as usize].contains(sq) {
                Color::White
            } else {
                Color::Black
            };
            
            self.occ[color as usize] = self.occ[color as usize].clear(sq);
            
            // Remove from all occupancy
            self.all = self.all.clear(sq);
            
            Some(Piece { color, kind })
        } else {
            None
        }
    }

    pub fn piece_at(&self, sq: Square) -> Option<Piece> {
        if !self.all.contains(sq) {
            return None;
        }
        
        let color = if self.occ[Color::White as usize].contains(sq) {
            Color::White
        } else {
            Color::Black
        };

        for (&kind, bb) in self.pieces.iter() {
            if bb.contains(sq) {
                return Some(Piece { color, kind });
            }
        }
        None
    }
    
    pub fn piece_bb(&self, color: Color, kind: PieceKind) -> BitBoard {
        let color_bb = self.occ[color as usize];
        let kind_bb = self.pieces.get(&kind).copied().unwrap_or_else(|| BitBoard::empty_for_dims(&self.dims));
        color_bb.intersect(kind_bb)
    }
    
    pub fn color_bb(&self, color: Color) -> BitBoard {
        self.occ[color as usize]
    }
    
    pub fn kind_bb(&self, kind: PieceKind) -> BitBoard {
        self.pieces.get(&kind).copied().unwrap_or_else(|| BitBoard::empty_for_dims(&self.dims))
    }
    
    pub fn switch_side(&mut self) {
        self.side_to_move = self.side_to_move.opposite();
    }
    
    pub fn is_occupied(&self, sq: Square) -> bool {
        self.all.contains(sq)
    }
    
    pub fn is_occupied_by(&self, sq: Square, color: Color) -> bool {
        self.occ[color as usize].contains(sq)
    }

    pub fn make_move(&mut self,mv: Move){
        let src = mv.src();
        let dst = mv.dst();
        let kind = mv.kind();
        let moving_piece = self.piece_at(src).expect("No piece on src");
        self.history.push(StateSnapshot { 
            captured: self.piece_at(dst), 
            castling_rights: self.castling_rights, 
            ep_square: self.ep_square, 
            halfmove_clock: self.halfmove_clock, 
            fullmove_number: self.fullmove_number 
        });
        self.ep_square = None;
        match kind{
            MoveType::Capture => {
                self.remove_piece(src);
                self.set_piece(dst, moving_piece);
                self.halfmove_clock = 0;
            }
            MoveType::Quiet =>{
                self.remove_piece(dst);
                self.remove_piece(src);
                self.set_piece(dst, moving_piece);
                self.halfmove_clock+=1;
            }
            MoveType::Promotion =>{
                let promo_kind = PieceKind::Queen;
                self.remove_piece(src);
                self.set_piece(dst, Piece { color: moving_piece.color, kind: promo_kind });
                self.halfmove_clock = 0;
            }
            MoveType::EnPassant =>{
                let ep_target = self.ep_square.expect("EP target missing");
                self.remove_piece(ep_target);
                self.remove_piece(src);
                self.set_piece(dst, moving_piece);
                self.halfmove_clock = 0;
            }
            MoveType::Castling => {
                /*self.remove_piece(src);
                self.set_piece(dst, moving_piece);
                let (rook_src, rook_dst) = get_castling_rook_squares(src, dst);
                let rook = self.piece_at(rook_src).unwrap();
                self.remove_piece(rook_src);
                self.set_piece(rook_dst, rook);*/

                self.halfmove_clock += 1;
            }
        }
        // handle ep creation for doule pawn push
        // handle castle rights removal
        self.switch_side();

        if self.side_to_move == Color::White {
            self.fullmove_number += 1;
        }

    }

    pub fn unmake_move(&mut self, mv: Move){
        let src = mv.src();
        let dst = mv.dst();
        let kind = mv.kind();
        let snapshot = self.history.pop().expect("No history");
        let captured = snapshot.captured;
        self.castling_rights = snapshot.castling_rights;
        self.ep_square = snapshot.ep_square;
        self.halfmove_clock = snapshot.halfmove_clock;
        self.fullmove_number = snapshot.fullmove_number;

        self.switch_side();
        let moving_piece = self.remove_piece(dst).expect("Missiong destination");
        match kind{
            MoveType::Quiet => {
                self.set_piece(src, moving_piece);
            }
            MoveType::Capture => {
                self.set_piece(src, moving_piece);
                if let Some(pc) = captured {
                    self.set_piece(dst, pc);
                }
            }
            MoveType::Promotion => {
                self.remove_piece(dst);
                self.set_piece(src, Piece { kind: PieceKind::Pawn, color: moving_piece.color });
                if let Some(pc) = captured {
                    self.set_piece(dst, pc);
                }
            }
            MoveType::EnPassant => {
                self.set_piece(src, moving_piece);
                if let Some(pc) = captured {
                    self.set_piece(self.ep_square.unwrap(),pc);
                    //self.set_piece(self.ep_capture_square(dst, moving_piece.color), pc);
                }
            }
            MoveType::Castling => {
                /*self.set_piece(src, moving_piece);
                let (rook_src, rook_dst) = castling_rook_squares(src, dst);
                let rook = self.remove_piece(rook_dst).unwrap();
                self.set_piece(rook_src, rook);*/
            }
        }
    }
    

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::{Dimensions, BB};

    #[test]
    fn position_basics() {
        let dims = Dimensions::standard();
        let mut pos = Position::new_empty(dims);
        
        let sq = Square::from_rank_file(0, 0, &dims);
        pos.set_piece(sq, Piece {
            color: Color::White,
            kind: PieceKind::Rook,
        });
        
        assert_eq!(pos.piece_at(sq), Some(Piece {
            color: Color::White,
            kind: PieceKind::Rook,
        }));
    }
    
    #[test]
    fn position_set_and_remove() {
        let dims = Dimensions::standard();
        let mut pos = Position::new_empty(dims);
        
        let sq = Square::from_rank_file(0, 0, &dims);
        let piece = Piece {
            color: Color::White,
            kind: PieceKind::Rook,
        };
        
        pos.set_piece(sq, piece);
        assert!(pos.is_occupied(sq));
        assert!(pos.is_occupied_by(sq, Color::White));
        assert_eq!(pos.piece_bb(Color::White, PieceKind::Rook).count(), 1);
        
        let removed = pos.remove_piece(sq);
        assert_eq!(removed, Some(piece));
        assert!(!pos.is_occupied(sq));
        assert_eq!(pos.piece_bb(Color::White, PieceKind::Rook).count(), 0);
    }
    
    #[test]
    fn position_switch_side() {
        let dims = Dimensions::standard();
        let mut pos = Position::new_empty(dims);
        
        assert_eq!(pos.side_to_move, Color::White);
        pos.switch_side();
        assert_eq!(pos.side_to_move, Color::Black);
        pos.switch_side();
        assert_eq!(pos.side_to_move, Color::White);
    }
    
    #[test]
    fn position_piece_bb() {
        let dims = Dimensions::standard();
        let mut pos = Position::new_empty(dims);
        
        let sq1 = Square::from_rank_file(0, 0, &dims);
        let sq2 = Square::from_rank_file(0, 7, &dims);
        
        pos.set_piece(sq1, Piece {
            color: Color::White,
            kind: PieceKind::Rook,
        });
        pos.set_piece(sq2, Piece {
            color: Color::White,
            kind: PieceKind::Rook,
        });
        
        let rooks = pos.piece_bb(Color::White, PieceKind::Rook);
        assert_eq!(rooks.count(), 2);
        assert!(rooks.contains(sq1));
        assert!(rooks.contains(sq2));
    }
}
