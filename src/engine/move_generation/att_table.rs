use arrayvec::ArrayVec;
use std::collections::HashMap;
use crate::engine::{
    bitboard::{Bitboard,to_pos,to_row,to_col},
    position::Color,
    utils::get_rank_attacks,
};

#[derive(Debug,PartialEq,Eq,Hash,Copy,Clone)]
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

pub struct AttackTable{
    //pub knight_attacks: ArrayVec::<Bitboard,256>,
    //pub king_attacks:ArrayVec::<Bitboard,256>,
    //pub pawn_attacks:[ArrayVec::<Bitboard,256>;2],
    pub slide_targets:HashMap<SlideDirection,ArrayVec::<Bitboard,256>>,
    pub occupancy_lookup: Vec<Vec<u16>>,
    pub files: Vec<Bitboard>,
    pub ranks: Vec<Bitboard>,
    pub diagonals: HashMap<i8,Bitboard>,
    pub anti_diagonals: HashMap<i8,Bitboard>, 
    pub main_diagonal: Bitboard,
    pub anti_diagonal: Bitboard,
    pub full_bitboard:Bitboard,
}

impl AttackTable{
    pub fn new()->Self{
        let dirs = vec![SlideDirection::East,SlideDirection::North,SlideDirection::West,
            SlideDirection::South,SlideDirection::NorthEast,SlideDirection::NorthWest,
            SlideDirection::SouthEast,SlideDirection::SouthWest];
        //let mut king_attacks = ArrayVec::<Bitboard,256>::new();
        //let mut pawn_attacks = [ArrayVec::<Bitboard,256>::new(),ArrayVec::<Bitboard,256>::new()];
        let mut slide_targets:HashMap<SlideDirection, ArrayVec::<Bitboard,256>> = Self::init_slide_hashmap(&dirs);
        let mut diagonals = Self::init_diagonal_hashmap(false);
        let mut anti_diagonals = Self::init_diagonal_hashmap(true);
        let mut files = vec![Bitboard::zero();16];
        let mut ranks = vec![Bitboard::zero();16];
        let full_bitboard = Bitboard::zero();

        for i in 0..16{
            for j in 0..16{
                let square = to_pos(i as u8, j as u8);
                //king_attacks.push(Bitboard::zero());
                //knight_attacks.push(Bitboard::zero());
                //pawn_attacks[Color::BLACK as usize].push(Bitboard::zero());
                //pawn_attacks[Color::WHITE as usize].push(Bitboard::zero());
                Self::mask_slide(i, j,&dirs,&mut slide_targets,square);
               //Self::mask_jump(i,j, width, height, &knight_offsets, &mut knight_attacks[square]);
                //Self::mask_jump(i,j, width, height, &king_offsets, &mut king_attacks[square]);
                //Self::mask_pawn_attacks(i,j,width,height,&mut pawn_attacks);
                files[j as usize].set_bit(square,true);
                ranks[i as usize].set_bit(square,true);
                
                diagonals.get_mut(&(j-i)).unwrap().set_bit(square,true);
                anti_diagonals.get_mut(&(j+i)).unwrap().set_bit(square,true);

            }
        }
        let occupancy_lookup = Self::gen_occupancy_lookup();
        let main_diagonal = &slide_targets.get(&SlideDirection::SouthWest).unwrap()[15] | &slide_targets.get(&SlideDirection::NorthEast).unwrap()[240];
        let anti_diagonal = &slide_targets.get(&SlideDirection::SouthEast).unwrap()[0] | &slide_targets.get(&SlideDirection::NorthWest).unwrap()[255];
        Self {
            slide_targets,
            occupancy_lookup,
            files,
            ranks,
            diagonals,
            anti_diagonals,
            main_diagonal,
            anti_diagonal,
            full_bitboard,
        }
    }
    

