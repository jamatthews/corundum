#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

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

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
