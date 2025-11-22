use crate::{
    board::{Dimensions, Square, BitBoard, BB},
    piece::{PieceKind, Color, Piece},
    position::Position,
    moves::{Move, MoveType},
    movegen::patterns::MovePattern,
    movegen::standard::StandardPatterns,
    movegen::attack_table::AttackTable,
};
use std::collections::HashMap;


pub struct MoveGenerator {
    dims: Dimensions,
    attack_table: AttackTable,
    custom_patterns: HashMap<PieceKind, Box<dyn MovePattern>>,
}

impl MoveGenerator {
    pub fn new(dims: Dimensions) -> Self {
        let attack_table = AttackTable::new(dims);
        Self {
            dims,
            attack_table,
            custom_patterns: HashMap::new(),
        }
    }
    
    pub fn register_custom_pattern(&mut self, kind: PieceKind, pattern: Box<dyn MovePattern>) {
        self.custom_patterns.insert(kind, pattern);
    }
    
    pub fn generate_pseudo_legal(&self, pos: &Position) -> Vec<Move> {
        let mut moves = Vec::new();
        let color = pos.side_to_move;
        let friendly = pos.color_bb(color);
        let enemy = pos.color_bb(color.opposite());
        let occupied = pos.all;
        
        for (&kind, &kind_bb) in pos.pieces.iter() {
            let pieces_bb = kind_bb.intersect(friendly);
            let mut pieces_bb = pieces_bb;
            
            while !pieces_bb.is_empty() {
                let Some(sq) = pieces_bb.pop_lsb() else { break };
                
                if kind == PieceKind::Pawn {
                    self.generate_pawn_moves(pos, sq, color, &mut moves);
                    continue;
                }
                
                let pattern: &dyn MovePattern = if let Some(custom) = self.custom_patterns.get(&kind) {
                    custom.as_ref()
                } else { 
                    self.generate_standard_moves(pos, sq, kind, color, &mut moves);
                    continue;
                };
                
                let mut attacks = pattern.attacks_from(sq, &self.dims, occupied, friendly);
                
                while !attacks.is_empty() {
                    let Some(target) = attacks.pop_lsb() else { break };
                    
                    let move_type = if enemy.contains(target) {
                        MoveType::Capture
                    } else {
                        MoveType::Quiet
                    };
                    
                    moves.push(Move::new(sq, target, move_type, 0));
                }
            }
        }
        
        self.generate_castling_moves(pos, color, &mut moves);
        
        moves
    }
    
    fn generate_castling_moves(
        &self,
        pos: &Position,
        color: Color,
        moves: &mut Vec<Move>,
    ) {
        if self.dims.width < 5 || self.dims.height < 5 {
            return;
        }
        
        let (king_rank, kingside_rook_file, queenside_rook_file) = match color {
            Color::White => (0, self.dims.width - 1, 0),
            Color::Black => (self.dims.height - 1, self.dims.width - 1, 0),
        };
        
        let king_file = match color {
            Color::White => {
                if self.dims.width >= 8 { 4 } else { self.dims.width / 2 }
            }
            Color::Black => {
                if self.dims.width >= 8 { 4 } else { self.dims.width / 2 }
            }
        };
        
        let king_sq = Square::from_rank_file(king_rank, king_file, &self.dims);
        
        if pos.piece_at(king_sq) != Some(Piece { color, kind: PieceKind::King }) {
            return;
        }
        
        let occupied = pos.all;
        

        if match color {
            Color::White => pos.castling_rights.has_white_kingside(),
            Color::Black => pos.castling_rights.has_black_kingside(),
        } {
            let kingside_rook_sq = Square::from_rank_file(king_rank, kingside_rook_file, &self.dims);
            
            if pos.piece_at(kingside_rook_sq) == Some(Piece { color, kind: PieceKind::Rook }) {
                let mut can_castle = true;
                let start_file = king_file.min(kingside_rook_file);
                let end_file = king_file.max(kingside_rook_file);
                
                for file in (start_file + 1)..end_file {
                    let sq = Square::from_rank_file(king_rank, file, &self.dims);
                    if occupied.contains(sq) {
                        can_castle = false;
                        break;
                    }
                }
                
                if can_castle {
                    let castling_king_file = if kingside_rook_file > king_file {
                        king_file + 2
                    } else {
                        king_file - 2
                    };
                    
                    if castling_king_file < self.dims.width {
                        let castling_king_sq = Square::from_rank_file(king_rank, castling_king_file, &self.dims);
                        moves.push(Move::new(king_sq, castling_king_sq, MoveType::Castling, 0));
                    }
                }
            }
        }
        
        if match color {
            Color::White => pos.castling_rights.has_white_queenside(),
            Color::Black => pos.castling_rights.has_black_queenside(),
        } {
            let queenside_rook_sq = Square::from_rank_file(king_rank, queenside_rook_file, &self.dims);
            
            if pos.piece_at(queenside_rook_sq) == Some(Piece { color, kind: PieceKind::Rook }) {
                let mut can_castle = true;
                let start_file = king_file.min(queenside_rook_file);
                let end_file = king_file.max(queenside_rook_file);
                
                for file in (start_file + 1)..end_file {
                    let sq = Square::from_rank_file(king_rank, file, &self.dims);
                    if occupied.contains(sq) {
                        can_castle = false;
                        break;
                    }
                }
                
                if can_castle {
                    let castling_king_file = if queenside_rook_file > king_file {
                        king_file + 2
                    } else {
                        king_file - 2
                    };
                    
                    if castling_king_file < self.dims.width {
                        let castling_king_sq = Square::from_rank_file(king_rank, castling_king_file, &self.dims);
                        moves.push(Move::new(king_sq, castling_king_sq, MoveType::Castling, 0));
                    }
                }
            }
        }
    }
    
