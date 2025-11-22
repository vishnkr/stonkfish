use std::collections::HashMap;

use crate::{bitboard::*, types::*};


#[derive(Debug)]
pub struct Position {
    pub dimensions: Dimensions,
    pub fen: String,
    pub white_to_move: bool,
    pub castling: u8,
    pub en_passant: Option<usize>,
    pub halfmove_clock: u32,
    pub fullmove_number: u32,

    pub pieces: HashMap<char, BitBoard>,
    pub walls: BitBoard,
    pub occupied: BitBoard,
    pub color_bitboards: HashMap<Color, BitBoard>,
    pub piece_kinds: HashMap<char, PieceKind>,
    //pub custom_piece_rules: HashMap<char, Vec<MovePattern>>,
}


impl Position {
    pub fn from_fen(fen: &str) -> Result<Self, String> {
        let parts: Vec<&str> = fen.split_whitespace().collect();
        if parts.len() < 4 {
            return Err("FEN must have at least 4 fields".to_string());
        }

        let board_part = parts[0];
        let white_to_move = parts[1] == "w";
        let castling = parse_castling_rights(parts[2]);
        let en_passant = parse_en_passant(parts[3])?;
        let dimensions = infer_dimensions(board_part)?;
        let size = dimensions.size();

        let mut position = Position {
            dimensions: dimensions.clone(),
            fen: fen.to_string(),
            white_to_move,
            castling,
            en_passant,
            halfmove_clock: parts.get(4).and_then(|s| s.parse().ok()).unwrap_or(0),
            fullmove_number: parts.get(5).and_then(|s| s.parse().ok()).unwrap_or(1),
            pieces: HashMap::new(),
            walls: BitBoard::zero(size),
            occupied: BitBoard::zero(size),
            color_bitboards: HashMap::new(),
            piece_kinds: HashMap::new(),
        };

        position.color_bitboards.insert(Color::White, BitBoard::zero(size));
        position.color_bitboards.insert(Color::Black, BitBoard::zero(size));

        parse_board_into_bitboards(board_part, &mut position)?;

        Ok(position)
    }

    pub fn turn(&self) -> Color {
        if self.white_to_move { Color::White } else { Color::Black }
    }

    pub fn is_wall(&self, idx: usize) -> bool {
        self.walls.bit(idx).unwrap()
    }

    pub fn dimensions(&self) -> &Dimensions {
        &self.dimensions
    }

    pub fn is_friendly(&self, idx: usize, color: Color) -> bool {
        self.color_bitboards[&color].bit(idx).unwrap()
    }

     pub fn is_enemy(&self, idx: usize, color: Color) -> bool {
        self.color_bitboards[&color.opposite()].bit(idx).unwrap()
    }
}

fn parse_castling_rights(s: &str) -> u8 {
    let mut rights = 0;
    if s.contains('K') { rights |= KSCW; }
    if s.contains('Q') { rights |= QSCW; }
    if s.contains('k') { rights |= KSCB; }
    if s.contains('q') { rights |= QSCB; }
    rights
}

fn parse_en_passant(s: &str) -> Result<Option<usize>, String> {
    if s == "-" {
        return Ok(None);
    }
    if s.len() < 2 {
        return Err("Invalid en passant square".to_string());
    }

    let file = (s.chars().next().unwrap() as u8 - b'a') as usize;
    let rank = s[1..].parse::<usize>().map_err(|_| "Invalid rank")?;
    Ok(Some(rank.saturating_sub(1) * 16 + file)) // Assumes 16x16 max
}

fn infer_dimensions(board: &str) -> Result<Dimensions, String> {
    let mut files = None;
    let ranks: Vec<&str> = board.split('/').collect();

    for rank in &ranks {
        let mut count = 0;
        for c in rank.chars() {
            count += c.to_digit(10).unwrap_or(1);
        }
        match files {
            Some(f) if f != count => return Err("Inconsistent rank width".to_string()),
            None => files = Some(count),
            _ => (),
        }
    }

    Ok(Dimensions {
        ranks: ranks.len() as u8,
        files: files.unwrap_or(0) as u8,
    })
}


fn parse_board_into_bitboards(fen_board: &str, pos: &mut Position) -> Result<(), String> {
    let mut rank = 0;
    let mut file = 0;

    for ch in fen_board.chars() {
        if ch == '/' {
            rank += 1;
            file = 0;
            continue;
        }

        if let Some(skip) = ch.to_digit(10) {
            file += skip as u8;
            continue;
        }

        let idx = pos.dimensions.to_index(file, rank);
        let size = pos.dimensions.size();

        if ch == '.' {
            pos.walls.set_bit(idx, true);
        } else {
            pos.pieces
                .entry(ch)
                .or_insert_with(|| BitBoard::zero(size))
                .set_bit(idx, true);

            let color = if ch.is_uppercase() { Color::White } else { Color::Black };
            pos.color_bitboards
                .get_mut(&color)
                .unwrap()
                .set_bit(idx, true);

            pos.occupied.set_bit(idx, true);
        }

        file += 1;
    }

    Ok(())
}

