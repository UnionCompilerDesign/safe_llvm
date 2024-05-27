/*  
    A struct for managing raw pointers in a multi-threaded context using Arc and RwLock.
    This struct provides immutable and mutable access to raw pointers within a non-null wrapper.
*/

extern crate llvm_sys as llvm;

use llvm::{core, prelude::{LLVMBasicBlockRef, LLVMBuilderRef, LLVMContextRef, LLVMModuleRef, LLVMTypeRef, LLVMValueRef}};

use std::{ffi::c_void, fmt, ptr::NonNull, sync::{Arc, PoisonError, RwLock}};

/// Enum to represent various LLVM reference types for better type management and safety.
#[derive(Debug, Clone, Copy)]
pub enum LLVMRef {
    Context(LLVMContextRef), // https://llvm.org/doxygen/classllvm_1_1LLVMContext.html
    Module(LLVMModuleRef), // https://llvm.org/doxygen/classllvm_1_1Module.html
    Value(LLVMValueRef), // https://llvm.org/doxygen/classllvm_1_1Value.html
    BasicBlock(LLVMBasicBlockRef), // https://llvm.org/doxygen/classllvm_1_1BasicBlock.html
    Builder(LLVMBuilderRef), // https://www.llvmpy.org/llvmpy-doc/0.12.7/doc/llvm.core.Builder.html
    Type(LLVMTypeRef), // https://llvm.org/doxygen/classllvm_1_1Type.html
}

/// Represents types of LLVM references to aid in safe runtime conversion.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LLVMRefType {
    Context,
    Module,
    Value,
    BasicBlock,
    Builder,
    Type,
}


/// Helper methods for the LLVMRef enum to manage raw pointer conversions safely.
impl LLVMRef {
    /// Converts an LLVMRef to a raw pointer.
    fn to_raw(self) -> Result<*mut c_void, SafeLLVMError> {
        let ptr = match self {
            LLVMRef::Context(ptr) => ptr as *mut c_void,
            LLVMRef::Module(ptr) => ptr as *mut c_void,
            LLVMRef::Value(ptr) => ptr as *mut c_void,
            LLVMRef::BasicBlock(ptr) => ptr as *mut c_void,
            LLVMRef::Builder(ptr) => ptr as *mut c_void,
            LLVMRef::Type(ptr) => ptr as *mut c_void,
        };
        if ptr.is_null() {
            Err(SafeLLVMError::InvalidPointer("Attempted to convert a null LLVMRef to a raw pointer".into()))
        } else {
            Ok(ptr)
        }
    }

    /// Constructs an LLVMRef from a raw pointer based on the specified kind.
    /// This is unsafe because it assumes the raw pointer is valid and properly typed.
    unsafe fn from_raw(ptr: *mut c_void, kind: LLVMRefType) -> Result<Self, SafeLLVMError> {
        if ptr.is_null() {
            return Err(SafeLLVMError::InvalidPointer("Attempted to convert a null raw pointer to LLVMRef".into()));
        }
        Ok(match kind {
            LLVMRefType::Context => LLVMRef::Context(ptr as LLVMContextRef),
            LLVMRefType::Module => LLVMRef::Module(ptr as LLVMModuleRef),
            LLVMRefType::Value => LLVMRef::Value(ptr as LLVMValueRef),
            LLVMRefType::BasicBlock => LLVMRef::BasicBlock(ptr as LLVMBasicBlockRef),
            LLVMRefType::Builder => LLVMRef::Builder(ptr as LLVMBuilderRef),
            LLVMRefType::Type => LLVMRef::Type(ptr as LLVMTypeRef),
        })
    }
}

/// Thread-safe pointer type for managing raw C pointers in a synchronized context.
pub struct SafeLLVMPointer {
    ptr: Arc<RwLock<NonNull<c_void>>>,
    kind: LLVMRefType,
}

