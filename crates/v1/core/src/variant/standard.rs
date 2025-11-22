use crate::board::position::Position;
use crate::variant::*;

#[derive(Debug)]
pub struct StandardVariant {
    dimensions: Dimensions,
    position: Position,
}

impl Variant for StandardVariant {
    fn name(&self) -> &str { "Standard" }

    fn dimensions(&self) -> Dimensions {
        self.dimensions.clone()
    }

    fn position(&self) -> &Position {
        &self.position
    }

    fn position_mut(&mut self) -> &mut Position {
        &mut self.position
    }

    fn legal_moves(&self) -> Vec<Move> {
        //movegen::generate_legal_moves(&self.position)
        todo!()
    }

    fn make_move(&mut self, mv: &Move) ->Position{
        //self.position = apply_move(&self.position, mv);
        todo!()
    }

    fn outcome(&self) -> Option<GameOutcome> {
        //detect_outcome(&self.position)
        todo!()
    }
}
