use cranelift::prelude::*;
use std::collections::HashMap;

use corundum_ruby::value::Value as RValue;

pub struct TranslationState {
    pub stack: Vec<Value>,
    pub static_stack: Vec<u64>,
    pub blocks: HashMap<i32,Ebb>,
    pub between_blocks: bool,
}

impl TranslationState {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            static_stack: Vec::new(),
            blocks: HashMap::new(),
            between_blocks: false,
        }
    }

    pub fn push(&mut self, val: Value) {
        self.stack.push(val);
    }

    pub fn pop(&mut self) -> Value {
        self.stack.pop().expect("Stack underflow")
    }

    pub fn push_static(&mut self, val: u64) {
        self.static_stack.push(val);
    }

    pub fn pop_static(&mut self) -> u64 {
        self.static_stack.pop().expect("Stack(static) underflow")
    }

    pub fn add_block(&mut self, label: i32, block: Ebb) {
        self.blocks.insert(label, block);
    }

    pub fn get_block(&mut self, label: i32) -> Option<Ebb> {
        match self.blocks.get(&label) {
            Some(block) => { Some(*block) },
            None => None
        }
    }
}
