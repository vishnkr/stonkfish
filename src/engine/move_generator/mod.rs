use crate::engine::moves::{Move};
use crate::engine::position;
use crate::engine::bitboard::*;
use std::vec::Vec;
use arrayvec::ArrayVec;
//Attack - Defend Map or Attack table
pub struct AttackMap{
    knight_masks: ArrayVec::<Bitboard,256>,
    bishop_masks:ArrayVec::<Bitboard,256>,
    rook_masks:ArrayVec::<Bitboard,256>,
    king_masks:ArrayVec::<Bitboard,256>
    //masked_knight_attacks: ArrayVec::<Bitboard,256>
}
pub struct MoveGenerator{
    at_map: AttackMap,
}
pub enum SlideDirs{
    North,
    South,
    East,
    West,
    NorthEast,
    SouthEast,
    NorthWest,
    SouthWest
}

impl AttackMap{
    pub fn new(dimensions:position::Dimensions)->Self{
        let mut knight_masks = ArrayVec::<Bitboard,256>::new();
        let mut rook_masks = ArrayVec::<Bitboard,256>::new();
        let mut bishop_masks = ArrayVec::<Bitboard,256>::new();
        let mut king_masks = ArrayVec::<Bitboard,256>::new();
        let width = dimensions.width as i8;
        let height = dimensions.height as i8;
        //mask knight moves
        let knight_offsets:[(i8,i8);8] = [(2,1),(2,-1),(-2,1),(-2,-1),(1,2),(1,-2),(-1,2),(-1,-2)];
        for i in 0..16{
            for j in 0..16{
                let square = to_pos(i, j);
                king_masks.push(Bitboard::zero());
                rook_masks.push(Bitboard::zero());
                bishop_masks.push(Bitboard::zero());
                knight_masks.push(Bitboard::zero());
                AttackMap::mask_slide(i as i8,j as i8,width,height,&mut rook_masks[square],&[SlideDirs::North,SlideDirs::South,SlideDirs::East,SlideDirs::West]);
                AttackMap::mask_slide(i as i8,j as i8,width,height,&mut bishop_masks[square],&[SlideDirs::NorthEast,SlideDirs::NorthWest,SlideDirs::SouthEast,SlideDirs::SouthWest]);
                
                for dir in knight_offsets{
                    let new_x = i as i8 +dir.0;
                    let new_y = j as i8 +dir.1;
                    if new_x>=0 && new_y>=0 && new_x<width && new_y<height{
                        knight_masks[square].set_bit(to_pos(new_x as u8,new_y as u8),true);
                    }
                }
            
                
            }
        }
        AttackMap{
            knight_masks,
            bishop_masks,
            rook_masks,
            king_masks
        }
    }

    pub fn mask_slide(x:i8,y:i8,width:i8,height:i8,bb:&mut Bitboard,dirs:&[SlideDirs]){
        let mut dx:i8;
        let mut dy:i8;
        for dir in dirs{
            match dir{
                SlideDirs::NorthEast=>{dx=-1;dy=1;}
                SlideDirs::SouthEast=>{dx=1;dy=1;}
                SlideDirs::NorthWest=>{dx=-1;dy=-1;}
                SlideDirs::SouthWest=>{dx=1;dy=-1;}
                SlideDirs::North=>{dx=-1;dy=0;}
                SlideDirs::East=>{dx=0;dy=1;}
                SlideDirs::South=>{dx=1;dy=0;}
                SlideDirs::West=>{dx=0;dy=-1;}
            }
            let mut newx = x+dx;
            let mut newy = y+dy;
            loop {
                if newx<0 || newy<0 || newx>width-1 || newy>height-1{break;}
                let square = to_pos(newx as u8, newy as u8);
                bb.set_bit(square,true);
                newx+=dx;
                newy+=dy;
            }
        }
    }
}

impl MoveGenerator{
    pub fn new()->MoveGenerator{
        MoveGenerator{at_map:AttackMap::new(position::Dimensions{width:10,height:10})}
    }

    pub fn generate_moves(&self,cur_position:&mut position::Position)->Vec<Move>{
        let moves = Vec::new();
        let _piece_set: &position::PieceSet = &cur_position.pieces[cur_position.turn as usize];
        moves
    }
}

#[cfg(test)]
mod movegen_tests{
    use super::*;
    use crate::engine::bitboard::{to_string};
    #[test]
    pub fn test_knight_attack_masks(){
        let at_map = AttackMap::new(position::Dimensions{width:10,height:10});
        to_string(&at_map.knight_masks[65]);
    }

    #[test]
    pub fn test_rook_attack_masks(){
        let at_map = AttackMap::new(position::Dimensions{width:7,height:7});
        to_string(&at_map.rook_masks[17]);
        to_string(&at_map.bishop_masks[16]);
    }

}