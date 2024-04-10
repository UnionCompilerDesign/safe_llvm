extern crate llvm_sys as llvm;

use llvm::{core, prelude::{LLVMTypeRef, LLVMValueRef}};

/// void type
pub fn void_type(context: *mut llvm::LLVMContext) -> *mut llvm::LLVMType {
    unsafe {
        core::LLVMVoidTypeInContext(context)
    }
}

/// integer type
pub fn int_type(context: *mut llvm::LLVMContext) -> *mut llvm::LLVMType {
    unsafe {
        core::LLVMIntTypeInContext(context, 64)
    }
}

/// float type
pub fn float_type(context: *mut llvm::LLVMContext) -> *mut llvm::LLVMType {
    unsafe {
        core::LLVMFloatTypeInContext(context) 
    }
}

/// boolean type
pub fn boolean_type(context: *mut llvm::LLVMContext) -> *mut llvm::LLVMType {
    unsafe {
        core::LLVMInt1TypeInContext(context)
    }
}

/// pointer type
pub fn pointer_type(element_type: LLVMTypeRef) -> *mut llvm::LLVMType {
    unsafe {
        core::LLVMPointerType(element_type, 0)
    }
}

/// array type
pub fn array_type(element_type: LLVMTypeRef, num_elements: u64) -> *mut llvm::LLVMType {
    unsafe {
        core::LLVMArrayType2(element_type, num_elements)
    }
}

/// struct type
pub fn struct_type(context: *mut llvm::LLVMContext, element_types: &[LLVMTypeRef], packed: bool) -> *mut llvm::LLVMType {
    unsafe {
        core::LLVMStructTypeInContext(context, element_types.as_ptr() as *mut _, element_types.len() as u32, packed as i32)
    }
}

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
