// use llvm::prelude::{LLVMContextRef, LLVMValueRef, LLVMBasicBlockRef, LLVMBuilderRef, LLVMTypeRef, LLVMModuleRef};

// use crate::{
//     interface::LLVMApi,
//     ir::SafeLLVM as IrCodeGenSafeLLVM,
//     memory_management::pointer::CPointer,
// }; 

// pub struct SafeLLVM {
//     ir_codegen_impl: IrCodeGenSafeLLVM,
// }

// impl LLVMApi for SafeLLVM {
//     /// --- BLOCK --- ///
//     fn create_basic_block(&self, context: CPointer<LLVMContextRef>, function: CPointer<LLVMValueRef>, name: &str) -> CPointer<LLVMBasicBlockRef> {
//         self.ir_codegen_impl.create_basic_block(context, function, name)
//     }

//     fn get_current_block(&self, builder: CPointer<LLVMBuilderRef>) -> CPointer<LLVMBasicBlockRef> {
//         self.ir_codegen_impl.get_current_block(builder)
//     }

//     fn create_cond_br(&self, builder: CPointer<LLVMBuilderRef>, condition: CPointer<LLVMValueRef>, then_bb: CPointer<LLVMBasicBlockRef>, else_bb: CPointer<LLVMBasicBlockRef>) -> CPointer<LLVMValueRef> {
//         self.ir_codegen_impl.create_cond_br(builder, condition, then_bb, else_bb)
//     }

//     fn create_br(&self, builder: CPointer<LLVMBuilderRef>, target_bb: CPointer<LLVMBasicBlockRef>) -> CPointer<LLVMValueRef> {
//         self.ir_codegen_impl.create_br(builder, target_bb)
//     }

//     fn insert_before_basic_block(&self, context: CPointer<LLVMContextRef>, before_target: CPointer<LLVMBasicBlockRef>, name: &str) -> CPointer<LLVMBasicBlockRef> {
//         self.ir_codegen_impl.insert_before_basic_block(context, before_target, name)
//     }

//     fn position_builder(&self, builder: CPointer<LLVMBuilderRef>, bb: CPointer<LLVMBasicBlockRef>) {
//         self.ir_codegen_impl.position_builder(builder, bb)
//     }

//     fn delete_basic_block(&self, bb: CPointer<LLVMBasicBlockRef>) {
//         self.ir_codegen_impl.delete_basic_block(bb)
//     }

//     fn get_first_instruction(&self, bb: CPointer<LLVMBasicBlockRef>) -> CPointer<LLVMValueRef> {
//         self.ir_codegen_impl.get_first_instruction(bb)
//     }

//     fn get_last_instruction(&self, bb: CPointer<LLVMBasicBlockRef>) -> CPointer<LLVMValueRef> {
//         self.ir_codegen_impl.get_last_instruction(bb)
//     }

//     fn create_phi(&self, builder: CPointer<LLVMBuilderRef>, possible_values: &[(CPointer<LLVMValueRef>, CPointer<LLVMBasicBlockRef>)], name: &str) -> CPointer<LLVMValueRef> {
//         self.ir_codegen_impl.create_phi(builder, possible_values, name)
//     }

//     /// --- BUILDER --- ///
//     fn create_builder(&self, context: CPointer<LLVMContextRef>) -> CPointer<LLVMBuilderRef> {
//         self.ir_codegen_impl.create_builder(context)
//     }


//     /// --- OPS --- ///
//     fn build_add(&self, builder: CPointer<LLVMBuilderRef>, param_a: CPointer<LLVMValueRef>, param_b: CPointer<LLVMValueRef>, sum: &str) -> CPointer<LLVMValueRef> {
//         self.ir_codegen_impl.build_add(builder, param_a, param_b, sum)
//     }

//     fn build_sub(&self, builder: CPointer<LLVMBuilderRef>, param_a: CPointer<LLVMValueRef>, param_b: CPointer<LLVMValueRef>, name: &str) -> CPointer<LLVMValueRef> {
//         self.ir_codegen_impl.build_sub(builder, param_a, param_b, name)
//     }

//     fn build_mul(&self, builder: CPointer<LLVMBuilderRef>, param_a: CPointer<LLVMValueRef>, param_b: CPointer<LLVMValueRef>, name: &str) -> CPointer<LLVMValueRef> {
//         self.ir_codegen_impl.build_mul(builder, param_a, param_b, name)
//     }

//     fn build_div(&self, builder: CPointer<LLVMBuilderRef>, param_a: CPointer<LLVMValueRef>, param_b: CPointer<LLVMValueRef>, name: &str) -> CPointer<LLVMValueRef> {
//         self.ir_codegen_impl.build_div(builder, param_a, param_b, name)
//     }

//     fn build_rem(&self, builder: CPointer<LLVMBuilderRef>, param_a: CPointer<LLVMValueRef>, param_b: CPointer<LLVMValueRef>, name: &str) -> CPointer<LLVMValueRef> {
//         self.ir_codegen_impl.build_rem(builder, param_a, param_b, name)
//     }

