extern crate llvm_sys as llvm;

use llvm::{core, prelude::*};
use std::ffi::CString;
use crate::memory_management::pointer::CPointer;

/// Basic addition
pub fn build_add(builder: CPointer<LLVMBuilderRef>, param_a: CPointer<LLVMValueRef>, param_b: CPointer<LLVMValueRef>, sum: &str) -> CPointer<LLVMValueRef> {
    let c_sum = CString::new(sum).expect("Failed to create CString from sum");
    let builder_ptr = builder.get_ref();
    let param_a_ptr = param_a.get_ref();
    let param_b_ptr = param_b.get_ref();

    let raw_ptr = unsafe {
        core::LLVMBuildAdd(*builder_ptr, *param_a_ptr, *param_b_ptr, c_sum.as_ptr())
    };
    let c_pointer = CPointer::new(raw_ptr as *mut _);
    if c_pointer.is_some() {
        return c_pointer.unwrap();
    }
    panic!("Missing c_pointer")}

/// Basic subtraction
pub fn build_sub(builder: CPointer<LLVMBuilderRef>, param_a: CPointer<LLVMValueRef>, param_b: CPointer<LLVMValueRef>, name: &str) -> CPointer<LLVMValueRef> {
    let c_name = CString::new(name).expect("Failed to create CString from name");
    let builder_ptr = builder.get_ref();
    let param_a_ptr = param_a.get_ref();
    let param_b_ptr = param_b.get_ref();

    let raw_ptr = unsafe {
        core::LLVMBuildSub(*builder_ptr, *param_a_ptr, *param_b_ptr, c_name.as_ptr())
    };
    let c_pointer = CPointer::new(raw_ptr as *mut _);
    if c_pointer.is_some() {
        return c_pointer.unwrap();
    }
    panic!("Missing c_pointer")}

/// Basic multiplication
pub fn build_mul(builder: CPointer<LLVMBuilderRef>, param_a: CPointer<LLVMValueRef>, param_b: CPointer<LLVMValueRef>, name: &str) -> CPointer<LLVMValueRef> {
    let c_name = CString::new(name).expect("Failed to create CString from name");
    let builder_ptr = builder.get_ref();
    let param_a_ptr = param_a.get_ref();
    let param_b_ptr = param_b.get_ref();

    let raw_ptr = unsafe {
        core::LLVMBuildMul(*builder_ptr, *param_a_ptr, *param_b_ptr, c_name.as_ptr())
    };
    let c_pointer = CPointer::new(raw_ptr as *mut _);
    if c_pointer.is_some() {
        return c_pointer.unwrap();
    }
    panic!("Missing c_pointer")}

/// Basic division
pub fn build_div(builder: CPointer<LLVMBuilderRef>, param_a: CPointer<LLVMValueRef>, param_b: CPointer<LLVMValueRef>, name: &str) -> CPointer<LLVMValueRef> {
    let c_name = CString::new(name).expect("Failed to create CString from name");
    let builder_ptr = builder.get_ref();
    let param_a_ptr = param_a.get_ref();
    let param_b_ptr = param_b.get_ref();

    let raw_ptr = unsafe {
        core::LLVMBuildSDiv(*builder_ptr, *param_a_ptr, *param_b_ptr, c_name.as_ptr()) // Signed division
    };
    let c_pointer = CPointer::new(raw_ptr as *mut _);
    if c_pointer.is_some() {
        return c_pointer.unwrap();
    }
    panic!("Missing c_pointer")}

/// Modular arithmetic
pub fn build_rem(builder: CPointer<LLVMBuilderRef>, param_a: CPointer<LLVMValueRef>, param_b: CPointer<LLVMValueRef>, name: &str) -> CPointer<LLVMValueRef> {
    let c_name = CString::new(name).expect("Failed to create CString from name");
    let builder_ptr = builder.get_ref();
    let param_a_ptr = param_a.get_ref();
    let param_b_ptr = param_b.get_ref();

    let raw_ptr = unsafe {
        core::LLVMBuildSRem(*builder_ptr, *param_a_ptr, *param_b_ptr, c_name.as_ptr()) // Signed remainder
    };
    let c_pointer = CPointer::new(raw_ptr as *mut _);
    if c_pointer.is_some() {
        return c_pointer.unwrap();
    }
    panic!("Missing c_pointer")}

