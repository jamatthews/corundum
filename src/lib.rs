#[macro_use]
extern crate helix;

use helix::sys::*;

mod ruby;

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
    }
}
