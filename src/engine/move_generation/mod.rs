use moves::*;
use numext_fixed_uint::u256;
use crate::engine::{
    bitboard::*,
    position::*,
    utils::get_rank_attacks,
};
use std::vec::Vec;
use arrayvec::ArrayVec;
use std::collections::HashMap;
//use self::move_mask::MoveMask;
pub mod moves;

//Attack - Defend Map or Attack table, precalculated for standard pieces
pub struct AttackTable{
    knight_attacks: ArrayVec::<Bitboard,256>,
    king_attacks:ArrayVec::<Bitboard,256>,
    pawn_attacks:[ArrayVec::<Bitboard,256>;2],
    slide_attacks:HashMap<SlideDirection,ArrayVec::<Bitboard,256>>,
    occupancy_lookup: Vec<Vec<u16>>,
    files: Vec<Bitboard>,
    ranks: Vec<Bitboard>,
    diagonals: HashMap<i8,Bitboard>,
    anti_diagonals: HashMap<i8,Bitboard>, 
    main_diagonal: Bitboard,
    anti_diagonal: Bitboard,
    full_bitboard:Bitboard,
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
        let mut diagonals = Self::init_diagonal_hashmap(false);
        let mut anti_diagonals = Self::init_diagonal_hashmap(true);
        let width = dimensions.width as i8;
        let height = dimensions.height as i8;
        let mut files = vec![Bitboard::zero();16];
        let mut ranks = vec![Bitboard::zero();16];
        let mut full_bitboard = Bitboard::zero();
        let knight_offsets:[(i8,i8);8] = [(2,1),(2,-1),(-2,1),(-2,-1),(1,2),(1,-2),(-1,2),(-1,-2)];
        let king_offsets:[(i8,i8);8] = [(0,1),(0,-1),(1,0),(-1,0),(1,-1),(-1,1),(1,1),(-1,-1)];
        for i in 0..16{
            for j in 0..16{
                let square = to_pos(i as u8, j as u8);
                king_attacks.push(Bitboard::zero());
                knight_attacks.push(Bitboard::zero());
                pawn_attacks[Color::BLACK as usize].push(Bitboard::zero());
                pawn_attacks[Color::WHITE as usize].push(Bitboard::zero());
                Self::mask_slide(i, j,&dirs,&mut slide_attacks,square);
                Self::mask_jump(i,j, width, height, &knight_offsets, &mut knight_attacks[square]);
                Self::mask_jump(i,j, width, height, &king_offsets, &mut king_attacks[square]);
                Self::mask_pawn_attacks(i,j,width,height,&mut pawn_attacks);
                files[j as usize].set_bit(square,true);
                ranks[i as usize].set_bit(square,true);
                if i <height && j<width{
                    full_bitboard.set_bit(square,true);
                }
                diagonals.get_mut(&(j-i)).unwrap().set_bit(square,true);
                anti_diagonals.get_mut(&(j+i)).unwrap().set_bit(square,true);

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
            diagonals,
            anti_diagonals,
            main_diagonal,
            anti_diagonal,
            full_bitboard
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
        if rank<0 || rank>15{
            return Bitboard::zero()
        }
        self.ranks[rank].to_owned()
    }

    pub fn get_king_attacks(&self,position:u8)->Bitboard{
        self.king_attacks[position as usize].to_owned()
    }

    pub fn get_knight_attacks(&self,position:u8)->Bitboard{
        self.knight_attacks[position as usize].to_owned()
    }

    pub fn get_pawn_attacks(&self,position:u8,color:Color,opponent:&Bitboard)->Bitboard{
        &self.pawn_attacks[color as usize][position as usize] & opponent
    }

    pub fn get_pawn_pushes(&self,position:u8,color:Color,dimensions:&Dimensions,player_bb:&Bitboard,opponent:&Bitboard)->Bitboard{
        let row = to_row(position);
        let next_rank_bb = match color {
            Color::WHITE => self.get_rank(position-16),
            Color::BLACK => self.get_rank(position+16)
        };
        let single_push :Bitboard = self.get_king_attacks(position) & self.get_file(position) & next_rank_bb;
        let mut double_push :Bitboard = Bitboard::zero();
        match color{
            Color::WHITE => {
                if row == dimensions.height-2 {
                    // no double push allowed if single push is blocked by own piece
                    if !player_bb.bit(single_push.lowest_one().unwrap()).unwrap(){
                        double_push = &single_push >> 16;
                    }
                    
                }
            },
            Color::BLACK => {
                if row == 1 {
                    if !player_bb.bit(single_push.lowest_one().unwrap()).unwrap(){
                        double_push = &single_push << 16;
                    }
                }
            }
        };
        (single_push | double_push) & !opponent
    }

    pub fn get_pawn_attacks_and_pushes(&self,position:u8,color:Color,dimensions:&Dimensions,player_bb:&Bitboard,opponent:&Bitboard)->Bitboard{
        self.get_pawn_pushes(position, color, dimensions, player_bb, opponent) | self.get_pawn_attacks(position, color, opponent)
    }

    pub fn get_bishop_attacks(&self,position:u8,occupancy:&Bitboard)->Bitboard{
        let row = to_row(position);
        let col = to_col(position);
        // diagonal attack -> diagonal.get(pos) & occupancy ->convert diagonal to rank -> lookup occupancy -> convert rank to diagonal
        let diagonal_as_rank = self.get_diagonal_occupancy_as_rank(occupancy, false, position);
        let diagonal_attacks = &(Bitboard::zero() | diagonal_as_rank).overflowing_mul(&self.files[0]).0 & self.diagonals.get(&(col as i8 - row as i8)).unwrap();

        let anti_diagonal_as_rank = self.get_diagonal_occupancy_as_rank(occupancy, true,position);
        let anti_diagonal_attacks = &(Bitboard::zero() | anti_diagonal_as_rank).overflowing_mul(&self.files[0]).0 & self.anti_diagonals.get(&(col as i8 + row as i8)).unwrap();
        
        diagonal_attacks | anti_diagonal_attacks
    }

    pub fn get_rook_attacks(&self,position:u8,occupancy:&Bitboard)->Bitboard{
        let row = to_row(position);
        let col = to_col(position);
        // file attacks
        let file_occupancy_as_rank = self.insert_rank_into_first_rank(self.get_file_occupancy_as_rank(occupancy, position));
        let file_occupancy_bitboard = self.map_rank_to_first_file(&file_occupancy_as_rank,position) << col;
        // rank attacks
        let rank_occupancy_bitboard = self.insert_rank_into_first_rank(self.get_rank_occupancy(occupancy, position).reverse_bits()) << (16*row);
        &rank_occupancy_bitboard ^ &file_occupancy_bitboard 
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
        let mut diagonal = match is_anti_diagonal {
            false => self.diagonals.get(&(col as i8 - row as i8)).unwrap(),
            true => self.anti_diagonals.get(&(col as i8 + row as i8)).unwrap(),
            } 
            & occupancy;
        let mapped_with_garbage = self.files[0].overflowing_mul(&diagonal).0;
        let rank_map:Bitboard = &mapped_with_garbage >> 240;
        let rank_occupancy = (rank_map.byte(0).unwrap() as u16) | ((rank_map.byte(1).unwrap() as u16) << 8);
        self.get_valid_occupancy(rank_occupancy,col)
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
       let mut mask_a_file = (&self.files[file as usize] & bb)>> file;
       let (mut masked_with_diagonal,valid) = mask_a_file.overflowing_mul(&self.anti_diagonal);
       &masked_with_diagonal >> (240)
    }

    pub fn insert_rank_into_first_rank(&self,rank_occupancy:u16)->Bitboard{
        Bitboard::zero() | rank_occupancy.reverse_bits()
    }

    pub fn map_rank_to_first_file(&self, bb: &Bitboard, pos: u8) -> Bitboard {
        let mut bb2 = bb.overflowing_mul(&self.anti_diagonal).0;
        let ret = (bb2 >> 15) & &self.files[0];
        ret
    }


}

impl MoveGenerator{
    pub fn new(dimensions:Dimensions)->Self{
        Self{attack_table : AttackTable::new(Dimensions{width:dimensions.width,height:dimensions.height})}
    }

