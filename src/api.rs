// use llvm::prelude::{LLVMContextRef, LLVMValueRef, LLVMBasicBlockRef, LLVMBuilderRef, LLVMTypeRef, LLVMModuleRef};

// use crate::{
//     interface::LLVMApi,
//     ir::SafeLLVM as IrCodeGenSafeLLVM,
//     memory_management::pointer::ThreadSafePtr,
// }; 

// pub struct SafeLLVM {
//     ir_codegen_impl: IrCodeGenSafeLLVM,
// }

// impl LLVMApi for SafeLLVM {
//     /// --- BLOCK --- ///
//     fn create_basic_block(&self, context: ThreadSafePtr<LLVMContextRef>, function: ThreadSafePtr<LLVMValueRef>, name: &str) -> ThreadSafePtr<LLVMBasicBlockRef> {
//         self.ir_codegen_impl.create_basic_block(context, function, name)
//     }

//     fn get_current_block(&self, builder: ThreadSafePtr<LLVMBuilderRef>) -> ThreadSafePtr<LLVMBasicBlockRef> {
//         self.ir_codegen_impl.get_current_block(builder)
//     }

//     fn create_cond_br(&self, builder: ThreadSafePtr<LLVMBuilderRef>, condition: ThreadSafePtr<LLVMValueRef>, then_bb: ThreadSafePtr<LLVMBasicBlockRef>, else_bb: ThreadSafePtr<LLVMBasicBlockRef>) -> ThreadSafePtr<LLVMValueRef> {
//         self.ir_codegen_impl.create_cond_br(builder, condition, then_bb, else_bb)
//     }

//     fn create_br(&self, builder: ThreadSafePtr<LLVMBuilderRef>, target_bb: ThreadSafePtr<LLVMBasicBlockRef>) -> ThreadSafePtr<LLVMValueRef> {
//         self.ir_codegen_impl.create_br(builder, target_bb)
//     }

//     fn insert_before_basic_block(&self, context: ThreadSafePtr<LLVMContextRef>, before_target: ThreadSafePtr<LLVMBasicBlockRef>, name: &str) -> ThreadSafePtr<LLVMBasicBlockRef> {
//         self.ir_codegen_impl.insert_before_basic_block(context, before_target, name)
//     }

//     fn position_builder(&self, builder: ThreadSafePtr<LLVMBuilderRef>, bb: ThreadSafePtr<LLVMBasicBlockRef>) {
//         self.ir_codegen_impl.position_builder(builder, bb)
//     }

//     fn delete_basic_block(&self, bb: ThreadSafePtr<LLVMBasicBlockRef>) {
//         self.ir_codegen_impl.delete_basic_block(bb)
//     }

//     fn get_first_instruction(&self, bb: ThreadSafePtr<LLVMBasicBlockRef>) -> ThreadSafePtr<LLVMValueRef> {
//         self.ir_codegen_impl.get_first_instruction(bb)
//     }

//     fn get_last_instruction(&self, bb: ThreadSafePtr<LLVMBasicBlockRef>) -> ThreadSafePtr<LLVMValueRef> {
//         self.ir_codegen_impl.get_last_instruction(bb)
//     }

//     fn create_phi(&self, builder: ThreadSafePtr<LLVMBuilderRef>, possible_values: &[(ThreadSafePtr<LLVMValueRef>, ThreadSafePtr<LLVMBasicBlockRef>)], name: &str) -> ThreadSafePtr<LLVMValueRef> {
//         self.ir_codegen_impl.create_phi(builder, possible_values, name)
//     }

//     /// --- BUILDER --- ///
//     fn create_builder(&self, context: ThreadSafePtr<LLVMContextRef>) -> ThreadSafePtr<LLVMBuilderRef> {
//         self.ir_codegen_impl.create_builder(context)
//     }


//     /// --- OPS --- ///
//     fn build_add(&self, builder: ThreadSafePtr<LLVMBuilderRef>, param_a: ThreadSafePtr<LLVMValueRef>, param_b: ThreadSafePtr<LLVMValueRef>, sum: &str) -> ThreadSafePtr<LLVMValueRef> {
//         self.ir_codegen_impl.build_add(builder, param_a, param_b, sum)
//     }

