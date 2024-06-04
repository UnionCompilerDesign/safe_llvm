// use analysis::validator::Validator;
// use common::{constants::{DEFAULT_BASIC_BLOCK_NAME, DEFAULT_FUNCTION_NAME, DEFAULT_MODULE_NAME}, target::*};
// use ir::core::IRManager;
// use jit::core::ExecutionEngine;

// #[test]
// fn test_execute_int_function() {
//     for _ in 1..1000 {
//         let mut pools = IRManager::new();
//         let context_tag = pools.create_context().expect("Failed to create context");
//         let module_tag = pools.create_module(DEFAULT_MODULE_NAME, context_tag).expect("Failed to create module");
//         let int_type = pools.int_type(context_tag, 64).expect("Failed to create int type");
//         let function_value = pools.create_function(Some(int_type), &[], false, context_tag).expect("Failed to create function prototype");
//         let function_tag = pools.add_function_to_module(module_tag, DEFAULT_FUNCTION_NAME, function_value).expect("Failed to add function to module");
//         let builder_tag = pools.create_builder(context_tag).expect("Failed to create builder");
//         let bb_tag = pools.create_basic_block(context_tag, function_tag, DEFAULT_BASIC_BLOCK_NAME).expect("Failed to create entry block");
//         let return_value = pools.create_integer(context_tag, 42).expect("Failed to create return value");
        
//         pools.position_builder_at_end(builder_tag, bb_tag);
//         pools.nonvoid_return(builder_tag, return_value);

//         let module = pools.get_module(module_tag).expect("Failed to retrieve module");

//         let validator = Validator::new(module.clone());
//         assert!(validator.is_valid_module(), "Invalid module");

//         let mut engine = ExecutionEngine::new(module, true);
//         engine.initialize_target(GeneralTargetConfigurator {}).expect("Failed to configure engine");

//         let result: Result<i64, String> = engine.execute(DEFAULT_FUNCTION_NAME, ());
//         assert_eq!(result.unwrap(), 42, "Function did not return the expected integer value");
//     }
// }

// #[test]
// fn test_execute_float_function() {
//     for _ in 1..1000 {
//         let mut pools = IRManager::new();
//         let context_tag = pools.create_context().expect("Failed to create context");
//         let module_tag = pools.create_module(DEFAULT_MODULE_NAME, context_tag).expect("Failed to create module");
//         let float_type = pools.float_type(context_tag).expect("Failed to create float type");
//         let function_value = pools.create_function(Some(float_type), &[], false, context_tag).expect("Failed to create function prototype");
//         let function_tag = pools.add_function_to_module(module_tag, DEFAULT_FUNCTION_NAME, function_value).expect("Failed to add function to module");
//         let builder_tag = pools.create_builder(context_tag).expect("Failed to create builder");
//         let bb_tag = pools.create_basic_block(context_tag, function_tag, DEFAULT_BASIC_BLOCK_NAME).expect("Failed to create entry block");
//         let return_value = pools.create_float(context_tag, 3.14).expect("Failed to create return value");
        
//         pools.position_builder_at_end(builder_tag, bb_tag);
//         pools.nonvoid_return(builder_tag, return_value);

//         let module = pools.get_module(module_tag).expect("Failed to retrieve module");

//         let validator = Validator::new(module.clone());
        
//         let function = pools.get_value(function_tag).expect("Failed to get function");
//         assert!(validator.is_valid_function(function), "Invalid function");
//         assert!(validator.is_valid_module(), "Invalid module");

//         let mut engine = ExecutionEngine::new(module, true);
//         engine.initialize_target(GeneralTargetConfigurator {}).expect("Failed to configure engine");

//         let result: Result<f32, String> = engine.execute(DEFAULT_FUNCTION_NAME, ());
//         assert!((result.unwrap() - 3.14).abs() < f32::EPSILON, "Function did not return the expected float value");
//     }
// }

// #[test]
// fn test_execute_void_function() {
//     for _ in 1..1000 {
//         let mut pools = IRManager::new();
//         let context_tag = pools.create_context().expect("Failed to create context");
//         let module_tag = pools.create_module(DEFAULT_MODULE_NAME, context_tag).expect("Failed to create module");
//         let void_type = pools.void_type(context_tag).expect("Failed to create void type");
//         let function_value = pools.create_function(Some(void_type), &[], false, context_tag).expect("Failed to create function prototype");
//         let function_tag = pools.add_function_to_module(module_tag, DEFAULT_FUNCTION_NAME, function_value).expect("Failed to add function to module");
//         let builder_tag = pools.create_builder(context_tag).expect("Failed to create builder");
//         let bb_tag = pools.create_basic_block(context_tag, function_tag, DEFAULT_BASIC_BLOCK_NAME).expect("Failed to create entry block");

//         pools.position_builder_at_end(builder_tag, bb_tag);
//         pools.void_return(builder_tag);

//         let module = pools.get_module(module_tag).expect("Failed to retrieve module");

//         let validator = Validator::new(module.clone());
//         assert!(validator.is_valid_module(), "Invalid module");

//         let mut engine = ExecutionEngine::new(module, true);
//         engine.initialize_target(GeneralTargetConfigurator {}).expect("Failed to configure engine");

//         let result: Result<(), String> = engine.execute(DEFAULT_FUNCTION_NAME, ());
//         assert!(result.is_ok(), "Function did not execute successfully");
//     }
// }

// #[test]
// fn test_execute_boolean_function() {
//     for _ in 1..1000 {
//         let mut pools = IRManager::new();
//         let context_tag = pools.create_context().expect("Failed to create context");
//         let module_tag = pools.create_module(DEFAULT_MODULE_NAME, context_tag).expect("Failed to create module");
//         let bool_type = pools.boolean_type(context_tag).expect("Failed to create boolean type");
//         let function_value = pools.create_function(Some(bool_type), &[], false, context_tag).expect("Failed to create function prototype");
//         let function_tag = pools.add_function_to_module(module_tag, "bool_func", function_value).expect("Failed to add function to module");
//         let builder_tag = pools.create_builder(context_tag).expect("Failed to create builder");
//         let bb_tag = pools.create_basic_block(context_tag, function_tag, DEFAULT_BASIC_BLOCK_NAME).expect("Failed to create entry block");
//         let return_value = pools.create_boolean(context_tag, true).expect("Failed to create return value");

//         pools.position_builder_at_end(builder_tag, bb_tag);
//         pools.nonvoid_return(builder_tag, return_value);

//         let module = pools.get_module(module_tag).expect("Failed to retrieve module");

//         let validator = Validator::new(module.clone());
//         assert!(validator.is_valid_module(), "Invalid module");

//         let mut engine = ExecutionEngine::new(module, true);
//         engine.initialize_target(GeneralTargetConfigurator {}).expect("Failed to configure engine");

//         let result: Result<bool, String> = engine.execute("bool_func", ());
//         assert_eq!(result.unwrap(), true, "Function did not return the expected boolean value");
//     }
// }