    pub fn generate_pseudolegal_moves(&self,cur_position:&mut Position)-> impl Iterator<Item=Move>{
        let mut move_masks :Vec<MoveMask> = Vec::new();
        let color = cur_position.turn;
        let opponent_bb =  &cur_position.position_bitboard & !&cur_position.pieces[color as usize].occupied;
        let player_bb = &cur_position.pieces[color as usize].occupied;
        let occupancy = &cur_position.position_bitboard;
        let mut gen_piece_moves_bb = |piece_type:PieceType,mut piece_bitboard:Bitboard|{
            while !piece_bitboard.is_zero(){
                let mut pos = piece_bitboard.lowest_one().unwrap_or(0) as u8;
                let mut attack_bb = match piece_type{
                    PieceType::King => self.attack_table.get_king_attacks(pos),
                    PieceType::Bishop => self.attack_table.get_bishop_attacks(pos,&occupancy),
                    PieceType::Rook => self.attack_table.get_rook_attacks(pos,&occupancy),
                    PieceType::Queen => self.attack_table.get_bishop_attacks(pos,&occupancy) | self.attack_table.get_rook_attacks(pos,&occupancy),
                    PieceType::Knight => self.attack_table.get_knight_attacks(pos),
                    PieceType::Pawn => self.attack_table.get_pawn_attacks_and_pushes(pos,cur_position.turn,&cur_position.dimensions,player_bb,&opponent_bb),
                    _ => self.attack_table.get_knight_attacks(pos),
                };
                
                attack_bb &= !player_bb;
                
                piece_bitboard.set_bit(pos.into(),false);
                attack_bb.set_bit(pos.into(),false);
                attack_bb &= &self.attack_table.full_bitboard;
                move_masks.push(
                    MoveMask{
                    bitboard:attack_bb,
                    src:pos.into(),
                    piece_type,
                    opponent: opponent_bb.to_owned()
                });
            }
            
        };


        let piece_set = &cur_position.pieces[cur_position.turn as usize].as_array();
        for piece in piece_set{
            gen_piece_moves_bb(piece.piece_type,(&piece.bitboard).to_owned());
        }

        let mut moves:Vec<Move> = vec![];
        let king_pos = cur_position.pieces[color as usize].king.bitboard.lowest_one().unwrap() as u8;
        if cur_position.valid_kingside_castle(){
            // is rook in position, is path blocked and will king move into check after 1 move
            let target_rank = to_row(king_pos as u8);
            let target_rook_pos = (16*target_rank+1)-1;
            if let Some(ref piece) = cur_position.pieces[color as usize].get_piece_from_sq(target_rook_pos.into()){
                if piece.piece_type == PieceType::Rook{
                    let mut rank_attack = Bitboard::from(get_rank_attacks(true, king_pos as u16));
                    rank_attack &= &cur_position.position_bitboard;
                    rank_attack.set_bit(target_rook_pos.into(), false);
                    let dest = to_pos(king_pos, king_pos+1) as u8;
                    if rank_attack.is_zero(){
                        let skipped_mv = Move::new(king_pos, dest, MType::Quiet,None);
                        if self.is_legal_move(cur_position, &skipped_mv){
                            moves.push(Move::new(king_pos, king_pos+2, MType::KingsideCastle,Some(AdditionalInfo::CastlingRookPos(target_rook_pos))));
                        }
                    }
                }
                
            }
        }
        if cur_position.valid_queenside_castle(){
            let target_rank = to_row(king_pos as u8);
            let target_rook_pos = 16*target_rank;
            if let Some(piece) = cur_position.pieces[color as usize].get_piece_from_sq(target_rook_pos.into()){
                if piece.piece_type == PieceType::Rook{
                    let mut rank_attack = Bitboard::from(get_rank_attacks(false, king_pos as u16));
                    rank_attack &= &cur_position.position_bitboard;
                    rank_attack.set_bit(target_rook_pos.into(), false);
                    let dest = to_pos(king_pos, king_pos-1) as u8;
                    if rank_attack.is_zero(){
                        let skipped_mv = Move::new(king_pos, dest, MType::Quiet,None);
                        if self.is_legal_move(cur_position, &skipped_mv){
                            moves.push(Move::new(king_pos, king_pos-2, MType::QueensideCastle,Some(AdditionalInfo::CastlingRookPos(target_rook_pos))));
                        }
                    }
                }
            }
        }
        
        move_masks.into_iter().flatten()
        
    }

