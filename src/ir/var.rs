extern crate llvm_sys as llvm;

use llvm::{core, LLVMBasicBlock, LLVMBuilder, LLVMContext, LLVMModule, LLVMType, LLVMValue};

use std::{ffi::CString, sync::{Arc, Mutex}};

use crate::memory_management::resource_pools::{Handle, ResourcePools};

/// Initializes a variable
pub fn init_var(
    pool: &Arc<Mutex<ResourcePools<LLVMContext, LLVMModule, LLVMValue, LLVMBasicBlock, LLVMBuilder, LLVMType>>>,    
    builder_handle: Handle, 
    var_name: &str, 
    data_type_handle: Handle, 
    initial_value_handle: Option<Handle>
) -> Option<Handle> {
    let pool_guard = pool.lock().unwrap();
    let builder = pool_guard.get_builder(builder_handle)?;
    let data_type = pool_guard.get_type(data_type_handle)?;
    drop(pool_guard);

    let var_name_cstr = CString::new(var_name).expect("Failed to create CString from var_name");

    let alloca = unsafe {
        builder.read().unwrap().use_ref(|builder_ptr| {
            data_type.read().unwrap().use_ref(|data_type_ptr| {
                core::LLVMBuildAlloca(builder_ptr, data_type_ptr, var_name_cstr.as_ptr())
            })
        })
    };

    if alloca.is_null() {
        None
    } else {
        let mut pool_guard = pool.lock().unwrap();
        let alloca_handle = pool_guard.create_value(alloca)?;

        if let Some(value_handle) = initial_value_handle {
            let value = pool_guard.get_value(value_handle)?;
            unsafe {
                value.read().unwrap().use_ref(|value_ptr| {
                    builder.read().unwrap().use_ref(|builder_ptr| {
                        core::LLVMBuildStore(builder_ptr, value_ptr, alloca);
                    });
                });
            }
        }
        Some(alloca_handle)
    }
}

/// Reassigns a variable
pub fn reassign_var(
    pool: &Arc<Mutex<ResourcePools<LLVMContext, LLVMModule, LLVMValue, LLVMBasicBlock, LLVMBuilder, LLVMType>>>,    
    builder_handle: Handle, 
    variable_alloc_handle: Handle, 
    new_value_handle: Handle
) -> Option<()> {
    let pool_guard = pool.lock().unwrap();
    let builder = pool_guard.get_builder(builder_handle)?;
    let variable_alloc = pool_guard.get_value(variable_alloc_handle)?;
    let new_value = pool_guard.get_value(new_value_handle)?;

    unsafe {
        variable_alloc.read().unwrap().use_ref(|variable_alloc_ptr| {
            new_value.read().unwrap().use_ref(|new_value_ptr| {
                builder.read().unwrap().use_ref(|builder_ptr| {
                    core::LLVMBuildStore(builder_ptr, new_value_ptr, variable_alloc_ptr);
                });
            });
        });
    }
    Some(())
}

/// Gets a variable
pub fn get_var(
    pool: &Arc<Mutex<ResourcePools<LLVMContext, LLVMModule, LLVMValue, LLVMBasicBlock, LLVMBuilder, LLVMType>>>,    
    builder_handle: Handle, 
    variable_type_handle: Handle, 
    variable_alloc_handle: Handle
) -> Option<Handle> {
    let pool_guard = pool.lock().unwrap();
    let builder = pool_guard.get_builder(builder_handle)?;
    let variable_type = pool_guard.get_type(variable_type_handle)?;
    let variable_alloc = pool_guard.get_value(variable_alloc_handle)?;
    drop(pool_guard);

    let tmp_load_cstr = CString::new("tmpload").expect("Failed to create CString for tmpload");

    let raw_ptr = unsafe {
        variable_alloc.read().unwrap().use_ref(|variable_alloc_ptr| {
            variable_type.read().unwrap().use_ref(|variable_type_ptr| {
                builder.read().unwrap().use_ref(|builder_ptr| {
                    core::LLVMBuildLoad2(builder_ptr, variable_type_ptr, variable_alloc_ptr, tmp_load_cstr.as_ptr())
                })
            })
        })
    };

    if raw_ptr.is_null() {
        None
    } else {
        let mut pool_guard = pool.lock().unwrap();
        pool_guard.create_value(raw_ptr)
    }
}
