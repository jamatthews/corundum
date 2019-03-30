extern crate cranelift;
extern crate cranelift_codegen;
extern crate cranelift_entity;
extern crate cranelift_frontend;
extern crate cranelift_module;
extern crate cranelift_simplejit;

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

        self.translate().unwrap();

        self.module.define_function(func_id, &mut self.codegen_context).unwrap();
        self.module.finalize_definitions();
        let code = self.module.get_finalized_function(func_id);
        Ok(code)
    }

    fn translate(&mut self) -> Result<(), String> {
        let mut builder = FunctionBuilder::new(&mut self.codegen_context.func, &mut self.builder_context);

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
