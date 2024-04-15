extern crate llvm_sys as llvm;

use std::ffi::CString;

use llvm::prelude::{LLVMBasicBlockRef, LLVMBuilderRef, LLVMContextRef, LLVMModuleRef, LLVMTypeRef, LLVMValueRef};

use crate::memory_management::ir_pointer::IRPointer;

pub trait LLVMApi {
    /// --- IR CODE GENERATION SECTION --- ///
    /// --- BLOCK --- ///
    fn create_basic_block(&self, context: LLVMContextRef, function: LLVMValueRef, name: &str) -> IRPointer<LLVMBasicBlockRef>;
    fn get_current_block(&self, builder: LLVMBuilderRef) -> IRPointer<LLVMBasicBlockRef>;
    fn create_cond_br(&self, builder: LLVMBuilderRef, condition: LLVMValueRef, then_bb: LLVMBasicBlockRef, else_bb: LLVMBasicBlockRef) -> IRPointer<LLVMValueRef>;
    fn create_br(&self, builder: LLVMBuilderRef, target_bb: LLVMBasicBlockRef) -> IRPointer<LLVMValueRef>;
    fn insert_before_basic_block(&self, context: LLVMContextRef, before_target: LLVMBasicBlockRef, name: &str) -> IRPointer<LLVMBasicBlockRef>;
    fn position_builder(&self, builder: LLVMBuilderRef, bb: LLVMBasicBlockRef);
    fn delete_basic_block(&self, bb: LLVMBasicBlockRef);
    fn get_first_instruction(&self, bb: LLVMBasicBlockRef) -> IRPointer<LLVMValueRef>;
    fn get_last_instruction(&self, bb: LLVMBasicBlockRef) -> IRPointer<LLVMValueRef>;   
    fn create_phi(&self, builder: LLVMBuilderRef, possible_values: &[(LLVMValueRef, LLVMBasicBlockRef)], name: &str) -> IRPointer<LLVMValueRef>; 
    
    /// --- BUILDER --- ///
    fn create_builder(&self, context: LLVMContextRef) -> IRPointer<LLVMBuilderRef>;

    /// --- OPERATIONS --- ///
    fn build_add(&self, builder: LLVMBuilderRef, param_a: LLVMValueRef, param_b: LLVMValueRef, sum: CString);
    fn build_sub(&self, builder: LLVMBuilderRef, param_a: LLVMValueRef, param_b: LLVMValueRef, name: CString);
    fn build_mul(&self, builder: LLVMBuilderRef, param_a: LLVMValueRef, param_b: LLVMValueRef, name: CString);
    fn build_div(&self, builder: LLVMBuilderRef, param_a: LLVMValueRef, param_b: LLVMValueRef, name: CString);
    fn build_rem(&self, builder: LLVMBuilderRef, param_a: LLVMValueRef, param_b: LLVMValueRef, name: CString);
    fn build_and(&self, builder: LLVMBuilderRef, param_a: LLVMValueRef, param_b: LLVMValueRef, name: CString);
    fn build_or(&self, builder: LLVMBuilderRef, param_a: LLVMValueRef, param_b: LLVMValueRef, name: CString);
    fn build_xor(&self, builder: LLVMBuilderRef, param_a: LLVMValueRef, param_b: LLVMValueRef, name: CString);
    fn build_shl(&self, builder: LLVMBuilderRef, param_a: LLVMValueRef, param_b: LLVMValueRef, name: CString);
    fn build_shr(&self, builder: LLVMBuilderRef, param_a: LLVMValueRef, param_b: LLVMValueRef, name: CString);
    fn build_icmp_gt(&self, builder: LLVMBuilderRef, param_a: LLVMValueRef, param_b: LLVMValueRef, name: CString);
    fn build_icmp_lt(&self, builder: LLVMBuilderRef, param_a: LLVMValueRef, param_b: LLVMValueRef, name: CString);
    fn build_icmp_eq(&self, builder: LLVMBuilderRef, param_a: LLVMValueRef, param_b: LLVMValueRef, name: CString);
    fn build_negation(&self, builder: LLVMBuilderRef, operand_ir: LLVMValueRef, name: CString) -> IRPointer<LLVMValueRef>;
    fn build_bitwise_not(&self, builder: LLVMBuilderRef, operand_ir: LLVMValueRef, name: CString) -> IRPointer<LLVMValueRef>;
    fn build_logical_not(&self, builder: LLVMBuilderRef, context: LLVMContextRef, operand_ir: LLVMValueRef, name: CString) -> IRPointer<LLVMValueRef>;

