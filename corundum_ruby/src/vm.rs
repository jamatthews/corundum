use value::Value;

#[repr(C)]
pub struct Iseq {
    pub flags: Value,
    pub reserved1: Value,
    pub body: *const IseqConstantBody,
    pub aux: IseqAux,
}

#[repr(C)]
pub union IseqAux {
    pub compile_data: *const IseqCompileData,
    pub loader: Loader,
}

#[repr(C)]
#[derive(Clone,Copy)]
pub struct Loader {
    pub obj: Value,
    pub index: i64,
}

#[repr(C)]
#[derive(Clone,Copy)]
pub struct IseqCompileData { _private: [u8; 0] }
#[repr(C)] pub struct IseqConstantBody { _private: [u8; 0] }

extern {
    pub fn rb_iseqw_to_iseq(x: Value) -> Iseq;
}
