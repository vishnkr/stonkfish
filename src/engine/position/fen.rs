use crate::engine::bitboard::Bitboard;

use super::{piece_collection::PieceCollection, Dimensions, get_dimensions, Color, Position};

pub const RADIX: u32 = 10;

pub fn load_from_fen(fen:String) -> Position{
    let board_data:String = fen.split(" ").collect();
    let dimensions:Dimensions = get_dimensions(board_data.split("/").map(|s| s.to_string()).collect());
    let mut white_piece_set:PieceCollection = PieceCollection::new(Color::WHITE,&dimensions);
    let mut black_piece_set:PieceCollection = PieceCollection::new(Color::BLACK,&dimensions);
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
        position_bitboard,
        zobrist_hash : Zobrist::new()
    }
}