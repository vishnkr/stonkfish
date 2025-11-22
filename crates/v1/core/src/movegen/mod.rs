

const ROOK_DIRS: &[(i8, i8)] = &[ (1, 0), (-1, 0), (0, 1), (0, -1) ];
const BISHOP_DIRS: &[(i8, i8)] = &[ (1, 1), (1, -1), (-1, 1), (-1, -1) ];
const QUEEN_DIRS: &[(i8, i8)] = &[ 
    (1, 0), (-1, 0), (0, 1), (0, -1), 
    (1, 1), (1, -1), (-1, 1), (-1, -1) 
];
const KNIGHT_OFFSETS: &[(i8, i8)] = &[ 
    (1, 2), (2, 1), (-1, 2), (-2, 1),
    (1, -2), (2, -1), (-1, -2), (-2, -1),
];

lazy_static! {
    static ref LAZY_ATTACK_TABLE: AttackTable = AttackTable::new();
}

fn slider_moves(pos: &Position, from: usize, color: Color, dirs: &[(i8, i8)]) -> Vec<Move> {
    let mut moves = vec![];
    let (fx, fy) = pos.dimensions().from_index(from);
    for &(dx, dy) in dirs {
        let mut x = fx as i8 + dx;
        let mut y = fy as i8 + dy;
        while pos.dimensions().in_bounds(x, y) {
            let to = pos.dimensions().to_index(x as u8, y as u8);
            if pos.is_wall(to) || pos.is_friendly(to, color) {
                break;
            }
            let mtype = if pos.is_enemy(to, color) {
                MType::Capture
            } else {
                MType::Quiet
            };
            
            moves.push(Move::encode_move(from as u8, to as u8, mtype, None));
            
            if pos.is_enemy(to, color) {
                break;
            }
            x += dx;
            y += dy;
        }
    }
    moves
}

fn knight_moves(pos: &Position, from: usize, color: Color) -> Vec<Move> {
    let mut moves = vec![];
    let (fx, fy) = pos.dimensions().from_index(from);
    for &(dx, dy) in KNIGHT_OFFSETS {
        let x = fx as i8 + dx;
        let y = fy as i8 + dy;
        if pos.dimensions().in_bounds(x, y) {
            let to = pos.dimensions().to_index(x as u8, y as u8);
            if !pos.is_wall(to) && !pos.is_friendly(to, color) {
                let mtype = if pos.is_enemy(to, color) {
                    MType::Capture
                } else {
                    MType::Quiet
                };
                
                moves.push(Move::encode_move(from as u8, to as u8, mtype, None));
                
            }
        }
    }
    moves
}