//     fn build_sub(&self, builder: ThreadSafePtr<LLVMBuilderRef>, param_a: ThreadSafePtr<LLVMValueRef>, param_b: ThreadSafePtr<LLVMValueRef>, name: &str) -> ThreadSafePtr<LLVMValueRef> {
//         self.ir_codegen_impl.build_sub(builder, param_a, param_b, name)
//     }

//     fn build_mul(&self, builder: ThreadSafePtr<LLVMBuilderRef>, param_a: ThreadSafePtr<LLVMValueRef>, param_b: ThreadSafePtr<LLVMValueRef>, name: &str) -> ThreadSafePtr<LLVMValueRef> {
//         self.ir_codegen_impl.build_mul(builder, param_a, param_b, name)
//     }

//     fn build_div(&self, builder: ThreadSafePtr<LLVMBuilderRef>, param_a: ThreadSafePtr<LLVMValueRef>, param_b: ThreadSafePtr<LLVMValueRef>, name: &str) -> ThreadSafePtr<LLVMValueRef> {
//         self.ir_codegen_impl.build_div(builder, param_a, param_b, name)
//     }

//     fn build_rem(&self, builder: ThreadSafePtr<LLVMBuilderRef>, param_a: ThreadSafePtr<LLVMValueRef>, param_b: ThreadSafePtr<LLVMValueRef>, name: &str) -> ThreadSafePtr<LLVMValueRef> {
//         self.ir_codegen_impl.build_rem(builder, param_a, param_b, name)
//     }

//     fn build_and(&self, builder: ThreadSafePtr<LLVMBuilderRef>, param_a: ThreadSafePtr<LLVMValueRef>, param_b: ThreadSafePtr<LLVMValueRef>, name: &str) -> ThreadSafePtr<LLVMValueRef> {
//         self.ir_codegen_impl.build_and(builder, param_a, param_b, name)
//     }

//     fn build_or(&self, builder: ThreadSafePtr<LLVMBuilderRef>, param_a: ThreadSafePtr<LLVMValueRef>, param_b: ThreadSafePtr<LLVMValueRef>, name: &str) -> ThreadSafePtr<LLVMValueRef> {
//         self.ir_codegen_impl.build_or(builder, param_a, param_b, name)
//     }

//     fn build_xor(&self, builder: ThreadSafePtr<LLVMBuilderRef>, param_a: ThreadSafePtr<LLVMValueRef>, param_b: ThreadSafePtr<LLVMValueRef>, name: &str) -> ThreadSafePtr<LLVMValueRef> {
//         self.ir_codegen_impl.build_xor(builder, param_a, param_b, name)
//     }

//     fn build_shl(&self, builder: ThreadSafePtr<LLVMBuilderRef>, param_a: ThreadSafePtr<LLVMValueRef>, param_b: ThreadSafePtr<LLVMValueRef>, name: &str) -> ThreadSafePtr<LLVMValueRef> {
//         self.ir_codegen_impl.build_shl(builder, param_a, param_b, name)
//     }

//     fn build_shr(&self, builder: ThreadSafePtr<LLVMBuilderRef>, param_a: ThreadSafePtr<LLVMValueRef>, param_b: ThreadSafePtr<LLVMValueRef>, name: &str) -> ThreadSafePtr<LLVMValueRef> {
//         self.ir_codegen_impl.build_shr(builder, param_a, param_b, name)
//     }

//     fn build_icmp_gt(&self, builder: ThreadSafePtr<LLVMBuilderRef>, param_a: ThreadSafePtr<LLVMValueRef>, param_b: ThreadSafePtr<LLVMValueRef>, name: &str) -> ThreadSafePtr<LLVMValueRef> {
//         self.ir_codegen_impl.build_icmp_gt(builder, param_a, param_b, name)
//     }

//     fn build_icmp_lt(&self, builder: ThreadSafePtr<LLVMBuilderRef>, param_a: ThreadSafePtr<LLVMValueRef>, param_b: ThreadSafePtr<LLVMValueRef>, name: &str) -> ThreadSafePtr<LLVMValueRef> {
//         self.ir_codegen_impl.build_icmp_lt(builder, param_a, param_b, name)
//     }

//     fn build_icmp_eq(&self, builder: ThreadSafePtr<LLVMBuilderRef>, param_a: ThreadSafePtr<LLVMValueRef>, param_b: ThreadSafePtr<LLVMValueRef>, name: &str) -> ThreadSafePtr<LLVMValueRef> {
//         self.ir_codegen_impl.build_icmp_eq(builder, param_a, param_b, name)
//     }

