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

        let opcodes = vec![
            // first block
            OpCode::PutObject(0),
            OpCode::SetLocal(0),
            OpCode::Jump(0),
            //body
            OpCode::Label(1),
            OpCode::GetLocal(0),
            OpCode::PutObject(1),
            OpCode::OptPlus,
            OpCode::SetLocal(0),
            OpCode::Jump(0),
            //condition
            OpCode::Label(0),
            OpCode::GetLocal(0),
            OpCode::PutObject(300000000),
            OpCode::OptLt,
            OpCode::BranchIf(1)
        ];

        for opcode in opcodes {
            opcode_translator::translate_code(opcode, &mut builder, &mut self.state);
        }

        builder.ins().return_(&[]);
        builder.seal_all_blocks();

        println!("{}", builder.display(None));

        Ok(())
    }
}
