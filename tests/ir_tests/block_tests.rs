use safe_llvm::{
    analysis::validator::Validator, 
    constants::{DEFAULT_BASIC_BLOCK_NAME, DEFAULT_FUNCTION_NAME, DEFAULT_MODULE_NAME}, 
    memory_management::resource_pools::ResourcePools, 
    utils::utils_struct::Utils
};

#[test]
fn test_create_basic_block() {
    let mut resource_pools = ResourcePools::new();

    let context_tag = resource_pools.create_context().expect("Failed to create context");
    let module_tag = resource_pools.create_module(DEFAULT_MODULE_NAME, context_tag).expect("Failed to create module within context");
    let function_type = resource_pools.int_type(context_tag, 64).expect("Failed to create function type");
    let function_value = resource_pools.create_function(Some(function_type), &[], false, context_tag).expect("Failed to create function prototype");
    let function_tag = resource_pools.add_function_to_module(module_tag, DEFAULT_FUNCTION_NAME, function_value).expect("Failed to add function to module");
    let builder_tag = resource_pools.create_builder(context_tag).expect("Failed to make builder");
    let block_tag = resource_pools.create_basic_block(context_tag, function_tag, DEFAULT_BASIC_BLOCK_NAME).expect("Failed to create block");
    let return_val = resource_pools.create_integer(context_tag, 32).expect("Failed to create return value");

    resource_pools.position_builder(builder_tag, block_tag).expect("Failed to position builder at end of block");
    resource_pools.nonvoid_return(builder_tag, return_val).expect("Failed to create branch");

    let module = resource_pools.get_module(module_tag).expect("Failed to get module");

    match Utils::write_to_file(module.clone(), "test_basic_block") {
        Ok(_) => {}
        Err(e) => {
            eprintln!("File write error: {}", e);
            panic!();        
        }
    }

    let validator = Validator::new(module);
    assert!(validator.is_valid_module(), "Invalid module");

    let function = resource_pools.get_value(function_tag).expect("Failed to get function");
    assert!(validator.is_valid_function(function), "Invalid function");
}



#[test]
fn test_get_current_block() {
    let mut resource_pools = ResourcePools::new();

    let context_tag = resource_pools.create_context().expect("Failed to create context");
    let module_tag = resource_pools.create_module(DEFAULT_MODULE_NAME, context_tag).expect("Failed to create module within context");
    let function_type = resource_pools.void_type(context_tag).expect("Failed to create function type");
    let function_value = resource_pools.create_function(Some(function_type), &[], false, context_tag).expect("Failed to create function prototype");
    let function_tag = resource_pools.add_function_to_module(module_tag, DEFAULT_FUNCTION_NAME, function_value).expect("Failed to add function to module");
    let builder_tag = resource_pools.create_builder(context_tag).expect("Failed to create builder");
    let bb_tag = resource_pools.create_basic_block(context_tag, function_tag, DEFAULT_BASIC_BLOCK_NAME).expect("Failed to create block");

    resource_pools.position_builder(builder_tag, bb_tag);
    resource_pools.void_return(builder_tag);  
    resource_pools.get_current_block(builder_tag).expect("Failed to get current block");

    let module = resource_pools.get_module(module_tag).expect("Failed to get module");

    match Utils::write_to_file(module.clone(), "test_get_current_block") {
        Ok(_) => {}
        Err(e) => {
            eprintln!("File write error: {}", e);
            panic!();        
        }
    }

    let validator = Validator::new(module);

    assert!(validator.is_valid_module(), "Invalid module");
    let function = resource_pools.get_value(function_tag).expect("Failed to get function");
    assert!(validator.is_valid_function(function), "Invalid function");
}

