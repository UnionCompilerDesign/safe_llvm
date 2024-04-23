
// use safe_llvm::ir::{block, builder, init, values};

// #[test]
// fn test_create_basic_block() {
//     let pool = init::create_llvm_resource_pool();
//     let context_handle = init::create_context(&pool).expect("Failed to create context");
//     let function_handle = builder::create_function(&pool, None, &[], false, context_handle).expect("Failed to create function");
//     let block_name = "test_block";

//     let block_handle = block::create_basic_block(&pool, context_handle, function_handle, block_name);
//     assert!(block_handle.is_some(), "Basic block should be created successfully");
// }

// #[test]
// fn test_get_current_block() {
//     let pool = init::create_llvm_resource_pool();
//     let context_handle = init::create_context(&pool).expect("Failed to create context");
//     let builder_handle = builder::create_builder(&pool, context_handle).expect("Failed to create builder");

//     let block_handle = block::get_current_block(&pool, builder_handle);
//     assert!(block_handle.is_none(), "Should return None when no block has been inserted yet");
// }

// #[test]
// fn test_create_cond_br() {
//     let pool = init::create_llvm_resource_pool();
//     let context_handle = init::create_context(&pool).expect("Failed to create context");
//     let builder_handle = builder::create_builder(&pool, context_handle).expect("Failed to create builder");
//     let condition_handle = values::create_integer(&pool, 1, context_handle).expect("Failed to create condition");
//     let then_bb_handle = block::create_basic_block(&pool, context_handle, builder_handle, "then").expect("Failed to create 'then' block");
//     let else_bb_handle = block::create_basic_block(&pool, context_handle, builder_handle, "else").expect("Failed to create 'else' block");

//     let cond_branch_handle = block::create_cond_br(&pool, builder_handle, condition_handle, then_bb_handle, else_bb_handle);
//     assert!(cond_branch_handle.is_some(), "Conditional branch should be created successfully");
// }

// #[test]
// fn test_create_br() {
//     let pool = init::create_llvm_resource_pool();
//     let context_handle = init::create_context(&pool).expect("Failed to create context");
//     let builder_handle = builder::create_builder(&pool, context_handle).expect("Failed to create builder");
//     let target_bb_handle = block::create_basic_block(&pool, context_handle, builder_handle, "target").expect("Failed to create target block");

//     let branch_handle = block::create_br(&pool, builder_handle, target_bb_handle);
//     assert!(branch_handle.is_some(), "Unconditional branch should be created successfully");
// }

// #[test]
// fn test_insert_before_basic_block() {
//     let pool = init::create_llvm_resource_pool();
//     let context_handle = init::create_context(&pool).expect("Failed to create context");
//     let target_bb_handle = block::create_basic_block(&pool, context_handle, builder_handle, "target").expect("Failed to create target block");
//     let insert_before_handle = block::insert_before_basic_block(&pool, context_handle, target_bb_handle, "insert_before");
    
//     assert!(insert_before_handle.is_some(), "Insert before block should be created successfully");
// }

// #[test]
// fn test_position_builder() {
//     let pool = init::create_llvm_resource_pool();
//     let context_handle = init::create_context(&pool).expect("Failed to create context");
//     let builder_handle = builder::create_builder(&pool, context_handle).expect("Failed to create builder");
//     let bb_handle = block::create_basic_block(&pool, context_handle, builder_handle, "position_here").expect("Failed to create block");

//     let result = block::position_builder(&pool, builder_handle, bb_handle);
//     assert!(result.is_some(), "Builder should be positioned at the end of the block successfully");
// }

// #[test]
// fn test_delete_basic_block() {
//     let pool = init::create_llvm_resource_pool();
//     let context_handle = init::create_context(&pool).expect("Failed to create context");
//     let bb_handle = block::create_basic_block(&pool, context_handle, builder_handle, "to_delete").expect("Failed to create block to delete");

//     let result = block::delete_basic_block(&pool, bb_handle);
//     assert!(result.is_some(), "Basic block should be deleted successfully");
// }

// #[test]
// fn test_get_first_instruction() {
//     let pool = init::create_llvm_resource_pool();
//     let context_handle = init::create_context(&pool).expect("Failed to create context");
//     let bb_handle = block::create_basic_block(&pool, context_handle, builder_handle, "has_instruction").expect("Failed to create block");

//     // Insert an instruction here for testing, e.g., a simple add or assignment

//     let instruction_handle = block::get_first_instruction(&pool, bb_handle);
//     assert!(instruction_handle.is_some(), "Should retrieve the first instruction in the block");
// }

// #[test]
// fn test_get_last_instruction() {
//     let pool = init::create_llvm_resource_pool();
//     let context_handle = init::create_context(&pool).expect("Failed to create context");
//     let bb_handle = block::create_basic_block(&pool, context_handle, builder_handle, "has_instruction").expect("Failed to create block");

//     // Insert an instruction here for testing, e.g., a simple add or assignment

//     let instruction_handle = block::get_last_instruction(&pool, bb_handle);
//     assert!(instruction_handle.is_some(), "Should retrieve the last instruction in the block");
// }

// #[test]
// fn test_create_phi() {
//     let pool = init::create_llvm_resource_pool();
//     let context_handle = init::create_context(&pool).expect("Failed to create context");
//     let builder_handle = builder::create_builder(&pool, context_handle).expect("Failed to create builder");
//     let value_handle1 = values::create_integer(&pool, 1, context_handle).expect("Failed to create first possible value");
//     let value_handle2 = values::create_integer(&pool, 2, context_handle).expect("Failed to create second possible value");
//     let block_handle1 = block::create_basic_block(&pool, context_handle, builder_handle, "block1").expect("Failed to create first block");
//     let block_handle2 = block::create_basic_block(&pool, context_handle, builder_handle, "block2").expect("Failed to create second block");
//     let possible_values = vec![(value_handle1, block_handle1), (value_handle2, block_handle2)];

//     let phi_handle = block::create_phi(&pool, builder_handle, &possible_values, "phi_node");
//     assert!(phi_handle.is_some(), "PHI node should be created successfully");
// }