    fn generate_standard_moves(
        &self,
        pos: &Position,
        sq: Square,
        kind: PieceKind,
        color: Color,
        moves: &mut Vec<Move>,
    ) {
        let friendly = pos.color_bb(color);
        let enemy = pos.color_bb(color.opposite());
        let occupied = pos.all;
        
        let mut attacks = match kind {
            PieceKind::Knight => {
                self.attack_table.knight_attacks(sq)
            }
            PieceKind::Bishop => {
                self.attack_table.bishop_attacks(sq, occupied)
            }
            PieceKind::Rook => {
                self.attack_table.rook_attacks(sq, occupied)
            }
            PieceKind::Queen => {
                self.attack_table.queen_attacks(sq, occupied)
            }
            PieceKind::King => {
                self.attack_table.king_attacks(sq)
            }
            _ => BitBoard::empty_for_dims(&self.dims),
        };
        
        attacks = attacks.difference(friendly);
        while !attacks.is_empty() {
            let Some(target) = attacks.pop_lsb() else { break };
            
            let move_type = if enemy.contains(target) {
                MoveType::Capture
            } else {
                MoveType::Quiet
            };
            
            moves.push(Move::new(sq, target, move_type, 0));
        }
    }
    
    fn generate_pawn_moves(
        &self,
        pos: &Position,
        sq: Square,
        color: Color,
        moves: &mut Vec<Move>,
    ) {
        let friendly = pos.color_bb(color);
        let enemy = pos.color_bb(color.opposite());
        let occupied = pos.all;
        
        let mut attacks = StandardPatterns::pawn_attacks(sq, color, &self.dims, friendly);
        attacks = attacks.intersect(enemy);
        
        while !attacks.is_empty() {
            let Some(target) = attacks.pop_lsb() else { break };
            
            let (_, rank) = target.file_rank(&self.dims);
            let promotion_rank = match color {
                Color::White => self.dims.height - 1,
                Color::Black => 0,
            };
            
            if rank == promotion_rank {
                moves.push(Move::new(sq, target, MoveType::Promotion, 0));
            } else {
                moves.push(Move::new(sq, target, MoveType::Capture, 0));
            }
        }
        
        let mut pushes = StandardPatterns::pawn_pushes(sq, color, &self.dims, occupied);
        
        while !pushes.is_empty() {
            let Some(target) = pushes.pop_lsb() else { break };
            
            let (_, rank) = target.file_rank(&self.dims);
            let promotion_rank = match color {
                Color::White => self.dims.height - 1,
                Color::Black => 0,
            };
            
            if rank == promotion_rank {
                moves.push(Move::new(sq, target, MoveType::Promotion, 0));
            } else {
                moves.push(Move::new(sq, target, MoveType::Quiet, 0));
            }
        }
        
        if let Some(ep_sq) = pos.ep_square {
            let attacks = StandardPatterns::pawn_attacks(sq, color, &self.dims, friendly);
            if attacks.contains(ep_sq) {
                moves.push(Move::new(sq, ep_sq, MoveType::EnPassant, 0));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Dimensions;
    use crate::piece::{Color, PieceKind, Piece};
    use crate::position::Fen;

    #[test]
    fn move_generation_king() {
        let dims = Dimensions::standard();
        let mut pos = Position::new_empty(dims);
                let e1 = Square::from_rank_file(0, 4, &dims);
        pos.set_piece(e1, Piece {
            color: Color::White,
            kind: PieceKind::King,
        });
        
        let gen = MoveGenerator::new(dims);
        let moves = gen.generate_pseudo_legal(&pos);
        
        assert!(moves.len() > 0);
        assert!(moves.len() <= 8);
    }
    
    #[test]
    fn move_generation_knight() {
        let dims = Dimensions::standard();
        let mut pos = Position::new_empty(dims);
        
        let b1 = Square::from_rank_file(0, 1, &dims);
        pos.set_piece(b1, Piece {
            color: Color::White,
            kind: PieceKind::Knight,
        });
        
        let gen = MoveGenerator::new(dims);
        let moves = gen.generate_pseudo_legal(&pos);
        
        assert!(moves.len() == 3);
    }
    
    #[test]
    fn move_generation_rook() {
        let dims = Dimensions::standard();
        let mut pos = Position::new_empty(dims);
        
        let a1 = Square::from_rank_file(0, 0, &dims);
        pos.set_piece(a1, Piece {
            color: Color::White,
            kind: PieceKind::Rook,
        });
        
        let gen = MoveGenerator::new(dims);
        let moves = gen.generate_pseudo_legal(&pos);
        
        assert!(moves.len() == 14);
    }
    
    #[test]
    fn move_generation_pawn() {
        let dims = Dimensions::standard();
        let mut pos = Position::new_empty(dims);
        
        let e2 = Square::from_rank_file(1, 4, &dims);
        pos.set_piece(e2, Piece {
            color: Color::White,
            kind: PieceKind::Pawn,
        });
        
        let gen = MoveGenerator::new(dims);
        let moves = gen.generate_pseudo_legal(&pos);
        
        assert!(moves.len() == 2);
    }
    
    #[test]
    fn move_generation_capture() {
        let dims = Dimensions::standard();
        let mut pos = Position::new_empty(dims);
        
        let a1 = Square::from_rank_file(0, 0, &dims);
        let a7 = Square::from_rank_file(6, 0, &dims);
        
        pos.set_piece(a1, Piece {
            color: Color::White,
            kind: PieceKind::Rook,
        });
        pos.set_piece(a7, Piece {
            color: Color::Black,
            kind: PieceKind::Pawn,
        });
        
        let gen = MoveGenerator::new(dims);
        let moves = gen.generate_pseudo_legal(&pos);
        
        let capture_moves: Vec<_> = moves.iter()
            .filter(|m| m.kind() == MoveType::Capture)
            .collect();
        assert!(capture_moves.len() > 0);
    }
    
    #[test]
    fn move_generation_castling() {
        let dims = Dimensions::standard();
        let fen_str = "r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQkq - 0 1";
        let pos = Fen::parse(fen_str, dims).unwrap();
        
        let gen = MoveGenerator::new(dims);
        let moves = gen.generate_pseudo_legal(&pos);
        
        let castling_moves: Vec<_> = moves.iter()
            .filter(|m| m.kind() == MoveType::Castling)
            .collect();
        assert!(castling_moves.len() >= 2);
    }
    
    #[test]
    fn move_generation_en_passant() {
        let dims = Dimensions::standard();
        let fen_str = "rnbqkbnr/pppppppp/8/8/4Pp2/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1";
        let pos = Fen::parse(fen_str, dims).unwrap();
        
        let gen = MoveGenerator::new(dims);
        let moves = gen.generate_pseudo_legal(&pos);
        
        let ep_moves: Vec<_> = moves.iter()
            .filter(|m| m.kind() == MoveType::EnPassant)
            .collect();
        assert!(ep_moves.len() > 0);
    }
    
    #[test]
    fn move_generation_complex_position() {
        let dims = Dimensions::standard();
        let fen_str = "r1bqkb1r/pppp1ppp/2n2n2/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 4 4";
        let pos = Fen::parse(fen_str, dims).unwrap();
        
        let gen = MoveGenerator::new(dims);
        let moves = gen.generate_pseudo_legal(&pos);
        
        assert!(moves.len() > 10);
    }
    
    #[test]
    fn move_generation_non_standard_board() {
        let dims = Dimensions::new(8, 8);
        let mut pos = Position::new_empty(dims);
        
        let king_sq = Square::from_rank_file(0, 4, &dims);
        pos.set_piece(king_sq, Piece {
            color: Color::White,
            kind: PieceKind::King,
        });
        
        let gen = MoveGenerator::new(dims);
        let moves = gen.generate_pseudo_legal(&pos);
        
        assert!(moves.len() > 0);
    }
    
    #[test]
    fn move_generation_small_board() {
        let dims = Dimensions::new(5, 5);
        let mut pos = Position::new_empty(dims);
        
        let king_sq = Square::from_rank_file(2, 2, &dims);
        pos.set_piece(king_sq, Piece {
            color: Color::White,
            kind: PieceKind::King,
        });
        
        let gen = MoveGenerator::new(dims);
        let moves = gen.generate_pseudo_legal(&pos);
        
        assert_eq!(moves.len(), 8);
    }
    
    #[test]
    fn move_generation_promotion() {
        let dims = Dimensions::standard();
        let fen_str = "8/4P3/8/8/8/8/8/8 w - - 0 1";
        let pos = Fen::parse(fen_str, dims).unwrap();
        
        let gen = MoveGenerator::new(dims);
        let moves = gen.generate_pseudo_legal(&pos);
        
        let promo_moves: Vec<_> = moves.iter()
            .filter(|m| m.kind() == MoveType::Promotion)
            .collect();
        assert!(promo_moves.len() > 0);
    }
}

