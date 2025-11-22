pub mod board;
pub mod piece;
pub mod position;
pub mod moves;
pub mod movegen;
pub mod perft;

pub mod prelude {
    pub use crate::board::{Dimensions, Square, BitBoard, BB};
}

#[cfg(test)]
mod integration_tests {
    use super::prelude::*;
    use super::{position::*, piece::*, movegen::*};
    use crate::board::BB;

    #[test]
    fn full_game_flow() {
        let dims = Dimensions::standard();
        let fen_str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        
        let pos = Fen::parse(fen_str, dims).unwrap();
        let gen = MoveGenerator::new(dims);
        let moves = gen.generate_pseudo_legal(&pos);
        
        assert!(moves.len() > 0);
        assert!(pos.piece_bb(Color::White, PieceKind::King).count() > 0);
        assert!(pos.piece_bb(Color::Black, PieceKind::King).count() > 0);
    }
}
