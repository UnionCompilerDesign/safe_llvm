/*  
    The ExecutionEngine class manages the initialization and operation of an LLVM execution engine, along with its context and module.
    It provides a structured way to manage these components using thread-safe mechanisms with Arc and RwLock.
*/

extern crate llvm_sys as llvm;

use llvm::{core, execution_engine};

use std::{ffi::{c_char, c_void, CStr, CString}, sync::{Arc, RwLock}};

use slog::{info, Logger};

use crate::{
    jit::target::{TargetConfigurator, GeneralTargetConfigurator},
    logger::init::init_logger, 
    memory_management::pointer::{CPointer, LLVMRef, LLVMRefType},
};


/// Represents an LLVM execution engine managed within a multi-threaded environment.
/// This struct encapsulates all necessary LLVM components such as context, module, and execution engine.
pub struct ExecutionEngine {
    context: Arc<RwLock<CPointer>>,
    engine: Arc<RwLock<CPointer>>,
    module: Arc<RwLock<CPointer>>,
    logger: Option<Logger>,
}

impl ExecutionEngine {
    /// Constructs a new ExecutionEngine.
    /// Initializes a new LLVM context and module, and optionally sets up debugging information.
    ///
    /// # Arguments
    /// * `debug_info` - If true, enables logging for this engine.
    pub fn new(debug_info: bool) -> Self {
        GeneralTargetConfigurator.configure();

        let context_ref = create_empty_context().expect("Failed to create context");
        let context_cptr = CPointer::new(context_ref).expect("Context cannot be null");

        let module_cptr = create_empty_module(&context_ref).expect("Failed to create module");

        let mut engine_ref: execution_engine::LLVMExecutionEngineRef = std::ptr::null_mut();
        let mut out_error: *mut c_char = std::ptr::null_mut();
        let engine_ptr = &mut engine_ref;
        
        module_cptr.read(LLVMRefType::Module, |module_ref| {
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
        
        let engine_cptr = CPointer::new(LLVMRef::ExecutionEngine(engine_ref)).expect("Engine cannot be null");

        let logger = if debug_info {
            Some(init_logger())
        } else {
            None
        };

        Self {
            context: Arc::new(RwLock::new(context_cptr)),
            engine: Arc::new(RwLock::new(engine_cptr)),
            module: Arc::new(RwLock::new(module_cptr)),
            logger,
        }
    }

    /// Configures and potentially starts the LLVM execution engine using a specified target configurator.
    ///
    /// # Arguments
    /// * `target_configurator` - The target configurator to setup necessary LLVM targets.
    /// * `start_engine` - Whether to start the engine immediately after configuration.
    ///
    /// # Returns
    /// Returns `Ok(())` on successful configuration and initialization, or `Err(String)` on failure.
    pub fn init_target<T: TargetConfigurator>(&mut self, target_configurator: T, start_engine: bool) -> Result<(), String> {
        target_configurator.configure();
        self.log_info("Target configured.");

        if start_engine {
            self.start_engine()?;
            self.log_info("Engine started.");
        }

        Ok(())
    }

    /// Sets the module for the execution engine.
    ///
    /// # Arguments
    /// * `new_module` - An Arc<RwLock<CPointer>> to the new module.
    pub fn set_module(&mut self, new_module: Arc<RwLock<CPointer>>) {
        self.module = new_module;
        self.log_info("Module set successfully.");
    }

    /// Sets the context for the execution engine.
    ///
    /// # Arguments
    /// * `new_context` - An Arc<RwLock<CPointer>> to the new context.
    pub fn set_context(&mut self, new_context: Arc<RwLock<CPointer>>) {
        self.context = new_context;
        self.log_info("Context set successfully.");
    }

    /// Starts the LLVM execution engine using the stored module and engine references.
    ///
    /// This method initializes the execution engine for the module that has been set in this execution engine instance.
    /// If initialization fails, it logs and returns an error detailing the issue.
    ///
    /// # Returns
    /// Returns `Ok(())` if the execution engine is successfully started, or `Err(String)` with an error message if an error occurs.
    pub fn start_engine(&mut self) -> Result<(), String> {
        // Using the safe `write` and `read` accessors to obtain module and engine pointers
        let mut out_error: *mut c_char = std::ptr::null_mut();

        let engine_result = self.engine.write().unwrap().write(LLVMRefType::ExecutionEngine, |engine_ref| {
            if let LLVMRef::ExecutionEngine(engine_ptr) = engine_ref {
                self.module.read().unwrap().read(LLVMRefType::Module, |module_ref| {
                    if let LLVMRef::Module(module_ptr) = module_ref {
                        // Attempt to create the execution engine for the module
                        unsafe {
                            if execution_engine::LLVMCreateExecutionEngineForModule(engine_ptr as *mut _, *module_ptr as *mut _, &mut out_error) != 0 {
                                let error_str = CStr::from_ptr(out_error).to_str().unwrap_or("Unknown error");
                                core::LLVMDisposeMessage(out_error);
                                self.log_info(&format!("Error starting engine: {}", error_str));
                                Err(error_str.to_owned())
                            } else {
                                self.log_info("Execution engine successfully started.");
                                Ok(())
                            }
                        }
                    } else {
                        Err("Invalid module reference.".to_string())
                    }
                })
            } else {
                Err("Invalid engine reference.".to_string())
            }
        });

        engine_result
    }

    /// Executes a specified function within an already compiled module.
    ///
    /// # Arguments
    /// * `module` - An Arc<RwLock<CPointer>> pointing to the pre-compiled LLVM module.
    /// * `function_name` - The name of the function within the module to call.
    ///
    /// # Returns
    /// Returns `Ok(i64)` with the function's result if successful, or `Err(String)` with an error message if the execution fails.
    pub fn execute(&mut self, function_name: &str) -> Result<c_void, String> {
        let engine_lock = self.engine.read().map_err(|e| format!("Failed to obtain read lock on engine: {}", e))?;

        let result = engine_lock.read(LLVMRefType::ExecutionEngine, |engine_ref| {
            if let LLVMRef::ExecutionEngine(engine_ptr) = engine_ref {
                let module_lock = self.module.read().map_err(|e| format!("Failed to obtain read lock on module: {}", e))?;
                module_lock.read(LLVMRefType::Module, |module_ref| {
                    if let LLVMRef::Module(_module_ptr) = module_ref {
                        let function_name_c = CString::new(function_name).map_err(|_| "Failed to create CString for function name.")?;
                        let function_address = unsafe { execution_engine::LLVMGetFunctionAddress(*engine_ptr, function_name_c.as_ptr()) };
                        if function_address == 0 {
                            return Err("Function not found in the module.".to_string());
                        }

                        // Cast the function pointer to a callable function type
                        let entry_main: extern "C" fn() -> c_void = unsafe { std::mem::transmute(function_address) };
                        Ok(entry_main())  // Execute the function
                    } else {
                        Err("Invalid module pointer.".to_string())
                    }
                })
            } else {
                Err("Invalid engine pointer.".to_string())
            }
        });

        match result {
            Ok(function_result) => {
                self.log_info(&format!("Function executed successfully."));
                Ok(function_result)
            },
            Err(e) => {
                self.log_info(&format!("Execution error: {}", e));
                Err(e)
            }
        }
    }

    /// Logs a message if the logger is available.
    ///
    /// # Arguments
    /// * `msg` - The message to log.
    fn log_info(&self, msg: &str) {
        if let Some(ref log) = self.logger {
            info!(log, "{}", msg);
        }
    }
}

/// Helper function to create an empty LLVM context.
///
/// # Returns
/// Returns an `Option<LLVMRef>` pointing to a new LLVM context.
fn create_empty_context() -> Option<LLVMRef> {
    unsafe { Some(LLVMRef::Context(core::LLVMContextCreate())) }
}

/// Helper function to create an empty LLVM module within a given context.
///
/// # Arguments
/// * `context_ref` - A reference to the LLVM context.
///
/// # Returns
/// Returns an `Option<CPointer>` pointing to a new LLVM module.
fn create_empty_module(context_ref: &LLVMRef) -> Option<CPointer> {
    if let LLVMRef::Context(context_ptr) = context_ref {
        let module_name = CString::new("temp").unwrap();
        unsafe {
            let module_ref = LLVMRef::Module(core::LLVMModuleCreateWithNameInContext(
                module_name.as_ptr(),
                *context_ptr,
            ));
            CPointer::new(module_ref)
        }
    } else {
        None
    }
}
