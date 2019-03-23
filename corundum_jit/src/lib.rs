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

pub fn run() {
    let mut module: Module<SimpleJITBackend> = Module::new(SimpleJITBuilder::new());

    let sig = Signature {
        params: vec![],
        returns: vec![],
        call_conv: CallConv::SystemV,
    };

    let func_id = module
        .declare_function("abc", Linkage::Local, &sig)
        .unwrap();

    let mut ctx = Context::new();
    ctx.func = Function::with_name_signature(ExternalName::user(0, func_id.as_u32()), sig);
    let mut func_ctx = FunctionBuilderContext::new();
    {
        let mut bcx: FunctionBuilder = FunctionBuilder::new(&mut ctx.func, &mut func_ctx);
        let ebb = bcx.create_ebb();
        bcx.switch_to_block(ebb);
        bcx.ins().return_(&[]);
    }

    module.define_function(func_id, &mut ctx).unwrap();
    module.finalize_definitions();
    let code = module.get_finalized_function(func_id);
    let function = unsafe { mem::transmute::<_, fn()>(code) };
    function();
}
