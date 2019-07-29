use cranelift::prelude::*;
use cranelift_codegen::Context;
use cranelift_codegen::isa::CallConv;
use cranelift_codegen::ir::Function;
use cranelift_codegen::ir::types::I64;
use cranelift_module::*;
use cranelift_module::FuncOrDataId::Func;
use cranelift_simplejit::*;

use corundum_ruby::rb_id2sym;
use corundum_ruby::fixnum::rb_int2inum;
use corundum_ruby::rb_iseq_t;
use corundum_ruby::rb_method_iseq;
use corundum_ruby::rb_obj_method;
use corundum_ruby::ruby_special_consts::RUBY_Qnil;
use corundum_ruby::ruby_current_execution_context_ptr;

use opcode::OpCode;
use method_translator::MethodTranslator;
use translation_state::TranslationState;

use corundum_ruby::rb_const_get;
use corundum_ruby::rb_cObject;

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

pub fn translate_code(op: OpCode, offset: i32, builder: &mut FunctionBuilder, state: &mut TranslationState, module: &mut Module<SimpleJITBackend>) {
    match op {
        OpCode::Nop => {},
        OpCode::GetConstant(arg) => {
            let constant = unsafe { rb_const_get(rb_cObject, arg) };
            state.push_static(constant);
        },
        OpCode::PutNil => {
            let value = builder.ins().iconst(I64, RUBY_Qnil as i64);
            state.push(value);
        },
        OpCode::PutSelf => {
            let self_ = unsafe {
                let ec = ruby_current_execution_context_ptr;
                 (*(*ec).cfp).self_
             };
            state.push_static(self_);
        },
        OpCode::PutObject(object) => {
            let value = builder.ins().iconst(I64, object as i64);
            state.push(value);
        },
        OpCode::Pop => {
            state.pop();
        },
        OpCode::OptSendWithoutBlock(call_info) => {
            println!("compiling call");
            unsafe {
                let receiver = state.pop_static();
                let method = rb_obj_method(receiver, rb_id2sym(call_info.mid));
                let iseq: rb_iseq_t = *rb_method_iseq(method);

                match (*module).get_name(&call_info.mid.to_string()) {
                    Some(Func(func_id)) => {
                        let local_callee = (*module).declare_func_in_func(func_id, &mut builder.func);
                        let call = if(call_info.orig_argc > 0) {
                            let arg = state.pop();
                             builder.ins().call(local_callee, &vec![arg])
                        } else {
                            builder.ins().call(local_callee, &vec![])
                        };

                        state.push(builder.inst_results(call)[0])
                    },
                    _ => {
                        let sig = Signature {
                            params: vec![AbiParam::new(I64)],
                            returns: vec![AbiParam::new(I64)],
                            call_conv: CallConv::SystemV,
                        };

                        let mut codegen_context = Context::new();
                        let func_id = (*module).declare_function(&call_info.mid.to_string(), Linkage::Local, &sig).unwrap();
                        codegen_context.func = Function::with_name_signature(ExternalName::user(0, func_id.as_u32()), sig);
                        {
                            let mut builder_context = FunctionBuilderContext::new();
                            let mut builder = FunctionBuilder::new(&mut codegen_context.func, &mut builder_context);

                            MethodTranslator::new(&mut (*module), builder).translate(iseq).unwrap();
                        }

                        (*module).define_function(func_id, &mut codegen_context).unwrap();

                        let local_callee = (*module).declare_func_in_func(func_id, &mut builder.func);

                        let call = if(call_info.orig_argc > 0) {
                            let arg = state.pop();
                             builder.ins().call(local_callee, &vec![arg])
                        } else {
                            builder.ins().call(local_callee, &vec![])
                        };
                        state.push(builder.inst_results(call)[0])
                    }
                }
            }
        },
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
        OpCode::GetInlineCache => {},
        OpCode::SetInlineCache => {},
        OpCode::OptPlus => {
            let lhs = state.pop();
            let rhs = state.pop();
            let lhs_int = value_2_i64!(lhs, builder);
            let rhs_int = value_2_i64!(rhs, builder);
            let result = builder.ins().iadd(lhs_int, rhs_int);
            state.push(i64_2_value!(result, builder));
        },
        OpCode::OptMinus => {
            let rhs = state.pop();
            let lhs = state.pop();
            let rhs_int = value_2_i64!(rhs, builder);
            let lhs_int = value_2_i64!(lhs, builder);
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
