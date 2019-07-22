use cranelift::prelude::*;
use cranelift_codegen::Context;
use cranelift_codegen::isa::CallConv;
use cranelift_codegen::ir::Function;
use cranelift_codegen::ir::types::I64;
use cranelift_module::*;
use cranelift_simplejit::*;

use opcode::OpCode;
use opcode_translator;
use translation_state::TranslationState;
use corundum_ruby::rb_iseq_t;

pub struct MethodTranslator<'a> {
    module: &'a mut Module<SimpleJITBackend>,
    builder: FunctionBuilder<'a>,
    state: TranslationState,
}

impl <'a> MethodTranslator<'a> {

    pub fn new(module: &'a mut Module<SimpleJITBackend>, builder: FunctionBuilder<'a>) -> Self {
        Self {
            module: module,
            builder: builder,
            state: TranslationState::new(),
        }
    }

    pub fn translate(&mut self, iseq: rb_iseq_t) -> Result<(), String> {
        setup_basic_blocks(&iseq, &mut self.builder, &mut self.state);
        let block = self.state.get_block(0).unwrap();
        self.builder.switch_to_block(block);
        self.builder.append_ebb_params_for_function_params(block);
        self.builder.ensure_inserted_ebb();

        //TODO handle mapping variables!
        self.builder.declare_var(Variable::with_u32(3), I64);

        let mut offset = 0;
        let max = unsafe { (*iseq.body).iseq_size };
        while offset < max {
            let insn_ptr = unsafe { (*iseq.body).iseq_encoded.offset(offset as isize) };
            let operands_ptr = unsafe { (*iseq.body).iseq_encoded.offset((offset+1) as isize) };
            let opcode: OpCode = (insn_ptr, operands_ptr).into();
            offset += opcode.size();

            let sig = Signature {
                params: vec![AbiParam::new(I64)],
                returns: vec![AbiParam::new(I64)],
                call_conv: CallConv::SystemV,
            };
            opcode_translator::translate_code(opcode, offset as i32, &mut self.builder, &mut self.state, &mut (*self.module));
            match self.state.get_block(offset as i32) {
                Some(block) => {
                    if !self.builder.is_filled() {
                        self.builder.ins().jump(block, &[]);
                    }
                    self.builder.switch_to_block(block)
                },
                _ => {}
            };
        }

        self.builder.seal_all_blocks();

        Ok(())
    }

    pub fn preview(&mut self, iseq: rb_iseq_t) -> Result<String, String> {
        Ok(self.builder.display(None).to_string())
    }
}

fn setup_basic_blocks(iseq: &rb_iseq_t, builder: &mut FunctionBuilder, state: &mut TranslationState){
    state.add_block(0, builder.create_ebb());

    let mut offset = 0;
    let max = unsafe { (*iseq.body).iseq_size };
    while offset < max {
        let insn_ptr = unsafe { (*iseq.body).iseq_encoded.offset(offset as isize) };
        let operands_ptr = unsafe { (*iseq.body).iseq_encoded.offset((offset+1) as isize) };
        let opcode: OpCode = (insn_ptr, operands_ptr).into();

        offset += opcode.size();
        match opcode {
            OpCode::BranchIf(target)|OpCode::BranchUnless(target) => {
                state.add_block(offset as i32 + target, builder.create_ebb());
            }
            OpCode::Jump(target) => {
                state.add_block(offset as i32, builder.create_ebb());
                state.add_block(offset as i32 + target, builder.create_ebb());
            },
            OpCode::Leave => {
                if offset < max {
                    state.add_block(offset as i32, builder.create_ebb());
                }
            }
            _ => {}
        }
    }
}
