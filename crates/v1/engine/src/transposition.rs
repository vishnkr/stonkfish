use crate::{move_generation::moves::{Move,MType}, position::zobrist::ZobristKey};

const ENTRIES_PER_BUCKET: usize = 4;

/// Represents type of a search node in the transposition table.
#[derive(Copy,Clone,PartialEq)]
pub enum NodeType{
    /// A search node with an exact score.
    Exact,
    /// A search node with a lower-bound score (alpha).
    Alpha,
    /// A search node with an upper-bound score (beta).
    Beta,
    None
}

#[derive(Copy,Clone)]
pub struct TableEntry{
    pub key: ZobristKey,
    pub node_type: NodeType,
    pub score: isize,
    pub depth: u8,
    pub best_move: Move,
}


impl TableEntry{
    pub fn new()->Self{
        Self{
            key:0,
            node_type: NodeType::None,
            score: 0,
            depth: 0,
            best_move : Move::encode_move(0,0,MType::None,None)
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

    /// Retrieve bucket with matching zobrist key
    pub fn get(&self,key:ZobristKey)->Option<&TableEntry>{
        for entry in &self.values{
            if entry.key == key{
                return Some(entry);
            }
        }
        None
    }

    /// Go through the entries in the bucket and find an empty slot or a slot with a matching key
    pub fn put(&mut self,entry:TableEntry){
        let mut idx = 0;
        while idx < ENTRIES_PER_BUCKET{
            if self.values[idx].node_type == NodeType::None ||self.values[idx].key == entry.key || entry.depth < self.values[idx].depth  {
                break;
            }
            idx+=1;
        }
        self.values[idx] = entry;
    }

}

pub struct TranspositionTable{
    buckets: Vec<Bucket>,
}

impl TranspositionTable{
    pub fn new(size:usize)->Self{
        let num_buckets = (size)/ENTRIES_PER_BUCKET;
        Self{
            buckets: vec![Bucket::new(); num_buckets],
        }
    }

    pub fn insert(&mut self,entry:TableEntry){
        let bucket_idx = (entry.key as usize) % self.buckets.len();
        self.buckets[bucket_idx].put(entry);
    }

    pub fn lookup(&mut self,key:ZobristKey)->Option<&TableEntry>{
        let bucket_idx = key as usize % self.buckets.len();
        self.buckets[bucket_idx].get(key)
    }

}

