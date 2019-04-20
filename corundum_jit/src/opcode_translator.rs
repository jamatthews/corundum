use cranelift::prelude::*;
use cranelift::prelude::Value as Value;
use cranelift_codegen::ir::types::I64;
use cranelift_codegen::isa::TargetFrontendConfig;

use opcode::OpCode;
use translation_state::TranslationState;

use corundum_ruby::value::*;
use corundum_ruby::value::Value as RValue;
use corundum_ruby::types::*;

use NIL;

pub fn translate_code(op: OpCode, builder: &mut FunctionBuilder, state: &mut TranslationState, return_pointer: &Value) {
    match op {
        OpCode::PutObject(obj) => {
            let value = builder.ins().iconst(I64, (&obj as *const RValue) as i64);
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
            let lhs_rvalue = state.pop();
            let rhs_rvalue = state.pop();

            let lhs = builder.ins().load(I64, MemFlags::new(), lhs_rvalue, 0);
            let rhs = builder.ins().load(I64, MemFlags::new(), rhs_rvalue, 0);

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
            let value = state.pop();
            // let value1 = builder.ins().load(I64, MemFlags::new(), pointer, 0);
            // builder.ins().store(MemFlags::new(), value1, *return_pointer, 0);
            // let value2 = builder.ins().load(I64, MemFlags::new(), pointer, 8);
            // builder.ins().store(MemFlags::new(), value2, *return_pointer, 8);
            // let value3 = builder.ins().load(I64, MemFlags::new(), pointer, 16);
            // builder.ins().store(MemFlags::new(), value3, *return_pointer, 16);
            // let value4 = builder.ins().load(I64, MemFlags::new(), pointer, 24);
            // builder.ins().store(MemFlags::new(), value4, *return_pointer, 24);
            // let value5 = builder.ins().load(I64, MemFlags::new(), pointer, 32);
            // builder.ins().store(MemFlags::new(), value5, *return_pointer, 32);
            builder.ins().return_(&[value]);
        },
        OpCode::PutNil => {
            if builder.is_filled() {
                state.between_blocks = true;
            } else {
                let raw_obj_pointer = (&NIL as *const RValue) as i64;
                println!("putnil: {:?}", raw_obj_pointer);
                let value = builder.ins().iconst(I64, (&NIL as *const RValue) as i64);
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

#[cfg(test)]
mod tests {
    use super::*;
    use jit::*;
    use corundum_ruby::fixnum::rb_int2inum;
    use NIL;

    #[test]
    fn putnil_and_leave() {
        let raw_obj_pointer = (&NIL as *const RValue) as i64;
        println!("test: {:?}", raw_obj_pointer);
        let bytecode: Vec<Vec<String>> = vec![vec!["putnil".into()], vec!["leave".into()]];
        let result = JIT::new().run("test", &bytecode, vec![]);
        let original = unsafe{ *(raw_obj_pointer as *const RValue) };
        assert_eq!(NIL, original);
        assert_eq!(NIL, result)
    }

    fn putobj_and_leave() {
        let bytecode: Vec<Vec<String>> = vec![vec!["putobject".into(), "0".into()], vec!["leave".into()]];
        let expected = unsafe { rb_int2inum(0) };
        let result = JIT::new().run("test", &bytecode, vec![]);
        assert!(result.is_fixnum())
    }

    #[test]
    fn my_sanity() {
        let raw_obj_pointer = (&NIL as *const RValue) as i64;
        let original = unsafe{ *(raw_obj_pointer as *const RValue) };
        assert_eq!(NIL, original);
        assert!(original.is_nil())
    }
}