//     fn build_negation(&self, builder: ThreadSafePtr<LLVMBuilderRef>, operand_ir: ThreadSafePtr<LLVMValueRef>, name: &str) -> ThreadSafePtr<LLVMValueRef> {
//         self.ir_codegen_impl.build_negation(builder, operand_ir, name)
//     }

//     fn build_bitwise_not(&self, builder: ThreadSafePtr<LLVMBuilderRef>, operand_ir: ThreadSafePtr<LLVMValueRef>, name: &str) -> ThreadSafePtr<LLVMValueRef> {
//         self.ir_codegen_impl.build_bitwise_not(builder, operand_ir, name)
//     }

//     fn build_logical_not(&self, builder: ThreadSafePtr<LLVMBuilderRef>, context: ThreadSafePtr<LLVMContextRef>, operand_ir: ThreadSafePtr<LLVMValueRef>, name: &str) -> ThreadSafePtr<LLVMValueRef> {
//         self.ir_codegen_impl.build_logical_not(builder, context, operand_ir, name)
//     }

//     fn nonvoid_return(&self, builder: ThreadSafePtr<LLVMBuilderRef>, value: ThreadSafePtr<LLVMValueRef>) -> ThreadSafePtr<LLVMValueRef> {
//         self.ir_codegen_impl.nonvoid_return(builder, value)
//     }

//     fn void_return(&self, builder: ThreadSafePtr<LLVMBuilderRef>) -> ThreadSafePtr<LLVMValueRef> {
//         self.ir_codegen_impl.void_return(builder)
//     }

//     /// --- TOP LEVEL EXP --- ///
//     fn get_param(&self, function: ThreadSafePtr<LLVMValueRef>, index: u32) -> ThreadSafePtr<LLVMValueRef> {
//         self.ir_codegen_impl.get_param(function, index)
//     }

//     fn add_function_to_module(&self, module: ThreadSafePtr<LLVMModuleRef>, function_name: &str, function_type: ThreadSafePtr<LLVMTypeRef>) -> ThreadSafePtr<LLVMValueRef> {
//         self.ir_codegen_impl.add_function_to_module(module, function_name, function_type)
//     }

//     /// --- TYPES --- ///
//     fn void_type(&self, context: ThreadSafePtr<LLVMContextRef>) -> ThreadSafePtr<LLVMTypeRef> {
//         self.ir_codegen_impl.void_type(context)
//     }

//     fn int_type(&self, context: ThreadSafePtr<LLVMContextRef>) -> ThreadSafePtr<LLVMTypeRef> {
//         self.ir_codegen_impl.int_type(context)
//     }

//     fn float_type(&self, context: ThreadSafePtr<LLVMContextRef>) -> ThreadSafePtr<LLVMTypeRef> {
//         self.ir_codegen_impl.float_type(context)
//     }

//     fn boolean_type(&self, context: ThreadSafePtr<LLVMContextRef>) -> ThreadSafePtr<LLVMTypeRef> {
//         self.ir_codegen_impl.boolean_type(context)
//     }

//     fn pointer_type(&self, element_type: ThreadSafePtr<LLVMTypeRef>) -> ThreadSafePtr<LLVMTypeRef> {
//         self.ir_codegen_impl.pointer_type(element_type)
//     }

//     fn array_type(&self, element_type: ThreadSafePtr<LLVMTypeRef>, num_elements: u64) -> ThreadSafePtr<LLVMTypeRef> {
//         self.ir_codegen_impl.array_type(element_type, num_elements)
//     }

//     fn struct_type(&self, context: ThreadSafePtr<LLVMContextRef>, element_types: &[ThreadSafePtr<LLVMTypeRef>], packed: bool) -> ThreadSafePtr<LLVMTypeRef> {
//         self.ir_codegen_impl.struct_type(context, element_types, packed)
//     }

//     /// --- VALUES --- ///
//     fn create_integer(&self, val: i64, context: ThreadSafePtr<LLVMContextRef>) -> ThreadSafePtr<LLVMValueRef> {
//         self.ir_codegen_impl.create_integer(val, context)
//     }

//     fn create_float(&self, val: f64, context: ThreadSafePtr<LLVMContextRef>) -> ThreadSafePtr<LLVMValueRef> {
//         self.ir_codegen_impl.create_float(val, context)
//     }

