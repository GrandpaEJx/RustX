# Codebase Structure

This document outlines the directory structure and module organization of the RustX project.

## Directory Layout

```
RustX/
├── src/                  # Source code
│   ├── ast/              # Abstract Syntax Tree definitions
│   ├── builtins/         # Built-in function implementations
│   ├── error/            # Error handling types and traits
│   ├── interpreter/      # AST interpreter and execution logic
│   ├── lexer/            # Tokenizer (Source -> Tokens)
│   ├── parser/           # Parser (Tokens -> AST)
│   ├── runtime/          # Runtime values and environment
│   ├── lib.rs            # Library entry point
│   └── main.rs           # CLI entry point
├── test/                 # Test scripts (*.rsx)
├── demo/                 # Demo scripts
├── docs/                 # Documentation
└── Cargo.toml            # Project configuration
```

## Modules

- **`ast`**: Defines the data structures representing the parsed code (Expressions, Statements).
- **`builtins`**: Contains standard library functions available in the language (e.g., `print`, `printf`).
- **`error`**: Centralized error handling logic.
- **`interpreter`**: The core engine that traverses the AST and executes instructions.
- **`lexer`**: Handles the first stage of compilation, converting raw text into a stream of tokens.
- **`parser`**: Handles the second stage, organizing tokens into a structured AST.
- **`runtime`**: Manages the program state, including variable scopes (Environment) and value types.
