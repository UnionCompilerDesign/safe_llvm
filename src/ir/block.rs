extern crate llvm_sys as llvm;

use llvm::{core, prelude::{LLVMBasicBlockRef, LLVMValueRef}, 
    LLVMBasicBlock, LLVMBuilder, LLVMContext, LLVMModule, LLVMType, LLVMValue
};


use std::{ffi::CString, sync::{Arc, Mutex}};

use crate::memory_management::resource_pools::{Handle, LLVMResourcePools};

/// Creates a basic block in context
pub fn create_basic_block(pool: &Arc<Mutex<LLVMResourcePools<LLVMContext, LLVMModule, LLVMValue, LLVMBasicBlock, LLVMBuilder, LLVMType>>>, context_handle: Handle, function_handle: Handle, name: &str) -> Option<Handle> {
    let pool_guard = pool.lock().unwrap();
    let context = pool_guard.get_context(context_handle)?;
    let function = pool_guard.get_value(function_handle)?;
    drop(pool_guard);

    let c_name = CString::new(name).expect("Failed to create CString from name");

    let basic_block = unsafe {
        context.read().unwrap().use_ref(|context_ptr| {
            function.read().unwrap().use_ref(|function_ptr| {
                core::LLVMAppendBasicBlockInContext(context_ptr, function_ptr, c_name.as_ptr())
            })
        })
    };

    if basic_block.is_null() {
        None
    } else {
        let mut pool_guard = pool.lock().unwrap();
        pool_guard.create_basic_block(basic_block)
    }
}

/// Retrieves the current insertion block
pub fn get_current_block(pool: &Arc<Mutex<LLVMResourcePools<LLVMContext, LLVMModule, LLVMValue, LLVMBasicBlock, LLVMBuilder, LLVMType>>>, builder_handle: Handle) -> Option<Handle> {
    let pool_guard = pool.lock().unwrap();
    let builder = pool_guard.get_builder(builder_handle)?;
    drop(pool_guard);

    let block = unsafe {
        builder.read().unwrap().use_ref(|builder_ptr| {
            core::LLVMGetInsertBlock(builder_ptr)
        })
    };

    if block.is_null() {
        None
    } else {
        let mut pool_guard = pool.lock().unwrap();
        pool_guard.create_basic_block(block)
    }
}

/// Creates a conditional branch
pub fn create_cond_br(pool: &Arc<Mutex<LLVMResourcePools<LLVMContext, LLVMModule, LLVMValue, LLVMBasicBlock, LLVMBuilder, LLVMType>>>, builder_handle: Handle, condition_handle: Handle, then_bb_handle: Handle, else_bb_handle: Handle) -> Option<Handle> {
    let pool_guard = pool.lock().unwrap();
    let builder = pool_guard.get_builder(builder_handle)?;
    let condition = pool_guard.get_value(condition_handle)?;
    let then_bb = pool_guard.get_basic_block(then_bb_handle)?;
    let else_bb = pool_guard.get_basic_block(else_bb_handle)?;
    drop(pool_guard);

    let branch = unsafe {
        builder.read().unwrap().use_ref(|builder_ptr| {
            condition.read().unwrap().use_ref(|condition_ptr| {
                then_bb.read().unwrap().use_ref(|then_bb_ptr| {
                    else_bb.read().unwrap().use_ref(|else_bb_ptr| {
                        core::LLVMBuildCondBr(builder_ptr, condition_ptr, then_bb_ptr, else_bb_ptr)
                    })
                })
            })
        })
    };

    if branch.is_null() {
        None
    } else {
        let mut pool_guard = pool.lock().unwrap();
        pool_guard.create_value(branch)
    }
}

/// Creates an unconditional branch
pub fn create_br(pool: &Arc<Mutex<LLVMResourcePools<LLVMContext, LLVMModule, LLVMValue, LLVMBasicBlock, LLVMBuilder, LLVMType>>>, builder_handle: Handle, target_bb_handle: Handle) -> Option<Handle> {
    let pool_guard = pool.lock().unwrap();
    let builder = pool_guard.get_builder(builder_handle)?;
    let target_bb = pool_guard.get_basic_block(target_bb_handle)?;
    drop(pool_guard);

    let branch = unsafe {
        builder.read().unwrap().use_ref(|builder_ptr| {
            target_bb.read().unwrap().use_ref(|target_bb_ptr| {
                core::LLVMBuildBr(builder_ptr, target_bb_ptr)
            })
        })
    };

    if branch.is_null() {
        None
    } else {
        let mut pool_guard = pool.lock().unwrap();
        pool_guard.create_value(branch)
    }
}
/// Inserts a basic block in the context before the specified basic block
pub fn insert_before_basic_block(pool: &Arc<Mutex<LLVMResourcePools<LLVMContext, LLVMModule, LLVMValue, LLVMBasicBlock, LLVMBuilder, LLVMType>>>, context_handle: Handle, before_target_handle: Handle, name: &str) -> Option<Handle> {
    let pool_guard = pool.lock().unwrap();
    let context = pool_guard.get_context(context_handle)?;
    let before_target = pool_guard.get_basic_block(before_target_handle)?;
    drop(pool_guard);

    let c_name = CString::new(name).expect("Failed to create CString from name");

    let basic_block = unsafe {
        context.read().unwrap().use_ref(|context_ptr| {
            before_target.read().unwrap().use_ref(|before_target_ptr| {
                core::LLVMInsertBasicBlockInContext(context_ptr, before_target_ptr, c_name.as_ptr())
            })
        })
    };

    if basic_block.is_null() {
        None
    } else {
        let mut pool_guard = pool.lock().unwrap();
        pool_guard.create_basic_block(basic_block)
    }
}

