extern crate llvm_sys as llvm;

use llvm::core;

use crate::common::pointer::{LLVMRef, LLVMRefType};

use super::core::{BuilderTag, ContextTag, IRGenerator, TypeTag, ValueTag};

impl IRGenerator {
    /// Returns the LLVM type for void in a given context.
    pub fn void_type(&mut self, context_tag: ContextTag) -> Option<TypeTag> {
        let context_arc_rwlock = self.get_context(context_tag)?;
        let void_type = {
            let context_rwlock = context_arc_rwlock.read().expect("Failed to lock context for reading");
            let context_ptr = context_rwlock.read(LLVMRefType::Context, |context_ref| {
                if let LLVMRef::Context(ptr) = context_ref {
                    Some(unsafe { core::LLVMVoidTypeInContext(*ptr) })
                } else {
                    None
                }
            })?;
            context_ptr
        };

        if void_type.is_null() {
            None
        } else {
            self.store_type(void_type)
        }
    }

    /// Returns the LLVM type for integers of specified bit width in a given context.
    pub fn int_type(&mut self, context_tag: ContextTag, bits: u32) -> Option<TypeTag> {
        let context_arc_rwlock = self.get_context(context_tag)?;
        let int_type = {
            let context_rwlock = context_arc_rwlock.read().expect("Failed to lock context for reading");
            let context_ptr = context_rwlock.read(LLVMRefType::Context, |context_ref| {
                if let LLVMRef::Context(ptr) = context_ref {
                    Some(unsafe { core::LLVMIntTypeInContext(*ptr, bits) })
                } else {
                    None
                }
            })?;
            context_ptr
        };

        if int_type.is_null() {
            None
        } else {
            self.store_type(int_type)
        }
    }

    /// Returns the LLVM type for float in a given context.
    pub fn float_type(&mut self, context_tag: ContextTag) -> Option<TypeTag> {
        let context_arc_rwlock = self.get_context(context_tag)?;
        let float_type = {
            let context_rwlock = context_arc_rwlock.read().expect("Failed to lock context for reading");
            let context_ptr = context_rwlock.read(LLVMRefType::Context, |context_ref| {
                if let LLVMRef::Context(ptr) = context_ref {
                    Some(unsafe { core::LLVMFloatTypeInContext(*ptr) })
                } else {
                    None
                }
            })?;
            context_ptr
        };

        if float_type.is_null() {
            None
        } else {
            self.store_type(float_type)
        }
    }

    /// Returns the LLVM type for boolean in a given context.
    pub fn boolean_type(&mut self, context_tag: ContextTag) -> Option<TypeTag> {
        let context_arc_rwlock = self.get_context(context_tag)?;
        let boolean_type = {
            let context_rwlock = context_arc_rwlock.read().expect("Failed to lock context for reading");
            let context_ptr = context_rwlock.read(LLVMRefType::Context, |context_ref| {
                if let LLVMRef::Context(ptr) = context_ref {
                    Some(unsafe { core::LLVMInt1TypeInContext(*ptr) })
                } else {
                    None
                }
            })?;
            context_ptr
        };

        if boolean_type.is_null() {
            None
        } else {
            self.store_type(boolean_type)
        }
    }

    /// Returns the LLVM pointer type for a given element type.
    pub fn pointer_type(&mut self, element_type_tag: TypeTag) -> Option<TypeTag> {
        let element_type_arc_rwlock = self.get_type(element_type_tag)?;
        let pointer_type = {
            let element_type_rwlock = element_type_arc_rwlock.read().expect("Failed to lock type for reading");
            let element_type_ptr = element_type_rwlock.read(LLVMRefType::Type, |type_ref| {
                if let LLVMRef::Type(ptr) = type_ref {
                    Some(unsafe { core::LLVMPointerType(*ptr, 0) })
                } else {
                    None
                }
            })?;
            element_type_ptr
        };

        if pointer_type.is_null() {
            None
        } else {
            self.store_type(pointer_type)
        }
    }

