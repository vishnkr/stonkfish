use serde::{Deserialize, Serialize};
use serde_json::{from_value, Value};

use std::{collections::HashMap, vec};

use self::position::Position;

pub mod position;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
enum ClassicMoveType {
    Quiet,
    Capture,
    DoublePawnPush,
    EnPassant,
    Castle,
    Promotion,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
enum VariantMoveType {
    DuckMove,
    Teleport,
}

#[derive(Debug, Serialize, Deserialize)]
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
    GoalChess,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
    Custom,
}

pub type Notation = char;

#[derive(Copy, Clone, PartialEq, Debug, Eq, Hash, Serialize, Deserialize)]
pub enum Color {
    #[serde(rename = "white")]
    WHITE,
    #[serde(rename = "black")]
    BLACK,
}

pub type Square = u32;
pub type Delta = isize;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PieceProps {
    pub jump_offsets: Vec<(Delta, Delta)>,
    pub slide_directions: Vec<(Delta, Delta)>,
}

impl Default for PieceProps {
    fn default() -> Self {
        PieceProps {
            jump_offsets: Vec::new(),
            slide_directions: Vec::new(),
        }
    }
}

impl PieceProps {
    fn new(
        jump_offsets: Option<Vec<(Delta, Delta)>>,
        slide_directions: Option<Vec<(Delta, Delta)>>,
    ) -> Self {
        let jump_offsets = jump_offsets.unwrap_or_else(Vec::new);
        let slide_directions = slide_directions.unwrap_or_else(Vec::new);

        PieceProps {
            jump_offsets,
            slide_directions,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Piece {
    pub piece_type: PieceType,
    pub notation: Notation,
    pub player: Color,
}

impl Piece {
    pub fn new(piece_type: PieceType, notation: Notation, player: Color) -> Self {
        Self {
            notation,
            piece_type,
            player,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Dimensions {
    pub ranks: Square,
    pub files: Square,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalProps {
    king_capture_allowed: bool,
    black_king_moved: bool,
    white_king_moved: bool,
    black_king_pos: Option<Square>,
    white_king_pos: Option<Square>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromotionProps {
    white_promotion_squares: HashMap<Square, bool>,
    black_promotion_squares: HashMap<Square, bool>,
    allowed_promo_types: Vec<PieceType>,
}

impl PromotionProps {
    pub fn default() -> Self {
        PromotionProps {
            white_promotion_squares: HashMap::from([
                (0, true),
                (1, true),
                (2, true),
                (3, true),
                (4, true),
                (5, true),
                (6, true),
                (7, true),
            ]),
            black_promotion_squares: HashMap::from([
                (56, true),
                (57, true),
                (58, true),
                (59, true),
                (60, true),
                (61, true),
                (62, true),
                (63, true),
            ]),
            allowed_promo_types: vec![],
        }
    }
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum GameResult {
    BlackWins,
    WhiteWins,
    Stalemate,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Move {
    src: Square,
    dest: Square,
    classic_move_type: ClassicMoveType,
    variant_move_type: Option<VariantMoveType>,
    piece: Piece,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RecentCaptureInfo {
    piece: Piece,
    square_id: Square,
    move_type: ClassicMoveType,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "variant_type")]
pub struct GameConfig {
    fen: String,
    dimensions: Dimensions,
    #[serde(default)]
    piece_props: Option<HashMap<Notation, PieceProps>>,
    #[serde(default)]
    additional_data: Option<AdditionalData>,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "additional_props")]
pub enum AdditionalData {
    NCheckProps { n: u8 },
}

impl<'de> Deserialize<'de> for Variant {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value: Value = Deserialize::deserialize(deserializer)?;
        let variant = match value["variant_type"].as_str() {
            Some("Checkmate") => {
                let data = from_value(value).map_err(serde::de::Error::custom)?;
                let variant = CheckmateVariant::new(data);
                Variant::Checkmate(variant)
            }
            Some("AntiChess") => {
                let data = from_value(value).map_err(serde::de::Error::custom)?;
                let variant = AntichessVariant::new(data);
                Variant::AntiChess(variant)
            }
            Some("NCheck") => {
                let data = from_value(value).map_err(serde::de::Error::custom)?;
                let variant = NCheckVariant::new(data);
                Variant::NCheck(variant)
            }
            _ => return Err(serde::de::Error::custom("Unknown variant type")),
        };
        Ok(variant)
    }
}

#[derive(Debug)]
pub enum Variant {
    Checkmate(CheckmateVariant),
    AntiChess(AntichessVariant),
    NCheck(NCheckVariant),
}

pub trait VariantActions {
    fn make_move(&mut self, mv: &Move) -> bool;
    fn unmake_move(&mut self, mv: &Move) -> bool;
    fn get_pseudo_legal_moves(&self, color: Color) -> Vec<Move>;
    fn get_legal_moves(&mut self) -> Vec<Move>;
    fn is_game_over(&self) -> Option<GameResult>;
    fn is_legal_move(&mut self, mv: &Move) -> bool;
}

impl VariantActions for Variant {
    fn make_move(&mut self, mv: &Move) -> bool {
        match self {
            Variant::Checkmate(checkmate_variant) => checkmate_variant.make_move(mv),
            Variant::AntiChess(antichess_variant) => antichess_variant.make_move(mv),
            Variant::NCheck(ncheck_variant) => ncheck_variant.make_move(mv),
        }
    }

    fn unmake_move(&mut self, mv: &Move) -> bool {
        match self {
            Variant::Checkmate(checkmate_variant) => checkmate_variant.unmake_move(mv),
            Variant::AntiChess(antichess_variant) => antichess_variant.unmake_move(mv),
            Variant::NCheck(ncheck_variant) => ncheck_variant.unmake_move(mv),
        }
    }

    fn get_legal_moves(&mut self) -> Vec<Move> {
        match self {
            Variant::Checkmate(checkmate_variant) => checkmate_variant.get_legal_moves(),
            Variant::AntiChess(antichess_variant) => antichess_variant.get_legal_moves(),
            Variant::NCheck(ncheck_variant) => ncheck_variant.get_legal_moves(),
        }
    }

    fn get_pseudo_legal_moves(&self, color: Color) -> Vec<Move> {
        match self {
            Variant::Checkmate(checkmate_variant) => {
                checkmate_variant.get_pseudo_legal_moves(color)
            }
            Variant::AntiChess(antichess_variant) => {
                antichess_variant.get_pseudo_legal_moves(color)
            }
            Variant::NCheck(ncheck_variant) => ncheck_variant.get_pseudo_legal_moves(color),
        }
    }

    fn is_game_over(&self) -> Option<GameResult> {
        match self {
            Variant::Checkmate(checkmate_variant) => checkmate_variant.is_game_over(),
            Variant::AntiChess(antichess_variant) => antichess_variant.is_game_over(),
            Variant::NCheck(ncheck_variant) => ncheck_variant.is_game_over(),
            _ => None,
        }
    }
    fn is_legal_move(&mut self, mv: &Move) -> bool {
        match self {
            Variant::Checkmate(checkmate_variant) => checkmate_variant.is_legal_move(mv),
            Variant::AntiChess(antichess_variant) => antichess_variant.is_legal_move(mv),
            Variant::NCheck(ncheck_variant) => ncheck_variant.is_legal_move(mv),
            _ => false,
        }
    }
}

pub fn load_config(config: String) -> Option<Variant> {
    let variant: Variant =
        serde_json::from_str(config.as_str()).expect("Failed to deserialize Game Config JSON");
    Some(variant)
}

#[derive(Deserialize, Debug)]
pub struct DefaultVariant {
    position: Position,
    recent_capture: Option<RecentCaptureInfo>,
    game_result: Option<GameResult>,
}

impl DefaultVariant {
    pub fn new(config: GameConfig) -> Self {
        let position = Position::from_config(config).unwrap();
        Self {
            position,
            recent_capture: None,
            game_result: None,
        }
    }

    pub fn default() -> Self {
        let position = Position::new();
        Self {
            position,
            recent_capture: None,
            game_result: None,
        }
    }
}

impl VariantActions for DefaultVariant {
    fn make_move(&mut self, mv: &Move) -> bool {
        match mv.classic_move_type {
            ClassicMoveType::Quiet => {
                if let Some(src_piece) = self.position.piece_locations.remove(&mv.src) {
                    self.position.piece_locations.insert(mv.dest, src_piece);
                } else {
                    return false;
                }
                if mv.piece.piece_type == PieceType::King {
                    if mv.piece.player == Color::BLACK {
                        self.position.additional_props.black_king_pos = Some(mv.dest);
                    } else {
                        self.position.additional_props.white_king_pos = Some(mv.dest);
                    }
                }
            }
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
                } else {
                    return false;
                }
            }

            ClassicMoveType::DoublePawnPush => {
                if let Some(src_piece) = self.position.piece_locations.remove(&mv.src) {
                    self.position.piece_locations.insert(mv.dest, src_piece);
                } else {
                    return false;
                }
            }

            ClassicMoveType::EnPassant => {
                if self.position.ep_square.is_some() {
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
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            }

            ClassicMoveType::Castle => todo!(),
            ClassicMoveType::Promotion => todo!(),
        }
        true
    }

    fn unmake_move(&mut self, mv: &Move) -> bool {
        match mv.classic_move_type {
            ClassicMoveType::Quiet => {
                if let Some(src_piece) = self.position.piece_locations.remove(&mv.dest) {
                    self.position.piece_locations.insert(mv.src, src_piece);
                } else {
                    return false;
                }
                if mv.piece.piece_type == PieceType::King {
                    if mv.piece.player == Color::BLACK {
                        self.position.additional_props.black_king_pos = Some(mv.src);
                    } else {
                        self.position.additional_props.white_king_pos = Some(mv.src);
                    }
                }
            }
            ClassicMoveType::Capture => {
                if let Some(src_piece) = self.position.piece_locations.remove(&mv.dest) {
                    let dest_piece = self.recent_capture.clone().unwrap().piece;
                    self.recent_capture = None;
                    self.position.piece_locations.insert(mv.src, src_piece);
                    self.position.piece_locations.insert(mv.dest, dest_piece);
                } else {
                    return false;
                }
            }
            ClassicMoveType::DoublePawnPush => {
                if let Some(src_piece) = self.position.piece_locations.remove(&mv.dest) {
                    self.position.piece_locations.insert(mv.src, src_piece);
                } else {
                    return false;
                }
            }
            ClassicMoveType::EnPassant => {
                let rc_info = self.recent_capture.clone().unwrap();
                if let Some(src_piece) = self.position.piece_locations.remove(&mv.dest) {
                    self.position.piece_locations.insert(mv.src, src_piece);
                    self.recent_capture = None;
                    self.position.ep_square = Some(rc_info.square_id);
                } else {
                    return false;
                }
            }
            ClassicMoveType::Castle => todo!(),
            ClassicMoveType::Promotion => todo!(),
        }
        true
    }

    fn get_legal_moves(&mut self) -> Vec<Move> {
        let pseudo_moves = self.get_pseudo_legal_moves(self.position.turn);
        let legal_moves = pseudo_moves
            .into_iter()
            .filter(|mv| {
                if mv.classic_move_type == ClassicMoveType::Capture {
                    if let Some(piece) = self.position.piece_locations.get(&mv.dest) {
                        if piece.piece_type == PieceType::King {
                            return false;
                        }
                    }
                }
                self.make_move(mv);
                let is_legal = !self.is_king_under_check();
                self.unmake_move(mv);

                is_legal
            })
            .collect();
        legal_moves
    }

    fn get_pseudo_legal_moves(&self, color: Color) -> Vec<Move> {
        let mut moves = vec![];
        for (sq, piece) in &self.position.piece_locations {
            if color != piece.player {
                continue;
            }
            match piece.piece_type {
                PieceType::Pawn => {
                    let mut pawn_moves = self.gen_pawn_moves(sq);
                    moves.append(&mut pawn_moves);
                }
                PieceType::King => {
                    moves.append(&mut self.gen_king_moves(piece, sq));
                }
                _ => {
                    moves.append(&mut self.gen_slide_moves(piece, sq));
                    moves.append(&mut self.gen_jump_moves(piece, sq));
                }
            }
        }
        moves
    }

    fn is_game_over(&self) -> Option<GameResult> {
        self.game_result.clone()
    }
    fn is_legal_move(&mut self, mv: &Move) -> bool {
        true
    }
}

impl DefaultVariant {
    fn gen_pawn_moves(&self, sq: &Square) -> Vec<Move> {
        let mut moves = vec![];
        let piece = self.position.piece_locations.get(sq).unwrap();
        let row_offset: Delta;
        let double_start_rank: Square;
        match piece.player {
            Color::WHITE => {
                row_offset = -1;
                double_start_rank = self.position.dimensions.ranks - 2;
            }
            Color::BLACK => {
                row_offset = 1;
                double_start_rank = 1;
            }
        }
        let src = self.position.to_row_col(sq);
        if let Ok(target_row) = self.position.add_delta_row(src.0, row_offset) {
            for i in -1..=1 {
                if i == 0 {
                    let t1 = self.position.to_pos(&target_row, &src.1);

                    if self.position.is_sq_empty(&t1) {
                        moves.push(Move {
                            src: *sq,
                            dest: t1,
                            classic_move_type: ClassicMoveType::Quiet,
                            variant_move_type: None,
                            piece: piece.clone(),
                        })
                    }
                    if let Ok(target_row_2) = self.position.add_delta_row(target_row, row_offset) {
                        let t2 = self.position.to_pos(&target_row_2, &src.1);
                        if self.position.is_sq_empty(&t1)
                            && self.position.is_sq_empty(&t2)
                            && src.0 == double_start_rank
                        {
                            moves.push(Move {
                                src: *sq,
                                dest: t2,
                                classic_move_type: ClassicMoveType::DoublePawnPush,
                                variant_move_type: None,
                                piece: piece.clone(),
                            })
                        }
                    }
                } else {
                    if let Ok(target_col) = self.position.add_delta_col(src.1, i as isize) {
                        let target = self.position.to_pos(&target_row, &target_col);
                        if let Some(opp) = self.position.piece_locations.get(&target) {
                            if !self.is_opp_king(piece.player, opp) {
                                moves.push(Move {
                                    src: *sq,
                                    dest: target,
                                    classic_move_type: ClassicMoveType::Capture,
                                    variant_move_type: None,
                                    piece: piece.clone(),
                                })
                            }
                        } else if self.position.is_sq_empty(sq)
                            && self.position.ep_square == Some(target)
                        {
                            moves.push(Move {
                                src: *sq,
                                dest: target,
                                classic_move_type: ClassicMoveType::EnPassant,
                                variant_move_type: None,
                                piece: piece.clone(),
                            })
                        }
                    }
                }
            }
        }
        moves
    }

    fn gen_king_moves(&self, piece: &Piece, sq: &Square) -> Vec<Move> {
        let mut moves = vec![];
        for row_delta in -1..=1 {
            for col_delta in -1..=1 {
                if row_delta == 0 && col_delta == 0 {
                    continue;
                }
                if let Some(target) = self.get_target(sq, &(row_delta, col_delta)) {
                    if self.position.is_sq_empty(&target) {
                        moves.push(Move {
                            src: *sq,
                            dest: target,
                            classic_move_type: ClassicMoveType::Quiet,
                            variant_move_type: None,
                            piece: piece.clone(),
                        });
                    } else if let Some(occ) = self.position.piece_locations.get(&target) {
                        if occ.player != piece.player && occ.piece_type != PieceType::King {
                            moves.push(Move {
                                src: *sq,
                                dest: target,
                                classic_move_type: ClassicMoveType::Capture,
                                variant_move_type: None,
                                piece: piece.clone(),
                            });
                        }
                    }
                }
            }
        }

        moves
    }

    fn get_target(&self, sq: &Square, offset: &(Delta, Delta)) -> Option<Square> {
        let src = self.position.to_row_col(sq);
        let target_x = &self.position.add_delta_row(src.0, offset.0);
        let target_y = &self.position.add_delta_col(src.1, offset.1);
        match (target_x, target_y) {
            (Ok(x), Ok(y)) => return Some(self.position.to_pos(x, y)),
            _ => return None,
        }
    }

    #[inline]
    fn total_sq(&self) -> u32 {
        self.position.dimensions.ranks * self.position.dimensions.files - 1
    }

    fn get_attacked_squares(&self, moves: Vec<Move>) -> HashMap<Square, bool> {
        let mut attacked_squares = HashMap::new();
        for mv in moves {
            match mv.classic_move_type {
                ClassicMoveType::Quiet => {
                    if mv.piece.piece_type != PieceType::Pawn {
                        attacked_squares.insert(mv.dest, true);
                    }
                }
                ClassicMoveType::Capture => {
                    attacked_squares.insert(mv.dest, true);
                }
                _ => {}
            }
        }
        attacked_squares
    }

    pub fn is_king_under_check(&self) -> bool {
        let opp_moves = self.get_pseudo_legal_moves(self.position.get_opponent_color());

        if let Some(king_pos) = match self.position.turn {
            Color::BLACK => self.position.additional_props.black_king_pos,
            Color::WHITE => self.position.additional_props.white_king_pos,
        } {
            let attacked_squares = self.get_attacked_squares(opp_moves);
            if let Some(_) = attacked_squares.get(&king_pos) {
                return true;
            }
        }
        false
    }

    fn gen_slide_moves(&self, piece: &Piece, sq: &Square) -> Vec<Move> {
        let mut moves = vec![];
        let piece_char = &piece.notation.to_lowercase().next();
        let option = self.position.piece_props.get(&piece_char.unwrap());
        let piece_props = option.unwrap();
        for offset in &piece_props.slide_directions {
            if let Some(mut target) = self.get_target(sq, offset) {
                while target < self.total_sq() {
                    match self.position.piece_locations.get(&target) {
                        Some(dest_piece) => {
                            if dest_piece.player != piece.player {
                                moves.push(Move {
                                    src: *sq,
                                    dest: target,
                                    classic_move_type: ClassicMoveType::Capture,
                                    variant_move_type: None,
                                    piece: piece.clone(),
                                });
                            }
                            break;
                        }
                        None => moves.push(Move {
                            src: *sq,
                            dest: target,
                            classic_move_type: ClassicMoveType::Quiet,
                            variant_move_type: None,
                            piece: piece.clone(),
                        }),
                    }
                    let new_target = self.get_target(&target, offset);
                    if new_target.is_none() {
                        break;
                    } else {
                        target = new_target.unwrap()
                    }
                }
            }
        }
        moves
    }

    fn gen_jump_moves(&self, piece: &Piece, sq: &Square) -> Vec<Move> {
        let mut moves = vec![];
        let piece_char = &piece.notation.to_lowercase().next();
        let option = self.position.piece_props.get(&piece_char.unwrap());
        let piece_props = option.unwrap();
        for offset in &piece_props.jump_offsets {
            if let Some(target) = self.get_target(sq, offset) {
                if target < self.total_sq() {
                    match self.position.piece_locations.get(&target) {
                        Some(dest_piece) => {
                            if dest_piece.player != piece.player {
                                if dest_piece.piece_type == PieceType::King {
                                    break;
                                }
                                moves.push(Move {
                                    src: *sq,
                                    dest: target,
                                    classic_move_type: ClassicMoveType::Capture,
                                    variant_move_type: None,
                                    piece: piece.clone(),
                                })
                            } else {
                                break;
                            }
                        }
                        None => moves.push(Move {
                            src: *sq,
                            dest: target,
                            classic_move_type: ClassicMoveType::Quiet,
                            variant_move_type: None,
                            piece: piece.clone(),
                        }),
                    }
                }
            }
        }
        moves
    }

    fn is_opp_king(&self, color: Color, piece: &Piece) -> bool {
        color != piece.player && piece.piece_type == PieceType::King
    }
}

#[derive(Debug)]
pub struct CheckmateVariant {
    variant: DefaultVariant,
}

impl CheckmateVariant {
    pub fn new(config: GameConfig) -> Self {
        Self {
            variant: DefaultVariant::new(config),
        }
    }

    pub fn make_move(&mut self, mv: &Move) -> bool {
        self.variant.make_move(mv)
    }
    pub fn unmake_move(&mut self, mv: &Move) -> bool {
        self.variant.unmake_move(mv)
    }
    pub fn get_legal_moves(&mut self) -> Vec<Move> {
        self.variant.get_legal_moves()
    }
    pub fn get_pseudo_legal_moves(&self, color: Color) -> Vec<Move> {
        self.variant.get_pseudo_legal_moves(color)
    }
    pub fn is_game_over(&self) -> Option<GameResult> {
        self.variant.is_game_over()
    }
    pub fn is_legal_move(&mut self, mv: &Move) -> bool {
        self.variant.is_legal_move(mv)
    }
}

#[derive(Debug)]
pub struct AntichessVariant {
    variant: DefaultVariant,
}

impl AntichessVariant {
    pub fn new(config: GameConfig) -> Self {
        Self {
            variant: DefaultVariant::new(config),
        }
    }
    pub fn make_move(&mut self, mv: &Move) -> bool {
        self.variant.make_move(mv)
    }
    pub fn unmake_move(&mut self, mv: &Move) -> bool {
        self.variant.unmake_move(mv)
    }

    pub fn get_legal_moves(&mut self) -> Vec<Move> {
        let pseudo_moves = self
            .variant
            .get_pseudo_legal_moves(self.variant.position.turn);
        //log::info!("anti {:?}",pseudo_moves);
        let capture_moves: Vec<Move> = pseudo_moves
            .iter()
            .cloned()
            .filter(|mv| mv.classic_move_type == ClassicMoveType::Capture)
            .collect();
        //log::info!("anti captures {:?}",capture_moves);
        if !capture_moves.is_empty() {
            return capture_moves;
        }
        pseudo_moves
    }

    pub fn get_pseudo_legal_moves(&self, color: Color) -> Vec<Move> {
        self.variant.get_pseudo_legal_moves(color)
    }
    pub fn is_game_over(&self) -> Option<GameResult> {
        self.variant.is_game_over()
    }
    pub fn is_legal_move(&mut self, mv: &Move) -> bool {
        self.variant.is_legal_move(mv)
    }
}

#[derive(Debug)]
pub struct NCheckVariant {
    variant: DefaultVariant,
    n: u8,
}

impl NCheckVariant {
    pub fn new(config: GameConfig) -> Self {
        Self {
            variant: DefaultVariant::new(config),
            n: 3,
        }
    }
    pub fn make_move(&mut self, mv: &Move) -> bool {
        self.variant.make_move(mv)
    }
    pub fn unmake_move(&mut self, mv: &Move) -> bool {
        self.variant.unmake_move(mv)
    }
    pub fn get_legal_moves(&mut self) -> Vec<Move> {
        self.variant.get_legal_moves()
    }
    pub fn get_pseudo_legal_moves(&self, color: Color) -> Vec<Move> {
        self.variant.get_pseudo_legal_moves(color)
    }
    pub fn is_game_over(&self) -> Option<GameResult> {
        self.variant.is_game_over()
    }
    pub fn is_legal_move(&mut self, mv: &Move) -> bool {
        self.variant.is_legal_move(mv)
    }
}

#[cfg(test)]
mod chesscore_test {
    use serde_json::json;

    use crate::{Variant,VariantActions};

    #[test]
    pub fn setup_variant() {
        let json = json!({
            "variant_type":"Checkmate",
            "fen":"r3k3/8/8/8/4G3/8/8/4K3 w - - 0 1",
            "dimensions": { "ranks":8, "files":8},
            "piece_props":{"g":{"slide_directions":[(1,0),(-1,0)],"jump_offsets":[]}}
        });
        println!("{:?}", json);
        let mut result: Variant = serde_json::from_value(json).unwrap();

        println!("{:#?}", result);
        println!("Moves: {:#?}", result.get_legal_moves());
    }
}