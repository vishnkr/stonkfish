use crate::engine::moves::{Move};
use crate::engine::position;
use crate::engine::bitboard::*;
use std::vec::Vec;
use arrayvec::ArrayVec;
//Attack - Defend Map or Attack table
pub struct AttackMap{
    knight_attacks: ArrayVec::<Bitboard,256>,
    slide_attacks:ArrayVec::<Bitboard,256>,
    //masked_knight_attacks: ArrayVec::<Bitboard,256>
}
pub struct MoveGenerator{
    at_map: AttackMap,
}

impl AttackMap{
    pub fn new()->AttackMap{
        let mut knight_attacks = ArrayVec::<Bitboard,256>::new();
        let mut slide_attacks = ArrayVec::<Bitboard,256>::new();
        let knight_offsets:[(i8,i8);8] = [(2,1),(2,-1),(-2,1),(-2,-1),(1,2),(1,-2),(-1,2),(-1,-2)];
        for i in 0..16{
            for j in 0..16{
                let square = to_pos(i, j);
                knight_attacks.push(Bitboard::zero());
                for dir in knight_offsets{
                    let new_x = i as i8 +dir.0;
                    let new_y = j as i8 +dir.1;
                    if new_x>=0 && new_y>=0 && new_x<16 && new_y<16{
                        knight_attacks[square].set_bit(to_pos(new_x as u8,new_y as u8),true);
                    }
                }
                to_string(&knight_attacks[square]);
                
            }
        }
        AttackMap{
            knight_attacks,
            slide_attacks
        }
    }
}

impl MoveGenerator{
    pub fn new()->MoveGenerator{
        MoveGenerator{at_map:AttackMap::new()}
    }

    pub fn generate_moves(&self,cur_position:&mut position::Position)->Vec<Move>{
        let moves = Vec::new();
        let piece_set: &position::PieceSet = &cur_position.pieces[cur_position.turn as usize];
        moves
    }
}

#[cfg(test)]
mod movegen_tests{
    use super::*;
    use crate::engine::bitboard::{to_string};
    #[test]

    pub fn test_knight_attack_masks(){
        let mvgen = MoveGenerator::new();
        to_string(&mvgen.at_map.knight_attacks[165]);
    }
}