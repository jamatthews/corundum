use cranelift::prelude::*;
use cranelift_codegen::ir::types::I64;

use helix::sys::Qnil;

use opcode::OpCode;
use translation_state::TranslationState;

pub fn translate_code(op: OpCode, builder: &mut FunctionBuilder, state: &mut TranslationState, _return_pointer: &Value) {
    match op {
        // OpCode::PutObject(obj) => {
        //     let value = builder.ins().iconst(I64, (&obj as *const RValue) as i64);
        //     state.push(value);
        // },
        // OpCode::SetLocal(index) => {
        //     let value = state.pop();
        //     builder.def_var(Variable::with_u32(index), value);
        // },
        // OpCode::GetLocal(index) => {
        //     state.push(builder.use_var(Variable::with_u32(index)))
        // },
        // OpCode::OptPlus => {
        //     let lhs_rvalue = state.pop();
        //     let rhs_rvalue = state.pop();
        //
        //     let lhs = builder.ins().load(I64, MemFlags::new(), lhs_rvalue, 0);
        //     let rhs = builder.ins().load(I64, MemFlags::new(), rhs_rvalue, 0);
        //
        //     let value = builder.ins().iadd(lhs, rhs);
        //     state.push(value);
        // },
        // OpCode::OptLt => {
        //     let rhs = state.pop();
        //     let lhs = state.pop();
        //     let c = builder.ins().icmp(IntCC::UnsignedLessThan, lhs, rhs);
        //     let value = builder.ins().bint(I64, c);
        //     state.push(value);
        // },
        // OpCode::Jump(label) => {
        //     builder.ins().jump(state.get_block(label), &[]);
        // },
        // OpCode::Label(label) => {
        //     if builder.is_filled() {
        //         builder.switch_to_block(state.get_block(label));
        //     } else {
        //         builder.ins().jump(state.get_block(label), &[]);
        //         builder.switch_to_block(state.get_block(label));
        //     }
        // },
        // OpCode::BranchIf(label) => {
        //     builder.ins().brnz(state.pop(), state.get_block(label), &[]);
        // },
        OpCode::Leave => {  //leave
            let pointer = state.pop();
            let value1 = builder.ins().load(I64, MemFlags::new(), pointer, 0);
            let value2 = builder.ins().load(I64, MemFlags::new(), pointer, 8);
            builder.ins().return_(&[value1, value2]);
        },
        OpCode::PutNil => { //putnil
            if builder.is_filled() {
                state.between_blocks = true;
            } else {
                let value = builder.ins().iconst(I64, unsafe{ (&Qnil as *const _) as i64 });
                state.push(value);
            }
        },
        OpCode::PutObjectInt2Fix0 => {
            let value = builder.ins().iconst(I64, unsafe{ (&Qnil as *const _) as i64 });
            state.push(value);
        },
        OpCode::SetLocalWc0(index) => {
            let value = state.pop();
            builder.def_var(Variable::with_u32(index), value);
        },
        OpCode::GetLocalWc0(index) => {
             state.push(builder.use_var(Variable::with_u32(index)))
        },

        // OpCode::Pop => {
        //     if state.between_blocks {
        //         state.between_blocks = false;
        //     } else {
        //         state.pop();
        //     }
        // }
    }
}
