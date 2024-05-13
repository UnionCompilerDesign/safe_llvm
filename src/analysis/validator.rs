extern crate llvm_sys as llvm;

use std::sync::{Arc, RwLock};

use llvm::{analysis, core};

use crate::memory_management::pointer::{SafeLLVMPointer, LLVMRef, LLVMRefType};

pub struct Validator {
    module: Arc<RwLock<SafeLLVMPointer>>,
}

impl Validator {
    // Constructs a new Validator
    pub fn new(module: Arc<RwLock<SafeLLVMPointer>>) -> Self {
        Self { module }
    }

    pub fn get_module(&self) -> &Arc<RwLock<SafeLLVMPointer>> {
       &self.module
    }

    // Validates an entire LLVM module
    pub fn is_valid_module(&self) -> bool {
        let mut error_message = std::ptr::null_mut();

        let action = analysis::LLVMVerifierFailureAction::LLVMPrintMessageAction;

        let module_rw_lock = self.module.read().expect("Failed to get module");

        let result = module_rw_lock.read(LLVMRefType::Module, |module_ref| {
            if let LLVMRef::Module(ptr) = module_ref {
                unsafe { analysis::LLVMVerifyModule(*ptr, action, &mut error_message) }
            } else {
                panic!("Incorrect pointer passed")
            }
        });
        
        if result == 0 {
            true
        } else {
            if !error_message.is_null() {
                let c_str =  unsafe { std::ffi::CStr::from_ptr(error_message) };
                let message = c_str.to_str().unwrap();
                eprintln!("Error validating module: {}", message);
                unsafe { core::LLVMDisposeMessage(error_message) }
            }
            false
        }
    }

    // Validates a specific function in the module
    pub fn is_valid_function(&self, function: Arc<RwLock<SafeLLVMPointer>>) -> bool {
        let action = analysis::LLVMVerifierFailureAction::LLVMPrintMessageAction;

        let function_rw_lock = function.read().expect("Failed to get function");

        let result = function_rw_lock.read(LLVMRefType::Value, |fn_ref| {
            if let LLVMRef::Value(ptr) = fn_ref {
                unsafe { analysis::LLVMVerifyFunction(*ptr, action) }
            }
            else {
                panic!("Incorrect pointer passed")
            }
        });

        if result == 0 {
            true
        } else {
            eprintln!("Function validation failed.");
            false
        }

    }
}
