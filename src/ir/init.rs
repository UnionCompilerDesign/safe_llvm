extern crate llvm_sys as llvm;

use llvm::{core, prelude::{LLVMContextRef, LLVMModuleRef}, LLVMBasicBlock, LLVMBuilder, LLVMContext, LLVMModule, LLVMType, LLVMValue};

use std::{ffi::CString, sync::{Arc, Mutex}};

use crate::memory_management::resource_pools::{LLVMResourcePools, Handle};

pub fn create_llvm_resource_pool() -> Arc<Mutex<LLVMResourcePools<LLVMContext, LLVMModule, LLVMValue, LLVMBasicBlock, LLVMBuilder, LLVMType>>> {
    Arc::new(Mutex::new(LLVMResourcePools::new()))
}

pub fn create_context(pool: &Arc<Mutex<LLVMResourcePools<LLVMContext, LLVMModule, LLVMValue, LLVMBasicBlock, LLVMBuilder, LLVMType>>>) -> Option<Handle> {
    let raw_ptr: LLVMContextRef = unsafe { core::LLVMContextCreate() };
    if raw_ptr.is_null() {
        return None;
    }

    let mut pool_locked = pool.lock().unwrap();
    pool_locked.create_context(raw_ptr)
}

pub fn create_module(pool: &Arc<Mutex<LLVMResourcePools<LLVMContext, LLVMModule, LLVMValue, LLVMBasicBlock, LLVMBuilder, LLVMType>>>, module_name: &str, context_handle: Handle) -> Option<Handle> {
    let c_module_name = CString::new(module_name).expect("Failed to create CString from module name");

    let pool_locked = pool.lock().unwrap();
    let context_lock = pool_locked.get_context(context_handle)?;
    drop(pool_locked);

    let context_cpointer = context_lock.read().unwrap();

    let module_ptr: LLVMModuleRef = context_cpointer.use_ref(|context_ptr| {
        unsafe { core::LLVMModuleCreateWithNameInContext(c_module_name.as_ptr(), context_ptr) }
    });

    if module_ptr.is_null() {
        return None;
    }
    
    let mut pool_locked = pool.lock().unwrap();
    pool_locked.create_module(module_ptr)
}
