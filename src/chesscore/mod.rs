use std::{collections::HashMap, vec};
use serde::{Serialize,Deserialize, de};
use serde_json::{Value, from_value};


use self::position::Position;

pub mod position;


#[derive(Debug, Clone, Copy, PartialEq,Serialize,Deserialize)]
enum ClassicMoveType {
    Quiet,
    Capture,
    DoublePawnPush,
    EnPassant,
    Castle,
    Promotion
}

#[derive(Debug, Clone, Copy, PartialEq,Serialize,Deserialize)]
enum VariantMoveType {
    DuckMove,
    Teleport
}


#[derive(Debug,Serialize,Deserialize)]
enum VariantType {
    #[serde(rename = "Checkmate")]
    Checkmate,
    #[serde(rename = "DuckChess")]
    DuckChess,
    #[serde(rename = "ArcherChess")]
    ArcherChess,
    #[serde(rename = "Wormhole")]
    Wormhole,
    #[serde(rename = "NCheck")]
    NCheck,
    #[serde(rename = "AntiChess")]
    AntiChess,
    #[serde(rename = "Capture")]
    CaptureTheKing,
    #[serde(rename = "GoalChess")]
    GoalChess
}

#[derive(Copy,Clone,Debug, PartialEq, Eq, Hash,Serialize,Deserialize)]
pub enum PieceType{
    Pawn,
	Knight,
	Bishop,
	Rook,
	Queen,
	King,
    Custom
}

pub type Notation = char;

#[derive(Copy, Clone,PartialEq,Debug,Eq,Hash,Serialize,Deserialize)]
pub enum Color{
    WHITE,
    BLACK
}

pub type Square = u8;

#[derive(Clone,Debug,PartialEq,Eq,Default,Serialize,Deserialize)]
pub struct PieceProps{
    pub jump_offsets:Vec<(i8,i8)>, 
    pub slide_directions:Vec<(i8,i8)>,
}

impl PieceProps{
    fn new()->Self{
        PieceProps{
            jump_offsets: vec![],
            slide_directions: vec![]
        }
    }
}

#[derive(Debug,Clone,PartialEq,Serialize,Deserialize)]
pub struct Piece{
    pub piece_type:PieceType,
    pub notation: Notation,
    pub player:Color,
}

impl Piece{
    pub fn new(piece_type:PieceType,notation:Notation,player:Color)->Self{
        Self{
            notation,
            piece_type,
            player,
        }
    }
}

#[derive(Debug,PartialEq,Clone,Serialize,Deserialize)]
pub struct Dimensions{
    pub ranks: Square,
    pub files: Square
}

#[derive(Debug,Serialize,Deserialize)]
pub struct AdditionalProps {
    king_capture_allowed: bool,
    black_king_moved: bool,
    white_king_moved: bool,
    black_king_pos: Option<Square>,
    white_king_pos: Option<Square>
}

#[derive(Debug,Serialize,Deserialize)]
pub struct PromotionProps{
    promotion_squares : HashMap<Square,bool>,
    allowed_promo_types: Vec<PieceType>
}

#[derive(Serialize,Deserialize,Debug)]
pub enum GameResult{
    BlackWins,
    WhiteWins,
    Stalemate
}

#[derive(Serialize,Deserialize)]
pub struct Move{
    src: Square,
    dest: Square,
    classic_move_type: ClassicMoveType,
    variant_move_type: Option<VariantMoveType>,
    piece: Piece

}
/*castling_rights |= match c {
                    'K'=>  1<<6,
                    'Q'=> 1<<4,
                    'k'=> 1<<2,
                    'q'=> 1,
                    _ => 0
                } */


#[derive(Debug,Clone,Serialize,Deserialize)]
struct RecentCaptureInfo{
    piece: Piece,
    square_id: u8,
    move_type: ClassicMoveType
}

#[derive(Debug,Deserialize)]
#[serde(tag = "variant_type")]
pub struct GameConfig{
    fen: String,
    dimensions: Dimensions,
    #[serde(default)]
    piece_props: Option<HashMap<Notation,PieceProps>>,
    #[serde(default)]
    additional_data: Option<AdditionalData>,
}   

#[derive(Deserialize,Debug)]
#[serde(tag = "additional_props")]
pub enum AdditionalData {
    NCheckProps{n:u8}
}

impl<'de> Deserialize<'de> for Variant{
    fn deserialize<D>(deserializer: D) -> Result<Self,D::Error>
        where
            D: serde::Deserializer<'de> {
        let value: Value = Deserialize::deserialize(deserializer)?;
        let variant = match value["variant_type"].as_str() {
            Some("Checkmate") => {
                let data = from_value(value).map_err(serde::de::Error::custom)?;
                let variant = CheckmateVariant::new(data);
                Variant::Checkmate(variant)
            },
            Some("AntiChess") => {
                let data = from_value(value).map_err(serde::de::Error::custom)?;
                let variant = AntichessVariant::new(data);
                Variant::AntiChess(variant)
            },
            Some("NCheck") => {
                let data = from_value(value).map_err(serde::de::Error::custom)?;
                let variant = NCheckVariant::new(data);
                Variant::NCheck(variant)
            },
            _ => return Err(serde::de::Error::custom("Unknown variant type")),
        };
        Ok(variant)
    }
}

