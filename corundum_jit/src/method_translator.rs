use cranelift::prelude::*;
use cranelift_codegen::ir::Function;
use cranelift_codegen::ir::types::I64;

use opcode::OpCode;
use opcode_translator;
use translation_state::TranslationState;
use corundum_ruby::rb_iseq_t;

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

    pub fn translate(&mut self, function: &mut Function, iseq: rb_iseq_t) -> Result<(), String> {
        let mut builder = FunctionBuilder::new(function, &mut self.builder_context);

        setup_basic_blocks(&iseq, &mut builder, &mut self.state);
        let block = self.state.get_block(0);
        builder.switch_to_block(block);
        builder.append_ebb_params_for_function_params(block);
        let return_pointer = builder.ebb_params(block)[0];
        builder.ensure_inserted_ebb();

        builder.declare_var(Variable::with_u32(3), I64);

        let mut i = 0;
        let max = unsafe { (*iseq.body).iseq_size };
        while i < max {
            let insn_ptr = unsafe { (*iseq.body).iseq_encoded.offset(i as isize) };
            let operands_ptr = unsafe { (*iseq.body).iseq_encoded.offset((i+1) as isize) };
            let opcode: OpCode = (insn_ptr, operands_ptr).into();
            i += opcode.size();
            opcode_translator::translate_code(opcode, &mut builder, &mut self.state, &return_pointer);
        }

        builder.seal_all_blocks();
        //println!("{}", builder.display(None).to_string());

        Ok(())
    }

    pub fn preview(&mut self, function: &mut Function, iseq: rb_iseq_t) -> Result<String, String> {
        let mut builder = FunctionBuilder::new(function, &mut self.builder_context);

        setup_basic_blocks(&iseq, &mut builder, &mut self.state);
        let block = self.state.get_block(0);
        builder.switch_to_block(block);
        builder.append_ebb_params_for_function_params(block);
        let return_pointer = builder.ebb_params(block)[0];
        builder.ensure_inserted_ebb();

        builder.declare_var(Variable::with_u32(3), I64);

        let mut i = 0;
        let max = unsafe { (*iseq.body).iseq_size };
        while i < max {
            let insn_ptr = unsafe { (*iseq.body).iseq_encoded.offset(i as isize) };
            let operands_ptr = unsafe { (*iseq.body).iseq_encoded.offset((i+1) as isize) };
            let opcode: OpCode = (insn_ptr, operands_ptr).into();
            i += opcode.size();
            opcode_translator::translate_code(opcode, &mut builder, &mut self.state, &return_pointer);
        }

        builder.seal_all_blocks();

        Ok(builder.display(None).to_string())
    }
}

fn setup_basic_blocks(iseq: &rb_iseq_t, builder: &mut FunctionBuilder, state: &mut TranslationState){
    state.add_block(0, builder.create_ebb());

    let mut i = 0;
    let max = unsafe { (*iseq.body).iseq_size };
    while i < max {
        let insn_ptr = unsafe { (*iseq.body).iseq_encoded.offset(i as isize) };
        let operands_ptr = unsafe { (*iseq.body).iseq_encoded.offset((i+1) as isize) };
        let opcode: OpCode = (insn_ptr, operands_ptr).into();

        i += opcode.size();
        match opcode {
            OpCode::Jump(target)|OpCode::BranchIf(target) => {
                state.add_block(i as i32 + target, builder.create_ebb())
            },
            _ => {}
        }


    }
}
