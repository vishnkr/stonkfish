use crate::engine::{move_generation::moves::{Move,MType}, position::zobrist::ZobristKey};

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
    pub key: ZobristKey,
    pub node_type: NodeType,
    pub value: usize,
    pub depth: u8,
    pub best_move: Move,
}


impl TableEntry{
    pub fn new()->Self{
        Self{
            key:0,
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
    buckets: Vec<Bucket>,
    size: usize,
}

impl TranspositionTable{
    pub fn new(size:usize)->Self{
        Self{
            buckets: vec![Bucket::new(); size],
            size
        }
    }

    pub fn insert(&mut self,key:ZobristKey,value:TableEntry){
        let bucket  = key as usize % self.size ;
        let mut min_depth_index = 0;
        for i in 1..ENTRIES_PER_BUCKET {
            if self.buckets[bucket].values[i].depth < value.depth{
                min_depth_index = i;
            }
        }
        self.buckets[bucket].values[min_depth_index] = value

    }

    pub fn get_entry(&mut self,key:ZobristKey)->Option<TableEntry>{
        let bucket = key as usize % self.size;
        for i in self.buckets[bucket].values {
            if i.key == key{
                return Some(i)
            }
        }
        None
    }
}

