use cranelift::prelude::*;
use cranelift_codegen::ir::types::I64;
use cranelift_codegen::ir::Function;

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

        builder.declare_var(Variable::with_u32(0), I64);

        for opcode in opcodes {
            opcode_translator::translate_code(opcode, &mut builder, &mut self.state);
        }

        builder.ins().return_(&[]);
        builder.seal_all_blocks();

        Ok(())
    }

    pub fn preview(&mut self, function: &mut Function, opcodes: Vec<OpCode>) -> Result<String, String> {
        let mut builder = FunctionBuilder::new(function, &mut self.builder_context);

        setup_basic_blocks(&opcodes, &mut builder, &mut self.state);
        builder.switch_to_block(self.state.get_block(0));

        builder.declare_var(Variable::with_u32(0), I64);

        for opcode in opcodes {
            opcode_translator::translate_code(opcode, &mut builder, &mut self.state);
        }

        builder.ins().return_(&[]);
        builder.seal_all_blocks();

        Ok(builder.display(None).to_string())
    }
}

fn setup_basic_blocks(opcodes: &Vec<OpCode>, builder: &mut FunctionBuilder, state: &mut TranslationState){
    state.push_block(builder.create_ebb());

    for opcode in opcodes {
        match opcode {
            OpCode::BranchIf(_) => state.push_block(builder.create_ebb()),
            OpCode::Jump(_) => state.push_block(builder.create_ebb()),
            _ => ()
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_compiles_while_loops() {
        let sig = Signature {
            params: vec![],
            returns: vec![],
            call_conv: CallConv::SystemV,
        };

        let mut module: Module<SimpleJITBackend> = Module::new(SimpleJITBuilder::new());
        let func_id = module.declare_function("test".into(), Linkage::Local, &sig).unwrap();
        let mut func = Function::with_name_signature(ExternalName::user(0, func_id.as_u32()), sig);

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

        MethodTranslator::new().translate(&mut func, opcodes).unwrap();
    }

    #[test]
    fn it_previews() {
        let sig = Signature {
            params: vec![],
            returns: vec![],
            call_conv: CallConv::SystemV,
        };

        let mut module: Module<SimpleJITBackend> = Module::new(SimpleJITBuilder::new());
        let func_id = module.declare_function("test1".into(), Linkage::Local, &sig).unwrap();
        let mut func = Function::with_name_signature(ExternalName::user(0, func_id.as_u32()), sig);

        let preview: String = MethodTranslator::new().preview(&mut func, vec![]).unwrap();
    }
}
