# Safe LLVM

## Contributors
Caleb L'Italien, John Daly

## Overview
SafeLLVM is a Rust-based library designed to provide a safe and user-friendly interface to the LLVM backend. It acts as the underlying architecture for the Simple Instructional C99 Compiler (SICC), which is developed as part of a computer science curriculum at Union College. The library aims to abstract and encapsulate the complexity of LLVM’s API, making it accessible and manageable for educational purposes.

## Project Structure
- `analysis/`: Tools for performing various analyses on LLVM IR, aiding in optimization and correctness assessments.
- `common/`: Common utilities and helper functions shared across the project, including error handling and configuration settings.
- `ir/`: Manages the representation and manipulation of LLVM Intermediate Representation (IR), crucial for code generation and optimization.
- `jit/`: Implements JIT compilation capabilities, allowing for dynamic compilation and execution of LLVM IR.
- `logging/`: Handles logging functionalities to assist in debugging and tracking the operations of the compiler interactions with LLVM.


## Getting Started

### Usage
To use SafeLLVM in your Rust projects, you can include it directly via Cargo by adding it as a dependency in your `Cargo.toml` file using its GitHub repository URL. This is particularly useful if you want to leverage the most recent updates from a specific branch, tag, or commit.

Here's how to set it up:

1. **Add SafeLLVM as a dependency in your `Cargo.toml`:**
    ```toml
    [dependencies]
    safe_llvm = { git = "git@github.com:UnionCompilerDesign/safe_llvm.git", branch = "main" }
    ```

2. **Refer to specific READMEs in each directory:**
For more specific usage details, please refer to the READMEs located in each directory. These documents provide further instructions and examples on how to use the components within SafeLLVM.

### How to Contribute
Contributions are welcome! Please refer to the `CONTRIBUTING` file for guidelines on how to contribute.

### License
Distributed under the MIT License. See the `LICENSE` file for more details.

### Acknowledgements
Special thanks to Professor Aaron Cass of Union College for his guidance and expertise throughout the development of this project.

### Contact
For any inquiries, contact Caleb L'Italien at litaliencaleb@gmail.com.