/// Positions the builder at the end of a block
pub fn position_builder(pool: &Arc<Mutex<LLVMResourcePools<LLVMContext, LLVMModule, LLVMValue, LLVMBasicBlock, LLVMBuilder, LLVMType>>>, builder_handle: Handle, bb_handle: Handle) -> Option<()> {
    let pool_guard = pool.lock().unwrap();
    let builder = pool_guard.get_builder(builder_handle)?;
    let bb = pool_guard.get_basic_block(bb_handle)?;

    unsafe {
        builder.read().unwrap().use_ref(|builder_ptr| {
            bb.read().unwrap().use_ref(|bb_ptr| {
                core::LLVMPositionBuilderAtEnd(builder_ptr, bb_ptr)
            })
        });
    }
    Some(())
}

/// Deletes a specified basic block
pub fn delete_basic_block(pool: &Arc<Mutex<LLVMResourcePools<LLVMContext, LLVMModule, LLVMValue, LLVMBasicBlock, LLVMBuilder, LLVMType>>>, bb_handle: Handle) -> Option<()> {
    let pool_guard = pool.lock().unwrap();
    let bb = pool_guard.get_basic_block(bb_handle)?;

    unsafe {
        bb.read().unwrap().use_ref(|bb_ptr| {
            core::LLVMDeleteBasicBlock(bb_ptr)
        });
    }

    Some(())
}

/// Retrieves the first instruction
pub fn get_first_instruction(pool: &Arc<Mutex<LLVMResourcePools<LLVMContext, LLVMModule, LLVMValue, LLVMBasicBlock, LLVMBuilder, LLVMType>>>, bb_handle: Handle) -> Option<Handle> {
    let pool_guard = pool.lock().unwrap();
    let bb = pool_guard.get_basic_block(bb_handle)?;
    drop(pool_guard);

    let instruction = unsafe {
        bb.read().unwrap().use_ref(|bb_ptr| {
            core::LLVMGetFirstInstruction(bb_ptr)
        })
    };

    if instruction.is_null() {
        None
    } else {
        let mut pool_guard = pool.lock().unwrap();
        pool_guard.create_value(instruction)
    }
}

/// Retrieves the last instruction
pub fn get_last_instruction(pool: &Arc<Mutex<LLVMResourcePools<LLVMContext, LLVMModule, LLVMValue, LLVMBasicBlock, LLVMBuilder, LLVMType>>>, bb_handle: Handle) -> Option<Handle> {
    let pool_guard = pool.lock().unwrap();
    let bb = pool_guard.get_basic_block(bb_handle)?;
    drop(pool_guard);

    let instruction = unsafe {
        bb.read().unwrap().use_ref(|bb_ptr| {
            core::LLVMGetLastInstruction(bb_ptr)
        })
    };

    if instruction.is_null() {
        None
    } else {
        let mut pool_guard = pool.lock().unwrap();
        pool_guard.create_value(instruction)
    }
}

/// Creates a PHI node in the specified basic block
pub fn create_phi(pool: &Arc<Mutex<LLVMResourcePools<LLVMContext, LLVMModule, LLVMValue, LLVMBasicBlock, LLVMBuilder, LLVMType>>>, builder_handle: Handle, possible_values: &[(Handle, Handle)], name: &str) -> Option<Handle> {
    let pool_guard = pool.lock().unwrap();
    let builder = pool_guard.get_builder(builder_handle)?;
    let first_value = pool_guard.get_value(possible_values[0].0)?;
    drop(pool_guard);

    let phi_type = unsafe {
        first_value.read().unwrap().use_ref(|first_value_ptr| {
            core::LLVMTypeOf(first_value_ptr)
        })
    };

    let c_name = CString::new(name).expect("Failed to create CString from name");

    let phi_node = unsafe {
        builder.read().unwrap().use_ref(|builder_ptr| {
            core::LLVMBuildPhi(builder_ptr, phi_type, c_name.as_ptr())
        })
    };

    if phi_node.is_null() {
        None
    } else {
        let pool_guard = pool.lock().unwrap();
        let mut values: Vec<LLVMValueRef> = possible_values.iter().map(|&(val_handle, _)| {
            let val = pool_guard.get_value(val_handle).unwrap();
            let unwrapped_val = val.read().unwrap().use_ref(|ptr| ptr as LLVMValueRef); 
            unwrapped_val
        }).collect();

        let mut blocks: Vec<LLVMBasicBlockRef> = possible_values.iter().map(|&(_, block_handle)| {
            let block = pool_guard.get_basic_block(block_handle).unwrap();
            let unwrapped_block = block.read().unwrap().use_ref(|ptr| ptr as *mut _); 
            unwrapped_block
        }).collect();

        drop(pool_guard);

        unsafe {
            core::LLVMAddIncoming(phi_node, values.as_mut_ptr(), blocks.as_mut_ptr(), values.len() as u32);
        }

        let mut pool_guard = pool.lock().unwrap();
        pool_guard.create_value(phi_node)
    }
}
