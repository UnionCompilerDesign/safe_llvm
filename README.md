# Safe LLVM

## Contributors
Caleb L'Italien, John Daly

## Overview
SafeLLVM is a Rust-based library designed to provide a safe and user-friendly interface to llvm-sys. It is developed specifically for the Simple Instructional C99 Compiler (SICC). The library aims to encapsulate the complexity of LLVM’s API, making it accessible for undergraduate work.

## Project Structure
- `analysis/`: Tools for performing various analyses on LLVM IR.
- `common/`: Common utilities and helper functions shared across the project.
- `ir/`: Manages the creation of LLVM Intermediate Representation (IR).
- `jit/`: Implements an ExecutionEngine for execution of pre-compiled LLVM modules.
- `logging/`: Handles logging functionalities for debugging.

## Getting Started

### Usage
To use SafeLLVM in your Rust projects, you can include it directly via Cargo by adding it as a dependency in your `Cargo.toml` file using its GitHub repository URL. Here's how to do it:

1. **Add SafeLLVM as a dependency in your `Cargo.toml`:**
    ```toml
    [dependencies]
    safe_llvm = { git = "git@github.com:UnionCompilerDesign/safe_llvm.git", branch = "main" }
    ```

2. **Refer to specific `README`s in each directory:**
For more specific usage details, please refer to the READMEs located in each directory. These documents provide further instructions and examples on how to use the components within SafeLLVM.

### How to Contribute
Contributions are welcome! Please refer to the `CONTRIBUTING` file for guidelines on how to contribute.

### License
Distributed under the MIT License. See the `LICENSE` file for more details.

### Acknowledgements
Special thanks to Professor Aaron Cass of Union College for his guidance and expertise throughout the development of this project.

### Contact
For any inquiries, contact Caleb L'Italien at litaliencaleb@gmail.com.