    pub fn gen_occupancy_lookup()->Vec<Vec<u16>>{
        

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

    pub fn get_file(&self,pos:u8)->Bitboard{
        let file = to_col(pos) as usize;
        self.files[file].to_owned()
    }

    pub fn get_rank(&self,pos:u8)->Bitboard{
        let rank = to_row(pos) as usize;
        if  rank>15{
            return Bitboard::zero()
        }
        self.ranks[rank].to_owned()
    }

    pub fn get_diagonal_attacks(&self,square:u8,occupancy:&Bitboard)->Bitboard{
        let (row,col) = (to_row(square) as i8,to_col(square) as i8);

        let diagonal_as_rank = self.get_diagonal_occupancy_as_rank(occupancy, false, square);

        &( Bitboard::zero() | diagonal_as_rank)
            .overflowing_mul(&self.files[0]).0 & self.diagonals.get(&(col-row))
            .unwrap()
    }

    pub fn get_anti_diagonal_attacks(&self,square:u8,occupancy:&Bitboard)->Bitboard{
        let (row,col) = (to_row(square) as i8,to_col(square) as i8);

        let anti_diagonal_as_rank = self.get_diagonal_occupancy_as_rank(occupancy, true,square);

        &(Bitboard::zero() | anti_diagonal_as_rank)
        .overflowing_mul(&self.files[0]).0 & self.anti_diagonals.get(&(col + row))
        .unwrap()
    }

    pub fn get_rank_attacks(&self,square:u8,occupancy:&Bitboard)->Bitboard{
        let row = to_row(square);

        Bitboard::zero() | self.insert_rank_into_first_rank(self.get_rank_occupancy(occupancy, square).reverse_bits()) << (16*row)
    }

    pub fn get_file_attacks(&self,square:u8,occupancy:&Bitboard)->Bitboard{
        let col = to_col(square);
        let file_occupancy_as_rank = self.insert_rank_into_first_rank(self.get_file_occupancy_as_rank(occupancy, square));
        self.map_rank_to_first_file(&file_occupancy_as_rank,square) << col
    }
    pub fn get_file_occupancy_as_rank(&self,occupancy:&Bitboard,pos:u8)->u16{
        let rank_map = self.map_file_to_first_rank(occupancy, pos);
        let row = to_row(pos);
        let rank_occupancy = (rank_map.byte(0).unwrap() as u16) | ((rank_map.byte(1).unwrap() as u16) << 8);
        self.get_valid_occupancy(rank_occupancy.reverse_bits(),row)
    }   

    pub fn get_diagonal_occupancy_as_rank(&self,occupancy:&Bitboard,is_anti_diagonal:bool,pos:u8)->u16{
        let col = to_col(pos);
        let row = to_row(pos);
        let diagonal = match is_anti_diagonal {
            false => self.diagonals.get(&(col as i8 - row as i8)).unwrap(),
            true => self.anti_diagonals.get(&(col as i8 + row as i8)).unwrap(),
            } 
            & occupancy;
        let mapped_with_garbage = self.files[0].overflowing_mul(&diagonal).0;
        let rank_map:Bitboard = &mapped_with_garbage >> 240;
        let rank_occupancy = (rank_map.byte(0).unwrap() as u16) | ((rank_map.byte(1).unwrap() as u16) << 8);
        self.get_valid_occupancy(rank_occupancy,col)
    }

    pub fn get_rank_occupancy(&self,occupancy:&Bitboard,square:u8)->u16{
        let (row,col) = (to_row(square),to_col(square));
        let rank = (&self.ranks[row as usize] & occupancy) >> (16*row);
        self.get_valid_occupancy((rank.byte(0).unwrap() as u16) | (((rank.byte(1).unwrap()) as u16) << 8),col)
    }

    pub fn get_valid_occupancy(&self,occ_rank:u16,file:u8)->u16{
        self.occupancy_lookup[file as usize][occ_rank as usize]
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

    pub fn init_diagonal_hashmap(is_anti_diagonal:bool)->HashMap<i8,Bitboard>{
        let mut hashmap:HashMap<i8,Bitboard> = HashMap::new();
        let mut min:i8 = -15;
        let mut max:i8 = 15;
        if is_anti_diagonal{
            min = 0;
            max = 30;
        }
        for i in min..max+1{
            hashmap.insert(i,Bitboard::zero());
        }
        hashmap
    }

    pub fn mask_slide(x:i8,y:i8,dirs:&Vec<SlideDirection>,hashmap:&mut HashMap<SlideDirection,ArrayVec::<Bitboard,256>>,square:usize){
        let mut dx:i8;
        let mut dy:i8;
        let width = 16;
        let height = 16;
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
       let mask_a_file = (&self.files[file as usize] & bb)>> file;
       let (masked_with_diagonal,_) = mask_a_file.overflowing_mul(&self.anti_diagonal);
       &masked_with_diagonal >> (240)
    }

    pub fn insert_rank_into_first_rank(&self,rank_occupancy:u16)->Bitboard{
        Bitboard::zero() | rank_occupancy.reverse_bits()
    }

    pub fn map_rank_to_first_file(&self, bb: &Bitboard, _pos: u8) -> Bitboard {
        let bb2 = bb.overflowing_mul(&self.anti_diagonal).0;
        (bb2 >> 15) & &self.files[0]
    }


}