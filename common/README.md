# Common Utilities for SafeLLVM

# Overview
The `common` module provides essential functionalities that support the broader `SafeLLVM` project. It includes handling I/O operations, managing C strings, ensuring safe pointer interactions, configuring target machines, and defining constants used across the project.

## Features
- **I/O Operations:** Supports serialization and writing of LLVM modules to files.
- **CString Utilities:** Supports converting paths to instances of `CString`.
- **Pointer Safety:** Offers abstractions to manage raw LLVM pointers safely.
- **Target Configuration:** Assists in setting up and managing LLVM target configurations.
- **Constants:** Centralizes constants that are reused throughout the SafeLLVM project.

## Usage
Here's how to use the modules within the `common` module:

### I/O Operations
```rust
    use safe_llvm::common::io;

    let module_pointer = ...; // Get a `SafeLLVMPointer` of type module
    io::write_ir_to_file(module_pointer, "output.ll").expect("Error writing to file");
```

### CString Utilities
```rust
    use safe_llvm::common::cstring;

    let path = Path::new("/path/to/file");
    let c_string = cstring::convert_path_to_cstring(path).expect("Failed to convert path");
```

### SafeLLVMPointer Operations
The `SafeLLVMPointer` struct provides safe, synchronized access to raw LLVM pointers across multiple threads, using `Arc` and `RwLock`.

#### Creating a `SafeLLVMPointer`
To encapsulate a raw LLVM reference within a `SafeLLVMPointer`:
```rust
    use safe_llvm::common::pointer::{SafeLLVMPointer, LLVMRef};

    let llvm_ref = ...; // Obtain an LLVMRef, for example, LLVMRef::Module(module_ptr)
    let safe_pointer = SafeLLVMPointer::new(llvm_ref).expect("Pointer must not be null");

    println!("A SafeLLVMPointer has been created successfully.");
```

#### Reading a `SafeLLVMPointer`
To safely read and access the value from a SafeLLVMPointer:
```rust
    let result = safe_pointer.read(LLVMRefType::Module, |module_ref| {
        // Access or use the module_ref safely here
        println!("Successfully accessed the module reference.");
    });
    
```

#### Writing to a `SafeLLVMPointer`
To safely write or modify the value managed by a SafeLLVMPointer:
```rust
    safe_pointer.write(LLVMRefType::Module, |module_ref| {
        // Modify the module_ref safely here
        println!("Successfully modified the module reference.");
    });
        
```

### Target Configuration
```rust
    use safe_llvm::common::target::GeneralTargetConfigurator;

    let configurator = GeneralTargetConfigurator;
    configurator.configure();
```

### Constants
```rust
    use safe_llvm::common::constants::{DEFAULT_FUNCTION_NAME, DEFAULT_MODULE_NAME};

    println!("Default function name: {}", DEFAULT_FUNCTION_NAME);
    println!("Default module name: {}", DEFAULT_MODULE_NAME);
```

## FAQ 

## Further Information
For further information or questions regarding the use of `common` in SafeLLVM, feel free to contact the main contributors or raise an issue on the GitHub repository.