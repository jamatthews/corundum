#[macro_use]
extern crate helix;
extern crate corundum_ruby;
extern crate corundum_jit;

use helix::sys::VALUE as HVALUE;
use corundum_ruby::value::Value as RValue;

ruby! {
    class Corundum {
        def preview_cranelift_ir(object: HVALUE, method: HVALUE) -> String {
            let object = unsafe { std::mem::transmute::<HVALUE, RValue>(object) };
            let method = unsafe { std::mem::transmute::<HVALUE, RValue>(method) };


            let mut jit = corundum_jit::jit::JIT::new();
            jit.preview(object, method)
        }

        def compile(object: HVALUE, method: HVALUE) {
            let object = unsafe { std::mem::transmute::<HVALUE, RValue>(object) };
            let method = unsafe { std::mem::transmute::<HVALUE, RValue>(method) };

            let mut jit = corundum_jit::jit::JIT::new();
            jit.compile(object, method).unwrap();
        }

        def compile_and_run(object: HVALUE, method: HVALUE) -> HVALUE {
            let object = unsafe { std::mem::transmute::<HVALUE, RValue>(object) };
            let method = unsafe { std::mem::transmute::<HVALUE, RValue>(method) };

            let mut jit = corundum_jit::jit::JIT::new();
            let x = jit.run(object, method);
            unsafe { std::mem::transmute::<RValue, HVALUE>(x) }
        }
    }
}
