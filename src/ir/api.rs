// use llvm::prelude::*;

// use crate::{
//     memory_management::pointer::ThreadSafePtr,
//     // interface::LLVMApi,
//     ir::{
//         block, builder, ops, top_level_exp, types, values, var,
//     },
// };

// pub struct SafeLLVM;

// // impl LLVMApi for SafeLLVM {
//     /// --- BLOCK --- ///
//     // fn create_basic_block(&self, context: ThreadSafePtr<LLVMContextRef>, function: ThreadSafePtr<LLVMValueRef>, name: &str) -> ThreadSafePtr<LLVMBasicBlockRef> {
//     //     block::create_basic_block(context, function, name)
//     // }

//     // fn get_current_block(&self, builder: ThreadSafePtr<LLVMBuilderRef>) -> ThreadSafePtr<LLVMBasicBlockRef> {
//     //     block::get_current_block(builder)
//     // }

//     // fn create_cond_br(&self, builder: ThreadSafePtr<LLVMBuilderRef>, condition: ThreadSafePtr<LLVMValueRef>, then_bb: ThreadSafePtr<LLVMBasicBlockRef>, else_bb: ThreadSafePtr<LLVMBasicBlockRef>) -> ThreadSafePtr<LLVMValueRef> {
//     //     block::create_cond_br(builder, condition, then_bb, else_bb)
//     // }

//     // fn create_br(&self, builder: ThreadSafePtr<LLVMBuilderRef>, target_bb: ThreadSafePtr<LLVMBasicBlockRef>) -> ThreadSafePtr<LLVMValueRef> {
//     //     block::create_br(builder, target_bb)
//     // }

//     // fn insert_before_basic_block(&self, context: ThreadSafePtr<LLVMContextRef>, before_target: ThreadSafePtr<LLVMBasicBlockRef>, name: &str) -> ThreadSafePtr<LLVMBasicBlockRef> {
//     //     block::insert_before_basic_block(context, before_target, name)
//     // }

//     // fn position_builder(&self, builder: ThreadSafePtr<LLVMBuilderRef>, bb: ThreadSafePtr<LLVMBasicBlockRef>) {
//     //     block::position_builder(builder, bb)
//     // }

//     // fn delete_basic_block(&self, bb: ThreadSafePtr<LLVMBasicBlockRef>) {
//     //     block::delete_basic_block(bb)
//     // }

//     // fn get_first_instruction(&self, bb: ThreadSafePtr<LLVMBasicBlockRef>) -> ThreadSafePtr<LLVMValueRef> {
//     //     block::get_first_instruction(bb)
//     // }

//     // fn get_last_instruction(&self, bb: ThreadSafePtr<LLVMBasicBlockRef>) -> ThreadSafePtr<LLVMValueRef> {
//     //     block::get_last_instruction(bb)
//     // }

//     // fn create_phi(&self, builder: ThreadSafePtr<LLVMBuilderRef>, possible_values: &[(ThreadSafePtr<LLVMValueRef>, ThreadSafePtr<LLVMBasicBlockRef>)], name: &str) -> ThreadSafePtr<LLVMValueRef> {
//     //     block::create_phi(builder, possible_values, name)
//     // }

//     // /// --- BUILDER --- ///
//     // fn create_builder(&self, context: ThreadSafePtr<LLVMContextRef>) -> ThreadSafePtr<LLVMBuilderRef> {
//     //     builder::create_builder(context)
//     // }

//     /// --- OPS --- ///
//     fn build_add(&self, builder: ThreadSafePtr<LLVMBuilderRef>, param_a: ThreadSafePtr<LLVMValueRef>, param_b: ThreadSafePtr<LLVMValueRef>, sum: &str) -> ThreadSafePtr<LLVMValueRef> {
//         ops::build_add(builder, param_a, param_b, sum)
//     }

//     fn build_sub(&self, builder: ThreadSafePtr<LLVMBuilderRef>, param_a: ThreadSafePtr<LLVMValueRef>, param_b: ThreadSafePtr<LLVMValueRef>, name: &str) -> ThreadSafePtr<LLVMValueRef> {
//         ops::build_sub(builder, param_a, param_b, name)
//     }

