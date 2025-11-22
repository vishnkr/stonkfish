use crate::{
    board::{Dimensions, Square},
    piece::{PieceKind, Color, Piece},
    position::Position,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FenError {
    InvalidFormat,
    RowCountMismatch { expected: u8, got: u8 },
    ColCountMismatch { expected: u8, got: u8 },
    UnknownPieceSymbol(char),
    InvalidActiveColor,
    InvalidEnPassant,
    InvalidCastling,
    InvalidNumber(String),
}

/// Standard chess piece symbols
fn piece_from_symbol(ch: char) -> Option<(Color, PieceKind)> {
    match ch {
        'P' => Some((Color::White, PieceKind::Pawn)),
        'N' => Some((Color::White, PieceKind::Knight)),
        'B' => Some((Color::White, PieceKind::Bishop)),
        'R' => Some((Color::White, PieceKind::Rook)),
        'Q' => Some((Color::White, PieceKind::Queen)),
        'K' => Some((Color::White, PieceKind::King)),
        'p' => Some((Color::Black, PieceKind::Pawn)),
        'n' => Some((Color::Black, PieceKind::Knight)),
        'b' => Some((Color::Black, PieceKind::Bishop)),
        'r' => Some((Color::Black, PieceKind::Rook)),
        'q' => Some((Color::Black, PieceKind::Queen)),
        'k' => Some((Color::Black, PieceKind::King)),
        _ => None,
    }
}

pub struct Fen;

impl Fen {
    /// Parse a FEN string for standard chess
    /// Format: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
    pub fn parse(input: &str, dims: Dimensions) -> Result<Position, FenError> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() < 4 {
            return Err(FenError::InvalidFormat);
        }

        let board_part = parts[0];
        let active_color_part = parts.get(1).copied().unwrap_or("w");
        let castling_part = parts.get(2).copied().unwrap_or("-");
        let ep_part = parts.get(3).copied().unwrap_or("-");
        let halfmove_part = parts.get(4).unwrap_or(&"0");
        let fullmove_part = parts.get(5).unwrap_or(&"1");

        let rows: Vec<&str> = board_part.split('/').collect();
        if rows.len() as u8 != dims.height {
            return Err(FenError::RowCountMismatch {
                expected: dims.height,
                got: rows.len() as u8,
            });
        }

        let mut pos = Position::new_empty(dims);

        for (rank_idx, row) in rows.iter().enumerate() {
            let rank = (dims.height - 1 - rank_idx as u8) as u8;
            let mut file: u8 = 0;
            let mut digit_buffer = String::new();

            for ch in row.chars() {
                if ch.is_ascii_digit() {
                    digit_buffer.push(ch);
                    continue;
                }

                if !digit_buffer.is_empty() {
                    let count: u8 = digit_buffer.parse()
                        .map_err(|_| FenError::InvalidNumber(digit_buffer.clone()))?;
                    file += count;
                    digit_buffer.clear();
                }

                if file >= dims.width {
                    return Err(FenError::ColCountMismatch {
                        expected: dims.width,
                        got: file + 1,
                    });
                }

                match piece_from_symbol(ch) {
                    Some((color, kind)) => {
                        let sq = Square::from_rank_file(rank, file, &dims);
                        pos.set_piece(sq, Piece { color, kind });
                    }
                    None => {
                        let color = if ch.is_uppercase() {
                            Color::White
                        } else {
                            Color::Black
                        };
                        let custom_id = (ch as u8) % 128;
                        let kind = PieceKind::Custom(custom_id);
                        let sq = Square::from_rank_file(rank, file, &dims);
                        pos.set_piece(sq, Piece { color, kind });
                    }
                }

                file += 1;
            }
                        if !digit_buffer.is_empty() {
                let count: u8 = digit_buffer.parse()
                    .map_err(|_| FenError::InvalidNumber(digit_buffer.clone()))?;
                file += count;
            }

            if file != dims.width {
                return Err(FenError::ColCountMismatch {
                    expected: dims.width,
                    got: file,
                });
            }
        }

        // Parse active color
        match active_color_part {
            "w" | "W" => pos.side_to_move = Color::White,
            "b" | "B" => pos.side_to_move = Color::Black,
            _ => return Err(FenError::InvalidActiveColor),
        }

        // Parse castling rights
        if castling_part != "-" {
            for ch in castling_part.chars() {
                match ch {
                    'K' => pos.castling_rights.set_white_kingside(true),
                    'Q' => pos.castling_rights.set_white_queenside(true),
                    'k' => pos.castling_rights.set_black_kingside(true),
                    'q' => pos.castling_rights.set_black_queenside(true),
                    _ => {}
                }
            }
        }

        if ep_part != "-" {
            if ep_part.len() == 2 {
                let mut chars = ep_part.chars();
                let file_char = chars.next().unwrap();
                let rank_char = chars.next().unwrap();
                
                if file_char.is_ascii_lowercase() && rank_char.is_ascii_digit() {
                    let file = (file_char as u8) - b'a';
                    let rank = (rank_char as u8) - b'1';
                    
                    if file < dims.width && rank < dims.height {
                        pos.ep_square = Some(Square::from_rank_file(rank, file, &dims));
                    }
                }
            }
        }

        pos.halfmove_clock = halfmove_part
            .parse()
            .map_err(|_| FenError::InvalidNumber(halfmove_part.to_string()))?;

        pos.fullmove_number = fullmove_part
            .parse()
            .map_err(|_| FenError::InvalidNumber(fullmove_part.to_string()))?;

        Ok(pos)
    }
    
    pub fn to_string(pos: &Position) -> String {
        let mut fen = String::new();
        
        for rank in (0..pos.dims.height).rev() {
            if rank < pos.dims.height - 1 {
                fen.push('/');
            }
            
            let mut empty_count = 0;
            for file in 0..pos.dims.width {
                let sq = Square::from_rank_file(rank, file, &pos.dims);
                
                match pos.piece_at(sq) {
                    Some(piece) => {
                        if empty_count > 0 {
                            fen.push_str(&empty_count.to_string());
                            empty_count = 0;
                        }
                        fen.push(piece_to_symbol(piece));
                    }
                    None => {
                        empty_count += 1;
                    }
                }
            }
            
            if empty_count > 0 {
                fen.push_str(&empty_count.to_string());
            }
        }
        
        fen.push(' ');
        fen.push(match pos.side_to_move {
            Color::White => 'w',
            Color::Black => 'b',
        });
        
        fen.push(' ');
        let mut castling = String::new();
        if pos.castling_rights.has_white_kingside() {
            castling.push('K');
        }
        if pos.castling_rights.has_white_queenside() {
            castling.push('Q');
        }
        if pos.castling_rights.has_black_kingside() {
            castling.push('k');
        }
        if pos.castling_rights.has_black_queenside() {
            castling.push('q');
        }
        if castling.is_empty() {
            fen.push('-');
        } else {
            fen.push_str(&castling);
        }
        
        fen.push(' ');
        match pos.ep_square {
            Some(sq) => {
                let (file, rank) = sq.file_rank(&pos.dims);
                fen.push((b'a' + file) as char);
                fen.push((b'1' + rank) as char);
            }
            None => fen.push('-'),
        }
        
        fen.push_str(&format!(" {} {}", pos.halfmove_clock, pos.fullmove_number));
        
        fen
    }
}

