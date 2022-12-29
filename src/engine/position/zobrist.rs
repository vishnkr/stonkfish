use crate::engine::position::*;
use std::{collections::HashMap};
use rand::{SeedableRng, Rng, rngs::StdRng};

pub type ZobristKey = u64;

#[derive(Debug)]
pub struct Zobrist{
    pub piece_keys: Vec<HashMap<PieceType,Vec<ZobristKey>>>,
    pub en_passant_keys:Vec<ZobristKey>,
    pub random_side: ZobristKey,
    pub black_to_move:ZobristKey,
}

impl Zobrist{
    pub fn new()->Self{
        let seed = 12345;
        let mut rng = StdRng::seed_from_u64(seed);
        let mut piece_keys = Vec::with_capacity(2);
        let random_side = rng.gen::<u64>();
        let black_to_move = rng.gen::<u64>();
        let en_passant_keys = Vec::new();
        for i in 0..2{
            let mut hashmap = HashMap::new();
            for piece_type in PieceType::as_vec(){
                let mut hash_vec = Vec::new();
                for sq in 0..256{
                    hash_vec.push(rng.gen::<u64>());
                }
                //println!("{:?}",hash_vec);
                hashmap.insert(piece_type,hash_vec);
            }
            piece_keys.push(hashmap);
        }

        Zobrist { piece_keys, en_passant_keys, random_side, black_to_move }
    }
}

#[cfg(test)]
mod zobrist_test{
    use super::*;
    #[test]
    fn print_zobrist_keys(){
        let zobrist = Zobrist::new();

    }
}