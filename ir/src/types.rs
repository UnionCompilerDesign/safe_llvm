//! This module provides functionality for creating and managing LLVM types within the IR generator.
//!
//! It supports creating basic types like integers, floating points, and booleans, as well as complex types like pointers, arrays, and structures. 
//! This module also includes functionality for creating enumeration types and managing their variants.

extern crate llvm_sys as llvm;
use std::collections::HashMap;
use llvm::{core, prelude::LLVMTypeRef};
use common::pointer::{LLVMRef, LLVMRefType};
use super::core::{EnumDefinition, BuilderTag, ContextTag, IRGenerator, TypeTag, ValueTag};

impl IRGenerator {
    /// Returns the LLVM type for void in a specified context.
    ///
    /// # Parameters
    /// * `context_tag` - Context identifier where the type is to be created.
    ///
    /// # Returns
    /// Option<TypeTag> - The tag of the created void type or None if the creation fails.
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

    /// Returns the LLVM type for integers with a specified bit width.
    ///
    /// # Parameters
    /// * `context_tag` - Context identifier where the type is to be created.
    /// * `bits` - The bit width of the integer type.
    ///
    /// # Returns
    /// Option<TypeTag> - The tag of the created integer type or None if the creation fails.
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

    /// Returns the LLVM type for floating-point numbers in a specified context.
    ///
    /// # Parameters
    /// * `context_tag` - Context identifier where the type is to be created.
    ///
    /// # Returns
    /// Option<TypeTag> - The tag of the created float type or None if the creation fails.
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

    /// Returns the LLVM type for boolean values in a specified context.
    ///
    /// # Parameters
    /// * `context_tag` - Context identifier where the type is to be created.
    ///
    /// # Returns
    /// Option<TypeTag> - The tag of the created boolean type or None if the creation fails.
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
    ///
    /// # Parameters
    /// * `element_type_tag` - Type tag of the element that the pointer will point to.
    ///
    /// # Returns
    /// Option<TypeTag> - The tag of the created pointer type or None if the creation fails.
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

    /// Returns the LLVM array type for a specified element type and number of elements.
    ///
    /// # Parameters
    /// * `element_type_tag` - Type tag of the array's element.
    /// * `num_elements` - Number of elements in the array.
    ///
    /// # Returns
    /// Option<TypeTag> - The tag of the created array type or None if the creation fails.
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

    /// Returns the LLVM struct type for a specified set of element types.
    ///
    /// # Parameters
    /// * `context_tag` - Context identifier where the struct type is to be created.
    /// * `element_type_tags` - Array of type tags for the elements of the struct.
    /// * `packed` - Boolean flag indicating whether the struct should have packed alignment.
    ///
    /// # Returns
    /// Option<TypeTag> - The tag of the created struct type or None if the creation fails.
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
    ///
    /// # Parameters
    /// * `builder_tag` - Tag of the builder where the void return instruction will be inserted.
    ///
    /// # Returns
    /// Option<ValueTag> - The tag of the created void return instruction or None if the creation fails.
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

    /// Builds a return instruction with a specified value in the current function.
    ///
    /// # Parameters
    /// * `builder_tag` - Tag of the builder where the return instruction will be inserted.
    /// * `value_tag` - Tag of the value to be returned.
    ///
    /// # Returns
    /// Option<ValueTag> - The tag of the created return instruction or None if the creation fails.
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

