// extern crate llvm_sys as llvm;

// use std::ffi::CString;

// use llvm::{
//         core::{
//             LLVMBuildAlloca, LLVMBuildStore, LLVMBuildLoad2,
//         }, 
//         prelude::{
//             LLVMBuilderRef, LLVMTypeRef, LLVMValueRef,
//         }
// };
// use crate::memory_management::pointer::CPointer;

// /// Initializes a variable
// pub fn init_var(
//     builder: CPointer<LLVMBuilderRef>, 
//     var_name: &str, 
//     data_type: CPointer<LLVMTypeRef>, 
//     initial_value: Option<CPointer<LLVMValueRef>>
// ) -> CPointer<LLVMValueRef> {
//     let var_name_cstr = CString::new(var_name).expect("Failed to create CString from var_name");
//     let builder_ptr = builder.get_ref();
//     let data_type_ptr = data_type.get_ref();

//     let alloca = unsafe {
//         LLVMBuildAlloca(*builder_ptr, *data_type_ptr, var_name_cstr.as_ptr())
//     };

//     if let Some(value) = initial_value {
//         let value_ptr = value.get_ref();
//         unsafe {
//             LLVMBuildStore(*builder_ptr, *value_ptr, alloca);
//         }
//     }
//     let c_pointer = CPointer::new(alloca as *mut _);
//     if c_pointer.is_some() {
//         return c_pointer.unwrap();
//     }
//     panic!("Missing c_pointer")
// }

// /// Reassigns a variable
// pub fn reassign_var(
//     builder: CPointer<LLVMBuilderRef>, 
//     variable_alloc: CPointer<LLVMValueRef>, 
//     new_value: CPointer<LLVMValueRef>
// ) {
//     let builder_ptr = builder.get_ref();
//     let variable_alloc_ptr = variable_alloc.get_ref();
//     let new_value_ptr = new_value.get_ref();

//     unsafe {
//         LLVMBuildStore(*builder_ptr, *new_value_ptr, *variable_alloc_ptr);
//     }
// }

// /// Gets a variable
// pub fn get_var(
//     builder: CPointer<LLVMBuilderRef>, 
//     variable_type: CPointer<LLVMTypeRef>, 
//     variable_alloc: CPointer<LLVMValueRef>
// ) -> CPointer<LLVMValueRef> {
//     let builder_ptr = builder.get_ref();
//     let variable_type_ptr = variable_type.get_ref();
//     let variable_alloc_ptr = variable_alloc.get_ref();

//     let raw_ptr = unsafe {
//         LLVMBuildLoad2(*builder_ptr, *variable_type_ptr, *variable_alloc_ptr, CString::new("tmpload").expect("Failed to create CString for tmpload").as_ptr())
//     };
//     let c_pointer = CPointer::new(raw_ptr as *mut _);
//     if c_pointer.is_some() {
//         return c_pointer.unwrap();
//     }
//     panic!("Missing c_pointer")
// }
