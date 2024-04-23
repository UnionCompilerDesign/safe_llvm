extern crate llvm_sys as llvm;

use llvm::{core, LLVMBasicBlock, LLVMBuilder, LLVMContext, LLVMIntPredicate, LLVMModule, LLVMType, LLVMValue};

use std::{ffi::CString, sync::{Arc, Mutex}};

use crate::memory_management::resource_pools::{Handle, LLVMResourcePools};

/// Basic addition
pub fn build_add(pool: &Arc<Mutex<LLVMResourcePools<LLVMContext, LLVMModule, LLVMValue, LLVMBasicBlock, LLVMBuilder, LLVMType>>>, builder_handle: Handle, param_a_handle: Handle, param_b_handle: Handle, name: &str) -> Option<Handle> {
    let pool_guard = pool.lock().unwrap();
    let builder = pool_guard.get_builder(builder_handle)?;
    let param_a = pool_guard.get_value(param_a_handle)?;
    let param_b = pool_guard.get_value(param_b_handle)?;
    let c_name = CString::new(name).expect("Failed to create CString");
    drop(pool_guard);

    let result = unsafe {
        builder.read().unwrap().use_ref(|builder_ptr| {
            param_a.read().unwrap().use_ref(|param_a_ptr| {
                param_b.read().unwrap().use_ref(|param_b_ptr| {
                    core::LLVMBuildAdd(builder_ptr, param_a_ptr, param_b_ptr, c_name.as_ptr())
                })
            })
        })
    };

    if result.is_null() {
        None
    } else {
        let mut pool_guard = pool.lock().unwrap();
        pool_guard.create_value(result)
    }
}

/// Basic subtraction
pub fn build_sub(pool: &Arc<Mutex<LLVMResourcePools<LLVMContext, LLVMModule, LLVMValue, LLVMBasicBlock, LLVMBuilder, LLVMType>>>, builder_handle: Handle, param_a_handle: Handle, param_b_handle: Handle, name: &str) -> Option<Handle> {
    let pool_guard = pool.lock().unwrap();
    let builder = pool_guard.get_builder(builder_handle)?;
    let param_a = pool_guard.get_value(param_a_handle)?;
    let param_b = pool_guard.get_value(param_b_handle)?;
    drop(pool_guard);

    let c_name = CString::new(name).expect("Failed to create CString");

    let result = unsafe {
        builder.read().unwrap().use_ref(|builder_ptr| {
            param_a.read().unwrap().use_ref(|param_a_ptr| {
                param_b.read().unwrap().use_ref(|param_b_ptr| {
                    core::LLVMBuildSub(builder_ptr, param_a_ptr, param_b_ptr, c_name.as_ptr())
                })
            })
        })
    };

    if result.is_null() {
        None
    } else {
        let mut pool_guard = pool.lock().unwrap();
        pool_guard.create_value(result)
    }
}

/// Basic multiplication
pub fn build_mul(pool: &Arc<Mutex<LLVMResourcePools<LLVMContext, LLVMModule, LLVMValue, LLVMBasicBlock, LLVMBuilder, LLVMType>>>, builder_handle: Handle, param_a_handle: Handle, param_b_handle: Handle, name: &str) -> Option<Handle> {
    let pool_guard = pool.lock().unwrap();
    let builder = pool_guard.get_builder(builder_handle)?;
    let param_a = pool_guard.get_value(param_a_handle)?;
    let param_b = pool_guard.get_value(param_b_handle)?;
    drop(pool_guard);

    let c_name = CString::new(name).expect("Failed to create CString");

    let result = unsafe {
        builder.read().unwrap().use_ref(|builder_ptr| {
            param_a.read().unwrap().use_ref(|param_a_ptr| {
                param_b.read().unwrap().use_ref(|param_b_ptr| {
                    core::LLVMBuildMul(builder_ptr, param_a_ptr, param_b_ptr, c_name.as_ptr())
                })
            })
        })
    };

    if result.is_null() {
        None
    } else {
        let mut pool_guard = pool.lock().unwrap();
        pool_guard.create_value(result)
    }
}

