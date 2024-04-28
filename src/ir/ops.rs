extern crate llvm_sys as llvm;

use llvm::{core, LLVMIntPredicate};

use std::ffi::CString;

use crate::memory_management::{
    pointer::{LLVMRef, LLVMRefType}, 
    resource_pools::{BuilderTag, ContextTag, ResourcePools, ValueTag}
};

impl ResourcePools {
    /// Basic addition
    pub fn build_add(
        &mut self,
        builder_tag: BuilderTag,
        param_a_tag: ValueTag,
        param_b_tag: ValueTag,
        name: &str
    ) -> Option<ValueTag> {
        let builder_arc_rwlock = self.get_builder(builder_tag)?;
        let param_a_arc_rwlock = self.get_value(param_a_tag)?;
        let param_b_arc_rwlock = self.get_value(param_b_tag)?;

        let c_name = CString::new(name).expect("Failed to create CString");

        let result = unsafe {
            let builder_ptr = builder_arc_rwlock.read().expect("Failed to lock builder for reading").read(LLVMRefType::Builder, |builder_ref| {
                if let LLVMRef::Builder(ptr) = builder_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            let param_a_ptr = param_a_arc_rwlock.read().expect("Failed to lock param a for reading").read(LLVMRefType::Value, |param_a_ref| {
                if let LLVMRef::Value(ptr) = param_a_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            let param_b_ptr = param_b_arc_rwlock.read().expect("Failed to lock param b for reading").read(LLVMRefType::Value, |param_b_ref| {
                if let LLVMRef::Value(ptr) = param_b_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            core::LLVMBuildAdd(builder_ptr, param_a_ptr, param_b_ptr, c_name.as_ptr())
        };

        if result.is_null() {
            None
        } else {
            self.store_value(result)
        }
    }

    /// Basic subtraction
    pub fn build_sub(
        &mut self,
        builder_tag: BuilderTag,
        param_a_tag: ValueTag,
        param_b_tag: ValueTag,
        name: &str
    ) -> Option<ValueTag> {
        let builder_arc_rwlock = self.get_builder(builder_tag)?;
        let param_a_arc_rwlock = self.get_value(param_a_tag)?;
        let param_b_arc_rwlock = self.get_value(param_b_tag)?;

        let c_name = CString::new(name).expect("Failed to create CString");

        let result = unsafe {
            let builder_ptr = builder_arc_rwlock.read().expect("Failed to lock builder for reading").read(LLVMRefType::Builder, |builder_ref| {
                if let LLVMRef::Builder(ptr) = builder_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            let param_a_ptr = param_a_arc_rwlock.read().expect("Failed to lock param a for reading").read(LLVMRefType::Value, |param_a_ref| {
                if let LLVMRef::Value(ptr) = param_a_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            let param_b_ptr = param_b_arc_rwlock.read().expect("Failed to lock param b for reading").read(LLVMRefType::Value, |param_b_ref| {
                if let LLVMRef::Value(ptr) = param_b_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            core::LLVMBuildSub(builder_ptr, param_a_ptr, param_b_ptr, c_name.as_ptr())
        };

        if result.is_null() {
            None
        } else {
            self.store_value(result)
        }
    }

    /// Basic multiplication
    pub fn build_mul(
        &mut self,
        builder_tag: BuilderTag,
        param_a_tag: ValueTag,
        param_b_tag: ValueTag,
        name: &str
    ) -> Option<ValueTag> {
        let builder_arc_rwlock = self.get_builder(builder_tag)?;
        let param_a_arc_rwlock = self.get_value(param_a_tag)?;
        let param_b_arc_rwlock = self.get_value(param_b_tag)?;

        let c_name = CString::new(name).expect("Failed to create CString");

        let result = unsafe {
            let builder_ptr = builder_arc_rwlock.read().expect("Failed to lock builder for reading").read(LLVMRefType::Builder, |builder_ref| {
                if let LLVMRef::Builder(ptr) = builder_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            let param_a_ptr = param_a_arc_rwlock.read().expect("Failed to lock param a for reading").read(LLVMRefType::Value, |param_a_ref| {
                if let LLVMRef::Value(ptr) = param_a_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            let param_b_ptr = param_b_arc_rwlock.read().expect("Failed to lock param b for reading").read(LLVMRefType::Value, |param_b_ref| {
                if let LLVMRef::Value(ptr) = param_b_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            core::LLVMBuildMul(builder_ptr, param_a_ptr, param_b_ptr, c_name.as_ptr())
        };

        if result.is_null() {
            None
        } else {
            self.store_value(result)
        }
    }

    /// Basic division
    pub fn build_div(
        &mut self,
        builder_tag: BuilderTag,
        param_a_tag: ValueTag,
        param_b_tag: ValueTag,
        name: &str
    ) -> Option<ValueTag> {
        let builder_arc_rwlock = self.get_builder(builder_tag)?;
        let param_a_arc_rwlock = self.get_value(param_a_tag)?;
        let param_b_arc_rwlock = self.get_value(param_b_tag)?;

        let c_name = CString::new(name).expect("Failed to create CString");

        let result = unsafe {
            let builder_ptr = builder_arc_rwlock.read().expect("Failed to lock builder for reading").read(LLVMRefType::Builder, |builder_ref| {
                if let LLVMRef::Builder(ptr) = builder_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            let param_a_ptr = param_a_arc_rwlock.read().expect("Failed to lock param a for reading").read(LLVMRefType::Value, |param_a_ref| {
                if let LLVMRef::Value(ptr) = param_a_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            let param_b_ptr = param_b_arc_rwlock.read().expect("Failed to lock param b for reading").read(LLVMRefType::Value, |param_b_ref| {
                if let LLVMRef::Value(ptr) = param_b_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            core::LLVMBuildSDiv(builder_ptr, param_a_ptr, param_b_ptr, c_name.as_ptr())
        };

        if result.is_null() {
            None
        } else {
            self.store_value(result)
        }
    }

    /// Modular arithmetic (remainder)
    pub fn build_rem(
        &mut self,
        builder_tag: BuilderTag,
        param_a_tag: ValueTag,
        param_b_tag: ValueTag,
        name: &str
    ) -> Option<ValueTag> {
        let builder_arc_rwlock = self.get_builder(builder_tag)?;
        let param_a_arc_rwlock = self.get_value(param_a_tag)?;
        let param_b_arc_rwlock = self.get_value(param_b_tag)?;

        let c_name = CString::new(name).expect("Failed to create CString");

        let result = unsafe {
            let builder_ptr = builder_arc_rwlock.read().expect("Failed to lock builder for reading").read(LLVMRefType::Builder, |builder_ref| {
                if let LLVMRef::Builder(ptr) = builder_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            let param_a_ptr = param_a_arc_rwlock.read().expect("Failed to lock param a for reading").read(LLVMRefType::Value, |param_a_ref| {
                if let LLVMRef::Value(ptr) = param_a_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            let param_b_ptr = param_b_arc_rwlock.read().expect("Failed to lock param b for reading").read(LLVMRefType::Value, |param_b_ref| {
                if let LLVMRef::Value(ptr) = param_b_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            core::LLVMBuildSRem(builder_ptr, param_a_ptr, param_b_ptr, c_name.as_ptr())
        };

        if result.is_null() {
            None
        } else {
            self.store_value(result)
        }
    }

    /// Logical and
    pub fn build_and(
        &mut self,
        builder_tag: BuilderTag,
        param_a_tag: ValueTag,
        param_b_tag: ValueTag,
        name: &str
    ) -> Option<ValueTag> {
        let builder_arc_rwlock = self.get_builder(builder_tag)?;
        let param_a_arc_rwlock = self.get_value(param_a_tag)?;
        let param_b_arc_rwlock = self.get_value(param_b_tag)?;

        let c_name = CString::new(name).expect("Failed to create CString");

        let result = unsafe {
            let builder_ptr = builder_arc_rwlock.read().expect("Failed to lock builder for reading").read(LLVMRefType::Builder, |builder_ref| {
                if let LLVMRef::Builder(ptr) = builder_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            let param_a_ptr = param_a_arc_rwlock.read().expect("Failed to lock param a for reading").read(LLVMRefType::Value, |param_a_ref| {
                if let LLVMRef::Value(ptr) = param_a_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            let param_b_ptr = param_b_arc_rwlock.read().expect("Failed to lock param b for reading").read(LLVMRefType::Value, |param_b_ref| {
                if let LLVMRef::Value(ptr) = param_b_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            core::LLVMBuildAnd(builder_ptr, param_a_ptr, param_b_ptr, c_name.as_ptr())
        };

        if result.is_null() {
            None
        } else {
            self.store_value(result)
        }
    }

    /// Logical or
    pub fn build_or(
        &mut self,
        builder_tag: BuilderTag,
        param_a_tag: ValueTag,
        param_b_tag: ValueTag,
        name: &str
    ) -> Option<ValueTag> {
        let builder_arc_rwlock = self.get_builder(builder_tag)?;
        let param_a_arc_rwlock = self.get_value(param_a_tag)?;
        let param_b_arc_rwlock = self.get_value(param_b_tag)?;

        let c_name = CString::new(name).expect("Failed to create CString");

        let result = unsafe {
            let builder_ptr = builder_arc_rwlock.read().expect("Failed to lock builder for reading").read(LLVMRefType::Builder, |builder_ref| {
                if let LLVMRef::Builder(ptr) = builder_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            let param_a_ptr = param_a_arc_rwlock.read().expect("Failed to lock param a for reading").read(LLVMRefType::Value, |param_a_ref| {
                if let LLVMRef::Value(ptr) = param_a_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            let param_b_ptr = param_b_arc_rwlock.read().expect("Failed to lock param b for reading").read(LLVMRefType::Value, |param_b_ref| {
                if let LLVMRef::Value(ptr) = param_b_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            core::LLVMBuildOr(builder_ptr, param_a_ptr, param_b_ptr, c_name.as_ptr())
        };

        if result.is_null() {
            None
        } else {
            self.store_value(result)
        }
    }

    /// Logical xor
    pub fn build_xor(
        &mut self,
        builder_tag: BuilderTag,
        param_a_tag: ValueTag,
        param_b_tag: ValueTag,
        name: &str
    ) -> Option<ValueTag> {
        let builder_arc_rwlock = self.get_builder(builder_tag)?;
        let param_a_arc_rwlock = self.get_value(param_a_tag)?;
        let param_b_arc_rwlock = self.get_value(param_b_tag)?;

        let c_name = CString::new(name).expect("Failed to create CString");

        let result = unsafe {
            let builder_ptr = builder_arc_rwlock.read().expect("Failed to lock builder for reading").read(LLVMRefType::Builder, |builder_ref| {
                if let LLVMRef::Builder(ptr) = builder_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            let param_a_ptr = param_a_arc_rwlock.read().expect("Failed to lock param a for reading").read(LLVMRefType::Value, |param_a_ref| {
                if let LLVMRef::Value(ptr) = param_a_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            let param_b_ptr = param_b_arc_rwlock.read().expect("Failed to lock param b for reading").read(LLVMRefType::Value, |param_b_ref| {
                if let LLVMRef::Value(ptr) = param_b_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            core::LLVMBuildXor(builder_ptr, param_a_ptr, param_b_ptr, c_name.as_ptr())
        };

        if result.is_null() {
            None
        } else {
            self.store_value(result)
        }
    }

    /// Logical left shift
    pub fn build_shl(
        &mut self,
        builder_tag: BuilderTag,
        param_a_tag: ValueTag,
        param_b_tag: ValueTag,
        name: &str
    ) -> Option<ValueTag> {
        let builder_arc_rwlock = self.get_builder(builder_tag)?;
        let param_a_arc_rwlock = self.get_value(param_a_tag)?;
        let param_b_arc_rwlock = self.get_value(param_b_tag)?;

        let c_name = CString::new(name).expect("Failed to create CString");

        let result = unsafe {
            let builder_ptr = builder_arc_rwlock.read().expect("Failed to lock builder for reading").read(LLVMRefType::Builder, |builder_ref| {
                if let LLVMRef::Builder(ptr) = builder_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            let param_a_ptr = param_a_arc_rwlock.read().expect("Failed to lock param a for reading").read(LLVMRefType::Value, |param_a_ref| {
                if let LLVMRef::Value(ptr) = param_a_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            let param_b_ptr = param_b_arc_rwlock.read().expect("Failed to lock param b for reading").read(LLVMRefType::Value, |param_b_ref| {
                if let LLVMRef::Value(ptr) = param_b_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            core::LLVMBuildShl(builder_ptr, param_a_ptr, param_b_ptr, c_name.as_ptr())
        };

        if result.is_null() {
            None
        } else {
            self.store_value(result)
        }
    }

    /// Logical right shift
    pub fn build_shr(
        &mut self,
        builder_tag: BuilderTag,
        param_a_tag: ValueTag,
        param_b_tag: ValueTag,
        name: &str
    ) -> Option<ValueTag> {
        let builder_arc_rwlock = self.get_builder(builder_tag)?;
        let param_a_arc_rwlock = self.get_value(param_a_tag)?;
        let param_b_arc_rwlock = self.get_value(param_b_tag)?;

        let c_name = CString::new(name).expect("Failed to create CString");

        let result = unsafe {
            let builder_ptr = builder_arc_rwlock.read().expect("Failed to lock builder for reading").read(LLVMRefType::Builder, |builder_ref| {
                if let LLVMRef::Builder(ptr) = builder_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            let param_a_ptr = param_a_arc_rwlock.read().expect("Failed to lock param a for reading").read(LLVMRefType::Value, |param_a_ref| {
                if let LLVMRef::Value(ptr) = param_a_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            let param_b_ptr = param_b_arc_rwlock.read().expect("Failed to lock param b for reading").read(LLVMRefType::Value, |param_b_ref| {
                if let LLVMRef::Value(ptr) = param_b_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            core::LLVMBuildLShr(builder_ptr, param_a_ptr, param_b_ptr, c_name.as_ptr())
        };

        if result.is_null() {
            None
        } else {
            self.store_value(result)
        }
    }

    /// Greater than comparison
    pub fn build_icmp_gt(
        &mut self,
        builder_tag: BuilderTag,
        param_a_tag: ValueTag,
        param_b_tag: ValueTag,
        name: &str
    ) -> Option<ValueTag> {
        let builder_arc_rwlock = self.get_builder(builder_tag)?;
        let param_a_arc_rwlock = self.get_value(param_a_tag)?;
        let param_b_arc_rwlock = self.get_value(param_b_tag)?;

        let c_name = CString::new(name).expect("Failed to create CString");

        let result = unsafe {
            let builder_ptr = builder_arc_rwlock.read().expect("Failed to lock builder for reading").read(LLVMRefType::Builder, |builder_ref| {
                if let LLVMRef::Builder(ptr) = builder_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            let param_a_ptr = param_a_arc_rwlock.read().expect("Failed to lock param a for reading").read(LLVMRefType::Value, |param_a_ref| {
                if let LLVMRef::Value(ptr) = param_a_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            let param_b_ptr = param_b_arc_rwlock.read().expect("Failed to lock param b for reading").read(LLVMRefType::Value, |param_b_ref| {
                if let LLVMRef::Value(ptr) = param_b_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            core::LLVMBuildICmp(builder_ptr, LLVMIntPredicate::LLVMIntSGT, param_a_ptr, param_b_ptr, c_name.as_ptr())
        };

        if result.is_null() {
            None
        } else {
            self.store_value(result)
        }
    }

    /// Less than comparison
    pub fn build_icmp_lt(
        &mut self,
        builder_tag: BuilderTag,
        param_a_tag: ValueTag,
        param_b_tag: ValueTag,
        name: &str
    ) -> Option<ValueTag> {
        let builder_arc_rwlock = self.get_builder(builder_tag)?;
        let param_a_arc_rwlock = self.get_value(param_a_tag)?;
        let param_b_arc_rwlock = self.get_value(param_b_tag)?;

        let c_name = CString::new(name).expect("Failed to create CString");

        let result = unsafe {
            let builder_ptr = builder_arc_rwlock.read().expect("Failed to lock builder for reading").read(LLVMRefType::Builder, |builder_ref| {
                if let LLVMRef::Builder(ptr) = builder_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            let param_a_ptr = param_a_arc_rwlock.read().expect("Failed to lock param a for reading").read(LLVMRefType::Value, |param_a_ref| {
                if let LLVMRef::Value(ptr) = param_a_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            let param_b_ptr = param_b_arc_rwlock.read().expect("Failed to lock param b for reading").read(LLVMRefType::Value, |param_b_ref| {
                if let LLVMRef::Value(ptr) = param_b_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            core::LLVMBuildICmp(builder_ptr, LLVMIntPredicate::LLVMIntSLT, param_a_ptr, param_b_ptr, c_name.as_ptr())
        };

        if result.is_null() {
            None
        } else {
            self.store_value(result)
        }
    }

    /// Equal comparison
    pub fn build_icmp_eq(
        &mut self,
        builder_tag: BuilderTag,
        param_a_tag: ValueTag,
        param_b_tag: ValueTag,
        name: &str
    ) -> Option<ValueTag> {
        let builder_arc_rwlock = self.get_builder(builder_tag)?;
        let param_a_arc_rwlock = self.get_value(param_a_tag)?;
        let param_b_arc_rwlock = self.get_value(param_b_tag)?;

        let c_name = CString::new(name).expect("Failed to create CString");

        let result = unsafe {
            let builder_ptr = builder_arc_rwlock.read().expect("Failed to lock builder for reading").read(LLVMRefType::Builder, |builder_ref| {
                if let LLVMRef::Builder(ptr) = builder_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            let param_a_ptr = param_a_arc_rwlock.read().expect("Failed to lock param a for reading").read(LLVMRefType::Value, |param_a_ref| {
                if let LLVMRef::Value(ptr) = param_a_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            let param_b_ptr = param_b_arc_rwlock.read().expect("Failed to lock param b for reading").read(LLVMRefType::Value, |param_b_ref| {
                if let LLVMRef::Value(ptr) = param_b_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            core::LLVMBuildICmp(builder_ptr, LLVMIntPredicate::LLVMIntEQ, param_a_ptr, param_b_ptr, c_name.as_ptr())
        };

        if result.is_null() {
            None
        } else {
            self.store_value(result)
        }
    }

    /// Negation
    pub fn build_negation(
        &mut self,
        builder_tag: BuilderTag,
        operand_tag: ValueTag,
        name: &str
    ) -> Option<ValueTag> {
        let builder_arc_rwlock = self.get_builder(builder_tag)?;
        let operand_arc_rwlock = self.get_value(operand_tag)?;

        let c_name = CString::new(name).expect("Failed to create CString");

        let result = unsafe {
            let builder_ptr = builder_arc_rwlock.read().expect("Failed to lock builder for reading").read(LLVMRefType::Builder, |builder_ref| {
                if let LLVMRef::Builder(ptr) = builder_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            let operand_ptr = operand_arc_rwlock.read().expect("Failed to lock operand for reading").read(LLVMRefType::Value, |operand_ref| {
                if let LLVMRef::Value(ptr) = operand_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            core::LLVMBuildNeg(builder_ptr, operand_ptr, c_name.as_ptr())
        };

        if result.is_null() {
            None
        } else {
            self.store_value(result)
        }
    }

    /// Bitwise not
    pub fn build_bitwise_not(
        &mut self,
        builder_tag: BuilderTag,
        operand_tag: ValueTag,
        name: &str
    ) -> Option<ValueTag> {
        let builder_arc_rwlock = self.get_builder(builder_tag)?;
        let operand_arc_rwlock = self.get_value(operand_tag)?;

        let c_name = CString::new(name).expect("Failed to create CString");

        let result = unsafe {
            let builder_ptr = builder_arc_rwlock.read().expect("Failed to lock builder for reading").read(LLVMRefType::Builder, |builder_ref| {
                if let LLVMRef::Builder(ptr) = builder_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            let operand_ptr = operand_arc_rwlock.read().expect("Failed to lock operand for reading").read(LLVMRefType::Value, |operand_ref| {
                if let LLVMRef::Value(ptr) = operand_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            core::LLVMBuildNot(builder_ptr, operand_ptr, c_name.as_ptr())
        };

        if result.is_null() {
            None
        } else {
            self.store_value(result)
        }
    }

    /// Logical not
    pub fn build_logical_not(
        &mut self,
        builder_tag: BuilderTag,
        context_tag: ContextTag,
        operand_tag: ValueTag,
        name: &str
    ) -> Option<ValueTag> {
        let builder_arc_rwlock = self.get_builder(builder_tag)?;
        let context_arc_rwlock = self.get_context(context_tag)?;
        let operand_arc_rwlock = self.get_value(operand_tag)?;

        let zero = unsafe { 
            let context_ptr = context_arc_rwlock.read().expect("Failed to lock context for reading").read(LLVMRefType::Context, |context_ref| {
                if let LLVMRef::Context(ptr) = context_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;
            core::LLVMConstInt(core::LLVMInt1TypeInContext(context_ptr), 0, 0)
        };
        
        let c_name = CString::new(name).expect("Failed to create CString");

        let result = unsafe {
            let builder_ptr = builder_arc_rwlock.read().expect("Failed to lock builder for reading").read(LLVMRefType::Builder, |builder_ref| {
                if let LLVMRef::Builder(ptr) = builder_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            let operand_ptr = operand_arc_rwlock.read().expect("Failed to lock operand for reading").read(LLVMRefType::Value, |operand_ref| {
                if let LLVMRef::Value(ptr) = operand_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            core::LLVMBuildICmp(builder_ptr, LLVMIntPredicate::LLVMIntEQ, operand_ptr, zero, c_name.as_ptr())
        };

        if result.is_null() {
            None
        } else {
            self.store_value(result)
        }
    }
}