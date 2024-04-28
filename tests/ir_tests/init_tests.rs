use safe_llvm::memory_management::resource_pools::ResourcePools;

#[test]
fn test_context_and_module_creation() {
    let mut llvm_resource_pool = ResourcePools::new();

    for _ in 0..100 {
        let context_tag = llvm_resource_pool.create_context()
            .expect("Failed to create context");

        let _module_tag = llvm_resource_pool.create_module("loop_test_module", context_tag)
            .expect("Module creation failed in the loop");
    }
}