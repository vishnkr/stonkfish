use crate::board::{Dimensions, Square, BitBoard, BB};


pub struct AttackTable {
    dims: Dimensions,
    knight_attacks: Vec<BitBoard>,
    king_attacks: Vec<BitBoard>,
    

    rook_rays: Vec<Vec<BitBoard>>,  // [square][direction] -> ray bitboard
    bishop_rays: Vec<Vec<BitBoard>>, // [square][direction] -> ray bitboard
    
    // Direction indices
    rook_dir_count: usize,
    bishop_dir_count: usize,
}

impl AttackTable {
    pub fn new(dims: Dimensions) -> Self {
        
        let knight_attacks = Self::precompute_knight_attacks(dims);
        let king_attacks = Self::precompute_king_attacks(dims);
        
        let (rook_rays, rook_dir_count) = Self::precompute_rook_rays(dims);
        let (bishop_rays, bishop_dir_count) = Self::precompute_bishop_rays(dims);
        
        Self {
            dims,
            knight_attacks,
            king_attacks,
            rook_rays,
            bishop_rays,
            rook_dir_count,
            bishop_dir_count,
        }
    }
    
    #[inline]
    pub fn knight_attacks(&self, sq: Square) -> BitBoard {
        self.knight_attacks[sq.idx() as usize]
    }
    
    #[inline]
    pub fn king_attacks(&self, sq: Square) -> BitBoard {
        self.king_attacks[sq.idx() as usize]
    }
    
    pub fn rook_attacks(&self, sq: Square, occupied: BitBoard) -> BitBoard {
        let sq_idx = sq.idx() as usize;
        let mut attacks = BitBoard::empty_for_dims(&self.dims);
        
        // For each direction, find the first blocker and mask the ray
        for dir_idx in 0..self.rook_dir_count {
            let ray = self.rook_rays[sq_idx][dir_idx];
            
            // Find blockers in this ray
            let blockers = ray.intersect(occupied);
            
            if blockers.is_empty() {
                // No blockers, entire ray is valid
                attacks = attacks.union(ray);
            } else {
                // Find the first blocker
                let first_blocker = Self::first_set_bit(blockers);
                if let Some(blocker_sq) = first_blocker {
                    // Include squares up to and including the blocker
                    let mask = Self::ray_mask(sq, blocker_sq, &self.dims);
                    attacks = attacks.union(ray.intersect(mask));
                }
            }
        }
        
        attacks
    }
    
    pub fn bishop_attacks(&self, sq: Square, occupied: BitBoard) -> BitBoard {
        let sq_idx = sq.idx() as usize;
        let mut attacks = BitBoard::empty_for_dims(&self.dims);
        
        for dir_idx in 0..self.bishop_dir_count {
            let ray = self.bishop_rays[sq_idx][dir_idx];
            
            let blockers = ray.intersect(occupied);
            
            if blockers.is_empty() {
                attacks = attacks.union(ray);
            } else {
                let first_blocker = Self::first_set_bit(blockers);
                if let Some(blocker_sq) = first_blocker {
                    let mask = Self::ray_mask(sq, blocker_sq, &self.dims);
                    attacks = attacks.union(ray.intersect(mask));
                }
            }
        }
        
        attacks
    }
    
    /// Get queen attacks (rook + bishop)
    pub fn queen_attacks(&self, sq: Square, occupied: BitBoard) -> BitBoard {
        self.rook_attacks(sq, occupied).union(self.bishop_attacks(sq, occupied))
    }
    
    
    fn precompute_knight_attacks(dims: Dimensions) -> Vec<BitBoard> {
        let num_squares = dims.num_squares() as usize;
        let mut attacks = vec![BitBoard::empty_for_dims(&dims); num_squares];
        
        let knight_offsets = [
            (1, 2), (2, 1), (-1, 2), (-2, 1),
            (1, -2), (2, -1), (-1, -2), (-2, -1),
        ];
        
        for rank in 0..dims.height {
            for file in 0..dims.width {
                let sq = Square::from_rank_file(rank, file, &dims);
                let mut attack_bb = BitBoard::empty_for_dims(&dims);
                
                for &(df, dr) in &knight_offsets {
                    let target_file = file as i8 + df;
                    let target_rank = rank as i8 + dr;
                    
                    if target_file >= 0 && target_file < dims.width as i8 &&
                       target_rank >= 0 && target_rank < dims.height as i8 {
                        let target_sq = Square::from_rank_file(
                            target_rank as u8,
                            target_file as u8,
                            &dims,
                        );
                        attack_bb = attack_bb.set(target_sq);
                    }
                }
                
                attacks[sq.idx() as usize] = attack_bb;
            }
        }
        
        attacks
    }
    
