use safe_llvm::ir::init;

#[test]
fn test_context_and_module_creation() {
    let llvm_resource_pool = init::create_llvm_resource_pool();
    for _ in 0..100 { 
        let context = init::create_context(&llvm_resource_pool);
        if let Some(context_handle) = context {
            let module = init::create_module(&llvm_resource_pool, "loop_test_module", context_handle);
            assert!(
                module.is_some(),
                "Module creation failed in the loop"
            );
        } else {
            panic!("Failed to create context");
        }
    }
}
