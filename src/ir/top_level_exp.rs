extern crate llvm_sys as llvm;

use llvm::core;

use std::ffi::CString;

use crate::memory_management::{pointer::{LLVMRef, LLVMRefType}, resource_pools::{ModuleTag, ResourcePools, TypeTag, ValueTag}};

impl ResourcePools {
    /// Gets a parameter from a function by its index.
    pub fn get_param(&mut self, function_tag: ValueTag, index: u32) -> Option<ValueTag> {
        let function_arc_rwlock = self.get_value(function_tag)?;
        
        let param = {
            let function_rwlock = function_arc_rwlock.read().expect("Failed to lock function for reading");
            let function_ptr = function_rwlock.read(LLVMRefType::Value, |value_ref| {
                if let LLVMRef::Value(ptr) = value_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            unsafe { core::LLVMGetParam(function_ptr, index) }
        };

        if param.is_null() {
            None
        } else {
            self.store_value(param)
        }
    }

    /// Adds a function to a module. 
    pub fn add_function_to_module(&mut self, module_tag: ModuleTag, function_name: &str, function_type_tag: TypeTag) -> Option<ValueTag> {
        let module_arc_rwlock = self.get_module(module_tag)?;
        let function_type_arc_rwlock = self.get_type(function_type_tag)?;

        let c_name = CString::new(function_name).expect("Failed to create CString for function name");

        let function = {
            let module_rwlock = module_arc_rwlock.read().expect("Failed to lock module for reading");
            let module_ptr = module_rwlock.read(LLVMRefType::Module, |module_ref| {
                if let LLVMRef::Module(ptr) = module_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            let function_type_ptr = {
                let function_type_rwlock = function_type_arc_rwlock.read().expect("Failed to lock function type for reading");
                function_type_rwlock.read(LLVMRefType::Type, |type_ref| {
                    if let LLVMRef::Type(ptr) = type_ref {
                        Some(*ptr)
                    } else {
                        None
                    }
                })?
            };

            unsafe { core::LLVMAddFunction(module_ptr, c_name.as_ptr(), function_type_ptr) }
        };

        if function.is_null() {
            None
        } else {
            self.store_value(function)
        }
    }
}