/// Logical and
pub fn build_and(builder: CPointer<LLVMBuilderRef>, param_a: CPointer<LLVMValueRef>, param_b: CPointer<LLVMValueRef>, name: &str) -> CPointer<LLVMValueRef> {
    let c_name = CString::new(name).expect("Failed to create CString from name");
    let builder_ptr = builder.get_ref();
    let param_a_ptr = param_a.get_ref();
    let param_b_ptr = param_b.get_ref();

    let raw_ptr = unsafe {
        core::LLVMBuildAnd(*builder_ptr, *param_a_ptr, *param_b_ptr, c_name.as_ptr())
    };
    let c_pointer = CPointer::new(raw_ptr as *mut _);
    if c_pointer.is_some() {
        return c_pointer.unwrap();
    }
    panic!("Missing c_pointer")}

/// Logical or
pub fn build_or(builder: CPointer<LLVMBuilderRef>, param_a: CPointer<LLVMValueRef>, param_b: CPointer<LLVMValueRef>, name: &str) -> CPointer<LLVMValueRef> {
    let c_name = CString::new(name).expect("Failed to create CString from name");
    let builder_ptr = builder.get_ref();
    let param_a_ptr = param_a.get_ref();
    let param_b_ptr = param_b.get_ref();

    let raw_ptr = unsafe {
        core::LLVMBuildOr(*builder_ptr, *param_a_ptr, *param_b_ptr, c_name.as_ptr())
    };
    let c_pointer = CPointer::new(raw_ptr as *mut _);
    if c_pointer.is_some() {
        return c_pointer.unwrap();
    }
    panic!("Missing c_pointer")}

/// Logical xor
pub fn build_xor(builder: CPointer<LLVMBuilderRef>, param_a: CPointer<LLVMValueRef>, param_b: CPointer<LLVMValueRef>, name: &str) -> CPointer<LLVMValueRef> {
    let c_name = CString::new(name).expect("Failed to create CString from name");
    let builder_ptr = builder.get_ref();
    let param_a_ptr = param_a.get_ref();
    let param_b_ptr = param_b.get_ref();

    let raw_ptr = unsafe {
        core::LLVMBuildXor(*builder_ptr, *param_a_ptr, *param_b_ptr, c_name.as_ptr())
    };
    let c_pointer = CPointer::new(raw_ptr as *mut _);
    if c_pointer.is_some() {
        return c_pointer.unwrap();
    }
    panic!("Missing c_pointer")}

/// Logical left shift
pub fn build_shl(builder: CPointer<LLVMBuilderRef>, param_a: CPointer<LLVMValueRef>, param_b: CPointer<LLVMValueRef>, name: &str) -> CPointer<LLVMValueRef> {
    let c_name = CString::new(name).expect("Failed to create CString from name");
    let builder_ptr = builder.get_ref();
    let param_a_ptr = param_a.get_ref();
    let param_b_ptr = param_b.get_ref();

    let raw_ptr = unsafe {
        core::LLVMBuildShl(*builder_ptr, *param_a_ptr, *param_b_ptr, c_name.as_ptr())
    };
    let c_pointer = CPointer::new(raw_ptr as *mut _);
    if c_pointer.is_some() {
        return c_pointer.unwrap();
    }
    panic!("Missing c_pointer")}

/// Logical right shift
pub fn build_shr(builder: CPointer<LLVMBuilderRef>, param_a: CPointer<LLVMValueRef>, param_b: CPointer<LLVMValueRef>, name: &str) -> CPointer<LLVMValueRef> {
    let c_name = CString::new(name).expect("Failed to create CString from name");
    let builder_ptr = builder.get_ref();
    let param_a_ptr = param_a.get_ref();
    let param_b_ptr = param_b.get_ref();

    let raw_ptr = unsafe {
        core::LLVMBuildLShr(*builder_ptr, *param_a_ptr, *param_b_ptr, c_name.as_ptr())
    };
    let c_pointer = CPointer::new(raw_ptr as *mut _);
    if c_pointer.is_some() {
        return c_pointer.unwrap();
    }
    panic!("Missing c_pointer")}

/// Greater than comparison
pub fn build_icmp_gt(builder: CPointer<LLVMBuilderRef>, param_a: CPointer<LLVMValueRef>, param_b: CPointer<LLVMValueRef>, name: &str) -> CPointer<LLVMValueRef> {
    let c_name = CString::new(name).expect("Failed to create CString from name");
    let builder_ptr = builder.get_ref();
    let param_a_ptr = param_a.get_ref();
    let param_b_ptr = param_b.get_ref();

    let raw_ptr = unsafe {
        core::LLVMBuildICmp(*builder_ptr, llvm::LLVMIntPredicate::LLVMIntSGT, *param_a_ptr, *param_b_ptr, c_name.as_ptr())
    };
    let c_pointer = CPointer::new(raw_ptr as *mut _);
    if c_pointer.is_some() {
        return c_pointer.unwrap();
    }
    panic!("Missing c_pointer")}

