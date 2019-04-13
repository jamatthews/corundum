use cranelift::prelude::*;
use cranelift_codegen::ir::types::I64;

use opcode::OpCode;
use translation_state::TranslationState;

pub fn translate_code(op: OpCode, builder: &mut FunctionBuilder, state: &mut TranslationState) {
    match op {
        OpCode::PutObject(obj) => {
            let value = builder.ins().iconst(I64, i64::from(obj));
            state.push(value);
        },
        OpCode::SetLocal(index) => {
            let value = state.pop();
            builder.def_var(Variable::with_u32(index), value);
        },
        OpCode::GetLocal(index) => {
            state.push(builder.use_var(Variable::with_u32(index)))
        },
        OpCode::OptPlus => {
            let lhs = state.pop();
            let rhs = state.pop();
            let value = builder.ins().iadd(lhs, rhs);
            state.push(value);
        },
        OpCode::OptLt => {
            let rhs = state.pop();
            let lhs = state.pop();
            let c = builder.ins().icmp(IntCC::UnsignedLessThan, lhs, rhs);
            let value = builder.ins().bint(I64, c);
            state.push(value);
        },
        OpCode::Jump(label) => {
            builder.ins().jump(state.get_block(label), &[]);
        },
        OpCode::Label(label) => {
            if builder.is_filled() {
                builder.switch_to_block(state.get_block(label));
            } else {
                builder.ins().jump(state.get_block(label), &[]);
                builder.switch_to_block(state.get_block(label));
            }
        },
        OpCode::BranchIf(label) => {
            builder.ins().brnz(state.pop(), state.get_block(label), &[]);
        },
        OpCode::Leave => {
            builder.ins().return_(&[state.pop()]);
        },
        OpCode::PutNil => {
            if builder.is_filled() {
                state.between_blocks = true;
            } else {
                let value = builder.ins().iconst(I64, i64::from(0));
                state.push(value);
            }
        },
        OpCode::Pop => {
            if state.between_blocks {
                state.between_blocks = false;
            } else {
                state.pop();
            }
        }
    }
}
