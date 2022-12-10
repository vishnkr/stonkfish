use moves::*;
use numext_fixed_uint::u256;
use crate::engine::position::*;
use crate::engine::bitboard::*;
use std::vec::Vec;
use arrayvec::ArrayVec;
use std::collections::HashMap;
//use self::move_mask::MoveMask;
pub mod moves;
pub mod move_mask;
//Attack - Defend Map or Attack table, precalculated for standard pieces
pub struct AttackTable{
    knight_attacks: ArrayVec::<Bitboard,256>,
    king_attacks:ArrayVec::<Bitboard,256>,
    pawn_attacks:[ArrayVec::<Bitboard,256>;2],
    slide_attacks:HashMap<SlideDirection,ArrayVec::<Bitboard,256>>,
    occupancy_lookup: Vec<Vec<u16>>,
    files: Vec<Bitboard>,
    ranks: Vec<Bitboard>,
    main_diagonal: Bitboard,
    anti_diagonal: Bitboard,
}
pub struct MoveGenerator{
    attack_table: AttackTable,
}

// contains bitboard with all possible moves for a piece which can be iterated to get a list of moves
pub struct MoveMask{
    pub bitboard: Bitboard,
    pub src: usize,
    pub piece_type: PieceType,
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
        let mut files = vec![Bitboard::zero();16];
        let mut ranks = vec![Bitboard::zero();16];
        let knight_offsets:[(i8,i8);8] = [(2,1),(2,-1),(-2,1),(-2,-1),(1,2),(1,-2),(-1,2),(-1,-2)];
        let king_offsets:[(i8,i8);8] = [(0,1),(0,-1),(1,0),(-1,0),(1,-1),(-1,1),(1,1),(-1,-1)];
        for i in 0..16{
            for j in 0..16{
                let square = to_pos(i as u8, j as u8);
                king_attacks.push(Bitboard::zero());
                knight_attacks.push(Bitboard::zero());
                pawn_attacks[Color::BLACK as usize].push(Bitboard::zero());
                pawn_attacks[Color::WHITE as usize].push(Bitboard::zero());
                Self::mask_slide(i, j,width,height,&dirs,&mut slide_attacks,square);
                Self::mask_jump(i,j, width, height, &knight_offsets, &mut knight_attacks[square]);
                Self::mask_jump(i,j, width, height, &king_offsets, &mut king_attacks[square]);
                Self::mask_pawn_attacks(i,j,width,height,&mut pawn_attacks);
                files[j as usize].set_bit(square,true);
                ranks[i as usize].set_bit(square,true);
            }
        }
        let occupancy_lookup = Self::gen_occupancy_lookup();
        let main_diagonal = &slide_attacks.get(&SlideDirection::SouthWest).unwrap()[15] | &slide_attacks.get(&SlideDirection::NorthEast).unwrap()[240];
        let anti_diagonal = &slide_attacks.get(&SlideDirection::SouthEast).unwrap()[0] | &slide_attacks.get(&SlideDirection::NorthWest).unwrap()[255];
        Self {
            knight_attacks,
            king_attacks,
            pawn_attacks,
            slide_attacks,
            occupancy_lookup,
            files,
            ranks,
            main_diagonal,
            anti_diagonal
        }
    }
    
    pub fn gen_occupancy_lookup()->Vec<Vec<u16>>{
        fn get_rank_attacks(is_right:bool,pos:u16)->u16{
            if is_right{
                if pos != 0 {
                    return pos-1
                }
                return 0u16
            }
            !pos & !get_rank_attacks(true,pos)
        };

        const TOTAL_OCCUPANCY_POSSIBILITES:u16 = u16::MAX;
        let mut occupancy_lookup = vec![vec![0; TOTAL_OCCUPANCY_POSSIBILITES.into()];16];

        for file in 0..16{
            for occupancy in 0..TOTAL_OCCUPANCY_POSSIBILITES{
                // src position of piece in a rank
                let piece_sq = (1<<file) as u16;
                // all attacked bits/files in each direction are set

                let mut right_attacks = get_rank_attacks(true,piece_sq);
                let mut left_attacks = get_rank_attacks(false,piece_sq);
                // set only the occupied bits in each direction
                let right_occ = right_attacks & occupancy;
                let left_occ = left_attacks & occupancy;
                if right_occ!=0{
                    let closest_right_blocker:u16 = 1 << (15-right_occ.leading_zeros());
                    right_attacks ^= get_rank_attacks(true,closest_right_blocker);
                }
                if left_occ!=0{
                    let closest_left_blocker:u16 = 1 << left_occ.trailing_zeros();
                    left_attacks ^= get_rank_attacks(false,closest_left_blocker);
                }
                

                occupancy_lookup[file][occupancy as usize] = left_attacks ^ right_attacks;
            }
        }
        
        occupancy_lookup
    }

    //TODO: split rank and file slide attacks further for each of the 8 directions for custom move patterns
    pub fn get_rank_slide_attacks(&self,rank:u16,file:usize,occupancy:usize)->Bitboard{
        let mut rank_attack = self.occupancy_lookup[file][occupancy];
        let mut rank_attack_bb = Bitboard::zero();
        rank_attack_bb |= rank_attack;
        rank_attack_bb << rank
    }   

    pub fn get_file_slide_attacks(&self,rank:u16,file:usize,occupancy:usize)->Bitboard{
        let mut rank_attack = self.occupancy_lookup[file][occupancy];
        let mut rank_attack_bb = Bitboard::zero();
        rank_attack_bb |= rank_attack;
        rank_attack_bb << rank
    }   

    pub fn get_file(&self,pos:u8)->Bitboard{
        let file = to_col(pos) as usize;
        self.files[file].to_owned()
    }

    pub fn get_rank(&self,pos:u8)->Bitboard{
        let rank = to_row(pos) as usize;
        self.ranks[rank].to_owned()
    }

    pub fn get_king_attacks(&self,position:u8)->Bitboard{
        self.king_attacks[position as usize].to_owned()
    }

    pub fn get_knight_attacks(&self,position:u8)->Bitboard{
        self.knight_attacks[position as usize].to_owned()
    }

    pub fn get_bishop_attacks(&self,position:u8)->Bitboard{
        let dirs = [SlideDirection::NorthWest,SlideDirection::SouthWest,SlideDirection::NorthEast,SlideDirection::SouthEast];
        let mut bb = Bitboard::zero();
        for dir in dirs{
            bb |= &self.slide_attacks[&dir][position as usize];
        }
        bb
    }

    pub fn get_rook_attacks(&self,position:u8,occupancy:&Bitboard)->Bitboard{
        let row = to_row(position);
        let col = to_col(position);
        // file attacks
        let file_occupancy_as_rank = self.insert_rank_into_first_rank(self.get_file_occupancy_as_rank(occupancy, position));
        let file_occupancy_bitboard = self.map_rank_to_first_file(&file_occupancy_as_rank,position) << col;

        // rank attacks
        let rank_occupancy_bitboard = self.insert_rank_into_first_rank(self.get_rank_occupancy(occupancy, position).reverse_bits()) << (16*row);
        rank_occupancy_bitboard | file_occupancy_bitboard 
    }

    pub fn get_file_occupancy_as_rank(&self,occupancy:&Bitboard,pos:u8)->u16{
        let rank_map = self.map_file_to_first_rank(occupancy, pos);
        let row = to_row(pos);
        self.get_valid_occupancy((rank_map.byte(0).unwrap() as u16) | (((rank_map.byte(1).unwrap()) as u16) << 8),row)
    }   

    pub fn get_rank_occupancy(&self,occupancy:&Bitboard,pos:u8)->u16{
        let row = to_row(pos);
        let col = to_col(pos);
        let rank = (&self.ranks[row as usize] & occupancy) >> (16*row);
        self.get_valid_occupancy((rank.byte(0).unwrap() as u16) | (((rank.byte(1).unwrap()) as u16) << 8),col)
    }

    pub fn get_valid_occupancy(&self,rank:u16,file:u8)->u16{
        self.occupancy_lookup[file as usize][rank as usize]
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
        }
    }

    pub fn map_file_to_first_rank(&self,bb:&Bitboard,pos:u8)->Bitboard{
       let file = to_col(pos);
       let mut mask_a_file = (&self.files[file as usize] & bb)>> file;
       let (mut masked_with_diagonal,_valid) = mask_a_file.overflowing_mul(&self.main_diagonal);
       &masked_with_diagonal >> (240)
    }

    pub fn map_main_diagonal_to_first_rank(&self,pos:u8){


    }

    pub fn insert_rank_into_first_rank(&self,rank_occupancy:u16)->Bitboard{
        Bitboard::zero() | rank_occupancy.reverse_bits()
    }

    pub fn map_rank_to_first_file(&self, bb: &Bitboard, pos: u8) -> Bitboard {
        let mut bb2 = bb.overflowing_mul(&(self.anti_diagonal)).0;
        (bb2 >> 15) & &self.files[0]
    }


}

