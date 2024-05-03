use safe_llvm::memory_management::resource_pools::ResourcePools;

#[test]
fn test_build_add() {
    let mut resource_pools = ResourcePools::new();

    let context_tag = resource_pools.create_context().expect("Failed to create context");
    let builder_tag = resource_pools.create_builder(context_tag).expect("Failed to create builder");
    let param_a_tag = resource_pools.create_integer(context_tag, 10).expect("Failed to create param a");
    let param_b_tag = resource_pools.create_integer(context_tag, 20).expect("Failed to create param b");
    let result_tag = resource_pools.build_add(builder_tag, param_a_tag, param_b_tag, "add_result");

    assert!(result_tag.is_some(), "Add operation should produce a result");
}

#[test]
fn test_build_sub() {
    let mut resource_pools = ResourcePools::new();

    let context_tag = resource_pools.create_context().expect("Failed to create context");
    let builder_tag = resource_pools.create_builder(context_tag).expect("Failed to create builder");
    let param_a_tag = resource_pools.create_integer(context_tag, 10).expect("Failed to create param a");
    let param_b_tag = resource_pools.create_integer(context_tag, 20).expect("Failed to create param b");
    let result_tag = resource_pools.build_sub(builder_tag, param_a_tag, param_b_tag, "sub_result");

    assert!(result_tag.is_some(), "Subtraction operation should produce a result");
}

#[test]
fn test_build_mul() {
    let mut resource_pools = ResourcePools::new();
    
    let context_tag = resource_pools.create_context().expect("Failed to create context");
    let builder_tag = resource_pools.create_builder(context_tag).expect("Failed to create builder");
    let param_a_tag = resource_pools.create_integer(context_tag, 10).expect("Failed to create param a");
    let param_b_tag = resource_pools.create_integer(context_tag, 20).expect("Failed to create param b");
    let result_tag = resource_pools.build_mul(builder_tag, param_a_tag, param_b_tag, "mul_result");

    assert!(result_tag.is_some(), "Multiplication operation should produce a result");
}

#[test]
fn test_build_div() {
    let mut resource_pools = ResourcePools::new();

    let context_tag = resource_pools.create_context().expect("Failed to create context");
    let builder_tag = resource_pools.create_builder(context_tag).expect("Failed to create builder");
    let param_a_tag = resource_pools.create_integer(context_tag, 20).expect("Failed to create param a");
    let param_b_tag = resource_pools.create_integer(context_tag, 10).expect("Failed to create param b");
    let result_tag = resource_pools.build_div(builder_tag, param_a_tag, param_b_tag, "div_result");

    assert!(result_tag.is_some(), "Division operation should produce a result");
}

#[test]
fn test_build_rem() {
    let mut resource_pools = ResourcePools::new();

    let context_tag = resource_pools.create_context().expect("Failed to create context");
    let builder_tag = resource_pools.create_builder(context_tag).expect("Failed to create builder");
    let param_a_tag = resource_pools.create_integer(context_tag, 23).expect("Failed to create param a");
    let param_b_tag = resource_pools.create_integer(context_tag, 5).expect("Failed to create param b");
    let result_tag = resource_pools.build_rem(builder_tag, param_a_tag, param_b_tag, "rem_result");

    assert!(result_tag.is_some(), "Remainder operation should produce a result");
}

#[test]
fn test_build_and() {
    let mut resource_pools = ResourcePools::new();

    let context_tag = resource_pools.create_context().expect("Failed to create context");
    let builder_tag = resource_pools.create_builder(context_tag).expect("Failed to create builder");
    let param_a_tag = resource_pools.create_integer(context_tag, 0xF0).expect("Failed to create param a");
    let param_b_tag = resource_pools.create_integer(context_tag, 0x33).expect("Failed to create param b");
    let result_tag = resource_pools.build_and(builder_tag, param_a_tag, param_b_tag, "and_result");

    assert!(result_tag.is_some(), "AND operation should produce a result");
}

#[test]
fn test_build_or() {
    let mut resource_pools = ResourcePools::new();

    let context_tag = resource_pools.create_context().expect("Failed to create context");
    let builder_tag = resource_pools.create_builder(context_tag).expect("Failed to create builder");
    let param_a_tag = resource_pools.create_integer(context_tag, 0x22).expect("Failed to create param a");
    let param_b_tag = resource_pools.create_integer(context_tag, 0x11).expect("Failed to create param b");
    let result_tag = resource_pools.build_or(builder_tag, param_a_tag, param_b_tag, "or_result");

    assert!(result_tag.is_some(), "OR operation should produce a result");
}

#[test]
fn test_build_xor() {
    let mut resource_pools = ResourcePools::new();

    let context_tag = resource_pools.create_context().expect("Failed to create context");
    let builder_tag = resource_pools.create_builder(context_tag).expect("Failed to create builder");
    let param_a_tag = resource_pools.create_integer(context_tag, 0xFF).expect("Failed to create param a");
    let param_b_tag = resource_pools.create_integer(context_tag, 0x0F).expect("Failed to create param b");
    let result_tag = resource_pools.build_xor(builder_tag, param_a_tag, param_b_tag, "xor_result");

    assert!(result_tag.is_some(), "XOR operation should produce a result");
}