#[derive(Debug)]
pub enum Variant{
    Checkmate(CheckmateVariant),
    AntiChess(AntichessVariant),
    NCheck(NCheckVariant)
}

impl VariantActions for Variant{
    fn make_move(&mut self,mv:Move)->bool{
        match self{
            Variant::Checkmate(checkmate_variant)=> checkmate_variant.make_move(mv),
            Variant::AntiChess(antichess_variant) => antichess_variant.make_move(mv),
            Variant::NCheck(ncheck_variant) => ncheck_variant.make_move(mv),
        }
    }

    fn unmake_move(&mut self, mv: Move)->bool {
        true
    }

    fn get_legal_moves(&self) -> Vec<Move> {
        let moves = vec![Move{
            src:5,
            dest:5,
            classic_move_type: ClassicMoveType::Capture,
            variant_move_type: None,
            piece: Piece::new(PieceType::Rook, 'r', Color::WHITE)
        }];
        moves
    }

    fn get_pseudo_legal_moves(&self, color: Color) -> Vec<Move> {
        let moves = vec![];
        moves
    }
    fn perform_move(&mut self, mv: Move) -> Result<bool,GameResult> {
        Ok(true)
    }
}

pub trait VariantActions {
    fn make_move(&mut self, mv: Move)->bool;
    fn unmake_move(&mut self, mv: Move)->bool;
    fn get_pseudo_legal_moves(&self, color: Color) -> Vec<Move>;
    fn get_legal_moves(&self) -> Vec<Move>;
    fn perform_move(&mut self, mv: Move) -> Result<bool, GameResult>;
}


pub fn load_config(config:String)->Option<Variant>{
    let variant: Variant = serde_json::from_str(config.as_str()).expect("Failed to deserialize Game Config JSON");
    Some(variant)
}

#[derive(Deserialize,Debug)]
pub struct DefaultVariant {
    position: Position,
    recent_capture: Option<RecentCaptureInfo>,
    game_result: Option<GameResult>,
}
impl DefaultVariant{
    pub fn new(config:GameConfig)->Self{
        let position = Position::from_config(config).unwrap();
        Self{
            position,    
            recent_capture: None,
            game_result: None,
        }
    }

    pub fn default()->Self{
        let position = Position::new();
        Self { position, recent_capture: None, game_result: None }
    }
}

impl VariantActions for DefaultVariant{
    fn make_move(&mut self,mv:Move)->bool{
        match mv.classic_move_type{
            ClassicMoveType::Quiet  => {
                if let Some(src_piece) = self.position.piece_locations.remove(&mv.src) {
                    self.position.piece_locations.insert(mv.dest, src_piece);
                } else{
                    return false;
                }
                if mv.piece.piece_type == PieceType::King{
                    if mv.piece.player==Color::BLACK{
                        self.position.additional_props.black_king_pos = Some(mv.dest);
                    } else{
                        self.position.additional_props.white_king_pos = Some(mv.dest);
                    }
                }
            },
            ClassicMoveType::Capture => {
                if let Some(src_piece) = self.position.piece_locations.remove(&mv.src) {
                    let dest_piece = self.position.piece_locations.remove(&mv.dest).unwrap();
                    self.recent_capture = Some(RecentCaptureInfo {
                        piece: dest_piece,
                        square_id: mv.dest,
                        move_type: ClassicMoveType::Capture,
                    });
                    self.position.piece_locations.remove(&mv.dest);
                    self.position.piece_locations.insert(mv.dest, src_piece);
                } else{
                    return false;
                }
            },
            
            ClassicMoveType::DoublePawnPush => {
                if let Some(src_piece) = self.position.piece_locations.remove(&mv.src) {
                    self.position.piece_locations.insert(mv.dest, src_piece);
                } else{
                    return false;
                }
            },

            ClassicMoveType::EnPassant => {
                if self.position.ep_square.is_some(){
                    let ep = self.position.ep_square.unwrap();
                    if let Some(src_piece) = self.position.piece_locations.remove(&mv.src) {
                        self.position.piece_locations.insert(mv.dest, src_piece);
                        let ep_piece = self.position.piece_locations.remove(&ep).unwrap();
                        self.recent_capture = Some(RecentCaptureInfo {
                            piece: ep_piece,
                            square_id: ep,
                            move_type: ClassicMoveType::EnPassant,
                        });
                        self.position.ep_square = None;
                    } else { return false; }
                } else { return false; }
            },

            ClassicMoveType::Castle => todo!(),
            ClassicMoveType::Promotion => todo!(),
        }
        true
    }

