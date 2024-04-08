extern crate llvm_sys as llvm;

use llvm::{core, prelude::*}; // TODO change to not use wild star import
use std::ffi::CString;

/// Initializes a context
pub fn create_context() -> LLVMContextRef {
    unsafe {
        core::LLVMContextCreate()
    }
}

/// Initializes a module
pub fn create_module(module_name: &str, context: LLVMContextRef) -> LLVMModuleRef {
    let c_module_name = CString::new(module_name).expect("Failed to create module name");
    unsafe {
        core::LLVMModuleCreateWithNameInContext(
            c_module_name.as_ptr(),
            context,
        )
    }
}

/// Creates a builder in context
pub fn create_builder(context: LLVMContextRef) -> LLVMBuilderRef {
    unsafe {
        core::LLVMCreateBuilderInContext(context)
    }
}

/// Creates a basic block in context
pub fn create_basic_block(context: LLVMContextRef, function: LLVMValueRef, name: &str) -> LLVMBasicBlockRef {
    let c_name = CString::new(name).expect("Failed to create basic block name");
    unsafe { 
        core::LLVMAppendBasicBlockInContext(context, function, c_name.as_ptr()) 
    }
}