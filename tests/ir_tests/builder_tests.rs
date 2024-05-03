use safe_llvm::memory_management::resource_pools::{ResourcePools, TypeTag};

#[test]
fn test_builder_creation() {
    let mut llvm_resource_pool = ResourcePools::new();
    let context_tag = llvm_resource_pool.create_context()
        .expect("Failed to create context");

    let _builder_tag = llvm_resource_pool.create_builder(context_tag)
        .expect("Builder creation failed");
}

#[test]
fn test_create_function_no_params_void_return() {
    let mut llvm_resource_pool = ResourcePools::new();
    let context_tag = llvm_resource_pool.create_context()
        .expect("Failed to create context");

    let _function_tag = llvm_resource_pool.create_function(None, &[], false, context_tag)
        .expect("Failed to create function with no parameters and void return");
}

#[test]
fn test_create_function_with_params() {
    let mut llvm_resource_pool = ResourcePools::new();
    let context_tag = llvm_resource_pool.create_context()
        .expect("Failed to create context");

    let int_type_tag = llvm_resource_pool.int_type(context_tag, 32)
        .expect("Failed to create integer type");  

    let param_tags: Vec<TypeTag> = vec![int_type_tag];
    let _function_tag = llvm_resource_pool.create_function(Some(int_type_tag), &param_tags, false, context_tag)
        .expect("Failed to create function with parameters");
}
