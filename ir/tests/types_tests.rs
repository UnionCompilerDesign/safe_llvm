use ir::core::IRGenerator;


#[test]
fn test_void_type() {
    let mut resource_pools = IRGenerator::new();

    let context_tag = resource_pools.create_context().expect("Failed to create context");
    let void_type_tag = resource_pools.void_type(context_tag);

    assert!(void_type_tag.is_some(), "Void type should be created successfully");
}

#[test]
fn test_int_type() {
    let mut resource_pools = IRGenerator::new();

    let context_tag = resource_pools.create_context().expect("Failed to create context");
    let int_type_tag = resource_pools.int_type(context_tag, 32);

    assert!(int_type_tag.is_some(), "Integer type with 32 bits should be created successfully");
}

#[test]
fn test_float_type() {
    let mut resource_pools = IRGenerator::new();

    let context_tag = resource_pools.create_context().expect("Failed to create context");
    let float_type_tag = resource_pools.float_type(context_tag);

    assert!(float_type_tag.is_some(), "Float type should be created successfully");
}

#[test]
fn test_boolean_type() {
    let mut resource_pools = IRGenerator::new();

    let context_tag = resource_pools.create_context().expect("Failed to create context");
    let boolean_type_tag = resource_pools.boolean_type(context_tag);

    assert!(boolean_type_tag.is_some(), "Boolean type should be created successfully");
}

#[test]
fn test_pointer_type() {
    let mut resource_pools = IRGenerator::new();

    let context_tag = resource_pools.create_context().expect("Failed to create context");
    let int_type_tag = resource_pools.int_type(context_tag, 32).expect("Failed to create integer type");
    let pointer_type_tag = resource_pools.pointer_type(int_type_tag);

    assert!(pointer_type_tag.is_some(), "Pointer type for integer should be created successfully");
}

#[test]
fn test_array_type() {
    let mut resource_pools = IRGenerator::new();

    let context_tag = resource_pools.create_context().expect("Failed to create context");
    let int_type_tag = resource_pools.int_type(context_tag, 32).expect("Failed to create integer type");
    let array_type_tag = resource_pools.array_type(int_type_tag, 10);

    assert!(array_type_tag.is_some(), "Array type of 10 integers should be created successfully");
}

#[test]
fn test_struct_type() {
    let mut resource_pools = IRGenerator::new();

    let context_tag = resource_pools.create_context().expect("Failed to create context");
    let int_type_tag = resource_pools.int_type(context_tag, 32).expect("Failed to create integer type");
    let float_type_tag = resource_pools.float_type(context_tag).expect("Failed to create float type");
    let element_types = vec![int_type_tag, float_type_tag];
    let struct_type_tag = resource_pools.struct_type(context_tag, &element_types, false);
    
    assert!(struct_type_tag.is_some(), "Struct type with integer and float fields should be created successfully");
}
