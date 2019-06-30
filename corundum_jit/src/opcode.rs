use corundum_ruby::rb_fix2int;
use corundum_ruby::rb_vm_insn_addr2insn;
use corundum_ruby::value::Value;

#[derive(Debug)]
pub enum OpCode {
    Nop,
    PutNil,
    PutObject(u64),
    Pop,
    Leave,
    Jump(i32),
    BranchIf(i32),
    BranchUnless(i32),
    OptPlus,
    OptLt,
    GetLocalWc0(u32),
    SetLocalWc0(u32),
    PutObjectInt2Fix0,
    PutObjectInt2Fix1,
}

impl From<(*const u64, *const u64)> for OpCode {
    fn from(pointers: (*const u64, *const u64)) -> Self {
        let insn: i32 = unsafe { rb_vm_insn_addr2insn(*pointers.0 as *const _) };

        match insn {
            0 => OpCode::Nop,
            16 => OpCode::PutNil,
            18 => OpCode::PutObject(unsafe { *pointers.1 }),
            35 => OpCode::Pop,
            57 => OpCode::Leave,
            59 => OpCode::Jump(unsafe { *pointers.1 } as i32),
            60 => OpCode::BranchIf(unsafe { *pointers.1 } as i32),
            61 => OpCode::BranchUnless(unsafe { *pointers.1 } as i32),
            67 => OpCode::OptPlus,
            74 => OpCode::OptLt,
            95 => OpCode::GetLocalWc0(unsafe { *pointers.1 } as u32),
            97 => { OpCode::SetLocalWc0(unsafe { *pointers.1 } as u32) },
            99 => OpCode::PutObjectInt2Fix0,
            100 => OpCode::PutObjectInt2Fix1,
             _ => { panic!("Unknown opcode: {:?}", insn) }
        }
    }
}

impl OpCode {
    pub fn size(&self) -> u32 {
        match *self {
            OpCode::OptPlus|OpCode::OptLt => 3,
            OpCode::PutObject(_)
                |OpCode::Jump(_)
                |OpCode::BranchIf(_)
                |OpCode::BranchUnless(_)
                |OpCode::SetLocalWc0(_)
                |OpCode::GetLocalWc0(_)
                => 2,
            _ => 1
        }
    }
}
