// extern crate llvm_sys as llvm;

// use llvm::prelude::{LLVMBasicBlockRef, LLVMBuilderRef, LLVMContextRef, LLVMModuleRef, LLVMTypeRef, LLVMValueRef};

// use crate::memory_management::pointer::ThreadSafePtr;

// pub trait LLVMApi {
//     /// --- IR CODE GENERATION SECTION --- ///
//     /// --- BLOCK --- ///
//     fn create_basic_block(&self, context: ThreadSafePtr<LLVMContextRef>, function: ThreadSafePtr<LLVMValueRef>, name: &str) -> ThreadSafePtr<LLVMBasicBlockRef>;
//     fn get_current_block(&self, builder: ThreadSafePtr<LLVMBuilderRef>) -> ThreadSafePtr<LLVMBasicBlockRef>;
//     fn create_cond_br(&self, builder: ThreadSafePtr<LLVMBuilderRef>, condition: ThreadSafePtr<LLVMValueRef>, then_bb: ThreadSafePtr<LLVMBasicBlockRef>, else_bb: ThreadSafePtr<LLVMBasicBlockRef>) -> ThreadSafePtr<LLVMValueRef>;
//     fn create_br(&self, builder: ThreadSafePtr<LLVMBuilderRef>, target_bb: ThreadSafePtr<LLVMBasicBlockRef>) -> ThreadSafePtr<LLVMValueRef>;
//     fn insert_before_basic_block(&self, context: ThreadSafePtr<LLVMContextRef>, before_target: ThreadSafePtr<LLVMBasicBlockRef>, name: &str) -> ThreadSafePtr<LLVMBasicBlockRef>;
//     fn position_builder(&self, builder: ThreadSafePtr<LLVMBuilderRef>, bb: ThreadSafePtr<LLVMBasicBlockRef>);
//     fn delete_basic_block(&self, bb: ThreadSafePtr<LLVMBasicBlockRef>);
//     fn get_first_instruction(&self, bb: ThreadSafePtr<LLVMBasicBlockRef>) -> ThreadSafePtr<LLVMValueRef>;
//     fn get_last_instruction(&self, bb: ThreadSafePtr<LLVMBasicBlockRef>) -> ThreadSafePtr<LLVMValueRef>;   
//     fn create_phi(&self, builder: ThreadSafePtr<LLVMBuilderRef>, possible_values: &[(ThreadSafePtr<LLVMValueRef>, ThreadSafePtr<LLVMBasicBlockRef>)], name: &str) -> ThreadSafePtr<LLVMValueRef>; 
    
//     /// --- BUILDER --- ///
//     fn create_builder(&self, context: ThreadSafePtr<LLVMContextRef>) -> ThreadSafePtr<LLVMBuilderRef>;

