#[macro_use]
extern crate helix;
extern crate corundum_ruby;
extern crate corundum_jit;

use helix::sys::VALUE as HVALUE;
use helix::sys::Qnil;
use corundum_ruby::value::Value as RValue;
use corundum_ruby::*;

ruby! {
    class Corundum {
        def preview_cranelift_ir(name: String, iseqw: HVALUE) -> String {
            let val = unsafe { std::mem::transmute::<HVALUE, corundum_ruby::VALUE>(iseqw) };
            let iseq = unsafe { *rb_iseqw_to_iseq(val) };
            let mut jit = corundum_jit::jit::JIT::new();
            jit.preview(&name, iseq)
        }

        def compile_and_run(name: String, iseqw: HVALUE ) -> HVALUE {
            let val = unsafe { std::mem::transmute::<HVALUE, corundum_ruby::VALUE>(iseqw) };
            let iseq = unsafe { *rb_iseqw_to_iseq(val) };

            let mut jit = corundum_jit::jit::JIT::new();
            let x = jit.run(&name, iseq);
            unsafe { std::mem::transmute::<RValue, HVALUE>(x) }
        }

        def preview_iseqw_to_iseq(iseqw: HVALUE) {
            let val = unsafe { std::mem::transmute::<HVALUE, corundum_ruby::VALUE>(iseqw) };
            let iseq = unsafe { rb_iseqw_to_iseq(val) };
            println!("{:?}", unsafe { *(*iseq).body });
        }
    }
}
