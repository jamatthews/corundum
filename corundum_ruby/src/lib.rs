#[macro_use]
extern crate derivative;
extern crate libc;

pub mod fixnum;
pub mod types;
pub mod typed_data;
pub mod value;
pub mod vm;

use types::Value;

extern {
    pub fn rb_fix_plus_fix(a: Value, b: Value) -> Value;
}
