//! This module provides functionality for handling variables within the IR generator.
//!
//! It includes operations for initializing, reassigning, and retrieving variables as well as managing their storage and retrieval in memory.

extern crate llvm_sys as llvm;

use llvm::core;
use std::ffi::CString;
use common::pointer::{LLVMRef, LLVMRefType};
use super::core::{BuilderTag, IRManager, TypeTag, ValueTag};

impl IRManager {
    /// Initializes a variable with an optional initial value in the specified builder context.
    ///
    /// # Parameters
    /// * `builder_tag` - Tag of the builder to use for variable initialization.
    /// * `var_name` - Name of the variable.
    /// * `data_type_tag` - Type tag of the variable.
    /// * `initial_value_tag` - Optional tag of the initial value to assign to the variable.
    ///
    /// # Returns
    /// Option<ValueTag> - The tag of the allocated variable or None if the initialization fails.
    pub fn init_var(
        &mut self,
        builder_tag: BuilderTag, 
        var_name: &str, 
        data_type_tag: TypeTag, 
        initial_value_tag: Option<ValueTag>
    ) -> Option<ValueTag> {
        let builder_arc_rwlock = self.get_builder(builder_tag)?;
        let data_type_arc_rwlock = self.get_type(data_type_tag)?;

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
            
            core::LLVMBuildAlloca(builder_ptr, data_type_ptr, var_name_cstr.as_ptr()) //problem is HERE
        };

        if alloca.is_null() {
            None
        } else {
            let alloca_tag = self.store_value(alloca)?;

            if let Some(value_tag) = initial_value_tag {
                let value_arc_rwlock = self.get_value(value_tag)?;
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

            Some(alloca_tag)
        }
    }

    /// Reassigns a new value to an existing variable.
    ///
    /// # Parameters
    /// * `builder_tag` - Tag of the builder used for variable assignment.
    /// * `variable_alloc_tag` - Tag of the variable allocation to be reassigned.
    /// * `new_value_tag` - Tag of the new value to assign.
    ///
    /// # Returns
    /// Option<()> - None if the reassignment is successful, or an error if it fails.
    pub fn reassign_var(
        &mut self,
        builder_tag: BuilderTag, 
        variable_alloc_tag: ValueTag, 
        new_value_tag: ValueTag
    ) -> Option<()> {
        let builder_arc_rwlock = self.get_builder(builder_tag)?;
        let variable_alloc_arc_rwlock = self.get_value(variable_alloc_tag)?;
        let new_value_arc_rwlock = self.get_value(new_value_tag)?;

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

    /// Retrieves a variable's current value.
    ///
    /// # Parameters
    /// * `builder_tag` - Tag of the builder to use for retrieving the variable.
    /// * `variable_type_tag` - Type tag of the variable being retrieved.
    /// * `variable_alloc_tag` - Tag of the variable allocation from which to load the value.
    ///
    /// # Returns
    /// Option<ValueTag> - The tag of the loaded value or None if the retrieval fails.
    pub fn get_var(
        &mut self,
        builder_tag: BuilderTag, 
        variable_type_tag: TypeTag, 
        variable_alloc_tag: ValueTag,
        temp_name: &str
    ) -> Option<ValueTag> {
        let builder_arc_rwlock = self.get_builder(builder_tag)?;
        let variable_type_arc_rwlock = self.get_type(variable_type_tag)?;
        let variable_alloc_arc_rwlock = self.get_value(variable_alloc_tag)?;

        let tmp_load_cstr = CString::new(temp_name).expect("Failed to create CString for tmpload");

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