use safe_llvm::{
    analysis::validator::Validator, common::{self, target::*}, constants::{DEFAULT_BASIC_BLOCK_NAME, DEFAULT_FUNCTION_NAME, DEFAULT_MODULE_NAME}, ir::core::IRGenerator, jit::execution_engine::ExecutionEngine
};

#[test]
fn test_execution_engine_with_general_targeting() {
    let mut resource_pools = IRGenerator::new();

    let context_tag = resource_pools.create_context().expect("Failed to create context");
    let module_tag = resource_pools.create_module(DEFAULT_MODULE_NAME, context_tag).expect("Failed to create module");
    let function_type = resource_pools.int_type(context_tag, 64).expect("Failed to create function type");
    let function_value = resource_pools.create_function(Some(function_type), &[], false, context_tag).expect("Failed to create function prototype");
    let function_tag = resource_pools.add_function_to_module(module_tag, DEFAULT_FUNCTION_NAME, function_value).expect("Failed to add function to module");
    let builder_tag = resource_pools.create_builder(context_tag).expect("Failed to create builder");
    let entry_bb_tag = resource_pools.create_basic_block(context_tag, function_tag, DEFAULT_BASIC_BLOCK_NAME).expect("Failed to create entry block");
    let return_value = resource_pools.create_integer(context_tag, 0).expect("Failed to create return val");

    resource_pools.position_builder(builder_tag, entry_bb_tag);
    resource_pools.nonvoid_return(builder_tag, return_value);

    let module = resource_pools.get_module(module_tag).expect("Failed to retrieve module");

    match common::write_ir::write_to_file(module.clone(), "test_execution_engine_with_general_targeting") {
        Ok(_) => {}
        Err(e) => {
            eprintln!("File write error: {}", e);
            panic!();        
        }
    }

    let validator = Validator::new(module.clone());
    assert!(validator.is_valid_module(), "Invalid module");

    let function = resource_pools.get_value(function_tag).expect("Failed to get function");
    assert!(validator.is_valid_function(function), "Invalid function");

    let mut engine = ExecutionEngine::new( module, true);
    engine.init_target(GeneralTargetConfigurator {}).expect("Failed to configure engine");

    let result = engine.execute(DEFAULT_FUNCTION_NAME);

    assert!(result.is_ok(), "Execution failed with error: {:?}", result.err());
    println!("Execution result: {:?}", result);
}

#[test]
fn test_execution_engine_with_arm_targeting() {
    let mut resource_pools = IRGenerator::new();

    let context_tag = resource_pools.create_context().expect("Failed to create context");
    let module_tag = resource_pools.create_module(DEFAULT_MODULE_NAME, context_tag).expect("Failed to create module");
    let function_type = resource_pools.int_type(context_tag, 64).expect("Failed to create function type");
    let function_value = resource_pools.create_function(Some(function_type), &[], false, context_tag).expect("Failed to create function prototype");
    let function_tag = resource_pools.add_function_to_module(module_tag, DEFAULT_FUNCTION_NAME, function_value).expect("Failed to add function to module");
    let builder_tag = resource_pools.create_builder(context_tag).expect("Failed to create builder");
    let entry_bb_tag = resource_pools.create_basic_block(context_tag, function_tag, DEFAULT_BASIC_BLOCK_NAME).expect("Failed to create entry block");
    let return_value = resource_pools.create_integer(context_tag, 0).expect("Failed to create return val");

    resource_pools.position_builder(builder_tag, entry_bb_tag);
    resource_pools.nonvoid_return(builder_tag, return_value);

    let module = resource_pools.get_module(module_tag).expect("Failed to retrieve module");

    match common::write_ir::write_to_file(module.clone(), "test_execution_engine_with_general_targeting") {
        Ok(_) => {}
        Err(e) => {
            eprintln!("File write error: {}", e);
            panic!();        
        }
    }

    let validator = Validator::new(module.clone());
    assert!(validator.is_valid_module(), "Invalid module");

    let function = resource_pools.get_value(function_tag).expect("Failed to get function");
    assert!(validator.is_valid_function(function), "Invalid function");

    let mut engine = ExecutionEngine::new( module, true);
    engine.init_target(ARMTargetConfigurator {}).expect("Failed to configure engine");

    let result = engine.execute(DEFAULT_FUNCTION_NAME);

    assert!(result.is_ok(), "Execution failed with error: {:?}", result.err());
    println!("Execution result: {:?}", result);
}

