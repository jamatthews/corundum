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

        let counter_constant = builder.ins().iconst(I64, i64::from(0));
        builder.declare_var(Variable::with_u32(0), I64);
        builder.def_var(Variable::with_u32(0), counter_constant);

        let limit_constant = builder.ins().iconst(I64, i64::from(3000000));
        builder.declare_var(Variable::with_u32(1), I64);
        builder.def_var(Variable::with_u32(1), limit_constant);

        let header_block = builder.create_ebb();
        let exit_block = builder.create_ebb();

        builder.ins().jump(header_block, &[]);
        builder.seal_block(block);
        builder.switch_to_block(header_block);

        let lhs = builder.use_var(Variable::with_u32(0));
        let rhs = builder.use_var(Variable::with_u32(1));
        let c = builder.ins().icmp(IntCC::SignedLessThan, lhs, rhs);
        let condition = builder.ins().bint(I64, c);

        builder.ins().brz(condition, exit_block, &[]);

        let lhs = builder.use_var(Variable::with_u32(0));
        let rhs = builder.ins().iconst(I64, i64::from(1));
        let result = builder.ins().iadd(lhs, rhs);
        builder.def_var(Variable::with_u32(0), result);

        builder.ins().jump(header_block, &[]);

        builder.switch_to_block(exit_block); //required?
        builder.seal_block(header_block);
        builder.ins().return_(&[]);
        builder.seal_block(exit_block);
        Ok(())
    }
}