    /// Allocates a function with specified return and parameter types in a given context, then stores it in the resource pool.
    ///
    /// # Parameters
    /// * `return_type_tag` - Optional tag of the return type for the function; if None, defaults to void.
    /// * `param_type_tags` - Array of tags for each parameter type of the function.
    /// * `is_var_arg` - Boolean indicating if the function accepts variable arguments.
    /// * `context_tag` - Context identifier where the function is to be created.
    ///
    /// # Returns
    /// Option<TypeTag> - The tag of the created function type or None if the creation fails.
    pub fn create_function(
        &mut self,
        return_type_tag: Option<TypeTag>,
        param_type_tags: &[TypeTag],
        is_var_arg: bool,
        context_tag: ContextTag,
    ) -> Option<TypeTag> {
        let context_arc_rwlock = self.get_context(context_tag)?;

        let context_ptr = context_arc_rwlock.read().expect("Failed to lock context for reading").read(LLVMRefType::Context, |context_ref| {
            if let LLVMRef::Context(ptr) = context_ref {
                Some(*ptr)
            } else {
                None
            }
        })?;

        let llvm_return_type = return_type_tag.map_or_else(|| unsafe { core::LLVMVoidTypeInContext(context_ptr) }, |tag| {
            let type_arc_rwlock = self.get_type(tag).expect("Failed to get type");
            let ptr = type_arc_rwlock.read().expect("Failed to lock type for reading").read(LLVMRefType::Type, |type_ref| {
                if let LLVMRef::Type(ptr) = type_ref {
                    Some(*ptr)
                } else {
                    None
                }
            }).expect("Failed to get return type"); 
            ptr
        });

        let mut llvm_param_types: Vec<LLVMTypeRef> = Vec::new();
        for tag in param_type_tags {
            let type_arc_rwlock = self.get_type(*tag)?;
            let type_ptr = type_arc_rwlock.read().expect("Failed to lock type for reading").read(LLVMRefType::Type, |type_ref| {
                if let LLVMRef::Type(ptr) = type_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;
            llvm_param_types.push(type_ptr);
        }

        let param_ptr = if llvm_param_types.is_empty() {
            std::ptr::null_mut()
        } else {
            llvm_param_types.as_mut_ptr()
        };

        let function_type = unsafe {
            core::LLVMFunctionType(llvm_return_type, param_ptr, llvm_param_types.len() as u32, is_var_arg as i32)
        };

        if function_type.is_null() {
            return None;
        }

        self.store_type(function_type)
    }


    /// Creates a new struct type in the LLVM context.
    ///
    /// # Parameters
    /// * `context_tag` - Context identifier where the struct type is to be created.
    /// * `member_types` - Vector of TypeTags representing each member of the struct.
    /// * `packed` - Boolean indicating if the struct should have packed alignment.
    ///
    /// # Returns
    /// Option<TypeTag> - The tag of the created struct type or None if the creation fails.
    pub fn create_struct(&mut self, context_tag: ContextTag, member_types: Vec<TypeTag>, packed: bool) -> Option<TypeTag> {
        let mut member_llvm_types: Vec<LLVMTypeRef> = member_types.iter()
            .map(|type_tag| {
                let type_arc_rwlock = self.get_type(*type_tag)?;
                let type_ptr = {
                    let type_rwlock = type_arc_rwlock.read().expect("Failed to lock type for reading");
                    type_rwlock.read(LLVMRefType::Type, |type_ref| {
                        if let LLVMRef::Type(ptr) = type_ref {
                            Some(*ptr)
                        } else {
                            None
                        }
                    })?
                };
                Some(type_ptr)
            })
            .collect::<Option<Vec<_>>>()?;

            let context_arc_rwlock = self.get_context(context_tag)?;
            let context_ptr = {
                let context_rwlock = context_arc_rwlock.read().expect("Failed to lock context for reading");
                context_rwlock.read(LLVMRefType::Context, |context_ref| {
                    if let LLVMRef::Context(ptr) = context_ref {
                        Some(*ptr)
                    } else {
                        None
                    }
                })?
            };

        let struct_type = unsafe {
            llvm::core::LLVMStructTypeInContext(context_ptr, member_llvm_types.as_mut_ptr(), member_llvm_types.len() as u32, packed as i32)
        };

        if struct_type.is_null() {
            None
        } else {
            self.store_type(struct_type)
        }
    }

    /// Creates an enum type represented by an integer of specified bit width and associated variants.
    ///
    /// # Parameters
    /// * `context_tag` - Context identifier where the enum type is to be created.
    /// * `num_bits` - The bit width of the integer representing the enum.
    /// * `name` - The name of the enum.
    /// * `variants` - A list of variant names for the enum, each mapped internally to an integer starting from 0.
    ///
    /// # Returns
    /// Option<TypeTag> - The tag of the created enum type or None if the creation fails.
    pub fn create_enum(&mut self, context_tag: ContextTag, num_bits: u32, name: &str, variants: &[String]) -> Option<TypeTag> {
        let context_arc_rwlock = self.get_context(context_tag)?;
        let context_ptr = {
            let context_rwlock = context_arc_rwlock.read().expect("Failed to lock context for reading");
            context_rwlock.read(LLVMRefType::Context, |context_ref| {
                if let LLVMRef::Context(ptr) = context_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?
        };

        let enum_type_ptr = unsafe { llvm::core::LLVMIntTypeInContext(context_ptr, num_bits) };
        
        if enum_type_ptr.is_null() {
            None
        } else {
            let type_tag = self.store_type(enum_type_ptr).expect("Failed to store type tag");
            let mut variant_map = HashMap::new();

            for (index, variant) in variants.iter().enumerate() {
                variant_map.insert(variant.clone(), index as i64);
            }

            self.store_enum_definition(type_tag, EnumDefinition::new(name.to_string(), variant_map));


            Some(type_tag)
        }
    }
}
