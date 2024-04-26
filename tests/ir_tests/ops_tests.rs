// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::sync::{Arc, Mutex};
//     use safe_llvm::{memory_management::resource_pools::Tag, init};

//     // Helper function to setup a basic environment with a builder and two integer parameters
//     fn setup_env_with_ints() -> (Arc<Mutex<ResourcePools>>, Tag, Tag, Tag) {
//         let pool = init::create_llvm_resource_pool();
//         let context_tag = init::create_context(&pool).expect("Failed to create context");
//         let builder_tag = builder::create_builder(&pool, context_tag).expect("Failed to create builder");
//         let int_type_tag = values::create_integer(&pool, 32, context_tag).expect("Failed to create integer type");
        
//         let param_a_tag = values::create_value(&pool, int_type_tag, "a").expect("Failed to create param a");
//         let param_b_tag = values::create_value(&pool, int_type_tag, "b").expect("Failed to create param b");

//         (pool, builder_tag, param_a_tag, param_b_tag)
//     }

//     #[test]
//     fn test_build_add() {
//         let (pool, builder_tag, param_a_tag, param_b_tag) = setup_env_with_ints();
//         let result_tag = build_add(&pool, builder_tag, param_a_tag, param_b_tag, "add_result");
//         assert!(result_tag.is_some(), "Add operation should produce a result");
//     }

//     #[test]
//     fn test_build_sub() {
//         let (pool, builder_tag, param_a_tag, param_b_tag) = setup_env_with_ints();
//         let result_tag = build_sub(&pool, builder_tag, param_a_tag, param_b_tag, "sub_result");
//         assert!(result_tag.is_some(), "Subtraction operation should produce a result");
//     }

//     #[test]
//     fn test_build_mul() {
//         let (pool, builder_tag, param_a_tag, param_b_tag) = setup_env_with_ints();
//         let result_tag = build_mul(&pool, builder_tag, param_a_tag, param_b_tag, "mul_result");
//         assert!(result_tag.is_some(), "Multiplication operation should produce a result");
//     }

//     #[test]
//     fn test_build_div() {
//         let (pool, builder_tag, param_a_tag, param_b_tag) = setup_env_with_ints();
//         let result_tag = build_div(&pool, builder_tag, param_a_tag, param_b_tag, "div_result");
//         assert!(result_tag.is_some(), "Division operation should produce a result");
//     }

//     #[test]
//     fn test_build_rem() {
//         let (pool, builder_tag, param_a_tag, param_b_tag) = setup_env_with_ints();
//         let result_tag = build_rem(&pool, builder_tag, param_a_tag, param_b_tag, "rem_result");
//         assert!(result_tag.is_some(), "Remainder operation should produce a result");
//     }

//     #[test]
//     fn test_build_and() {
//         let (pool, builder_tag, param_a_tag, param_b_tag) = setup_env_with_ints();
//         let result_tag = build_and(&pool, builder_tag, param_a_tag, param_b_tag, "and_result");
//         assert!(result_tag.is_some(), "AND operation should produce a result");
//     }

//     #[test]
//     fn test_build_or() {
//         let (pool, builder_tag, param_a_tag, param_b_tag) = setup_env_with_ints();
//         let result_tag = build_or(&pool, builder_tag, param_a_tag, param_b_tag, "or_result");
//         assert!(result_tag.is_some(), "OR operation should produce a result");
//     }

//     #[test]
//     fn test_build_xor() {
//         let (pool, builder_tag, param_a_tag, param_b_tag) = setup_env_with_ints();
//         let result_tag = build_xor(&pool, builder_tag, param_a_tag, param_b_tag, "xor_result");
//         assert!(result_tag.is_some(), "XOR operation should produce a result");
//     }

//     #[test]
//     fn test_build_shl() {
//         let (pool, builder_tag, param_a_tag, param_b_tag) = setup_env_with_ints();
//         let result_tag = build_shl(&pool, builder_tag, param_a_tag, param_b_tag, "shl_result");
//         assert!(result_tag.is_some(), "Shift left operation should produce a result");
//     }

//     #[test]
//     fn test_build_shr() {
//         let (pool, builder_tag, param_a_tag, param_b_tag) = setup_env_with_ints();
//         let result_tag = build_shr(&pool, builder_tag, param_a_tag, param_b_tag, "shr_result");
//         assert!(result_tag.is_some(), "Shift right operation should produce a result");
//     }

//     #[test]
//     fn test_build_icmp_gt() {
//         let (pool, builder_tag, param_a_tag, param_b_tag) = setup_env_with_ints();
//         let result_tag = build_icmp_gt(&pool, builder_tag, param_a_tag, param_b_tag, "gt_result");
//         assert!(result_tag.is_some(), "Greater than comparison should produce a result");
//     }

//     #[test]
//     fn test_build_icmp_lt() {
//         let (pool, builder_tag, param_a_tag, param_b_tag) = setup_env_with_ints();
//         let result_tag = build_icmp_lt(&pool, builder_tag, param_a_tag, param_b_tag, "lt_result");
//         assert!(result_tag.is_some(), "Less than comparison should produce a result");
//     }

//     #[test]
//     fn test_build_icmp_eq() {
//         let (pool, builder_tag, param_a_tag, param_b_tag) = setup_env_with_ints();
//         let result_tag = build_icmp_eq(&pool, builder_tag, param_a_tag, param_b_tag, "eq_result");
//         assert!(result_tag.is_some(), "Equal comparison should produce a result");
//     }

//     #[test]
//     fn test_build_negation() {
//         let (pool, builder_tag, param_a_tag, _) = setup_env_with_ints();
//         let result_tag = build_negation(&pool, builder_tag, param_a_tag, "neg_result");
//         assert!(result_tag.is_some(), "Negation operation should produce a result");
//     }

//     #[test]
//     fn test_build_bitwise_not() {
//         let (pool, builder_tag, param_a_tag, _) = setup_env_with_ints();
//         let result_tag = build_bitwise_not(&pool, builder_tag, param_a_tag, "not_result");
//         assert!(result_tag.is_some(), "Bitwise NOT operation should produce a result");
//     }

//     #[test]
//     fn test_build_logical_not() {
//         let (pool, builder_tag, context_tag, param_a_tag) = setup_env_with_ints();
//         let result_tag = build_logical_not(&pool, builder_tag, context_tag, param_a_tag, "logical_not_result");
//         assert!(result_tag.is_some(), "Logical NOT operation should produce a result");
//     }
// }
