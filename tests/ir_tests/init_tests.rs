use safe_llvm::ir::init;

#[test]
fn test_repeated_context_and_module_creation() {
    for _ in 0..100 { 
        let context = init::create_context();
        let module = init::create_module("loop_test_module", context);
        // assert!(!module.get_ref().is_null(), "Module pointer should not be null in loop");
    }
}