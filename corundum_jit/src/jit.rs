use cranelift::prelude::*;
use cranelift_codegen::ir::*;
use cranelift_codegen::isa::CallConv;
use cranelift_codegen::Context;
use cranelift_codegen::ir::types::I64;
use cranelift_entity::EntityRef;
use cranelift_frontend::*;
use cranelift_module::*;
use cranelift_simplejit::*;

use std::mem;

use method_translator::MethodTranslator;

pub struct JIT {
    module: Module<SimpleJITBackend>,
    codegen_context: Context,
    builder_context: FunctionBuilderContext,
}

impl JIT {
    pub fn new() -> Self {
        JIT {
            module: Module::new(SimpleJITBuilder::new()),
            codegen_context: Context::new(),
            builder_context: FunctionBuilderContext::new(),
        }
    }

    pub fn run(&mut self, name: &str, iseq: &Vec<String>) {

        let function = self.compile(name, iseq).unwrap();
        let function = unsafe { mem::transmute::<_, fn()>(function) };
        function();
    }

    fn compile(&mut self, name: &str, iseq: &Vec<String>) -> Result<*const u8, String> {
        let sig = Signature {
            params: vec![],
            returns: vec![],
            call_conv: CallConv::SystemV,
        };

        let func_id = self.module.declare_function(name, Linkage::Local, &sig).unwrap();
        self.codegen_context.func = Function::with_name_signature(ExternalName::user(0, func_id.as_u32()), sig);

        MethodTranslator::new().translate(&mut self.codegen_context.func).unwrap();

        self.module.define_function(func_id, &mut self.codegen_context).unwrap();
        self.module.finalize_definitions();
        let code = self.module.get_finalized_function(func_id);
        Ok(code)
    }
}
