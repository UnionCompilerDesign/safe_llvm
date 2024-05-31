extern crate llvm_sys as llvm;

use llvm::{core, LLVMIntPredicate};

use std::ffi::CString;

use crate::common::pointer::{LLVMRef, LLVMRefType};

use super::core::{BasicBlockTag, BuilderTag, ContextTag, IRGenerator, TypeTag, ValueTag};

impl IRGenerator {
    /// Creates an integer constant of 64 bits in the specified context.
    pub fn create_integer(&mut self, context_tag: ContextTag, val: i64) -> Option<ValueTag> {
        let context_arc_rwlock = self.get_context(context_tag)?;
        let integer_value = {
            let context_rwlock = context_arc_rwlock.read().expect("Failed to lock context for reading");
            let context_ptr = context_rwlock.read(LLVMRefType::Context, |context_ref| {
                if let LLVMRef::Context(ptr) = context_ref {
                    Some(unsafe {
                        core::LLVMConstInt(core::LLVMInt64TypeInContext(*ptr), val as u64, 0)
                    })
                } else {
                    None
                }
            })?;
            context_ptr
        };

        if integer_value.is_null() {
            None
        } else {
            self.store_value(integer_value)
        }
    }

    /// Creates a floating-point constant in the specified context.
    pub fn create_float(&mut self, context_tag: ContextTag, val: f64) -> Option<ValueTag> {
        let context_arc_rwlock = self.get_context(context_tag)?;
        let float_value = {
            let context_rwlock = context_arc_rwlock.read().expect("Failed to lock context for reading");
            let context_ptr = context_rwlock.read(LLVMRefType::Context, |context_ref| {
                if let LLVMRef::Context(ptr) = context_ref {
                    Some(unsafe { core::LLVMConstReal(core::LLVMDoubleTypeInContext(*ptr), val) })
                } else {
                    None
                }
            })?;
            context_ptr
        };

        if float_value.is_null() {
            None
        } else {
            self.store_value(float_value)
        }
    }

    /// Creates a boolean
    pub fn create_boolean(&mut self, context_tag: ContextTag, val: bool) -> Option<ValueTag> {
        let context_arc_rwlock = self.get_context(context_tag)?;
        let boolean_value = {
            let context_rwlock = context_arc_rwlock.read().expect("Failed to lock context for reading");
            let context_ptr = context_rwlock.read(LLVMRefType::Context, |context_ref| {
                if let LLVMRef::Context(ptr) = context_ref {
                    Some(unsafe { core::LLVMConstInt(core::LLVMInt1TypeInContext(*ptr), val as u64, 0) })
                } else {
                    None
                }
            })?;
            context_ptr
        };

        if boolean_value.is_null() {
            None
        } else {
            self.store_value(boolean_value)
        }
    }

