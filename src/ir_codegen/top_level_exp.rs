extern crate llvm_sys as llvm;

use std::ffi::CString;

use llvm::{core, prelude::{LLVMModuleRef, LLVMTypeRef, LLVMValueRef}};

use crate::memory_management::ir_pointer::IRPointer;

/// Gets the parameter of a function
pub fn get_param(function: *mut llvm::LLVMValue, index: u32) -> IRPointer<LLVMValueRef> {
    let raw_ptr = unsafe {
        core::LLVMGetParam(function, index)
    };
    IRPointer::new(Some(raw_ptr as *mut _))
}

/// adds a function to a module
pub fn add_function_to_module(module: LLVMModuleRef, function_name: &str, function_type: LLVMTypeRef) -> IRPointer<LLVMValueRef> {
    let c_name = CString::new(function_name).expect("Failed to create CString for function name");
    let raw_ptr = unsafe {
        core::LLVMAddFunction(module, c_name.as_ptr(), function_type)
    };
    IRPointer::new(Some(raw_ptr as *mut _))
}
