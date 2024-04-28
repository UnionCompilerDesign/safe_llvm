extern crate llvm_sys as llvm;

use std::ffi::CString;

use llvm::{core, prelude::{LLVMContextRef, LLVMModuleRef}};

use crate::memory_management::{
    pointer::{LLVMRef, LLVMRefType}, 
    resource_pools::{ContextTag, ModuleTag, ResourcePools}
};

impl ResourcePools {
    /// Allocates a new LLVM context and stores it in the resource pool.
    pub fn create_context(&mut self) -> Option<ContextTag> {
        let raw_ptr: LLVMContextRef = unsafe { core::LLVMContextCreate() };

        if raw_ptr.is_null() {
            return None;
        }

        self.store_context(raw_ptr)
    }

    /// Allocates a new LLVM module in a specified context and stores it in the resource pool.
    pub fn create_module(&mut self, module_name: &str, context_tag: ContextTag) -> Option<ModuleTag> {
        let c_module_name: CString = CString::new(module_name).expect("Failed to create CString from module name");

        let context_arc_rwlock = self.get_context(context_tag)?;
        
        let context_rwlock = context_arc_rwlock.read().expect("Failed to lock context for reading");

        let context_ptr = context_rwlock.read(LLVMRefType::Context, |context_ref| {
            if let LLVMRef::Context(ptr) = context_ref {
                Some(*ptr)  
            } else {
                return None;
            }
        })?;

        let module_ptr: LLVMModuleRef = unsafe {
            core::LLVMModuleCreateWithNameInContext(c_module_name.as_ptr(), context_ptr) 
        }; 

        if module_ptr.is_null() {
            return None;
        }

        self.store_module(module_ptr)
    }
}

