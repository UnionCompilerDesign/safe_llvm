use safe_llvm::{analysis::validator::Validator, memory_management::resource_pools::ResourcePools};

#[test]
fn test_context_and_module_creation() {
    let mut llvm_resource_pool = ResourcePools::new();

    for _ in 0..100 {
        let context_tag = llvm_resource_pool.create_context()
            .expect("Failed to create context");

        let module_tag = llvm_resource_pool.create_module("loop_test_module", context_tag)
            .expect("Module creation failed in the loop");

        let module = llvm_resource_pool.get_module(module_tag).expect("Failed to get module");

        let validator = Validator::new(module);
        if !validator.is_valid_module() {
            panic!("Invalid module")
        }
    }
}