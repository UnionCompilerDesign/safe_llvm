extern crate llvm_sys as llvm;

use llvm::{core, prelude::{LLVMBuilderRef, LLVMTypeRef}};

use crate::memory_management::{pointer::{LLVMRef, LLVMRefType, SafeLLVMError}, resource_pools::{BuilderTag, ContextTag, ResourcePools, TypeTag}};

impl ResourcePools {
    /// Allocates a builder in a specified context and stores it in the resource pool.
    pub fn create_builder(&mut self, context_tag: ContextTag) -> Result<BuilderTag, SafeLLVMError> {
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
                let builder_ptr: LLVMBuilderRef = unsafe {
                    core::LLVMCreateBuilderInContext(context_ptr)
                };
    
                if builder_ptr.is_null() {
                    Err(SafeLLVMError::InvalidPointer("Failed to create LLVM builder".into()))
                } else {
                    self.store_builder(builder_ptr)
                        .map_err(|e| e.into())
                }
            },
            Ok(None) => Err(SafeLLVMError::IncorrectPointerType("Expected a context type but got a different type".into())),
            Err(e) => Err(e)
        }
    }

    /// Allocates a function with specified return and parameter types in a given context, then stores it in the resource pool.
    pub fn create_function(
        &mut self,
        return_type_tag: Option<TypeTag>,
        param_type_tags: &[TypeTag],
        is_var_arg: bool,
        context_tag: ContextTag,
    ) -> Result<TypeTag, SafeLLVMError> {
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

        let llvm_return_type = match context_ptr {
            Ok(Some(context_ptr)) => {
                match return_type_tag {
                    Some(tag) => {
                        let type_arc_rwlock = self.get_type(tag)
                            .ok_or(SafeLLVMError::InvalidPointer("Return type tag not found".into()))?;
                        let type_ptr = type_arc_rwlock.read().map_err(|e| SafeLLVMError::from(e))?.read(LLVMRefType::Type, |type_ref| {
                            if let LLVMRef::Type(ptr) = type_ref {
                                Some(*ptr)
                            } else {
                                None
                            }
                        })?;
                        type_ptr
                    },
                    None => Some(unsafe { core::LLVMVoidTypeInContext(context_ptr) }),
                }
            },
            Ok(None) => return Err(SafeLLVMError::IncorrectPointerType("Expected a context type but got a different type".into())),
            Err(e) => return Err(e),
        }.expect("Failed to get return type");

        let mut llvm_param_types: Vec<LLVMTypeRef> = Vec::new();
        for tag in param_type_tags {
            let type_arc_rwlock = self.get_type(*tag)
                .ok_or(SafeLLVMError::InvalidPointer("Parameter type tag not found".into()))?;
            let type_ptr = type_arc_rwlock.read().map_err(|e| SafeLLVMError::from(e))?.read(LLVMRefType::Type, |type_ref| {
                if let LLVMRef::Type(ptr) = type_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?.expect("Failed to get type pointer");
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
            return Err(SafeLLVMError::InvalidPointer("Failed to create LLVM function type".into()));
        }

        self.store_type(function_type)
            .map_err(|e| e.into())
    }
}
