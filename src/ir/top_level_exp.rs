extern crate llvm_sys as llvm;

use llvm::core;

use std::{collections::HashMap, ffi::CString};

use crate::memory_management::{definitions::EnumDefinition, pointer::{LLVMRef, LLVMRefType}, resource_pools::{ContextTag, ModuleTag, ResourcePools, TypeTag, ValueTag}};

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

    /// Creates an enum type represented by an integer of specified bit width and associated variants.
    /// Each variant is internally mapped to an integer value starting from 0.
    pub fn create_enum(&mut self, context_tag: ContextTag, num_bits: u32, name: &str, variants: &[String]) -> Option<TypeTag> {
        let context_arc_rwlock = self.get_context(context_tag)?;
        let context_ptr = {
            let context_rwlock = context_arc_rwlock.read().expect("Failed to lock context for reading");
            context_rwlock.read(LLVMRefType::Context, |context_ref| {
                if let LLVMRef::Context(ptr) = context_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?
        };

        let enum_type_ptr = unsafe { llvm::core::LLVMIntTypeInContext(context_ptr, num_bits) };
        
        if enum_type_ptr.is_null() {
            None
        } else {
            let type_tag = self.store_type(enum_type_ptr).expect("Failed to store type tag");
            let mut variant_map = HashMap::new();

            for (index, variant) in variants.iter().enumerate() {
                variant_map.insert(variant.clone(), index as i64);
            }

            self.store_enum_definition(type_tag, EnumDefinition::new(name.to_string(), variant_map));


            Some(type_tag)
        }
    }
}