#[test]
fn test_execution_engine_with_x86_targeting() {
    let mut resource_pools = IRGenerator::new();

    let context_tag = resource_pools.create_context().expect("Failed to create context");
    let module_tag = resource_pools.create_module(DEFAULT_MODULE_NAME, context_tag).expect("Failed to create module");
    let function_type = resource_pools.int_type(context_tag, 64).expect("Failed to create function type");
    let function_value = resource_pools.create_function(Some(function_type), &[], false, context_tag).expect("Failed to create function prototype");
    let function_tag = resource_pools.add_function_to_module(module_tag, DEFAULT_FUNCTION_NAME, function_value).expect("Failed to add function to module");
    let builder_tag = resource_pools.create_builder(context_tag).expect("Failed to create builder");
    let entry_bb_tag = resource_pools.create_basic_block(context_tag, function_tag, DEFAULT_BASIC_BLOCK_NAME).expect("Failed to create entry block");
    let return_value = resource_pools.create_integer(context_tag, 0).expect("Failed to create return val");

    resource_pools.position_builder(builder_tag, entry_bb_tag);
    resource_pools.nonvoid_return(builder_tag, return_value);

    let module = resource_pools.get_module(module_tag).expect("Failed to retrieve module");

    match common::write_ir::write_to_file(module.clone(), "test_execution_engine_with_general_targeting") {
        Ok(_) => {}
        Err(e) => {
            eprintln!("File write error: {}", e);
            panic!();        
        }
    }

    let validator = Validator::new(module.clone());
    assert!(validator.is_valid_module(), "Invalid module");

    let function = resource_pools.get_value(function_tag).expect("Failed to get function");
    assert!(validator.is_valid_function(function), "Invalid function");

    let mut engine = ExecutionEngine::new( module, true);
    engine.init_target(X86TargetConfigurator {}).expect("Failed to configure engine");

    let result = engine.execute(DEFAULT_FUNCTION_NAME);

    assert!(result.is_ok(), "Execution failed with error: {:?}", result.err());
    println!("Execution result: {:?}", result);
}

#[test]
fn test_execution_engine_with_mips_targeting() {
    let mut resource_pools = IRGenerator::new();

    let context_tag = resource_pools.create_context().expect("Failed to create context");
    let module_tag = resource_pools.create_module(DEFAULT_MODULE_NAME, context_tag).expect("Failed to create module");
    let function_type = resource_pools.int_type(context_tag, 64).expect("Failed to create function type");
    let function_value = resource_pools.create_function(Some(function_type), &[], false, context_tag).expect("Failed to create function prototype");
    let function_tag = resource_pools.add_function_to_module(module_tag, DEFAULT_FUNCTION_NAME, function_value).expect("Failed to add function to module");
    let builder_tag = resource_pools.create_builder(context_tag).expect("Failed to create builder");
    let entry_bb_tag = resource_pools.create_basic_block(context_tag, function_tag, DEFAULT_BASIC_BLOCK_NAME).expect("Failed to create entry block");
    let return_value = resource_pools.create_integer(context_tag, 0).expect("Failed to create return val");

    resource_pools.position_builder(builder_tag, entry_bb_tag);
    resource_pools.nonvoid_return(builder_tag, return_value);

    let module = resource_pools.get_module(module_tag).expect("Failed to retrieve module");

    match common::write_ir::write_to_file(module.clone(), "test_execution_engine_with_general_targeting") {
        Ok(_) => {}
        Err(e) => {
            eprintln!("File write error: {}", e);
            panic!();        
        }
    }

    let validator = Validator::new(module.clone());
    assert!(validator.is_valid_module(), "Invalid module");

    let function = resource_pools.get_value(function_tag).expect("Failed to get function");
    assert!(validator.is_valid_function(function), "Invalid function");

    let mut engine = ExecutionEngine::new( module, true);
    engine.init_target(MIPSTargetConfigurator {}).expect("Failed to configure engine");

    let result = engine.execute(DEFAULT_FUNCTION_NAME);

    assert!(result.is_ok(), "Execution failed with error: {:?}", result.err());
    println!("Execution result: {:?}", result);
}