//     fn create_boolean(&self, val: bool, context: ThreadSafePtr<LLVMContextRef>) -> ThreadSafePtr<LLVMValueRef> {
//         self.ir_codegen_impl.create_boolean(val, context)
//     }

//     fn create_function(&self, return_type: Option<ThreadSafePtr<LLVMTypeRef>>, param_types: &[ThreadSafePtr<LLVMTypeRef>], is_var_arg: bool, context: ThreadSafePtr<LLVMContextRef>) -> Option<ThreadSafePtr<LLVMTypeRef>> {
//         self.ir_codegen_impl.create_function(return_type, param_types, is_var_arg, context)
//     }

//     fn create_array(&self, value: ThreadSafePtr<LLVMValueRef>, num_elements: u64) -> ThreadSafePtr<LLVMValueRef> {
//         self.ir_codegen_impl.create_array(value, num_elements)
//     }

//     fn create_pointer(&self, value: ThreadSafePtr<LLVMTypeRef>) -> ThreadSafePtr<LLVMValueRef> {
//         self.ir_codegen_impl.create_pointer(value)
//     }

//     fn create_struct(&self, values: &[ThreadSafePtr<LLVMValueRef>], context: ThreadSafePtr<LLVMContextRef>, packed: bool) -> ThreadSafePtr<LLVMValueRef> {
//         self.ir_codegen_impl.create_struct(values, context, packed)
//     }

//     fn create_global_variable(&self, module: ThreadSafePtr<LLVMModuleRef>, initializer: ThreadSafePtr<LLVMValueRef>, name: &str) -> ThreadSafePtr<LLVMValueRef> {
//         self.ir_codegen_impl.create_global_variable(module, initializer, name)
//     }

//     fn create_string(&self, val: &str, builder: ThreadSafePtr<LLVMBuilderRef>) -> ThreadSafePtr<LLVMValueRef> {
//         self.ir_codegen_impl.create_string(val, builder)
//     }

//     fn create_mut_string(&self, val: &str, context: ThreadSafePtr<LLVMContextRef>, builder: ThreadSafePtr<LLVMBuilderRef>) -> ThreadSafePtr<LLVMValueRef> {
//         self.ir_codegen_impl.create_mut_string(val, context, builder)
//     }

//     fn create_null_pointer(&self, ty: ThreadSafePtr<LLVMTypeRef>) -> ThreadSafePtr<LLVMValueRef> {
//         self.ir_codegen_impl.create_null_pointer(ty)
//     }

//     fn create_continue_statement(&self, builder: ThreadSafePtr<LLVMBuilderRef>, continue_block: ThreadSafePtr<LLVMBasicBlockRef>) -> ThreadSafePtr<LLVMValueRef> {
//         self.ir_codegen_impl.create_continue_statement(builder, continue_block)
//     }

//     fn create_break_statement(&self, builder: ThreadSafePtr<LLVMBuilderRef>, break_block: ThreadSafePtr<LLVMBasicBlockRef>) -> ThreadSafePtr<LLVMValueRef> {
//         self.ir_codegen_impl.create_break_statement(builder, break_block)
//     }


//     /// --- VARIABLES --- ///
//     fn init_var(&self, builder: ThreadSafePtr<LLVMBuilderRef>, var_name: &str, data_type: ThreadSafePtr<LLVMTypeRef>, initial_value: Option<ThreadSafePtr<LLVMValueRef>>) -> ThreadSafePtr<LLVMValueRef> {
//         self.ir_codegen_impl.init_var(builder, var_name, data_type, initial_value)
//     }

//     fn reassign_var(&self, builder: ThreadSafePtr<LLVMBuilderRef>, variable_alloc: ThreadSafePtr<LLVMValueRef>, new_value: ThreadSafePtr<LLVMValueRef>) {
//         self.ir_codegen_impl.reassign_var(builder, variable_alloc, new_value)
//     }

//     fn get_var(&self, builder: ThreadSafePtr<LLVMBuilderRef>, variable_type: ThreadSafePtr<LLVMTypeRef>, variable_alloc: ThreadSafePtr<LLVMValueRef>) -> ThreadSafePtr<LLVMValueRef> {
//         self.ir_codegen_impl.get_var(builder, variable_type, variable_alloc)
//     }
// }
