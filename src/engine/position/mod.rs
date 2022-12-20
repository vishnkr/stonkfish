use core::fmt;

use crate::engine::bitboard::{Bitboard,to_pos,display_bitboard};
use crate::engine::move_generator::moves::*;

use super::bitboard::display_bitboard_with_board_desc;

#[derive(Copy, Clone,PartialEq,Eq)]
pub enum PieceType{
    Pawn,
	Knight,
	Bishop,
	Rook,
	Queen,
	King,
    Custom,
}
impl PieceType{
    pub fn to_string(&self)->String{
        format!("{:?}",self)
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

#[derive(Copy, Clone,PartialEq,Debug)]
pub enum Color{
    WHITE,
    BLACK
}

#[derive(Debug,PartialEq)]
pub struct Piece{
    pub piece_type:PieceType,
    pub bitboard: Bitboard,
    pub piece_repr: char,
    pub player:u8
}
impl Piece{
    pub fn new_piece(player:u8, repr:char) -> Self{
        let mut piece:Piece = Piece{bitboard:Bitboard::zero(),player:0,piece_repr:repr,piece_type:PieceType::Custom};
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
    //custom:Vec<Piece>
    
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
        }
    }
    pub fn as_array(&self) -> [&Piece; 6] {
        return [&self.king, &self.pawn, &self.bishop,&self.queen,&self.rook,&self.knight]
    }
    pub fn get_piece_from_sq(&mut self,loc:usize)->Option<&mut Piece>{
        if self.pawn.bitboard.bit(loc).unwrap(){
            Some(&mut self.pawn)
        } else if self.bishop.bitboard.bit(loc).unwrap(){
            Some(&mut self.bishop)
        } else if self.rook.bitboard.bit(loc).unwrap(){
            Some(&mut self.rook)
        } else if self.king.bitboard.bit(loc).unwrap(){
            Some(&mut self.king)
        } else if self.queen.bitboard.bit(loc).unwrap(){
            Some(&mut self.queen)
        } else if self.knight.bitboard.bit(loc).unwrap(){
            Some(&mut self.knight)
        }  else{
            None
        }
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

#[derive(Debug)]
pub struct Position{
    pub turn: Color,
    pub dimensions:Dimensions,
    //ind-0: white set, ind-1: black set
    pub pieces: Vec<PieceSet>,
    // castling right is encoded as follows - white kingside (2 bits) | white queenside (2 bits) | black kingside (2 bits) | black queenside (2 bits)
    pub castling_rights: u8,
    pub has_king_moved: bool,
    pub recent_capture: Option<(PieceType,char)>,
    pub position_bitboard: Bitboard
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.turn == other.turn &&
        self.dimensions == other.dimensions &&
        self.pieces == other.pieces &&
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

const RADIX: u32 = 10;

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
        Position::load_from_fen(fen)
    }
    pub fn load_from_fen(fen:String) -> Position{
        let board_data:String = fen.split(" ").collect();
        let dimensions:Dimensions = get_dimensions(board_data.split("/").map(|s| s.to_string()).collect());
        let mut white_piece_set:PieceSet = PieceSet::new(Color::WHITE as u8);
        let mut black_piece_set:PieceSet = PieceSet::new(Color::BLACK as u8);
        let mut turn = Color::WHITE;
        let mut fen_part = 0;
        let mut sec_digit = 0;
        let mut col = 0;
        let mut row = 0;
        let mut count;
        let mut castling_rights:u8 = 0;
        for (i,c) in fen.chars().enumerate(){
            if c==' '{
                fen_part+=1;
            }
            match fen_part{
                0=>{
                    if c=='/'{
                        col=0;
                        row+=1;
                        sec_digit = 0;
                        continue;
                    } else if c.is_digit(RADIX){
                        count = c.to_digit(RADIX).unwrap_or(0);
                        if i+1<dimensions.width.into() && (fen.as_bytes()[i+1] as char).is_digit(RADIX){
                            sec_digit = c.to_digit(RADIX).unwrap_or(0);
                        } else {
                            col+=(sec_digit*10+count) as u8;
                            sec_digit=0;
                        }
                    } else {
                        let all_pieces_bb: &mut Bitboard = if c.is_ascii_lowercase(){&mut black_piece_set.occupied} else {&mut white_piece_set.occupied};
                        let bitboard: &mut Bitboard = match c.to_ascii_lowercase(){
                            'p'=> if c.is_ascii_lowercase(){&mut black_piece_set.pawn.bitboard} else {&mut white_piece_set.pawn.bitboard}
                            'k'=> if c.is_ascii_lowercase(){&mut black_piece_set.king.bitboard} else {&mut white_piece_set.king.bitboard}
                            'b'=> if c.is_ascii_lowercase(){&mut black_piece_set.bishop.bitboard} else {&mut white_piece_set.bishop.bitboard}
                            'n'=> if c.is_ascii_lowercase(){&mut black_piece_set.knight.bitboard} else {&mut white_piece_set.knight.bitboard}
                            'r'=> if c.is_ascii_lowercase(){&mut black_piece_set.rook.bitboard} else {&mut white_piece_set.rook.bitboard}
                            'q'=> if c.is_ascii_lowercase(){&mut black_piece_set.queen.bitboard} else {&mut white_piece_set.queen.bitboard}
                            _=> continue
                        };
                        let pos = to_pos(row,col);
                        bitboard.set_bit(pos,true);
                        all_pieces_bb.set_bit(pos,true);
                        col+=1
                    }
                }
                1=>{
                    if c=='w' {turn=Color::WHITE;}
                    else{turn=Color::BLACK;}
                }
                2=>{
                    castling_rights |= match c {
                        'K'=>  1<<6,
                        'Q'=> 1<<4,
                        'k'=> 1<<2,
                        'q'=> 1,
                        _ => 0
                    }
                }
                _ => continue,
            }
        }
        let mut pieces = Vec::new();
        let position_bitboard = Bitboard::zero() | &white_piece_set.occupied | &black_piece_set.occupied;
        pieces.push(white_piece_set);
        pieces.push(black_piece_set);
        Position{
            dimensions,
            turn,
            pieces,
            castling_rights,
            recent_capture: None,
            has_king_moved: false,
            position_bitboard 
        }
    }

    pub fn get_opponent_position_bb(&self,color:Color)-> Bitboard{
        return &self.position_bitboard & !&self.pieces[color as usize].occupied;
    }

    pub fn make_move(&mut self,color:Color,mv:&Move){
        let src:usize = mv.parse_from() as usize;
        let dest:usize = mv.parse_to() as usize;
        let mtype = mv.parse_mtype().unwrap();
        //let piece:&mut Piece = self.pieces[color as usize].get_piece_from_sq(src).unwrap();
        match mtype{
            MType::Quiet =>{
                self.move_piece(color, (src,dest));
            },
            MType::KingsideCastle => {},
            MType::QueensideCastle => {},
            MType::Capture =>{
                let mut opponent_color = Color::WHITE;
                if color == Color::WHITE{ opponent_color = Color::BLACK }
                let captured_piece = self.pieces[opponent_color as usize].get_piece_from_sq(dest).unwrap();
                self.recent_capture = Some((captured_piece.piece_type,captured_piece.piece_repr));
                self.remove_piece(opponent_color,dest);
                self.move_piece(color,(src,dest));
            },
            MType::Promote =>{},
            MType::EnPassant =>{}
        }
        self.update_occupied_bitboard();
        
    }

    pub fn remove_piece(&mut self,color:Color,sq:usize){
        let piece:&mut Piece = self.pieces[color as usize].get_piece_from_sq(sq).unwrap();
        self.position_bitboard.set_bit(sq,false);
        piece.bitboard.set_bit(sq,false);
    }
    
    pub fn undo_remove_piece(&mut self,color:Color,sq:usize, piece_type:PieceType){
        let piece:&mut Piece = self.pieces[color as usize].get_piece_from_piecetype(piece_type).unwrap();
        self.position_bitboard.set_bit(sq,true);
        piece.bitboard.set_bit(sq,true);
    }

    pub fn update_occupied_bitboard(&mut self){
        const colors: [Color;2] = [Color::WHITE,Color::BLACK];
        for color in colors{
            let mut new_val = Bitboard::zero();
            for piece in self.pieces[color as usize].as_array(){
                new_val |= &piece.bitboard;
            }
            self.pieces[color as usize].occupied = new_val;
        }
    }

    pub fn move_piece(&mut self,color:Color,from_to:(usize,usize)){
        let src = from_to.0;
        let dest = from_to.1;
        let piece:&mut Piece = self.pieces[color as usize].get_piece_from_sq(src).unwrap();
        self.position_bitboard.set_bit(src,false);
        self.position_bitboard.set_bit(dest,true);
        piece.bitboard.set_bit(dest,true);
        piece.bitboard.set_bit(src,false);
    }

    pub fn unmake_move(&mut self,color:Color,mv:&Move){
        let src:usize = mv.parse_from() as usize;
        let dest:usize = mv.parse_to() as usize;
        let mtype = mv.parse_mtype().unwrap_or(MType::Quiet);
        //let piece:&mut Piece = self.pieces[color as usize].get_piece_from_sq(dest).unwrap();
        
        match mtype{
            MType::Quiet =>{
                self.move_piece(color, (dest,src));
            },
            MType::KingsideCastle => {},
            MType::QueensideCastle => {},
            MType::Capture =>{
                let opponent_color:Color = self.get_opponent_color(color);
                let captured_piece = self.recent_capture;
                self.undo_remove_piece(opponent_color,dest, captured_piece.unwrap().0);
                let piece:&mut Piece = self.pieces[color as usize].get_piece_from_sq(dest).unwrap();
                self.position_bitboard.set_bit(src,true);
                piece.bitboard.set_bit(src,true);
                piece.bitboard.set_bit(dest,false);
            },
            MType::Promote =>{},
            MType::EnPassant =>{}
        }
        self.update_occupied_bitboard()
    }

    pub fn get_opponent_color(&self,color:Color)->Color{
        if color == Color::WHITE{
            return Color::BLACK
        }
        Color::WHITE
    }
}


#[cfg(test)]
mod position_tests{
    use super::*;
    #[test]
    fn test_fen_to_position_conversion(){
        let fen:String = "12/5p1k4/12/p2p1P6/5q6/P1PbN2p4/7P4/2Q3K5/12/12/12/12 w - - 1 44".to_string();
        let dimensions:Dimensions = Dimensions{width:12,height:12};
        let turn:Color= Color::WHITE;
        let result: Position = Position::load_from_fen(fen);
        assert_eq!(result.dimensions,dimensions);
        assert_eq!(result.turn,turn);
    }
}
