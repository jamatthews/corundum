#[macro_use]
extern crate helix;
extern crate corundum_ruby;
extern crate corundum_jit;

use helix::sys::*;
use corundum_ruby::value::Value;
use corundum_ruby::fixnum::rb_int2inum;
use corundum_ruby::vm::rb_iseqw_to_iseq;

ruby! {
    class Corundum {
        def preview_cranelift_ir(name: String, iseq: Vec<Vec<String>>, args: Vec<VALUE>) -> String {
            let mut jit = corundum_jit::jit::JIT::new();
            jit.preview(&name, &iseq, args)
        }

        def compile_and_run(name: String, iseq: Vec<Vec<String>>, args: Vec<VALUE>) -> VALUE {
            let mut jit = corundum_jit::jit::JIT::new();
            let x = jit.run(&name, &iseq, args);
            unsafe { std::mem::transmute::<Value, VALUE>(x) }
        }
    }
}
