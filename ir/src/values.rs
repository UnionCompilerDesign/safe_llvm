//! This module provides functionality to create and manage LLVM values within the IR generator.
//!
//! It includes operations for creating various types of constants, arrays, pointers, and strings, as well as handling control flow constructs like continue and break statements.

extern crate llvm_sys as llvm;

use llvm::core;
use std::ffi::CString;
use common::pointer::{LLVMRef, LLVMRefType};
use super::core::{BasicBlockTag, BuilderTag, ContextTag, IRManager, TypeTag, ValueTag};

impl IRManager {
    /// Creates a 64-bit integer constant in a specified context.
    ///
    /// # Parameters
    /// * `context_tag` - Context identifier where the integer constant is to be created.
    /// * `val` - The integer value to be converted into a constant.
    ///
    /// # Returns
    /// Option<ValueTag> - The tag of the created integer constant or None if the creation fails.
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

    /// Creates a floating-point constant in a specified context.
    ///
    /// # Parameters
    /// * `context_tag` - Context identifier where the floating-point constant is to be created.
    /// * `val` - The float value to be converted into a constant.
    ///
    /// # Returns
    /// Option<ValueTag> - The tag of the created floating-point constant or None if the creation fails.
    pub fn create_float(&mut self, context_tag: ContextTag, val: f64) -> Option<ValueTag> {
        let context_arc_rwlock = self.get_context(context_tag)?;
        let float_value = {
            let context_rwlock = context_arc_rwlock.read().expect("Failed to lock context for reading");
            let context_ptr = context_rwlock.read(LLVMRefType::Context, |context_ref| {
                if let LLVMRef::Context(ptr) = context_ref {
                    Some(unsafe { core::LLVMConstReal(core::LLVMFloatTypeInContext(*ptr), val) })
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

    /// Creates a boolean constant in a specified context.
    ///
    /// # Parameters
    /// * `context_tag` - Context identifier where the boolean constant is to be created.
    /// * `val` - The boolean value to be converted into a constant.
    ///
    /// # Returns
    /// Option<ValueTag> - The tag of the created boolean constant or None if the creation fails.
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

    /// Creates an array type constant.
    ///
    /// # Parameters
    /// * `value_tag` - Tag of the value to be used as an array element.
    /// * `num_elements` - Number of elements in the array.
    ///
    /// # Returns
    /// Option<ValueTag> - The tag of the created array or None if the creation fails.
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

    /// Creates a pointer constant that points to a type.
    ///
    /// # Parameters
    /// * `element_type_tag` - Type tag of the element to which the pointer will point.
    ///
    /// # Returns
    /// Option<ValueTag> - The tag of the created pointer or None if the creation fails.
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
    
    /// Creates an immutable (global) string constant.
    ///
    /// # Parameters
    /// * `val` - The string value to be converted into a global string constant.
    ///
    /// # Returns
    /// Option<ValueTag> - The tag of the created string constant or None if the creation fails.
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

    /// Creates a mutable (local) string in the specified context and builder.
    ///
    /// # Parameters
    /// * `val` - The string value to be converted into a local string.
    /// * `context_tag` - Context identifier where the local string is to be created.
    /// * `builder_tag` - Builder tag used to create the local string.
    ///
    /// # Returns
    /// Option<ValueTag> - The tag of the created local string or None if the creation fails.
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

    /// Creates a null pointer for a specified type.
    ///
    /// # Parameters
    /// * `ty_tag` - Type tag for which the null pointer is created.
    ///
    /// # Returns
    /// Option<ValueTag> - The tag of the created null pointer or None if the creation fails.
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

    /// Creates a continue statement, directing control flow to continue at the specified block.
    ///
    /// # Parameters
    /// * `builder_tag` - Builder tag used to create the continue statement.
    /// * `continue_block_tag` - Basic block tag where the continue statement will jump.
    ///
    /// # Returns
    /// Option<ValueTag> - The tag of the created continue statement or None if the creation fails.
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

    /// Creates a break statement, directing control flow to break out to the specified block.
    ///
    /// # Parameters
    /// * `builder_tag` - Builder tag used to create the break statement.
    /// * `break_block_tag` - Basic block tag where the break statement will jump.
    ///
    /// # Returns
    /// Option<ValueTag> - The tag of the created break statement or None if the creation fails.
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

    /// Retrieves a function parameter by its index.
    ///
    /// # Parameters
    /// * `function_tag` - Tag of the function from which the parameter is retrieved.
    /// * `index` - The zero-based index of the parameter to retrieve.
    ///
    /// # Returns
    /// Option<ValueTag> - The tag of the retrieved parameter or None if the parameter cannot be retrieved.
    pub fn get_param(&mut self, function_tag: ValueTag, index: u32) -> Option<ValueTag> {
        let function_arc_rwlock = self.get_value(function_tag)?;
        
        let param = {
            let function_rwlock = function_arc_rwlock.read().expect("Failed to lock function for reading");
            let function_ptr = function_rwlock.read(LLVMRefType::Value, |value_ref| {
                if let LLVMRef::Value(ptr) = value_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            unsafe { core::LLVMGetParam(function_ptr, index) }
        };

        if param.is_null() {
            None
        } else {
            self.store_value(param)
        }
    }
}