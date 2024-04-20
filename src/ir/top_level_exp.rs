extern crate llvm_sys as llvm;

use std::ffi::CString;

use llvm::{core, prelude::{LLVMModuleRef, LLVMTypeRef, LLVMValueRef}};

use crate::memory_management::pointer::CPointer;

/// Gets the parameter of a function
pub fn get_param(function: CPointer<LLVMValueRef>, index: u32) -> CPointer<LLVMValueRef> {
    let function_ptr = function.get_ref();
    let raw_ptr = unsafe {
        core::LLVMGetParam(*function_ptr, index)
    };
    let c_pointer = CPointer::new(raw_ptr as *mut _);
    if c_pointer.is_some() {
        return c_pointer.unwrap();
    }
    panic!("Missing c_pointer")}

/// Adds a function to a module
pub fn add_function_to_module(module: CPointer<LLVMModuleRef>, function_name: &str, function_type: CPointer<LLVMTypeRef>) -> CPointer<LLVMValueRef> {
    let c_name = CString::new(function_name).expect("Failed to create CString for function name");
    let module_ptr = module.get_ref();
    let function_type_ptr = function_type.get_ref();
    let raw_ptr = unsafe {
        core::LLVMAddFunction(*module_ptr, c_name.as_ptr(), *function_type_ptr)
    };
    let c_pointer = CPointer::new(raw_ptr as *mut _);
    if c_pointer.is_some() {
        return c_pointer.unwrap();
    }
    panic!("Missing c_pointer")}