//     fn build_and(&self, builder: CPointer<LLVMBuilderRef>, param_a: CPointer<LLVMValueRef>, param_b: CPointer<LLVMValueRef>, name: &str) -> CPointer<LLVMValueRef> {
//         self.ir_codegen_impl.build_and(builder, param_a, param_b, name)
//     }

//     fn build_or(&self, builder: CPointer<LLVMBuilderRef>, param_a: CPointer<LLVMValueRef>, param_b: CPointer<LLVMValueRef>, name: &str) -> CPointer<LLVMValueRef> {
//         self.ir_codegen_impl.build_or(builder, param_a, param_b, name)
//     }

//     fn build_xor(&self, builder: CPointer<LLVMBuilderRef>, param_a: CPointer<LLVMValueRef>, param_b: CPointer<LLVMValueRef>, name: &str) -> CPointer<LLVMValueRef> {
//         self.ir_codegen_impl.build_xor(builder, param_a, param_b, name)
//     }

//     fn build_shl(&self, builder: CPointer<LLVMBuilderRef>, param_a: CPointer<LLVMValueRef>, param_b: CPointer<LLVMValueRef>, name: &str) -> CPointer<LLVMValueRef> {
//         self.ir_codegen_impl.build_shl(builder, param_a, param_b, name)
//     }

//     fn build_shr(&self, builder: CPointer<LLVMBuilderRef>, param_a: CPointer<LLVMValueRef>, param_b: CPointer<LLVMValueRef>, name: &str) -> CPointer<LLVMValueRef> {
//         self.ir_codegen_impl.build_shr(builder, param_a, param_b, name)
//     }

//     fn build_icmp_gt(&self, builder: CPointer<LLVMBuilderRef>, param_a: CPointer<LLVMValueRef>, param_b: CPointer<LLVMValueRef>, name: &str) -> CPointer<LLVMValueRef> {
//         self.ir_codegen_impl.build_icmp_gt(builder, param_a, param_b, name)
//     }

//     fn build_icmp_lt(&self, builder: CPointer<LLVMBuilderRef>, param_a: CPointer<LLVMValueRef>, param_b: CPointer<LLVMValueRef>, name: &str) -> CPointer<LLVMValueRef> {
//         self.ir_codegen_impl.build_icmp_lt(builder, param_a, param_b, name)
//     }

//     fn build_icmp_eq(&self, builder: CPointer<LLVMBuilderRef>, param_a: CPointer<LLVMValueRef>, param_b: CPointer<LLVMValueRef>, name: &str) -> CPointer<LLVMValueRef> {
//         self.ir_codegen_impl.build_icmp_eq(builder, param_a, param_b, name)
//     }

//     fn build_negation(&self, builder: CPointer<LLVMBuilderRef>, operand_ir: CPointer<LLVMValueRef>, name: &str) -> CPointer<LLVMValueRef> {
//         self.ir_codegen_impl.build_negation(builder, operand_ir, name)
//     }

//     fn build_bitwise_not(&self, builder: CPointer<LLVMBuilderRef>, operand_ir: CPointer<LLVMValueRef>, name: &str) -> CPointer<LLVMValueRef> {
//         self.ir_codegen_impl.build_bitwise_not(builder, operand_ir, name)
//     }

//     fn build_logical_not(&self, builder: CPointer<LLVMBuilderRef>, context: CPointer<LLVMContextRef>, operand_ir: CPointer<LLVMValueRef>, name: &str) -> CPointer<LLVMValueRef> {
//         self.ir_codegen_impl.build_logical_not(builder, context, operand_ir, name)
//     }

//     fn nonvoid_return(&self, builder: CPointer<LLVMBuilderRef>, value: CPointer<LLVMValueRef>) -> CPointer<LLVMValueRef> {
//         self.ir_codegen_impl.nonvoid_return(builder, value)
//     }

//     fn void_return(&self, builder: CPointer<LLVMBuilderRef>) -> CPointer<LLVMValueRef> {
//         self.ir_codegen_impl.void_return(builder)
//     }

//     /// --- TOP LEVEL EXP --- ///
//     fn get_param(&self, function: CPointer<LLVMValueRef>, index: u32) -> CPointer<LLVMValueRef> {
//         self.ir_codegen_impl.get_param(function, index)
//     }

//     fn add_function_to_module(&self, module: CPointer<LLVMModuleRef>, function_name: &str, function_type: CPointer<LLVMTypeRef>) -> CPointer<LLVMValueRef> {
//         self.ir_codegen_impl.add_function_to_module(module, function_name, function_type)
//     }

//     /// --- TYPES --- ///
//     fn void_type(&self, context: CPointer<LLVMContextRef>) -> CPointer<LLVMTypeRef> {
//         self.ir_codegen_impl.void_type(context)
//     }

//     fn int_type(&self, context: CPointer<LLVMContextRef>) -> CPointer<LLVMTypeRef> {
//         self.ir_codegen_impl.int_type(context)
//     }

//     fn float_type(&self, context: CPointer<LLVMContextRef>) -> CPointer<LLVMTypeRef> {
//         self.ir_codegen_impl.float_type(context)
//     }