    /// Returns the LLVM array type for a given element type and number of elements.
    pub fn array_type(&mut self, element_type_tag: TypeTag, num_elements: u64) -> Option<TypeTag> {
        let element_type_arc_rwlock = self.get_type(element_type_tag)?;
        let array_type = {
            let element_type_rwlock = element_type_arc_rwlock.read().expect("Failed to lock type for reading");
            let element_type_ptr = element_type_rwlock.read(LLVMRefType::Type, |type_ref| {
                if let LLVMRef::Type(ptr) = type_ref {
                    Some(unsafe { core::LLVMArrayType2(*ptr, num_elements) })
                } else {
                    None
                }
            })?;
            element_type_ptr
        };

        if array_type.is_null() {
            None
        } else {
            self.store_type(array_type)
        }
    }

    /// Returns the LLVM struct type for a given set of element types.
    pub fn struct_type(&mut self, context_tag: ContextTag, element_type_tags: &[TypeTag], packed: bool) -> Option<TypeTag> {
        let context_arc_rwlock = self.get_context(context_tag)?;
        let mut element_types = {
            let mut types = Vec::new();
            for &tag in element_type_tags {
                let element_type_arc_rwlock = self.get_type(tag)?;
                let element_type = {
                    let element_type_rwlock = element_type_arc_rwlock.read().expect("Failed to lock element type for reading");
                    element_type_rwlock.read(LLVMRefType::Type, |type_ref| {
                        if let LLVMRef::Type(ptr) = type_ref {
                            Some(*ptr)
                        } else {
                            None
                        }
                    })?
                };
                types.push(element_type);
            }
            types
        };

        let struct_type = {
            let context_rwlock = context_arc_rwlock.read().expect("Failed to lock context for reading");
            let context_ptr = context_rwlock.read(LLVMRefType::Context, |context_ref| {
                if let LLVMRef::Context(ptr) = context_ref {
                    Some(unsafe { core::LLVMStructTypeInContext(*ptr, element_types.as_mut_ptr(), element_types.len() as u32, packed as i32) })
                } else {
                    None
                }
            })?;
            context_ptr
        };

        if struct_type.is_null() {
            None
        } else {
            self.store_type(struct_type)
        }
    }

    /// Builds a void return instruction in the current function.
    pub fn void_return(&mut self, builder_tag: BuilderTag) -> Option<ValueTag> {
        let builder_arc_rwlock = self.get_builder(builder_tag)?;
        let void_return_inst = {
            let builder_rwlock = builder_arc_rwlock.read().expect("Failed to lock builder for reading");
            let builder_ptr = builder_rwlock.read(LLVMRefType::Builder, |builder_ref| {
                if let LLVMRef::Builder(ptr) = builder_ref {
                    Some(unsafe { core::LLVMBuildRetVoid(*ptr) })
                } else {
                    None
                }
            })?;
            builder_ptr
        };

        if void_return_inst.is_null() {
            None
        } else {
            self.store_value(void_return_inst)
        }
    }

    /// Builds a return instruction with a specified value.
    pub fn nonvoid_return(&mut self, builder_tag: BuilderTag, value_tag: ValueTag) -> Option<ValueTag> {
        let builder_arc_rwlock = self.get_builder(builder_tag)?;
        let value_arc_rwlock = self.get_value(value_tag)?;

        let nonvoid_return_inst = {
            let builder_rwlock = builder_arc_rwlock.read().expect("Failed to lock builder for reading");
            let builder_ptr = builder_rwlock.read(LLVMRefType::Builder, |builder_ref| {
                if let LLVMRef::Builder(ptr) = builder_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            let value_ptr = {
                let value_rwlock = value_arc_rwlock.read().expect("Failed to lock value for reading");
                let value_ref = value_rwlock.read(LLVMRefType::Value, |value_ref| {
                    if let LLVMRef::Value(ptr) = value_ref {
                        Some(*ptr)
                    } else {
                        None
                    }
                })?;
                value_ref
            };

            unsafe { core::LLVMBuildRet(builder_ptr, value_ptr) }
        };

        if nonvoid_return_inst.is_null() {
            None
        } else {
            self.store_value(nonvoid_return_inst)
        }
    }
}
