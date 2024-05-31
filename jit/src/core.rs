/// The ExecutionEngine class manages the initialization and operation of an LLVM execution engine, along with its context and module.

extern crate llvm_sys as llvm;

use llvm::{core, execution_engine};

use std::{ffi::{c_char, CStr, CString}, sync::{Arc, RwLock}};

use slog::Logger;

use common::{pointer::{LLVMRef, LLVMRefType, SafeLLVMPointer}, target::{GeneralTargetConfigurator, TargetConfigurator}};


/// Represents an LLVM execution engine for a multi-threaded environment.
/// This struct encapsulates all necessary LLVM components: context, module, and execution engine.
pub struct ExecutionEngine {
    engine: Arc<RwLock<SafeLLVMPointer>>,
    logger: Option<Logger>,
}

impl ExecutionEngine {
    /// Constructs a new ExecutionEngine.
    /// Initializes a new LLVM context and module, and optionally sets up debugging information.
    ///
    /// # Arguments
    /// * 'module` - A thread safe `SafeLLVMPointer` containing an LLVMModuleRef
    /// * `debug_info` - If true, enables logging for this engine.
    pub fn new(module: Arc<RwLock<SafeLLVMPointer>>, debug_info: bool) -> Self {
        GeneralTargetConfigurator.configure();

        let mut engine_ref: execution_engine::LLVMExecutionEngineRef = std::ptr::null_mut();
        let mut out_error: *mut c_char = std::ptr::null_mut();
        let engine_ptr = &mut engine_ref;

        module.read().unwrap().read(LLVMRefType::Module, |module_ref| {
            if let LLVMRef::Module(module_ptr) = module_ref {
                unsafe {
                    if execution_engine::LLVMCreateExecutionEngineForModule(engine_ptr, *module_ptr, &mut out_error) != 0 {
                        if !out_error.is_null() {
                            let error_str = CStr::from_ptr(out_error).to_str().unwrap_or("Unknown error");
                            eprintln!("{}", error_str);
                            core::LLVMDisposeMessage(out_error);
                            panic!("Failed to create execution engine");
                        } else {
                            panic!("Failed to create execution engine with unknown error.");
                        }
                    }
                }
            } else {
                panic!("Module pointer is not correctly retrieved.");
            }
        });

        let engine_cptr = SafeLLVMPointer::new(LLVMRef::ExecutionEngine(engine_ref)).expect("Engine cannot be null");

        let logger = if debug_info {
            Some(logging::core::init_logger())
        } else {
            None
        };

        Self {
            engine: Arc::new(RwLock::new(engine_cptr)),
            logger,
        }
    }

    /// Configures the LLVM execution engine using a specified target configurator.
    ///
    /// # Arguments
    /// * `target_configurator` - The target configurator to setup necessary LLVM targets.
    ///
    /// # Returns
    /// Returns `Ok(())` on successful configuration and initialization, or `Err(String)` on failure.
    pub fn init_target<T: TargetConfigurator>(&mut self, target_configurator: T) -> Result<(), String> {
        target_configurator.configure();
        self.log_info("Target configured.");
        Ok(())
    }

    /// Executes a specified function.
    pub fn execute(&mut self, function_name: &str) -> Result<(), String> {
        let engine_lock = self.engine.read().map_err(|e| format!("Failed to obtain read lock on engine: {}", e))?;

        let result = engine_lock.read(LLVMRefType::ExecutionEngine, |engine_ref| {
            if let LLVMRef::ExecutionEngine(engine_ptr) = engine_ref {                  
                let function_name_c = CString::new(function_name).map_err(|_| "Failed to create CString for function name.")?;
                let function_address = unsafe { execution_engine::LLVMGetFunctionAddress(*engine_ptr, function_name_c.as_ptr()) };
                
                if function_address == 0 {
                    self.log_warning(&format!("Function \"{}\" not found.", function_name));
                    return Err("Function not found in given module.".to_string());
                }

                let main: extern "C" fn() -> () = unsafe { std::mem::transmute(function_address) }; 
                main(); 

                Ok(())
            } else {
                Err("Invalid engine pointer.".to_string())
            }
        });
    
        match result {
            Ok(_) => {
                self.log_info(&format!("Function '{}' executed successfully.", function_name));
                Ok(())
            },
            Err(e) => {
                self.log_error(&format!("Execution error: {}", e));
                Err(e)
            }
        }
    }
    
    fn log_info(&self, msg: &str) {
        if let Some(log) = &self.logger {
            logging::core::log_info(log, msg)
        }
    }

    fn log_warning(&self, msg: &str) {
        if let Some(log) = &self.logger {
            logging::core::log_warning(log, msg)
        }
    }

    fn log_error(&self, msg: &str) {
        if let Some(log) = &self.logger {
            logging::core::log_error(log, msg)
        }
    }
}