    fn unmake_move(&mut self, mv: Move)->bool {
        match mv.classic_move_type {
            ClassicMoveType::Quiet  => {
                if let Some(src_piece) = self.position.piece_locations.remove(&mv.dest) {
                    self.position.piece_locations.insert(mv.src, src_piece);
                } else{
                    return false;
                }
                if mv.piece.piece_type == PieceType::King{
                    if mv.piece.player==Color::BLACK{
                        self.position.additional_props.black_king_pos = Some(mv.src);
                    } else{
                        self.position.additional_props.white_king_pos = Some(mv.src);
                    }
                }
            },
            ClassicMoveType::Capture =>{
                if let Some(src_piece) = self.position.piece_locations.remove(&mv.dest) {
                    let dest_piece = self.recent_capture.clone().unwrap().piece;
                    self.recent_capture = None;
                    self.position.piece_locations.insert(mv.src, src_piece);
                    self.position.piece_locations.insert(mv.dest, dest_piece);
                } else{
                    return false;
                }
            },
            ClassicMoveType::DoublePawnPush => {
                if let Some(src_piece) = self.position.piece_locations.remove(&mv.dest) {
                    self.position.piece_locations.insert(mv.src, src_piece);
                } else{
                    return false;
                }
            },
            ClassicMoveType::EnPassant => {
                let rc_info = self.recent_capture.clone().unwrap();
                if let Some(src_piece) = self.position.piece_locations.remove(&mv.dest) {
                    self.position.piece_locations.insert(mv.src, src_piece);
                    self.recent_capture = None;
                    self.position.ep_square = Some(rc_info.square_id);
                } else { return false; }
            },
            ClassicMoveType::Castle => todo!(),
            ClassicMoveType::Promotion => todo!(),
        }
        true
    }

    fn get_legal_moves(&self) -> Vec<Move> {
        let moves = vec![Move{
            src:5,
            dest:5,
            classic_move_type: ClassicMoveType::Capture,
            variant_move_type: None,
            piece: Piece::new(PieceType::Rook, 'r', Color::WHITE)
        }];
        moves
    }

    fn get_pseudo_legal_moves(&self, color: Color) -> Vec<Move> {
        let moves = vec![];
        moves
    }
    fn perform_move(&mut self, mv: Move) -> Result<bool, GameResult> {
        Ok(true)
    }

}

#[derive(Debug)]
pub struct CheckmateVariant{
    variant: DefaultVariant
}

impl CheckmateVariant{
    pub fn new(config:GameConfig)->Self{
        Self{
            variant: DefaultVariant::new(config)
        }
    }

    fn make_move(&mut self, mv: Move)->bool {
        self.variant.make_move(mv)
    }
    fn unmake_move(&mut self, mv: Move)->bool {
        self.variant.unmake_move(mv)
    }
    fn get_legal_moves(&self) -> Vec<Move> {
        self.variant.get_legal_moves()
    }
    fn get_pseudo_legal_moves(&self, color: Color) -> Vec<Move> {
        self.variant.get_legal_moves()
    }
    fn perform_move(&mut self, mv: Move) -> Result<bool, GameResult> {
        self.variant.perform_move(mv)
    }
}

#[derive(Debug)]
pub struct AntichessVariant{
    variant: DefaultVariant
}

impl AntichessVariant{
    pub fn new(config:GameConfig)->Self{
        Self{
            variant: DefaultVariant::new(config)
        }
    }
    fn make_move(&mut self, mv: Move)->bool {
        self.variant.make_move(mv)
    }
    fn unmake_move(&mut self, mv: Move)->bool {
        self.variant.unmake_move(mv)
    }
    fn get_legal_moves(&self) -> Vec<Move> {
        self.variant.get_legal_moves()
    }
    fn get_pseudo_legal_moves(&self, color: Color) -> Vec<Move> {
        self.variant.get_legal_moves()
    }
    fn perform_move(&mut self, mv: Move) -> Result<bool, GameResult> {
        self.variant.perform_move(mv)
    }

}

#[derive(Debug)]
pub struct NCheckVariant{
    variant: DefaultVariant,
    n: u8
}

impl NCheckVariant{
    pub fn new(config:GameConfig)->Self{
        Self{
            variant: DefaultVariant::new(config),
            n: 3
        }
    }
    fn make_move(&mut self, mv: Move)->bool {
        self.variant.make_move(mv)
    }
    fn unmake_move(&mut self, mv: Move)->bool {
        self.variant.unmake_move(mv)
    }
    fn get_legal_moves(&self) -> Vec<Move> {
        self.variant.get_legal_moves()
    }
    fn get_pseudo_legal_moves(&self, color: Color) -> Vec<Move> {
        self.variant.get_legal_moves()
    }
    fn perform_move(&mut self, mv: Move) -> Result<bool, GameResult> {
        self.variant.perform_move(mv)
    }
}


#[cfg(test)]
mod chesscore_test{
    use serde_json::json;

    use crate::chesscore::Variant;
    #[test]
    pub fn setup_variant(){
        let json = json!({
            "variant_type":"AntiChess",  
            "fen":"rnbqkbnr/pppppp../8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq 34 0 1",
            "dimensions": { "ranks":8, "files":8},
            "piece_props":{}
        });
        let result:Variant = serde_json::from_value(json).unwrap();
        println!("{:#?}", result);
    }
}