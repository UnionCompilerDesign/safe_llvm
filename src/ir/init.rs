extern crate llvm_sys as llvm;

use std::{ffi::CString, sync::Mutex};

use llvm::{core, prelude::{LLVMContextRef, LLVMModuleRef}, LLVMBasicBlock, LLVMBuilder, LLVMContext, LLVMModule, LLVMType, LLVMValue};
use once_cell::sync::Lazy;

use crate::memory_management::resource_pools::{LLVMResourcePools, Handle};

static POOL: Lazy<Mutex<LLVMResourcePools<LLVMContext, LLVMModule, LLVMValue, LLVMBasicBlock, LLVMBuilder, LLVMType>>> = Lazy::new(|| {
    Mutex::new(LLVMResourcePools::new())
});

/// Initializes a context and stores it in the global pool, returning a handle to it.
pub fn create_context() -> Option<Handle> {
    let raw_ptr: LLVMContextRef = unsafe { core::LLVMContextCreate() };
    if raw_ptr.is_null() {
        return None;
    }

    let mut pool = POOL.lock().unwrap();
    pool.create_context(raw_ptr as *mut LLVMContext)
}

/// Initializes a module in the specified LLVM context, returning a handle to the module.
pub fn create_module(module_name: &str, context_handle: Handle) -> Option<Handle> {
    let c_module_name = CString::new(module_name).expect("Failed to create CString from module name");

    let pool = POOL.lock().unwrap();
    let context_lock = pool.get_context(context_handle)?;
    let context_cpointer = context_lock.read().unwrap(); 

    drop(pool);

    let module_ptr: LLVMModuleRef = context_cpointer.use_ref(|context_ptr| {
        unsafe { core::LLVMModuleCreateWithNameInContext(c_module_name.as_ptr(), context_ptr) }
    });

    if module_ptr.is_null() {
        return None;
    }

    let mut pool = POOL.lock().unwrap();

    pool.create_module(module_ptr as *mut LLVMModule)
}