impl SafeLLVMPointer {
    /// Constructs a new `SafeLLVMPointer` by taking an `LLVMRef` and converting it to a non-null raw pointer.
    /// Returns an `Option` wrapped instance of `SafeLLVMPointer` if the pointer is non-null.
    pub fn new(llvm_ref: LLVMRef, kind: LLVMRefType) -> Result<Self, SafeLLVMError> {
        let raw_ptr = llvm_ref.to_raw()?;
        NonNull::new(raw_ptr).map(|nn_ptr| SafeLLVMPointer {
            ptr: Arc::new(RwLock::new(nn_ptr)),
            kind
        }).ok_or(SafeLLVMError::InvalidPointer("Failed to create a non-null pointer from LLVMRef".into()))
    }

    /// Provides read-only access to the pointed-to value.
    /// The read operation is safely performed within the bounds of an `RwLock`, ensuring no concurrent write operations.
    /// A closure receives an immutable reference to the value of `LLVMRef`.
    pub fn read<FnType, ReturnType>(&self, kind: LLVMRefType, f: FnType) -> Result<ReturnType, SafeLLVMError>
    where
        FnType: FnOnce(&LLVMRef) -> ReturnType,
    {
        if kind != self.kind {
            return Err(SafeLLVMError::IncorrectPointerType(format!(
                "Expected: {:?}, Actual: {:?}",
                self.kind, kind
            )));        
        }
    
        match self.ptr.read() {
            Ok(lock) => {
                let ref_to_value = unsafe { LLVMRef::from_raw(lock.as_ptr(), kind)? };
                Ok(f(&ref_to_value))
            },
            Err(e) => Err(SafeLLVMError::LockError(format!("Failed to acquire read lock: {}", e))),
        }
    }

    /// Provides write access to the pointed-to value.
    /// The write operation ensures exclusive, mutable access via an `RwLock`.
    /// A closure receives a mutable reference to the value of `LLVMRef`.
    pub fn write<FnType, ReturnType>(&self, kind: LLVMRefType, f: FnType) -> Result<ReturnType, SafeLLVMError>
    where
        FnType: FnOnce(&mut LLVMRef) -> ReturnType,
    {
        if kind != self.kind {
            return Err(SafeLLVMError::IncorrectPointerType(format!(
                "Expected: {:?}, Actual: {:?}",
                self.kind, kind
            )));
        }

        match self.ptr.write() {
            Ok(mut lock) => {
                let mut ref_to_mut_value = unsafe { LLVMRef::from_raw(lock.as_ptr(), kind)? };
                Ok(f(&mut ref_to_mut_value))
            },
            Err(e) => Err(SafeLLVMError::LockError(format!("Failed to acquire write lock: {}", e))),
        }
    }
}

impl Drop for SafeLLVMPointer {
    fn drop(&mut self) {
        let ptr = self.ptr.write().expect("Lock poisoned on drop.");
        let raw_ptr = ptr.as_ptr();  

        unsafe {
            match self.kind {
                LLVMRefType::Context => core::LLVMContextDispose(raw_ptr as LLVMContextRef),
                LLVMRefType::Module => core::LLVMDisposeModule(raw_ptr as LLVMModuleRef),
                LLVMRefType::Builder => core::LLVMDisposeBuilder(raw_ptr as LLVMBuilderRef),
                LLVMRefType::Value | 
                LLVMRefType::BasicBlock |
                LLVMRefType::Type => {}
            }
        }
    }
}

// Error type for SafeLLVMPointer operations
#[derive(Debug)]
pub enum SafeLLVMError {
    InvalidPointer(String),
    IncorrectPointerType(String),
    LockError(String),
}

impl fmt::Display for SafeLLVMError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SafeLLVMError::InvalidPointer(msg) => write!(f, "Invalid pointer error: {}", msg),
            SafeLLVMError::LockError(msg) => write!(f, "Lock error: {}", msg),
            SafeLLVMError::IncorrectPointerType(msg) => write!(f, "Incorrect pointer type: {}", msg),
        }
    }
}

impl<T> From<PoisonError<T>> for SafeLLVMError {
    fn from(error: PoisonError<T>) -> Self {
        SafeLLVMError::LockError(format!("{}", error))
    }
}