use safe_llvm::memory_management::resource_pools::ResourcePools;

#[test]
fn test_add_function_to_module() {
    let mut resource_pools = ResourcePools::new();
    
    let context_tag = resource_pools.create_context().expect("Failed to create context");
    let module_tag = resource_pools.create_module("test_module", context_tag).expect("Failed to create module");
    let void_type_tag = resource_pools.void_type(context_tag).expect("Failed to create void type");
    let function_tag = resource_pools.create_function(Some(void_type_tag), &[], false, context_tag).expect("Failed to create function");
    let _added_function_tag = resource_pools.add_function_to_module(module_tag, "function_name", function_tag).expect("Failed to add function to module");
}

#[test]
fn test_get_param() {
    let mut resource_pools = ResourcePools::new();

    let context_tag = resource_pools.create_context().expect("Failed to create context");
    let module_tag = resource_pools.create_module("test_module", context_tag).expect("Failed to create module");
    let int_type_tag = resource_pools.int_type(context_tag, 32).expect("Failed to create integer type");
    let function_tag = resource_pools.create_function(Some(int_type_tag), &[int_type_tag], false, context_tag).expect("Failed to create function with parameters");
    let added_function_tag = resource_pools.add_function_to_module(module_tag, "function_name", function_tag).expect("Failed to add function to module");
    let _param_tag = resource_pools.get_param(added_function_tag, 0).expect("Failed to get parameter");
}
