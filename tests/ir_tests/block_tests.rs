use safe_llvm::memory_management::resource_pools::ResourcePools;

#[test]
fn test_create_basic_block() {
    let mut resource_pools = ResourcePools::new();

    let context_tag =  resource_pools.create_context().expect("Failed to create context");
    let module_tag =  resource_pools.create_module("dummy_module", context_tag).expect("Failed to create module within context");
    let function_type = resource_pools.int_type(context_tag, 32).expect("Failed to create function type");
    let function_value = resource_pools.create_function(Some(function_type), &[], false, context_tag).expect("Failed to create function prototype");
    let function_tag = resource_pools.add_function_to_module(module_tag, "test_func", function_value).expect("Failed to add function to module");
    let block_name = "test_block";

    let block_tag = resource_pools.create_basic_block(context_tag, function_tag, block_name);

    assert!(block_tag.is_some(), "Basic block should be created successfully");
}


#[test]
fn test_get_current_block() {
    let mut resource_pools = ResourcePools::new();

    let context_tag =  resource_pools.create_context().expect("Failed to create context");
    let builder_tag = resource_pools.create_builder(context_tag).expect("Failed to create builder");
    let block_tag = resource_pools.get_current_block(builder_tag);

    assert!(block_tag.is_none(), "Should return None when no block has been inserted yet");
}

#[test]
fn test_create_cond_br() {
    let mut resource_pools = ResourcePools::new();

    let context_tag =  resource_pools.create_context().expect("Failed to create context");
    let module_tag =  resource_pools.create_module("dummy_module", context_tag).expect("Failed to create module within context");
    let function_type = resource_pools.void_type(context_tag).expect("Failed to create function type");
    let function_value = resource_pools.create_function(Some(function_type), &[], false, context_tag).expect("Failed to create function prototype");
    let function_tag = resource_pools.add_function_to_module(module_tag, "test_func", function_value).expect("Failed to add function to module");
    let builder_tag = resource_pools.create_builder(context_tag).expect("Failed to create builder");
    let condition_value = resource_pools.create_integer(context_tag, 1).expect("Failed to create condition");
    let then_bb_tag = resource_pools.create_basic_block(context_tag, function_tag, "then").expect("Failed to create 'then' block");
    let else_bb_tag = resource_pools.create_basic_block(context_tag, function_tag, "else").expect("Failed to create 'else' block");
    let cond_branch_tag = resource_pools.create_cond_br(builder_tag, condition_value, then_bb_tag, else_bb_tag);

    assert!(cond_branch_tag.is_some(), "Conditional branch should be created successfully");
}

#[test]
fn test_create_br() {
    let mut resource_pools = ResourcePools::new();

    let context_tag =  resource_pools.create_context().expect("Failed to create context");
    let module_tag =  resource_pools.create_module("dummy_module", context_tag).expect("Failed to create module within context");
    let function_type = resource_pools.void_type(context_tag).expect("Failed to create function type");
    let function_value = resource_pools.create_function(Some(function_type), &[], false, context_tag).expect("Failed to create function prototype");
    let function_tag = resource_pools.add_function_to_module(module_tag, "test_func", function_value).expect("Failed to add function to module");
    let builder_tag = resource_pools.create_builder(context_tag).expect("Failed to create builder");
    let target_bb_tag = resource_pools.create_basic_block(context_tag, function_tag, "target").expect("Failed to create target block");
    let branch_tag = resource_pools.create_br(builder_tag, target_bb_tag);

    assert!(branch_tag.is_some(), "Unconditional branch should be created successfully");
}

#[test]
fn test_insert_before_basic_block() {
    let mut resource_pools = ResourcePools::new();

    let context_tag =  resource_pools.create_context().expect("Failed to create context");
    let module_tag =  resource_pools.create_module("dummy_module", context_tag).expect("Failed to create module within context");    let function_type = resource_pools.void_type(context_tag).expect("Failed to create function type");
    let function_value = resource_pools.create_function(Some(function_type), &[], false, context_tag).expect("Failed to create function prototype");
    let function_tag = resource_pools.add_function_to_module(module_tag, "test_func", function_value).expect("Failed to add function to module");
    let _builder_tag = resource_pools.create_builder(context_tag).expect("Failed to create builder");
    let target_bb_tag = resource_pools.create_basic_block(context_tag, function_tag, "target").expect("Failed to create target block");
    let insert_before_tag = resource_pools.insert_before_basic_block(context_tag, target_bb_tag, "insert_before");

    assert!(insert_before_tag.is_some(), "Insert before block should be created successfully");
}