#[test]
fn test_execution_engine_with_rv_targeting() {
    let mut resource_pools = IRGenerator::new();

    let context_tag = resource_pools.create_context().expect("Failed to create context");
    let module_tag = resource_pools.create_module(DEFAULT_MODULE_NAME, context_tag).expect("Failed to create module");
    let function_type = resource_pools.int_type(context_tag, 64).expect("Failed to create function type");
    let function_value = resource_pools.create_function(Some(function_type), &[], false, context_tag).expect("Failed to create function prototype");
    let function_tag = resource_pools.add_function_to_module(module_tag, DEFAULT_FUNCTION_NAME, function_value).expect("Failed to add function to module");
    let builder_tag = resource_pools.create_builder(context_tag).expect("Failed to create builder");
    let entry_bb_tag = resource_pools.create_basic_block(context_tag, function_tag, DEFAULT_BASIC_BLOCK_NAME).expect("Failed to create entry block");
    let return_value = resource_pools.create_integer(context_tag, 0).expect("Failed to create return val");

    resource_pools.position_builder(builder_tag, entry_bb_tag);
    resource_pools.nonvoid_return(builder_tag, return_value);

    let module = resource_pools.get_module(module_tag).expect("Failed to retrieve module");

    match common::write_ir::write_to_file(module.clone(), "test_execution_engine_with_general_targeting") {
        Ok(_) => {}
        Err(e) => {
            eprintln!("File write error: {}", e);
            panic!();        
        }
    }

    let validator = Validator::new(module.clone());
    assert!(validator.is_valid_module(), "Invalid module");

    let function = resource_pools.get_value(function_tag).expect("Failed to get function");
    assert!(validator.is_valid_function(function), "Invalid function");

    let mut engine = ExecutionEngine::new( module, true);
    engine.init_target(RVTargetConfigurator {}).expect("Failed to configure engine");

    let result = engine.execute(DEFAULT_FUNCTION_NAME);

    assert!(result.is_ok(), "Execution failed with error: {:?}", result.err());
    println!("Execution result: {:?}", result);
}

#[test]
fn test_execution_engine_with_wasm_targeting() {
    let mut resource_pools = IRGenerator::new();

    let context_tag = resource_pools.create_context().expect("Failed to create context");
    let module_tag = resource_pools.create_module(DEFAULT_MODULE_NAME, context_tag).expect("Failed to create module");
    let function_type = resource_pools.int_type(context_tag, 64).expect("Failed to create function type");
    let function_value = resource_pools.create_function(Some(function_type), &[], false, context_tag).expect("Failed to create function prototype");
    let function_tag = resource_pools.add_function_to_module(module_tag, DEFAULT_FUNCTION_NAME, function_value).expect("Failed to add function to module");
    let builder_tag = resource_pools.create_builder(context_tag).expect("Failed to create builder");
    let entry_bb_tag = resource_pools.create_basic_block(context_tag, function_tag, DEFAULT_BASIC_BLOCK_NAME).expect("Failed to create entry block");
    let return_value = resource_pools.create_integer(context_tag, 0).expect("Failed to create return val");

    resource_pools.position_builder(builder_tag, entry_bb_tag);
    resource_pools.nonvoid_return(builder_tag, return_value);

    let module = resource_pools.get_module(module_tag).expect("Failed to retrieve module");

    match common::write_ir::write_to_file(module.clone(), "test_execution_engine_with_general_targeting") {
        Ok(_) => {}
        Err(e) => {
            eprintln!("File write error: {}", e);
            panic!();        
        }
    }

    let validator = Validator::new(module.clone());
    assert!(validator.is_valid_module(), "Invalid module");

    let function = resource_pools.get_value(function_tag).expect("Failed to get function");
    assert!(validator.is_valid_function(function), "Invalid function");

    let mut engine = ExecutionEngine::new( module, true);
    engine.init_target(WasmTargetConfigurator {}).expect("Failed to configure engine");

    let result = engine.execute(DEFAULT_FUNCTION_NAME);

    assert!(result.is_ok(), "Execution failed with error: {:?}", result.err());
    println!("Execution result: {:?}", result);
}

