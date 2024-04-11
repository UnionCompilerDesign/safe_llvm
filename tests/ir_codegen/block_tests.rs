extern crate llvm_sys as llvm;

use llvm::prelude::{LLVMBasicBlockRef, LLVMBuilderRef, LLVMContextRef, LLVMValueRef};
use mockall::mock;

use safe_llvm::{api::api_definition::LLVMApi, memory_management::ir_pointer::IRPointer};

mock! {
    LLVMApi {}
    impl LLVMApi for LLVMApi {
        fn create_basic_block(&self, context: LLVMContextRef, function: LLVMValueRef, name: &str) -> IRPointer<LLVMBasicBlockRef>;
            fn get_current_block(&self, builder: *mut llvm::LLVMBuilder) -> IRPointer<LLVMBasicBlockRef>;
            fn create_cond_br(&self, builder: LLVMBuilderRef, condition: LLVMValueRef, then_bb: LLVMBasicBlockRef, else_bb: LLVMBasicBlockRef) -> IRPointer<LLVMValueRef>;
            fn create_br(&self, builder: LLVMBuilderRef, target_bb: LLVMBasicBlockRef) -> IRPointer<LLVMValueRef>;
            fn insert_before_basic_block(&self, context: LLVMContextRef, before_target: LLVMBasicBlockRef, name: &str) -> IRPointer<LLVMBasicBlockRef>;
            fn position_builder(&self, builder: *mut llvm::LLVMBuilder, bb: *mut llvm::LLVMBasicBlock);
            fn delete_basic_block(&self, bb: LLVMBasicBlockRef);
            fn get_first_instruction(&self, bb: LLVMBasicBlockRef) -> IRPointer<LLVMValueRef>;
            fn get_last_instruction(&self, bb: LLVMBasicBlockRef) -> IRPointer<LLVMValueRef>;   
            fn create_phi(&self, builder: LLVMBuilderRef, possible_values: &[(LLVMValueRef, LLVMBasicBlockRef)], name: &str) -> IRPointer<LLVMValueRef>; 
    }
}

#[test]
fn test_create_basic_block() {
    let mut mock_api = MockLLVMApi::new();
    let raw_ptr = IRPointer::new(Some(std::ptr::null_mut()));

    mock_api.expect_create_basic_block()
            .returning(|_, _, _| raw_ptr); 

    let context = std::ptr::null_mut();
    let function = std::ptr::null_mut();
    let bb = (&mock_api).create_basic_block(context, function, "test");
    assert!(bb.is_null(), "Expected null pointer return for basic block");
}