/// Basic division
pub fn build_div(pool: &Arc<Mutex<LLVMResourcePools<LLVMContext, LLVMModule, LLVMValue, LLVMBasicBlock, LLVMBuilder, LLVMType>>>, builder_handle: Handle, param_a_handle: Handle, param_b_handle: Handle, name: &str) -> Option<Handle> {
    let pool_guard = pool.lock().unwrap();
    let builder = pool_guard.get_builder(builder_handle)?;
    let param_a = pool_guard.get_value(param_a_handle)?;
    let param_b = pool_guard.get_value(param_b_handle)?;
    drop(pool_guard);

    let c_name = CString::new(name).expect("Failed to create CString");

    let result = unsafe {
        builder.read().unwrap().use_ref(|builder_ptr| {
            param_a.read().unwrap().use_ref(|param_a_ptr| {
                param_b.read().unwrap().use_ref(|param_b_ptr| {
                    core::LLVMBuildSDiv(builder_ptr, param_a_ptr, param_b_ptr, c_name.as_ptr())
                })
            })
        })
    };

    if result.is_null() {
        None
    } else {
        let mut pool_guard = pool.lock().unwrap();
        pool_guard.create_value(result)
    }
}

/// Modular arithmetic (remainder)
pub fn build_rem(pool: &Arc<Mutex<LLVMResourcePools<LLVMContext, LLVMModule, LLVMValue, LLVMBasicBlock, LLVMBuilder, LLVMType>>>, builder_handle: Handle, param_a_handle: Handle, param_b_handle: Handle, name: &str) -> Option<Handle> {
    let pool_guard = pool.lock().unwrap();
    let builder = pool_guard.get_builder(builder_handle)?;
    let param_a = pool_guard.get_value(param_a_handle)?;
    let param_b = pool_guard.get_value(param_b_handle)?;
    drop(pool_guard);

    let c_name = CString::new(name).expect("Failed to create CString");

    let result = unsafe {
        builder.read().unwrap().use_ref(|builder_ptr| {
            param_a.read().unwrap().use_ref(|param_a_ptr| {
                param_b.read().unwrap().use_ref(|param_b_ptr| {
                    core::LLVMBuildSRem(builder_ptr, param_a_ptr, param_b_ptr, c_name.as_ptr())
                })
            })
        })
    };

    if result.is_null() {
        None
    } else {
        let mut pool_guard = pool.lock().unwrap();
        pool_guard.create_value(result)
    }
}

/// Logical and
pub fn build_and(pool: &Arc<Mutex<LLVMResourcePools<LLVMContext, LLVMModule, LLVMValue, LLVMBasicBlock, LLVMBuilder, LLVMType>>>, builder_handle: Handle, param_a_handle: Handle, param_b_handle: Handle, name: &str) -> Option<Handle> {
    let pool_guard = pool.lock().unwrap();
    let builder = pool_guard.get_builder(builder_handle)?;
    let param_a = pool_guard.get_value(param_a_handle)?;
    let param_b = pool_guard.get_value(param_b_handle)?;
    drop(pool_guard);

    let c_name = CString::new(name).expect("Failed to create CString");

    let result = unsafe {
        builder.read().unwrap().use_ref(|builder_ptr| {
            param_a.read().unwrap().use_ref(|param_a_ptr| {
                param_b.read().unwrap().use_ref(|param_b_ptr| {
                    core::LLVMBuildAnd(builder_ptr, param_a_ptr, param_b_ptr, c_name.as_ptr())
                })
            })
        })
    };

    if result.is_null() {
        None
    } else {
        let mut pool_guard = pool.lock().unwrap();
        pool_guard.create_value(result)
    }
}