#[test]
fn test_execution_engine_with_ppc_targeting() {
    let mut resource_pools = IRGenerator::new();

    let context_tag = resource_pools.create_context().expect("Failed to create context");
    let module_tag = resource_pools.create_module(DEFAULT_MODULE_NAME, context_tag).expect("Failed to create module");
    let function_type = resource_pools.int_type(context_tag, 64).expect("Failed to create function type");
    let function_value = resource_pools.create_function(Some(function_type), &[], false, context_tag).expect("Failed to create function prototype");
    let function_tag = resource_pools.add_function_to_module(module_tag, DEFAULT_FUNCTION_NAME, function_value).expect("Failed to add function to module");
    let builder_tag = resource_pools.create_builder(context_tag).expect("Failed to create builder");
    let entry_bb_tag = resource_pools.create_basic_block(context_tag, function_tag, DEFAULT_BASIC_BLOCK_NAME).expect("Failed to create entry block");
    let return_value = resource_pools.create_integer(context_tag, 0).expect("Failed to create return val");

    resource_pools.position_builder(builder_tag, entry_bb_tag);
    resource_pools.nonvoid_return(builder_tag, return_value);

    let module = resource_pools.get_module(module_tag).expect("Failed to retrieve module");

    match common::write_ir::write_to_file(module.clone(), "test_execution_engine_with_general_targeting") {
        Ok(_) => {}
        Err(e) => {
            eprintln!("File write error: {}", e);
            panic!();        
        }
    }

    let validator = Validator::new(module.clone());
    assert!(validator.is_valid_module(), "Invalid module");

    let function = resource_pools.get_value(function_tag).expect("Failed to get function");
    assert!(validator.is_valid_function(function), "Invalid function");

    let mut engine = ExecutionEngine::new(module, true);
    engine.init_target(PPCTargetConfigurator {}).expect("Failed to configure engine");

    let result = engine.execute(DEFAULT_FUNCTION_NAME);

    assert!(result.is_ok(), "Execution failed with error: {:?}", result.err());
    println!("Execution result: {:?}", result);
}

#[test]
fn test_execution_engine_with_sparc_targeting() {
    let mut resource_pools = IRGenerator::new();

    let context_tag = resource_pools.create_context().expect("Failed to create context");
    let module_tag = resource_pools.create_module(DEFAULT_MODULE_NAME, context_tag).expect("Failed to create module");
    let function_type = resource_pools.int_type(context_tag, 64).expect("Failed to create function type");
    let function_value = resource_pools.create_function(Some(function_type), &[], false, context_tag).expect("Failed to create function prototype");
    let function_tag = resource_pools.add_function_to_module(module_tag, DEFAULT_FUNCTION_NAME, function_value).expect("Failed to add function to module");
    let builder_tag = resource_pools.create_builder(context_tag).expect("Failed to create builder");
    let entry_bb_tag = resource_pools.create_basic_block(context_tag, function_tag, DEFAULT_BASIC_BLOCK_NAME).expect("Failed to create entry block");
    let return_value = resource_pools.create_integer(context_tag, 0).expect("Failed to create return val");

    resource_pools.position_builder(builder_tag, entry_bb_tag);
    resource_pools.nonvoid_return(builder_tag, return_value);

    let module = resource_pools.get_module(module_tag).expect("Failed to retrieve module");

    match common::write_ir::write_to_file(module.clone(), "test_execution_engine_with_general_targeting") {
        Ok(_) => {}
        Err(e) => {
            eprintln!("File write error: {}", e);
            panic!();        
        }
    }

    let validator = Validator::new(module.clone());
    assert!(validator.is_valid_module(), "Invalid module");

    let function = resource_pools.get_value(function_tag).expect("Failed to get function");
    assert!(validator.is_valid_function(function), "Invalid function");

    let mut engine = ExecutionEngine::new( module, true);
    engine.init_target(SparcTargetConfigurator {}).expect("Failed to configure engine");

    let result = engine.execute(DEFAULT_FUNCTION_NAME);

    assert!(result.is_ok(), "Execution failed with error: {:?}", result.err());
    println!("Execution result: {:?}", result);
}

