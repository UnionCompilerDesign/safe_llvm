use safe_llvm::memory_management::resource_pools::IRGenerator;

#[test]
fn test_create_integer() {
    let mut resource_pools = IRGenerator::new();
    let context_tag = resource_pools.create_context().expect("Failed to create context");
    let integer_value_tag = resource_pools.create_integer(context_tag, 42);
    assert!(integer_value_tag.is_some(), "Integer value should be created successfully");
}

#[test]
fn test_create_float() {
    let mut resource_pools = IRGenerator::new();
    let context_tag = resource_pools.create_context().expect("Failed to create context");
    let float_value_tag = resource_pools.create_float(context_tag, 3.14);
    assert!(float_value_tag.is_some(), "Float value should be created successfully");
}

#[test]
fn test_create_boolean() {
    let mut resource_pools = IRGenerator::new();
    let context_tag = resource_pools.create_context().expect("Failed to create context");
    let boolean_value_tag = resource_pools.create_boolean(context_tag, true);
    assert!(boolean_value_tag.is_some(), "Boolean value should be created successfully");
}

#[test]
fn test_create_array() {
    let mut resource_pools = IRGenerator::new();
    let context_tag = resource_pools.create_context().expect("Failed to create context");
    let integer_tag = resource_pools.create_integer(context_tag, 32).expect("Failed to create integer type");
    let array_value_tag = resource_pools.create_array(integer_tag, 10);
    assert!(array_value_tag.is_some(), "Array of integers should be created successfully");
}

#[test]
fn test_create_pointer() {
    let mut resource_pools = IRGenerator::new();
    let context_tag = resource_pools.create_context().expect("Failed to create context");
    let integer_type_tag = resource_pools.int_type(context_tag, 32).expect("Failed to create integer type");
    let pointer_value_tag = resource_pools.create_pointer(integer_type_tag);
    assert!(pointer_value_tag.is_some(), "Pointer to integer should be created successfully");
}

#[test]
fn test_create_string() {
    let mut resource_pools = IRGenerator::new();
    let _context_tag = resource_pools.create_context().expect("Failed to create context");
    let string_value_tag = resource_pools.create_string("hello, world");
    assert!(string_value_tag.is_some(), "String value should be created successfully");
}

// #[test]
// fn test_create_mut_string() {
//     let mut resource_pools = IRGenerator::new();
//     let context_tag = resource_pools.create_context().expect("Failed to create context");
//     let builder_tag = resource_pools.create_builder(context_tag).expect("Failed to create builder");
//     let mut_string_value_tag = resource_pools.create_mut_string("mutable string", context_tag, builder_tag);
//     assert!(mut_string_value_tag.is_some(), "Mutable string value should be created successfully");
// }

#[test]
fn test_create_null_pointer() {
    let mut resource_pools = IRGenerator::new();
    let context_tag = resource_pools.create_context().expect("Failed to create context");
    let int_type_tag = resource_pools.int_type(context_tag, 32).expect("Failed to create integer type");
    let null_pointer_tag = resource_pools.create_null_pointer(int_type_tag);
    assert!(null_pointer_tag.is_some(), "Null pointer should be created successfully");
}