/// Logical or
pub fn build_or(pool: &Arc<Mutex<LLVMResourcePools<LLVMContext, LLVMModule, LLVMValue, LLVMBasicBlock, LLVMBuilder, LLVMType>>>, builder_handle: Handle, param_a_handle: Handle, param_b_handle: Handle, name: &str) -> Option<Handle> {
    let pool_guard = pool.lock().unwrap();
    let builder = pool_guard.get_builder(builder_handle)?;
    let param_a = pool_guard.get_value(param_a_handle)?;
    let param_b = pool_guard.get_value(param_b_handle)?;
    drop(pool_guard);
    
    let c_name = CString::new(name).expect("Failed to create CString");

    let result = unsafe {
        builder.read().unwrap().use_ref(|builder_ptr| {
            param_a.read().unwrap().use_ref(|param_a_ptr| {
                param_b.read().unwrap().use_ref(|param_b_ptr| {
                    core::LLVMBuildOr(builder_ptr, param_a_ptr, param_b_ptr, c_name.as_ptr())
                })
            })
        })
    };

    if result.is_null() {
        None
    } else {
        let mut pool_guard = pool.lock().unwrap();
        pool_guard.create_value(result)
    }
}

/// Logical xor
pub fn build_xor(pool: &Arc<Mutex<LLVMResourcePools<LLVMContext, LLVMModule, LLVMValue, LLVMBasicBlock, LLVMBuilder, LLVMType>>>, builder_handle: Handle, param_a_handle: Handle, param_b_handle: Handle, name: &str) -> Option<Handle> {
    let pool_guard = pool.lock().unwrap();
    let builder = pool_guard.get_builder(builder_handle)?;
    let param_a = pool_guard.get_value(param_a_handle)?;
    let param_b = pool_guard.get_value(param_b_handle)?;
    drop(pool_guard);
    
    let c_name = CString::new(name).expect("Failed to create CString");

    let result = unsafe {
        builder.read().unwrap().use_ref(|builder_ptr| {
            param_a.read().unwrap().use_ref(|param_a_ptr| {
                param_b.read().unwrap().use_ref(|param_b_ptr| {
                    core::LLVMBuildXor(builder_ptr, param_a_ptr, param_b_ptr, c_name.as_ptr())
                })
            })
        })
    };

    if result.is_null() {
        None
    } else {
        let mut pool_guard = pool.lock().unwrap();
        pool_guard.create_value(result)
    }
}

/// Logical left shift
pub fn build_shl(pool: &Arc<Mutex<LLVMResourcePools<LLVMContext, LLVMModule, LLVMValue, LLVMBasicBlock, LLVMBuilder, LLVMType>>>, builder_handle: Handle, param_a_handle: Handle, param_b_handle: Handle, name: &str) -> Option<Handle> {
    let pool_guard = pool.lock().unwrap();
    let builder = pool_guard.get_builder(builder_handle)?;
    let param_a = pool_guard.get_value(param_a_handle)?;
    let param_b = pool_guard.get_value(param_b_handle)?;
    drop(pool_guard);
    
    let c_name = CString::new(name).expect("Failed to create CString");

    let result = unsafe {
        builder.read().unwrap().use_ref(|builder_ptr| {
            param_a.read().unwrap().use_ref(|param_a_ptr| {
                param_b.read().unwrap().use_ref(|param_b_ptr| {
                    core::LLVMBuildShl(builder_ptr, param_a_ptr, param_b_ptr, c_name.as_ptr())
                })
            })
        })
    };

    if result.is_null() {
        None
    } else {
        let mut pool_guard = pool.lock().unwrap();
        pool_guard.create_value(result)
    }
}

