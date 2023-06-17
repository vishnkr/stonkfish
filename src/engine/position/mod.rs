use core::fmt;
use std::collections::HashMap;
use std::convert::TryInto;
use std::ops::Not;

use crate::engine::bitboard::{Bitboard,to_row,to_col};
use crate::engine::move_generation::moves::*;

use self::fen::{RADIX, load_from_fen};
use self::piece::{Piece,PieceType, PieceRepr};
use self::piece_collection::PieceCollection;
use self::zobrist::Zobrist;

pub mod zobrist;
pub mod piece;
pub mod piece_collection;
pub mod fen;
pub type JumpOffset = HashMap<PieceRepr, Vec<(i8, i8)>>;


impl PieceType{
    pub fn to_string(&self)->String{
        format!("{:?}",self)
    }
    pub fn as_vec()-> Vec<PieceType>{
        vec!(PieceType::Pawn,PieceType::Knight,PieceType::Bishop,PieceType::Rook,PieceType::Queen,PieceType::King)
    }
}

impl fmt::Debug for PieceType{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match *self {
            PieceType::Pawn => write!(f,"Pawn"),
            PieceType::Knight => write!(f,"Knight"),
            PieceType::Bishop => write!(f,"Bishop"),
            PieceType::Queen => write!(f,"Queen"),
            PieceType::Rook => write!(f,"Rook"),
            PieceType::King => write!(f,"King"),
            PieceType::Custom => write!(f,"Custom"),
        }
    }
}

#[derive(Copy, Clone,PartialEq,Debug,Eq,Hash)]
pub enum Color{
    WHITE,
    BLACK
}

impl Not for Color {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Color::BLACK => Color::WHITE,
            Color::WHITE => Color::BLACK
        }
    }
}

#[derive(Debug)]
pub struct Position{
    pub turn: Color,
    pub dimensions:Dimensions,
    ///idx -0: white set, idx-1: black set
    pub piece_collections: Vec<PieceCollection>,
    /// castling right is encoded as follows - white kingside (2 bits) | white queenside (2 bits) | black kingside (2 bits) | black queenside (2 bits)
    pub castling_rights: u8,
    pub has_king_moved: bool,
    // idx-0 : capturing piece, idx-1 : captured piece
    pub recent_capture: Option<(PieceRepr,PieceRepr)>,
    pub position_bitboard: Bitboard,
    pub zobrist_hash: Zobrist,
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.turn == other.turn &&
        self.dimensions == other.dimensions &&
        self.piece_collections == other.piece_collections &&
        self.castling_rights == other.castling_rights &&
        self.has_king_moved == other.has_king_moved &&
        self.position_bitboard == other.position_bitboard
    }
}


#[derive(Debug,PartialEq,Clone)]
pub struct Dimensions{
    pub height: u8,
    pub width: u8
}

impl Dimensions{
    pub fn is_pos_in_range(&self, pos:usize)->bool{
        let row = to_row(pos.try_into().unwrap());
        let col = to_col(pos.try_into().unwrap());
        col<=self.width && row<=self.height
    }
}


pub fn get_dimensions(fen_first_part:Vec<String>)-> Dimensions{
    let mut col_count = 0;
    let mut sec_digit = 0;
    let mut count;
    for (i,c) in fen_first_part[0].chars().enumerate(){
        if c.is_digit(RADIX){
            count = c.to_digit(RADIX).unwrap_or(0);
            if i+1<fen_first_part[0].len() && (fen_first_part[0].as_bytes()[i+1] as char).is_digit(RADIX){
                sec_digit = c.to_digit(RADIX).unwrap_or(0);
            } else{
                col_count+=sec_digit*10+count;
                sec_digit=0;
            }
        } else {col_count+=1}
    }
    Dimensions{width:fen_first_part.len() as u8, height: col_count as u8}
}

impl Position{

    pub fn new(fen:String)->Position{
        load_from_fen(fen)
    }

