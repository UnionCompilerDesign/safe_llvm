extern crate llvm_sys as llvm;

use llvm::core;

use std::ffi::CString;

use crate::memory_management::{
    pointer::{LLVMRef, LLVMRefType}, 
    resource_pools::{BuilderHandle, ResourcePools, TypeHandle, ValueHandle}
};

impl ResourcePools {
    /// Initializes a variable
    pub fn init_var(
        &mut self,
        builder_handle: BuilderHandle, 
        var_name: &str, 
        data_type_handle: TypeHandle, 
        initial_value_handle: Option<ValueHandle>
    ) -> Option<ValueHandle> {
        let builder_arc_rwlock = self.get_builder(builder_handle)?;
        let data_type_arc_rwlock = self.get_type(data_type_handle)?;

        let var_name_cstr = CString::new(var_name).expect("Failed to create CString from var_name");

        let alloca = unsafe {
            let builder_ptr = builder_arc_rwlock.read().expect("Failed to lock builder for reading").read(LLVMRefType::Builder, |builder_ref| {
                if let LLVMRef::Builder(ptr) = builder_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            let data_type_ptr = data_type_arc_rwlock.read().expect("Failed to lock data type for reading").read(LLVMRefType::Type, |data_type_ref| {
                if let LLVMRef::Type(ptr) = data_type_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            core::LLVMBuildAlloca(builder_ptr, data_type_ptr, var_name_cstr.as_ptr())
        };

        if alloca.is_null() {
            None
        } else {
            let alloca_handle = self.store_value(alloca)?;

            if let Some(value_handle) = initial_value_handle {
                let value_arc_rwlock = self.get_value(value_handle)?;
                unsafe {
                    let value_ptr = value_arc_rwlock.read().expect("Failed to lock value for reading").read(LLVMRefType::Value, |value_ref| {
                        if let LLVMRef::Value(ptr) = value_ref {
                            Some(*ptr)
                        } else {
                            None
                        }
                    })?;
                    let builder_ptr = builder_arc_rwlock.read().expect("Failed to lock builder for reading").read(LLVMRefType::Builder, |builder_ref| {
                        if let LLVMRef::Builder(ptr) = builder_ref {
                            Some(*ptr)
                        } else {
                            None
                        }
                    })?;

                    core::LLVMBuildStore(builder_ptr, value_ptr, alloca);
                }
            }

            Some(alloca_handle)
        }
    }

    /// Reassigns a variable
    pub fn reassign_var(
        &mut self,
        builder_handle: BuilderHandle, 
        variable_alloc_handle: ValueHandle, 
        new_value_handle: ValueHandle
    ) -> Option<()> {
        let builder_arc_rwlock = self.get_builder(builder_handle)?;
        let variable_alloc_arc_rwlock = self.get_value(variable_alloc_handle)?;
        let new_value_arc_rwlock = self.get_value(new_value_handle)?;

        unsafe {
            let builder_ptr = builder_arc_rwlock.read().expect("Failed to lock builder for reading").read(LLVMRefType::Builder, |builder_ref| {
                if let LLVMRef::Builder(ptr) = builder_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            let variable_alloc_ptr = variable_alloc_arc_rwlock.read().expect("Failed to lock variable alloc for reading").read(LLVMRefType::Value, |variable_alloc_ref| {
                if let LLVMRef::Value(ptr) = variable_alloc_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            let new_value_ptr = new_value_arc_rwlock.read().expect("Failed to lock new value for reading").read(LLVMRefType::Value, |new_value_ref| {
                if let LLVMRef::Value(ptr) = new_value_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            core::LLVMBuildStore(builder_ptr, new_value_ptr, variable_alloc_ptr);
        }

        Some(())
    }

    /// Gets a variable
    pub fn get_var(
        &mut self,
        builder_handle: BuilderHandle, 
        variable_type_handle: TypeHandle, 
        variable_alloc_handle: ValueHandle
    ) -> Option<ValueHandle> {
        let builder_arc_rwlock = self.get_builder(builder_handle)?;
        let variable_type_arc_rwlock = self.get_type(variable_type_handle)?;
        let variable_alloc_arc_rwlock = self.get_value(variable_alloc_handle)?;

        let tmp_load_cstr = CString::new("tmpload").expect("Failed to create CString for tmpload");

        let raw_ptr = unsafe {
            let builder_ptr = builder_arc_rwlock.read().expect("Failed to lock builder for reading").read(LLVMRefType::Builder, |builder_ref| {
                if let LLVMRef::Builder(ptr) = builder_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            let variable_type_ptr = variable_type_arc_rwlock.read().expect("Failed to lock variable type for reading").read(LLVMRefType::Type, |variable_type_ref| {
                if let LLVMRef::Type(ptr) = variable_type_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            let variable_alloc_ptr = variable_alloc_arc_rwlock.read().expect("Failed to lock variable alloc for reading").read(LLVMRefType::Value, |variable_alloc_ref| {
                if let LLVMRef::Value(ptr) = variable_alloc_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            core::LLVMBuildLoad2(builder_ptr, variable_type_ptr, variable_alloc_ptr, tmp_load_cstr.as_ptr())
        };

        if raw_ptr.is_null() {
            None
        } else {
            self.store_value(raw_ptr)
        }
    }
}