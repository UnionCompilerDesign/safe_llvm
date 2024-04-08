extern crate llvm_sys as llvm;

use llvm::{core, prelude::LLVMBasicBlockRef};

/// Positions the builder at the end of a block
pub fn position_builder(builder: *mut llvm::LLVMBuilder, bb: *mut llvm::LLVMBasicBlock) {
    unsafe {
        core::LLVMPositionBuilderAtEnd(builder, bb);
    }
}

/// Retrieves the current insertion block
pub fn get_current_block(builder: *mut llvm::LLVMBuilder) -> LLVMBasicBlockRef {
    unsafe {
        core::LLVMGetInsertBlock(builder)
    }
}
