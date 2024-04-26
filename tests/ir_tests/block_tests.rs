
use safe_llvm::ir::{block, builder, init, values};

#[test]
fn test_create_basic_block() {
    let pool = init::create_llvm_resource_pool();
    let context_tag = init::create_context(&pool).expect("Failed to create context");
    let function_tag = builder::create_function(&pool, None, &[], false, context_tag).expect("Failed to create function");
    let block_name = "test_block";

    let block_tag = block::create_basic_block(&pool, context_tag, function_tag, block_name);
    assert!(block_tag.is_some(), "Basic block should be created successfully");
}

#[test]
fn test_get_current_block() {
    let pool = init::create_llvm_resource_pool();
    let context_tag = init::create_context(&pool).expect("Failed to create context");
    let builder_tag = builder::create_builder(&pool, context_tag).expect("Failed to create builder");

    let block_tag = block::get_current_block(&pool, builder_tag);
    assert!(block_tag.is_none(), "Should return None when no block has been inserted yet");
}

#[test]
fn test_create_cond_br() {
    let pool = init::create_llvm_resource_pool();
    let context_tag = init::create_context(&pool).expect("Failed to create context");
    let builder_tag = builder::create_builder(&pool, context_tag).expect("Failed to create builder");
    let condition_tag = values::create_integer(&pool, 1, context_tag).expect("Failed to create condition");
    let then_bb_tag = block::create_basic_block(&pool, context_tag, builder_tag, "then").expect("Failed to create 'then' block");
    let else_bb_tag = block::create_basic_block(&pool, context_tag, builder_tag, "else").expect("Failed to create 'else' block");

    let cond_branch_tag = block::create_cond_br(&pool, builder_tag, condition_tag, then_bb_tag, else_bb_tag);
    assert!(cond_branch_tag.is_some(), "Conditional branch should be created successfully");
}

#[test]
fn test_create_br() {
    let pool = init::create_llvm_resource_pool();
    let context_tag = init::create_context(&pool).expect("Failed to create context");
    let builder_tag = builder::create_builder(&pool, context_tag).expect("Failed to create builder");
    let target_bb_tag = block::create_basic_block(&pool, context_tag, builder_tag, "target").expect("Failed to create target block");

    let branch_tag = block::create_br(&pool, builder_tag, target_bb_tag);
    assert!(branch_tag.is_some(), "Unconditional branch should be created successfully");
}

#[test]
fn test_insert_before_basic_block() {
    let pool = init::create_llvm_resource_pool();
    let context_tag = init::create_context(&pool).expect("Failed to create context");
    let target_bb_tag = block::create_basic_block(&pool, context_tag, builder_tag, "target").expect("Failed to create target block");
    let insert_before_tag = block::insert_before_basic_block(&pool, context_tag, target_bb_tag, "insert_before");
    
    assert!(insert_before_tag.is_some(), "Insert before block should be created successfully");
}

#[test]
fn test_position_builder() {
    let pool = init::create_llvm_resource_pool();
    let context_tag = init::create_context(&pool).expect("Failed to create context");
    let builder_tag = builder::create_builder(&pool, context_tag).expect("Failed to create builder");
    let bb_tag = block::create_basic_block(&pool, context_tag, builder_tag, "position_here").expect("Failed to create block");

    let result = block::position_builder(&pool, builder_tag, bb_tag);
    assert!(result.is_some(), "Builder should be positioned at the end of the block successfully");
}

#[test]
fn test_delete_basic_block() {
    let pool = init::create_llvm_resource_pool();
    let context_tag = init::create_context(&pool).expect("Failed to create context");
    let bb_tag = block::create_basic_block(&pool, context_tag, builder_tag, "to_delete").expect("Failed to create block to delete");

    let result = block::delete_basic_block(&pool, bb_tag);
    assert!(result.is_some(), "Basic block should be deleted successfully");
}

#[test]
fn test_get_first_instruction() {
    let pool = init::create_llvm_resource_pool();
    let context_tag = init::create_context(&pool).expect("Failed to create context");
    let bb_tag = block::create_basic_block(&pool, context_tag, builder_tag, "has_instruction").expect("Failed to create block");

    let instruction_tag = block::get_first_instruction(&pool, bb_tag);
    assert!(instruction_tag.is_some(), "Should retrieve the first instruction in the block");
}

#[test]
fn test_get_last_instruction() {
    let pool = init::create_llvm_resource_pool();
    let context_tag = init::create_context(&pool).expect("Failed to create context");
    let bb_tag = block::create_basic_block(&pool, context_tag, builder_tag, "has_instruction").expect("Failed to create block");


    let instruction_tag = block::get_last_instruction(&pool, bb_tag);
    assert!(instruction_tag.is_some(), "Should retrieve the last instruction in the block");
}