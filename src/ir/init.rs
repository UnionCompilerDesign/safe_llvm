extern crate llvm_sys as llvm;

use std::ffi::CString;

use llvm::{core, prelude::{LLVMContextRef, LLVMModuleRef}};

use crate::memory_management::{
    pointer::{LLVMRef, LLVMRefType, SafeLLVMError}, 
    resource_pools::{ContextTag, ModuleTag, ResourcePools}
};

impl ResourcePools {
    /// Allocates a new LLVM context and stores it in the resource pool.
    pub fn create_context(&mut self) -> Result<ContextTag, SafeLLVMError> {
        let raw_ptr: LLVMContextRef = unsafe { core::LLVMContextCreate() };

        if raw_ptr.is_null() {
            return Err(SafeLLVMError::InvalidPointer("Failed to create LLVM context".into()));
        }

        self.store_context(raw_ptr)
            .map_err(|e| e.into())
    }

    /// Allocates a new LLVM module in a specified context and stores it in the resource pool.
    pub fn create_module(&mut self, module_name: &str, context_tag: ContextTag) -> Result<ModuleTag, SafeLLVMError> {
        let c_module_name = CString::new(module_name).expect("Failed to create CString from module name");

        let context_arc_rwlock = self.get_context(context_tag)
            .ok_or(SafeLLVMError::InvalidPointer("Context tag not found".into()))?;
        
        let context_rwlock = context_arc_rwlock.read().map_err(|e| SafeLLVMError::from(e))?;
        
        let context_ptr = context_rwlock.read(LLVMRefType::Context, |context_ref| {
            if let LLVMRef::Context(ptr) = context_ref {
                Some(*ptr)
            } else {
                None
            }
        });

        match context_ptr {
            Ok(Some(context_ptr)) => {
                let module_ptr: LLVMModuleRef = unsafe {
                    core::LLVMModuleCreateWithNameInContext(c_module_name.as_ptr(), context_ptr)
                };
    
                if module_ptr.is_null() {
                    Err(SafeLLVMError::InvalidPointer("Failed to create LLVM module".into()))
                } else {
                    self.store_module(module_ptr)
                        .map_err(|e| e.into())
                }
            },
            Ok(None) => Err(SafeLLVMError::IncorrectPointerType("Expected a context type but got a different type".into())),
            Err(e) => Err(e)
        }
    }
}

