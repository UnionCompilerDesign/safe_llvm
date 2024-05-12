use safe_llvm::{
    analysis::validator::Validator, constants::DEFAULT_FUNCTION_NAME, jit::{execution_engine::ExecutionEngine, target::GeneralTargetConfigurator}, memory_management::resource_pools::ResourcePools, utils::utils_struct::Utils
};

#[test]
fn test_execution_engine_with_resource_pools_module() {
    let mut resource_pools = ResourcePools::new();

    let context_tag = resource_pools.create_context().expect("Failed to create context");
    let module_tag = resource_pools.create_module("test_module", context_tag).expect("Failed to create module");
    let function_type = resource_pools.void_type(context_tag).expect("Failed to create function type");
    let function_value = resource_pools.create_function(Some(function_type), &[], false, context_tag).expect("Failed to create function prototype");
    let function_tag = resource_pools.add_function_to_module(module_tag, DEFAULT_FUNCTION_NAME, function_value).expect("Failed to add function to module");
    let builder_tag = resource_pools.create_builder(context_tag).expect("Failed to create builder");
    let entry_bb_tag = resource_pools.create_basic_block(context_tag, function_tag, "entry").expect("Failed to create entry block");

    resource_pools.position_builder(builder_tag, entry_bb_tag);
    resource_pools.void_return(builder_tag);

    let module = resource_pools.get_module(module_tag).expect("Failed to retrieve module");
    
    match Utils::write_to_file(module.clone(), "test_delete_basic_block") {
        Ok(_) => {}
        Err(e) => {
            eprintln!("File write error: {}", e);
            panic!();        
        }
    }

    let validator = Validator::new(module.clone());
    assert!(validator.is_valid_module(), "Invalid module after deleting block");

    let function = resource_pools.get_value(function_tag).expect("Failed to get function");
    assert!(validator.is_valid_function(function), "Invalid function after deleting block");

    let mut engine = ExecutionEngine::new(true);
    engine.init_target(GeneralTargetConfigurator {}, false).expect("Failed to configure engine");
    engine.set_module(module.clone());
    engine.start_engine().expect("Failed to start the engine");

    let result = engine.execute(DEFAULT_FUNCTION_NAME);

    assert!(result.is_ok(), "Execution failed with error: {:?}", result.err());
    println!("Execution result: {:?}", result);
}