//     /// --- OPERATIONS --- ///
//     fn build_add(&self, builder: ThreadSafePtr<LLVMBuilderRef>, param_a: ThreadSafePtr<LLVMValueRef>, param_b: ThreadSafePtr<LLVMValueRef>, sum: &str) -> ThreadSafePtr<LLVMValueRef>;
//     fn build_sub(&self, builder: ThreadSafePtr<LLVMBuilderRef>, param_a: ThreadSafePtr<LLVMValueRef>, param_b: ThreadSafePtr<LLVMValueRef>, name: &str) -> ThreadSafePtr<LLVMValueRef>;
//     fn build_mul(&self, builder: ThreadSafePtr<LLVMBuilderRef>, param_a: ThreadSafePtr<LLVMValueRef>, param_b: ThreadSafePtr<LLVMValueRef>, name: &str) -> ThreadSafePtr<LLVMValueRef>;
//     fn build_div(&self, builder: ThreadSafePtr<LLVMBuilderRef>, param_a: ThreadSafePtr<LLVMValueRef>, param_b: ThreadSafePtr<LLVMValueRef>, name: &str) -> ThreadSafePtr<LLVMValueRef>;
//     fn build_rem(&self, builder: ThreadSafePtr<LLVMBuilderRef>, param_a: ThreadSafePtr<LLVMValueRef>, param_b: ThreadSafePtr<LLVMValueRef>, name: &str) -> ThreadSafePtr<LLVMValueRef>;
//     fn build_and(&self, builder: ThreadSafePtr<LLVMBuilderRef>, param_a: ThreadSafePtr<LLVMValueRef>, param_b: ThreadSafePtr<LLVMValueRef>, name: &str) -> ThreadSafePtr<LLVMValueRef>;
//     fn build_or(&self, builder: ThreadSafePtr<LLVMBuilderRef>, param_a: ThreadSafePtr<LLVMValueRef>, param_b: ThreadSafePtr<LLVMValueRef>, name: &str) -> ThreadSafePtr<LLVMValueRef>;
//     fn build_xor(&self, builder: ThreadSafePtr<LLVMBuilderRef>, param_a: ThreadSafePtr<LLVMValueRef>, param_b: ThreadSafePtr<LLVMValueRef>, name: &str) -> ThreadSafePtr<LLVMValueRef>;
//     fn build_shl(&self, builder: ThreadSafePtr<LLVMBuilderRef>, param_a: ThreadSafePtr<LLVMValueRef>, param_b: ThreadSafePtr<LLVMValueRef>, name: &str) -> ThreadSafePtr<LLVMValueRef>;
//     fn build_shr(&self, builder: ThreadSafePtr<LLVMBuilderRef>, param_a: ThreadSafePtr<LLVMValueRef>, param_b: ThreadSafePtr<LLVMValueRef>, name: &str) -> ThreadSafePtr<LLVMValueRef>;
//     fn build_icmp_gt(&self, builder: ThreadSafePtr<LLVMBuilderRef>, param_a: ThreadSafePtr<LLVMValueRef>, param_b: ThreadSafePtr<LLVMValueRef>, name: &str) -> ThreadSafePtr<LLVMValueRef>;
//     fn build_icmp_lt(&self, builder: ThreadSafePtr<LLVMBuilderRef>, param_a: ThreadSafePtr<LLVMValueRef>, param_b: ThreadSafePtr<LLVMValueRef>, name: &str) -> ThreadSafePtr<LLVMValueRef>;
//     fn build_icmp_eq(&self, builder: ThreadSafePtr<LLVMBuilderRef>, param_a: ThreadSafePtr<LLVMValueRef>, param_b: ThreadSafePtr<LLVMValueRef>, name: &str) -> ThreadSafePtr<LLVMValueRef>;
//     fn build_negation(&self, builder: ThreadSafePtr<LLVMBuilderRef>, operand_ir: ThreadSafePtr<LLVMValueRef>, name: &str) -> ThreadSafePtr<LLVMValueRef>;
//     fn build_bitwise_not(&self, builder: ThreadSafePtr<LLVMBuilderRef>, operand_ir: ThreadSafePtr<LLVMValueRef>, name: &str) -> ThreadSafePtr<LLVMValueRef>;
//     fn build_logical_not(&self, builder: ThreadSafePtr<LLVMBuilderRef>, context: ThreadSafePtr<LLVMContextRef>, operand_ir: ThreadSafePtr<LLVMValueRef>, name: &str) -> ThreadSafePtr<LLVMValueRef>;

//     /// --- TOP LEVEL EXPRESSIONS --- ///
//     fn get_param(&self, function: ThreadSafePtr<LLVMValueRef>, index: u32) -> ThreadSafePtr<LLVMValueRef>;
//     fn add_function_to_module(&self, module: ThreadSafePtr<LLVMModuleRef>, function_name: &str, function_type: ThreadSafePtr<LLVMTypeRef>) -> ThreadSafePtr<LLVMValueRef>;

