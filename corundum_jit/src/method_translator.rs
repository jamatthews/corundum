use cranelift::prelude::*;
use cranelift_codegen::ir::Function;
use cranelift_codegen::ir::types::I64;

use opcode_translator;
use opcode::OpCode;
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

    pub fn translate(&mut self, function: &mut Function, opcodes: Vec<OpCode>) -> Result<(), String> {
        let mut builder = FunctionBuilder::new(function, &mut self.builder_context);

        setup_basic_blocks(&opcodes, &mut builder, &mut self.state);
        builder.switch_to_block(self.state.get_block(0));
        builder.ensure_inserted_ebb();

        builder.declare_var(Variable::with_u32(3), I64);

        for opcode in opcodes {
            opcode_translator::translate_code(opcode, &mut builder, &mut self.state);
        }

        builder.seal_all_blocks();

        Ok(())
    }

    pub fn preview(&mut self, function: &mut Function, opcodes: Vec<OpCode>) -> Result<String, String> {
        let mut builder = FunctionBuilder::new(function, &mut self.builder_context);

        setup_basic_blocks(&opcodes, &mut builder, &mut self.state);
        builder.switch_to_block(self.state.get_block(0));

        builder.declare_var(Variable::with_u32(3), I64);

        for opcode in opcodes {
            opcode_translator::translate_code(opcode, &mut builder, &mut self.state);
        }

        builder.seal_all_blocks();

        Ok(builder.display(None).to_string())
    }
}

fn setup_basic_blocks(opcodes: &Vec<OpCode>, builder: &mut FunctionBuilder, state: &mut TranslationState){
    state.add_block(0, builder.create_ebb());

    for opcode in opcodes {
        match opcode {
            OpCode::Label(x) => state.add_block(*x, builder.create_ebb()),
            _ => ()
        }
    }
}