    /// Creates an array
    pub fn create_array(&mut self, value_tag: ValueTag, num_elements: u64) -> Option<ValueTag> {
        let value_arc_rwlock = self.get_value(value_tag)?;
        
        let array_type = unsafe {
            let value_ptr = value_arc_rwlock.read().expect("Failed to lock value for reading").read(LLVMRefType::Value, |value_ref| {
                if let LLVMRef::Value(ptr) = value_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            let element_type = core::LLVMTypeOf(value_ptr);
            let mut values = vec![value_ptr; num_elements as usize];
            core::LLVMConstArray2(element_type, values.as_mut_ptr(), values.len() as u64)
        };

        if array_type.is_null() {
            None
        } else {
            self.store_value(array_type)
        }
    }

    /// Creates a pointer
    pub fn create_pointer(&mut self, element_type_tag: TypeTag) -> Option<ValueTag> {
        let element_type_arc_rwlock = self.get_type(element_type_tag)?;

        let pointer_type = unsafe {
            let element_type_ptr = element_type_arc_rwlock.read().expect("Failed to lock type for reading").read(LLVMRefType::Type, |element_type_ref| {
                if let LLVMRef::Type(ptr) = element_type_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            core::LLVMConstPointerNull(core::LLVMPointerType(element_type_ptr, 0))
        };

        if pointer_type.is_null() {
            None
        } else {
            self.store_value(pointer_type)
        }
    }
    
    /// Creates an immutable (global) string
    pub fn create_string(
        &mut self,
        val: &str,
    ) -> Option<ValueTag> {

        let c_val = CString::new(val).expect("Failed to create CString for string value");

        let str_pointer = unsafe {
            core::LLVMConstString(c_val.as_ptr(), val.len() as u32, 1)
            
        };

        if !str_pointer.is_null() {
            return self.store_value(str_pointer);
        }
        
        
        None
    }

    /// Creates a mutable (local) string
    pub fn create_mut_string(
        &mut self,
        val: &str,
        context_tag: ContextTag,
        builder_tag: BuilderTag
    ) -> Option<ValueTag> {
        let context_arc_rwlock = self.get_context(context_tag)?;
        let builder_arc_rwlock = self.get_builder(builder_tag)?;

        let c_str_name = CString::new("local_str").expect("Failed to create CString for string name");

        let local_str = unsafe {
            let context_ptr = context_arc_rwlock.read().expect("Failed to lock context for reading").read(LLVMRefType::Context, |context_ref| {
                if let LLVMRef::Context(ptr) = context_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            let builder_ptr = builder_arc_rwlock.read().expect("Failed to lock builder for reading").read(LLVMRefType::Builder, |builder_ref| {
                if let LLVMRef::Builder(ptr) = builder_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            let i8_type = core::LLVMInt8TypeInContext(context_ptr);
            let str_type = core::LLVMArrayType2(i8_type, val.len() as u64);
            let local_str = core::LLVMBuildAlloca(builder_ptr.clone(), str_type, c_str_name.as_ptr());

            for (i, &byte) in val.as_bytes().iter().enumerate() {
                let index = core::LLVMConstInt(core::LLVMInt32TypeInContext(context_ptr), i as u64, 0);
                let mut indices = [index];
                let gep = core::LLVMBuildGEP2(builder_ptr.clone(), str_type, local_str, indices.as_mut_ptr(), indices.len() as u32, c_str_name.as_ptr());
                core::LLVMBuildStore(builder_ptr.clone(), core::LLVMConstInt(i8_type, byte as u64, 0), gep);
            }

            local_str
        };

        if local_str.is_null() {
            None
        } else {
            self.store_value(local_str)
        }
    }

    /// Creates a null pointer
    pub fn create_null_pointer(&mut self, ty_tag: TypeTag) -> Option<ValueTag> {
        let ty_arc_rwlock = self.get_type(ty_tag)?;

        let null_pointer = unsafe {
            let ty_ptr = ty_arc_rwlock.read().expect("Failed to lock type for reading").read(LLVMRefType::Type, |ty_ref| {
                if let LLVMRef::Type(ptr) = ty_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            core::LLVMConstPointerNull(ty_ptr)
        };

        if null_pointer.is_null() {
            None
        } else {
            self.store_value(null_pointer)
        }
    }

    /// Creates a continue statement
    pub fn create_continue_statement(
        &mut self,
        builder_tag: BuilderTag,
        continue_block_tag: BasicBlockTag
    ) -> Option<ValueTag> {
        let builder_arc_rwlock = self.get_builder(builder_tag)?;
        let continue_block_arc_rwlock = self.get_basic_block(continue_block_tag)?;

        let continue_statement = unsafe {
            let builder_ptr = builder_arc_rwlock.read().expect("Failed to lock builder for reading").read(LLVMRefType::Builder, |builder_ref| {
                if let LLVMRef::Builder(ptr) = builder_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            let continue_block_ptr = continue_block_arc_rwlock.read().expect("Failed to lock continue block for reading").read(LLVMRefType::Value, |continue_block_ref| {
                if let LLVMRef::BasicBlock(ptr) = continue_block_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            Some(core::LLVMBuildBr(builder_ptr, continue_block_ptr))
        };

        if let Some(continue_statement) = continue_statement {
            self.store_value(continue_statement)
        } else {
            None
        }
    }

    /// Creates a break statement
    pub fn create_break_statement(
        &mut self,
        builder_tag: BuilderTag,
        break_block_tag: BasicBlockTag
    ) -> Option<ValueTag> {
        let builder_arc_rwlock = self.get_builder(builder_tag)?;
        let break_block_arc_rwlock = self.get_basic_block(break_block_tag)?;

        let break_statement = unsafe {
            let builder_ptr = builder_arc_rwlock.read().expect("Failed to lock builder for reading").read(LLVMRefType::Builder, |builder_ref| {
                if let LLVMRef::Builder(ptr) = builder_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            let break_block_ptr = break_block_arc_rwlock.read().expect("Failed to lock break block for reading").read(LLVMRefType::Value, |break_block_ref| {
                if let LLVMRef::BasicBlock(ptr) = break_block_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            Some(core::LLVMBuildBr(builder_ptr, break_block_ptr))
        };

        if let Some(break_statement) = break_statement {
            self.store_value(break_statement)
        } else {
            None
        }
    }

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