    fn generate_legal_moves<'a>(&'a self,cur_position:&'a mut Position)-> Vec<(u8,u8)>{
       let mut pseudo_moves = self.generate_pseudolegal_moves(cur_position);
       let mut legal_moves:Vec<(u8,u8)> = Vec::new();
       for mv in pseudo_moves.filter(|mv| self.is_legal_move(cur_position, &mv)){
        legal_moves.push((mv.parse_from() as u8,mv.parse_to() as u8));
       }
       legal_moves
    }

    pub fn is_legal_move(&self,position:&mut Position, mv: &Move)->bool{
        position.make_move(mv);
        let mut is_under_check = false;
        
        if self.is_king_under_check(position){
            is_under_check = true;
        }
        position.unmake_move(mv);
        !is_under_check
    }

    pub fn is_king_under_check(&self,position:&mut Position)-> bool{
        let pieces = &mut position.pieces;
        let color = position.turn.clone();
        let opponent_color = Position::get_opponent_color(position.turn);
        let mut opponent_bb =  &position.position_bitboard & !&pieces[color as usize].occupied;
        let occupancy = &position.position_bitboard;

        while !opponent_bb.is_zero(){
                let mut pos = opponent_bb.lowest_one().unwrap_or(0) as u8;
                let piece = pieces[opponent_color as usize].get_piece_from_sq(pos.into()).unwrap();
                let mut attack_bb = match piece.piece_type{
                    PieceType::King => self.attack_table.get_king_attacks(pos),
                    PieceType::Bishop => self.attack_table.get_bishop_attacks(pos,&occupancy),
                    PieceType::Rook => self.attack_table.get_rook_attacks(pos,&occupancy),
                    PieceType::Queen => self.attack_table.get_bishop_attacks(pos,&occupancy) | self.attack_table.get_rook_attacks(pos,&occupancy),
                    PieceType::Knight => self.attack_table.get_knight_attacks(pos),
                    PieceType::Pawn => {
                        let player_bb = &pieces[color as usize].occupied;
                        self.attack_table.get_pawn_attacks(pos,opponent_color,&opponent_bb)
                    },
                    _ => self.attack_table.get_knight_attacks(pos),
                };
                if !(attack_bb & &pieces[color as usize].king.bitboard).is_zero(){
                    return true
                }
                opponent_bb.set_bit(pos.into(),false);
        }

        false
    }


    
}

