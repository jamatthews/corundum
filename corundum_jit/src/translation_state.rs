use cranelift::prelude::*;
use std::collections::HashMap;

pub struct TranslationState {
    pub stack: Vec<Value>,
    pub blocks: HashMap<usize,Ebb>,
}

impl TranslationState {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            blocks: HashMap::new(),
        }
    }

    pub fn push(&mut self, val: Value) {
        self.stack.push(val);
    }

    pub fn pop(&mut self) -> Value {
        self.stack.pop().unwrap()
    }

    pub fn add_block(&mut self, label: usize, block: Ebb) {
        self.blocks.insert(label, block);
    }

    pub fn get_block(&mut self, label: usize) -> Ebb {
        match self.blocks.get(&label) {
            Some(block) => *block,
            None => panic!("Couldn't get block {}", label)
        }
    }
}