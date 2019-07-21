use cranelift::prelude::*;
use cranelift_codegen::ir::*;
use cranelift_codegen::ir::types::I64;
use cranelift_codegen::isa::CallConv;
use cranelift_codegen::Context;
use cranelift_module::*;
use cranelift_simplejit::*;

use std::mem;

use method_translator::MethodTranslator;

use corundum_ruby::value::Value as RValue;
use corundum_ruby::*;

pub struct JIT {
    module: Module<SimpleJITBackend>,
    codegen_context: Context,
}

impl JIT {
    pub fn new() -> Self {
        JIT {
            module: Module::new(SimpleJITBuilder::new()),
            codegen_context: Context::new(),
        }
    }

    pub fn run(&mut self, name: &str, iseq: rb_iseq_t) -> RValue {
        let function = self.compile(name, iseq).unwrap();
        let function = unsafe { mem::transmute::<_, fn() -> RValue >(function) };
        function()
    }

    pub fn compile(&mut self, name: &str, iseq: rb_iseq_t) -> Result<*const u8, String> {
        let sig = Signature {
            params: vec![AbiParam::new(I64)],
            returns: vec![AbiParam::new(I64)],
            call_conv: CallConv::SystemV,
        };

        let func_id = self.module.declare_function(name, Linkage::Local, &sig).unwrap();
        self.codegen_context.func = Function::with_name_signature(ExternalName::user(0, func_id.as_u32()), sig);
        {
            let mut builder_context = FunctionBuilderContext::new();
            let mut builder = FunctionBuilder::new(&mut self.codegen_context.func, &mut builder_context);

            MethodTranslator::new(builder).translate(iseq).unwrap();
        }

        self.module.define_function(func_id, &mut self.codegen_context).unwrap();
        self.module.finalize_definitions();
        let code = self.module.get_finalized_function(func_id);
        Ok(code)
    }

    pub fn preview(&mut self, name: &str, iseq: rb_iseq_t) -> String {
        let sig = Signature {
            params: vec![AbiParam::new(I64)],
            returns: vec![AbiParam::new(I64)],
            call_conv: CallConv::SystemV,
        };

        let func_id = self.module.declare_function(name, Linkage::Local, &sig).unwrap();
        self.codegen_context.func = Function::with_name_signature(ExternalName::user(0, func_id.as_u32()), sig);
        let mut builder_context = FunctionBuilderContext::new();
        let mut builder = FunctionBuilder::new(&mut self.codegen_context.func, &mut builder_context);

        MethodTranslator::new(builder).preview(iseq).unwrap()
    }
}