fn piece_to_symbol(piece: Piece) -> char {
    match (piece.color, piece.kind) {
        (Color::White, PieceKind::Pawn) => 'P',
        (Color::White, PieceKind::Knight) => 'N',
        (Color::White, PieceKind::Bishop) => 'B',
        (Color::White, PieceKind::Rook) => 'R',
        (Color::White, PieceKind::Queen) => 'Q',
        (Color::White, PieceKind::King) => 'K',
        (Color::Black, PieceKind::Pawn) => 'p',
        (Color::Black, PieceKind::Knight) => 'n',
        (Color::Black, PieceKind::Bishop) => 'b',
        (Color::Black, PieceKind::Rook) => 'r',
        (Color::Black, PieceKind::Queen) => 'q',
        (Color::Black, PieceKind::King) => 'k',
        (Color::White, PieceKind::Custom(id)) => {
            if id < 26 {
                (b'A' + id) as char
            } else {
                '?'
            }
        }
        (Color::Black, PieceKind::Custom(id)) => {
            if id < 26 {
                (b'a' + id) as char
            } else {
                '?'
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::{Dimensions, BB};
    use crate::piece::{Color, PieceKind};

    #[test]
    fn fen_parse_starting_position() {
        let dims = Dimensions::standard();
        let fen_str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        
        let pos = Fen::parse(fen_str, dims).unwrap();
        
        assert!(pos.piece_bb(Color::White, PieceKind::King).count() == 1);
        assert!(pos.piece_bb(Color::Black, PieceKind::Pawn).count() == 8);
        assert!(pos.all.count() == 32);
        assert!(pos.occ[0].count() == 16);
        assert!(pos.occ[1].count() == 16);
        assert!(pos.castling_rights.has_white_kingside());
        assert!(pos.castling_rights.has_white_queenside());
        assert!(pos.castling_rights.has_black_kingside());
        assert!(pos.castling_rights.has_black_queenside());
        assert!(pos.ep_square.is_none());
        assert_eq!(pos.halfmove_clock, 0);
        assert_eq!(pos.fullmove_number, 1);
        assert_eq!(pos.side_to_move, Color::White);
    }
    
    #[test]
    fn fen_parse_and_to_string() {
        let dims = Dimensions::standard();
        let fen_str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        
        let pos = Fen::parse(fen_str, dims).unwrap();
        let fen_output = Fen::to_string(&pos);
        
        assert!(fen_output.contains("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"));
        assert!(fen_output.contains("w"));
    }
    
    #[test]
    fn fen_parse_empty_position() {
        let dims = Dimensions::standard();
        let fen_str = "8/8/8/8/8/8/8/8 w - - 0 1";
        
        let pos = Fen::parse(fen_str, dims).unwrap();
        
        assert_eq!(pos.all.count(), 0);
        assert_eq!(pos.side_to_move, Color::White);
    }
    
    #[test]
    fn fen_parse_black_to_move() {
        let dims = Dimensions::standard();
        let fen_str = "8/8/8/8/8/8/8/8 b - - 0 1";
        
        let pos = Fen::parse(fen_str, dims).unwrap();
        
        assert_eq!(pos.side_to_move, Color::Black);
    }
    
    #[test]
    fn fen_parse_en_passant() {
        let dims = Dimensions::standard();
        let fen_str = "4k3/6p1/8/pP1pP3/7P/8/8/4K3 w - d6 0 6";
        let pos = Fen::parse(fen_str, dims).unwrap();
        assert!(pos.ep_square.is_some());
        let ep_sq = pos.ep_square.unwrap();
        let (file, rank) = ep_sq.file_rank(&dims);
        assert_eq!(file, 4);
        assert_eq!(rank, 2);
    }
    
    #[test]
    fn fen_parse_castling_rights() {
        let dims = Dimensions::standard();
        let fen_str = "r3kbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQK2R w Kq - 0 1";
        
        let pos = Fen::parse(fen_str, dims).unwrap();
        
        assert!(pos.castling_rights.has_white_kingside());
        assert!(!pos.castling_rights.has_white_queenside());
        assert!(!pos.castling_rights.has_black_kingside());
        assert!(pos.castling_rights.has_black_queenside());
    }
    
    #[test]
    fn fen_parse_multi_digit_empty() {
        let dims = Dimensions::new(12, 8);
        let fen_str = "12/12/12/12/12/12/12/12 w - - 0 1";
        
        let pos = Fen::parse(fen_str, dims).unwrap();
        assert_eq!(pos.all.count(), 0);
    }
    
    #[test]
    fn fen_parse_custom_pieces() {
        let dims = Dimensions::standard();
        let fen_str = "8/8/8/8/3Xx3/8/8/8 w - - 0 1";        
        let pos = Fen::parse(fen_str, dims).unwrap();
        assert_eq!(pos.all.count(), 2);
    }
    
    #[test]
    fn fen_parse_12x10_board() {
        let dims = Dimensions::new(12, 10);
        let fen_str = "12/12/12/12/12/12/12/12/12/12 w - - 0 1";
        
        let pos = Fen::parse(fen_str, dims).unwrap();
        assert_eq!(pos.all.count(), 0);
        assert_eq!(pos.dims.width, 12);
        assert_eq!(pos.dims.height, 10);
    }
    
    #[test]
    fn fen_parse_7x9_board() {
        let dims = Dimensions::new(7, 9);
        let fen_str = "7/7/7/7/7/7/7/7/7 w - - 0 1";
        
        let pos = Fen::parse(fen_str, dims).unwrap();
        assert_eq!(pos.dims.width, 7);
        assert_eq!(pos.dims.height, 9);
    }
    
    #[test]
    fn fen_parse_5x5_board() {
        let dims = Dimensions::new(5, 5);
        let fen_str = "5/5/5/5/5 w - - 0 1";
        
        let pos = Fen::parse(fen_str, dims).unwrap();
        assert_eq!(pos.dims.width, 5);
        assert_eq!(pos.dims.height, 5);
    }
    
    #[test]
    fn fen_parse_10x10_board() {
        let dims = Dimensions::new(10, 10);
        let fen_str = "10/10/10/10/10/10/10/10/10/10 w - - 0 1";
        
        let pos = Fen::parse(fen_str, dims).unwrap();
        assert_eq!(pos.dims.width, 10);
        assert_eq!(pos.dims.height, 10);
    }
    
    #[test]
    fn fen_parse_complex_position() {
        let dims = Dimensions::standard();
        let fen_str = "r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQkq - 0 1";
        
        let pos = Fen::parse(fen_str, dims).unwrap();
        assert!(pos.piece_bb(Color::White, PieceKind::Rook).count() == 2);
        assert!(pos.piece_bb(Color::Black, PieceKind::Rook).count() == 2);
        assert!(pos.castling_rights.has_white_kingside());
        assert!(pos.castling_rights.has_white_queenside());
    }
    
    #[test]
    fn fen_parse_mixed_digits() {
        let dims = Dimensions::standard();
        let fen_str = "2r3k1/8/8/8/8/8/8/8 w - - 0 1";

        let pos = Fen::parse(fen_str, dims).unwrap();
        assert_eq!(pos.all.count(), 2);
    }
}