/// Logical right shift
pub fn build_shr(pool: &Arc<Mutex<LLVMResourcePools<LLVMContext, LLVMModule, LLVMValue, LLVMBasicBlock, LLVMBuilder, LLVMType>>>, builder_handle: Handle, param_a_handle: Handle, param_b_handle: Handle, name: &str) -> Option<Handle> {
    let pool_guard = pool.lock().unwrap();
    let builder = pool_guard.get_builder(builder_handle)?;
    let param_a = pool_guard.get_value(param_a_handle)?;
    let param_b = pool_guard.get_value(param_b_handle)?;
    drop(pool_guard);
    
    let c_name = CString::new(name).expect("Failed to create CString");

    let result = unsafe {
        builder.read().unwrap().use_ref(|builder_ptr| {
            param_a.read().unwrap().use_ref(|param_a_ptr| {
                param_b.read().unwrap().use_ref(|param_b_ptr| {
                    core::LLVMBuildLShr(builder_ptr, param_a_ptr, param_b_ptr, c_name.as_ptr())
                })
            })
        })
    };

    if result.is_null() {
        None
    } else {
        let mut pool_guard = pool.lock().unwrap();
        pool_guard.create_value(result)
    }
}

/// Greater than comparison
pub fn build_icmp_gt(pool: &Arc<Mutex<LLVMResourcePools<LLVMContext, LLVMModule, LLVMValue, LLVMBasicBlock, LLVMBuilder, LLVMType>>>, builder_handle: Handle, param_a_handle: Handle, param_b_handle: Handle, name: &str) -> Option<Handle> {
    let pool_guard = pool.lock().unwrap();
    let builder = pool_guard.get_builder(builder_handle)?;
    let param_a = pool_guard.get_value(param_a_handle)?;
    let param_b = pool_guard.get_value(param_b_handle)?;
    drop(pool_guard);
    
    let c_name = CString::new(name).expect("Failed to create CString");

    let result = unsafe {
        builder.read().unwrap().use_ref(|builder_ptr| {
            param_a.read().unwrap().use_ref(|param_a_ptr| {
                param_b.read().unwrap().use_ref(|param_b_ptr| {
                    core::LLVMBuildICmp(builder_ptr, LLVMIntPredicate::LLVMIntSGT, param_a_ptr, param_b_ptr, c_name.as_ptr())
                })
            })
        })
    };

    if result.is_null() {
        None
    } else {
        let mut pool_guard = pool.lock().unwrap();
        pool_guard.create_value(result)
    }
}

/// Less than comparison
pub fn build_icmp_lt(pool: &Arc<Mutex<LLVMResourcePools<LLVMContext, LLVMModule, LLVMValue, LLVMBasicBlock, LLVMBuilder, LLVMType>>>, builder_handle: Handle, param_a_handle: Handle, param_b_handle: Handle, name: &str) -> Option<Handle> {
    let pool_guard = pool.lock().unwrap();
    let builder = pool_guard.get_builder(builder_handle)?;
    let param_a = pool_guard.get_value(param_a_handle)?;
    let param_b = pool_guard.get_value(param_b_handle)?;
    drop(pool_guard);

    let c_name = CString::new(name).expect("Failed to create CString");

    let result = unsafe {
        builder.read().unwrap().use_ref(|builder_ptr| {
            param_a.read().unwrap().use_ref(|param_a_ptr| {
                param_b.read().unwrap().use_ref(|param_b_ptr| {
                    core::LLVMBuildICmp(builder_ptr, LLVMIntPredicate::LLVMIntSLT, param_a_ptr, param_b_ptr, c_name.as_ptr())
                })
            })
        })
    };

    if result.is_null() {
        None
    } else {
        let mut pool_guard = pool.lock().unwrap();
        pool_guard.create_value(result)
    }
}

/// Equal comparison
pub fn build_icmp_eq(pool: &Arc<Mutex<LLVMResourcePools<LLVMContext, LLVMModule, LLVMValue, LLVMBasicBlock, LLVMBuilder, LLVMType>>>, builder_handle: Handle, param_a_handle: Handle, param_b_handle: Handle, name: &str) -> Option<Handle> {
    let pool_guard = pool.lock().unwrap();
    let builder = pool_guard.get_builder(builder_handle)?;
    let param_a = pool_guard.get_value(param_a_handle)?;
    let param_b = pool_guard.get_value(param_b_handle)?;
    drop(pool_guard);

    let c_name = CString::new(name).expect("Failed to create CString");

    let result = unsafe {
        builder.read().unwrap().use_ref(|builder_ptr| {
            param_a.read().unwrap().use_ref(|param_a_ptr| {
                param_b.read().unwrap().use_ref(|param_b_ptr| {
                    core::LLVMBuildICmp(builder_ptr, LLVMIntPredicate::LLVMIntEQ, param_a_ptr, param_b_ptr, c_name.as_ptr())
                })
            })
        })
    };

    if result.is_null() {
        None
    } else {
        let mut pool_guard = pool.lock().unwrap();
        pool_guard.create_value(result)
    }
}

