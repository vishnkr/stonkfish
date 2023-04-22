use crate::engine::bitboard::{Bitboard, to_pos};

use super::{piece_collection::PieceCollection, Dimensions, get_dimensions, Color, Position, piece::Piece, zobrist::Zobrist};

pub const RADIX: u32 = 10;

pub fn load_from_fen(fen:String) -> Position{
    let board_data:String = fen.split(" ").collect();
    let dimensions:Dimensions = get_dimensions(board_data.split("/").map(|s| s.to_string()).collect());
    let mut white_piece_set:PieceCollection = PieceCollection::new(Color::WHITE);
    let mut black_piece_set:PieceCollection = PieceCollection::new(Color::BLACK);
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
                    let bitboard: &mut Bitboard = match c.is_ascii_lowercase(){
                        true=>{
                            if !black_piece_set.pieces.contains_key(&c){
                                black_piece_set.pieces.insert(c, Piece::new_piece(Color::BLACK,c,&dimensions));
                            }
                            &mut black_piece_set.pieces.get_mut(&c).unwrap().bitboard
                        }
                        false=>{
                            if !white_piece_set.pieces.contains_key(&c){
                                white_piece_set.pieces.insert(c, Piece::new_piece(Color::WHITE,c.to_ascii_uppercase(),&dimensions));
                            }
                            &mut white_piece_set.pieces.get_mut(&c).unwrap().bitboard
                        }
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
        piece_collections:pieces,
        castling_rights,
        recent_capture: None,
        has_king_moved: false,
        position_bitboard,
        zobrist_hash : Zobrist::new()
    }
}

#[cfg(test)]
mod fen_tests{
    use crate::engine::position::*;

    #[test]
    fn test_fen_to_position_conversion(){
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let position = load_from_fen(fen.to_string());
    
        assert_eq!(position.turn, Color::WHITE);
        assert_eq!(position.dimensions.width, 8);
        assert_eq!(position.dimensions.height, 8);
        assert_eq!(position.position_bitboard.count_ones(), 32);
        assert_eq!(position.piece_collections[0].pieces.get(&'P').unwrap().bitboard.count_ones(), 8);
        assert_eq!(position.piece_collections[1].pieces.get(&'p').unwrap().bitboard.count_ones(), 8);
        assert_eq!(position.piece_collections[0].pieces.get(&'N').unwrap().bitboard.count_ones(), 2);
        assert_eq!(position.piece_collections[1].pieces.get(&'n').unwrap().bitboard.count_ones(), 2);
        assert_eq!(position.piece_collections[0].pieces.get(&'K').unwrap().bitboard.count_ones(), 1);
        assert_eq!(position.piece_collections[1].pieces.get(&'k').unwrap().bitboard.count_ones(), 1);
        assert_eq!(position.piece_collections[0].pieces.get(&'R').unwrap().bitboard.count_ones(), 2);
        assert_eq!(position.piece_collections[1].pieces.get(&'r').unwrap().bitboard.count_ones(), 2);
        assert_eq!(position.piece_collections[0].pieces.get(&'Q').unwrap().bitboard.count_ones(), 1);
        assert_eq!(position.piece_collections[1].pieces.get(&'q').unwrap().bitboard.count_ones(), 1);
        assert_eq!(position.piece_collections[0].pieces.get(&'B').unwrap().bitboard.count_ones(), 2);
        assert_eq!(position.piece_collections[1].pieces.get(&'b').unwrap().bitboard.count_ones(), 2);
    }

    #[test]
    fn test_fen_to_position_conversion_2(){
        let position = fen::load_from_fen("3r4/8/8/8/8/8/3R4/3K4 w - - 0 1".to_string());
        assert_eq!(position.position_bitboard.count_ones(), 3);
        assert_eq!(position.piece_collections[0].pieces.get(&'R').unwrap().bitboard.count_ones(), 1);
        assert_eq!(position.piece_collections[1].pieces.get(&'r').unwrap().bitboard.count_ones(), 1);
        assert_eq!(position.piece_collections[0].pieces.get(&'K').unwrap().bitboard.count_ones(), 1);
    }
}