#[test]
fn test_build_shl() {
    let mut resource_pools = ResourcePools::new();
    let context_tag = resource_pools.create_context().expect("Failed to create context");
    let builder_tag = resource_pools.create_builder(context_tag).expect("Failed to create builder");
    let param_a_tag = resource_pools.create_integer( context_tag, 1).expect("Failed to create param a");
    let param_b_tag = resource_pools.create_integer( context_tag, 2).expect("Failed to create param a");
    let result_tag = resource_pools.build_shl(builder_tag, param_a_tag,  param_b_tag, "shl_result"); 
    assert!(result_tag.is_some(), "Shift left operation should produce a result");
}

#[test]
fn test_build_shr() {
    let mut resource_pools = ResourcePools::new();
    let context_tag = resource_pools.create_context().expect("Failed to create context");
    let builder_tag = resource_pools.create_builder(context_tag).expect("Failed to create builder");
    let param_a_tag = resource_pools.create_integer( context_tag, 4).expect("Failed to create param a");
    let param_b_tag = resource_pools.create_integer( context_tag, 4).expect("Failed to create param a");
    let result_tag = resource_pools.build_shr(builder_tag, param_a_tag, param_b_tag, "shr_result"); 
    assert!(result_tag.is_some(), "Shift right operation should produce a result");
}

#[test]
fn test_build_icmp_gt() {
    let mut resource_pools = ResourcePools::new();

    let context_tag = resource_pools.create_context().expect("Failed to create context");
    let builder_tag = resource_pools.create_builder(context_tag).expect("Failed to create builder");
    let param_a_tag = resource_pools.create_integer(context_tag, 10).expect("Failed to create param a");
    let param_b_tag = resource_pools.create_integer(context_tag, 5).expect("Failed to create param b");
    let result_tag = resource_pools.build_icmp_gt(builder_tag, param_a_tag, param_b_tag, "gt_result");

    assert!(result_tag.is_some(), "Greater than comparison should produce a result");
}

#[test]
fn test_build_icmp_lt() {
    let mut resource_pools = ResourcePools::new();

    let context_tag = resource_pools.create_context().expect("Failed to create context");
    let builder_tag = resource_pools.create_builder(context_tag).expect("Failed to create builder");
    let param_a_tag = resource_pools.create_integer(context_tag, 5).expect("Failed to create param a");
    let param_b_tag = resource_pools.create_integer(context_tag, 10).expect("Failed to create param b");
    let result_tag = resource_pools.build_icmp_lt(builder_tag, param_a_tag, param_b_tag, "lt_result");

    assert!(result_tag.is_some(), "Less than comparison should produce a result");
}
#[test]
fn test_build_icmp_eq() {
    let mut resource_pools = ResourcePools::new();

    let context_tag = resource_pools.create_context().expect("Failed to create context");
    let builder_tag = resource_pools.create_builder(context_tag).expect("Failed to create builder");
    let param_a_tag = resource_pools.create_integer(context_tag, 10).expect("Failed to create param a");
    let param_b_tag = resource_pools.create_integer(context_tag, 10).expect("Failed to create param b");
    let result_tag = resource_pools.build_icmp_eq(builder_tag, param_a_tag, param_b_tag, "eq_result");

    assert!(result_tag.is_some(), "Equal comparison should produce a result");
}

#[test]
fn test_build_negation() {
    let mut resource_pools = ResourcePools::new();
    let context_tag = resource_pools.create_context().expect("Failed to create context");
    let builder_tag = resource_pools.create_builder(context_tag).expect("Failed to create builder");
    let param_a_tag = resource_pools.create_integer(context_tag, 10).expect("Failed to create param a");
    let result_tag = resource_pools.build_negation(builder_tag, param_a_tag, "neg_result");
    assert!(result_tag.is_some(), "Negation operation should produce a result");
}

#[test]
fn test_build_bitwise_not() {
    let mut resource_pools = ResourcePools::new();
    let context_tag = resource_pools.create_context().expect("Failed to create context");
    let builder_tag = resource_pools.create_builder(context_tag).expect("Failed to create builder");
    let param_a_tag = resource_pools.create_integer(context_tag, 0xF0).expect("Failed to create param a");
    let result_tag = resource_pools.build_bitwise_not(builder_tag, param_a_tag, "not_result");
    assert!(result_tag.is_some(), "Bitwise NOT operation should produce a result");
}

#[test]
fn test_build_logical_not() {
    let mut resource_pools = ResourcePools::new();
    let context_tag = resource_pools.create_context().expect("Failed to create context");
    let builder_tag = resource_pools.create_builder(context_tag).expect("Failed to create builder");
    let param_a_tag = resource_pools.create_boolean(context_tag, true).expect("Failed to create param a");
    let result_tag = resource_pools.build_logical_not(builder_tag, context_tag, param_a_tag, "logical_not_result");
    assert!(result_tag.is_some(), "Logical NOT operation should produce a result");
}