/// Negation
pub fn build_negation(pool: &Arc<Mutex<LLVMResourcePools<LLVMContext, LLVMModule, LLVMValue, LLVMBasicBlock, LLVMBuilder, LLVMType>>>, builder_handle: Handle, operand_handle: Handle, name: &str) -> Option<Handle> {
    let pool_guard = pool.lock().unwrap();
    let builder = pool_guard.get_builder(builder_handle)?;
    let operand = pool_guard.get_value(operand_handle)?;
    drop(pool_guard);
   
    let c_name = CString::new(name).expect("Failed to create CString");

    let result = unsafe {
        builder.read().unwrap().use_ref(|builder_ptr| {
            operand.read().unwrap().use_ref(|operand_ptr| {
                core::LLVMBuildNeg(builder_ptr, operand_ptr, c_name.as_ptr())
            })
        })
    };

    if result.is_null() {
        None
    } else {
        let mut pool_guard = pool.lock().unwrap();
        pool_guard.create_value(result)
    }
}

/// Bitwise not
pub fn build_bitwise_not(pool: &Arc<Mutex<LLVMResourcePools<LLVMContext, LLVMModule, LLVMValue, LLVMBasicBlock, LLVMBuilder, LLVMType>>>, builder_handle: Handle, operand_handle: Handle, name: &str) -> Option<Handle> {
    let pool_guard = pool.lock().unwrap();
    let builder = pool_guard.get_builder(builder_handle)?;
    let operand = pool_guard.get_value(operand_handle)?;
    drop(pool_guard);
   
    let c_name = CString::new(name).expect("Failed to create CString");

    let result = unsafe {
        builder.read().unwrap().use_ref(|builder_ptr| {
            operand.read().unwrap().use_ref(|operand_ptr| {
                core::LLVMBuildNot(builder_ptr, operand_ptr, c_name.as_ptr())
            })
        })
    };

    if result.is_null() {
        None
    } else {
        let mut pool_guard = pool.lock().unwrap();
        pool_guard.create_value(result)
    }
}

/// Logical not
pub fn build_logical_not(pool: &Arc<Mutex<LLVMResourcePools<LLVMContext, LLVMModule, LLVMValue, LLVMBasicBlock, LLVMBuilder, LLVMType>>>, builder_handle: Handle, context_handle: Handle, operand_handle: Handle, name: &str) -> Option<Handle> {
    let pool_guard = pool.lock().unwrap();
    let builder = pool_guard.get_builder(builder_handle)?;
    let context = pool_guard.get_context(context_handle)?;
    let operand = pool_guard.get_value(operand_handle)?;
    drop(pool_guard);
    
    let zero = unsafe { context.read().unwrap().use_ref(|context_ptr| {
        core::LLVMConstInt(core::LLVMInt1TypeInContext(context_ptr), 0, 0)
    })};
    let c_name = CString::new(name).expect("Failed to create CString");

    let result = unsafe {
        builder.read().unwrap().use_ref(|builder_ptr| {
            operand.read().unwrap().use_ref(|operand_ptr| {
                core::LLVMBuildICmp(builder_ptr, LLVMIntPredicate::LLVMIntEQ, operand_ptr, zero, c_name.as_ptr())
            })
        })
    };

    if result.is_null() {
        None
    } else {
        let mut pool_guard = pool.lock().unwrap();
        pool_guard.create_value(result)
    }
}