#[test]
fn test_create_cond_br() {
    let mut resource_pools = ResourcePools::new();

    let context_tag =  resource_pools.create_context().expect("Failed to create context");
    let module_tag =  resource_pools.create_module(DEFAULT_MODULE_NAME, context_tag).expect("Failed to create module within context");
    let function_type = resource_pools.void_type(context_tag).expect("Failed to create function type");
    let function_value = resource_pools.create_function(Some(function_type), &[], false, context_tag).expect("Failed to create function prototype");
    let function_tag = resource_pools.add_function_to_module(module_tag, DEFAULT_FUNCTION_NAME, function_value).expect("Failed to add function to module");
    let builder_tag = resource_pools.create_builder(context_tag).expect("Failed to create builder");
    let entry_bb_tag = resource_pools.create_basic_block(context_tag, function_tag, DEFAULT_BASIC_BLOCK_NAME).expect("Failed to create 'entry' block");
    let then_bb_tag = resource_pools.create_basic_block(context_tag, function_tag, "then").expect("Failed to create 'then' block");
    let else_bb_tag = resource_pools.create_basic_block(context_tag, function_tag, "else").expect("Failed to create 'else' block");
    let condition_value = resource_pools.create_boolean(context_tag, true).expect("Failed to create condition");

    resource_pools.position_builder(builder_tag, entry_bb_tag);
    resource_pools.create_cond_br(builder_tag, condition_value, then_bb_tag, else_bb_tag).expect("Failed to create condition");
    resource_pools.position_builder(builder_tag, then_bb_tag);
    resource_pools.void_return(builder_tag).expect("Failed to create return1");
    resource_pools.position_builder(builder_tag, else_bb_tag);
    resource_pools.void_return(builder_tag).expect("Failed to create return2");

    let module = resource_pools.get_module(module_tag).expect("Failed to get module");

    match Utils::write_to_file(module.clone(), "test_create_cond_br") {
        Ok(_) => {}
        Err(e) => {
            eprintln!("File write error: {}", e);
            panic!();        
        }
    }

    let validator = Validator::new(module);
    assert!(validator.is_valid_module(), "Invalid module");

    let function = resource_pools.get_value(function_tag).expect("Failed to get function");
    assert!(validator.is_valid_function(function), "Invalid function");
}

#[test]
fn test_create_br() {
    let mut resource_pools = ResourcePools::new();

    let context_tag =  resource_pools.create_context().expect("Failed to create context");
    let module_tag =  resource_pools.create_module(DEFAULT_MODULE_NAME, context_tag).expect("Failed to create module within context");
    let function_type = resource_pools.void_type(context_tag).expect("Failed to create function type");
    let function_value = resource_pools.create_function(Some(function_type), &[], false, context_tag).expect("Failed to create function prototype");
    let function_tag = resource_pools.add_function_to_module(module_tag, DEFAULT_FUNCTION_NAME, function_value).expect("Failed to add function to module");
    let builder_tag = resource_pools.create_builder(context_tag).expect("Failed to create builder");
    let entry_bb_tag = resource_pools.create_basic_block(context_tag, function_tag, DEFAULT_BASIC_BLOCK_NAME).expect("Failed to create entry block");
    let target_bb_tag = resource_pools.create_basic_block(context_tag, function_tag, "target").expect("Failed to create target block");

    resource_pools.position_builder(builder_tag, entry_bb_tag);
    resource_pools.create_br(builder_tag, target_bb_tag);
    resource_pools.position_builder(builder_tag, target_bb_tag);
    resource_pools.void_return(builder_tag);

    let module = resource_pools.get_module(module_tag).expect("Failed to get module");

    match Utils::write_to_file(module.clone(), "test_create_br") {
        Ok(_) => {}
        Err(e) => {
            eprintln!("File write error: {}", e);
            panic!();        
        }
    }

    let validator = Validator::new(module);

    if !validator.is_valid_module() {
        panic!("Invalid module")
    }

    let function = resource_pools.get_value(function_tag).expect("Failed to get function");
    if !validator.is_valid_function(function) {
        panic!("Invalid function")
    }
}

