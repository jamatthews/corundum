#[macro_use]
extern crate helix;

ruby! {
    class Corundum {
        def preview_cranelift_ir() {
            println!("Rusty!");
        }
    }
}
