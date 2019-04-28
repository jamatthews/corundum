use libc::c_char;
use value::{Value,RubySpecialConsts};
use types::InternalValue;

extern "C" {
   fn rb_iseqw_to_iseq(x: Value) -> *const Iseq;
   fn rb_vm_insn_addr2insn(x: Value) -> u64;
}

pub fn vm_insn_addr2insn(x: Value) -> u64 {
   unsafe { rb_vm_insn_addr2insn(x) }
}

pub fn iseqw_to_iseq(x: Value) -> Iseq {
   unsafe { *rb_iseqw_to_iseq(x) }
}

#[repr(C)]
#[derive(Clone,Copy,Derivative)]
#[derivative(Debug)]
pub struct Iseq {
    pub flags: Value,
    pub reserved1: Value,
    pub body: *const IseqConstantBody,
    #[derivative(Debug="ignore")]
    pub aux: IseqAux,
}

#[repr(C)]
#[derive(Clone,Copy)]
pub union IseqAux {
    pub compile_data: *const Unused,
    pub loader: Loader,
}

#[repr(C)]
#[derive(Clone,Copy)]
pub struct Loader {
    pub obj: Value,
    pub index: i64,
}

#[repr(C)]
#[derive(Clone,Copy,Derivative)]
#[derivative(Debug)]
pub struct IseqConstantBody {
    #[derivative(Debug="ignore")]
    iseq_type: IseqType,
    pub iseq_size: u64,
    #[derivative(Debug="ignore")]
    pub iseq_encoded: Value,
    #[derivative(Debug="ignore")]
    param: *const Unused,
    #[derivative(Debug="ignore")]
    location: *const Unused,
    #[derivative(Debug="ignore")]
    insns_info: *const Unused,
    #[derivative(Debug="ignore")]
    local_table: *const Unused,
    #[derivative(Debug="ignore")]
    parent_iseq: *const Unused,
    #[derivative(Debug="ignore")]
    local_iseq: *const Unused,
    #[derivative(Debug="ignore")]
    is_entries: *const Unused,
    #[derivative(Debug="ignore")]
    ci_entries: *const Unused,
    #[derivative(Debug="ignore")]
    cc_entries: *const Unused,
    #[derivative(Debug="ignore")]
    variable: *const Unused,
    local_table_size: u64,
    is_size: u64,
    ci_size: u64,
    ci_kw_size: u64,
    stack_max: u64,
    #[derivative(Debug="ignore")]
    jit_func: *const Unused,
    total_calls: u64,
    #[derivative(Debug="ignore")]
    jit_unit: *const Unused,
    #[derivative(Debug="ignore")]
    catch_except_p: c_char,
}

#[repr(C)]
#[derive(Clone,Copy)]
pub struct Unused { _private: [u8; 0] }

#[repr(C)]
#[derive(Clone,Copy)]
pub enum IseqType {
    Top,
    Method,
    Block,
    Class,
    Rescue,
    Ensure,
    Eval,
    Main,
    Plain
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rb_iseqw_to_iseq() {
        let addr = Value{ value: RubySpecialConsts::Nil as InternalValue };
        iseqw_to_iseq(addr);
    }

    #[test]
    fn test_vm_insn() {
        let addr = Value{ value: RubySpecialConsts::Nil as InternalValue };
        vm_insn_addr2insn(addr);
    }
}
