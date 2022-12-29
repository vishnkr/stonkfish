use crate::engine::move_generation::moves::{Move,MType};

const ENTRIES_PER_BUCKET: usize = 4;

#[derive(Copy,Clone)]
pub enum NodeType{
    Exact,
    Alpha,
    Beta,
    None
}

#[derive(Copy,Clone)]
pub struct TableEntry{
    pub node_type: NodeType,
    pub value: usize,
    pub depth: i8,
    pub best_move: Move,
}


impl TableEntry{
    pub fn new()->Self{
        Self{
            node_type: NodeType::None,
            value: 0,
            depth: 0,
            best_move : Move::new(0,0,MType::None)
        }
    }
}

#[derive(Clone)]
pub struct Bucket{
    values: [TableEntry;ENTRIES_PER_BUCKET]
}

impl Bucket{
    pub fn new()->Self{
        Bucket{
            values: [TableEntry::new();ENTRIES_PER_BUCKET]
        }
    }
}
pub struct TranspositionTable{
    buckets: Vec<Bucket>
}

impl TranspositionTable{
    pub fn new(size:usize)->Self{
        Self{
            buckets: vec![Bucket::new(); size]
        }
    }
}

