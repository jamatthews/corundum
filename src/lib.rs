#[macro_use]
extern crate helix;

ruby! {
    class Corundum {
        def preview_cranelift_ir(name: String, iseq: Vec<String>) {
            let mut jit = corundum_jit::JIT::new();
            jit.run(&name, &iseq);
            println!("Ran a JITed function!");
        }
    }
}
