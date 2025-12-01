# Contributing to RustX

Thank you for your interest in contributing to RustX! This document provides guidelines for contributing to the project.

## Development Setup

1. Clone the repository
2. Install Rust (latest stable version)
3. Build the project: `cargo build`
4. Run tests: `cargo test`
5. Run examples: `cargo run demo/main.rsx`

## Project Structure

The codebase is organized into focused modules:

- **`error.rs`** - Error handling
- **`lexer.rs`** - Tokenization  
- **`parser.rs`** - Parsing and AST building
- **`ast.rs`** - AST node definitions
- **`runtime.rs`** - Runtime values and environment
- **`interpreter.rs`** - Expression evaluation and execution
- **`builtins.rs`** - Built-in functions
- **`lib.rs`** - Main library interface
- **`main.rs`** - CLI entry point

## How to Contribute

### Adding New Features

1. **Lexer**: Add new token types in `lexer.rs`
2. **AST**: Define new node types in `ast.rs`
3. **Parser**: Implement parsing logic in `parser.rs`
4. **Interpreter**: Add evaluation logic in `interpreter.rs`
5. **Built-ins**: Add new functions in `builtins.rs`

### Bug Fixes

1. Create an issue describing the bug
2. Write a test case that reproduces the bug
3. Fix the bug following the existing code patterns
4. Ensure all tests pass

### Code Style

- Follow Rust standard formatting: `cargo fmt`
- Use meaningful variable and function names
- Add documentation for public APIs
- Keep functions small and focused
- Add error handling where appropriate

### Testing

- Add unit tests for new features
- Include integration tests in the `demo/` directory
- Test edge cases and error conditions

## Getting Help

- Create an issue for questions
- Check existing issues and discussions
- Review the codebase for patterns

## Pull Request Process

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Ensure `cargo fmt` is run
6. Submit a pull request with a clear description

Thank you for contributing to RustX!