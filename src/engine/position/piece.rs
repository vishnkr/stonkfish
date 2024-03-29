use std::{collections::{HashSet}, convert::TryInto};
use crate::{engine::{bitboard::{Bitboard, to_pos}}};
use super::{Color, Dimensions};


#[derive(Copy,Clone, PartialEq, Eq, Hash)]
pub enum PieceType{
    Pawn,
	Knight,
	Bishop,
	Rook,
	Queen,
	King,
    Custom
}

pub type PieceRepr = char;

#[derive(Debug,PartialEq)]
pub struct Piece{
    pub piece_type:PieceType,
    pub bitboard: Bitboard,
    pub piece_repr: PieceRepr,
    pub player:Color,
    pub props:PieceProps

}

impl Piece{
    pub fn new_piece(color:Color, repr:char,dimensions:&Dimensions) -> Self{
        let piece_type:PieceType;
        let piece_props:PieceProps;
        match repr.to_ascii_lowercase(){
            'p'=>  { piece_type = PieceType::Pawn; piece_props = PieceProps::create_pawn(color,dimensions)},
            'r'=> {piece_type = PieceType::Rook; piece_props = PieceProps::create_rook()},
            'k'=> {piece_type = PieceType::King; piece_props = PieceProps::create_king()},
            'q'=> {piece_type = PieceType::Queen; piece_props = PieceProps::create_queen()},
            'b'=> {piece_type = PieceType::Bishop; piece_props = PieceProps::create_bishop()},
            'n'=> {piece_type = PieceType::Knight; piece_props = PieceProps::create_knight()},
            _=> {piece_type = PieceType::Custom; piece_props = PieceProps::create_default()},
        }
         Piece{
            bitboard:Bitboard::zero(),
            player:color,
            piece_repr:repr,
            piece_type, 
            props:piece_props
        }
    }
}



#[derive(Clone,Debug,PartialEq,Eq,Default)]
pub struct PieceProps{
    // jump moves that can capture and move
    pub jump_offsets:Vec<(i8,i8)>, 
    pub slide_directions:Vec<(i8,i8)>,
    
    pub can_double_jump: bool,
    // squares from which a piece can perform a non capture double jump move  using any of the jump offset definitions (ex: pawn double jump from starting pos)
    pub double_jump_squares: Option<HashSet<u8>>,
    // similar to jump except a piece can only mnove to the target if its a capture move
    pub capture_only_jump_offsets: Vec<(i8,i8)>,
    pub non_capture_only_jump_offsets: Vec<(i8,i8)>,
    pub can_promote: bool,
    pub promotion_squares: Option<Vec<(i8,i8)>>,

    pub can_en_passant:bool
}


impl PieceProps{

    pub fn create_default()->PieceProps{
        PieceProps { 
            jump_offsets: vec![], 
            slide_directions: vec![],
            can_double_jump: false,
            double_jump_squares: Some(HashSet::new()),
            capture_only_jump_offsets: vec![],
            non_capture_only_jump_offsets: vec![],
            can_promote: false,
            promotion_squares: None ,
            can_en_passant: false 
        }
    }

    pub fn create_pawn(color:Color, dimensions:&Dimensions)-> PieceProps{
        let mut double_jump_squares:HashSet<u8> = HashSet::new();
        let promotion_rank:i8;
        let double_jump_rank:u8;
        let mut promotion_squares:Vec<(i8,i8)> = vec![];
        let capture_rank_dir: i8;
        match color{
            Color::BLACK=>{
                double_jump_rank = 1;
                promotion_rank = (dimensions.height-1) as i8;
                capture_rank_dir = 1;
            },
            Color::WHITE=>{
                double_jump_rank = dimensions.height-2;
                promotion_rank = 0;
                capture_rank_dir = -1;
            }
        }
        for i in 0..(dimensions.width as i8){
            promotion_squares.push((promotion_rank,i));
            double_jump_squares.insert(to_pos(double_jump_rank,i as u8).try_into().unwrap());
        }

        PieceProps { 
            jump_offsets: vec![], 
            slide_directions: vec![], 
            can_double_jump: true, 
            double_jump_squares: Some(double_jump_squares), 
            capture_only_jump_offsets: vec![(capture_rank_dir,-1),(capture_rank_dir,1)],
            non_capture_only_jump_offsets: vec![(capture_rank_dir,0)], 
            can_promote: true ,
            promotion_squares:Some(promotion_squares), 
            can_en_passant: true,
        }
    }

    pub fn create_bishop()->PieceProps{
        PieceProps { 
            jump_offsets: vec![], 
            slide_directions: vec![(-1,-1),(1,1),(-1,1),(1,-1)], 
            can_double_jump: false, 
            double_jump_squares: None, 
            capture_only_jump_offsets: vec![],
            non_capture_only_jump_offsets: vec![], 
            can_promote: false, 
            promotion_squares: None, 
            can_en_passant: false
        }
    }

    pub fn create_rook()->PieceProps{
        PieceProps { 
            jump_offsets: vec![], 
            slide_directions: vec![(-1,0),(1,0),(0,1),(0,-1)], 
            can_double_jump: false, 
            double_jump_squares: None, 
            capture_only_jump_offsets: vec![],
            non_capture_only_jump_offsets: vec![],
            can_promote: false, 
            promotion_squares: None, 
            can_en_passant: false
        }
    }

    pub fn create_queen()->PieceProps{
        PieceProps { 
            jump_offsets: vec![], 
            slide_directions: vec![(-1,0),(1,0),(0,1),(0,-1),(-1,-1),(1,1),(-1,1),(1,-1)], 
            can_double_jump: false, 
            double_jump_squares: None, 
            capture_only_jump_offsets: vec![],
            non_capture_only_jump_offsets: vec![],
            can_promote: false, 
            promotion_squares: None, 
            can_en_passant: false
        }
    }

    pub fn create_knight()->PieceProps{
        PieceProps { 
            jump_offsets: vec![(1, 2), (1, -2), (-1, 2), (-1, -2), (2, 1), (2, -1), (-2, 1), (-2, -1)], 
            slide_directions: vec![], 
            can_double_jump: false, 
            double_jump_squares: None, 
            capture_only_jump_offsets: vec![],
            non_capture_only_jump_offsets: vec![],
            can_promote: false, 
            promotion_squares: None, 
            can_en_passant: false
        }
    }
    pub fn create_king()->PieceProps{
        PieceProps { 
            jump_offsets: vec![(1, 0), (1, 1), (-1, -1), (-1, 0), (0, 1), (0, -1), (-1,1), (1, -1)], 
            slide_directions: vec![], 
            can_double_jump: false, 
            double_jump_squares: None, 
            capture_only_jump_offsets: vec![],
            non_capture_only_jump_offsets: vec![],
            can_promote: false, 
            promotion_squares: None, 
            can_en_passant: false
        }
    }

}