use safe_llvm::memory_management::resource_pools::ResourcePools;

#[test]
fn test_init_var() {
    let mut resource_pools = ResourcePools::new();

    let context_tag = resource_pools.create_context().expect("Failed to create context");
    let builder_tag = resource_pools.create_builder(context_tag).expect("Failed to create builder");
    let int_type_tag = resource_pools.int_type(context_tag, 32).expect("Failed to create integer type");
    let initial_value_tag = resource_pools.create_integer(context_tag, 42).expect("Failed to create initial value");

    let var_tag = resource_pools.init_var(builder_tag, "myVar", int_type_tag, Some(initial_value_tag));
    assert!(var_tag.is_some(), "Variable should be initialized successfully");
}

// #[test]
// fn test_reassign_var() {
//     let mut resource_pools = ResourcePools::new();
//     let context_tag = resource_pools.create_context().expect("Failed to create context");
//     let builder_tag = resource_pools.create_builder(context_tag).expect("Failed to create builder");
//     let int_type_tag = resource_pools.int_type(context_tag, 32).expect("Failed to create integer type");
//     let initial_value_tag = resource_pools.create_integer(context_tag, 42).expect("Failed to create initial value");
//     let var_tag = resource_pools.init_var(builder_tag, "myVar", int_type_tag, Some(initial_value_tag)).expect("Failed to initialize variable");

//     let new_value_tag = resource_pools.create_integer(context_tag, 100).expect("Failed to create new value");
//     let reassign_result = resource_pools.reassign_var(builder_tag, var_tag, new_value_tag);
//     assert!(reassign_result.is_some(), "Variable should be reassigned successfully");
// }

// #[test]
// fn test_get_var() {
//     let mut resource_pools = ResourcePools::new();
//     let context_tag = resource_pools.create_context().expect("Failed to create context");
//     let builder_tag = resource_pools.create_builder(context_tag).expect("Failed to create builder");
//     let int_type_tag = resource_pools.int_type(context_tag, 32).expect("Failed to create integer type");
//     let initial_value_tag = resource_pools.create_integer(context_tag, 42).expect("Failed to create initial value");
//     let var_tag = resource_pools.init_var(builder_tag, "myVar", int_type_tag, Some(initial_value_tag)).expect("Failed to initialize variable");

//     let retrieved_var_tag = resource_pools.get_var(builder_tag, int_type_tag, var_tag);
//     assert!(retrieved_var_tag.is_some(), "Variable should be retrieved successfully");
// }