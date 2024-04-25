use safe_llvm::memory_management::resource_pools::ResourcePools;

#[test]
fn test_context_and_module_creation() {
    let mut llvm_resource_pool = ResourcePools::new();

    for _ in 0..100 {
        let context_handle = llvm_resource_pool.allocate_context()
            .expect("Failed to create context");

        let _module_handle = llvm_resource_pool.allocate_module("loop_test_module", context_handle)
            .expect("Module creation failed in the loop");
    }
}