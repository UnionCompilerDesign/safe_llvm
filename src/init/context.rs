extern crate llvm_sys as llvm;

use llvm::{core, prelude::LLVMContextRef};

/// Initializes a context
pub fn create_context() -> LLVMContextRef {
    unsafe {
        core::LLVMContextCreate()
    }
}