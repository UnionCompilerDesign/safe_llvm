extern crate llvm_sys as llvm;

use llvm::prelude::{LLVMBasicBlockRef, LLVMBuilderRef, LLVMContextRef, LLVMValueRef};

pub trait LLVMApi {
    /// --- IR_CODEGEN : BLOCK --- ///
    fn create_basic_block(&self, context: LLVMContextRef, function: LLVMValueRef, name: &str) -> LLVMBasicBlockRef;
    fn get_current_block(&self, builder: *mut llvm::LLVMBuilder) -> LLVMBasicBlockRef;
    fn create_cond_br(&self, builder: LLVMBuilderRef, condition: LLVMValueRef, then_bb: LLVMBasicBlockRef, else_bb: LLVMBasicBlockRef) -> LLVMValueRef;
    fn create_br(&self, builder: LLVMBuilderRef, target_bb: LLVMBasicBlockRef) -> LLVMValueRef;
    fn insert_before_basic_block(&self, context: LLVMContextRef, before_target: LLVMBasicBlockRef, name: &str) -> LLVMBasicBlockRef;
    fn position_builder(&self, builder: *mut llvm::LLVMBuilder, bb: *mut llvm::LLVMBasicBlock);
    fn delete_basic_block(&self, bb: LLVMBasicBlockRef);
    fn get_first_instruction(&self, bb: LLVMBasicBlockRef) -> LLVMValueRef;
    fn get_last_instruction(&self, bb: LLVMBasicBlockRef) -> LLVMValueRef;   
    fn create_phi(&self, builder: LLVMBuilderRef, possible_values: &[(LLVMValueRef, LLVMBasicBlockRef)], name: &str) -> LLVMValueRef; 
}