#[test]
fn test_position_builder() {
    let mut resource_pools = ResourcePools::new();

    let context_tag =  resource_pools.create_context().expect("Failed to create context");
    let module_tag =  resource_pools.create_module("dummy_module", context_tag).expect("Failed to create module within context");    let function_type = resource_pools.void_type(context_tag).expect("Failed to create function type");
    let function_value = resource_pools.create_function(Some(function_type), &[], false, context_tag).expect("Failed to create function prototype");
    let function_tag = resource_pools.add_function_to_module(module_tag, "test_func", function_value).expect("Failed to add function to module");
    let builder_tag = resource_pools.create_builder(context_tag).expect("Failed to create builder");
    let bb_tag = resource_pools.create_basic_block(context_tag, function_tag, "position_here").expect("Failed to create block");
    let result = resource_pools.position_builder(builder_tag, bb_tag);

    assert!(result.is_some(), "Builder should be positioned at the end of the block successfully");
}

#[test]
fn test_delete_basic_block() {
    let mut resource_pools = ResourcePools::new();

    let context_tag =  resource_pools.create_context().expect("Failed to create context");
    let module_tag =  resource_pools.create_module("dummy_module", context_tag).expect("Failed to create module within context");    let function_type = resource_pools.void_type(context_tag).expect("Failed to create function type");
    let function_value = resource_pools.create_function(Some(function_type), &[], false, context_tag).expect("Failed to create function prototype");
    let function_tag = resource_pools.add_function_to_module(module_tag, "test_func", function_value).expect("Failed to add function to module");
    let _builder_tag = resource_pools.create_builder(context_tag).expect("Failed to create builder");
    let bb_tag = resource_pools.create_basic_block(context_tag, function_tag, "to_delete").expect("Failed to create block to delete");
    let result = resource_pools.delete_basic_block(bb_tag);

    assert!(result.is_some(), "Basic block should be deleted successfully");
}

#[test]
fn test_get_first_instruction() {
    let mut resource_pools = ResourcePools::new();

    let context_tag =  resource_pools.create_context().expect("Failed to create context");
    let module_tag =  resource_pools.create_module("dummy_module", context_tag).expect("Failed to create module within context");    let function_type = resource_pools.void_type(context_tag).expect("Failed to create function type");
    let function_value = resource_pools.create_function(Some(function_type), &[], false, context_tag).expect("Failed to create function prototype");
    let function_tag = resource_pools.add_function_to_module(module_tag, "test_func", function_value).expect("Failed to add function to module");
    let _builder_tag = resource_pools.create_builder(context_tag).expect("Failed to create builder");
    let bb_tag = resource_pools.create_basic_block(context_tag, function_tag, "has_instruction").expect("Failed to create block");
    let instruction_tag = resource_pools.get_first_instruction(bb_tag);

    assert!(instruction_tag.is_none(), "Should retrieve nothing");
}

#[test]
fn test_get_last_instruction() {
    let mut resource_pools = ResourcePools::new();

    let context_tag =  resource_pools.create_context().expect("Failed to create context");
    let module_tag =  resource_pools.create_module("dummy_module", context_tag).expect("Failed to create module within context");    let function_type = resource_pools.void_type(context_tag).expect("Failed to create function type");
    let function_value = resource_pools.create_function(Some(function_type), &[], false, context_tag).expect("Failed to create function prototype");
    let function_tag = resource_pools.add_function_to_module(module_tag, "test_func", function_value).expect("Failed to add function to module");
    let _builder_tag = resource_pools.create_builder(context_tag).expect("Failed to create builder");
    let bb_tag = resource_pools.create_basic_block(context_tag, function_tag, "has_instruction").expect("Failed to create block");
    let instruction_tag = resource_pools.get_last_instruction(bb_tag);

    assert!(instruction_tag.is_none(), "Should retrieve no instruction");
}
