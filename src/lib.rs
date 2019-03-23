#[macro_use]
extern crate helix;

ruby! {
    class Corundum {
        def preview_cranelift_ir(iseq: Vec<String>) {
            corundum_jit::run();
            println!("Ran a JITed function!");
        }
    }
}
