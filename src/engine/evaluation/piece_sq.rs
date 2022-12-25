use arrayvec::ArrayVec;
use crate::engine::position::*;
use crate::engine::bitboard::{to_pos};

type PieceSquareTable = ArrayVec<i8,256>;

pub struct PieceSquareTables{
    pub king: PieceSquareTable,
    pub bishop: PieceSquareTable,
    pub pawn: PieceSquareTable,
    pub rook: PieceSquareTable,
    pub knight: PieceSquareTable,
    pub queen: PieceSquareTable,

}

impl PieceSquareTables{
    
    pub fn new(dimensions:Dimensions){
        let rook_psqt = Self::setup_piece_sq_table(dimensions,PieceType::Rook);

    }

    pub fn setup_piece_sq_table(dimensions:Dimensions,piece_type:PieceType)->PieceSquareTable{
        let mut piece_sq_table:PieceSquareTable = PieceSquareTable::new(); //with_capacity(dimensions.width * dimensions.height);
        match piece_type{
            PieceType::Rook =>{
                // white rooks are stronger usually on the seventh rank/ rank before kings starting rank,
                // assuming most games start with the  king in the 1st and last rank
                let second_last_rank: u8 = dimensions.height-1;
                let mid: f32 = (dimensions.width /2).into();
                for rank in 0..dimensions.height{
                    for file in 0..dimensions.width{
                        let pos = to_pos(rank,file);
                        println!("{} {} {}",piece_sq_table.len(),rank,file);
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

            }
            _ => {}
        }
        display_piece_sq_table(&piece_sq_table, dimensions);
        piece_sq_table
    }
}

pub fn display_piece_sq_table(psqt: &PieceSquareTable,dimensions:Dimensions){
    let mut count = 0;
    for i in 0..256{
        if count>=16{
            count = 0;
        }
        print!("{} ",psqt[i]);
        count+=1;
    }
}

#[cfg(test)]
mod psqt_tests{
    use super::*;
    #[test]
    pub fn test_rook_psqt(){
        let psqt = PieceSquareTables::new(Dimensions{width:16,height:16});
    }
}