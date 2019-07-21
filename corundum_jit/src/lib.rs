extern crate cranelift;
extern crate cranelift_codegen;
extern crate cranelift_entity;
extern crate cranelift_frontend;
extern crate cranelift_module;
extern crate cranelift_simplejit;

extern crate helix;

extern crate corundum_ruby;

pub mod jit;

mod method_translator;
mod opcode;
mod opcode_translator;
mod translation_state;
