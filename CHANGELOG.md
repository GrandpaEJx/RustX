# Changelog

All notable changes to this project will be documented in this file.

## [0.5.0] - 2025-12-27

### Added
- **Enhanced CLI Interface**
  - Added `-o` flag for build output (shorthand for `rustx build`)
  - Added `--rs` flag for transpile-only mode (generate Rust source without compiling)
  - Smart output naming: auto-infers binary name from input file
  - Example: `rustx script.rsx -o` creates `./script` binary
  - Example: `rustx script.rsx --rs` creates `script.rs`

- **Standalone Binary Support**
  - Rust toolchain detection with helpful error messages
  - Graceful fallback when JIT features require Rust
  - Installation script (`install.sh`) for pre-built binaries
  - Comprehensive `INSTALL.md` guide
  - Optimized release profile (stripped, LTO enabled)
  
- **JIT Compiler Optimization**
  - Type inference for native code generation (~6x faster than Node.js)
  - Single build sandbox for faster recompilation
  - Native `i64` and `bool` code paths for numeric operations
  
- **Benchmark Suite**
  - Language comparison benchmarks (Python, Node.js, RustX)
  - Automated reporting with `compare.sh`
  - Performance metrics: Time, CPU%, RAM

### Changed
- Default mode is now interpreter (no Rust required)
- JIT compilation requires explicit `build` command or `-o` flag
- Improved error messages explaining Interpreter vs JIT modes
- Updated README with standalone binary installation

### Performance
- Loop benchmarks: **13.59ms** (vs Node.js 45ms, Python 240ms)
- Fibonacci(30): **11.77ms** (vs Node.js 73ms, Python 170ms)

## [0.4.0] - 2025-12-14

### Added

- **Documentation**: Comprehensive documentation for standard library modules.
- **Web Module**: Full docs for `web.app()`, `web.json()`, and HTTP routing (`app.get()`, `app.post()`, `app.listen()`).
- **JSON Module**: Documentation for `json.parse()` and `json.stringify()` functions.
- **Time Module**: Documentation for `time.now()` and `time.sleep()` functions.
- **HTTP Module**: Documentation for `http.get()` and `http.post()` client functions.
- **OS Module**: Documentation for `os.env()` and `os.args()` system functions.

### Improved

- **Performance**: Web server benchmarks updated - achieving 67k RPS (100 connections) and 57k RPS (1000 connections).
- **Documentation Structure**: Enhanced built-in functions reference with module-based organization.
- **Examples**: Web server example now demonstrates JSON parsing, routing, and API endpoints.

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