//     /// --- TYPES --- ///    
//     fn void_type(&self, context: ThreadSafePtr<LLVMContextRef>) -> ThreadSafePtr<LLVMTypeRef>;
//     fn int_type(&self, context: ThreadSafePtr<LLVMContextRef>) -> ThreadSafePtr<LLVMTypeRef>;
//     fn float_type(&self, context: ThreadSafePtr<LLVMContextRef>) -> ThreadSafePtr<LLVMTypeRef>;
//     fn boolean_type(&self, context: ThreadSafePtr<LLVMContextRef>) -> ThreadSafePtr<LLVMTypeRef>;
//     fn pointer_type(&self, element_type: ThreadSafePtr<LLVMTypeRef>) -> ThreadSafePtr<LLVMTypeRef>;
//     fn array_type(&self, element_type: ThreadSafePtr<LLVMTypeRef>, num_elements: u64) -> ThreadSafePtr<LLVMTypeRef>;
//     fn struct_type(&self, context: ThreadSafePtr<LLVMContextRef>, element_types: &[ThreadSafePtr<LLVMTypeRef>], packed: bool) -> ThreadSafePtr<LLVMTypeRef>;
//     fn void_return(&self, builder: ThreadSafePtr<LLVMBuilderRef>) -> ThreadSafePtr<LLVMValueRef>;
//     fn nonvoid_return(&self, builder: ThreadSafePtr<LLVMBuilderRef>, value: ThreadSafePtr<LLVMValueRef>) -> ThreadSafePtr<LLVMValueRef>;

//     /// --- VALUES --- ///
//     fn create_integer(&self, val: i64, context: ThreadSafePtr<LLVMContextRef>) -> ThreadSafePtr<LLVMValueRef>;
//     fn create_float(&self, val: f64, context: ThreadSafePtr<LLVMContextRef>) -> ThreadSafePtr<LLVMValueRef>;
//     fn create_boolean(&self, val: bool, context: ThreadSafePtr<LLVMContextRef>) -> ThreadSafePtr<LLVMValueRef>;
//     fn create_function(&self, return_type: Option<ThreadSafePtr<LLVMTypeRef>>, param_types: &[ThreadSafePtr<LLVMTypeRef>], is_var_arg: bool, context: ThreadSafePtr<LLVMContextRef>) -> Option<ThreadSafePtr<LLVMTypeRef>>;
//     fn create_array(&self, value: ThreadSafePtr<LLVMValueRef>, num_elements: u64) -> ThreadSafePtr<LLVMValueRef>;
//     fn create_pointer(&self, value: ThreadSafePtr<LLVMTypeRef>) -> ThreadSafePtr<LLVMValueRef>;
//     fn create_struct(&self, values: &[ThreadSafePtr<LLVMValueRef>], context: ThreadSafePtr<LLVMContextRef>, packed: bool) -> ThreadSafePtr<LLVMValueRef>;
//     fn create_global_variable(&self, module: ThreadSafePtr<LLVMModuleRef>, initializer: ThreadSafePtr<LLVMValueRef>, name: &str) -> ThreadSafePtr<LLVMValueRef>;
//     fn create_string(&self, val: &str, builder: ThreadSafePtr<LLVMBuilderRef>) -> ThreadSafePtr<LLVMValueRef>;
//     fn create_mut_string(&self, val: &str, context: ThreadSafePtr<LLVMContextRef>, builder: ThreadSafePtr<LLVMBuilderRef>) -> ThreadSafePtr<LLVMValueRef>;
//     fn create_null_pointer(&self, ty: ThreadSafePtr<LLVMTypeRef>) -> ThreadSafePtr<LLVMValueRef>;
//     fn create_continue_statement(&self, builder: ThreadSafePtr<LLVMBuilderRef>, continue_block: ThreadSafePtr<LLVMBasicBlockRef>) -> ThreadSafePtr<LLVMValueRef>;
//     fn create_break_statement(&self, builder: ThreadSafePtr<LLVMBuilderRef>, break_block: ThreadSafePtr<LLVMBasicBlockRef>) -> ThreadSafePtr<LLVMValueRef>;

//     /// --- VARIABLES --- ///
//     fn init_var(&self, builder: ThreadSafePtr<LLVMBuilderRef>, var_name: &str, data_type: ThreadSafePtr<LLVMTypeRef>, initial_value: Option<ThreadSafePtr<LLVMValueRef>>) -> ThreadSafePtr<LLVMValueRef>;
//     fn reassign_var(&self, builder: ThreadSafePtr<LLVMBuilderRef>, variable_alloc: ThreadSafePtr<LLVMValueRef>, new_value: ThreadSafePtr<LLVMValueRef>);
//     fn get_var(&self, builder: ThreadSafePtr<LLVMBuilderRef>, variable_type: ThreadSafePtr<LLVMTypeRef>, variable_alloc: ThreadSafePtr<LLVMValueRef>) -> ThreadSafePtr<LLVMValueRef>;
// }
