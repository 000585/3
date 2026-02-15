use crate::block::Block;
use std::collections::HashSet;

// Делаем структуру публичной, чтобы lib.rs ее видел
pub struct BlockDAG {
    pub tips: HashSet<[u8; 32]>,
}

impl BlockDAG {
    pub fn new() -> Self {
        Self {
            tips: HashSet::new(),
        }
    }

    pub fn add_block(&mut self, block: &Block) {
        self.tips.insert(block.hash);
    }
}
