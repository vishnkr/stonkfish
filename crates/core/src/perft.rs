use crate::position::Position;
use crate::{movegen::MoveGenerator};

pub fn perft(pos: &Position, gen: &MoveGenerator, depth: u32) -> u64 {
    if depth == 0 {
        return 1;
    }

    let moves = gen.generate_pseudo_legal(pos);
    if depth == 1 {
        return moves.len() as u64;
    }

    let mut nodes = 0;

    for m in moves {
        let mut next = pos.clone();
        next.make_move(m); // must NOT validate yet
        nodes += perft(&next, gen, depth - 1);
    }

    nodes
}


pub fn perft_divide(pos: &Position, gen: &MoveGenerator, depth: u32) -> Vec<(String, u64)> {
    let moves = gen.generate_pseudo_legal(pos);
    let mut result = Vec::new();

    for m in moves {
        let mut next = pos.clone();
        next.make_move(m);
        let count = perft(&next, gen, depth - 1);
        result.push((m.to_uci(&pos.dims), count));
    }
    result
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::{Dimensions};
    use crate::position::Fen;

    #[test]
    fn perft_initial_position_depth_1() {
        let dims = Dimensions::standard();
        let pos = Fen::parse("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", dims).unwrap();
        let gen = MoveGenerator::new(dims);

        let nodes = perft(&pos, &gen, 5);
        println!("nodes = {nodes}");

        assert!(nodes > 0);
    }

    #[test]
    fn perft_initial_position_depth_2() {
        let dims = Dimensions::new(8,11);
        let pos = Fen::parse("r6k/8/8/8/8/8/8/8/8/8/1R2K3 w - - 0 1", dims).unwrap();
        let gen = MoveGenerator::new(dims);
        let moves = gen.generate_pseudo_legal(&pos);
        
        let nodes = perft(&pos, &gen, 3);
        println!("nodes = {nodes}");

        assert!(nodes > 0);
    }
}
