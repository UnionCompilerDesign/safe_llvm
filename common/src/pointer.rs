//! Provides utilities for safe, synchronized access to raw LLVM pointers.
//!
//! This module defines structures and enums to safely handle raw pointers across multiple threads,
//! encapsulating them within `Arc` and `RwLock` for concurrent access.

extern crate llvm_sys as llvm;
use llvm::{execution_engine::{self, LLVMExecutionEngineRef}, prelude::{LLVMBasicBlockRef, LLVMBuilderRef, LLVMContextRef, LLVMModuleRef, LLVMTypeRef, LLVMValueRef}};
use llvm_sys::core::LLVMContextDispose;
use std::{ffi::c_void, ptr::NonNull, sync::{Arc, RwLock}};

/// Enum to represent various LLVM references for type management.
#[derive(Debug, Clone, Copy)]
pub enum LLVMRef {
    /// Encapsulates an LLVM context, a mechanism for holding the global
    /// data used throughout the LLVM infrastructure during the compilation process.
    Context(LLVMContextRef), // https://llvm.org/doxygen/classllvm_1_1LLVMContext.html
    /// Represents an LLVM module, which is a collection of related code that
    /// together forms a program or part of a program. It is the primary unit of code packaging in LLVM.
    Module(LLVMModuleRef), // https://llvm.org/doxygen/classllvm_1_1Module.html
    /// Represents a value in the LLVM system, which could be a function argument,
    /// a local variable, a constant, or any type of data.
    Value(LLVMValueRef), // https://llvm.org/doxygen/classllvm_1_1Value.html
    /// Represents a basic block, which is a straight-line code sequence
    /// with no branches in except to the entry and no branches out except at the exit.
    BasicBlock(LLVMBasicBlockRef), // https://llvm.org/doxygen/classllvm_1_1BasicBlock.html
    /// Provides a mechanism to construct an LLVM IR. Builders are the means
    /// through which instructions are constructed for LLVM programs.
    Builder(LLVMBuilderRef), // https://www.llvmpy.org/llvmpy-doc/0.12.7/doc/llvm.core.Builder.html
    /// Represents a type in LLVM, which could be a primitive type (like integer or
    /// floating-point), or a derived type (like arrays, pointers, structures, etc.).
    Type(LLVMTypeRef), // https://llvm.org/doxygen/classllvm_1_1Type.html
    /// Represents an LLVM execution engine, which is capable
    /// of compiling and executing LLVM bitcode to native machine code.
    ExecutionEngine(LLVMExecutionEngineRef) // https://llvm.org/doxygen/group__LLVMCExecutionEngine.html
}

/// Represents types of LLVM references for runtime conversion.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LLVMRefType {
    /// Refers to an LLVM context, which is an environment in which LLVM IR
    /// generation and optimizations occur. All LLVM entities (modules, types, values)
    /// belong to a specific context.
    Context,
    /// Represents an LLVM module, which acts as a container for function
    /// definitions, global variables, and symbol declarations. 
    Module,
    /// Encompasses any value computed or used by a program. In LLVM, everything
    /// from functions to variables to constants is a value.
    Value,
    /// A basic block is a sequence of instructions that execute sequentially.
    /// It is a single-entry, single-exit section of code with no branches except at
    /// the entry and the exit.
    BasicBlock,
    /// Used for building LLVM IR. It maintains a current position in a block
    /// where new instructions are inserted.
    Builder,
    /// Represents a data type in LLVM. Types are metadata that describe the high-level
    /// structure of data used within modules and provide the type system for LLVM IR.  
    Type,
    /// Manages the execution of compiled LLVM code. It is responsible for
    /// compiling LLVM IR to machine code and executing or providing access to that code.
    ExecutionEngine,
}

/// Helper methods for the LLVMRef enum to manage raw pointer conversions safely.
impl LLVMRef {
    /// Converts an LLVMRef to a raw pointer.
    /// 
    /// # Returns
    /// A raw pointer to the underlying LLVM object.
    fn to_raw(self) -> *mut c_void {
        match self {
            LLVMRef::Context(ptr) => ptr as *mut c_void,
            LLVMRef::Module(ptr) => ptr as *mut c_void,
            LLVMRef::Value(ptr) => ptr as *mut c_void,
            LLVMRef::BasicBlock(ptr) => ptr as *mut c_void,
            LLVMRef::Builder(ptr) => ptr as *mut c_void,
            LLVMRef::Type(ptr) => ptr as *mut c_void,
            LLVMRef::ExecutionEngine(ptr) => ptr as *mut c_void,
        }
    }

