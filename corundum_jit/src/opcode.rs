pub enum OpCode {
    PutObject(i64),
    SetLocal(u32),
    GetLocal(u32),
    OptPlus,
    OptLt,
    Jump(usize),
    Label(usize),
    BranchIf(usize),
    PutNil,
    Pop,
    Leave,
}

impl From<&Vec<String>> for OpCode {
    fn from(instruction: &Vec<String>) -> Self {
        match instruction[0].as_str() {
            "putobject_OP_INT2FIX_O_0_C_" => OpCode::PutObject(0),
            "putobject_OP_INT2FIX_O_1_C_" => OpCode::PutObject(1),
            "putobject" => OpCode::PutObject(instruction[1].parse::<i64>().expect("putobj failed")),
            "setlocal_OP__WC__0" => OpCode::SetLocal(instruction[1].parse::<u32>().expect("setlocal failed")),
            "jump" => OpCode::Jump(instruction[1].parse::<usize>().expect("jump failed")),
            "putnil" => OpCode::PutNil,
            "label" => OpCode::Label(instruction[1].parse::<usize>().expect("putobj failed")),
            "pop" => OpCode::Pop,
            "getlocal_OP__WC__0" => OpCode::GetLocal(instruction[1].parse::<u32>().expect("getlocal failed")),
            "opt_plus" => OpCode::OptPlus,
            "branchif" => OpCode::BranchIf(instruction[1].parse::<usize>().expect("branchif failed")),
            "leave" => OpCode::Leave,
            "opt_lt" => OpCode::OptLt,
            x => panic!("unknown opcode: {}", x),
        }
    }
}
