use llvm::{core, prelude::*}; // change to not use wild star import

/// returns nothing
pub fn void_return(builder: *mut llvm::LLVMBuilder) -> LLVMValueRef {
    unsafe {
        core::LLVMBuildRetVoid(builder)
    }
}

/// returns something
pub fn nonvoid_return(builder: *mut llvm::LLVMBuilder, value: LLVMValueRef) -> LLVMValueRef {
    unsafe {
        core::LLVMBuildRet(builder, value)
    }
}
