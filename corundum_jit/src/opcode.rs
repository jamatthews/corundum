pub enum OpCode {
    PutObject(i64),
    SetLocal(u32),
    GetLocal(u32),
    OptPlus,
    OptLt,
    Jump(usize),
    Label(usize),
    BranchIf(usize),
}
