// extern crate llvm_sys as llvm;

// use llvm::prelude::{LLVMBasicBlockRef, LLVMBuilderRef, LLVMContextRef, LLVMValueRef};
// use mockall;

// use safe_llvm::{
//     interface::LLVMApi, 
//     memory_management::ir_pointer::IRPointer
// };
// mockall::mock! {
//     BlockLLVMApi {}
//     impl LLVMApi for BlockLLVMApi {
//         fn create_basic_block(&self, context: LLVMContextRef, function: LLVMValueRef, name: &str) -> IRPointer<LLVMBasicBlockRef>;
//         fn get_current_block(&self, builder: *mut llvm::LLVMBuilder) -> IRPointer<LLVMBasicBlockRef>;
//         fn create_cond_br(&self, builder: LLVMBuilderRef, condition: LLVMValueRef, then_bb: LLVMBasicBlockRef, else_bb: LLVMBasicBlockRef) -> IRPointer<LLVMValueRef>;
//         fn create_br(&self, builder: LLVMBuilderRef, target_bb: LLVMBasicBlockRef) -> IRPointer<LLVMValueRef>;
//         fn insert_before_basic_block(&self, context: LLVMContextRef, before_target: LLVMBasicBlockRef, name: &str) -> IRPointer<LLVMBasicBlockRef>;
//         fn position_builder(&self, builder: *mut llvm::LLVMBuilder, bb: *mut llvm::LLVMBasicBlock);
//         fn delete_basic_block(&self, bb: LLVMBasicBlockRef);
//         fn get_first_instruction(&self, bb: LLVMBasicBlockRef) -> IRPointer<LLVMValueRef>;
//         fn get_last_instruction(&self, bb: LLVMBasicBlockRef) -> IRPointer<LLVMValueRef>;   
//         fn create_phi(&self, builder: LLVMBuilderRef, possible_values: &[(LLVMValueRef, LLVMBasicBlockRef)], name: &str) -> IRPointer<LLVMValueRef>; 
//     }
// }

// #[test]
// fn test_create_basic_block() {
//     let mut mock_api: MockBlockLLVMApi = MockBlockLLVMApi::new();
//     let raw_ptr: IRPointer<LLVMBasicBlockRef> = IRPointer::new(Some(std::ptr::null_mut()));

//     mock_api.expect_create_basic_block()
//             .returning( move |_, _, _| raw_ptr.clone()); 

//     let context: LLVMContextRef = std::ptr::null_mut();
//     let function: LLVMValueRef = std::ptr::null_mut();
//     let bb: IRPointer<LLVMBasicBlockRef> = (&mock_api).create_basic_block(context, function, "test");
//     assert!(bb.is_null(), "Expected null pointer return for basic block");
// }

// #[test]
// fn test_get_current_block() {
//     let mut mock_api: MockBlockLLVMApi = MockBlockLLVMApi::new();
//     let raw_ptr: IRPointer<LLVMBasicBlockRef> = IRPointer::new(Some(std::ptr::null_mut()));

//     mock_api.expect_get_current_block()
//             .returning(move |_| raw_ptr.clone());

//     let builder: *mut llvm::LLVMBuilder = std::ptr::null_mut();
//     let bb: IRPointer<LLVMBasicBlockRef> = (&mock_api).get_current_block(builder);
//     assert!(bb.is_null(), "Expected null pointer return for current block");
// }

// #[test]
// fn test_create_cond_br() {
//     let mut mock_api: MockBlockLLVMApi = MockBlockLLVMApi::new();
//     let raw_ptr: IRPointer<LLVMValueRef> = IRPointer::new(Some(std::ptr::null_mut()));

//     mock_api.expect_create_cond_br()
//             .returning(move |_, _, _, _| raw_ptr.clone());

//     let builder: LLVMBuilderRef = std::ptr::null_mut();
//     let condition: LLVMValueRef = std::ptr::null_mut();
//     let then_bb: LLVMBasicBlockRef = std::ptr::null_mut();
//     let else_bb: LLVMBasicBlockRef = std::ptr::null_mut();
//     let bb: IRPointer<LLVMValueRef> = (&mock_api).create_cond_br(builder, condition, then_bb, else_bb);
//     assert!(bb.is_null(), "Expected null pointer return for conditional branch");
// }

// #[test]
// fn test_create_br() {
//     let mut mock_api = MockBlockLLVMApi::new();
//     let raw_ptr = IRPointer::new(Some(std::ptr::null_mut()));

//     mock_api.expect_create_br()
//             .returning(move |_, _| raw_ptr.clone());

//     let builder = std::ptr::null_mut();
//     let target_bb = std::ptr::null_mut();
//     let result = (&mock_api).create_br(builder, target_bb);
//     assert!(result.is_null(), "Expected null pointer return for branch");
// }

// #[test]
// fn test_insert_before_basic_block() {
//     let mut mock_api = MockBlockLLVMApi::new();
//     let raw_ptr = IRPointer::new(Some(std::ptr::null_mut()));

//     mock_api.expect_insert_before_basic_block()
//             .returning(move |_, _, _| raw_ptr.clone());

//     let context = std::ptr::null_mut();
//     let before_target = std::ptr::null_mut();
//     let result = (&mock_api).insert_before_basic_block(context, before_target, "test");
//     assert!(result.is_null(), "Expected null pointer return for insert before basic block");
// }

// #[test]
// fn test_position_builder() {
//     let mut mock_api = MockBlockLLVMApi::new();

//     mock_api.expect_position_builder()
//             .returning(move |_, _| ());

//     let builder = std::ptr::null_mut();
//     let bb = std::ptr::null_mut();
//     (&mock_api).position_builder(builder, bb);
// }

// #[test]
// fn test_delete_basic_block() {
//     let mut mock_api = MockBlockLLVMApi::new();

//     mock_api.expect_delete_basic_block()
//             .returning(move |_| ());

//     let bb = std::ptr::null_mut();
//     (&mock_api).delete_basic_block(bb);
// }

// #[test]
// fn test_get_first_instruction() {
//     let mut mock_api = MockBlockLLVMApi::new();
//     let raw_ptr = IRPointer::new(Some(std::ptr::null_mut()));

//     mock_api.expect_get_first_instruction()
//             .returning(move |_| raw_ptr.clone());

//     let bb = std::ptr::null_mut();
//     let result = (&mock_api).get_first_instruction(bb);
//     assert!(result.is_null(), "Expected null pointer return for first instruction");
// }

// #[test]
// fn test_get_last_instruction() {
//     let mut mock_api = MockBlockLLVMApi::new();
//     let raw_ptr = IRPointer::new(Some(std::ptr::null_mut()));

//     mock_api.expect_get_last_instruction()
//             .returning(move |_| raw_ptr.clone());

//     let bb = std::ptr::null_mut();
//     let result = (&mock_api).get_last_instruction(bb);
//     assert!(result.is_null(), "Expected null pointer return for last instruction");
// }

// #[test]
// fn test_create_phi() {
//     let mut mock_api = MockBlockLLVMApi::new();
//     let raw_ptr = IRPointer::new(Some(std::ptr::null_mut()));

//     let values = &[];

//     mock_api.expect_create_phi()
//             .returning(move |_, _, _| raw_ptr.clone());

//     let builder = std::ptr::null_mut();
//     let result = (&mock_api).create_phi(builder, values, "phi");
//     assert!(result.is_null(), "Expected null pointer return for phi node");
// }
