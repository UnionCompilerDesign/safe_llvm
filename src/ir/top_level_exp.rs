extern crate llvm_sys as llvm;

use llvm::{core, LLVMBasicBlock, LLVMBuilder, LLVMContext, LLVMModule, LLVMType, LLVMValue};

use std::{ffi::CString, sync::{Arc, Mutex}};

use crate::memory_management::resource_pools::{Handle, LLVMResourcePools};

/// Gets the parameter of a function
pub fn get_param(pool: &Arc<Mutex<LLVMResourcePools<LLVMContext, LLVMModule, LLVMValue, LLVMBasicBlock, LLVMBuilder, LLVMType>>>, function_handle: Handle, index: u32) -> Option<Handle> {
    let pool_guard = pool.lock().unwrap();
    let function = pool_guard.get_value(function_handle)?;
    drop(pool_guard);

    let param = unsafe {
        function.read().unwrap().use_ref(|function_ptr| {
            core::LLVMGetParam(function_ptr, index)
        })
    };

    if param.is_null() {
        None
    } else {
        let mut pool_guard = pool.lock().unwrap();
        pool_guard.create_value(param)
    }
}

/// Adds a function to a module
pub fn add_function_to_module(
    pool: &Arc<Mutex<LLVMResourcePools<LLVMContext, LLVMModule, LLVMValue, LLVMBasicBlock, LLVMBuilder, LLVMType>>>,
    module_handle: Handle,
    function_name: &str,
    function_type_handle: Handle
) -> Option<Handle> {
    let pool_guard = pool.lock().unwrap();
    let module = pool_guard.get_module(module_handle)?;
    let function_type = pool_guard.get_type(function_type_handle)?;
    drop(pool_guard);

    if function_name.is_empty() {
        panic!("Function name is empty");
    }

    let c_name = CString::new(function_name).expect("Failed to create CString for function name");

    let function = unsafe {
        module.read().unwrap().use_ref(|module_ptr| {
            function_type.read().unwrap().use_ref(|function_type_ptr| {
                core::LLVMAddFunction(module_ptr, c_name.as_ptr(), function_type_ptr)
            })
        })
    };

    if function.is_null() {
        None
    } else {
        let mut pool_guard = pool.lock().unwrap();
        pool_guard.create_value(function)
    }
}