//     fn build_mul(&self, builder: ThreadSafePtr<LLVMBuilderRef>, param_a: ThreadSafePtr<LLVMValueRef>, param_b: ThreadSafePtr<LLVMValueRef>, name: &str) -> ThreadSafePtr<LLVMValueRef> {
//         ops::build_mul(builder, param_a, param_b, name)
//     }

//     fn build_div(&self, builder: ThreadSafePtr<LLVMBuilderRef>, param_a: ThreadSafePtr<LLVMValueRef>, param_b: ThreadSafePtr<LLVMValueRef>, name: &str) -> ThreadSafePtr<LLVMValueRef> {
//         ops::build_div(builder, param_a, param_b, name)
//     }

//     fn build_rem(&self, builder: ThreadSafePtr<LLVMBuilderRef>, param_a: ThreadSafePtr<LLVMValueRef>, param_b: ThreadSafePtr<LLVMValueRef>, name: &str) -> ThreadSafePtr<LLVMValueRef> {
//         ops::build_rem(builder, param_a, param_b, name)
//     }

//     fn build_and(&self, builder: ThreadSafePtr<LLVMBuilderRef>, param_a: ThreadSafePtr<LLVMValueRef>, param_b: ThreadSafePtr<LLVMValueRef>, name: &str) -> ThreadSafePtr<LLVMValueRef> {
//         ops::build_and(builder, param_a, param_b, name)
//     }

//     fn build_or(&self, builder: ThreadSafePtr<LLVMBuilderRef>, param_a: ThreadSafePtr<LLVMValueRef>, param_b: ThreadSafePtr<LLVMValueRef>, name: &str) -> ThreadSafePtr<LLVMValueRef> {
//         ops::build_or(builder, param_a, param_b, name)
//     }

//     fn build_xor(&self, builder: ThreadSafePtr<LLVMBuilderRef>, param_a: ThreadSafePtr<LLVMValueRef>, param_b: ThreadSafePtr<LLVMValueRef>, name: &str) -> ThreadSafePtr<LLVMValueRef> {
//         ops::build_xor(builder, param_a, param_b, name)
//     }

//     fn build_shl(&self, builder: ThreadSafePtr<LLVMBuilderRef>, param_a: ThreadSafePtr<LLVMValueRef>, param_b: ThreadSafePtr<LLVMValueRef>, name: &str) -> ThreadSafePtr<LLVMValueRef> {
//         ops::build_shl(builder, param_a, param_b, name)
//     }

//     fn build_shr(&self, builder: ThreadSafePtr<LLVMBuilderRef>, param_a: ThreadSafePtr<LLVMValueRef>, param_b: ThreadSafePtr<LLVMValueRef>, name: &str) -> ThreadSafePtr<LLVMValueRef> {
//         ops::build_shr(builder, param_a, param_b, name)
//     }

//     fn build_icmp_gt(&self, builder: ThreadSafePtr<LLVMBuilderRef>, param_a: ThreadSafePtr<LLVMValueRef>, param_b: ThreadSafePtr<LLVMValueRef>, name: &str) -> ThreadSafePtr<LLVMValueRef> {
//         ops::build_icmp_gt(builder, param_a, param_b, name)
//     }

//     fn build_icmp_lt(&self, builder: ThreadSafePtr<LLVMBuilderRef>, param_a: ThreadSafePtr<LLVMValueRef>, param_b: ThreadSafePtr<LLVMValueRef>, name: &str) -> ThreadSafePtr<LLVMValueRef> {
//         ops::build_icmp_lt(builder, param_a, param_b, name)
//     }

//     fn build_icmp_eq(&self, builder: ThreadSafePtr<LLVMBuilderRef>, param_a: ThreadSafePtr<LLVMValueRef>, param_b: ThreadSafePtr<LLVMValueRef>, name: &str) -> ThreadSafePtr<LLVMValueRef> {
//         ops::build_icmp_eq(builder, param_a, param_b, name)
//     }

