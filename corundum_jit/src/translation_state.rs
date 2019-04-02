use cranelift::prelude::*;
use cranelift_codegen::ir::*;
use cranelift_codegen::isa::CallConv;
use cranelift_codegen::Context;
use cranelift_codegen::ir::types::I64;
use cranelift_codegen::ir::Function;
use cranelift_entity::EntityRef;
use cranelift_frontend::*;
use cranelift_module::*;
use cranelift_simplejit::*;

pub struct TranslationState {
    pub stack: Vec<Value>,
    pub blocks: Vec<Ebb>,
}

impl TranslationState {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            blocks: Vec::new(),
        }
    }

    pub fn push(&mut self, val: Value) {
        self.stack.push(val);
    }

    pub fn pop(&mut self) -> Value {
        self.stack.pop().unwrap()
    }

    pub fn push_block(&mut self, block: Ebb) {
        self.blocks.push(block);
    }

    pub fn get_block(&mut self, index: usize) -> Ebb {
        *self.blocks.get(index).unwrap()
    }
}
