#[macro_use]
use cranelift::prelude::*;
use cranelift_codegen::ir::types::I64;

use corundum_ruby::fixnum::rb_int2inum;
use corundum_ruby::ruby_special_consts::RUBY_Qnil;

use opcode::OpCode;
use translation_state::TranslationState;

pub fn translate_code(op: OpCode, offset: i32, builder: &mut FunctionBuilder, state: &mut TranslationState, _return_pointer: &Value) {
    match op {
        OpCode::Nop => {},
        OpCode::PutNil => {
            let value = builder.ins().iconst(I64, RUBY_Qnil as i64);
            state.push(value);
        },
        OpCode::PutSelf => {},
        OpCode::PutObject(object) => {
            let value = builder.ins().iconst(I64, object as i64);
            state.push(value);
        },
        OpCode::Pop => {
            state.pop();
        },
        OpCode::OptSendWithoutBlock(_) => {},
        OpCode::Leave => {
            let value = state.pop();
            builder.ins().return_(&[value]);
        },
        OpCode::Jump(target) => {
            builder.ins().jump(state.get_block(target + offset).unwrap(), &[]);
        },
        OpCode::BranchIf(target) => {
            builder.ins().brnz(state.pop(), state.get_block(target + offset).unwrap(), &[]);
        },
        OpCode::BranchUnless(target) => {
            builder.ins().brz(state.pop(), state.get_block(target + offset).unwrap(), &[]);
        },
        OpCode::OptPlus => {
            let lhs = state.pop();
            let rhs = state.pop();
            let result = builder.ins().iadd(lhs, rhs);
            state.push(result);
        },
        OpCode::OptMinus => {
            let lhs = state.pop();
            let rhs = state.pop();
            let result = builder.ins().isub(lhs, rhs);
            state.push(result);
        },
        OpCode::OptMulti => {
            let lhs = state.pop();
            let rhs = state.pop();
            let result = builder.ins().imul(lhs, rhs);
            state.push(result);
        },
        OpCode::OptLt => {
            let rhs = state.pop();
            let lhs = state.pop();
            let result = builder.ins().icmp(IntCC::SignedLessThan, lhs, rhs);
            state.push(result);
        },
        OpCode::OptGt => {
            let rhs = state.pop();
            let lhs = state.pop();
            let result = builder.ins().icmp(IntCC::SignedGreaterThan, lhs, rhs);
            state.push(result);
        },
        OpCode::SetLocalWc0(index) => {
            let value = state.pop();
            builder.def_var(Variable::with_u32(index), value);
        },
        OpCode::GetLocalWc0(index) => {
             state.push(builder.use_var(Variable::with_u32(index)))
        },
        OpCode::PutObjectInt2Fix0 => {
            let value = builder.ins().iconst(I64, 0);
            state.push(value);
        },
        OpCode::PutObjectInt2Fix1 => {
            let value = builder.ins().iconst(I64, 1);
            state.push(value);
        },
    }
}