    pub fn get_opponent_position_bb(&self,color:Color)-> Bitboard{
        return &self.position_bitboard & !&self.piece_collections[color as usize].occupied;
    }

    pub fn make_move(&mut self,mv:&Move){
        let (src,dest,mtype) = (mv.get_src_square() as usize, mv.get_dest_square() as usize,mv.get_mtype().unwrap());

        
        match mtype{
            MType::Quiet =>{
                self.move_piece(self.turn, (src,dest));
            },
            MType::KingsideCastle => {
                self.move_piece(self.turn, (src,dest));
                
            },
            MType::QueensideCastle => {
                self.move_piece(self.turn, (src,dest));
            },
            MType::Capture =>{
                let opponent_color =!self.turn;
                let capturing_piece = self.piece_collections[self.turn as usize].get_piece_from_sq(src).unwrap();
                let captured_piece = self.piece_collections[opponent_color as usize].get_piece_from_sq(dest).unwrap();
                self.recent_capture = Some((capturing_piece.piece_repr,captured_piece.piece_repr));
                self.remove_piece_from(opponent_color,dest);
                self.move_piece(self.turn,(src,dest));
            },
            MType::Promote =>{},
            MType::EnPassant =>{},
            MType::None => {},
        }
        self.turn = !self.turn;
        self.update_occupied_bitboard();
        
    }

    pub fn unmake_move(&mut self,mv:&Move){
        let src:usize = mv.get_src_square() as usize;
        let dest:usize = mv.get_dest_square() as usize;
        let mtype = mv.get_mtype().unwrap_or(MType::None);
        self.turn = !self.turn;
        let my_color = self.turn;
        match mtype{
            MType::Quiet =>{
                self.move_piece(self.turn,(dest,src));
            },
            MType::KingsideCastle => {},
            MType::QueensideCastle => {},
            MType::Capture =>{
                let (_,captured_piece) = self.recent_capture.unwrap();
                self.move_piece(my_color,(dest,src));
                self.add_piece_at(!my_color, dest, captured_piece);
            },
            MType::Promote =>{},
            MType::EnPassant =>{},
            MType::None=>{}
        }
        self.update_occupied_bitboard();
        
    }

    pub fn remove_piece_from(&mut self,color:Color,sq:usize){
        if let Some(piece) = self.piece_collections[color as usize].get_mut_piece_from_sq(sq){
            self.position_bitboard.set_bit(sq,false);
            piece.bitboard.set_bit(sq,false);
        } else { println!("error removing piece at sq - {}",sq)}
    }

    pub fn add_piece_at(&mut self, color:Color, sq:usize, piece_repr: PieceRepr){
        if let Some(piece) = self.piece_collections[color as usize].pieces.get_mut(&piece_repr){
            piece.bitboard.set_bit(sq,true);
            self.position_bitboard.set_bit(sq,true);
        } else { println!("error adding piece at sq - {}",sq)}
    }

    pub fn update_occupied_bitboard(&mut self){
        let colors: [Color;2] = [Color::WHITE,Color::BLACK];
        for color in colors{
            let mut new_val = Bitboard::zero();
            for (_,piece) in self.piece_collections[color as usize].pieces.iter(){
                new_val |= &piece.bitboard;
            }
            self.piece_collections[color as usize].occupied = new_val;
        }
    }

    pub fn move_piece(&mut self,color:Color,from_to:(usize,usize)){
        let (src,dest) = from_to;
        if self.piece_collections[color as usize].has_piece_at(src){
            let piece:&mut Piece = self.piece_collections[color as usize].get_mut_piece_from_sq(src).unwrap();
            self.position_bitboard.set_bit(src,false);
            self.position_bitboard.set_bit(dest,true);
            piece.bitboard.set_bit(src,false);
            piece.bitboard.set_bit(dest,true);
        }
    }

