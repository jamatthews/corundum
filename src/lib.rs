#[macro_use]
extern crate helix;

ruby! {
    class Corundum {
        def preview_cranelift_ir(name: String, iseq: Vec<String>) {
            let mut jit = corundum_jit::jit::JIT::new();
            jit.preview(&name, &iseq);
        }
    }
}
