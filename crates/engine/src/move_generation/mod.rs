use arrayvec::ArrayVec;
use moves::*;
use crate::{
    bitboard::*,
    position::{*, piece::Piece},
};
use std::{vec::{Vec}, collections::HashMap};
use self::att_table::AttackTable;
use lazy_static::lazy_static;

use super::position::piece::PieceRepr;

pub mod moves;
pub mod att_table;

pub type JumpAttackTable = HashMap<PieceRepr,ArrayVec::<Bitboard,256>>;

lazy_static! {
    static ref LAZY_ATTACK_TABLE: AttackTable = AttackTable::new();
}

pub struct MoveGenerator{
    pub attack_table: &'static AttackTable,
    pub dimensions: Dimensions,
    pub size_dependent_bitboards:SizeDependentBitboards,
    pub jump_attack_table: JumpAttackTable
}


impl MoveGenerator{
    pub fn new(dimensions:Dimensions,jump_offsets:Option<JumpOffset>)->Self{
        let size_dependent_bitboards = SizeDependentBitboards::new(&dimensions);
        let mut jump_attack_table:JumpAttackTable = HashMap::new();
        if let Some(jump_info) = jump_offsets{
            Self::setup_jump_targets(&mut jump_attack_table, &jump_info);
        }
        Self{
            attack_table: &LAZY_ATTACK_TABLE,
            dimensions,
            size_dependent_bitboards,
            jump_attack_table,
        }
    }

    pub fn setup_jump_targets(jump_attack_table:&mut JumpAttackTable ,piece_info:&JumpOffset){
        for (piece_repr,jump_offsets) in piece_info{
            Self::setup_jump_target(jump_attack_table,*piece_repr,jump_offsets);
        }
    }

    pub fn setup_jump_target(jump_attack_table:&mut JumpAttackTable,piece_repr:PieceRepr,offsets:&Vec<(i8,i8)>){
            let mut jump_targets_bitboards = ArrayVec::<Bitboard,256>::new();
            for i in 0..16{
                for j in 0..16{
                    let mut bb = Bitboard::zero();
                    for (dx,dy) in offsets{
                        let (x,y) = (i+dx,j+dy);
                        if x>=0 && x<16 && y>=0 && y<16{
                            bb.set_bit(to_pos(x as u8,y as u8), true);
                        }
                    }
                    jump_targets_bitboards.push(bb);
                }         
            }
        // jump offsets are same for all pieces in both colors except pawns where the direction changes based on color
        if piece_repr=='P' {
            jump_attack_table.insert(piece_repr, jump_targets_bitboards);
            return;
        }
        jump_attack_table.insert(piece_repr.to_ascii_lowercase(), jump_targets_bitboards);
    }

    pub  fn get_all_attacks_bb(&self,piece:&Piece,src:u8,occupancy:&Bitboard)->Bitboard{
        let mut attack_bb = Bitboard::zero();
        attack_bb |= self.generate_slide_moves(src,&piece,occupancy);
        attack_bb |= self.generate_jump_moves(&piece,src);
        attack_bb
    }

    pub fn get_other_jump_moves(&self,src:u8,capture_only_offsets:&Vec<(i8,i8)>)->Bitboard{
        let mut attack_bb = Bitboard::zero();
        let (row,col) = (to_row(src),to_col(src));
        for (dx,dy) in capture_only_offsets{
            let capture_coords = (add_u8_i8(row, *dx), add_u8_i8(col, *dy));
            match capture_coords{
                (Some(x),Some(y))=>{
                    let pos = to_pos(x,y);
                    attack_bb.set_bit(pos, true);
                },
                _=> continue
            }
        }
        attack_bb
    }

    pub fn generate_pseudolegal_moves(&self,cur_position:&mut Position)->impl Iterator<Item=Move>{
        let color = cur_position.turn;
        let opponent_bb = cur_position.piece_collections[!color as usize].occupied.clone();
        let opp_king_pos = &cur_position.piece_collections[!color as usize].get_king().bitboard.clone();
        let player_pieces = &mut cur_position.piece_collections[color as usize];
        
        let occupied_bb = &cur_position.position_bitboard;
        
        let mut moves = vec![];
        let occ = &player_pieces.occupied;
        for (_,piece) in player_pieces.pieces.iter_mut(){
            
            let mut piece_bb = piece.bitboard.clone();
            while !piece_bb.is_zero(){
                let src = piece_bb.lowest_one().unwrap_or(0) as u8;
                let mut attack_bb = self.get_all_attacks_bb(piece, src, occupied_bb);
                if piece.props.can_double_jump{
                    self.generate_double_jump_moves(&mut moves,&piece,src,occupied_bb);
                }
                if piece.props.capture_only_jump_offsets.len() > 0{
                    attack_bb|= self.get_other_jump_moves(src, &piece.props.capture_only_jump_offsets) & &opponent_bb;
                }
                if piece.props.non_capture_only_jump_offsets.len()>0{
                    let non_capture_bb = self.get_other_jump_moves(src, &piece.props.non_capture_only_jump_offsets) & !occupied_bb;
                    attack_bb |= non_capture_bb;
                }
                attack_bb &= !opp_king_pos & !occ;
                piece_bb.set_bit(src as usize, false);
                if !attack_bb.is_zero(){
                    flatten_bitboard(&mut attack_bb,&mut moves, &opponent_bb, piece.piece_type, src);
                }
            }
            
        }
        moves.into_iter()
    }

