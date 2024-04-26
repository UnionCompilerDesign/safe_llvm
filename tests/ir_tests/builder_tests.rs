use safe_llvm::memory_management::resource_pools::ResourcePools;

#[test]
fn test_builder_creation() {
    let mut llvm_resource_pool = ResourcePools::new();
    let context_tag = llvm_resource_pool.allocate_context()
        .expect("Failed to create context");

    let _builder_tag = llvm_resource_pool.allocate_builder(context_tag)
        .expect("Builder creation failed");
}

#[test]
fn test_create_function_no_params_void_return() {
    let mut llvm_resource_pool = ResourcePools::new();
    let context_tag = llvm_resource_pool.allocate_context()
        .expect("Failed to create context");

    let _function_tag = llvm_resource_pool.allocate_function(None, &[], false, context_tag)
        .expect("Failed to create function with no parameters and void return");
}

#[test]
fn test_create_function_with_params() {
    // let mut llvm_resource_pool = ResourcePools::new();
    // let context_tag = llvm_resource_pool.allocate_context()
    //     .expect("Failed to create context");

    // // let int_type_tag = llvm_resource_pool.create_integer_type(32, context_tag)
    // //     .expect("Failed to create integer type");  

    // let param_tags: Vec<TypeTag> = vec![int_type_tag];
    // let function_tag = llvm_resource_pool.allocate_function(Some(int_type_tag), &param_tags, false, context_tag)
    //     .expect("Failed to create function with parameters");
}
