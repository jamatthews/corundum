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

impl MethodTranslator {

    pub fn new() -> Self {
        Self {
            builder_context: FunctionBuilderContext::new(),
            state: TranslationState::new(),
        }
    }

    pub fn translate(&mut self, function: &mut Function) -> Result<(), String> {
        let mut builder = FunctionBuilder::new(function, &mut self.builder_context);

        self.state.push_block(builder.create_ebb());
        self.state.push_block(builder.create_ebb());
        self.state.push_block(builder.create_ebb());
        builder.switch_to_block(self.state.get_block(0));

        builder.declare_var(Variable::with_u32(0), I64);

        // first block
        translate_code(OpCode::PutObject(0), &mut builder, &mut self.state);
        translate_code(OpCode::SetLocal(0), &mut builder, &mut self.state);
        translate_code(OpCode::Jump(0), &mut builder, &mut self.state);

        //body
        translate_code(OpCode::Label(1), &mut builder, &mut self.state);
        translate_code(OpCode::GetLocal(0), &mut builder, &mut self.state);
        translate_code(OpCode::PutObject(1), &mut builder, &mut self.state);
        translate_code(OpCode::OptPlus, &mut builder, &mut self.state);
        translate_code(OpCode::SetLocal(0), &mut builder, &mut self.state);

        translate_code(OpCode::Jump(0), &mut builder, &mut self.state); //ok, but how do I know I need to add this?

        //condition
        translate_code(OpCode::Label(0), &mut builder, &mut self.state);
        translate_code(OpCode::GetLocal(0), &mut builder, &mut self.state);
        translate_code(OpCode::PutObject(300000000), &mut builder, &mut self.state);
        translate_code(OpCode::OptLt, &mut builder, &mut self.state);
        translate_code(OpCode::BranchIf(1), &mut builder, &mut self.state);
        builder.ins().return_(&[]);
        builder.seal_all_blocks();

        println!("{}", builder.display(None));

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
        },
        OpCode::Jump(label) => {
            builder.ins().jump(state.get_block(label+1), &[]);
        },
        OpCode::Label(label) => {
            builder.switch_to_block(state.get_block(label+1));
        },
        OpCode::BranchIf(label) => {
            builder.ins().brz(state.pop(), state.get_block(label+1), &[]);
        }
    }
}

enum OpCode {
    PutObject(i64),
    SetLocal(u32),
    GetLocal(u32),
    OptPlus,
    OptLt,
    Jump(usize),
    Label(usize),
    BranchIf(usize),
}