//     fn boolean_type(&self, context: CPointer<LLVMContextRef>) -> CPointer<LLVMTypeRef> {
//         self.ir_codegen_impl.boolean_type(context)
//     }

//     fn pointer_type(&self, element_type: CPointer<LLVMTypeRef>) -> CPointer<LLVMTypeRef> {
//         self.ir_codegen_impl.pointer_type(element_type)
//     }

//     fn array_type(&self, element_type: CPointer<LLVMTypeRef>, num_elements: u64) -> CPointer<LLVMTypeRef> {
//         self.ir_codegen_impl.array_type(element_type, num_elements)
//     }

//     fn struct_type(&self, context: CPointer<LLVMContextRef>, element_types: &[CPointer<LLVMTypeRef>], packed: bool) -> CPointer<LLVMTypeRef> {
//         self.ir_codegen_impl.struct_type(context, element_types, packed)
//     }

//     /// --- VALUES --- ///
//     fn create_integer(&self, val: i64, context: CPointer<LLVMContextRef>) -> CPointer<LLVMValueRef> {
//         self.ir_codegen_impl.create_integer(val, context)
//     }

//     fn create_float(&self, val: f64, context: CPointer<LLVMContextRef>) -> CPointer<LLVMValueRef> {
//         self.ir_codegen_impl.create_float(val, context)
//     }

//     fn create_boolean(&self, val: bool, context: CPointer<LLVMContextRef>) -> CPointer<LLVMValueRef> {
//         self.ir_codegen_impl.create_boolean(val, context)
//     }

//     fn create_function(&self, return_type: Option<CPointer<LLVMTypeRef>>, param_types: &[CPointer<LLVMTypeRef>], is_var_arg: bool, context: CPointer<LLVMContextRef>) -> Option<CPointer<LLVMTypeRef>> {
//         self.ir_codegen_impl.create_function(return_type, param_types, is_var_arg, context)
//     }

//     fn create_array(&self, value: CPointer<LLVMValueRef>, num_elements: u64) -> CPointer<LLVMValueRef> {
//         self.ir_codegen_impl.create_array(value, num_elements)
//     }

//     fn create_pointer(&self, value: CPointer<LLVMTypeRef>) -> CPointer<LLVMValueRef> {
//         self.ir_codegen_impl.create_pointer(value)
//     }

//     fn create_struct(&self, values: &[CPointer<LLVMValueRef>], context: CPointer<LLVMContextRef>, packed: bool) -> CPointer<LLVMValueRef> {
//         self.ir_codegen_impl.create_struct(values, context, packed)
//     }

//     fn create_global_variable(&self, module: CPointer<LLVMModuleRef>, initializer: CPointer<LLVMValueRef>, name: &str) -> CPointer<LLVMValueRef> {
//         self.ir_codegen_impl.create_global_variable(module, initializer, name)
//     }

//     fn create_string(&self, val: &str, builder: CPointer<LLVMBuilderRef>) -> CPointer<LLVMValueRef> {
//         self.ir_codegen_impl.create_string(val, builder)
//     }

//     fn create_mut_string(&self, val: &str, context: CPointer<LLVMContextRef>, builder: CPointer<LLVMBuilderRef>) -> CPointer<LLVMValueRef> {
//         self.ir_codegen_impl.create_mut_string(val, context, builder)
//     }

//     fn create_null_pointer(&self, ty: CPointer<LLVMTypeRef>) -> CPointer<LLVMValueRef> {
//         self.ir_codegen_impl.create_null_pointer(ty)
//     }

//     fn create_continue_statement(&self, builder: CPointer<LLVMBuilderRef>, continue_block: CPointer<LLVMBasicBlockRef>) -> CPointer<LLVMValueRef> {
//         self.ir_codegen_impl.create_continue_statement(builder, continue_block)
//     }

//     fn create_break_statement(&self, builder: CPointer<LLVMBuilderRef>, break_block: CPointer<LLVMBasicBlockRef>) -> CPointer<LLVMValueRef> {
//         self.ir_codegen_impl.create_break_statement(builder, break_block)
//     }


//     /// --- VARIABLES --- ///
//     fn init_var(&self, builder: CPointer<LLVMBuilderRef>, var_name: &str, data_type: CPointer<LLVMTypeRef>, initial_value: Option<CPointer<LLVMValueRef>>) -> CPointer<LLVMValueRef> {
//         self.ir_codegen_impl.init_var(builder, var_name, data_type, initial_value)
//     }

//     fn reassign_var(&self, builder: CPointer<LLVMBuilderRef>, variable_alloc: CPointer<LLVMValueRef>, new_value: CPointer<LLVMValueRef>) {
//         self.ir_codegen_impl.reassign_var(builder, variable_alloc, new_value)
//     }

//     fn get_var(&self, builder: CPointer<LLVMBuilderRef>, variable_type: CPointer<LLVMTypeRef>, variable_alloc: CPointer<LLVMValueRef>) -> CPointer<LLVMValueRef> {
//         self.ir_codegen_impl.get_var(builder, variable_type, variable_alloc)
//     }
// }
