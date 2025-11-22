
use crate::board::{position::Position,moves::*};
use crate::types::*;

pub mod standard;

pub trait Variant: std::fmt::Debug + Send + Sync {
    fn name(&self) -> &str;

    fn dimensions(&self) -> Dimensions;

    fn position(&self) -> &Position;

    fn position_mut(&mut self) -> &mut Position;

    fn legal_moves(&self) -> Vec<Move>;

    fn make_move(&mut self, mv: &Move) -> Position;

    fn outcome(&self) -> Option<GameOutcome>;

    fn is_legal_move(&self, mv: &Move) -> bool {
        self.legal_moves().contains(mv)
    }
}