//     fn build_negation(&self, builder: ThreadSafePtr<LLVMBuilderRef>, operand_ir: ThreadSafePtr<LLVMValueRef>, name: &str) -> ThreadSafePtr<LLVMValueRef> {
//         ops::build_negation(builder, operand_ir, name)
//     }

//     fn build_bitwise_not(&self, builder: ThreadSafePtr<LLVMBuilderRef>, operand_ir: ThreadSafePtr<LLVMValueRef>, name: &str) -> ThreadSafePtr<LLVMValueRef> {
//         ops::build_bitwise_not(builder, operand_ir, name)
//     }

//     fn build_logical_not(&self, builder: ThreadSafePtr<LLVMBuilderRef>, context: ThreadSafePtr<LLVMContextRef>, operand_ir: ThreadSafePtr<LLVMValueRef>, name: &str) -> ThreadSafePtr<LLVMValueRef> {
//         ops::build_logical_not(builder, context, operand_ir, name)
//     }

//     /// --- TOP LEVEL EXP --- ///
//     fn get_param(&self, function: ThreadSafePtr<LLVMValueRef>, index: u32) -> ThreadSafePtr<LLVMValueRef> {
//         top_level_exp::get_param(function, index)
//     }

//     fn add_function_to_module(&self, module: ThreadSafePtr<LLVMModuleRef>, function_name: &str, function_type: ThreadSafePtr<LLVMTypeRef>) -> ThreadSafePtr<LLVMValueRef> {
//         top_level_exp::add_function_to_module(module, function_name, function_type)
//     }

//     /// --- TYPES --- ///
//     fn void_type(&self, context: ThreadSafePtr<LLVMContextRef>) -> ThreadSafePtr<LLVMTypeRef> {
//         types::void_type(context)
//     }

//     fn int_type(&self, context: ThreadSafePtr<LLVMContextRef>) -> ThreadSafePtr<LLVMTypeRef> {
//         types::int_type(context)
//     }

//     fn float_type(&self, context: ThreadSafePtr<LLVMContextRef>) -> ThreadSafePtr<LLVMTypeRef> {
//         types::float_type(context)
//     }

//     fn boolean_type(&self, context: ThreadSafePtr<LLVMContextRef>) -> ThreadSafePtr<LLVMTypeRef> {
//         types::boolean_type(context)
//     }

//     fn pointer_type(&self, element_type: ThreadSafePtr<LLVMTypeRef>) -> ThreadSafePtr<LLVMTypeRef> {
//         types::pointer_type(element_type)
//     }

//     fn array_type(&self, element_type: ThreadSafePtr<LLVMTypeRef>, num_elements: u64) -> ThreadSafePtr<LLVMTypeRef> {
//         types::array_type(element_type, num_elements)
//     }

//     // fn struct_type(&self, context: ThreadSafePtr<LLVMContextRef>, element_types: &[ThreadSafePtr<LLVMTypeRef>], packed: bool) -> ThreadSafePtr<LLVMTypeRef> {
//     //     types::struct_type(context, element_types, packed)
//     // }

//     fn void_return(&self, builder: ThreadSafePtr<LLVMBuilderRef>) -> ThreadSafePtr<LLVMValueRef> {
//         types::void_return(builder)
//     }

//     fn nonvoid_return(&self, builder: ThreadSafePtr<LLVMBuilderRef>, value: ThreadSafePtr<LLVMValueRef>) -> ThreadSafePtr<LLVMValueRef> {
//         types::nonvoid_return(builder, value)
//     }


//     /// --- VALUES --- ///
//     fn create_integer(&self, val: i64, context: ThreadSafePtr<LLVMContextRef>) -> ThreadSafePtr<LLVMValueRef> {
//         values::create_integer(val, context)
//     }

//     fn create_float(&self, val: f64, context: ThreadSafePtr<LLVMContextRef>) -> ThreadSafePtr<LLVMValueRef> {
//         values::create_float(val, context)
//     }

