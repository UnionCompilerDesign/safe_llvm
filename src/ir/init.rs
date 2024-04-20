extern crate llvm_sys as llvm;

use std::ffi::CString;

use llvm::{core, prelude::{LLVMContextRef, LLVMModuleRef}};

use crate::memory_management::pointer::CPointer;

/// Initializes a context
pub fn create_context() -> CPointer<LLVMContextRef> {
    let raw_ptr = unsafe {
        core::LLVMContextCreate()
    };
    
    CPointer::new(Some(raw_ptr as *mut _))
}

/// Initializes a module in the specified LLVM context
pub fn create_module(module_name: &str, context: CPointer<LLVMContextRef>) -> CPointer<LLVMModuleRef> {
    let c_module_name = CString::new(module_name).expect("Failed to create module name");

    let context_ptr = context.get_ref();
    if context_ptr.is_null() || unsafe { *context_ptr }.is_null() {
        panic!("Context pointer is null or points to null");
    }

    let raw_ptr = unsafe {
        core::LLVMModuleCreateWithNameInContext(
            c_module_name.as_ptr(),
            *context_ptr
        )
    };

    if raw_ptr.is_null() {
        panic!("Failed to create LLVM module");
    }

    CPointer::new(Some(raw_ptr as *mut _))
}