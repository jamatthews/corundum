#[macro_use]
extern crate helix;

ruby! {
    class Corundum {
        def preview_cranelift_ir(iseq: Vec<String>) {
            let mut jit = corundum_jit::JIT::new();
            jit.run();
            println!("Ran a JITed function!");
        }
    }
}
