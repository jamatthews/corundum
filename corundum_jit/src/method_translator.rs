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

use opcode_translator;
use opcode_translator::OpCode;
use translation_state::TranslationState;

pub struct MethodTranslator {
    builder_context: FunctionBuilderContext,
    state: TranslationState,
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
        opcode_translator::translate_code(OpCode::PutObject(0), &mut builder, &mut self.state);
        opcode_translator::translate_code(OpCode::SetLocal(0), &mut builder, &mut self.state);
        opcode_translator::translate_code(OpCode::Jump(0), &mut builder, &mut self.state);

        //body
        opcode_translator::translate_code(OpCode::Label(1), &mut builder, &mut self.state);
        opcode_translator::translate_code(OpCode::GetLocal(0), &mut builder, &mut self.state);
        opcode_translator::translate_code(OpCode::PutObject(1), &mut builder, &mut self.state);
        opcode_translator::translate_code(OpCode::OptPlus, &mut builder, &mut self.state);
        opcode_translator::translate_code(OpCode::SetLocal(0), &mut builder, &mut self.state);

        opcode_translator::translate_code(OpCode::Jump(0), &mut builder, &mut self.state); //ok, but how do I know I need to add this?

        //condition
        opcode_translator::translate_code(OpCode::Label(0), &mut builder, &mut self.state);
        opcode_translator::translate_code(OpCode::GetLocal(0), &mut builder, &mut self.state);
        opcode_translator::translate_code(OpCode::PutObject(300000000), &mut builder, &mut self.state);
        opcode_translator::translate_code(OpCode::OptLt, &mut builder, &mut self.state);
        opcode_translator::translate_code(OpCode::BranchIf(1), &mut builder, &mut self.state);
        builder.ins().return_(&[]);
        builder.seal_all_blocks();

        println!("{}", builder.display(None));

        Ok(())
    }
}
