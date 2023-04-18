use super::move_generation::moves::{Move, MType};


pub struct Stats{
    pub search_stats: SearchStats,
    pub move_gen_stats: MoveGenStats
}

impl Stats{
    pub fn new()->Self{
        Self{
            search_stats: SearchStats{ node_count : 0},
            move_gen_stats: MoveGenStats{
                moves_generated : 0,
                depth: 0,
                quiet_count: 0,
                ep_count: 0,
                capture_count:0,
                promo_count: 0,
                castle_count: 0
            }
        }
    }
}
pub struct SearchStats{
    pub node_count: u32
}

pub struct MoveGenStats{
    pub moves_generated : u64,
    pub depth: u8,
    pub quiet_count: u64,
    pub ep_count: u64,
    pub capture_count:u64,
    pub promo_count: u64,
    pub castle_count: u64
}

impl MoveGenStats{
    pub fn display_stats(&self){
        println!("Move Generation Statistics:");
        println!("===========================");
        println!("Total Moves Generated: {}", self.moves_generated);
        println!("Quiet Moves: {}", self.quiet_count);
        println!("Capture Moves: {}", self.capture_count);
        println!("Castle Moves: {}", self.castle_count);
        println!("En Passant Moves: {}", self.ep_count);
        println!("Promotion Moves: {}", self.promo_count);
    }

    pub fn update_move_type_count(&mut self,mv:&Move){
        self.moves_generated+=1;
        match mv.get_mtype() {
            Some(MType::Quiet) => self.quiet_count+=1,
            Some(MType::Capture) => self.capture_count+=1,
            Some(MType::KingsideCastle) | Some(MType::QueensideCastle) => self.castle_count+=1,
            Some(MType::EnPassant) => self.ep_count+=1,
            Some(MType::Promote) => self.promo_count+=1,
            _ => self.moves_generated-=1
        }
    }
}