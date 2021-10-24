
use crate::utils::bitboard::{Bitboard,to_pos,to_string};


pub enum PieceType{
    Pawn,
	Knight,
	Bishop,
	Rook,
	Queen,
	King,
    Custom
}

pub struct Piece{
    pub piece_type:PieceType,
    pub bitboard: Bitboard,
    piece_repr: char,
    pub player:u8
}

impl Piece{
    pub fn new_pawn(player:u8)->Self{
        let piece_repr:char = 'p';
        let bitboard = Bitboard::zero();          
        Piece{piece_type:PieceType::Pawn,bitboard,piece_repr,player}
    }
    pub fn new_knight(player:u8)->Self{
        let piece_repr:char = 'n';
        let bitboard = Bitboard::zero();          
        Piece{piece_type:PieceType::Knight,bitboard,piece_repr,player}
    }
    pub fn new_bishop(player:u8)->Self{
        let piece_repr:char = 'b';
        let bitboard = Bitboard::zero();          
        Piece{piece_type:PieceType::Bishop,bitboard,piece_repr,player}
    }
    pub fn new_king(player:u8)->Self{
        let piece_repr:char = 'k';
        let bitboard = Bitboard::zero();          
        Piece{piece_type:PieceType::King,bitboard,piece_repr,player}
    }
    pub fn new_queen(player:u8)->Self{
        let piece_repr:char = 'q';
        let bitboard = Bitboard::zero();          
        Piece{piece_type:PieceType::Queen,bitboard,piece_repr,player}
    }
    pub fn new_rook(player:u8)->Self{
        let piece_repr:char = 'r';
        let bitboard = Bitboard::zero();          
        Piece{piece_type:PieceType::Rook,bitboard,piece_repr,player}
    }
}


pub struct PieceSet{
    pub player:u8,
    pub king:Piece,
    pub queen:Piece,
    pub rook:Piece,
    pub bishop:Piece,
    pub knight:Piece,
    pub pawn:Piece,
    //custom:Vec<Piece>
}

impl PieceSet{
    pub fn new(player:u8)->Self{
        PieceSet{
            player:player,
            king: Piece::new_king(player),
            queen: Piece::new_queen(player),
            rook: Piece::new_rook(player),
            bishop: Piece::new_bishop(player),
            knight: Piece::new_knight(player),
            pawn: Piece::new_pawn(player)
        }
    }
}

pub struct Position{
    pub turn: u8,
    pub dimensions:Dimensions,
    pub pieces: Vec<PieceSet>
}

#[derive(Debug,PartialEq)]
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

    pub fn new()->Position{
        Position::load_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string())
    }
    pub fn load_from_fen(fen:String) -> Position{
        println!("Erg {}",fen.split_whitespace().nth(0).as_deref().unwrap_or("nop"));
        let board_data:String = fen.split(" ").collect(); //_whitespace().nth(0).as_deref().unwrap_or("nop");
        let dimensions:Dimensions = get_dimensions(board_data.split("/").map(|s| s.to_string()).collect());
        let mut white_piece_set:PieceSet = PieceSet::new(super::WHITE);
        let mut black_piece_set:PieceSet = PieceSet::new(super::BLACK);
        let mut turn:u8 = 0;
        let mut fen_part = 0;
        let mut sec_digit = 0;
        let mut col:u8 = 0;
        let mut row = 0;
        let mut count:u32;
        let mut _castle_white_kingside = false;
        let mut _castle_white_queenside = false;
        let mut _castle_black_kingside = false;
        let mut _castle_black_queenside = false;
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
                        
                        let bitboard: &mut Bitboard = match c.to_ascii_lowercase(){
                            'p'=> if c.is_ascii_lowercase(){&mut white_piece_set.pawn.bitboard} else {&mut black_piece_set.pawn.bitboard}
                            'k'=> if c.is_ascii_lowercase(){&mut white_piece_set.king.bitboard} else {&mut black_piece_set.king.bitboard}
                            'b'=> if c.is_ascii_lowercase(){&mut white_piece_set.bishop.bitboard} else {&mut black_piece_set.bishop.bitboard}
                            'n'=> if c.is_ascii_lowercase(){&mut white_piece_set.knight.bitboard} else {&mut black_piece_set.knight.bitboard}
                            'r'=> if c.is_ascii_lowercase(){&mut white_piece_set.rook.bitboard} else {&mut black_piece_set.rook.bitboard}
                            'q'=> if c.is_ascii_lowercase(){&mut white_piece_set.queen.bitboard} else {&mut black_piece_set.queen.bitboard}
                            _=> continue
                        };
                        bitboard.set_bit(to_pos(row,col),true);
                        println!("piece- {}",c);
                        to_string(&bitboard);
                        col+=1
                    }
                }
                1=>{
                    if c=='w' {turn=0;}
                    else{turn=1;}
                }
                2=>{
                    match c {
                        'K'=> _castle_white_kingside=true,
                        'Q'=>_castle_white_queenside=true,
                        'k'=>_castle_black_kingside=true,
                        'q'=>_castle_black_queenside=true,
                        _ => {}
                    }
                }
                _ => continue,
            }
        }
        let mut pieces = Vec::new();
        pieces.push(white_piece_set);
        pieces.push(black_piece_set);
        Position{dimensions:dimensions,turn:turn,pieces:pieces}
    }
}


#[cfg(test)]
mod position_tests{
    use super::*;
    #[test]
    fn test_fen_to_position_conversion(){
        let fen:String = "12/5p1k4/12/p2p1P6/5q6/P1PbN2p4/7P4/2Q3K5/12/12/12/12 w - - 1 44".to_string();
        let dimensions:Dimensions = Dimensions{width:12,height:12};
        let turn:u8=0;
        let result: Position = Position::load_from_fen(fen);
        assert_eq!(result.dimensions,dimensions);
        assert_eq!(result.turn,turn);
    }
}