use std::collections::HashMap;
use crate::engine::bitboard::Bitboard;
use super::MovePattern;



#[derive(Copy,Clone, PartialEq, Eq, Hash)]
pub enum PieceType{
    Pawn,
	Knight,
	Bishop,
	Rook,
	Queen,
	King,
    Custom
}

#[derive(Debug,PartialEq)]
pub struct Piece{
    pub piece_type:PieceType,
    pub bitboard: Bitboard,
    pub piece_repr: char,
    pub player:u8,
    // move_patterns set only for custom pieces
    pub move_patterns: Option<MovePattern>
}

impl Piece{
    pub fn new_piece(player:u8, repr:char) -> Self{
        let mut piece:Piece = Piece{bitboard:Bitboard::zero(),player,piece_repr:repr,piece_type:PieceType::Pawn, move_patterns:None};
        match repr{
            'p'=> piece.piece_type = PieceType::Pawn,
            'r'=> piece.piece_type = PieceType::Rook,
            'k'=> piece.piece_type = PieceType::King,
            'q'=> piece.piece_type = PieceType::Queen,
            'b'=> piece.piece_type = PieceType::Bishop,
            'n'=> piece.piece_type = PieceType::Knight,
            _=> piece.piece_type = PieceType::Custom,
        }
        piece
    }
}

#[derive(Debug,PartialEq)]
pub struct PieceSet{
    pub player:u8,
    pub king:Piece,
    pub queen:Piece,
    pub rook:Piece,
    pub bishop:Piece,
    pub knight:Piece,
    pub pawn:Piece,
    pub occupied: Bitboard,
    pub custom: HashMap<char,Piece>
}

impl PieceSet{
    pub fn new(player:u8)->Self{
        PieceSet{
            player:player,
            king: Piece::new_piece(player,'k'),
            queen: Piece::new_piece(player,'q'),
            rook: Piece::new_piece(player,'r'),
            bishop: Piece::new_piece(player,'b'),
            knight: Piece::new_piece(player,'n'),
            pawn: Piece::new_piece(player,'p'),
            occupied: Bitboard::zero(),
            custom: HashMap::new()
        }
    }
    pub fn as_array(&self) -> [&Piece; 6] {
        return [&self.king, &self.pawn, &self.bishop,&self.queen,&self.rook,&self.knight]
    }
    pub fn get_mut_piece_from_sq(&mut self,loc:usize)->Option<&mut Piece>{
        if self.pawn.bitboard.bit(loc).unwrap(){
            return Some(&mut self.pawn);
        } else if self.bishop.bitboard.bit(loc).unwrap(){
            return Some(&mut self.bishop);
        } else if self.rook.bitboard.bit(loc).unwrap(){
            return Some(&mut self.rook);
        } else if self.king.bitboard.bit(loc).unwrap(){
            return Some(&mut self.king);
        } else if self.queen.bitboard.bit(loc).unwrap(){
            return Some(&mut self.queen);
        } else if self.knight.bitboard.bit(loc).unwrap(){
            return Some(&mut self.knight);
        }  
        self.get_mut_custom_piece_from_sq(loc)
    }

    pub fn get_piece_from_sq(&self,loc:usize)->Option<&Piece>{
        if self.pawn.bitboard.bit(loc).unwrap(){
            return Some(&self.pawn);
        } else if self.bishop.bitboard.bit(loc).unwrap(){
            return Some(&self.bishop);
        } else if self.rook.bitboard.bit(loc).unwrap(){
            return Some(&self.rook);
        } else if self.king.bitboard.bit(loc).unwrap(){
            return Some(&self.king);
        } else if self.queen.bitboard.bit(loc).unwrap(){
            return Some(&self.queen);
        } else if self.knight.bitboard.bit(loc).unwrap(){
            return Some(&self.knight);
        }  
        self.get_custom_piece_from_sq(loc)
    }

    pub fn get_custom_piece_from_sq(&self, index:usize)->Option<&Piece>{
        for (_,piece) in self.custom.iter(){
            if piece.bitboard.bit(index).unwrap(){
                return Some(piece);
            }
        }
        None
    }

    pub fn get_mut_custom_piece_from_sq(&mut self, index:usize)->Option<&mut Piece>{
        for (_,piece) in self.custom.iter_mut(){
            if piece.bitboard.bit(index).unwrap(){
                return Some(piece);
            }
        }
        None
    }

    pub fn get_piece_from_piecetype(&mut self,piece_type:PieceType)->Option<&mut Piece>{
        match piece_type{
            PieceType::Bishop=> Some(&mut self.bishop),
            PieceType::Knight=> Some(&mut self.knight),
            PieceType::King=> Some(&mut self.king),
            PieceType::Queen=> Some(&mut self.queen),
            PieceType::Pawn=> Some(&mut self.pawn),
            PieceType::Custom=> None,
            PieceType::Rook=> Some(&mut self.rook),
        }
    }
}
