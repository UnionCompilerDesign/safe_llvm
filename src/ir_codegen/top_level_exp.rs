extern crate llvm_sys as llvm;

use std::ffi::CString;

use llvm::{core, prelude::{LLVMModuleRef, LLVMTypeRef, LLVMValueRef}};

/// Gets the parameter of a function
pub fn get_param(function: *mut llvm::LLVMValue, index: u32) -> *mut llvm::LLVMValue{
    unsafe {
        core::LLVMGetParam(function, index)
    }
}

/// adds a function to a module
pub fn add_function_to_module(module: LLVMModuleRef, function_name: &str, function_type: LLVMTypeRef) -> LLVMValueRef {
    let c_name = CString::new(function_name).expect("Failed to create CString for function name");
    unsafe {
        core::LLVMAddFunction(module, c_name.as_ptr(), function_type)
    }
}