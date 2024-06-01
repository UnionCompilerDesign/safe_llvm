//! This module provides functionality to manipulate LLVM builder operations and handle
//! various arithmetic and logical instructions. This includes operations for creating
//! builders, adding functions to modules, and building basic arithmetic and logical
//! instructions using LLVM's Intermediate Representation (IR).

extern crate llvm_sys as llvm;

use std::ffi::CString;
use llvm::{core, prelude::LLVMBuilderRef, LLVMIntPredicate};
use common::pointer::{LLVMRef, LLVMRefType};
use crate::core::{BuilderTag, ContextTag, IRGenerator, ModuleTag, TypeTag, ValueTag};

impl IRGenerator {
    /// Allocates a builder in a specified context and stores it in the resource pool.
    ///
    /// # Parameters
    /// * `context_tag` - The context within which the builder is created.
    ///
    /// # Returns
    /// Option<BuilderTag> - The tag of the newly created builder or None if the builder cannot be created.
    pub fn create_builder(&mut self, context_tag: ContextTag) -> Option<BuilderTag> {

        let context_arc_rwlock = self.get_context(context_tag)?;

        let builder_ptr: LLVMBuilderRef = unsafe {
            let context_rwlock = context_arc_rwlock.read().expect("Failed to lock context for reading");
            let context_ptr = context_rwlock.read(LLVMRefType::Context, |context_ref| {
                if let LLVMRef::Context(ptr) = context_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            core::LLVMCreateBuilderInContext(context_ptr)
        };


        if builder_ptr.is_null() {
            return None;
        }

        self.store_builder(builder_ptr)
    }

    /// Adds a function to a module.
    ///
    /// # Parameters
    /// * `module_tag` - Tag of the module to which the function is being added.
    /// * `function_name` - The name of the function.
    /// * `function_type_tag` - The tag representing the type of the function.
    ///
    /// # Returns
    /// Option<ValueTag> - The tag of the newly added function or None if the function cannot be added.
    pub fn add_function_to_module(&mut self, module_tag: ModuleTag, function_name: &str, function_type_tag: TypeTag) -> Option<ValueTag> {
        let module_arc_rwlock = self.get_module(module_tag)?;
        let function_type_arc_rwlock = self.get_type(function_type_tag)?;

        let c_name = CString::new(function_name).expect("Failed to create CString for function name");

        let function = {
            let module_rwlock = module_arc_rwlock.read().expect("Failed to lock module for reading");
            let module_ptr = module_rwlock.read(LLVMRefType::Module, |module_ref| {
                if let LLVMRef::Module(ptr) = module_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            let function_type_ptr = {
                let function_type_rwlock = function_type_arc_rwlock.read().expect("Failed to lock function type for reading");
                function_type_rwlock.read(LLVMRefType::Type, |type_ref| {
                    if let LLVMRef::Type(ptr) = type_ref {
                        Some(*ptr)
                    } else {
                        None
                    }
                })?
            };

            unsafe { core::LLVMAddFunction(module_ptr, c_name.as_ptr(), function_type_ptr) }
        };

        if function.is_null() {
            None
        } else {
            self.store_value(function)
        }
    }

    /// Builds a logical 'AND' operation between two values.
    ///
    /// # Parameters
    /// * `builder_tag` - Tag of the builder to use for this operation.
    /// * `param_a_tag` - First operand.
    /// * `param_b_tag` - Second operand.
    /// * `name` - The name for the newly created instruction.
    ///
    /// # Returns
    /// Option<ValueTag> - The result of the logical AND operation or None if the operation fails.
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

    /// Builds a logical 'OR' operation between two values.
    ///
    /// # Parameters
    /// * `builder_tag` - Tag of the builder to use for this operation.
    /// * `param_a_tag` - First operand.
    /// * `param_b_tag` - Second operand.
    /// * `name` - The name for the newly created instruction.
    ///
    /// # Returns
    /// Option<ValueTag> - The result of the logical OR operation or None if the operation fails.
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

    /// Builds a logical 'XOR' operation between two values.
    ///
    /// # Parameters
    /// * `builder_tag` - Tag of the builder to use for this operation.
    /// * `param_a_tag` - First operand.
    /// * `param_b_tag` - Second operand.
    /// * `name` - The name for the newly created instruction.
    ///
    /// # Returns
    /// Option<ValueTag> - The result of the logical XOR operation or None if the operation fails.
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

    /// Builds a left shift operation between two values.
    ///
    /// # Parameters
    /// * `builder_tag` - Tag of the builder to use for this operation.
    /// * `param_a_tag` - Value to be shifted.
    /// * `param_b_tag` - Number of positions to shift.
    /// * `name` - The name for the newly created instruction.
    ///
    /// # Returns
    /// Option<ValueTag> - The result of the shift operation or None if the operation fails.
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

    /// Builds a right shift operation between two values.
    ///
    /// # Parameters
    /// * `builder_tag` - Tag of the builder to use for this operation.
    /// * `param_a_tag` - Value to be shifted.
    /// * `param_b_tag` - Number of positions to shift.
    /// * `name` - The name for the newly created instruction.
    ///
    /// # Returns
    /// Option<ValueTag> - The result of the shift operation or None if the operation fails.
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

    /// Builds a 'greater than' comparison between two values.
    ///
    /// # Parameters
    /// * `builder_tag` - Tag of the builder to use for this operation.
    /// * `param_a_tag` - First operand.
    /// * `param_b_tag` - Second operand.
    /// * `name` - The name for the newly created instruction.
    ///
    /// # Returns
    /// Option<ValueTag> - The result of the comparison or None if the operation fails.
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

    /// Builds a 'less than' comparison between two values.
    ///
    /// # Parameters
    /// * `builder_tag` - Tag of the builder to use for this operation.
    /// * `param_a_tag` - First operand.
    /// * `param_b_tag` - Second operand.
    /// * `name` - The name for the newly created instruction.
    ///
    /// # Returns
    /// Option<ValueTag> - The result of the comparison or None if the operation fails.
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

    /// Builds an 'equal to' comparison between two values.
    ///
    /// # Parameters
    /// * `builder_tag` - Tag of the builder to use for this operation.
    /// * `param_a_tag` - First operand.
    /// * `param_b_tag` - Second operand.
    /// * `name` - The name for the newly created instruction.
    ///
    /// # Returns
    /// Option<ValueTag> - The result of the comparison or None if the operation fails.
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

    /// Builds a negation operation on a single value.
    ///
    /// # Parameters
    /// * `builder_tag` - Tag of the builder to use for this operation.
    /// * `operand_tag` - The operand to be negated.
    /// * `name` - The name for the newly created instruction.
    ///
    /// # Returns
    /// Option<ValueTag> - The result of the negation or None if the operation fails.
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

    /// Builds a bitwise NOT operation on a single value.
    ///
    /// # Parameters
    /// * `builder_tag` - Tag of the builder to use for this operation.
    /// * `operand_tag` - The operand on which the NOT operation is to be performed.
    /// * `name` - The name for the newly created instruction.
    ///
    /// # Returns
    /// Option<ValueTag> - The result of the bitwise NOT operation or None if the operation fails.
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

        /// Builds a logical NOT operation on a single value.
    ///
    /// # Parameters
    /// * `builder_tag` - Tag of the builder to use for this operation.
    /// * `context_tag` - Context in which the operation is performed.
    /// * `operand_tag` - The operand on which the NOT operation is to be performed.
    /// * `name` - The name for the newly created instruction.
    ///
    /// # Returns
    /// Option<ValueTag> - The result of the logical NOT operation or None if the operation fails.
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

    /// Builds an addition operation between two values.
    ///
    /// # Parameters
    /// * `builder_tag` - Tag of the builder to use for this operation.
    /// * `param_a_tag` - First operand for the addition.
    /// * `param_b_tag` - Second operand for the addition.
    /// * `name` - The name for the newly created instruction.
    ///
    /// # Returns
    /// Option<ValueTag> - The result of the addition operation or None if the operation fails.
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

    /// Builds a subtraction operation between two values.
    ///
    /// # Parameters
    /// * `builder_tag` - Tag of the builder to use for this operation.
    /// * `param_a_tag` - First operand for the subtraction.
    /// * `param_b_tag` - Second operand for the subtraction.
    /// * `name` - The name for the newly created instruction.
    ///
    /// # Returns
    /// Option<ValueTag> - The result of the subtraction operation or None if the operation fails.
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

    /// Builds a multiplication operation between two values.
    ///
    /// # Parameters
    /// * `builder_tag` - Tag of the builder to use for this operation.
    /// * `param_a_tag` - First operand for the multiplication.
    /// * `param_b_tag` - Second operand for the multiplication.
    /// * `name` - The name for the newly created instruction.
    ///
    /// # Returns
    /// Option<ValueTag> - The result of the multiplication operation or None if the operation fails.
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

    /// Builds a division operation between two values.
    ///
    /// # Parameters
    /// * `builder_tag` - Tag of the builder to use for this operation.
    /// * `param_a_tag` - Dividend operand.
    /// * `param_b_tag` - Divisor operand.
    /// * `name` - The name for the newly created instruction.
    ///
    /// # Returns
    /// Option<ValueTag> - The result of the division operation or None if the operation fails.
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

    /// Builds a remainder operation between two values.
    ///
    /// # Parameters
    /// * `builder_tag` - Tag of the builder to use for this operation.
    /// * `param_a_tag` - Dividend operand.
    /// * `param_b_tag` - Divisor operand.
    /// * `name` - The name for the newly created instruction.
    ///
    /// # Returns
    /// Option<ValueTag> - The result of the remainder operation or None if the operation fails.
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
}