    /// Constructs an LLVMRef from a raw pointer based on the specified kind.
    /// 
    /// # Parameters
    /// * `ptr` - A raw pointer to the underlying LLVM object.
    /// * `kind` - A kind of LLVMRef to create, which ensures the type safety of the raw pointer.
    /// 
    /// # Safety
    /// This function is unsafe because it assumes the raw pointer is valid and properly typed.
    /// 
    /// # Returns
    /// A new instance of LLVMRef corresponding to the specified type.
    unsafe fn from_raw(ptr: *mut c_void, kind: LLVMRefType) -> Self {
        match kind {
            LLVMRefType::Context => LLVMRef::Context(ptr as LLVMContextRef),
            LLVMRefType::Module => LLVMRef::Module(ptr as LLVMModuleRef),
            LLVMRefType::Value => LLVMRef::Value(ptr as LLVMValueRef),
            LLVMRefType::BasicBlock => LLVMRef::BasicBlock(ptr as LLVMBasicBlockRef),
            LLVMRefType::Builder => LLVMRef::Builder(ptr as LLVMBuilderRef),
            LLVMRefType::Type => LLVMRef::Type(ptr as LLVMTypeRef),
            LLVMRefType::ExecutionEngine => LLVMRef::ExecutionEngine(ptr as LLVMExecutionEngineRef),
        }
    }
}

/// Thread-safe pointer type for managing raw C pointers in a synchronized context.
pub struct SafeLLVMPointer {
    ptr: Arc<RwLock<NonNull<c_void>>>,
    kind: LLVMRefType,
}

impl SafeLLVMPointer {
    /// Constructs a new `SafeLLVMPointer` by taking an `LLVMRef` and converting it to a non-null raw pointer.
    /// 
    /// # Parameters
    /// * `llvm_ref` - An `LLVMRef` to be encapsulated.
    /// 
    /// # Returns
    /// An `Option` wrapped `SafeLLVMPointer` if the pointer is non-null, `None` otherwise.
    pub fn new(llvm_ref: LLVMRef, kind: LLVMRefType) -> Option<Self> {
        let raw_ptr = llvm_ref.to_raw();
        NonNull::new(raw_ptr).map(|nn_ptr| SafeLLVMPointer {
            ptr: Arc::new(RwLock::new(nn_ptr)),
            kind,
        })
    }

    /// Provides read-only access to the pointed-to value.
    /// 
    /// # Parameters
    /// * `kind` - The type of LLVM reference expected.
    /// * `f` - A closure that is executed with a reference to the value, allowing safe access.
    /// 
    /// # Type Parameters
    /// * `Closure` - A closure or function pointer that takes a reference to an `LLVMRef`.
    /// It performs operations based on the type of reference it receives.
    /// * `ReturnType` - The return type of the closure `Closure`. This type is determined by
    /// the return value of the closure. 
    /// 
    /// # Returns
    /// The result of the closure, which can be any type defined by the closure's functionality.
    /// 
    /// # Errors
    /// If the RwLock is poisoned, an error will be thrown, crashing the program or propagating the panic.
    pub fn read<Closure, ReturnType>(&self, kind: LLVMRefType, closure: Closure) -> ReturnType
    where
        Closure: FnOnce(&LLVMRef) -> ReturnType,
    {
        if kind == self.kind {
            let lock = self.ptr.read().expect("RwLock has been poisoned");
            let ref_to_value = unsafe { LLVMRef::from_raw(lock.as_ptr(), kind) };
            return closure(&ref_to_value);
        }
        panic!("Incorrect type for read.")
    }

    /// Provides write access to the pointed-to value.
    /// 
    /// # Parameters
    /// * `kind` - The type of LLVM reference expected.
    /// * `closure` - A closure that is executed with a mutable reference to the value, allowing safe modifications.
    /// 
    /// # Returns
    /// The result of the closure, which can be any type defined by the closure's functionality.
    /// 
    /// # Errors
    /// If the RwLock is poisoned, an error will be thrown, crashing the program or propagating the panic.
    pub fn write<Closure, ReturnType>(&self, kind: LLVMRefType, closure: Closure) -> ReturnType
    where
        Closure: FnOnce(&mut LLVMRef) -> ReturnType,
    {
        if kind == self.kind {
            let lock = self.ptr.write().expect("RwLock has been poisoned");
            let mut ref_to_mut_value = unsafe { LLVMRef::from_raw(lock.as_ptr(), kind) };
            return closure(&mut ref_to_mut_value);
        }
        panic!("Incorrect type for read.")
    }
}

impl Drop for SafeLLVMPointer {
    fn drop(&mut self) {
        if let Ok(lock) = self.ptr.write() {
            let raw_ptr = lock.as_ptr();
            if !raw_ptr.is_null() {
                match self.kind {
                    LLVMRefType::ExecutionEngine => {
                        let llvm_ref = unsafe { LLVMRef::from_raw(raw_ptr, LLVMRefType::ExecutionEngine) };
                        if let LLVMRef::ExecutionEngine(engine) = llvm_ref {
                            unsafe {
                                execution_engine::LLVMDisposeExecutionEngine(engine);
                            }
                        }
                    },
                    LLVMRefType::Context => {
                        let llvm_ref = unsafe { LLVMRef::from_raw(raw_ptr, LLVMRefType::Context) };
                        if let LLVMRef::Context(context) = llvm_ref {
                            unsafe {
                                LLVMContextDispose(context);
                            }
                        }
                    },
                    _ => {}  // No action needed for other types
                }
            } else {
                eprintln!("Attempted to dispose a null pointer for {:?}", self.kind);
            }
        } else {
            eprintln!("Failed to acquire lock on LLVM pointer for disposal");
        }
    }
}