# RustX Language Project TODO & Rules

> [!IMPORTANT] > **Engineering Rules (Highly Refactored & Maintainable)**
>
> 1.  **Modularity**: Functions must be small and focused. No 100-line functions.
> 2.  **Safety**: No `unwrap()` in production code. Use `Result` and `Option` propagation.
> 3.  **Typos**: Zero tolerance for typos in variable names or comments.
> 4.  **Documentation**: All public structs and functions must have doc comments (`///`).
> 5.  **Clean Architecture**: Separation of concerns between Lexer, Parser, and AST.

## Roadmap

### Phase 1: Core Engine (Refactored) ✅

- [x] Initialize Workspace (`rustx-lang` with `core`, `macros`, `cli`)
- [x] **Lexer**: Implement `Token` enum and tokenizer with `//` comments support.
- [x] **AST**: Define minimal `Expr` and `Stmt` nodes.
- [x] **Parser**: Recursive descent parser with error recovery.
- [x] **Interpreter**: Tree-walk interpreter for expressions.
- [x] **Refactoring**: Split into modular files (ast/, interpreter/)
- [x] **Bug Fixes**: Fixed loop scoping issues

### Phase 2: Interop & Macros ✅

- [x] **Macro Logic**: Implement `rx!` parsing using `syn`.
- [x] **Type Conversion**: Traits for `Rust -> Value` and `Value -> Rust`.
- [x] **Testing**: Integration tests for macros (6/6 passing)
- [ ] **Context Capture**: Mechanism to discover and inject Rust variables.

### Phase 3: Standalone & Imports 🔄

- [x] **CLI**: `rustx` binary implementation with REPL.
- [x] **Enhanced REPL**: Command history, colored output, special commands
- [x] **Debug Flags**: --ast, --tokens, --time, --verbose
- [ ] **Import System**: Resolver for `.rsl` files.
- [ ] **Std Lib**: Basic string/math functions.

### Phase 4: Final Polish 🔄

- [x] Comprehensive Testing (Unit + Integration) - 11/11 tests passing
- [x] Documentation - Beginner-friendly README
- [x] Rust-style docs with `cargo doc`
- [ ] Verify "Seamless" Experience with advanced macro features
- [ ] Performance benchmarks

## Recent Accomplishments ✅

### Loop Bug Fix

- Fixed infinite loop issue caused by variable scoping
- Added `Environment::update()` method
- All loop types now work correctly (while, for, range)

### CLI Enhancements

- Added colored output (rustyline, colored crates)
- Enhanced REPL with command history
- Special commands (:help, :exit, :clear, :vars)
- Debug flags (--ast, --tokens, --time, --verbose)

### Macro Implementation

- Implemented `rx!` and `rsx!` macros
- Type conversion for i64, f64, String, bool
- 6/6 integration tests passing

### Codebase Refactoring

- Split AST module into 4 files (expr, stmt, ops, mod)
- Split Interpreter module into 6 files (environment, eval_stmt, eval_expr, eval_ops, builtins, mod)
- Reduced average file size by 73% (300 → 80 lines)
- Deleted 2,670 lines of old code
- Added 812 lines of modular code
- 13 commits with clear one-line messages

### Documentation

- Created beginner-friendly README with examples
- Generated full Rust-style documentation
- Added troubleshooting guide and FAQ

## Next Steps

### High Priority (Do Now)

- [/] Complete parser module refactoring (1/5 files done)
- [ ] Complete lexer module refactoring
- [x] Add more built-in functions (len, push, pop, type) ✅
- [ ] Add string manipulation functions (split, join, trim, upper, lower)
- [ ] Add math functions (abs, min, max, floor, ceil, round)
- [ ] Add array functions (map, filter, reduce, reverse, sort)
- [ ] Run all examples and verify output
- [ ] Update FAQ in README with common questions

### Medium Priority

- [ ] Import system for `.rsl` files
- [ ] Standard library (string, math, array functions)
- [ ] Context capture for macros (access Rust variables)
- [ ] More comprehensive examples (file I/O, JSON parsing)
- [ ] Add error line numbers in error messages
- [ ] Performance benchmarks

### Low Priority

- [ ] Performance optimizations
- [ ] REPL improvements (syntax highlighting, autocomplete)
- [ ] VSCode extension for syntax highlighting
- [ ] Online playground
- [ ] Package manager for RustX libraries
