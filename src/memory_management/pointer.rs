/*  
    A struct for managing raw pointers in a multi-threaded context using Arc and RwLock.
    This struct provides controlled, mutable access to raw pointers encapsulated within a non-null safety wrapper.
    It is intended for use in environments where multi-threaded access requires synchronization to prevent race conditions.
*/

extern crate llvm_sys as llvm;

use llvm::prelude::{LLVMBasicBlockRef, LLVMBuilderRef, LLVMContextRef, LLVMModuleRef, LLVMTypeRef, LLVMValueRef};

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

/// Helper methods for the LLVMRef enum to manage raw pointer conversions safely.
impl LLVMRef {
    /// Converts an LLVMRef to a raw pointer.
    fn to_raw(self) -> *mut c_void {
        match self {
            LLVMRef::Context(ptr) => ptr as *mut c_void,
            LLVMRef::Module(ptr) => ptr as *mut c_void,
            LLVMRef::Value(ptr) => ptr as *mut c_void,
            LLVMRef::BasicBlock(ptr) => ptr as *mut c_void,
            LLVMRef::Builder(ptr) => ptr as *mut c_void,
            LLVMRef::Type(ptr) => ptr as *mut c_void,
        }
    }

    /// Constructs an LLVMRef from a raw pointer based on the specified kind.
    /// This is unsafe because it assumes the raw pointer is valid and properly typed.
    unsafe fn from_raw(ptr: *mut c_void, kind: LLVMRefType) -> Self {
        match kind {
            LLVMRefType::Context => LLVMRef::Context(ptr as LLVMContextRef),
            LLVMRefType::Module => LLVMRef::Module(ptr as LLVMModuleRef),
            LLVMRefType::Value => LLVMRef::Value(ptr as LLVMValueRef),
            LLVMRefType::BasicBlock => LLVMRef::BasicBlock(ptr as LLVMBasicBlockRef),
            LLVMRefType::Builder => LLVMRef::Builder(ptr as LLVMBuilderRef),
            LLVMRefType::Type => LLVMRef::Type(ptr as LLVMTypeRef),
        }
    }
}

/// Thread-safe pointer type for managing raw C pointers in a synchronized context.
pub struct SafeLLVMPointer {
    ptr: Arc<RwLock<NonNull<c_void>>>,
}

impl SafeLLVMPointer {
    /// Constructs a new `SafeLLVMPointer` by taking an `LLVMRef` and converting it to a non-null raw pointer.
    /// Returns an `Option` wrapped instance of `SafeLLVMPointer` if the pointer is non-null.
    pub fn new(llvm_ref: LLVMRef) -> Result<Self, PointerError> {
        let raw_ptr = llvm_ref.to_raw();
        NonNull::new(raw_ptr).map(|nn_ptr| SafeLLVMPointer {
            ptr: Arc::new(RwLock::new(nn_ptr)),
        }).ok_or(PointerError::NullPointer)
    }

    /// Provides read-only access to the pointed-to value.
    /// The read operation is safely performed within the bounds of an `RwLock`, ensuring no concurrent write operations.
    /// A closure receives an immutable reference to the value of `LLVMRef`.
    pub fn read<F, R>(&self, kind: LLVMRefType, f: F) -> Result<R, PointerError>
    where
        F: FnOnce(&LLVMRef) -> R,
    {
        let lock = self.ptr.read()?;
        let ref_to_value = unsafe { LLVMRef::from_raw(lock.as_ptr(), kind) };
        Ok(f(&ref_to_value))
    }

    /// Provides write access to the pointed-to value.
    /// The write operation ensures exclusive, mutable access via an `RwLock`.
    /// A closure receives a mutable reference to the value of `LLVMRef`.
    pub fn write<F, R>(&self, kind: LLVMRefType, f: F) -> Result<R, PointerError>
    where
        F: FnOnce(&mut LLVMRef) -> R,
    {
        let mut lock = self.ptr.write()?;
        let mut ref_to_mut_value = unsafe { LLVMRef::from_raw(lock.as_ptr(), kind) };
        Ok(f(&mut ref_to_mut_value))
    }
}

/// Represents types of LLVM references to aid in safe runtime conversion.
#[derive(Debug, Clone, Copy)]
pub enum LLVMRefType {
    Context,
    Module,
    Value,
    BasicBlock,
    Builder,
    Type,
}

// Define an error type for your SafeLLVMPointer operations
#[derive(Debug)]
pub enum PointerError {
    NullPointer,
    LockError(String),
}

impl fmt::Display for PointerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PointerError::NullPointer => write!(f, "Attempted to create a pointer to a null reference"),
            PointerError::LockError(ref err) => write!(f, "Lock error: {}", err),
        }
    }
}

impl<T> From<PoisonError<T>> for PointerError {
    fn from(error: PoisonError<T>) -> Self {
        PointerError::LockError(format!("{}", error))
    }
}