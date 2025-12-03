# RustX-Lang - A Minimal Scripting Language  [BUILD WITH AI]

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
cargo run --bin rustx-lang demo/main.rsx
```

### Start REPL
```bash
cargo run --bin rustx-lang
```

### Build
```bash
cargo build
```

### Convert Rust to RSX
```bash
cargo run --bin rustx-lang -r main.rs
```

### Convert RSX to Rust
```bash
cargo run --bin rustx-lang -s demo/main.rsx
```

### Compile to Binary
```bash
cargo run --bin rustx-lang -o demo/main.rsx
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

## Embedding in Rust Projects

RustX can be used as a crate to embed scripting capabilities in your Rust applications.

### Basic Embedding
```rust
use rustx_lang::run_code;

fn main() {
    run_code(r#"
        Int x = 42
        println(x)
    "#).unwrap();
}
```

### Calling Rust Functions from RSX
```rust
use rustx_lang::{Interpreter, Value};

let mut interpreter = Interpreter::new();
interpreter.register_function("double", |args| {
    if let Some(Value::Integer(n)) = args.first() {
        Ok(Value::Integer(n * 2))
    } else {
        Err(rustx_lang::Error::RuntimeError("Expected integer".to_string()))
    }
});

// Now you can call double() from RSX code
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
- **`transpiler.rs`** - Rust to RSX transpiler
- **`lib.rs`** - Main library interface and REPL
- **`main.rs`** - CLI entry point

## CLI Usage

```
RustX - A minimal scripting language

Usage:
  rustx-lang <file.rsx>     Run a RustX script file directly
  rustx-lang -o <file.rsx>  Compile to binary executable
  rustx-lang -s <file.rsx>  Convert to Rust (.rs) file
  rustx-lang -r <file.rs>   Convert Rust (.rs) to RSX (.rsx) file
  rustx-lang --help         Show this help message

Examples:
  rustx-lang demo/main.rsx       # Run the script
  rustx-lang -o demo/main.rsx    # Create binary executable
  rustx-lang -s demo/main.rsx    # Convert to main.rs
  rustx-lang -r main.rs          # Convert to main.rsx
```

## Contributing

The modular architecture makes it easy to extend:
- Add new tokens in `lexer.rs`
- Define new AST nodes in `ast.rs`
- Implement new language features in `parser.rs` and `interpreter.rs`
- Add built-in functions in `builtins.rs`
- Add transpiler rules in `transpiler.rs`

## License

MIT License