    fn precompute_king_attacks(dims: Dimensions) -> Vec<BitBoard> {
        let num_squares = dims.num_squares() as usize;
        let mut attacks = vec![BitBoard::empty_for_dims(&dims); num_squares];
        
        let king_offsets = [
            (0, 1), (1, 1), (1, 0), (1, -1),
            (0, -1), (-1, -1), (-1, 0), (-1, 1),
        ];
        
        for rank in 0..dims.height {
            for file in 0..dims.width {
                let sq = Square::from_rank_file(rank, file, &dims);
                let mut attack_bb = BitBoard::empty_for_dims(&dims);
                
                for &(df, dr) in &king_offsets {
                    let target_file = file as i8 + df;
                    let target_rank = rank as i8 + dr;
                    
                    if target_file >= 0 && target_file < dims.width as i8 &&
                       target_rank >= 0 && target_rank < dims.height as i8 {
                        let target_sq = Square::from_rank_file(
                            target_rank as u8,
                            target_file as u8,
                            &dims,
                        );
                        attack_bb = attack_bb.set(target_sq);
                    }
                }
                
                attacks[sq.idx() as usize] = attack_bb;
            }
        }
        
        attacks
    }
    
    fn precompute_rook_rays(dims: Dimensions) -> (Vec<Vec<BitBoard>>, usize) {
        let num_squares = dims.num_squares() as usize;
        let directions = [
            (0, 1),
            (0, -1),
            (1, 0),
            (-1, 0),
        ];
        
        let empty_bb = BitBoard::empty_for_dims(&dims);
        let mut rays = vec![vec![empty_bb; directions.len()]; num_squares];
        
        for rank in 0..dims.height {
            for file in 0..dims.width {
                let sq = Square::from_rank_file(rank, file, &dims);
                let sq_idx = sq.idx() as usize;
                
                for (dir_idx, &(df, dr)) in directions.iter().enumerate() {
                    let mut ray = BitBoard::empty_for_dims(&dims);
                    let mut current_file = file as i8;
                    let mut current_rank = rank as i8;
                    
                    loop {
                        current_file += df;
                        current_rank += dr;
                        
                        if current_file < 0 || current_file >= dims.width as i8 ||
                           current_rank < 0 || current_rank >= dims.height as i8 {
                            break;
                        }
                        
                        let target_sq = Square::from_rank_file(
                            current_rank as u8,
                            current_file as u8,
                            &dims,
                        );
                        ray = ray.set(target_sq);
                    }
                    
                    rays[sq_idx][dir_idx] = ray;
                }
            }
        }
        
        (rays, directions.len())
    }
    
    fn precompute_bishop_rays(dims: Dimensions) -> (Vec<Vec<BitBoard>>, usize) {
        let num_squares = dims.num_squares() as usize;
        let directions = [
            (1, 1),    // Northeast
            (1, -1),   // Southeast
            (-1, -1),  // Southwest
            (-1, 1),   // Northwest
        ];
        
        let empty_bb = BitBoard::empty_for_dims(&dims);
        let mut rays = vec![vec![empty_bb; directions.len()]; num_squares];
        
        for rank in 0..dims.height {
            for file in 0..dims.width {
                let sq = Square::from_rank_file(rank, file, &dims);
                let sq_idx = sq.idx() as usize;
                
                for (dir_idx, &(df, dr)) in directions.iter().enumerate() {
                    let mut ray = BitBoard::empty_for_dims(&dims);
                    let mut current_file = file as i8;
                    let mut current_rank = rank as i8;
                    
                    loop {
                        current_file += df;
                        current_rank += dr;
                        
                        if current_file < 0 || current_file >= dims.width as i8 ||
                           current_rank < 0 || current_rank >= dims.height as i8 {
                            break;
                        }
                        
                        let target_sq = Square::from_rank_file(
                            current_rank as u8,
                            current_file as u8,
                            &dims,
                        );
                        ray = ray.set(target_sq);
                    }
                    
                    rays[sq_idx][dir_idx] = ray;
                }
            }
        }
        
        (rays, directions.len())
    }
    
    #[inline]
    fn first_set_bit(bb: BitBoard) -> Option<Square> {
        if bb.is_empty() {
            None
        } else {
            let mut temp = bb;
            temp.pop_lsb()
        }
    }
    
    /// Create a mask for squares between (and including) two squares on a ray
    fn ray_mask(from: Square, to: Square, dims: &Dimensions) -> BitBoard {
        let (from_file, from_rank) = from.file_rank(dims);
        let (to_file, to_rank) = to.file_rank(dims);
        
        let file_delta = to_file as i8 - from_file as i8;
        let rank_delta = to_rank as i8 - from_rank as i8;
        
        let file_step = if file_delta != 0 { file_delta.signum() } else { 0 };
        let rank_step = if rank_delta != 0 { rank_delta.signum() } else { 0 };
        
        let mut mask = BitBoard::empty_for_dims(&dims);
        let mut current_file = from_file as i8;
        let mut current_rank = from_rank as i8;
        
        loop {
            let sq = Square::from_rank_file(
                current_rank as u8,
                current_file as u8,
                dims,
            );
            mask = mask.set(sq);
            
            if current_file == to_file as i8 && current_rank == to_rank as i8 {
                break;
            }
            
            current_file += file_step;
            current_rank += rank_step;
        }
        
        mask
    }
}

