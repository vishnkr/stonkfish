use std::collections::HashMap;

use crate::{bitboard::*, types::*};

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
        let (ranks, files) = infer_dimensions(board_part)?;
        let largest_dimension = ranks.max(files);

        let mut position = Position {
            files,
            ranks,
            fen: fen.to_string(),
            largest_dimension,
            white_to_move,
            castling,
            en_passant,
            halfmove_clock: parts.get(4).and_then(|s| s.parse().ok()).unwrap_or(0),
            fullmove_number: parts.get(5).and_then(|s| s.parse().ok()).unwrap_or(1),
            pieces: HashMap::new(),
            walls: BitBoard::zero(),
            occupied: BitBoard::zero(),
            color_bitboards: HashMap::new(),
            custom_piece_rules: HashMap::new(),
        };

        position.color_bitboards.insert(Color::White, BitBoard::zero());
        position.color_bitboards.insert(Color::Black, BitBoard::zero());

        parse_board_into_bitboards(
            board_part,
            &mut position,
        )?;

        Ok(position)
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

fn infer_dimensions(board: &str) -> Result<(u8, u8), String> {
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
    Ok((ranks.len() as u8, files.unwrap_or(0) as u8))
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

        let idx = to_pos(rank, file) as usize;
        if ch == '.' {
            pos.walls.set_bit(idx, true);
        } else {
            pos.pieces.entry(ch).or_insert(BitBoard::zero()).set_bit(idx, true);
            let color = if ch.is_uppercase() { Color::White } else { Color::Black };
            pos.color_bitboards.get_mut(&color).unwrap().set_bit(idx, true);
            pos.occupied.set_bit(idx, true);
        }

        file += 1;
    }

    Ok(())
}