/// Less than comparison
pub fn build_icmp_lt(builder: CPointer<LLVMBuilderRef>, param_a: CPointer<LLVMValueRef>, param_b: CPointer<LLVMValueRef>, name: &str) -> CPointer<LLVMValueRef> {
    let c_name = CString::new(name).expect("Failed to create CString from name");
    let builder_ptr = builder.get_ref();
    let param_a_ptr = param_a.get_ref();
    let param_b_ptr = param_b.get_ref();

    let raw_ptr = unsafe {
        core::LLVMBuildICmp(*builder_ptr, llvm::LLVMIntPredicate::LLVMIntSLT, *param_a_ptr, *param_b_ptr, c_name.as_ptr())
    };
    let c_pointer = CPointer::new(raw_ptr as *mut _);
    if c_pointer.is_some() {
        return c_pointer.unwrap();
    }
    panic!("Missing c_pointer")}

/// Equal comparison
pub fn build_icmp_eq(builder: CPointer<LLVMBuilderRef>, param_a: CPointer<LLVMValueRef>, param_b: CPointer<LLVMValueRef>, name: &str) -> CPointer<LLVMValueRef> {
    let c_name = CString::new(name).expect("Failed to create CString from name");
    let builder_ptr = builder.get_ref();
    let param_a_ptr = param_a.get_ref();
    let param_b_ptr = param_b.get_ref();

    let raw_ptr = unsafe {
        core::LLVMBuildICmp(*builder_ptr, llvm::LLVMIntPredicate::LLVMIntEQ, *param_a_ptr, *param_b_ptr, c_name.as_ptr())
    };
    let c_pointer = CPointer::new(raw_ptr as *mut _);
    if c_pointer.is_some() {
        return c_pointer.unwrap();
    }
    panic!("Missing c_pointer")}

/// Negation
pub fn build_negation(builder: CPointer<LLVMBuilderRef>, operand_ir: CPointer<LLVMValueRef>, name: &str) -> CPointer<LLVMValueRef> {
    let c_name = CString::new(name).expect("Failed to create CString from name");
    let builder_ptr = builder.get_ref();
    let operand_ir_ptr = operand_ir.get_ref();

    let raw_ptr = unsafe {
        core::LLVMBuildNeg(*builder_ptr, *operand_ir_ptr, c_name.as_ptr())
    };
    let c_pointer = CPointer::new(raw_ptr as *mut _);
    if c_pointer.is_some() {
        return c_pointer.unwrap();
    }
    panic!("Missing c_pointer")}

/// Bitwise not
pub fn build_bitwise_not(builder: CPointer<LLVMBuilderRef>, operand_ir: CPointer<LLVMValueRef>, name: &str) -> CPointer<LLVMValueRef> {
    let c_name = CString::new(name).expect("Failed to create CString from name");
    let builder_ptr = builder.get_ref();
    let operand_ir_ptr = operand_ir.get_ref();

    let raw_ptr = unsafe {
        core::LLVMBuildNot(*builder_ptr, *operand_ir_ptr, c_name.as_ptr())
    };
    let c_pointer = CPointer::new(raw_ptr as *mut _);
    if c_pointer.is_some() {
        return c_pointer.unwrap();
    }
    panic!("Missing c_pointer")}

/// Logical not
pub fn build_logical_not(builder: CPointer<LLVMBuilderRef>, context: CPointer<LLVMContextRef>, operand_ir: CPointer<LLVMValueRef>, name: &str) -> CPointer<LLVMValueRef> {
    let c_name = CString::new(name).expect("Failed to create CString from name");
    let builder_ptr = builder.get_ref();
    let context_ptr = context.get_ref();
    let operand_ir_ptr = operand_ir.get_ref();
    let zero = unsafe { core::LLVMConstInt(core::LLVMInt1TypeInContext(*context_ptr), 0, 0) };

    let raw_ptr = unsafe {
        core::LLVMBuildICmp(*builder_ptr, llvm::LLVMIntPredicate::LLVMIntEQ, *operand_ir_ptr, zero, c_name.as_ptr())
    };
    let c_pointer = CPointer::new(raw_ptr as *mut _);
    if c_pointer.is_some() {
        return c_pointer.unwrap();
    }
    panic!("Missing c_pointer")}