    /// --- TOP LEVEL EXPRESSIONS --- ///
    fn get_param(&self, function: LLVMValueRef, index: u32) -> IRPointer<LLVMValueRef>;
    fn add_function_to_module(&self, module: LLVMModuleRef, function_name: &str, function_type: LLVMTypeRef) -> IRPointer<LLVMValueRef>;

    /// --- TYPES --- ///    
    fn void_type(&self, context: LLVMContextRef) -> IRPointer<LLVMTypeRef>;
    fn int_type(&self, context: LLVMContextRef) -> IRPointer<LLVMTypeRef>;
    fn float_type(&self, context: LLVMContextRef) -> IRPointer<LLVMTypeRef>;
    fn boolean_type(&self, context: LLVMContextRef) -> IRPointer<LLVMTypeRef>;
    fn pointer_type(&self, element_type: LLVMTypeRef) -> IRPointer<LLVMTypeRef>;
    fn array_type(&self, element_type: LLVMTypeRef, num_elements: u64) -> IRPointer<LLVMTypeRef>;
    fn struct_type(&self, context: LLVMContextRef, element_types: &[LLVMTypeRef], packed: bool) -> IRPointer<LLVMTypeRef>;
    fn void_return(&self, builder: LLVMBuilderRef) -> IRPointer<LLVMValueRef>;
    fn nonvoid_return(&self, builder: LLVMBuilderRef, value: LLVMValueRef) -> IRPointer<LLVMValueRef>;
    
    /// --- VALUES --- ///
    fn create_integer(&self, val: i64, context: LLVMContextRef) -> IRPointer<LLVMValueRef>;
    fn create_float(&self, val: f64, context: LLVMContextRef) -> IRPointer<LLVMValueRef>;
    fn create_boolean(&self, val: bool, context: LLVMContextRef) -> IRPointer<LLVMValueRef>;
    fn create_function(&self, name: &str, return_type: Option<LLVMTypeRef>, param_types: &[LLVMTypeRef], is_var_arg: bool, module: LLVMModuleRef) -> IRPointer<LLVMValueRef>;
    fn create_array(&self, value: LLVMValueRef, num_elements: u64) -> IRPointer<LLVMValueRef>;
    fn create_pointer(&self, value: LLVMValueRef) -> IRPointer<LLVMValueRef>;
    fn create_struct(&self, values: &[LLVMValueRef], context: LLVMContextRef, packed: bool) -> IRPointer<LLVMValueRef>;
    fn create_global_variable(&self, module: LLVMModuleRef, initializer: LLVMValueRef, name: &str) -> IRPointer<LLVMValueRef>;
    fn create_string(&self, val: &str, builder: LLVMBuilderRef) -> IRPointer<LLVMValueRef>;
    fn create_mut_string(&self, val: &str, context: LLVMContextRef, builder: LLVMBuilderRef) -> IRPointer<LLVMValueRef>;
    fn create_null_pointer(&self, ty: LLVMTypeRef) -> IRPointer<LLVMValueRef>;
    fn create_continue_statement(&self, builder: LLVMBuilderRef, continue_block: LLVMBasicBlockRef);
    fn create_break_statement(&self, builder: LLVMBuilderRef, break_block: LLVMBasicBlockRef);
    fn create_function_type(&self, return_type: LLVMTypeRef, param_types: &[LLVMTypeRef], is_var_arg: bool) -> IRPointer<LLVMTypeRef>;

    /// --- VARIABLES --- ///
    fn init_var(&self, builder: LLVMBuilderRef, var_name: &str, data_type: LLVMTypeRef, initial_value: Option<LLVMValueRef>) -> IRPointer<LLVMValueRef>;
    fn reassign_var(&self, builder: LLVMBuilderRef, variable_alloc: LLVMValueRef, new_value: LLVMValueRef);
    fn get_var(&self, builder: LLVMBuilderRef, variable_type: LLVMTypeRef, variable_alloc: LLVMValueRef) -> IRPointer<LLVMValueRef>;
}
