extern crate llvm_sys as llvm;

use llvm::{core, prelude::*}; // TODO change to not use wild star import
use std::ffi::CString;

/// Creates a builder in context
pub fn create_builder(context: LLVMContextRef) -> LLVMBuilderRef {
    unsafe {
        core::LLVMCreateBuilderInContext(context)
    }
}