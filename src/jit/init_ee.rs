extern crate llvm_sys as llvm;

use std::ffi::{CString, CStr};

use llvm::{
    execution_engine::{LLVMLinkInMCJIT, LLVMCreateExecutionEngineForModule, LLVMOpaqueExecutionEngine}, 
    target::{
        LLVM_InitializeAllTargetInfos,
        LLVM_InitializeAllTargets,
        LLVM_InitializeAllTargetMCs,
        LLVM_InitializeAllAsmParsers,
        LLVM_InitializeAllAsmPrinters,
        LLVM_InitializeNativeTarget,
        LLVM_InitializeNativeAsmParser,
        LLVM_InitializeNativeAsmPrinter,
    }, core::{LLVMModuleCreateWithNameInContext, LLVMDisposeMessage},
};

/// Initialize targets of JIT
pub fn init_ee_targets() {
    unsafe {
        LLVM_InitializeAllTargetInfos();
        LLVM_InitializeAllTargets();
        LLVM_InitializeAllTargetMCs();
        LLVM_InitializeAllAsmParsers();
        LLVM_InitializeAllAsmPrinters();
        LLVM_InitializeNativeTarget();
        LLVM_InitializeNativeAsmParser();
        LLVM_InitializeNativeAsmPrinter();
        LLVMLinkInMCJIT();
    };
}

/// Initializes execution engine
pub fn init_engine(context: *mut llvm::LLVMContext, mut engine: *mut LLVMOpaqueExecutionEngine)
        -> (*mut llvm::LLVMContext, *mut LLVMOpaqueExecutionEngine) {
    unsafe {
        let mut tmp_module: *mut llvm::LLVMModule = std::ptr::null_mut(); // ignore this warning
        tmp_module = LLVMModuleCreateWithNameInContext(CString::new("temp").unwrap().as_ptr(), context); // and this one
        if tmp_module.is_null() {
            panic!("Failed to create temporary module");
        }

        let mut out_error: *mut i8 = std::ptr::null_mut();
        let result: i32 = LLVMCreateExecutionEngineForModule(&mut engine, tmp_module, &mut out_error);
        if result != 0 {
            let error_str: String = CStr::from_ptr(out_error).to_str().unwrap_or("Unknown error").to_owned();
            LLVMDisposeMessage(out_error);
            panic!("{}", error_str);
        }
        (context, engine)
    }
}