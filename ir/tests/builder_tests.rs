use safe_llvm::{
    analysis::validator::Validator, common, constants::{DEFAULT_BASIC_BLOCK_NAME, DEFAULT_FUNCTION_NAME}, ir::core::{IRGenerator, TypeTag}
};

#[test]
fn test_builder_creation() {
    let mut llvm_resource_pool = IRGenerator::new();
    let context_tag = llvm_resource_pool.create_context()
        .expect("Failed to create context");

    let _builder_tag = llvm_resource_pool.create_builder(context_tag)
        .expect("Builder creation failed");
}

#[test]
fn test_create_function_no_params_void_return() {
    let mut llvm_resource_pool = IRGenerator::new();

    let context_tag = llvm_resource_pool.create_context().expect("Failed to create context");
    let module_tag = llvm_resource_pool.create_module("test_module", context_tag).expect("Failed to create module");
    let function_type = llvm_resource_pool.int_type(context_tag, 64).expect("Failed to create function type");
    let function_value = llvm_resource_pool.create_function(Some(function_type), &[], false, context_tag).expect("Failed to create function prototype");
    let function_tag = llvm_resource_pool.add_function_to_module(module_tag, DEFAULT_FUNCTION_NAME, function_value).expect("Failed to add function to module");
    let builder_tag = llvm_resource_pool.create_builder(context_tag).expect("Failed to create builder");
    let entry_bb_tag = llvm_resource_pool.create_basic_block(context_tag, function_tag, DEFAULT_BASIC_BLOCK_NAME).expect("Failed to create entry block");
    let return_val = llvm_resource_pool.create_integer(context_tag, 64).expect("Failed to create value");
    
    llvm_resource_pool.position_builder(builder_tag, entry_bb_tag);
    llvm_resource_pool.nonvoid_return(builder_tag, return_val);

    let module = llvm_resource_pool.get_module(module_tag).expect("Failed to get module");

    match common::write_ir::write_to_file(module.clone(), "test_create_function_no_params_void_return") {
        Ok(_) => {}
        Err(e) => {
            eprintln!("File write error: {}", e);
            panic!();
        }
    }

    let validator = Validator::new(module);
    assert!(validator.is_valid_module(), "Invalid module");

    let function = llvm_resource_pool.get_value(function_tag).expect("Failed to get function");
    assert!(validator.is_valid_function(function), "Invalid function");
}
#[test]
fn test_create_function_with_params() {
    let mut llvm_resource_pool = IRGenerator::new();
    let context_tag = llvm_resource_pool.create_context().expect("Failed to create context");
    let module_tag = llvm_resource_pool.create_module("test_module_with_params", context_tag).expect("Failed to create module");
    let int_type_tag = llvm_resource_pool.int_type(context_tag, 32).expect("Failed to create integer type");
    let param_tags: Vec<TypeTag> = vec![int_type_tag];
    let function_type = llvm_resource_pool.int_type(context_tag, 64).expect("Failed to create function type");
    let function_value = llvm_resource_pool.create_function(Some(function_type), &param_tags, false, context_tag).expect("Failed to create function prototype");
    let function_tag = llvm_resource_pool.add_function_to_module(module_tag, DEFAULT_FUNCTION_NAME, function_value).expect("Failed to add function to module");
    let builder_tag = llvm_resource_pool.create_builder(context_tag).expect("Failed to create builder");
    let entry_bb_tag = llvm_resource_pool.create_basic_block(context_tag, function_tag, DEFAULT_BASIC_BLOCK_NAME).expect("Failed to create entry block");
    let return_val = llvm_resource_pool.create_integer(context_tag, 32).expect("Failed to create return value");

    llvm_resource_pool.position_builder(builder_tag, entry_bb_tag);
    llvm_resource_pool.nonvoid_return(builder_tag, return_val);

    let module = llvm_resource_pool.get_module(module_tag).expect("Failed to get module");

    match common::write_ir::write_to_file(module.clone(), "test_create_function_with_params") {
        Ok(_) => {}
        Err(e) => {
            eprintln!("File write error: {}", e);
            panic!();
        }
    }

    let validator = Validator::new(module);
    assert!(validator.is_valid_module(), "Invalid module");

    let function = llvm_resource_pool.get_value(function_tag).expect("Failed to get function");
    assert!(validator.is_valid_function(function), "Invalid function");
}
