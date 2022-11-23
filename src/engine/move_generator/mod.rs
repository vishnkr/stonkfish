use crate::engine::moves::{Move};
use crate::engine::position::*;
use crate::engine::bitboard::{Bitboard,to_pos,to_string};
use std::vec::Vec;
use arrayvec::ArrayVec;

//Attack - Defend Map or Attack table, precalculated for standard pieces
pub struct AttackTable{
    knight_attacks: ArrayVec::<Bitboard,256>,
    bishop_masks:ArrayVec::<Bitboard,256>,
    rook_masks:ArrayVec::<Bitboard,256>,
    king_attacks:ArrayVec::<Bitboard,256>,
    pawn_attacks:[ArrayVec::<Bitboard,256>;2],
    //masked_knight_attacks: ArrayVec::<Bitboard,256>
}
pub struct MoveGenerator{
    attack_table: AttackTable,
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


impl AttackTable{
    pub fn new(dimensions:Dimensions)->Self{
        let mut knight_attacks = ArrayVec::<Bitboard,256>::new();
        let mut rook_masks = ArrayVec::<Bitboard,256>::new();
        let mut bishop_masks = ArrayVec::<Bitboard,256>::new();
        let mut king_attacks = ArrayVec::<Bitboard,256>::new();
        let mut pawn_attacks = [ArrayVec::<Bitboard,256>::new(),ArrayVec::<Bitboard,256>::new()];
        let width = dimensions.width as i8;
        let height = dimensions.height as i8;
        
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
                Self::mask_slide(i,j, width,height,&mut rook_masks[square],&[SlideDirs::North,SlideDirs::South,SlideDirs::East,SlideDirs::West]);
                Self::mask_slide(i,j, width,height,&mut bishop_masks[square],&[SlideDirs::NorthEast,SlideDirs::NorthWest,SlideDirs::SouthEast,SlideDirs::SouthWest]);
                Self::mask_jump(i,j, width, height, &knight_offsets, &mut knight_attacks[square]);
                Self::mask_jump(i,j, width, height, &king_offsets, &mut king_attacks[square]);
                Self::mask_pawn_attacks(i,j,width,height,&mut pawn_attacks);
            }
        }
        Self {
            knight_attacks,
            bishop_masks,
            rook_masks,
            king_attacks,
            pawn_attacks,
        }
    }

    pub fn get_king_attacks(&self,position:usize)->Bitboard{
        self.king_attacks[position].to_owned()
    }
    pub fn get_knight_attacks(&self,position:usize)->Bitboard{
        self.knight_attacks[position].to_owned()
    }
    pub fn get_bishop_attacks(&self,position:usize)->Bitboard{
        self.bishop_masks[position].to_owned()
    }
    pub fn get_rook_attacks(&self,position:usize)->Bitboard{
        self.rook_masks[position].to_owned()
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
    pub fn new(dimensions:Dimensions)->MoveGenerator{
        Self{attack_table : AttackTable::new(Dimensions{width:dimensions.width,height:dimensions.height})}
    }

    pub fn generate_pseudolegal_moves(&self,cur_position:&mut Position)->Vec<Move>{
        let moves = Vec::new();
        let piece_set = &cur_position.pieces[cur_position.turn as usize].as_array();
        //&cur_position.pieces[cur_position.turn as usize];
        for x in 0..cur_position.dimensions.width{
            for y in 0..cur_position.dimensions.height{
                for piece in piece_set{
                    if piece.bitboard == Bitboard::zero() {continue}
                    let position:usize = piece.bitboard.lowest_one().unwrap() as usize;
                    let attack_bb: Bitboard = match piece.piece_type{
                        PieceType::King =>{self.attack_table.get_king_attacks(position)}
                        PieceType::Bishop =>{self.attack_table.get_bishop_attacks(position)}
                        PieceType::Rook =>{self.attack_table.get_rook_attacks(position)}
                        PieceType::Queen =>{self.attack_table.get_bishop_attacks(position) | self.attack_table.get_rook_attacks(position)}
                        PieceType::Knight =>{self.attack_table.get_knight_attacks(position)}
                        _ => self.attack_table.get_knight_attacks(position),
                    };
                    break;
                }
            }
        }
        
        moves
        
    }
}

#[cfg(test)]
mod movegen_tests{
    use super::*;
    use crate::engine::bitboard::{to_string};
    #[test]
    pub fn test_knight_attack_masks(){
        let at_map = AttackTable::new(Dimensions{width:10,height:10});
        //to_string(&at_map.knight_attacks[65]);
        to_string(&at_map.pawn_attacks[1][22]);
        to_string(&at_map.pawn_attacks[0][22]);
    }

    #[test]
    pub fn test_rook_attack_masks(){
        let at_map = AttackTable::new(Dimensions{width:7,height:7});
        to_string(&at_map.rook_masks[17]);
        to_string(&at_map.bishop_masks[16]);
        to_string(&at_map.king_attacks[5]);
    }

}