    pub fn generate_slide_moves(&self,square:u8,piece:&Piece,occupancy:&Bitboard)->Bitboard{
        let mut final_bb = Bitboard::zero();
        let (mut can_rank_slide,mut can_file_slide,mut can_diagonal_slide,mut can_antidiagonal_slide) = (false,false,false,false);
        for (dx,dy) in piece.props.slide_directions.iter(){
            match (dx,dy){
                (1,0) | (-1,0) => can_rank_slide = true,
                (0,1) | (0,-1) => can_file_slide = true,
                (1,1) | (-1,-1) => can_antidiagonal_slide = true,
                (1,-1) | (-1,1) => can_diagonal_slide = true,
                _=>{}
            }
        }
        if can_rank_slide {
            final_bb |= self.attack_table.get_rank_attacks(square, occupancy);
        }
        if  can_file_slide {
            final_bb |= self.attack_table.get_file_attacks(square, occupancy);
        }
        if can_diagonal_slide {
            final_bb |= self.attack_table.get_diagonal_attacks(square, occupancy);
        }
        if  can_antidiagonal_slide {
            final_bb |= self.attack_table.get_anti_diagonal_attacks(square, occupancy);
        }
        final_bb &= &self.size_dependent_bitboards.full_bitboard;
        final_bb
    }   

    pub fn generate_jump_moves(&self,piece:&Piece,square:u8)->Bitboard{
        let mut char = piece.piece_repr;
        let mut final_bb = Bitboard::zero();
        if char!='P' {
            char = char.to_ascii_lowercase();
        }
        match self.jump_attack_table.get(&char){
            Some(jump_offsets) =>{
                final_bb |= &jump_offsets[square as usize];
            },
            None =>{}//throw error }s
        }
        final_bb &= &self.size_dependent_bitboards.full_bitboard;
        final_bb
    }

    pub fn generate_double_jump_moves(&self,moves:&mut Vec<Move>,piece:&Piece,src:u8,occupied:&Bitboard){
        let (src_row,src_col) = (to_row(src),to_col(src));
        if (piece.props.double_jump_squares.as_ref().unwrap()).contains(&src){
            for (dx,dy) in piece.props.non_capture_only_jump_offsets.iter(){
                let (s_jump_row,s_jump_col) = (add_u8_i8(src_row, *dx),add_u8_i8(src_col, *dy));
                let (d_jump_row,d_jump_col) = (add_u8_i8(src_row, dx*2),add_u8_i8(src_col, dy*2));
                match (s_jump_row,s_jump_col,d_jump_row,d_jump_col){
                    (Some(s_row),Some(s_col),Some(d_row),Some(d_col))=>{
                        let (s_jump_dest,d_jump_dest) = ( to_pos(s_row,s_col) , to_pos(d_row,d_col));
        
                        if !occupied.bit(s_jump_dest).unwrap_or(true)&& !occupied.bit(d_jump_dest).unwrap(){
                                moves.push(Move::encode_move(src, d_jump_dest as u8, MType::Quiet, None));
                        }
                    },
                    _=>{}
                }
                
            }
        }
    }
    
    pub fn is_legal_move(&self,position:&mut Position, mv: &Move)->bool{
        let mut is_under_check = false;
        let turn = position.turn;
        position.make_move(mv);
        if self.is_king_under_check(position,turn){
            is_under_check = true;
        }
        
        position.unmake_move(mv);
        !is_under_check
    }

    pub fn is_promotion_push(&self,index:u8,color:Color)->bool{
        match color{
            Color::WHITE => to_row(index) == 1,
            Color::BLACK => to_row(index) == self.dimensions.height-1
        }
    }

    pub fn is_king_under_check(&self,position:&mut Position,color:Color)-> bool{
        let opponent_color = !color;
        let mut opponent_bb =  &position.position_bitboard & !&position.piece_collections[color as usize].occupied;
        let pieces = &mut position.piece_collections;
        let occupancy = &position.position_bitboard;
        while !opponent_bb.is_zero(){
            let src = opponent_bb.lowest_one().unwrap_or(0) as u8;
            let piece = pieces[opponent_color as usize].get_mut_piece_from_sq(src.into()).unwrap();
            let attack_bb = self.get_all_attacks_bb(piece, src, occupancy);
            if !(attack_bb & &pieces[color as usize].get_king().bitboard).is_zero(){
                return true
            }
            opponent_bb.set_bit(src.into(),false);
        }

        false
    }    
}

