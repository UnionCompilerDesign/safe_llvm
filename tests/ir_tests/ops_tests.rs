// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::sync::{Arc, Mutex};
//     use safe_llvm::{memory_management::resource_pools::Handle, init};

//     // Helper function to setup a basic environment with a builder and two integer parameters
//     fn setup_env_with_ints() -> (Arc<Mutex<ResourcePools>>, Handle, Handle, Handle) {
//         let pool = init::create_llvm_resource_pool();
//         let context_handle = init::create_context(&pool).expect("Failed to create context");
//         let builder_handle = builder::create_builder(&pool, context_handle).expect("Failed to create builder");
//         let int_type_handle = values::create_integer(&pool, 32, context_handle).expect("Failed to create integer type");
        
//         let param_a_handle = values::create_value(&pool, int_type_handle, "a").expect("Failed to create param a");
//         let param_b_handle = values::create_value(&pool, int_type_handle, "b").expect("Failed to create param b");

//         (pool, builder_handle, param_a_handle, param_b_handle)
//     }

//     #[test]
//     fn test_build_add() {
//         let (pool, builder_handle, param_a_handle, param_b_handle) = setup_env_with_ints();
//         let result_handle = build_add(&pool, builder_handle, param_a_handle, param_b_handle, "add_result");
//         assert!(result_handle.is_some(), "Add operation should produce a result");
//     }

//     #[test]
//     fn test_build_sub() {
//         let (pool, builder_handle, param_a_handle, param_b_handle) = setup_env_with_ints();
//         let result_handle = build_sub(&pool, builder_handle, param_a_handle, param_b_handle, "sub_result");
//         assert!(result_handle.is_some(), "Subtraction operation should produce a result");
//     }

//     #[test]
//     fn test_build_mul() {
//         let (pool, builder_handle, param_a_handle, param_b_handle) = setup_env_with_ints();
//         let result_handle = build_mul(&pool, builder_handle, param_a_handle, param_b_handle, "mul_result");
//         assert!(result_handle.is_some(), "Multiplication operation should produce a result");
//     }

//     #[test]
//     fn test_build_div() {
//         let (pool, builder_handle, param_a_handle, param_b_handle) = setup_env_with_ints();
//         let result_handle = build_div(&pool, builder_handle, param_a_handle, param_b_handle, "div_result");
//         assert!(result_handle.is_some(), "Division operation should produce a result");
//     }

//     #[test]
//     fn test_build_rem() {
//         let (pool, builder_handle, param_a_handle, param_b_handle) = setup_env_with_ints();
//         let result_handle = build_rem(&pool, builder_handle, param_a_handle, param_b_handle, "rem_result");
//         assert!(result_handle.is_some(), "Remainder operation should produce a result");
//     }

//     #[test]
//     fn test_build_and() {
//         let (pool, builder_handle, param_a_handle, param_b_handle) = setup_env_with_ints();
//         let result_handle = build_and(&pool, builder_handle, param_a_handle, param_b_handle, "and_result");
//         assert!(result_handle.is_some(), "AND operation should produce a result");
//     }

//     #[test]
//     fn test_build_or() {
//         let (pool, builder_handle, param_a_handle, param_b_handle) = setup_env_with_ints();
//         let result_handle = build_or(&pool, builder_handle, param_a_handle, param_b_handle, "or_result");
//         assert!(result_handle.is_some(), "OR operation should produce a result");
//     }

//     #[test]
//     fn test_build_xor() {
//         let (pool, builder_handle, param_a_handle, param_b_handle) = setup_env_with_ints();
//         let result_handle = build_xor(&pool, builder_handle, param_a_handle, param_b_handle, "xor_result");
//         assert!(result_handle.is_some(), "XOR operation should produce a result");
//     }

//     #[test]
//     fn test_build_shl() {
//         let (pool, builder_handle, param_a_handle, param_b_handle) = setup_env_with_ints();
//         let result_handle = build_shl(&pool, builder_handle, param_a_handle, param_b_handle, "shl_result");
//         assert!(result_handle.is_some(), "Shift left operation should produce a result");
//     }

//     #[test]
//     fn test_build_shr() {
//         let (pool, builder_handle, param_a_handle, param_b_handle) = setup_env_with_ints();
//         let result_handle = build_shr(&pool, builder_handle, param_a_handle, param_b_handle, "shr_result");
//         assert!(result_handle.is_some(), "Shift right operation should produce a result");
//     }

//     #[test]
//     fn test_build_icmp_gt() {
//         let (pool, builder_handle, param_a_handle, param_b_handle) = setup_env_with_ints();
//         let result_handle = build_icmp_gt(&pool, builder_handle, param_a_handle, param_b_handle, "gt_result");
//         assert!(result_handle.is_some(), "Greater than comparison should produce a result");
//     }

//     #[test]
//     fn test_build_icmp_lt() {
//         let (pool, builder_handle, param_a_handle, param_b_handle) = setup_env_with_ints();
//         let result_handle = build_icmp_lt(&pool, builder_handle, param_a_handle, param_b_handle, "lt_result");
//         assert!(result_handle.is_some(), "Less than comparison should produce a result");
//     }

//     #[test]
//     fn test_build_icmp_eq() {
//         let (pool, builder_handle, param_a_handle, param_b_handle) = setup_env_with_ints();
//         let result_handle = build_icmp_eq(&pool, builder_handle, param_a_handle, param_b_handle, "eq_result");
//         assert!(result_handle.is_some(), "Equal comparison should produce a result");
//     }

//     #[test]
//     fn test_build_negation() {
//         let (pool, builder_handle, param_a_handle, _) = setup_env_with_ints();
//         let result_handle = build_negation(&pool, builder_handle, param_a_handle, "neg_result");
//         assert!(result_handle.is_some(), "Negation operation should produce a result");
//     }

//     #[test]
//     fn test_build_bitwise_not() {
//         let (pool, builder_handle, param_a_handle, _) = setup_env_with_ints();
//         let result_handle = build_bitwise_not(&pool, builder_handle, param_a_handle, "not_result");
//         assert!(result_handle.is_some(), "Bitwise NOT operation should produce a result");
//     }

//     #[test]
//     fn test_build_logical_not() {
//         let (pool, builder_handle, context_handle, param_a_handle) = setup_env_with_ints();
//         let result_handle = build_logical_not(&pool, builder_handle, context_handle, param_a_handle, "logical_not_result");
//         assert!(result_handle.is_some(), "Logical NOT operation should produce a result");
//     }
// }
