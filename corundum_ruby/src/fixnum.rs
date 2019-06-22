use types::{c_long, SignedValue, Value};

extern "C" {
    pub fn rb_int2inum(num: SignedValue) -> Value;
}