#[cfg(test)]
mod movegen_tests{
    use super::*;
    use crate::engine::bitboard::{display_bitboard};
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
        let row = to_row(pos) as i8;
        let col = to_col(pos) as i8;
        display_bitboard_with_board_desc(&(at.diagonals.get(&(col-row)).unwrap()), "Diagonal from pos");
        display_bitboard_with_board_desc(&(at.anti_diagonals.get(&(col+row)).unwrap()), "aNTI Diagonal from pos");
    }
    #[test]
    pub fn print_helper_test(){
        let dimensions = Dimensions{width:8,height:8};
        let mvgen = MoveGenerator::new(dimensions);
        let mut position = Position::load_from_fen("3k4/8/8/8/1n3b2/P1P1P3/1PP3P1/3K4 w - - 0 1".to_string());
        let val = mvgen.generate_pseudolegal_moves(&mut position);
        for mv in val{
            println!("{}",mv);
        }
    }

    #[test]
    pub fn test_legal_movegen(){
        let dimensions = Dimensions{width:8,height:8};
        let mvgen = MoveGenerator::new(dimensions);
        let mut position = Position::load_from_fen("3r4/8/8/8/8/8/3R4/3K4 w - - 0 1".to_string());
        for mv in mvgen.generate_legal_moves(&mut position){
            println!("src {} {}, dest {} {}",to_row(mv.0),to_col(mv.0),to_row(mv.1),to_col(mv.1));
        }
        
    }

    #[test]
    pub fn test_unmake_move(){
        let dimensions = Dimensions{width:8,height:8};
        let mvgen = MoveGenerator::new(dimensions);
        let position = &mut Position::load_from_fen("3k4/8/8/8/1n3b2/P1P1P3/1PP3P1/3K4 w - - 0 1".to_string());
        let original = &Position::load_from_fen("3k4/8/8/8/1n3b2/P1P1P3/1PP3P1/3K4 w - - 0 1".to_string());
        for mv in mvgen.generate_pseudolegal_moves(position){
            position.make_move(&mv);
            position.unmake_move( &mv);
            assert_eq!(original.position_bitboard,position.position_bitboard);
            assert_eq!(original.pieces[Color::WHITE as usize].occupied,position.pieces[Color::WHITE as usize].occupied);
            assert_eq!(original.pieces[Color::WHITE as usize].pawn.bitboard,position.pieces[Color::WHITE as usize].pawn.bitboard);
            assert_eq!(original.pieces[Color::WHITE as usize].king.bitboard,position.pieces[Color::WHITE as usize].king.bitboard);
            assert_eq!(original.pieces[Color::WHITE as usize].knight.bitboard,position.pieces[Color::WHITE as usize].knight.bitboard);
            assert_eq!(original.pieces[Color::WHITE as usize].queen.bitboard,position.pieces[Color::WHITE as usize].queen.bitboard);
            assert_eq!(original.pieces[Color::WHITE as usize].rook.bitboard,position.pieces[Color::WHITE as usize].rook.bitboard);
            assert_eq!(original.pieces[Color::WHITE as usize].bishop.bitboard,position.pieces[Color::WHITE as usize].bishop.bitboard);
        }
    }

}