#[test]
fn test_execution_engine_with_systemz_targeting() {
    let mut resource_pools = IRGenerator::new();

    let context_tag = resource_pools.create_context().expect("Failed to create context");
    let module_tag = resource_pools.create_module(DEFAULT_MODULE_NAME, context_tag).expect("Failed to create module");
    let function_type = resource_pools.int_type(context_tag, 64).expect("Failed to create function type");
    let function_value = resource_pools.create_function(Some(function_type), &[], false, context_tag).expect("Failed to create function prototype");
    let function_tag = resource_pools.add_function_to_module(module_tag, DEFAULT_FUNCTION_NAME, function_value).expect("Failed to add function to module");
    let builder_tag = resource_pools.create_builder(context_tag).expect("Failed to create builder");
    let entry_bb_tag = resource_pools.create_basic_block(context_tag, function_tag, DEFAULT_BASIC_BLOCK_NAME).expect("Failed to create entry block");
    let return_value = resource_pools.create_integer(context_tag, 0).expect("Failed to create return val");

    resource_pools.position_builder(builder_tag, entry_bb_tag);
    resource_pools.nonvoid_return(builder_tag, return_value);

    let module = resource_pools.get_module(module_tag).expect("Failed to retrieve module");

    match common::write_ir::write_to_file(module.clone(), "test_execution_engine_with_general_targeting") {
        Ok(_) => {}
        Err(e) => {
            eprintln!("File write error: {}", e);
            panic!();        
        }
    }

    let validator = Validator::new(module.clone());
    assert!(validator.is_valid_module(), "Invalid module");

    let function = resource_pools.get_value(function_tag).expect("Failed to get function");
    assert!(validator.is_valid_function(function), "Invalid function");

    let mut engine = ExecutionEngine::new(module, true);
    engine.init_target(SystemZTargetConfigurator {}).expect("Failed to configure engine");

    let result = engine.execute(DEFAULT_FUNCTION_NAME);

    assert!(result.is_ok(), "Execution failed with error: {:?}", result.err());
    println!("Execution result: {:?}", result);
}

#[test]
fn test_execution_engine_with_aarch64_targeting() {
    let mut resource_pools = IRGenerator::new();

    let context_tag = resource_pools.create_context().expect("Failed to create context");
    let module_tag = resource_pools.create_module(DEFAULT_MODULE_NAME, context_tag).expect("Failed to create module");
    let function_type = resource_pools.int_type(context_tag, 64).expect("Failed to create function type");
    let function_value = resource_pools.create_function(Some(function_type), &[], false, context_tag).expect("Failed to create function prototype");
    let function_tag = resource_pools.add_function_to_module(module_tag, DEFAULT_FUNCTION_NAME, function_value).expect("Failed to add function to module");
    let builder_tag = resource_pools.create_builder(context_tag).expect("Failed to create builder");
    let entry_bb_tag = resource_pools.create_basic_block(context_tag, function_tag, DEFAULT_BASIC_BLOCK_NAME).expect("Failed to create entry block");
    let return_value = resource_pools.create_integer(context_tag, 0).expect("Failed to create return val");

    resource_pools.position_builder(builder_tag, entry_bb_tag);
    resource_pools.nonvoid_return(builder_tag, return_value);

    let module = resource_pools.get_module(module_tag).expect("Failed to retrieve module");

    match common::write_ir::write_to_file(module.clone(), "test_execution_engine_with_general_targeting") {
        Ok(_) => {}
        Err(e) => {
            eprintln!("File write error: {}", e);
            panic!();        
        }
    }

    let validator = Validator::new(module.clone());
    assert!(validator.is_valid_module(), "Invalid module");

    let function = resource_pools.get_value(function_tag).expect("Failed to get function");
    assert!(validator.is_valid_function(function), "Invalid function");

    let mut engine = ExecutionEngine::new( module, true);
    engine.init_target(AArch64TargetConfigurator {}).expect("Failed to configure engine");

    let result = engine.execute(DEFAULT_FUNCTION_NAME);

    assert!(result.is_ok(), "Execution failed with error: {:?}", result.err());
    println!("Execution result: {:?}", result);
}

