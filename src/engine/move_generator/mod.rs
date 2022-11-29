use moves::*;
use crate::engine::position::*;
use crate::engine::bitboard::{Bitboard,to_pos,to_string};
use std::vec::Vec;
use arrayvec::ArrayVec;
use std::collections::HashMap;
use self::move_mask::MoveMask;
pub mod moves;
pub mod move_mask;
//Attack - Defend Map or Attack table, precalculated for standard pieces
pub struct AttackTable{
    knight_attacks: ArrayVec::<Bitboard,256>,
    king_attacks:ArrayVec::<Bitboard,256>,
    pawn_attacks:[ArrayVec::<Bitboard,256>;2],
    slide_attacks:HashMap<SlideDirection,ArrayVec::<Bitboard,256>>
}
pub struct MoveGenerator{
    attack_table: AttackTable,
}

#[derive(PartialEq,Eq,Hash,Copy,Clone)]
pub enum SlideDirection{
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
        let dirs = vec![SlideDirection::East,SlideDirection::North,SlideDirection::West,
            SlideDirection::South,SlideDirection::NorthEast,SlideDirection::NorthWest,
            SlideDirection::SouthEast,SlideDirection::SouthWest];
        let mut knight_attacks = ArrayVec::<Bitboard,256>::new();
        let mut king_attacks = ArrayVec::<Bitboard,256>::new();
        let mut pawn_attacks = [ArrayVec::<Bitboard,256>::new(),ArrayVec::<Bitboard,256>::new()];
        let mut slide_attacks:HashMap<SlideDirection, ArrayVec::<Bitboard,256>> = Self::init_slide_hashmap(&dirs);
        let width = dimensions.width as i8;
        let height = dimensions.height as i8;
        
