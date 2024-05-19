use safe_llvm::{
    analysis::validator::Validator, 
    constants::{DEFAULT_BASIC_BLOCK_NAME, DEFAULT_FUNCTION_NAME, DEFAULT_MODULE_NAME}, 
    memory_management::resource_pools::ResourcePools, 
    utils::utils_struct::Utils
};

#[test]
fn test_write_function_to_str() {
    let mut resource_pools = ResourcePools::new();

    let context_tag = resource_pools.create_context().expect("Failed to create context");
    let module_tag = resource_pools.create_module("test_module", context_tag).expect("Failed to create module");
    let void_type_tag = resource_pools.void_type(context_tag).expect("Failed to create void type");
    let function_tag = resource_pools.create_function(Some(void_type_tag), &[], false, context_tag).expect("Failed to create function");
    let added_function_tag = resource_pools.add_function_to_module(module_tag, "main", function_tag).expect("Failed to add function to module");
    let builder_tag = resource_pools.create_builder(context_tag).expect("Failed to create builder");
    let entry_bb_tag = resource_pools.create_basic_block(context_tag, added_function_tag, "entry").expect("Failed to create entry block");

    resource_pools.position_builder(builder_tag, entry_bb_tag);
    resource_pools.void_return(builder_tag);

    let module = resource_pools.get_module(module_tag).expect("Failed to get module");

    let to_compare: &str = "; ModuleID = 'test_module'\nsource_filename = \"test_module\"\n\ndefine void @main() {\nentry:\n  ret void\n}\n";

    match Utils::write_to_string(module.clone()) {
        Ok(string) => {assert_eq!(string, to_compare)}
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