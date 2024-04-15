extern crate llvm_sys as llvm;

use llvm::{core, prelude::*};
use std::ffi::CString;
use crate::memory_management::ir_pointer::IRPointer;

/// Basic addition
pub fn build_add(builder: *mut llvm::LLVMBuilder, param_a: *mut llvm::LLVMValue, param_b: *mut llvm::LLVMValue, sum: CString) 
        -> IRPointer<LLVMValueRef> {
    let raw_ptr = unsafe {
        core::LLVMBuildAdd(builder, param_a, param_b, sum.as_ptr())
    };
    IRPointer::new(Some(raw_ptr as *mut _))
}

/// Basic subtraction
pub fn build_sub(builder: *mut llvm::LLVMBuilder, param_a: *mut llvm::LLVMValue, param_b: *mut llvm::LLVMValue, name: CString) 
        -> IRPointer<LLVMValueRef> {
    let raw_ptr = unsafe {
        core::LLVMBuildSub(builder, param_a, param_b, name.as_ptr())
    };
    IRPointer::new(Some(raw_ptr as *mut _))
}

/// Basic multiplication
pub fn build_mul(builder: *mut llvm::LLVMBuilder, param_a: *mut llvm::LLVMValue, param_b: *mut llvm::LLVMValue, name: CString) 
        -> IRPointer<LLVMValueRef> {
    let raw_ptr = unsafe {
        core::LLVMBuildMul(builder, param_a, param_b, name.as_ptr())
    };
    IRPointer::new(Some(raw_ptr as *mut _))
}

/// Basic division
pub fn build_div(builder: *mut llvm::LLVMBuilder, param_a: *mut llvm::LLVMValue, param_b: *mut llvm::LLVMValue, name: CString) 
        -> IRPointer<LLVMValueRef> {
    let raw_ptr = unsafe {
        core::LLVMBuildSDiv(builder, param_a, param_b, name.as_ptr()) // Signed division
    };
    IRPointer::new(Some(raw_ptr as *mut _))
}

/// Modular arithmetic
pub fn build_rem(builder: *mut llvm::LLVMBuilder, param_a: *mut llvm::LLVMValue, param_b: *mut llvm::LLVMValue, name: CString) 
        -> IRPointer<LLVMValueRef> {
    let raw_ptr = unsafe {
        core::LLVMBuildSRem(builder, param_a, param_b, name.as_ptr()) // Signed remainder
    };
    IRPointer::new(Some(raw_ptr as *mut _))
}

/// Logical and
pub fn build_and(builder: *mut llvm::LLVMBuilder, param_a: *mut llvm::LLVMValue, param_b: *mut llvm::LLVMValue, name: CString) 
        -> IRPointer<LLVMValueRef> {
    let raw_ptr = unsafe {
        core::LLVMBuildAnd(builder, param_a, param_b, name.as_ptr())
    };
    IRPointer::new(Some(raw_ptr as *mut _))
}

/// Logical or
pub fn build_or(builder: *mut llvm::LLVMBuilder, param_a: *mut llvm::LLVMValue, param_b: *mut llvm::LLVMValue, name: CString) 
        -> IRPointer<LLVMValueRef> {
    let raw_ptr = unsafe {
        core::LLVMBuildOr(builder, param_a, param_b, name.as_ptr())
    };
    IRPointer::new(Some(raw_ptr as *mut _))
}

/// Logical xor
pub fn build_xor(builder: *mut llvm::LLVMBuilder, param_a: *mut llvm::LLVMValue, param_b: *mut llvm::LLVMValue, name: CString) 
        -> IRPointer<LLVMValueRef> {
    let raw_ptr = unsafe {
        core::LLVMBuildXor(builder, param_a, param_b, name.as_ptr())
    };
    IRPointer::new(Some(raw_ptr as *mut _))
}

/// Logical left shift
pub fn build_shl(builder: *mut llvm::LLVMBuilder, param_a: *mut llvm::LLVMValue, param_b: *mut llvm::LLVMValue, name: CString) 
        -> IRPointer<LLVMValueRef> {
    let raw_ptr = unsafe {
        core::LLVMBuildShl(builder, param_a, param_b, name.as_ptr())
    };
    IRPointer::new(Some(raw_ptr as *mut _))
}

/// Logical right shift
pub fn build_shr(builder: *mut llvm::LLVMBuilder, param_a: *mut llvm::LLVMValue, param_b: *mut llvm::LLVMValue, name: CString) 
        -> IRPointer<LLVMValueRef> {
    let raw_ptr = unsafe {
        core::LLVMBuildLShr(builder, param_a, param_b, name.as_ptr())
    };
    IRPointer::new(Some(raw_ptr as *mut _))
}

/// Greater than comparison
pub fn build_icmp_gt(builder: *mut llvm::LLVMBuilder, param_a: *mut llvm::LLVMValue, param_b: *mut llvm::LLVMValue, name: CString) 
        -> IRPointer<LLVMValueRef> {
    let raw_ptr = unsafe {
        core::LLVMBuildICmp(builder, llvm::LLVMIntPredicate::LLVMIntSGT, param_a, param_b, name.as_ptr())
    };
    IRPointer::new(Some(raw_ptr as *mut _))
}

/// Less than comparison
pub fn build_icmp_lt(builder: *mut llvm::LLVMBuilder, param_a: *mut llvm::LLVMValue, param_b: *mut llvm::LLVMValue, name: CString) 
        -> IRPointer<LLVMValueRef> {
    let raw_ptr = unsafe {
        core::LLVMBuildICmp(builder, llvm::LLVMIntPredicate::LLVMIntSLT, param_a, param_b, name.as_ptr())
    };
    IRPointer::new(Some(raw_ptr as *mut _))
}

/// Equal comparison
pub fn build_icmp_eq(builder: *mut llvm::LLVMBuilder, param_a: *mut llvm::LLVMValue, param_b: *mut llvm::LLVMValue, name: CString) 
        -> IRPointer<LLVMValueRef> {
    let raw_ptr = unsafe {
        core::LLVMBuildICmp(builder, llvm::LLVMIntPredicate::LLVMIntEQ, param_a, param_b, name.as_ptr())
    };
    IRPointer::new(Some(raw_ptr as *mut _))
}

/// Negation
pub fn build_negation(builder: *mut llvm::LLVMBuilder, operand_ir: LLVMValueRef, name: CString) -> IRPointer<LLVMValueRef> {
    let raw_ptr = unsafe {
        core::LLVMBuildNeg(builder, operand_ir, name.as_ptr())
    };
    IRPointer::new(Some(raw_ptr as *mut _))
}

/// Bitwise not
pub fn build_bitwise_not(builder: *mut llvm::LLVMBuilder, operand_ir: LLVMValueRef, name: CString) -> IRPointer<LLVMValueRef> {
    let raw_ptr = unsafe {
        core::LLVMBuildNot(builder, operand_ir, name.as_ptr())
    };
    IRPointer::new(Some(raw_ptr as *mut _))
}

/// Logical not
pub fn build_logical_not(builder: *mut llvm::LLVMBuilder, context: LLVMContextRef, operand_ir: LLVMValueRef, name: CString) -> IRPointer<LLVMValueRef> {
    let raw_ptr = unsafe {
        core::LLVMBuildICmp(builder, llvm::LLVMIntPredicate::LLVMIntEQ, operand_ir, core::LLVMConstInt(core::LLVMInt1TypeInContext(context), 0, 0), name.as_ptr())
    };
    IRPointer::new(Some(raw_ptr as *mut _))
}
