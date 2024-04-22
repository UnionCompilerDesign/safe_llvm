extern crate llvm_sys as llvm;

use std::ffi::CString;

use llvm::{core, prelude::{LLVMContextRef, LLVMModuleRef}};

use crate::memory_management::pointer::CPointer;

/// Initializes a context
pub fn create_context() -> CPointer<LLVMContextRef> {
    let raw_ptr = unsafe { core::LLVMContextCreate() };

    if raw_ptr.is_null() {
        panic!("Failed to create LLVM context; the returned pointer is null.");
    }

    CPointer::new(raw_ptr as *mut LLVMContextRef).expect("Failed to wrap the LLVM context in CPointer.")
}

/// Initializes a module in the specified LLVM context
pub fn create_module(module_name: &str, context: CPointer<LLVMContextRef>) -> CPointer<LLVMModuleRef> {
    if module_name.is_empty() {
        panic!("Module name cannot be empty.");
    }

    let c_module_name = CString::new(module_name).expect("Failed to create CString from module name");

    let context_ptr = context.get_ref();
    if context_ptr.is_null() {
        panic!("Context CPointer is null, which indicates an invalid pointer handling.");
    }

    let raw_ptr = unsafe { core::LLVMModuleCreateWithNameInContext(c_module_name.as_ptr(), *context_ptr ) };

    if raw_ptr.is_null() {
        panic!("Failed to create LLVM module; the returned pointer is null.");
    }

    CPointer::new(raw_ptr as *mut LLVMModuleRef).expect("Failed to wrap the LLVM module in CPointer.")
}