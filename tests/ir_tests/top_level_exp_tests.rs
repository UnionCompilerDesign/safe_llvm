use safe_llvm::{analysis::validator::Validator, memory_management::resource_pools::ResourcePools, utils::utils_struct::Utils};

#[test]
fn test_add_function_to_module() {
    let mut resource_pools = ResourcePools::new();

    let context_tag = resource_pools.create_context().expect("Failed to create context");
    let module_tag = resource_pools.create_module("test_module", context_tag).expect("Failed to create module");
    let void_type_tag = resource_pools.void_type(context_tag).expect("Failed to create void type");
    let function_tag = resource_pools.create_function(Some(void_type_tag), &[], false, context_tag).expect("Failed to create function");
    let added_function_tag = resource_pools.add_function_to_module(module_tag, "function_name", function_tag).expect("Failed to add function to module");
    let builder_tag = resource_pools.create_builder(context_tag).expect("Failed to create builder");
    let entry_bb_tag = resource_pools.create_basic_block(context_tag, added_function_tag, "entry").expect("Failed to create entry block");

    resource_pools.position_builder(builder_tag, entry_bb_tag);
    resource_pools.void_return(builder_tag);

    let module = resource_pools.get_module(module_tag).expect("Failed to get module");

    match Utils::write_to_file(module.clone(), "test_add_function_to_module") {
        Ok(_) => {}
        Err(e) => {
            eprintln!("File write error: {}", e);
            panic!();
        }
    }

    let validator = Validator::new(module);
    assert!(validator.is_valid_module(), "Invalid module");

    let function = resource_pools.get_value(added_function_tag).expect("Failed to get function");
    assert!(validator.is_valid_function(function), "Invalid function");
}

#[test]
fn test_get_param() {
    let mut resource_pools = ResourcePools::new();

    let context_tag = resource_pools.create_context().expect("Failed to create context");
    let module_tag = resource_pools.create_module("test_module", context_tag).expect("Failed to create module");
    let int_type_tag = resource_pools.int_type(context_tag, 32).expect("Failed to create integer type");
    let function_tag = resource_pools.create_function(Some(int_type_tag), &[int_type_tag], false, context_tag).expect("Failed to create function with parameters");
    let added_function_tag = resource_pools.add_function_to_module(module_tag, "function_name", function_tag).expect("Failed to add function to module");
    let builder_tag = resource_pools.create_builder(context_tag).expect("Failed to create builder");
    let entry_bb_tag = resource_pools.create_basic_block(context_tag, added_function_tag, "entry").expect("Failed to create entry block");
    let return_val = resource_pools.get_param(added_function_tag, 0).expect("Failed to get parameter");

    resource_pools.position_builder(builder_tag, entry_bb_tag);
    resource_pools.nonvoid_return(builder_tag, return_val);

    let module = resource_pools.get_module(module_tag).expect("Failed to get module");

    match Utils::write_to_file(module.clone(), "test_get_param") {
        Ok(_) => {}
        Err(e) => {
            eprintln!("File write error: {}", e);
            panic!();
        }
    }

    let validator = Validator::new(module);
    assert!(validator.is_valid_module(), "Invalid module");
    let function = resource_pools.get_value(added_function_tag).expect("Failed to get function");
    assert!(validator.is_valid_function(function), "Invalid function");
}

#[test]
fn test_create_enum() {
    let mut resource_pools = ResourcePools::new();

    let context_tag = resource_pools.create_context().expect("Failed to create context");

    let variants = vec!["Red".to_string(), "Green".to_string(), "Blue".to_string()];

    let num_bits = 2;
    let enum_type_tag = resource_pools.create_enum(context_tag, num_bits, "Color", &variants).expect("Failed to create enum type");

    let enum_def = resource_pools.get_enum_definition(enum_type_tag).expect("Failed to retrieve enum definition");

    assert_eq!(enum_def.get_name(), "Color", "Enum name does not match");
    assert_eq!(enum_def.get_variant("Red"), Some(0), "Red should be mapped to 0");
    assert_eq!(enum_def.get_variant("Green"), Some(1), "Green should be mapped to 1");
    assert_eq!(enum_def.get_variant("Blue"), Some(2), "Blue should be mapped to 2");
    assert_eq!(enum_def.get_variant("Yellow"), None, "Yellow should not be present");
}
