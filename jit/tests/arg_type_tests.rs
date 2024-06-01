use common::{constants::{DEFAULT_BASIC_BLOCK_NAME, DEFAULT_FUNCTION_NAME, DEFAULT_MODULE_NAME}, target::*};
use ir::core::IRGenerator;
use jit::core::ExecutionEngine;

#[test]fn test_function_with_integer_argument() {
    let mut pools = IRGenerator::new();
    let context_tag = pools.create_context().expect("Failed to create context");
    let module_tag = pools.create_module(DEFAULT_MODULE_NAME, context_tag).expect("Failed to create module");
    let int_type = pools.int_type(context_tag, 32).expect("Failed to create int type");
    let function_value = pools.create_function(Some(int_type), &[int_type], false, context_tag).expect("Failed to create function prototype");
    let function_tag = pools.add_function_to_module(module_tag, DEFAULT_FUNCTION_NAME, function_value).expect("Failed to add function to module");
    let builder_tag = pools.create_builder(context_tag).expect("Failed to create builder");
    let bb_tag = pools.create_basic_block(context_tag, function_tag, DEFAULT_BASIC_BLOCK_NAME).expect("Failed to create entry block");
    let param = pools.get_param(function_tag, 0).expect("Failed to get parameter");
    pools.position_builder_at_end(builder_tag, bb_tag);
    pools.nonvoid_return(builder_tag, param);

    let module = pools.get_module(module_tag).expect("Failed to retrieve module");
    let mut engine = ExecutionEngine::new(module, true);
    engine.initialize_target(GeneralTargetConfigurator {}).expect("Failed to configure engine");

    let arg_value: i32 = 58;
    let result: Result<i32, String> = engine.execute(DEFAULT_FUNCTION_NAME, (arg_value,));
    assert_eq!(result.unwrap(), arg_value, "Function did not return the expected integer value");
}

#[test]
fn test_function_with_float_argument() {
    let mut pools = IRGenerator::new();
    let context_tag = pools.create_context().expect("Failed to create context");
    let module_tag = pools.create_module(DEFAULT_MODULE_NAME, context_tag).expect("Failed to create module");
    let float_type = pools.float_type(context_tag).expect("Failed to create float type");
    let function_value = pools.create_function(Some(float_type), &[float_type], false, context_tag).expect("Failed to create function prototype");
    let function_tag = pools.add_function_to_module(module_tag, DEFAULT_FUNCTION_NAME, function_value).expect("Failed to add function to module");
    let builder_tag = pools.create_builder(context_tag).expect("Failed to create builder");
    let bb_tag = pools.create_basic_block(context_tag, function_tag, DEFAULT_BASIC_BLOCK_NAME).expect("Failed to create entry block");
    let param = pools.get_param(function_tag, 0).expect("Failed to get parameter");
    pools.position_builder_at_end(builder_tag, bb_tag);
    pools.nonvoid_return(builder_tag, param);

    let module = pools.get_module(module_tag).expect("Failed to retrieve module");
    let mut engine = ExecutionEngine::new(module, true);
    engine.initialize_target(GeneralTargetConfigurator {}).expect("Failed to configure engine");

    let arg_value: f64 = 3.14159;
    let result: Result<f64, String> = engine.execute(DEFAULT_FUNCTION_NAME, (arg_value,));
    assert!((result.unwrap() - arg_value).abs() < f64::EPSILON, "Function did not return the expected float value");
}

#[test]
fn test_function_with_boolean_argument() {
    let mut pools = IRGenerator::new();
    let context_tag = pools.create_context().expect("Failed to create context");
    let module_tag = pools.create_module(DEFAULT_MODULE_NAME, context_tag). expect("Failed to create module");
    let bool_type = pools.boolean_type(context_tag).expect("Failed to create boolean type");
    let function_value = pools.create_function(Some(bool_type), &[bool_type], false, context_tag).expect("Failed to create function prototype");
    let function_tag = pools.add_function_to_module(module_tag, DEFAULT_FUNCTION_NAME, function_value).expect("Failed to add function to module");
    let builder_tag = pools.create_builder(context_tag).expect("Failed to create builder");
    let bb_tag = pools.create_basic_block(context_tag, function_tag, DEFAULT_BASIC_BLOCK_NAME).expect("Failed to create entry block");
    let param = pools.get_param(function_tag, 0).expect("Failed to get parameter");
    pools.position_builder_at_end(builder_tag, bb_tag);
    pools.nonvoid_return(builder_tag, param);

    let module = pools.get_module(module_tag).expect("Failed to retrieve module");
    let mut engine = ExecutionEngine::new(module, true);
    engine.initialize_target(GeneralTargetConfigurator {}).expect("Failed to configure engine");

    let arg_value: bool = true;
    let result: Result<bool, String> = engine.execute(DEFAULT_FUNCTION_NAME, (arg_value,));
    assert_eq!(result.unwrap(), arg_value, "Function did not return the expected boolean value");
}
