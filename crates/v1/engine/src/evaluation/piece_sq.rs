 
use arrayvec::ArrayVec;
use crate::{bitboard::{to_pos, to_row, to_col},position::{*, piece::PieceType}};
use std::iter::repeat;

type PieceSquareTable = ArrayVec<i32,256>;

pub struct PieceSquareTables{
    pub king: PieceSquareTable,
    pub bishop: PieceSquareTable,
    pub pawn: PieceSquareTable,
    pub rook: PieceSquareTable,
    pub knight: PieceSquareTable,
    pub queen: PieceSquareTable,

}

impl PieceSquareTables{
   
    pub fn new(start_pos: &Position)->Self{
        let rook_psqt = Self::setup_piece_sq_table(&start_pos.dimensions,PieceType::Rook);
        let pawn_psqt = Self::setup_pawn_pqst(start_pos); 
        return PieceSquareTables{
            king: PieceSquareTable::new(),
            bishop: PieceSquareTable::new(),
            pawn: pawn_psqt,
            rook: rook_psqt,
            knight:PieceSquareTable::new(),
            queen: PieceSquareTable::new(),
        }
    }

    pub fn compute_piece_square_table()-> Vec<i32>{
        vec![]
    }

    pub fn setup_pawn_pqst(start_pos:&Position)->PieceSquareTable{
        let width = start_pos.dimensions.width;
        let height = start_pos.dimensions.height;
        let mut pqst:PieceSquareTable = repeat(0).take(256).collect();
        let kingpos = start_pos.piece_collections[0].get_king().bitboard.lowest_one().unwrap() as u8;
        let kingrow = to_row(kingpos);
        let kingcol = to_col(kingpos);

        for rank in 0..height{
            for file in 0..width{
                let index = rank * width + file;

                // Encourage advancing pawns
                let rank_bonus = match rank {
                    r if r == 0 || r == height - 1 => 0,
                    r if r == 1 => 50,
                    r if r == height - 2 => 50,
                    r if r == 2 => 10 + (file as i32) * 10 / (width as i32),
                    r if r == height - 3 => 10 + (file as i32) * 10 / (width as i32),
                    r if r == 3 => 5 + (file as i32) * 5 / (width as i32),
                    r if r == height - 4 => 5 + (file as i32) * 5 / (width as i32),
                    _ => 0,
                };
    
                // Penalize unmoved central pawns
                let central_penalty = if file == width / 2 || file == width / 2 - 1 {
                    match rank {
                        0 => -10,
                        1 => -5,
                        _ => 0,
                    }
                } else {
                    0
                };
                pqst[to_pos(rank,file)] = rank_bonus + central_penalty;
            }
            
        }
        pqst


    }

    pub fn setup_piece_sq_table(dimensions:&Dimensions,piece_type:PieceType)->PieceSquareTable{
        let mut piece_sq_table:PieceSquareTable = PieceSquareTable::new(); 
        match piece_type{
            PieceType::Rook =>{
                // white rooks are stronger usually on the seventh rank/ rank before kings starting rank,
                // assuming most games start with the  king in the 1st and last rank
                let second_last_rank: u8 = dimensions.height-1;
                let mid: f32 = (dimensions.width /2).into();
                for rank in 0..dimensions.height{
                    for file in 0..dimensions.width{
                        let pos = to_pos(rank,file);
                        let value = match(rank,file){
                            (1,_)=>{
                                if dimensions.width%2==0 && (file == mid.ceil() as u8 || file== mid.floor() as u8){
                                    piece_sq_table.push(20);
                                }
                                piece_sq_table.push(15);
                            }
                            (second_last_rank , _)=>{
                                // needs re-work to assign values to files after castling (determined by king position)
                                if dimensions.width%2==0 && (file == mid.ceil() as u8 || file== mid.floor() as u8){
                                    piece_sq_table.push(20);
                                }
                                piece_sq_table.push(0);
                            }
                            _ => piece_sq_table.push(0),

                        };
                    }
                }
                
            },
            PieceType::Bishop=>{
                // avoid corners, preference to b3,b5,c4,d3 (or equivalent on variants boards) and central squares

            },
            PieceType::Pawn=>{
                
            }
            _ => {}
        }
        //display_piece_sq_table(&piece_sq_table, &dimensions);
        piece_sq_table
    }

}

pub fn display_piece_sq_table(psqt: &PieceSquareTable,dimensions:&Dimensions){
    let mut count = 0;
    for i in 0..(dimensions.width*dimensions.height){
        if count>=dimensions.width{
            count = 0;
            println!("");
        }
        print!("{} ",psqt[i as usize]);
        count+=1;
    }
}

#[cfg(test)]
mod psqt_tests{
    use super::*;
    #[test]
    pub fn test_psqt(){
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string();
        let position = Position::new(fen);
        let psqt =  PieceSquareTables::setup_pawn_pqst(&position); 
        display_piece_sq_table(&psqt, &position.dimensions)
    }
}