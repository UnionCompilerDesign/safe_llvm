
use super::*;
use mockall::mock;

mock! {
    LLVMApi {}
    impl LLVMApi for LLVMApi {
        fn append_basic_block_in_context(&self, context: LLVMContextRef, function: LLVMValueRef, name: &str) -> LLVMBasicBlockRef;
    }
}

#[test]
fn test_create_basic_block() {
    let mut mock_api = MockLLVMApi::new();
    mock_api.expect_append_basic_block_in_context()
            .returning(|_, _, _| std::ptr::null_mut()); 

    let context = std::ptr::null_mut();
    let function = std::ptr::null_mut();
    let bb = create_basic_block(&mock_api, context, function, "test");
    assert!(bb.is_null(), "Expected null pointer return for basic block");
}