#[test]
fn test_insert_before_basic_block() {
    let mut resource_pools = ResourcePools::new();

    let context_tag =  resource_pools.create_context().expect("Failed to create context");
    let module_tag =  resource_pools.create_module(DEFAULT_MODULE_NAME, context_tag).expect("Failed to create module within context");    let function_type = resource_pools.void_type(context_tag).expect("Failed to create function type");
    let function_value = resource_pools.create_function(Some(function_type), &[], false, context_tag).expect("Failed to create function prototype");
    let function_tag = resource_pools.add_function_to_module(module_tag, DEFAULT_FUNCTION_NAME, function_value).expect("Failed to add function to module");
    let target_bb_tag = resource_pools.create_basic_block(context_tag, function_tag, "target").expect("Failed to create target block");
    let entry_bb_tag = resource_pools.insert_before_basic_block(context_tag, target_bb_tag, DEFAULT_BASIC_BLOCK_NAME).expect("Failed to insert entry");
    let builder_tag = resource_pools.create_builder(context_tag).expect("Failed to create builder");

    resource_pools.position_builder(builder_tag, entry_bb_tag);
    resource_pools.create_br(builder_tag, target_bb_tag);
    resource_pools.position_builder(builder_tag, target_bb_tag);
    resource_pools.void_return(builder_tag);
    
    let module = resource_pools.get_module(module_tag).expect("Failed to get module");

    match Utils::write_to_file(module.clone(), "test_insert_before_basic_block") {
        Ok(_) => {}
        Err(e) => {
            eprintln!("File write error: {}", e);
            panic!();        
        }
    }

    let validator = Validator::new(module);

    if !validator.is_valid_module() {
        panic!("Invalid module")
    }

    let function = resource_pools.get_value(function_tag).expect("Failed to get function");
    if !validator.is_valid_function(function) {
        panic!("Invalid function")
    }
}

#[test]
fn test_position_builder() {
    let mut resource_pools = ResourcePools::new();

    let context_tag = resource_pools.create_context().expect("Failed to create context");
    let module_tag = resource_pools.create_module(DEFAULT_MODULE_NAME, context_tag).expect("Failed to create module within context");
    let function_type = resource_pools.void_type(context_tag).expect("Failed to create function type");
    let function_value = resource_pools.create_function(Some(function_type), &[], false, context_tag).expect("Failed to create function prototype");
    let function_tag = resource_pools.add_function_to_module(module_tag, DEFAULT_FUNCTION_NAME, function_value).expect("Failed to add function to module");
    let builder_tag = resource_pools.create_builder(context_tag).expect("Failed to create builder");
    let bb_tag = resource_pools.create_basic_block(context_tag, function_tag, "position_here").expect("Failed to create block");

    resource_pools.position_builder(builder_tag, bb_tag);
    resource_pools.void_return(builder_tag);  

    let result = resource_pools.position_builder(builder_tag, bb_tag);

    assert!(result.is_some(), "Builder should be positioned at the end of the block successfully");

    let module = resource_pools.get_module(module_tag).expect("Failed to get module");

    match Utils::write_to_file(module.clone(), "test_position_builder") {
        Ok(_) => {}
        Err(e) => {
            eprintln!("File write error: {}", e);
            panic!();        
        }
    }

    let validator = Validator::new(module);

    assert!(validator.is_valid_module(), "Invalid module");
    let function = resource_pools.get_value(function_tag).expect("Failed to get function");
    assert!(validator.is_valid_function(function), "Invalid function");
}


#[test]
fn test_delete_basic_block() {
    let mut resource_pools = ResourcePools::new();

    let context_tag = resource_pools.create_context().expect("Failed to create context");
    let module_tag = resource_pools.create_module(DEFAULT_MODULE_NAME, context_tag).expect("Failed to create module within context");
    let function_type = resource_pools.void_type(context_tag).expect("Failed to create function type");
    let function_value = resource_pools.create_function(Some(function_type), &[], false, context_tag).expect("Failed to create function prototype");
    let function_tag = resource_pools.add_function_to_module(module_tag, DEFAULT_FUNCTION_NAME, function_value).expect("Failed to add function to module");
    let builder_tag = resource_pools.create_builder(context_tag).expect("Failed to create builder");
    let bb_tag = resource_pools.create_basic_block(context_tag, function_tag, "to_delete").expect("Failed to create block to delete");

    resource_pools.position_builder(builder_tag, bb_tag);
    resource_pools.void_return(builder_tag);  

    let result = resource_pools.delete_basic_block(bb_tag);

    let module = resource_pools.get_module(module_tag).expect("Failed to get module");

    match Utils::write_to_file(module.clone(), "test_delete_basic_block") {
        Ok(_) => {}
        Err(e) => {
            eprintln!("File write error: {}", e);
            panic!();        
        }
    }

    let validator = Validator::new(module);

    assert!(validator.is_valid_module(), "Invalid module after deleting block");
    assert!(result.is_some(), "Basic block should be deleted successfully");

    let function = resource_pools.get_value(function_tag).expect("Failed to get function");
    assert!(validator.is_valid_function(function), "Invalid function after deleting block");
}


