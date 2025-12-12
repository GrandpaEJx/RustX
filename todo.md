# RustX Language Project TODO & Rules

> [!IMPORTANT] > **Engineering Rules (Highly Refactored & Maintainable)**
>
> 1.  **Modularity**: Functions must be small and focused. No 100-line functions.
> 2.  **Safety**: No `unwrap()` in production code. Use `Result` and `Option` propagation.
> 3.  **Typos**: Zero tolerance for typos in variable names or comments.
> 4.  **Documentation**: All public structs and functions must have doc comments (`///`).
> 5.  **Clean Architecture**: Separation of concerns between Lexer, Parser, and AST.

## Roadmap

### Phase 1: Core Engine (Refactored)

- [/] Initialize Workspace (`rustx-lang` with `core`, `macros`, `cli`)
- [ ] **Lexer**: Implement `Token` enum and tokenizer with `//` comments support.
- [ ] **AST**: Define minimal `Expr` and `Stmt` nodes.
- [ ] **Parser**: Recursive descent parser with error recovery.
- [ ] **Interpreter**: Tree-walk interpreter for expressions.

### Phase 2: Interop & Macros

- [ ] **Macro Logic**: Implement `rx!` parsing using `syn`.
- [ ] **Context Capture**: Mechanism to discover and inject Rust variables.
- [ ] **Type Conversion**: Traits for `Rust -> Value` and `Value -> Rust`.

### Phase 3: Standalone & Imports

- [ ] **CLI**: `rustx` binary implementation.
- [ ] **Import System**: Resolver for `.rsl` files.
- [ ] **Std Lib**: Basic string/math functions.

### Phase 4: Final Polish

- [ ] Comprehensive Testing (Unit + Integration).
- [ ] Verify "Seamless" Experience.
