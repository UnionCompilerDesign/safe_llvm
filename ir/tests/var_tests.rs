use ir::core::IRGenerator;

#[test]
fn test_init_var() {
    let mut resource_pools = IRGenerator::new();

    let context_tag = resource_pools.create_context().expect("Failed to create context");
    let module_tag = resource_pools.create_module("MyModule", context_tag).expect("Failed to create module");
    let builder_tag = resource_pools.create_builder(context_tag).expect("Failed to create builder");
    let int_type_tag = resource_pools.int_type(context_tag, 32).expect("Failed to create integer type");
    let initial_value_tag = resource_pools.create_integer(context_tag, 42).expect("Failed to create initial value");

    let function_type_tag = resource_pools.create_function(Some(int_type_tag), &[], false, context_tag).expect("Failed to create function type");
    let function_value_tag = resource_pools.add_function_to_module(module_tag, "testFunction", function_type_tag).expect("Failed to add function to module");

    let block_tag = resource_pools.create_basic_block(context_tag, function_value_tag, "entry").expect("Failed to create basic block");
    resource_pools.position_builder_at_end(builder_tag, block_tag).expect("Failed to position builder");

    let var_tag = resource_pools.init_var(builder_tag, "myVar", int_type_tag, Some(initial_value_tag));
    assert!(var_tag.is_some(), "Variable should be initialized successfully");
}


#[test]
fn test_reassign_var() {
    let mut resource_pools = IRGenerator::new();

    let context_tag = resource_pools.create_context().expect("Failed to create context");
    let module_tag = resource_pools.create_module("MyModule", context_tag).expect("Failed to create module");
    let builder_tag = resource_pools.create_builder(context_tag).expect("Failed to create builder");
    let int_type_tag = resource_pools.int_type(context_tag, 32).expect("Failed to create integer type");
    let initial_value_tag = resource_pools.create_integer(context_tag, 42).expect("Failed to create initial value");

    let function_type_tag = resource_pools.create_function(Some(int_type_tag), &[], false, context_tag).expect("Failed to create function type");
    let function_value_tag = resource_pools.add_function_to_module(module_tag, "testFunction", function_type_tag).expect("Failed to add function to module");
    let block_tag = resource_pools.create_basic_block(context_tag, function_value_tag, "entry").expect("Failed to create basic block");
    resource_pools.position_builder_at_end(builder_tag, block_tag).expect("Failed to position builder");

    let var_tag = resource_pools.init_var(builder_tag, "myVar", int_type_tag, Some(initial_value_tag)).expect("Variable should be initialized successfully");
    let new_value_tag = resource_pools.create_integer(context_tag, 100).expect("Failed to create new value");
    resource_pools.reassign_var(builder_tag, var_tag, new_value_tag).expect("Variable reassignment failed");

    assert!(resource_pools.get_var(builder_tag, int_type_tag, var_tag).is_some(), "Variable should be reassigned successfully");
}


#[test]
fn test_get_var() {
    let mut resource_pools = IRGenerator::new();

    let context_tag = resource_pools.create_context().expect("Failed to create context");
    let module_tag = resource_pools.create_module("MyModule", context_tag).expect("Failed to create module");
    let builder_tag = resource_pools.create_builder(context_tag).expect("Failed to create builder");
    let int_type_tag = resource_pools.int_type(context_tag, 32).expect("Failed to create integer type");
    let initial_value_tag = resource_pools.create_integer(context_tag, 42).expect("Failed to create initial value");

    let function_type_tag = resource_pools.create_function(Some(int_type_tag), &[], false, context_tag).expect("Failed to create function type");
    let function_value_tag = resource_pools.add_function_to_module(module_tag, "testFunction", function_type_tag).expect("Failed to add function to module");
    let block_tag = resource_pools.create_basic_block(context_tag, function_value_tag, "entry").expect("Failed to create basic block");
    resource_pools.position_builder_at_end(builder_tag, block_tag).expect("Failed to position builder");

    let var_tag = resource_pools.init_var(builder_tag, "myVar", int_type_tag, Some(initial_value_tag)).expect("Variable should be initialized successfully");
    let retrieved_var_tag = resource_pools.get_var(builder_tag, int_type_tag, var_tag);

    assert!(retrieved_var_tag.is_some(), "Variable should be retrieved successfully");
}