#[test]
fn test_get_first_instruction() {
    let mut resource_pools = ResourcePools::new();

    let context_tag = resource_pools.create_context().expect("Failed to create context");
    let module_tag = resource_pools.create_module(DEFAULT_MODULE_NAME, context_tag).expect("Failed to create module within context");
    let function_type = resource_pools.void_type(context_tag).expect("Failed to create function type");
    let function_value = resource_pools.create_function(Some(function_type), &[], false, context_tag).expect("Failed to create function prototype");
    let function_tag = resource_pools.add_function_to_module(module_tag, DEFAULT_FUNCTION_NAME, function_value).expect("Failed to add function to module");
    let builder_tag = resource_pools.create_builder(context_tag).expect("Failed to create builder");
    let bb_tag = resource_pools.create_basic_block(context_tag, function_tag, DEFAULT_BASIC_BLOCK_NAME).expect("Failed to create block");

    resource_pools.position_builder(builder_tag, bb_tag);
    resource_pools.void_return(builder_tag);

    let instruction_tag = resource_pools.get_first_instruction(bb_tag);

    let module = resource_pools.get_module(module_tag).expect("Failed to get module");

    match Utils::write_to_file(module.clone(), "test_get_first_instruction") {
        Ok(_) => {}
        Err(e) => {
            eprintln!("File write error: {}", e);
            panic!();        
        }
    }

    let validator = Validator::new(module);

    assert!(validator.is_valid_module(), "Invalid module");
    assert!(instruction_tag.is_some(), "Should retrieve the first instruction (void return)");

    let function = resource_pools.get_value(function_tag).expect("Failed to get function");
    assert!(validator.is_valid_function(function), "Invalid function");
}

#[test]
fn test_get_last_instruction() {
    let mut resource_pools = ResourcePools::new();

    let context_tag = resource_pools.create_context().expect("Failed to create context");
    let module_tag = resource_pools.create_module(DEFAULT_MODULE_NAME, context_tag).expect("Failed to create module within context");
    let function_type = resource_pools.void_type(context_tag).expect("Failed to create function type");
    let function_value = resource_pools.create_function(Some(function_type), &[], false, context_tag).expect("Failed to create function prototype");
    let function_tag = resource_pools.add_function_to_module(module_tag, DEFAULT_FUNCTION_NAME, function_value).expect("Failed to add function to module");
    let builder_tag = resource_pools.create_builder(context_tag).expect("Failed to create builder");
    let bb_tag = resource_pools.create_basic_block(context_tag, function_tag, "has_instruction").expect("Failed to create block");

    resource_pools.position_builder(builder_tag, bb_tag);
    resource_pools.void_return(builder_tag);

    let instruction_tag = resource_pools.get_last_instruction(bb_tag);

    let module = resource_pools.get_module(module_tag).expect("Failed to get module");

    match Utils::write_to_file(module.clone(), "test_get_last_instruction") {
        Ok(_) => {}
        Err(e) => {
            eprintln!("File write error: {}", e);
            panic!();        
        }
    }

    let validator = Validator::new(module);

    assert!(validator.is_valid_module(), "Invalid module");
    assert!(instruction_tag.is_some(), "Should retrieve the last instruction (void return)");

    let function = resource_pools.get_value(function_tag).expect("Failed to get function");
    assert!(validator.is_valid_function(function), "Invalid function");
}