pub fn generate_legal_moves(move_generator :&MoveGenerator,position :&mut Position)->Vec<Move>{
    let moves = move_generator.generate_pseudolegal_moves(position);
    let mut legal_moves:Vec<Move> = Vec::new();
    for mv in moves.filter(|mv| move_generator.is_legal_move(position, &mv)){
     legal_moves.push(mv);
    }
    legal_moves
}

#[cfg(test)]
mod movegen_tests{
    use super::*;
    #[test]
    pub fn test_rank_attack_occupancy_lookup(){
        let occupancy_lookup:Vec<Vec<u16>> = AttackTable::gen_occupancy_lookup();
        assert_eq!(occupancy_lookup[4][0b01010101],0b01101100);
        assert_eq!(occupancy_lookup[0][0b01010101],0b0110);
    }

    #[test]
    pub fn test_get_rook_attacks(){
        let position = fen::load_from_fen("8/3b4/8/8/Q2R4/8/8/3n4 w - - 0 1".to_string());
        let mvgen = MoveGenerator::new(Dimensions { height:12, width:12 },position.get_jump_offets());
        let pos = 67;
        let row = to_row(pos) as i8;
        let col = to_col(pos) as i8;
        display_bitboard_with_board_desc(&(mvgen.attack_table.diagonals.get(&(col-row)).unwrap()), "Diagonal from pos");
        display_bitboard_with_board_desc(&(mvgen.attack_table.anti_diagonals.get(&(col+row)).unwrap()), "aNTI Diagonal from pos");
    }
    #[test]
    pub fn print_helper_test(){
        let dimensions = Dimensions{width:8,height:8};
        let fen = "3k4/8/8/8/1n3b2/P1P1P3/1PP3P1/3K4 w - - 0 1";
        let mut position = fen::load_from_fen(fen.to_string());
        let mvgen = MoveGenerator::new(dimensions,position.get_jump_offets());
        let val = mvgen.generate_pseudolegal_moves(&mut position);
        for mv in val{
            println!("{}",mv.to_algebraic_notation(8, Color::WHITE, &position.piece_collections[Color::WHITE as usize]));
        }
    }

    #[test]
    pub fn test_is_king_under_check(){
        let dimensions = Dimensions{width:8,height:8};
        let fen = "3q4/8/8/8/8/8/8/3K4 w - - 0 1";
        let mut position = fen::load_from_fen(fen.to_string());
        let mvgen = MoveGenerator::new(dimensions,position.get_jump_offets());
        assert_eq!(mvgen.is_king_under_check(&mut position,Color::WHITE),true);
    }
    #[test]
    pub fn test_legal_movegen(){
        let dimensions = Dimensions{width:8,height:8};
        let mut position = fen::load_from_fen("3r1k2/8/8/8/8/8/3R4/3K4 w - - 0 1".to_string());
        let mvgen = MoveGenerator::new(dimensions,position.get_jump_offets());
        for mv in generate_legal_moves(&mvgen,&mut position){
            println!("{}",mv.to_algebraic_notation(8, Color::WHITE, &position.piece_collections[Color::WHITE as usize]));
        }
        
    }

    #[test]
    pub fn test_unmake_move(){
        let dimensions = Dimensions{width:8,height:8};
        let position = &mut fen::load_from_fen("3k4/8/8/8/1n3b2/P1P1P3/1PP3P1/3K4 w - - 0 1".to_string());
        let mvgen = MoveGenerator::new(dimensions,position.get_jump_offets());
        let original = &fen::load_from_fen("3k4/8/8/8/1n3b2/P1P1P3/1PP3P1/3K4 w - - 0 1".to_string());
        for mv in mvgen.generate_pseudolegal_moves(position){
            position.make_move(&mv);
            position.unmake_move( &mv);
            assert_eq!(original.position_bitboard,position.position_bitboard);
        }
    }

    #[test]
    pub fn test_quick_position(){
        let dimensions = Dimensions{width:8,height:8};
        let fen = "rnbqkbnr/1ppppppp/8/p7/P7/8/1PPPPPPP/RNBQKBNR w KQkq - 0 1";
        let mut position = fen::load_from_fen(fen.to_string());
        let mvgen = MoveGenerator::new(dimensions,position.get_jump_offets());
        let moves = generate_legal_moves(&mvgen,&mut position);
        for mv in moves{
            println!("{}",mv.to_algebraic_notation(8, Color::WHITE, &position.piece_collections[Color::WHITE as usize]));
        }
    }

}