        let knight_offsets:[(i8,i8);8] = [(2,1),(2,-1),(-2,1),(-2,-1),(1,2),(1,-2),(-1,2),(-1,-2)];
        let king_offsets:[(i8,i8);8] = [(0,1),(0,-1),(1,0),(-1,0),(1,-1),(-1,1),(1,1),(-1,-1)];
        for i in 0..16{
            for j in 0..16{
                let square = to_pos(i as u8, j as u8);
                king_attacks.push(Bitboard::zero());
                //rook_masks.push(Bitboard::zero());
                //bishop_masks.push(Bitboard::zero());
                knight_attacks.push(Bitboard::zero());
                pawn_attacks[Color::BLACK as usize].push(Bitboard::zero());
                pawn_attacks[Color::WHITE as usize].push(Bitboard::zero());
                //Self::mask_slides(i,j, width,height,&mut rook_masks[square],&[SlideDirection::North,SlideDirection::South,SlideDirection::East,SlideDirection::West]);
                //Self::mask_slides(i,j, width,height,&mut bishop_masks[square],&[SlideDirection::NorthEast,SlideDirection::NorthWest,SlideDirection::SouthEast,SlideDirection::SouthWest]);
                Self::mask_slide(i, j,width,height,&dirs,&mut slide_attacks,square);
                Self::mask_jump(i,j, width, height, &knight_offsets, &mut knight_attacks[square]);
                Self::mask_jump(i,j, width, height, &king_offsets, &mut king_attacks[square]);
                Self::mask_pawn_attacks(i,j,width,height,&mut pawn_attacks);
            }
        }
        Self {
            knight_attacks,
            king_attacks,
            pawn_attacks,
            slide_attacks
        }
    }

    pub fn get_king_attacks(&self,position:usize)->Bitboard{
        self.king_attacks[position].to_owned()
    }
    pub fn get_knight_attacks(&self,position:usize)->Bitboard{
        self.knight_attacks[position].to_owned()
    }
    pub fn get_bishop_attacks(&self,position:usize)->Bitboard{
        let dirs = [SlideDirection::NorthWest,SlideDirection::SouthWest,SlideDirection::NorthEast,SlideDirection::SouthEast];
        let mut bb = Bitboard::zero();
        for dir in dirs{
            bb |= &self.slide_attacks[&dir][position];
        }
        bb
    }
    pub fn get_rook_attacks(&self,position:usize)->Bitboard{
        let dirs = [SlideDirection::North,SlideDirection::South,SlideDirection::East,SlideDirection::West];
        let mut bb = Bitboard::zero();
        for dir in dirs{
            bb |= &self.slide_attacks[&dir][position];
        }
        bb
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

    pub fn init_slide_hashmap(dirs:&Vec<SlideDirection>)->HashMap<SlideDirection,ArrayVec::<Bitboard,256>>{
        let mut hashmap:HashMap<SlideDirection,ArrayVec::<Bitboard,256>> = HashMap::new();
        for dir in dirs{
            let mut bb_array = ArrayVec::<Bitboard,256>::new();
            for _ in 0..256{
                bb_array.push(Bitboard::zero());
            }
            hashmap.insert(*dir,bb_array);
        }
        hashmap
    }
    pub fn mask_slide(x:i8,y:i8,width:i8,height:i8,dirs:&Vec<SlideDirection>,hashmap:&mut HashMap<SlideDirection,ArrayVec::<Bitboard,256>>,square:usize){
        let mut dx:i8;
        let mut dy:i8;
        for dir in dirs{
            let bb_array = hashmap.get_mut(dir).unwrap();

            let bb: &mut Bitboard = &mut bb_array[square];
            match dir{
                SlideDirection::NorthEast=>{dx=-1;dy=1;}
                SlideDirection::SouthEast=>{dx=1;dy=1;}
                SlideDirection::NorthWest=>{dx=-1;dy=-1;}
                SlideDirection::SouthWest=>{dx=1;dy=-1;}
                SlideDirection::North=>{dx=-1;dy=0;}
                SlideDirection::East=>{dx=0;dy=1;}
                SlideDirection::South=>{dx=1;dy=0;}
                SlideDirection::West=>{dx=0;dy=-1;}
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
            to_string(&bb);

        }
    }

    pub fn mask_slides(x:i8,y:i8,width:i8,height:i8,bb:&mut Bitboard,dirs:&[SlideDirection]){
        let mut dx:i8;
        let mut dy:i8;
        for dir in dirs{
            match dir{
                SlideDirection::NorthEast=>{dx=-1;dy=1;}
                SlideDirection::SouthEast=>{dx=1;dy=1;}
                SlideDirection::NorthWest=>{dx=-1;dy=-1;}
                SlideDirection::SouthWest=>{dx=1;dy=-1;}
                SlideDirection::North=>{dx=-1;dy=0;}
                SlideDirection::East=>{dx=0;dy=1;}
                SlideDirection::South=>{dx=1;dy=0;}
                SlideDirection::West=>{dx=0;dy=-1;}
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
    pub fn new(dimensions:Dimensions)->Self{
        Self{attack_table : AttackTable::new(Dimensions{width:dimensions.width,height:dimensions.height})}
    }

    pub fn generate_pseudolegal_moves(&self,cur_position:&mut Position)->Vec<Move>{
        let moves = Vec::new();
        let piece_set = &cur_position.pieces[cur_position.turn as usize].as_array();
        let opponent_bb: Bitboard = cur_position.get_opponent_position_bb();

        let gen_piece_moves_bb = |piece_type:PieceType,mut piece_bitboard:Bitboard|->MoveMask {
            let mut final_bb = Bitboard::zero();
            let mut pos:usize = 0;
            while !piece_bitboard.is_zero(){
                pos = piece_bitboard.lowest_one().unwrap();
                let mut attack_bb = match piece_type{
                    PieceType::King => self.attack_table.get_king_attacks(pos),
                    PieceType::Bishop => self.attack_table.get_bishop_attacks(pos),
                    PieceType::Rook => self.attack_table.get_rook_attacks(pos),
                    PieceType::Queen => (self.attack_table.get_bishop_attacks(pos) | self.attack_table.get_rook_attacks(pos)),
                    PieceType::Knight => self.attack_table.get_knight_attacks(pos),
                    _ => self.attack_table.get_king_attacks(pos),
                };
                
                attack_bb &= !&cur_position.pieces[cur_position.turn as usize].all_pieces;
                final_bb |= &attack_bb;
                
                piece_bitboard.set_bit(pos,false);
            }
            MoveMask{
                bitboard:final_bb,
                src:pos,
                piece_type,
                //opponent_bb:&opponent_bb,
            }
        };
        for piece in piece_set{
            gen_piece_moves_bb(piece.piece_type,(&piece.bitboard).to_owned());
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
    pub fn print_helper_test(){
        let mvgen = MoveGenerator::new(Dimensions{width:8,height:8});
        let mut position = Position::load_from_fen("8/ppp1p1pp/8/8/2Q3R1/4R3/8/4K3 b - - 0 1".to_string());
        mvgen.generate_pseudolegal_moves(&mut position);
    }

}