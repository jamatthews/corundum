#[macro_use]
extern crate helix;
extern crate corundum_ruby;
extern crate corundum_jit;

use helix::sys::*;
use corundum_ruby::value::Value;
use corundum_ruby::fixnum::rb_int2inum;

ruby! {
    class Corundum {
        def preview_cranelift_ir(name: String, iseq: Vec<Vec<String>>, args: Vec<VALUE>) -> String {
            let mut jit = corundum_jit::jit::JIT::new();
            jit.preview(&name, &iseq, args)
        }

        def compile_and_run(name: String, iseq: Vec<Vec<String>>, args: Vec<VALUE>) -> i64 {
            let mut jit = corundum_jit::jit::JIT::new();
            jit.run(&name, &iseq, args)
        }

        def thingy(a: VALUE, b: VALUE) -> VALUE {
            unsafe{
                let x = rb_int2inum(2);
                std::mem::transmute::<Value, VALUE>(x)
            }
        }
    }
}
