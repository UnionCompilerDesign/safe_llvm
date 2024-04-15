use std::ffi::CString;

use llvm::{core, prelude::{LLVMContextRef, LLVMModuleRef}};

extern crate llvm_sys as llvm;


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