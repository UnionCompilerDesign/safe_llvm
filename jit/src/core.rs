//! The ExecutionEngine class manages the initialization and operation of an LLVM execution engine, along with its context and module.

extern crate llvm_sys as llvm;
use analysis::validator::Validator;
use llvm::{core, execution_engine};
use llvm_sys::target;
use std::{any::Any, ffi::{CStr, CString}, sync::{Arc, RwLock}};
use slog::Logger;
use common::{pointer::{LLVMRef, LLVMRefType, SafeLLVMPointer}, target::{GeneralTargetConfigurator, TargetConfigurator}};

/// Represents an LLVM execution engine for a multi-threaded environment.
/// This struct encapsulates all necessary LLVM components: context, module, and execution engine.
pub struct ExecutionEngine {
    engine: Arc<RwLock<SafeLLVMPointer>>,
    logger: Option<Logger>,
}

impl ExecutionEngine {
    /// Constructs a new `ExecutionEngine`.
    ///
    /// This method initializes a new LLVM ExecutionEngine, configures the general target,
    /// and optionally sets up a logger for debugging information based on the `debug_info` parameter.
    ///
    /// # Parameters
    /// * `module` - A thread-safe `SafeLLVMPointer` containing an `LLVMModuleRef`.
    /// * `debug_info` - If true, initializes a logger to record debugging information.
    ///
    /// # Returns
    /// A new instance of `ExecutionEngine`.
    pub fn new(module: Arc<RwLock<SafeLLVMPointer>>, debug_info: bool) -> Self {
        // GeneralTargetConfigurator.configure();
        // unsafe { execution_engine::LLVMLinkInMCJIT(); }
        // check mcjit versus normal
        // check memory manager

        let logger = if debug_info {
            Some(logging::core::init_logger())
        } else {
            None
        };

        logging::core::log_info(&logging::core::init_logger(), "made it1");

        unsafe {
            target::LLVM_InitializeAllTargetInfos();
            target::LLVM_InitializeAllTargets();
            target::LLVM_InitializeAllTargetMCs();
            target::LLVM_InitializeAllAsmParsers();
            target::LLVM_InitializeAllAsmPrinters();
            target::LLVM_InitializeNativeTarget();
            target::LLVM_InitializeNativeAsmParser();
            target::LLVM_InitializeNativeAsmPrinter();
            execution_engine::LLVMLinkInMCJIT();
        }
        logging::core::log_info(&logging::core::init_logger(), "made it2");

        let validator = Validator::new(module.clone());
        if !validator.is_valid_module() {
            panic!("failed to validate")
        }
        
        let mut engine_ref = std::ptr::null_mut();
        let mut out_error = std::ptr::null_mut();

        let module_rw_lock = module.try_read().expect("Failed to read module");
        module_rw_lock.read(LLVMRefType::Module, |module_ref| {
            if let LLVMRef::Module(module_ptr) = module_ref {
                unsafe {
                    logging::core::log_info(&logging::core::init_logger(), "made it3");
                    if execution_engine::LLVMCreateExecutionEngineForModule(&mut engine_ref, *module_ptr, &mut out_error) != 0 {
                        logging::core::log_info(&logging::core::init_logger(), "made it4");
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

        let engine_cptr = SafeLLVMPointer::new(LLVMRef::ExecutionEngine(engine_ref), LLVMRefType::ExecutionEngine).expect("Engine cannot be null");

        Self {
            engine: Arc::new(RwLock::new(engine_cptr)),
            logger,
        }
    }

    /// Configures the LLVM execution engine using a specified target configurator.
    ///
    /// # Parameters
    /// * `target_configurator` - The target configurator to setup necessary LLVM targets.
    ///
    /// # Returns
    /// Returns `Ok(())` on successful configuration and initialization, or `Err(String)` on failure,
    /// including detailed error messages.
    pub fn initialize_target<T: TargetConfigurator>(&mut self, target_configurator: T) -> Result<(), String> {
        target_configurator.configure();
        if let Some(logger) = &self.logger {
            logging::core::log_info(&logger, "Target configured.");
        }
        Ok(())
    }

    /// Executes a specified function within the module.
    ///
    /// # Parameters
    /// * `function_name` - The name of the function to be executed.
    ///
    /// # Returns
    /// Returns `Ok(())` if the function is executed successfully, or `Err(String)` if an error occurs,
    /// which could include the function not being found or an execution error.
    /// Executes a specified function.
    pub fn execute<ReturnType, ArgType>(&mut self, function_name: &str, args: ArgType) -> Result<ReturnType, String>
    where
        ReturnType: 'static, 
        ArgType: Any + Send + Sync, 
    {
        let engine_lock = self.engine.try_read().map_err(|e| format!("Failed to obtain read lock on engine: {}", e))?;
        let result = engine_lock.read(LLVMRefType::ExecutionEngine, |engine_ref| {
            if let LLVMRef::ExecutionEngine(engine_ptr) = engine_ref {
                let function_name_c = CString::new(function_name).map_err(|_| "Failed to create CString for function name.")?;
                let function_address = unsafe { execution_engine::LLVMGetFunctionAddress(*engine_ptr, function_name_c.as_ptr()) };
                
                if function_address == 0 {
                    if let Some(logger) = &self.logger {
                        logging::core::log_warning(&logger, &format!("Function \"{}\" not found.", function_name));
                    }
                    return Err("Function not found in given module.".to_string());
                }

                unsafe {
                    let func: extern "C" fn(ArgType) -> ReturnType = std::mem::transmute(function_address);
                    Ok(func(args))
                }
            } else {
                Err("Invalid engine pointer.".to_string())
            }
        });

        match result {
            Ok(value) => {
                if let Some(logger) = &self.logger {
                    logging::core::log_info(&logger, &format!("Function '{}' executed successfully.", function_name));
                }
                Ok(value)
            },
            Err(e) => {
                if let Some(logger) = &self.logger {
                    logging::core::log_error(&logger, &format!("Execution error: {}", e));
                }
                Err(e)
            }
        }
    }
}