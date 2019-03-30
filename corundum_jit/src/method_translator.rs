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

pub struct MethodTranslator {
    builder_context: FunctionBuilderContext,
    state: TranslationState,
}

pub struct TranslationState {
    pub stack: Vec<Value>,
}

impl TranslationState {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
        }
    }

    pub fn push(&mut self, val: Value) {
        self.stack.push(val);
    }

    pub fn pop(&mut self) -> Value {
        self.stack.pop().unwrap()
    }
}

impl MethodTranslator {

    pub fn new() -> Self {
        Self {
            builder_context: FunctionBuilderContext::new(),
            state: TranslationState::new(),
        }
    }

    pub fn translate(&mut self, function: &mut Function) -> Result<(), String> {
        let mut builder = FunctionBuilder::new(function, &mut self.builder_context);

        let block = builder.create_ebb();
        builder.switch_to_block(block);

        builder.declare_var(Variable::with_u32(0), I64);

        translate_code(OpCode::PutObject(0), &mut builder, &mut self.state);
        translate_code(OpCode::SetLocal(0), &mut builder, &mut self.state);

        let header_block = builder.create_ebb();
        let exit_block = builder.create_ebb();

        builder.ins().jump(header_block, &[]);
        builder.seal_block(block);
        builder.switch_to_block(header_block);

        translate_code(OpCode::GetLocal(0), &mut builder, &mut self.state);
        translate_code(OpCode::PutObject(3000000), &mut builder, &mut self.state);
        translate_code(OpCode::OptLt, &mut builder, &mut self.state);

        builder.ins().brz(self.state.pop(), exit_block, &[]);

        translate_code(OpCode::GetLocal(0), &mut builder, &mut self.state);
        translate_code(OpCode::PutObject(1), &mut builder, &mut self.state);
        translate_code(OpCode::OptPlus, &mut builder, &mut self.state);

        builder.ins().jump(header_block, &[]);

        builder.switch_to_block(exit_block); //required?
        builder.seal_block(header_block);
        builder.ins().return_(&[]);
        builder.seal_block(exit_block);
        Ok(())
    }
}

fn translate_code(op: OpCode, builder: &mut FunctionBuilder, state: &mut TranslationState) {
    match op {
        OpCode::PutObject(obj) => {
            let value = builder.ins().iconst(I64, i64::from(obj));
            state.push(value);
        },
        OpCode::SetLocal(index) => {
            let value = state.pop();
            builder.def_var(Variable::with_u32(index), value);
        },
        OpCode::GetLocal(index) => {
            state.push(builder.use_var(Variable::with_u32(index)))
        },
        OpCode::OptPlus => {
            let lhs = state.pop();
            let rhs = state.pop();
            let value = builder.ins().iadd(lhs, rhs);
            builder.def_var(Variable::with_u32(0), value);
            state.push(value);
        },
        OpCode::OptLt => {
            let lhs = state.pop();
            let rhs = state.pop();
            let c = builder.ins().icmp(IntCC::SignedLessThan, lhs, rhs);
            let value = builder.ins().bint(I64, c);
            state.push(value);
        }
    }
}

enum OpCode {
    PutObject(i64),
    SetLocal(u32),
    GetLocal(u32),
    OptPlus,
    OptLt,
}
