

pub trait GameVariant{
    fn make_move(&mut self,mv: Move)->bool;
    fn is_game_over(&self)->bool;
    fn legal_moves(&self)->Vec<Move>;
}

pub enum VariantType{
    Standard(StandardGame),
    Antichess(AntichessGame)
}

pub struct StandardGame{
    
}