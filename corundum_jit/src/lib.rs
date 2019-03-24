extern crate cranelift;
extern crate cranelift_codegen;
extern crate cranelift_entity;
extern crate cranelift_frontend;
extern crate cranelift_module;
extern crate cranelift_simplejit;

use cranelift_codegen::ir::*;
use cranelift_codegen::isa::CallConv;
use cranelift_codegen::Context;
use cranelift_entity::EntityRef;
use cranelift_frontend::*;
use cranelift_module::*;
use cranelift_simplejit::*;

use std::mem;

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
        let sig = Signature {
            params: vec![],
            returns: vec![],
            call_conv: CallConv::SystemV,
        };

        let func_id = self.module.declare_function(name, Linkage::Local, &sig).unwrap();

        self.codegen_context.func = Function::with_name_signature(ExternalName::user(0, func_id.as_u32()), sig);

        {
            let mut bcx: FunctionBuilder = FunctionBuilder::new(&mut self.codegen_context.func, &mut self.builder_context);
            let ebb = bcx.create_ebb();
            bcx.switch_to_block(ebb);
            bcx.ins().return_(&[]);
        }

        self.module.define_function(func_id, &mut self.codegen_context).unwrap();
        self.module.finalize_definitions();
        let code = self.module.get_finalized_function(func_id);
        let function = unsafe { mem::transmute::<_, fn()>(code) };
        function();
    }
}
