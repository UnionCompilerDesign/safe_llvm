use safe_llvm::ir::init;

#[test]
fn test_repeated_context_and_module_creation() {
    for _ in 0..100 { 
        let context = init::create_context();
        if let Some(context_handle) = context {
            let module = init::create_module("loop_test_module", context_handle);
            assert!(
                module.is_some(),
                "Module creation failed in the loop"
            );
        } else {
            panic!("Failed to create context");
        }
    }
}
