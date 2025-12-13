# Changelog

All notable changes to this project will be documented in this file.

## [0.3.0] - 2025-12-13

### Added

- **Crate Imports**: Support for importing external Rust crates using `use crate "name" = "version"`.
- **Embedded Rust**: Support for including raw Rust code blocks using `rust { ... }`.
- **JIT Compilation**: The `run` command now automatically compiles scripts that use native features or dependencies, managing a temporary Cargo project under the hood.
- **Actix Web Support**: Capability to run web servers (like Actix-web) directly from RustX scripts.
- **Method Chaining**: Support for chaining methods on values (e.g., `"hello".upper().trim()`).
- **New Methods**: Added `upper()`, `lower()`, `trim()`, `abs()`, `floor()`, `ceil()`, `push()`, `pop()`, `split()`.

### Improved

- **CLI**: Enhanced `run` command to detect JIT requirements and handle compilation seamlessly.
- **Error Handling**: Better error messages for runtime and compilation errors.
- **Lexer/Parser**: Expanded support for Rust tokens (attributes `#`, `?` operator, `->`, `::`).

## [0.2.0] - 2025-12-12

- Initial Release with Interpreter, REPL, and basic syntax.
