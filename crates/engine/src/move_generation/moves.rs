
use std::{fmt, convert::TryInto};
use crate::{bitboard::*, position::{piece::{PieceType}, Color, piece_collection::PieceCollection}};

#[derive(PartialEq,Copy,Clone)]
pub struct Move(u32);

#[derive(PartialEq)]
pub enum MType{
    Quiet,
    Capture,
    Promote,
    // refers to castling on the right since custom positions could have king placed wherever
    KingsideCastle,
    // refers to castling on the left
    QueensideCastle,
    EnPassant,
    PromotionCapture,
    None
}

pub enum AdditionalInfo {
    PromoPieceType(PieceType),
    CastlingRookPos(u8)
}

impl fmt::Display for MType{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result{
        match self{
            MType::Quiet => write!(f, "Quiet"),
            MType::Capture => write!(f, "Capture"),
            MType::Promote => write!(f, "Promote"),
            MType::KingsideCastle => write!(f, "KS Castle"),
            MType::QueensideCastle => write!(f, "QS Castle"),
            MType::EnPassant => write!(f, "En Passant"),
            MType::PromotionCapture => write!(f, "Promotion Capture"),
            MType::None => write!(f, "Invalid move"),
        }
    }
}

impl fmt::Debug for MType{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result{
        match self{
            MType::Quiet => write!(f, "Quiet"),
            MType::Capture => write!(f, "Capture"),
            MType::Promote => write!(f, "Promote"),
            MType::KingsideCastle => write!(f, "KS Castle"),
            MType::QueensideCastle => write!(f, "QS Castle"),
            MType::EnPassant => write!(f, "En Passant"),
            MType::PromotionCapture => write!(f, "Promotion Capture"),
            MType::None => write!(f, "Invalid move"),
        }
    }
}

//encoding based on chessprogramming wiki - https://www.chessprogramming.org/Encoding_Moves

impl Move{
    pub fn encode_move(src:u8,dest:u8,mtype:MType,additional_info: Option<AdditionalInfo>)->Move{
        let mut value  =
            (((0 | (src as u32))<< 16u32) | (dest as u32) << 8u32)| 
            match mtype {
                MType::Quiet => 0,
                MType::Capture => 1u32,
                MType::Promote => 2u32,
                MType::KingsideCastle => 3u32,
                MType::QueensideCastle => 4u32,
                MType::EnPassant => 5u32,
                MType::PromotionCapture => 6u32,
                MType::None => 7u32
            };
            if let Some(additional_info) = additional_info {
                match additional_info {
                    AdditionalInfo::PromoPieceType(c) => {
                        let mut promo_value = c as u32;
                        promo_value = (promo_value << 24) & 0xff000000;
                        value |= promo_value;
                    },
                    AdditionalInfo::CastlingRookPos(pos) => {
                        let mut pos_value = pos as u32;
                        pos_value = (pos_value << 24) & 0xff000000;
                        value |= pos_value;
                    },
                }
            }
        Move(value)
    }

    pub fn display_move(&self){
        println!("{}",format!("{:b}", self));
    }
    pub fn get_src_square(&self)->u8{
        ((self.0 >>16) & 0xFF) as u8
    }
    pub fn get_dest_square(&self)->u8{
        ((self.0 >>8) & 0xFF) as u8
    }
    pub fn get_mtype(&self)->Option<MType>{
        let mtype = (self.0 & 0xFF) as u32;
        match mtype {
            0 => Some(MType::Quiet),
            1u32 => Some(MType::Capture),
            2u32 => Some(MType::Promote),
            3u32 => Some(MType::KingsideCastle),
            4u32 => Some(MType::QueensideCastle),
            5u32 => Some(MType::EnPassant),
            _ => Some(MType::None)
        }
    }

    pub fn to_algebraic_notation(&self,rows:u8,color:Color, piece_collections: &PieceCollection)->String{
        let (src,dest,mtype) = (self.get_src_square(), self.get_dest_square(),self.get_mtype().unwrap());
        let piece = piece_collections.get_piece_from_sq(src.into()).unwrap();
        if piece.piece_type == PieceType::Pawn{
            match mtype{
                MType::Quiet=>{ format!("{}{}",sq_to_notation(src,rows),sq_to_notation(dest,rows)) },
                MType::Capture=>{format!("{}x{}",sq_to_notation(src,rows),sq_to_notation(dest,rows))},
                _=> "".to_string()
            }

        } else {
            let mut piece_type_str = piece.piece_repr.to_string();
            if color == Color::WHITE{ piece_type_str = piece_type_str.to_ascii_uppercase()}
            match mtype{
                MType::Quiet=>{ format!("{}{}{}",piece_type_str,sq_to_notation(src,rows),sq_to_notation(dest,rows)) },
                MType::KingsideCastle => "O-O".to_string(),
                MType::QueensideCastle => "O-O-O".to_string(),
                MType::Capture=>{format!("{}{}x{}",piece_type_str,sq_to_notation(src,rows),sq_to_notation(dest,rows))},
                //capture promos will be formated as promo for now
                MType::Promote =>{format!("{}{}={}",piece_type_str,sq_to_notation(src,rows),sq_to_notation(dest,rows))},
                _=> "".to_string()
            }
        }
    }
}

pub fn flatten_bitboard(bitboard:&mut Bitboard,moves:&mut Vec<Move>,opponent_bb:&Bitboard,_piece_type:PieceType,src:u8){
    while !bitboard.is_zero(){
        let dest = bitboard.lowest_one().unwrap();
        bitboard.set_bit(dest, false);
        let mut mtype = MType::Quiet;
        if opponent_bb.bit(dest).unwrap(){
            mtype = MType::Capture;
        }
        moves.push(Move::encode_move(src, dest.try_into().unwrap(), mtype, None));
    }
}

impl fmt::Binary for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"{:#032b}",self.0)
    }
}

impl fmt::Debug for Move{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result{
        let mtype = self.get_mtype().unwrap().to_string();
        let dest_pos = self.get_dest_square();
        let src_pos = self.get_src_square();
        write!(f,"Move {} from {} to {}",mtype,src_pos,dest_pos)
    }
}

impl fmt::Display for Move{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result{
        let mtype = self.get_mtype().unwrap().to_string();
        let dest_pos = self.get_dest_square();
        let src_pos = self.get_src_square();
        let src_coords = (to_row(src_pos as u8),to_col(src_pos as u8));
        let dest_coords = (to_row(dest_pos as u8),to_col(dest_pos as u8));
        write!(f," {} from {} (row-{}, col-{}) to {} (row-{}, col-{})",mtype,src_pos,src_coords.0,src_coords.1,dest_pos,dest_coords.0,dest_coords.1)
    }
}