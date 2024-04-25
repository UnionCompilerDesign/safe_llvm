extern crate llvm_sys as llvm;

use llvm::{core, prelude::{LLVMBuilderRef, LLVMTypeRef}};

use crate::memory_management::{resource_pools::{ResourcePools, ContextHandle, BuilderHandle, TypeHandle}, pointer::{LLVMRef, LLVMRefType}};

impl ResourcePools {
    /// Allocates a builder in a specified context and stores it in the resource pool.
    pub fn allocate_builder(&mut self, context_handle: ContextHandle) -> Option<BuilderHandle> {
        let context_arc_rwlock = self.get_context(context_handle)?;

        let builder_ptr: LLVMBuilderRef = unsafe {
            let context_rwlock = context_arc_rwlock.read().expect("Failed to lock context for reading");
            let context_ptr = context_rwlock.read(LLVMRefType::Context, |context_ref| {
                if let LLVMRef::Context(ptr) = context_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            core::LLVMCreateBuilderInContext(context_ptr)
        };

        if builder_ptr.is_null() {
            return None;
        }

        self.store_builder(builder_ptr)
    }

    /// Allocates a function with specified return and parameter types in a given context, then stores it in the resource pool.
    pub fn allocate_function(
        &mut self,
        return_type_handle: Option<TypeHandle>,
        param_type_handles: &[TypeHandle],
        is_var_arg: bool,
        context_handle: ContextHandle,
    ) -> Option<TypeHandle> {
        let context_arc_rwlock = self.get_context(context_handle)?;

        let context_ptr = context_arc_rwlock.read().expect("Failed to lock context for reading").read(LLVMRefType::Context, |context_ref| {
            if let LLVMRef::Context(ptr) = context_ref {
                Some(*ptr)
            } else {
                None
            }
        })?;

        let llvm_return_type = return_type_handle.map_or_else(|| unsafe { core::LLVMVoidTypeInContext(context_ptr) }, |handle| {
            let type_arc_rwlock = self.get_type(handle).expect("Failed to get type");
            let ptr = type_arc_rwlock.read().expect("Failed to lock type for reading").read(LLVMRefType::Type, |type_ref| {
                if let LLVMRef::Type(ptr) = type_ref {
                    Some(*ptr)
                } else {
                    None
                }
            }).expect("Failed to get return type"); 
            ptr
        });

        let mut llvm_param_types: Vec<LLVMTypeRef> = Vec::new();
        for handle in param_type_handles {
            let type_arc_rwlock = self.get_type(*handle)?;
            let type_ptr = type_arc_rwlock.read().expect("Failed to lock type for reading").read(LLVMRefType::Type, |type_ref| {
                if let LLVMRef::Type(ptr) = type_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;
            llvm_param_types.push(type_ptr);
        }

        let param_ptr = if llvm_param_types.is_empty() {
            std::ptr::null_mut()
        } else {
            llvm_param_types.as_mut_ptr()
        };

        let function_type = unsafe {
            core::LLVMFunctionType(llvm_return_type, param_ptr, llvm_param_types.len() as u32, is_var_arg as i32)
        };

        if function_type.is_null() {
            return None;
        }

        self.store_type(function_type)
    }
}
