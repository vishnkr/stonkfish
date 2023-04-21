use arrayvec::ArrayVec;
use moves::*;
use crate::engine::{
    bitboard::*,
    position::{*, piece::Piece},
};
use std::{vec::{Vec}, convert::TryInto, collections::HashMap};
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
    pub fn new(dimensions:Dimensions,jump_offsets:Option<Vec<JumpOffset>>)->Self{
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

    pub fn setup_jump_targets(jump_attack_table:&mut JumpAttackTable ,piece_info:&Vec<JumpOffset>){
        for (piece_repr,jump_offsets) in piece_info{
            Self::setup_jump_target(jump_attack_table,piece_repr,jump_offsets);
        }
    }

    pub fn setup_jump_target(jump_attack_table:&mut JumpAttackTable,piece_repr:&PieceRepr,offsets:&Vec<(i8,i8)>){
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
        jump_attack_table.insert(piece_repr.to_ascii_lowercase(), jump_targets_bitboards);
    }

    pub  fn get_all_attacks_bb(&self,piece:&Piece,src:u8,occupancy:&Bitboard)->Bitboard{
        let mut attack_bb = Bitboard::zero();
        attack_bb |= self.generate_slide_moves(src,&piece,occupancy);
        attack_bb |= self.generate_jump_moves(&piece,src);
        attack_bb
    }

    pub fn generate_pseudolegal_moves(&self,cur_position:&mut Position)->impl Iterator<Item=Move>{
        let color = cur_position.turn;
        let opponent_bb = cur_position.piece_collections[!color as usize].occupied.clone();
        let player_pieces = &mut cur_position.piece_collections[color as usize];
        
        let occupied_bb = &cur_position.position_bitboard;

        let mut moves = vec![];
        let _occ = &player_pieces.occupied;
        for (char,piece) in player_pieces.pieces.iter_mut(){
            
            let mut piece_bb = piece.bitboard.clone();
            while !piece_bb.is_zero(){
                let src = piece_bb.lowest_one().unwrap_or(0) as u8;
                let mut attack_bb = self.get_all_attacks_bb(piece, src, occupied_bb);
                display_bitboard_with_board_desc(&piece_bb, format!("piece - {}", char).as_str());
                display_bitboard_with_board_desc(&attack_bb, format!("piece attack_bb - {}", char).as_str());
                if piece.props.can_double_jump{
                    self.generate_double_jump_moves(&mut moves,&piece,src,occupied_bb);
                }
                piece_bb.set_bit(src as usize, false);
                attack_bb |= &self.size_dependent_bitboards.full_bitboard;
                flatten_bitboard(&mut attack_bb, &mut moves, &opponent_bb, piece.piece_type, src);
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
        final_bb
    }   

    pub fn generate_jump_moves(&self,piece:&Piece,square:u8)->Bitboard{
        let piece_repr = &piece.piece_repr;
        let mut final_bb = Bitboard::zero();
        println!("ja-table - {:#?}",self.jump_attack_table);
        match self.jump_attack_table.get(&piece_repr.to_ascii_lowercase()){
            Some(jump_offsets) =>{
                final_bb |= &jump_offsets[square as usize];
            },
            None =>{}//throw error }s
        }
        final_bb
    }

    pub fn generate_double_jump_moves(&self,moves:&mut Vec<Move>,piece:&Piece,src:u8,occupied:&Bitboard){
        let (src_row,src_col) = (to_row(src),to_col(src));
        if (piece.props.double_jump_squares.as_ref().unwrap()).contains(&src){
            for (dx,dy) in piece.props.jump_offsets.iter(){
                let (row,col) = ((dx+src_row as i8).try_into().unwrap(),(dy+src_col as i8).try_into().unwrap());
                let dest = to_pos(row,col);
                if row<self.dimensions.height 
                    && col<=self.dimensions.width 
                    && !occupied.bit(dest).unwrap(){
                    moves.push(Move::encode_move(src, dest as u8, MType::Quiet, None));
                }
                
            }
        }
    }
    /* 
    pub fn generate_pseudolegal_moves2(&self,cur_position:&mut Position)-> impl Iterator<Item=Move>{
        let mut move_masks :Vec<MoveMask> = Vec::new();
        let color = cur_position.turn;
        let opponent_bb =  &cur_position.position_bitboard & !&cur_position.pieces[color as usize].occupied;
        let player_bb = &cur_position.pieces[color as usize].occupied;
        let occupancy = &cur_position.position_bitboard;
        let king_pos = cur_position.pieces[color as usize].king.bitboard.lowest_one().unwrap() as u8;
        let opp_king_pos = cur_position.pieces[!color as usize].king.bitboard.lowest_one().unwrap();
        let mut moves = vec![];
        let mut gen_piece_moves_bb = |piece_type:PieceType,mut piece_bitboard:Bitboard|->(){
            while !piece_bitboard.is_zero(){
                let mut pos = piece_bitboard.lowest_one().unwrap_or(0) as u8;
                let mut attack_bb:Bitboard = match piece_type{
                    PieceType::King => self.attack_table.get_king_attacks(pos),
                    PieceType::Bishop => self.attack_table.get_bishop_attacks(pos,&occupancy),
                    PieceType::Rook => self.attack_table.get_rook_attacks(pos,&occupancy),
                    PieceType::Queen => self.attack_table.get_bishop_attacks(pos,&occupancy) | self.attack_table.get_rook_attacks(pos,&occupancy),
                    PieceType::Knight => self.attack_table.get_knight_attacks(pos),
                    PieceType::Pawn => {
                        if(!self.is_promotion_push(pos,color)){
                            self.attack_table.get_pawn_attacks_and_pushes(pos,cur_position.turn,&cur_position.dimensions,player_bb,&opponent_bb)
                        } else{ Bitboard::zero() }
                    },
                    _ => self.attack_table.get_knight_attacks(pos),
                };
                
                attack_bb &= !player_bb;
                attack_bb.set_bit(opp_king_pos, false);
                piece_bitboard.set_bit(pos.into(),false);
                attack_bb.set_bit(pos.into(),false);
                attack_bb &= &self.attack_table.full_bitboard;
                while !attack_bb.is_zero(){
                    let dest_pos = match attack_bb.lowest_one(){
                        Some(x) => x,
                        None=> {panic!("invalid attack pos");}
                    };
                    attack_bb.set_bit(dest_pos,false);
                    let mut mtype = MType::Quiet;
                    if opponent_bb.bit(dest_pos).unwrap(){
                        mtype = MType::Capture;
                    }
                    moves.push(Move::encode_move(pos, dest_pos as u8, mtype, None));
                }
            }
            
        };


        let piece_set = &cur_position.pieces[cur_position.turn as usize].as_array();
        for piece in piece_set{
            gen_piece_moves_bb(piece.piece_type,(&piece.bitboard).to_owned());
        }

        let mut non_quiet_moves:Vec<Move> = vec![];

        let mut pawn_bb_rank = cur_position.pieces[color as usize].pawn.bitboard.clone();
        // preserve only rank before promotion
        pawn_bb_rank &= match color {
                Color::BLACK => &self.attack_table.ranks[(self.dimensions.height-1) as usize],
                Color::WHITE=> &self.attack_table.ranks[1]
        };
        let possible_promos = [PieceType::Bishop,PieceType::Knight,PieceType::Rook,PieceType::Queen];
        while !pawn_bb_rank.is_zero(){
            let pos = pawn_bb_rank.lowest_one().unwrap();
            let row = to_row(pos as u8);
            let col = to_col(pos as u8);
            let targets : (usize,usize,usize);
            match color{
                Color::BLACK => {
                    if row!=self.dimensions.height-1 { continue }
                    targets = (to_pos(0,col-1),to_pos(0,col) ,to_pos(0,col+1));
                }
                Color::WHITE=> {
                    if row!=1 { continue }
                    targets  = (pos+15,pos+16,pos+17);
                }
            }
            for piece in possible_promos{
                if !opponent_bb.bit(targets.1).unwrap() && !occupancy.bit(targets.1).unwrap(){
                    non_quiet_moves.push(Move::encode_move(pos as u8,targets.1 as u8,MType::Promote,Some(PromoPieceType(piece))));
                }
                if self.dimensions.is_pos_in_range(targets.0) 
                   && opponent_bb.bit(targets.0).unwrap_or(false) 
                   && !cur_position.pieces[(!color) as usize].king.bitboard.bit(targets.0).unwrap_or(true){
                    non_quiet_moves.push(Move::encode_move(pos as u8,targets.0 as u8,MType::Promote,Some(PromoPieceType(piece))));
                }
                if self.dimensions.is_pos_in_range(targets.2)  && opponent_bb.bit(targets.2).unwrap_or(false) && !cur_position.pieces[!color as usize].king.bitboard.bit(targets.2).unwrap_or(true){
                    non_quiet_moves.push(Move::encode_move(pos as u8,targets.2 as u8,MType::Promote,Some(PromoPieceType(piece))));
                }
            }
            pawn_bb_rank.set_bit(pos.into(), false);
        }


        
        if cur_position.valid_kingside_castle(){
            // is rook in position, is path blocked and will king move into check after 1 move
            let target_rank = to_row(king_pos as u8);
            let target_rook_pos = (16*target_rank+1)-1;
            if let Some(ref piece) = cur_position.pieces[color as usize].get_mut_piece_from_sq(target_rook_pos.into()){
                if piece.piece_type == PieceType::Rook{
                    let mut rank_attack = Bitboard::from(get_rank_attacks(true, king_pos as u16));
                    rank_attack &= &cur_position.position_bitboard;
                    rank_attack.set_bit(target_rook_pos.into(), false);
                    let dest = to_pos(king_pos, king_pos+1) as u8;
                    if rank_attack.is_zero(){
                        let skipped_mv = Move::encode_move(king_pos, dest, MType::Quiet,None);
                        if self.is_legal_move(cur_position, &skipped_mv){
                            non_quiet_moves.push(Move::encode_move(king_pos, king_pos+2, MType::KingsideCastle,Some(AdditionalInfo::CastlingRookPos(target_rook_pos))));
                        }
                    }
                }
            }
        }

        if cur_position.valid_queenside_castle(){
            let target_rank = to_row(king_pos as u8);
            let target_rook_pos = 16*target_rank;
            if let Some(piece) = cur_position.pieces[color as usize].get_mut_piece_from_sq(target_rook_pos.into()){
                if piece.piece_type == PieceType::Rook{
                    let mut rank_attack = Bitboard::from(get_rank_attacks(false, king_pos as u16));
                    rank_attack &= &cur_position.position_bitboard;
                    rank_attack.set_bit(target_rook_pos.into(), false);
                    let dest = to_pos(king_pos, king_pos-1) as u8;
                    if rank_attack.is_zero(){
                        let skipped_mv = Move::encode_move(king_pos, dest, MType::Quiet,None);
                        if self.is_legal_move(cur_position, &skipped_mv){
                            non_quiet_moves.push(Move::encode_move(king_pos, king_pos-2, MType::QueensideCastle,Some(AdditionalInfo::CastlingRookPos(target_rook_pos))));
                        }
                    }
                }
            }
        }

        //move_masks.into_iter().flatten().into_iter().chain(non_quiet_moves.into_iter())
        moves.into_iter().chain(non_quiet_moves)
        
    }*/


    pub fn is_legal_move(&self,position:&mut Position, mv: &Move)->bool{
        position.make_move(mv);
        let mut is_under_check = false;
        
        if self.is_king_under_check(position){
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

    pub fn is_king_under_check(&self,position:&mut Position)-> bool{
        
        let color = position.turn.clone();
        let opponent_color = Position::get_opponent_color(position.turn);
        let mut opponent_bb =  &position.position_bitboard & !&position.piece_collections[color as usize].occupied;
        let pieces = &mut position.piece_collections;
        let occupancy = &position.position_bitboard;
        /* 
        while !opponent_bb.is_zero(){
                let mut pos = opponent_bb.lowest_one().unwrap_or(0) as u8;
                let piece = pieces[opponent_color as usize].get_mut_piece_from_sq(pos.into()).unwrap();
                let mut attack_bb = match piece.piece_type{
                    PieceType::King => self.attack_table.get_king_attacks(pos),
                    PieceType::Bishop => self.attack_table.get_bishop_attacks(pos,&occupancy),
                    PieceType::Rook => self.attack_table.get_rook_attacks(pos,&occupancy),
                    PieceType::Queen => self.attack_table.get_bishop_attacks(pos,&occupancy) | self.attack_table.get_rook_attacks(pos,&occupancy),
                    PieceType::Knight => self.attack_table.get_knight_attacks(pos),
                    PieceType::Pawn => {
                        let player_bb = &pieces[color as usize].occupied;
                        self.attack_table.get_pawn_attacks(pos,opponent_color,&opponent_bb)
                    },
                    _ => self.attack_table.get_knight_attacks(pos),
                };
                if !(attack_bb & &pieces[color as usize].king.bitboard).is_zero(){
                    return true
                }
                opponent_bb.set_bit(pos.into(),false);
        }*/
        while !opponent_bb.is_zero(){
            let src = opponent_bb.lowest_one().unwrap_or(0) as u8;
            let piece = pieces[opponent_color as usize].get_mut_piece_from_sq(src.into()).unwrap();
            let attack_bb = self.get_all_attacks_bb(piece, src, occupancy);
            if !(attack_bb & &pieces[color as usize].get_king_mut().bitboard).is_zero(){
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
        let mvgen = MoveGenerator::new(Dimensions { height:12, width:12 },Some(position.get_jump_offets()));
        let pos = 67;
        let row = to_row(pos) as i8;
        let col = to_col(pos) as i8;
        display_bitboard_with_board_desc(&(mvgen.attack_table.diagonals.get(&(col-row)).unwrap()), "Diagonal from pos");
        display_bitboard_with_board_desc(&(mvgen.attack_table.anti_diagonals.get(&(col+row)).unwrap()), "aNTI Diagonal from pos");
    }
    #[test]
    pub fn print_helper_test(){
        let dimensions = Dimensions{width:8,height:8};
        let mut position = fen::load_from_fen("3k4/8/8/8/1n3b2/P1P1P3/1PP3P1/3K4 w - - 0 1".to_string());
        let mvgen = MoveGenerator::new(dimensions,Some(position.get_jump_offets()));
        let val = mvgen.generate_pseudolegal_moves(&mut position);
        for mv in val{
            println!("{}",mv.to_algebraic_notation(8, Color::WHITE, &position.piece_collections[Color::WHITE as usize]));
        }
    }

    #[test]
    pub fn test_legal_movegen(){
        let dimensions = Dimensions{width:8,height:8};
        let mut position = fen::load_from_fen("3r4/8/8/8/8/8/3R4/3K4 w - - 0 1".to_string());
        let mvgen = MoveGenerator::new(dimensions,Some(position.get_jump_offets()));
        for mv in generate_legal_moves(&mvgen,&mut position){
            mv.display_move();
        }
        
    }

    #[test]
    pub fn test_unmake_move(){
        let dimensions = Dimensions{width:8,height:8};
        let position = &mut fen::load_from_fen("3k4/8/8/8/1n3b2/P1P1P3/1PP3P1/3K4 w - - 0 1".to_string());
        let mvgen = MoveGenerator::new(dimensions,Some(position.get_jump_offets()));
        let original = &fen::load_from_fen("3k4/8/8/8/1n3b2/P1P1P3/1PP3P1/3K4 w - - 0 1".to_string());
        for mv in mvgen.generate_pseudolegal_moves(position){
            position.make_move(&mv);
            position.unmake_move( &mv);
            assert_eq!(original.position_bitboard,position.position_bitboard);
        }
    }

}