#[macro_use]
use cranelift::prelude::*;
use cranelift_codegen::ir::types::I64;

use corundum_ruby::fixnum::rb_int2inum;
use corundum_ruby::ruby_special_consts::RUBY_Qnil;

use opcode::OpCode;
use translation_state::TranslationState;

macro_rules! b1_2_value {
    ($x:ident, $builder:ident) => {{
        let fifth_bit = $builder.ins().bint(I64, $x);
        let fifth_bit = $builder.ins().ishl_imm(fifth_bit, 4);
        let third_bit = $builder.ins().bint(I64, $x);
        let third_bit = $builder.ins().ishl_imm(third_bit, 2);
        $builder.ins().iadd(fifth_bit, third_bit)
    }}
}

macro_rules! value_2_i64 {
    ($x:ident, $builder:ident) => {{
        $builder.ins().ushr_imm($x, 1)
    }}
}

macro_rules! i64_2_value {
    ($x:ident, $builder:ident) => {{
        let value = $builder.ins().ishl_imm($x, 1);
        $builder.ins().iadd_imm(value, 1)
    }}
}

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
        OpCode::OptSendWithoutBlock => {},
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
            let lhs_int = value_2_i64!(lhs, builder);
            let rhs_int = value_2_i64!(rhs, builder);
            let result = builder.ins().iadd(lhs_int, rhs_int);
            state.push(i64_2_value!(result, builder));
        },
        OpCode::OptMinus => {
            let lhs = state.pop();
            let rhs = state.pop();
            let lhs_int = value_2_i64!(lhs, builder);
            let rhs_int = value_2_i64!(rhs, builder);
            let result = builder.ins().isub(lhs_int, rhs_int);
            state.push(i64_2_value!(result, builder));
        },
        OpCode::OptMulti => {
            let lhs = state.pop();
            let rhs = state.pop();
            let lhs_int = value_2_i64!(lhs, builder);
            let rhs_int = value_2_i64!(rhs, builder);
            let result = builder.ins().imul(lhs_int, rhs_int);
            state.push(i64_2_value!(result, builder));
        },
        OpCode::OptLt => {
            let rhs = state.pop();
            let lhs = state.pop();
            let rhs_int = value_2_i64!(rhs, builder);
            let lhs_int = value_2_i64!(lhs, builder);
            let result = builder.ins().icmp(IntCC::SignedLessThan, lhs_int, rhs_int);
            state.push(b1_2_value!(result, builder));
        },
        OpCode::OptGt => {
            let rhs = state.pop();
            let lhs = state.pop();
            let rhs_int = value_2_i64!(rhs, builder);
            let lhs_int = value_2_i64!(lhs, builder);
            let result = builder.ins().icmp(IntCC::SignedGreaterThan, lhs_int, rhs_int);
            state.push(b1_2_value!(result, builder));
        },
        OpCode::SetLocalWc0(index) => {
            let value = state.pop();
            builder.def_var(Variable::with_u32(index), value);
        },
        OpCode::GetLocalWc0(index) => {
             state.push(builder.use_var(Variable::with_u32(index)))
        },
        OpCode::PutObjectInt2Fix0 => {
            let zero = unsafe { rb_int2inum(0) };
            let value = builder.ins().iconst(I64, zero.value as i64);
            state.push(value);
        },
        OpCode::PutObjectInt2Fix1 => {
            let one = unsafe { rb_int2inum(1) };
            let value = builder.ins().iconst(I64, one.value as i64);
            state.push(value);
        },
    }
}
