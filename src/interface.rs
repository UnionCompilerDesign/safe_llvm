extern crate llvm_sys as llvm;

use llvm::prelude::{LLVMBasicBlockRef, LLVMBuilderRef, LLVMContextRef, LLVMModuleRef, LLVMTypeRef, LLVMValueRef};

use crate::memory_management::pointer::CPointer;

pub trait LLVMApi {
    /// --- IR CODE GENERATION SECTION --- ///
    /// --- BLOCK --- ///
    fn create_basic_block(&self, context: CPointer<LLVMContextRef>, function: CPointer<LLVMValueRef>, name: &str) -> CPointer<LLVMBasicBlockRef>;
    fn get_current_block(&self, builder: CPointer<LLVMBuilderRef>) -> CPointer<LLVMBasicBlockRef>;
    fn create_cond_br(&self, builder: CPointer<LLVMBuilderRef>, condition: CPointer<LLVMValueRef>, then_bb: CPointer<LLVMBasicBlockRef>, else_bb: CPointer<LLVMBasicBlockRef>) -> CPointer<LLVMValueRef>;
    fn create_br(&self, builder: CPointer<LLVMBuilderRef>, target_bb: CPointer<LLVMBasicBlockRef>) -> CPointer<LLVMValueRef>;
    fn insert_before_basic_block(&self, context: CPointer<LLVMContextRef>, before_target: CPointer<LLVMBasicBlockRef>, name: &str) -> CPointer<LLVMBasicBlockRef>;
    fn position_builder(&self, builder: CPointer<LLVMBuilderRef>, bb: CPointer<LLVMBasicBlockRef>);
    fn delete_basic_block(&self, bb: CPointer<LLVMBasicBlockRef>);
    fn get_first_instruction(&self, bb: CPointer<LLVMBasicBlockRef>) -> CPointer<LLVMValueRef>;
    fn get_last_instruction(&self, bb: CPointer<LLVMBasicBlockRef>) -> CPointer<LLVMValueRef>;   
    fn create_phi(&self, builder: CPointer<LLVMBuilderRef>, possible_values: &[(CPointer<LLVMValueRef>, CPointer<LLVMBasicBlockRef>)], name: &str) -> CPointer<LLVMValueRef>; 
    
    /// --- BUILDER --- ///
    fn create_builder(&self, context: CPointer<LLVMContextRef>) -> CPointer<LLVMBuilderRef>;

    /// --- OPERATIONS --- ///
    fn build_add(&self, builder: CPointer<LLVMBuilderRef>, param_a: CPointer<LLVMValueRef>, param_b: CPointer<LLVMValueRef>, sum: &str) -> CPointer<LLVMValueRef>;
    fn build_sub(&self, builder: CPointer<LLVMBuilderRef>, param_a: CPointer<LLVMValueRef>, param_b: CPointer<LLVMValueRef>, name: &str) -> CPointer<LLVMValueRef>;
    fn build_mul(&self, builder: CPointer<LLVMBuilderRef>, param_a: CPointer<LLVMValueRef>, param_b: CPointer<LLVMValueRef>, name: &str) -> CPointer<LLVMValueRef>;
    fn build_div(&self, builder: CPointer<LLVMBuilderRef>, param_a: CPointer<LLVMValueRef>, param_b: CPointer<LLVMValueRef>, name: &str) -> CPointer<LLVMValueRef>;
    fn build_rem(&self, builder: CPointer<LLVMBuilderRef>, param_a: CPointer<LLVMValueRef>, param_b: CPointer<LLVMValueRef>, name: &str) -> CPointer<LLVMValueRef>;
    fn build_and(&self, builder: CPointer<LLVMBuilderRef>, param_a: CPointer<LLVMValueRef>, param_b: CPointer<LLVMValueRef>, name: &str) -> CPointer<LLVMValueRef>;
    fn build_or(&self, builder: CPointer<LLVMBuilderRef>, param_a: CPointer<LLVMValueRef>, param_b: CPointer<LLVMValueRef>, name: &str) -> CPointer<LLVMValueRef>;
    fn build_xor(&self, builder: CPointer<LLVMBuilderRef>, param_a: CPointer<LLVMValueRef>, param_b: CPointer<LLVMValueRef>, name: &str) -> CPointer<LLVMValueRef>;
    fn build_shl(&self, builder: CPointer<LLVMBuilderRef>, param_a: CPointer<LLVMValueRef>, param_b: CPointer<LLVMValueRef>, name: &str) -> CPointer<LLVMValueRef>;
    fn build_shr(&self, builder: CPointer<LLVMBuilderRef>, param_a: CPointer<LLVMValueRef>, param_b: CPointer<LLVMValueRef>, name: &str) -> CPointer<LLVMValueRef>;
    fn build_icmp_gt(&self, builder: CPointer<LLVMBuilderRef>, param_a: CPointer<LLVMValueRef>, param_b: CPointer<LLVMValueRef>, name: &str) -> CPointer<LLVMValueRef>;
    fn build_icmp_lt(&self, builder: CPointer<LLVMBuilderRef>, param_a: CPointer<LLVMValueRef>, param_b: CPointer<LLVMValueRef>, name: &str) -> CPointer<LLVMValueRef>;
    fn build_icmp_eq(&self, builder: CPointer<LLVMBuilderRef>, param_a: CPointer<LLVMValueRef>, param_b: CPointer<LLVMValueRef>, name: &str) -> CPointer<LLVMValueRef>;
    fn build_negation(&self, builder: CPointer<LLVMBuilderRef>, operand_ir: CPointer<LLVMValueRef>, name: &str) -> CPointer<LLVMValueRef>;
    fn build_bitwise_not(&self, builder: CPointer<LLVMBuilderRef>, operand_ir: CPointer<LLVMValueRef>, name: &str) -> CPointer<LLVMValueRef>;
    fn build_logical_not(&self, builder: CPointer<LLVMBuilderRef>, context: CPointer<LLVMContextRef>, operand_ir: CPointer<LLVMValueRef>, name: &str) -> CPointer<LLVMValueRef>;

