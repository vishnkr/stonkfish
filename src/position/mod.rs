
use crate::utils::bitboard;


enum PieceType{
    Pawn,
	Knight,
	Bishop,
	Rook,
	Queen,
	King,
    Custom
}

pub struct Piece{
    piece_type:PieceType,
    bitboard: &bitboard::Bitboard,
    piece_repr: String,
    player:u8
}

pub struct PieceSet{
    player:u8,
    king:&Piece,
    queen:&Piece,
    rook:&Piece,
    bishop:&Piece,
    knight:&Piece,
    pawn:&Piece,
    custom:Vec<&Piece>
}

pub struct Position{
    turn: u8,
    dimensions:&Dimensions,
    pieces: Vec<PieceSet>
}

pub struct Dimensions{
    height: u8,
    width: u8
}

const RADIX: u32 = 10;

pub fn get_dimensions(fen_first_part:Vec<&str>)-> Dimensions{
    let mut col_count:u8= 0;
    let mut sec_digit = 0;
    let mut row=0;
    for (i,c) in fen_first_part[0].chars().enumerate(){
        if c.is_digit(RADIX){
            if i+1<width && fen[i+1].is_digit(RADIX){
                sec_digit = c.to_digit(RADIX);
            } else{
                col_count+=sec_digit*10+count;
                sec_digit=0;
            }
        } else {col_count+=1}
    }
    Dimensions{width:fen_first_part.len() as u8, height: col_count}
}

impl Position{
    pub fn load_from_fen(fen:String) -> Position{
        let mut dimensions:Dimensions = get_dimensions((fen.split_whitespace()[0]).split('/'));
        let mut turn:u8 = 0;
        let mut fen_part = 0;
        let mut sec_digit = 0;
        let mut col=0;
        let mut row=0;
        for (i,c) in fen.chars().enumerate(){
            if c==" "{
                fen_part+=1;
            }
            match fen_part{
                0=>{
                    if c=="/"{
                        col=0;
                        row+=1;
                        sec_digit = 0;
                        continue;
                    } else if c.is_digit(RADIX){
                        if i+1<width && fen[i+1].is_digit(RADIX){
                            sec_digit = c.to_digit(RADIX);
                        }
                    }
                }
            }
        }
    }
}
