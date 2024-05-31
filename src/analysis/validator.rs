//! This module provides functionality for checking the correctness of LLVM modules and functions.

extern crate llvm_sys as llvm; 
use std::sync::{Arc, RwLock}; 
use llvm::{analysis, core}; 
use crate::memory_management::pointer::{SafeLLVMPointer, LLVMRef, LLVMRefType}; 

/// A Validator struct that encapsulates an LLVM module within a thread-safe, reference-counted pointer.
pub struct Validator {
    module: Arc<RwLock<SafeLLVMPointer>>, // Encapsulated LLVM module pointer.
}

impl Validator {
    /// Constructs a new `Validator`.
    ///
    /// # Parameters
    /// * `module` - An `Arc<RwLock<SafeLLVMPointer>>` pointing to the LLVM module to be checkd.
    ///
    /// # Returns
    /// A new instance of `Validator`.
    pub fn new(module: Arc<RwLock<SafeLLVMPointer>>) -> Self {
        Self { module }
    }

    /// Retrieves a shared reference to the encapsulated module.
    ///
    /// # Returns
    /// A shared reference to the Arc<RwLock<SafeLLVMPointer>> of the module.
    pub fn get_module(&self) -> &Arc<RwLock<SafeLLVMPointer>> {
       &self.module
    }

    /// Checks the entire LLVM module for correctness.
    ///
    /// This function uses LLVM's verification function to check the module and prints an error message if validation fails.
    ///
    /// # Returns
    /// True if the module is valid, false otherwise.
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

    /// Checks a specific function within the module for correctness.
    ///
    /// This function reads the function from its encapsulated pointer and Checks it using LLVM's function verification API.
    ///
    /// # Parameters
    /// * `function` - An Arc<RwLock<SafeLLVMPointer>> pointing to the LLVM function to be checkd.
    ///
    /// # Returns
    /// True if the function is valid, false otherwise.
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