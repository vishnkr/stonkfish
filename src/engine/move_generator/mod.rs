use crate::engine::moves::{Move};
use crate::engine::position::{Color,PieceSet,Dimensions,Position};
use crate::engine::bitboard::*;
use std::vec::Vec;
use arrayvec::ArrayVec;
use std::ops::Index;
//Attack - Defend Map or Attack table
pub struct AttackMap{
    knight_attacks: ArrayVec::<Bitboard,256>,
    bishop_masks:ArrayVec::<Bitboard,256>,
    rook_masks:ArrayVec::<Bitboard,256>,
    king_attacks:ArrayVec::<Bitboard,256>,
    pawn_attacks:[ArrayVec::<Bitboard,256>;2],
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
    pub fn new(dimensions:Dimensions)->Self{
        let mut knight_attacks = ArrayVec::<Bitboard,256>::new();
        let mut rook_masks = ArrayVec::<Bitboard,256>::new();
        let mut bishop_masks = ArrayVec::<Bitboard,256>::new();
        let mut king_attacks = ArrayVec::<Bitboard,256>::new();
        let mut pawn_attacks = [ArrayVec::<Bitboard,256>::new(),ArrayVec::<Bitboard,256>::new()];
        let width = dimensions.width as i8;
        let height = dimensions.height as i8;
        
        //mask knight moves
        let knight_offsets:[(i8,i8);8] = [(2,1),(2,-1),(-2,1),(-2,-1),(1,2),(1,-2),(-1,2),(-1,-2)];
        let king_offsets:[(i8,i8);8] = [(0,1),(0,-1),(1,0),(-1,0),(1,-1),(-1,1),(1,1),(-1,-1)];
        for i in 0..16{
            for j in 0..16{
                let square = to_pos(i as u8, j as u8);
                king_attacks.push(Bitboard::zero());
                rook_masks.push(Bitboard::zero());
                bishop_masks.push(Bitboard::zero());
                knight_attacks.push(Bitboard::zero());
                pawn_attacks[Color::BLACK as usize].push(Bitboard::zero());
                pawn_attacks[Color::WHITE as usize].push(Bitboard::zero());
                AttackMap::mask_slide(i,j, width,height,&mut rook_masks[square],&[SlideDirs::North,SlideDirs::South,SlideDirs::East,SlideDirs::West]);
                AttackMap::mask_slide(i,j, width,height,&mut bishop_masks[square],&[SlideDirs::NorthEast,SlideDirs::NorthWest,SlideDirs::SouthEast,SlideDirs::SouthWest]);
                AttackMap::mask_jump(i,j, width, height, &knight_offsets, &mut knight_attacks[square]);
                AttackMap::mask_jump(i,j, width, height, &king_offsets, &mut king_attacks[square]);
                AttackMap::mask_pawn_attacks(i,j,width,height,&mut pawn_attacks);
            }
        }
        AttackMap{
            knight_attacks,
            bishop_masks,
            rook_masks,
            king_attacks,
            pawn_attacks,
        }
    }

    pub fn mask_jump(x:i8,y:i8,width:i8,height:i8,offsets:&[(i8,i8)],bb:&mut Bitboard){
        for dir in offsets{
            let new_x = x as i8 +dir.0;
            let new_y = y as i8 +dir.1;
            if new_x>=0 && new_y>=0 && new_x<width && new_y<height{
                bb.set_bit(to_pos(new_x as u8,new_y as u8),true);
            }
        }
    }

    pub fn mask_pawn_attacks(x:i8,y:i8,width:i8,height:i8,pawn_attacks:&mut [ArrayVec::<Bitboard,256>;2]){
        let square:usize = to_pos(x as u8,y as u8);
        if x+1<height && y+1<width{
            pawn_attacks[Color::BLACK as usize][square].set_bit(to_pos((x+1) as u8,(y+1) as u8),true);
        }
        if x+1<height && y-1>=0{
            pawn_attacks[Color::BLACK as usize][square].set_bit(to_pos((x+1) as u8,(y-1) as u8),true);
        }
        if x-1>=0 && y+1<width{
            pawn_attacks[Color::WHITE as usize][square].set_bit(to_pos((x-1) as u8,(y+1) as u8),true);
        }
        if x-1>=0 && y-1>=0{
            pawn_attacks[Color::WHITE as usize][square].set_bit(to_pos((x-1) as u8,(y-1) as u8),true);
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
        MoveGenerator{at_map:AttackMap::new(Dimensions{width:10,height:10})}
    }

    pub fn generate_moves(&self,cur_position:&mut Position)->Vec<Move>{
        let moves = Vec::new();
        let _piece_set: &PieceSet = &cur_position.pieces[cur_position.turn as usize];
        moves
    }
}

#[cfg(test)]
mod movegen_tests{
    use super::*;
    use crate::engine::bitboard::{to_string};
    #[test]
    pub fn test_knight_attack_masks(){
        let at_map = AttackMap::new(Dimensions{width:10,height:10});
        //to_string(&at_map.knight_attacks[65]);
        to_string(&at_map.pawn_attacks[1][18]);
        to_string(&at_map.pawn_attacks[0][22]);
    }

    #[test]
    pub fn test_rook_attack_masks(){
        let at_map = AttackMap::new(Dimensions{width:7,height:7});
        to_string(&at_map.rook_masks[17]);
        to_string(&at_map.bishop_masks[16]);
        to_string(&at_map.king_attacks[5]);
    }

}