    /// --- TOP LEVEL EXPRESSIONS --- ///
    fn get_param(&self, function: CPointer<LLVMValueRef>, index: u32) -> CPointer<LLVMValueRef>;
    fn add_function_to_module(&self, module: CPointer<LLVMModuleRef>, function_name: &str, function_type: CPointer<LLVMTypeRef>) -> CPointer<LLVMValueRef>;

    /// --- TYPES --- ///    
    fn void_type(&self, context: CPointer<LLVMContextRef>) -> CPointer<LLVMTypeRef>;
    fn int_type(&self, context: CPointer<LLVMContextRef>) -> CPointer<LLVMTypeRef>;
    fn float_type(&self, context: CPointer<LLVMContextRef>) -> CPointer<LLVMTypeRef>;
    fn boolean_type(&self, context: CPointer<LLVMContextRef>) -> CPointer<LLVMTypeRef>;
    fn pointer_type(&self, element_type: CPointer<LLVMTypeRef>) -> CPointer<LLVMTypeRef>;
    fn array_type(&self, element_type: CPointer<LLVMTypeRef>, num_elements: u64) -> CPointer<LLVMTypeRef>;
    fn struct_type(&self, context: CPointer<LLVMContextRef>, element_types: &[CPointer<LLVMTypeRef>], packed: bool) -> CPointer<LLVMTypeRef>;
    fn void_return(&self, builder: CPointer<LLVMBuilderRef>) -> CPointer<LLVMValueRef>;
    fn nonvoid_return(&self, builder: CPointer<LLVMBuilderRef>, value: CPointer<LLVMValueRef>) -> CPointer<LLVMValueRef>;

    /// --- VALUES --- ///
    fn create_integer(&self, val: i64, context: CPointer<LLVMContextRef>) -> CPointer<LLVMValueRef>;
    fn create_float(&self, val: f64, context: CPointer<LLVMContextRef>) -> CPointer<LLVMValueRef>;
    fn create_boolean(&self, val: bool, context: CPointer<LLVMContextRef>) -> CPointer<LLVMValueRef>;
    fn create_function(&self, return_type: Option<CPointer<LLVMTypeRef>>, param_types: &[CPointer<LLVMTypeRef>], is_var_arg: bool, context: CPointer<LLVMContextRef>) -> Option<CPointer<LLVMTypeRef>>;
    fn create_array(&self, value: CPointer<LLVMValueRef>, num_elements: u64) -> CPointer<LLVMValueRef>;
    fn create_pointer(&self, value: CPointer<LLVMTypeRef>) -> CPointer<LLVMValueRef>;
    fn create_struct(&self, values: &[CPointer<LLVMValueRef>], context: CPointer<LLVMContextRef>, packed: bool) -> CPointer<LLVMValueRef>;
    fn create_global_variable(&self, module: CPointer<LLVMModuleRef>, initializer: CPointer<LLVMValueRef>, name: &str) -> CPointer<LLVMValueRef>;
    fn create_string(&self, val: &str, builder: CPointer<LLVMBuilderRef>) -> CPointer<LLVMValueRef>;
    fn create_mut_string(&self, val: &str, context: CPointer<LLVMContextRef>, builder: CPointer<LLVMBuilderRef>) -> CPointer<LLVMValueRef>;
    fn create_null_pointer(&self, ty: CPointer<LLVMTypeRef>) -> CPointer<LLVMValueRef>;
    fn create_continue_statement(&self, builder: CPointer<LLVMBuilderRef>, continue_block: CPointer<LLVMBasicBlockRef>) -> CPointer<LLVMValueRef>;
    fn create_break_statement(&self, builder: CPointer<LLVMBuilderRef>, break_block: CPointer<LLVMBasicBlockRef>) -> CPointer<LLVMValueRef>;

    /// --- VARIABLES --- ///
    fn init_var(&self, builder: CPointer<LLVMBuilderRef>, var_name: &str, data_type: CPointer<LLVMTypeRef>, initial_value: Option<CPointer<LLVMValueRef>>) -> CPointer<LLVMValueRef>;
    fn reassign_var(&self, builder: CPointer<LLVMBuilderRef>, variable_alloc: CPointer<LLVMValueRef>, new_value: CPointer<LLVMValueRef>);
    fn get_var(&self, builder: CPointer<LLVMBuilderRef>, variable_type: CPointer<LLVMTypeRef>, variable_alloc: CPointer<LLVMValueRef>) -> CPointer<LLVMValueRef>;
}
