extern crate llvm_sys as llvm;

use llvm::{core, prelude::*};

use crate::memory_management::ir_pointer::IRPointer; 

/// Creates a builder in context
pub fn create_builder(context: LLVMContextRef) -> IRPointer<LLVMBuilderRef> {
    let raw_ptr = unsafe {
        core::LLVMCreateBuilderInContext(context)
    };
    IRPointer::new(Some(raw_ptr as *mut _))
}
