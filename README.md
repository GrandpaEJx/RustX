# RustX - A Minimal Scripting Language

A lightweight, easy-to-use scripting language built in Rust with a clean, modular architecture.

## Features

- **Simple Syntax**: Clean, readable code with optional semicolons
- **Type System**: Str, Int, Bool, Float with automatic conversion
- **Variable Declarations**: `Str name = "Rust X"`, `Int a = 10`
- **String Interpolation**: `printf("Hello {name}")`
- **Arithmetic Operations**: `+`, `-`, `*`, `/` with proper precedence
- **Built-in Functions**: `print()`, `println()`, `printf()`
- **REPL**: Interactive read-eval-print loop

## Quick Start

### Run a Script
```bash
cargo run demo/main.rsx
```

### Start REPL
```bash
cargo run
```

### Build
```bash
cargo build
```

## Examples

### Basic Variables
```
Str name = "Rust X"
Int a = 10
Bool isTrue = true
Float b = 10.5
```

### String Interpolation
```
printf("Hello {name}")
```

### Arithmetic
```
Int x = 10
Int y = 5
print(x + y)  // Output: 15
print(x / y)  // Output: 2
```

## Architecture

The project is structured with separate, focused modules:

- **`error.rs`** - Error handling and custom result types
- **`lexer.rs`** - Tokenizer for converting source code into tokens
- **`parser.rs`** - Recursive descent parser building AST
- **`ast.rs`** - Abstract Syntax Tree node definitions
- **`runtime.rs`** - Value types and environment management
- **`interpreter.rs`** - Execution engine with expression evaluation
- **`builtins.rs`** - Built-in functions library
- **`lib.rs`** - Main library interface and REPL
- **`main.rs`** - CLI entry point

## Contributing

The modular architecture makes it easy to extend:
- Add new tokens in `lexer.rs`
- Define new AST nodes in `ast.rs`
- Implement new language features in `parser.rs` and `interpreter.rs`
- Add built-in functions in `builtins.rs`

## License

MIT License