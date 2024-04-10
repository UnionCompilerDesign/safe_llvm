use std::ffi::CString;

use llvm::{
    core::LLVMCreateMemoryBufferWithMemoryRange, 
    prelude::LLVMBool, 
    ir_reader::LLVMParseIRInContext, 
    execution_engine::{
        LLVMCreateExecutionEngineForModule, 
        LLVMGetFunctionAddress, LLVMOpaqueExecutionEngine
    }
};

/// need to modularize to be used for any file
pub fn parse_ir(ir: CString, buffer_name: CString,
                mut engine: *mut LLVMOpaqueExecutionEngine, mut module: *mut llvm::LLVMModule , args: &[i64],
                function_name: &str, context: *mut llvm::LLVMContext, 
        ) -> i64 {
    let memory_buffer: *mut llvm::LLVMMemoryBuffer = 
        unsafe {
            LLVMCreateMemoryBufferWithMemoryRange(
            ir.as_ptr(),
            ir.as_bytes().len(),
            buffer_name.as_ptr(),
            0 as LLVMBool,
        )};
        
    unsafe { 
        LLVMParseIRInContext(context, memory_buffer, &mut module, std::ptr::null_mut()) 
    };
    
    if module.is_null() {
        panic!("Module is null");
    }
    if engine.is_null() {
        panic!("Engine is null");
    }
    
    // create the execution engine
    if 
        unsafe { 
            LLVMCreateExecutionEngineForModule(&mut engine, module, std::ptr::null_mut()) 
        } != 0 {
            panic!("Failed to create execution engine");
    }

    // lookup the function
    let function_name_c: CString = CString::new(function_name).unwrap();
    let function_address: u64 = unsafe { 
        LLVMGetFunctionAddress(engine, function_name_c.as_ptr()) 
    };
    if function_address == 0 {
        panic!("Function not found");
    }

    // define the function type
    let add_function: extern "C" fn(i64, i64) -> i64 = unsafe { 
        std::mem::transmute(function_address) 
    };

    // execute the function
    add_function(args[0], args[1])
}