//     fn create_boolean(&self, val: bool, context: ThreadSafePtr<LLVMContextRef>) -> ThreadSafePtr<LLVMValueRef> {
//         values::create_boolean(val, context)
//     }
 
//     fn create_function(&self, return_type: Option<ThreadSafePtr<LLVMTypeRef>>, param_types: &[ThreadSafePtr<LLVMTypeRef>], is_var_arg: bool, context: ThreadSafePtr<LLVMContextRef>) -> Option<ThreadSafePtr<LLVMTypeRef>> {
//         builder::create_function(return_type, param_types, is_var_arg, context)
//     }

//     fn create_array(&self, value: ThreadSafePtr<LLVMValueRef>, num_elements: u64) -> ThreadSafePtr<LLVMValueRef> {
//         values::create_array(value, num_elements)
//     }

//     fn create_pointer(&self, value: ThreadSafePtr<LLVMTypeRef>) -> ThreadSafePtr<LLVMValueRef> {
//         values::create_pointer(value)
//     }

//     // fn create_struct(&self, values: &[ThreadSafePtr<LLVMValueRef>], context: ThreadSafePtr<LLVMContextRef>, packed: bool) -> ThreadSafePtr<LLVMValueRef> {
//     //     values::create_struct(values, context, packed)
//     // }

//     fn create_global_variable(&self, module: ThreadSafePtr<LLVMModuleRef>, initializer: ThreadSafePtr<LLVMValueRef>, name: &str) -> ThreadSafePtr<LLVMValueRef> {
//         values::create_global_variable(module, initializer, name)
//     }

//     fn create_string(&self, val: &str, builder: ThreadSafePtr<LLVMBuilderRef>) -> ThreadSafePtr<LLVMValueRef> {
//         values::create_string(val, builder)
//     }

//     fn create_mut_string(&self, val: &str, context: ThreadSafePtr<LLVMContextRef>, builder: ThreadSafePtr<LLVMBuilderRef>) -> ThreadSafePtr<LLVMValueRef> {
//         values::create_mut_string(val, context, builder)
//     }

//     fn create_null_pointer(&self, ty: ThreadSafePtr<LLVMTypeRef>) -> ThreadSafePtr<LLVMValueRef> {
//         values::create_null_pointer(ty)
//     }

//     fn create_continue_statement(&self, builder: ThreadSafePtr<LLVMBuilderRef>, continue_block: ThreadSafePtr<LLVMBasicBlockRef>) -> ThreadSafePtr<LLVMValueRef> {
//         values::create_continue_statement(builder, continue_block)
//     }

//     fn create_break_statement(&self, builder: ThreadSafePtr<LLVMBuilderRef>, break_block: ThreadSafePtr<LLVMBasicBlockRef>) -> ThreadSafePtr<LLVMValueRef> {
//         values::create_break_statement(builder, break_block)
//     }

//     /// --- VARIABLES --- ///
//     fn init_var(&self, builder: ThreadSafePtr<LLVMBuilderRef>, var_name: &str, data_type: ThreadSafePtr<LLVMTypeRef>, initial_value: Option<ThreadSafePtr<LLVMValueRef>>) -> ThreadSafePtr<LLVMValueRef> {
//         var::init_var(builder, var_name, data_type, initial_value)
//     }

//     fn reassign_var(&self, builder: ThreadSafePtr<LLVMBuilderRef>, variable_alloc: ThreadSafePtr<LLVMValueRef>, new_value: ThreadSafePtr<LLVMValueRef>) {
//         var::reassign_var(builder, variable_alloc, new_value)
//     }

//     fn get_var(&self, builder: ThreadSafePtr<LLVMBuilderRef>, variable_type: ThreadSafePtr<LLVMTypeRef>, variable_alloc: ThreadSafePtr<LLVMValueRef>) -> ThreadSafePtr<LLVMValueRef> {
//         var::get_var(builder, variable_type, variable_alloc)
//     }
// // }
