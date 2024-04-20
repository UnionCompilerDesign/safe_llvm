extern crate llvm_sys as llvm;

use std::ffi::CString;

use llvm::{core,
    prelude::{
        LLVMBasicBlockRef, LLVMBuilderRef, LLVMContextRef, LLVMValueRef}, 
        LLVMBasicBlock, LLVMValue
    };

use crate::memory_management::pointer::CPointer;

/// Creates a basic block in context
pub fn create_basic_block(context: CPointer<LLVMContextRef>, function: CPointer<LLVMValueRef>, name: &str) -> CPointer<LLVMBasicBlockRef> {
    let c_name: CString = CString::new(name).expect("Failed to create basic block name");

    let context_ptr: *mut LLVMContextRef = context.get_ref();
    let function_ptr: *mut LLVMValueRef = function.get_ref();

    let raw_ptr: *mut LLVMBasicBlock = unsafe { 
        core::LLVMAppendBasicBlockInContext(*context_ptr, *function_ptr, c_name.as_ptr()) 
    };

    CPointer::new(Some(raw_ptr as *mut _))
}

/// Retrieves the current insertion block
pub fn get_current_block(builder: CPointer<LLVMBuilderRef>) -> CPointer<LLVMBasicBlockRef> {
    let builder_ptr: *mut LLVMBuilderRef = builder.get_ref();
    
    let raw_ptr: *mut LLVMBasicBlock = unsafe {
        core::LLVMGetInsertBlock(*builder_ptr)
    };

    CPointer::new(Some(raw_ptr as *mut _))
}

/// creates a conditional branch
pub fn create_cond_br(builder: CPointer<LLVMBuilderRef>, condition: CPointer<LLVMValueRef>, then_bb: CPointer<LLVMBasicBlockRef>, else_bb: CPointer<LLVMBasicBlockRef>) -> CPointer<LLVMValueRef> {
    let builder_ptr: *mut LLVMBuilderRef = builder.get_ref();
    let condition_ptr: *mut LLVMValueRef = condition.get_ref();
    let then_bb_ptr: *mut LLVMBasicBlockRef = then_bb.get_ref();
    let else_bb_ptr: *mut LLVMBasicBlockRef = else_bb.get_ref();
    
    let raw_ptr: *mut LLVMValue = unsafe {
        core::LLVMBuildCondBr(*builder_ptr,*condition_ptr, *then_bb_ptr, *else_bb_ptr)
    };

    CPointer::new(Some(raw_ptr as *mut _))
}

/// creates an unconditional branch
pub fn create_br(builder: CPointer<LLVMBuilderRef>, target_bb: CPointer<LLVMBasicBlockRef>) -> CPointer<LLVMValueRef> {
    let builder_ptr: *mut LLVMBuilderRef = builder.get_ref();
    let target_bb_ptr: *mut LLVMBasicBlockRef = target_bb.get_ref();
    
    let raw_ptr: *mut LLVMValue = unsafe {
        core::LLVMBuildBr(*builder_ptr, *target_bb_ptr)
    };

    CPointer::new(Some(raw_ptr as *mut _))
}

/// Inserts a basic block in the context before the specified basic block
pub fn insert_before_basic_block(context: CPointer<LLVMContextRef>, before_target: CPointer<LLVMBasicBlockRef>, name: &str) -> CPointer<LLVMBasicBlockRef> {
    let c_name: CString = CString::new(name).unwrap();

    let context_ptr: *mut LLVMContextRef = context.get_ref();
    let before_target_ptr: *mut LLVMBasicBlockRef = before_target.get_ref();

    let raw_ptr: *mut LLVMBasicBlock = unsafe {
        core::LLVMInsertBasicBlockInContext(*context_ptr, *before_target_ptr, c_name.as_ptr())
    };

    CPointer::new(Some(raw_ptr as *mut _))
}

/// Positions the builder at the end of a block
pub fn position_builder(builder: CPointer<LLVMBuilderRef>, bb: CPointer<LLVMBasicBlockRef>) {
    let builder_ptr: *mut LLVMBuilderRef = builder.get_ref();
    let bb_ptr: *mut LLVMBasicBlockRef = bb.get_ref();

    unsafe {
        core::LLVMPositionBuilderAtEnd(*builder_ptr, *bb_ptr);
    }
}

/// Deletes a specified basic block
pub fn delete_basic_block(bb: CPointer<LLVMBasicBlockRef>) {
    let bb_ptr: *mut LLVMBasicBlockRef = bb.get_ref();
    
    unsafe {
        core::LLVMDeleteBasicBlock(*bb_ptr);
    }
}

/// Retrieves the first instruction 
pub fn get_first_instruction(bb: CPointer<LLVMBasicBlockRef>) -> CPointer<LLVMValueRef> {
    let bb_ptr: *mut LLVMBasicBlockRef = bb.get_ref();
    
    let raw_ptr: *mut LLVMValue = unsafe { 
        core::LLVMGetFirstInstruction(*bb_ptr)
    };

    CPointer::new(Some(raw_ptr as *mut _))
}

/// Retrieves the last instruction
pub fn get_last_instruction(bb: CPointer<LLVMBasicBlockRef>) -> CPointer<LLVMValueRef> {
    let bb_ptr: *mut LLVMBasicBlockRef = bb.get_ref();
    
    let raw_ptr: *mut LLVMValue = unsafe { 
        core::LLVMGetLastInstruction(*bb_ptr) 
    };

    CPointer::new(Some(raw_ptr as *mut _))
}

/// Creates a PHI node in the specified basic block
pub fn create_phi(builder: CPointer<LLVMBuilderRef>, possible_values: &[(CPointer<LLVMValueRef>, CPointer<LLVMBasicBlockRef>)], name: &str) -> CPointer<LLVMValueRef> {
    let builder_ptr: *mut LLVMBuilderRef = builder.get_ref();

    let first_value_ptr: *mut LLVMValueRef = possible_values[0].0.get_ref();
    let phi_type = unsafe { llvm::core::LLVMTypeOf(*first_value_ptr) };

    let c_name = CString::new(name).expect("Failed to create CString from name");
    let phi_node = unsafe {
        llvm::core::LLVMBuildPhi(*builder_ptr, phi_type, c_name.as_ptr())
    };

    let values: Vec<*mut LLVMValueRef> = possible_values.iter()
        .map(|&(ref v, _)| v.get_ref())
        .collect();

    let blocks: Vec<*mut LLVMBasicBlockRef> = possible_values.iter()
        .map(|&(_, ref b)| b.get_ref())
        .collect();

    unsafe {
        llvm::core::LLVMAddIncoming(phi_node, values.as_ptr() as *mut _, blocks.as_ptr() as *mut _, values.len() as u32);
    }

    CPointer::new(Some(phi_node as *mut _))
}
