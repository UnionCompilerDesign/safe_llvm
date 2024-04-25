use safe_llvm::memory_management::resource_pools::ResourcePools;

#[test]
fn test_builder_creation() {
    let mut llvm_resource_pool = ResourcePools::new();
    let context_handle = llvm_resource_pool.allocate_context()
        .expect("Failed to create context");

    let _builder_handle = llvm_resource_pool.allocate_builder(context_handle)
        .expect("Builder creation failed");
}

#[test]
fn test_create_function_no_params_void_return() {
    let mut llvm_resource_pool = ResourcePools::new();
    let context_handle = llvm_resource_pool.allocate_context()
        .expect("Failed to create context");

    let _function_handle = llvm_resource_pool.allocate_function(None, &[], false, context_handle)
        .expect("Failed to create function with no parameters and void return");
}

#[test]
fn test_create_function_with_params() {
    // let mut llvm_resource_pool = ResourcePools::new();
    // let context_handle = llvm_resource_pool.allocate_context()
    //     .expect("Failed to create context");

    // // let int_type_handle = llvm_resource_pool.create_integer_type(32, context_handle)
    // //     .expect("Failed to create integer type");  

    // let param_handles: Vec<TypeHandle> = vec![int_type_handle];
    // let function_handle = llvm_resource_pool.allocate_function(Some(int_type_handle), &param_handles, false, context_handle)
    //     .expect("Failed to create function with parameters");
}
