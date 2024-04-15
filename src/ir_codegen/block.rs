extern crate llvm_sys as llvm;

use std::ffi::CString;

use llvm::{core,
    prelude::{
        LLVMBasicBlockRef, LLVMBuilderRef, LLVMContextRef, LLVMValueRef}, 
        LLVMBasicBlock, LLVMValue
    };

use crate::memory_management::ir_pointer::IRPointer;

/// Creates a basic block in context
pub fn create_basic_block(context: LLVMContextRef, function: LLVMValueRef, name: &str) -> IRPointer<LLVMBasicBlockRef> {
    let c_name: CString = CString::new(name).expect("Failed to create basic block name");
    let raw_ptr: *mut LLVMBasicBlock = unsafe { 
        core::LLVMAppendBasicBlockInContext(context, function, c_name.as_ptr()) 
    };
    IRPointer::new(Some(raw_ptr as *mut _))
}

/// Retrieves the current insertion block
pub fn get_current_block(builder: *mut llvm::LLVMBuilder) -> IRPointer<LLVMBasicBlockRef> {
    let raw_ptr: *mut LLVMBasicBlock = unsafe {
        core::LLVMGetInsertBlock(builder)
    };
    IRPointer::new(Some(raw_ptr as *mut _))
}

/// creates a conditional branch
pub fn create_cond_br(builder: LLVMBuilderRef, condition: LLVMValueRef, then_bb: LLVMBasicBlockRef, else_bb: LLVMBasicBlockRef) -> IRPointer<LLVMValueRef> {
    let raw_ptr: *mut LLVMValue = unsafe {
        core::LLVMBuildCondBr(builder, condition, then_bb, else_bb)
    };
    IRPointer::new(Some(raw_ptr as *mut _))
}

/// creates an unconditional branch
pub fn create_br(builder: LLVMBuilderRef, target_bb: LLVMBasicBlockRef) -> IRPointer<LLVMValueRef> {
    let raw_ptr: *mut LLVMValue = unsafe {
        core::LLVMBuildBr(builder, target_bb)
    };
    IRPointer::new(Some(raw_ptr as *mut _))
}

/// Inserts a basic block in the context before the specified basic block
pub fn insert_before_basic_block(context: LLVMContextRef, before_target: LLVMBasicBlockRef, name: &str) -> IRPointer<LLVMBasicBlockRef> {
    let c_name = CString::new(name).unwrap();
    let raw_ptr: *mut LLVMBasicBlock = unsafe {
        core::LLVMInsertBasicBlockInContext(context, before_target, c_name.as_ptr())
    };
    IRPointer::new(Some(raw_ptr as *mut _))
}

/// Positions the builder at the end of a block
pub fn position_builder(builder: *mut llvm::LLVMBuilder, bb: *mut llvm::LLVMBasicBlock) {
    unsafe {
        core::LLVMPositionBuilderAtEnd(builder, bb);
    }
}

/// Deletes a specified basic block
pub fn delete_basic_block(bb: LLVMBasicBlockRef) {
    unsafe {
        core::LLVMDeleteBasicBlock(bb);
    }
}

/// Retrieves the first instruction 
pub fn get_first_instruction(bb: LLVMBasicBlockRef) -> IRPointer<LLVMValueRef> {
    let raw_ptr: *mut LLVMValue = unsafe { 
        core::LLVMGetFirstInstruction(bb)
    };
    IRPointer::new(Some(raw_ptr as *mut _))
}

/// Retrieves the last instruction
pub fn get_last_instruction(bb: LLVMBasicBlockRef) -> IRPointer<LLVMValueRef> {
    let raw_ptr: *mut LLVMValue = unsafe { 
        core::LLVMGetLastInstruction(bb) 
    };
    IRPointer::new(Some(raw_ptr as *mut _))
}

/// Creates a PHI node in the specified basic block
pub fn create_phi(builder: LLVMBuilderRef, possible_values: &[(LLVMValueRef, LLVMBasicBlockRef)], name: &str) -> IRPointer<LLVMValueRef> {
    let phi_type = unsafe { core::LLVMTypeOf(possible_values[0].0) };
    let phi_node = unsafe { core::LLVMBuildPhi(builder, phi_type, CString::new(name).unwrap().as_ptr()) };
    let values: Vec<LLVMValueRef> = possible_values.iter().map(|&(v, _)| v).collect();
    let blocks: Vec<LLVMBasicBlockRef> = possible_values.iter().map(|&(_, b)| b).collect();
    unsafe {
        core::LLVMAddIncoming(phi_node, values.as_ptr() as *mut _, blocks.as_ptr() as *mut _, values.len() as u32);
    }
    IRPointer::new(Some(phi_node as *mut _))
}
