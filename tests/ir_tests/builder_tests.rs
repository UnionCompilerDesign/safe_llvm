use safe_llvm::{ir::{builder, init, values}, memory_management::resource_pools::Handle};

#[test]
fn test_builder_creation() {
    let pool = init::create_llvm_resource_pool();
    let context_handle = init::create_context(&pool).expect("Failed to create context");
    let builder_handle = builder::create_builder(&pool, context_handle);
    assert!(builder_handle.is_some(), "Builder handle should not be None");
    
}

#[test]
fn test_create_function_no_params_void_return() {
    let pool = init::create_llvm_resource_pool();
    let context_handle = init::create_context(&pool).expect("Failed to create context");
    let function_handle = builder::create_function(&pool, None, &[], false, context_handle);
    assert!(function_handle.is_some(), "Function handle should not be None");
}

#[test]
fn test_create_function_with_params() {
    let pool = init::create_llvm_resource_pool();
    let context_handle = init::create_context(&pool).expect("Failed to create context");
    let int_type_handle = values::create_integer(&pool, 3, context_handle).expect("Failed to get integer"); 

    let param_handles: Vec<Handle> = vec![int_type_handle];
    let function_handle = builder::create_function(&pool, Some(int_type_handle), &param_handles, false, context_handle);
    assert!(function_handle.is_some(), "Function handle with parameters should not be None");
}