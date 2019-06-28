use cranelift::prelude::*;
use cranelift_codegen::ir::types::I64;

use corundum_ruby::fixnum::rb_int2inum;
use corundum_ruby::ruby_special_consts::RUBY_Qnil;

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
            let value = state.pop();
            builder.ins().return_(&[value]);
        },
        OpCode::PutNil => { //putnil
            if builder.is_filled() {
                state.between_blocks = true;
            } else {
                let value = builder.ins().iconst(I64, RUBY_Qnil as i64);
                state.push(value);
            }
        },
        OpCode::OptPlus => {
            let lhs_value = state.pop();
            let rhs_value = state.pop();

            //VALUE has the lowest bit set to 1 to flag integers
            let lhs_int = builder.ins().ushr_imm(lhs_value, 1);
            let rhs_int = builder.ins().ushr_imm(rhs_value, 1);
            let result_int = builder.ins().iadd(lhs_int, rhs_int);
            let value = builder.ins().ishl_imm(result_int, 1);
            let value = builder.ins().iadd_imm(value, 1);

            state.push(value);
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
            let zero = unsafe { rb_int2inum(1) };
            let value = builder.ins().iconst(I64, zero.value as i64);
            state.push(value);
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