    pub fn get_zobrist_hash(&self)-> u64{
        let mut zobrist_hash_key = 0u64;
        for (i,collection) in self.piece_collections.iter().enumerate(){
            for (_,piece) in collection.pieces.iter(){
                let mut bitboard = piece.bitboard.clone();
                while !bitboard.is_zero(){
                    let pos = bitboard.lowest_one().unwrap_or(0);
                    zobrist_hash_key ^= self.zobrist_hash.piece_keys[i][&piece.piece_type][pos];
                    bitboard.set_bit(pos,false);
                }
            }
        }
        if self.turn == Color::BLACK{
            zobrist_hash_key ^= self.zobrist_hash.black_to_move;
        }
        zobrist_hash_key
    }

    pub fn switch_turn(&mut self){ self.turn = !self.turn }

    pub fn valid_kingside_castle(&self)->bool{
        match self.turn{
            Color::BLACK=> ((self.castling_rights >> 6) & 1) == 1,
            Color::WHITE=> ((self.castling_rights >> 2)) & 1 == 1
        }
    }
    pub fn valid_queenside_castle(&self)->bool{
        match self.turn{
            Color::BLACK=> ((self.castling_rights >> 4) & 1) == 1,
            Color::WHITE=> ((self.castling_rights)) & 1 == 1
        }
    }

    pub fn get_jump_offets(&self)->Option<JumpOffset>{
        let mut jump_offsets:JumpOffset = HashMap::new();
        for pieceset in &self.piece_collections{
            for (repr,piece) in &pieceset.pieces{
                if piece.props.jump_offsets.len()>0{
                    if piece.piece_type==PieceType::Pawn{
                        jump_offsets.insert(*repr,piece.props.jump_offsets.clone());
                    } else {
                        if !jump_offsets.contains_key(&repr.to_ascii_lowercase()){
                            jump_offsets.insert(repr.to_ascii_lowercase(),piece.props.jump_offsets.clone());
                        }
                    }
                }
                
            }
        }
        if jump_offsets.len()==0{
            return None;
        }
        Some(jump_offsets)
    }
}


#[cfg(test)]
mod position_tests{
    use crate::engine::position::*;

    #[test]
    fn test_eq_zobrist_hash(){
        let pos1 = fen::load_from_fen("12/5p1k4/12/p2p1P6/5q6/P1PbN2p4/7P4/2Q3K5/12/12/12/12 b - - 1 44".to_string());
        let pos2 = fen::load_from_fen("12/5p1k4/12/p2p1P6/5q6/P1PbN2p4/7P4/2Q3K5/12/12/12/12 b - - 1 44".to_string());
        assert_eq!(pos1.get_zobrist_hash(),pos2.get_zobrist_hash())
    }

    #[test]
    fn test_unequal_zobrist_hash(){
        let pos1 = fen::load_from_fen("12/5p1k4/12/p2p1P6/5q6/P1PbN2p4/7P4/2Q3K5/12/12/12/12 b - - 1 44".to_string());
        let pos2 = fen::load_from_fen("12/5p1k4/12/p2p1P6/5q6/P1PbN2p4/7P4/2Q3K5/12/12/12/12 w - - 1 44".to_string());
        assert_ne!(pos1.get_zobrist_hash(),pos2.get_zobrist_hash())
    }

    #[test]
    pub fn test_make_move(){
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let mut position = load_from_fen(fen.to_string());
        let result_fen = "rnbqkbnr/pppppppp/8/8/8/3P4/PPP1PPPP/RNBQKBNR w KQkq - 0 1";
        let result = load_from_fen(result_fen.to_string());
        let mv = Move::encode_move(99, 83, MType::Quiet, None);
        position.make_move(&mv);
        assert_eq!(position.piece_collections,result.piece_collections);
    }
    #[test]
    pub fn test_make_move2(){
        let fen = "3r4/8/8/8/8/8/3R4/3K4 w - - 0 1";
        let mut position = load_from_fen(fen.to_string());
  
        let result = load_from_fen(fen.to_string());
        let mv = Move::encode_move(99, 83, MType::Quiet, None);
        position.make_move(&mv);
        assert_eq!(position.piece_collections,result.piece_collections);
    }
}

