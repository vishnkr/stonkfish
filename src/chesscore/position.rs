use std::collections::HashMap;

use super::*;



#[derive(Debug,Serialize,Deserialize)]
pub struct Position {
    pub piece_locations: HashMap<Square, Piece>,
    pub piece_props: HashMap<Notation, PieceProps>,
    pub turn: Color,
    pub dimensions: Dimensions,
    pub wall_squares: HashMap<Square, bool>,
    pub castling_rights: u8,
    pub attacked_squares: HashMap<Square, bool>,
    pub additional_props: AdditionalProps,
    pub promotion_props: PromotionProps,
    pub ep_square: Option<Square>,
}


impl Position{
    pub fn new()->Self{
        Position{
            piece_locations: HashMap::new(),
            piece_props:HashMap::new(),
            turn: Color::WHITE,
            dimensions: Dimensions{ranks:8, files:8},
            wall_squares: HashMap::new(),
            castling_rights: 0,
            attacked_squares: HashMap::new(),
            additional_props: AdditionalProps{
                king_capture_allowed: false, 
                black_king_moved: false,
                white_king_moved: false,
                black_king_pos: None,
                white_king_pos: None
            },
            promotion_props: PromotionProps{ promotion_squares : HashMap::new(), allowed_promo_types: vec![]},
            ep_square: None,
        }
    }   

    pub fn from_config(config:GameConfig)->Result<Self,()>{
        let mut position = Position::new();
        position.load_from_fen(config.fen);
        position.dimensions = config.dimensions;
        position.piece_props = config.piece_props.unwrap();
        Ok(position)
    }

    pub fn load_from_fen(&mut self,fen:String){
        //let board_data:String = fen.split(" ").collect();
        let standard_piece_map:HashMap<char, PieceType> = HashMap::from([
            ('p',PieceType::Pawn),
            ('P',PieceType::Pawn),
            ('r',PieceType::Rook),
            ('R',PieceType::Rook),
            ('k',PieceType::King),
            ('K',PieceType::King),
            ('q',PieceType::Queen),
            ('Q',PieceType::Queen),
            ('b',PieceType::Bishop),
            ('B',PieceType::Bishop),
            ('n',PieceType::Knight),
            ('N',PieceType::Knight),
            ]);
        let is_standard_piece = |k:char|->bool{ standard_piece_map.contains_key(&k)} ;
        let mut fen_part = 0;
        let mut sec_digit = 0;
        let mut col = 0;
        let mut row = 0;
        let mut count;
        let mut castling_rights:u8 = 0;
        let mut id = 0;
        let mut ep_str = String::new();
        for (i,c) in fen.chars().enumerate(){
            if c==' '{
                fen_part+=1;
            }
            match fen_part{
                0=>{
                    if c=='/'{
                        row+=1;
                        sec_digit = 0;
                        continue;
                    } else if c.is_digit(10){
                        count = c.to_digit(10).unwrap_or(0);
                        if i+1<self.dimensions.files.into() && (fen.as_bytes()[i+1] as char).is_digit(10){
                            sec_digit = c.to_digit(10).unwrap_or(0);
                        } else {
                            id+=(sec_digit*10+count) as u8;
                            sec_digit=0;
                        }
                    } else if c=='.'{
                        self.wall_squares.insert(id, true);
                        id+=1;
                    } else {
                        let mut piece:Piece = Piece::new(PieceType::Custom, c, Color::WHITE);
                        if is_standard_piece(c){
                            piece.piece_type = standard_piece_map[&c.to_ascii_lowercase()];   
                        }
                        if c.is_ascii_lowercase(){ 
                            piece.player = Color::BLACK; 
                            if piece.piece_type ==PieceType::King{ self.additional_props.black_king_pos = Some(id)}
                        } else if piece.piece_type ==PieceType::King{ self.additional_props.white_king_pos = Some(id)}
                        self.piece_locations.insert(id, piece);
                        id+=1;
                    }
                },
                1 =>{
                    if c=='w'{
                        self.turn = Color::WHITE;
                    } else {
                        self.turn = Color::BLACK;
                    }
                },
                2 =>{
                    castling_rights |= match c {
                        'K'=>  1<<6,
                        'Q'=> 1<<4,
                        'k'=> 1<<2,
                        'q'=> 1,
                        _ => 0
                    }
                },
                3 =>{
                    if c.is_digit(10) {
                        ep_str.push(c);
                    }
                }
                4=>{
                    if !ep_str.is_empty() {
                        self.ep_square = Some(ep_str.parse().unwrap_or(0));
                    }
                }
                _=>{}
            }
        }
        self.attacked_squares = HashMap::new();
        self.castling_rights = castling_rights;
    }
}