#[test]
fn test_execution_engine_with_amdgpu_targeting() {
    let mut resource_pools = IRGenerator::new();

    let context_tag = resource_pools.create_context().expect("Failed to create context");
    let module_tag = resource_pools.create_module(DEFAULT_MODULE_NAME, context_tag).expect("Failed to create module");
    let function_type = resource_pools.int_type(context_tag, 64).expect("Failed to create function type");
    let function_value = resource_pools.create_function(Some(function_type), &[], false, context_tag).expect("Failed to create function prototype");
    let function_tag = resource_pools.add_function_to_module(module_tag, DEFAULT_FUNCTION_NAME, function_value).expect("Failed to add function to module");
    let builder_tag = resource_pools.create_builder(context_tag).expect("Failed to create builder");
    let entry_bb_tag = resource_pools.create_basic_block(context_tag, function_tag, DEFAULT_BASIC_BLOCK_NAME).expect("Failed to create entry block");
    let return_value = resource_pools.create_integer(context_tag, 0).expect("Failed to create return val");

    resource_pools.position_builder(builder_tag, entry_bb_tag);
    resource_pools.nonvoid_return(builder_tag, return_value);

    let module = resource_pools.get_module(module_tag).expect("Failed to retrieve module");

    match common::write_ir::write_to_file(module.clone(), "test_execution_engine_with_general_targeting") {
        Ok(_) => {}
        Err(e) => {
            eprintln!("File write error: {}", e);
            panic!();        
        }
    }

    let validator = Validator::new(module.clone());
    assert!(validator.is_valid_module(), "Invalid module");

    let function = resource_pools.get_value(function_tag).expect("Failed to get function");
    assert!(validator.is_valid_function(function), "Invalid function");

    let mut engine = ExecutionEngine::new( module, true);
    engine.init_target(AMDGPUTargetConfigurator {}).expect("Failed to configure engine");

    let result = engine.execute(DEFAULT_FUNCTION_NAME);

    assert!(result.is_ok(), "Execution failed with error: {:?}", result.err());
    println!("Execution result: {:?}", result);
}

#[test]
fn test_execution_engine_with_bpf_targeting() {
    let mut resource_pools = IRGenerator::new();

    let context_tag = resource_pools.create_context().expect("Failed to create context");
    let module_tag = resource_pools.create_module(DEFAULT_MODULE_NAME, context_tag).expect("Failed to create module");
    let function_type = resource_pools.int_type(context_tag, 64).expect("Failed to create function type");
    let function_value = resource_pools.create_function(Some(function_type), &[], false, context_tag).expect("Failed to create function prototype");
    let function_tag = resource_pools.add_function_to_module(module_tag, DEFAULT_FUNCTION_NAME, function_value).expect("Failed to add function to module");
    let builder_tag = resource_pools.create_builder(context_tag).expect("Failed to create builder");
    let entry_bb_tag = resource_pools.create_basic_block(context_tag, function_tag, DEFAULT_BASIC_BLOCK_NAME).expect("Failed to create entry block");
    let return_value = resource_pools.create_integer(context_tag, 0).expect("Failed to create return val");

    resource_pools.position_builder(builder_tag, entry_bb_tag);
    resource_pools.nonvoid_return(builder_tag, return_value);

    let module = resource_pools.get_module(module_tag).expect("Failed to retrieve module");

    match common::write_ir::write_to_file(module.clone(), "test_execution_engine_with_general_targeting") {
        Ok(_) => {}
        Err(e) => {
            eprintln!("File write error: {}", e);
            panic!();        
        }
    }

    let validator = Validator::new(module.clone());
    assert!(validator.is_valid_module(), "Invalid module");

    let function = resource_pools.get_value(function_tag).expect("Failed to get function");
    assert!(validator.is_valid_function(function), "Invalid function");

    let mut engine = ExecutionEngine::new( module, true);
    engine.init_target(BPFTargetConfigurator {}).expect("Failed to configure engine");

    let result = engine.execute(DEFAULT_FUNCTION_NAME);

    assert!(result.is_ok(), "Execution failed with error: {:?}", result.err());
    println!("Execution result: {:?}", result);
}