impl MoveGenerator{
    pub fn new(dimensions:Dimensions)->Self{
        Self{attack_table : AttackTable::new(Dimensions{width:dimensions.width,height:dimensions.height})}
    }

    pub fn generate_pseudolegal_moves(&self,cur_position:&mut Position)->(Vec<MoveMask>,Bitboard){
        let mut move_masks :Vec<MoveMask> = Vec::new();
        let opponent_bb = &cur_position.get_opponent_position_bb();
        let player_bb = &cur_position.pieces[cur_position.turn as usize].occupied;
        let occupancy = &cur_position.position_bitboard;
        let mut gen_piece_moves_bb = |piece_type:PieceType,mut piece_bitboard:Bitboard|{
            while !piece_bitboard.is_zero(){
                let mut final_bb = Bitboard::zero();
                let mut pos = piece_bitboard.lowest_one().unwrap_or(0) as u8;
                let mut attack_bb = match piece_type{
                    PieceType::King => self.attack_table.get_king_attacks(pos),
                    PieceType::Bishop => self.attack_table.get_bishop_attacks(pos),
                    PieceType::Rook => self.attack_table.get_rook_attacks(pos,&occupancy),
                    PieceType::Queen => (self.attack_table.get_bishop_attacks(pos) | self.attack_table.get_rook_attacks(pos,&occupancy)),
                    PieceType::Knight => self.attack_table.get_knight_attacks(pos),
                    _ => self.attack_table.get_king_attacks(pos),
                };
                

                attack_bb ^= player_bb;
                final_bb |= &attack_bb;
                
                piece_bitboard.set_bit(pos.into(),false);

                move_masks.push(
                    MoveMask{
                    bitboard:final_bb,
                    src:pos.into(),
                    piece_type,
                });
            }
            
        };

        let piece_set = &cur_position.pieces[cur_position.turn as usize].as_array();
        for piece in piece_set{
            gen_piece_moves_bb(piece.piece_type,(&piece.bitboard).to_owned());
        }
        
        (move_masks,opponent_bb.clone())
        
    }

    
}

#[cfg(test)]
mod movegen_tests{
    use super::*;
    use crate::engine::bitboard::{to_string};
    #[test]
    pub fn test_rank_attack_occupancy_lookup(){
        let occupancy_lookup:Vec<Vec<u16>> = AttackTable::gen_occupancy_lookup();
        assert_eq!(occupancy_lookup[4][0b01010101],0b01101100);
        assert_eq!(occupancy_lookup[0][0b01010101],0b0110);
    }

    #[test]
    pub fn test_get_rook_attacks(){
        let mut position = Position::load_from_fen("8/3b4/8/8/Q2R4/8/8/3n4 w - - 0 1".to_string());
        let at = AttackTable::new(Dimensions { height: 16, width: 16 });
        let pos = 67;
        let occ = &position.position_bitboard;
        to_string(&(at.get_rook_attacks(pos, occ)));
    }
    #[test]
    pub fn print_helper_test(){
        /*let mvgen = MoveGenerator::new(Dimensions{width:8,height:8});
        let mut position = Position::load_from_fen("8/ppp1p1pp/8/8/2Q3R1/4R3/8/4K3 b - - 0 1".to_string());
        mvgen.generate_pseudolegal_moves(&mut position);
        */
    }

}