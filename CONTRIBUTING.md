# Contributing to SafeLLVM

Welcome to the SafeLLVM project! We appreciate your interest in contributing to SafeLLVM, a wrapper around **[llvm-sys](https://github.com/tari/llvm-sys.rs)** designed to serve as the backend for the Simple Instructional C99 Compiler (SICC).

## Disclaimer

SafeLLVM is designed for educational purposes and is not intended for production use. If you are looking for a production-level LLVM crate for Rust projects, consider using the following crates:

- **[Inkwell](https://github.com/TheDan64/inkwell)**
- **[llvm-ir](https://github.com/cdisselkoen/llvm-ir)**

## Getting Started

Before contributing, please make sure to familiarize yourself with the project structure and the goals of both SafeLLVM and SICC. This understanding will help you make meaningful contributions that align with the project's objectives.

### Setting Up Your Development Environment

1. **Install the prerequisites found in README.md**

2. **Fork and clone the repository:**
    Start by cloning the SafeLLVM repository to your local machine.
    ```
    git clone git@github.com:UnionCompilerDesign/safe_llvm.git
    ```

3. **Build and test the project:**
    ```
    cd safe_llvm
    cargo build --all
    cargo test --all
    ```

## Making Contributions

### Reporting Bugs

If you find a bug, please report it by opening a new issue on GitHub. Include as much detail as possible and please use the template found in `safe_llvm/.github/ISSUE_TEMPLATE`.

### Suggesting Enhancements

We welcome suggestions for improvements! If you have an idea to enhance SafeLLVM, please create an issue to discuss so we can discuss it.

### Pull Requests

We actively welcome your pull requests:

1. Fork the repo and create your branch from `main`.
2. If you've added code that should be tested, add tests.
3. Ensure your code lints (use `cargo clippy`).
4. Issue the pull request!

### Coding Guidelines

- Follow the [Rust style guide](https://doc.rust-lang.org/1.0.0/style/README.html).
- Write clean, readable code and include comments where necessary.
- Ensure that all tests pass before you make a pull request.

## Recognition

Contributors who make significant improvements will be recognized as "Contributors" in the project README.

Thank you for your interest in contributing to SafeLLVM! We look forward to your contributions and are excited to see how the project grows and evolves with your help.