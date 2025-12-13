# RustX

> A clean, minimal scripting language that seamlessly integrates with Rust.

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

## Quick Start

```bash
# Install
git clone https://github.com/GrandpaEJx/RustX.git
cd RustX
cargo build --release

# Run a script
cargo run --bin rustx_lang -- examples/basic.rsx

# Start REPL
cargo run --bin rustx_lang repl
```

## Hello World

```rustx
name = "World"
print(`Hello, {name}!`)
```

## Key Features

- 🚀 **Simple Syntax** - Clean, Python-like syntax
- 🔗 **Rust Integration** - Use RustX in Rust via macros
- 📦 **Rich Built-ins** - 15+ built-in functions
- 🎯 **Template Strings** - Backtick strings with `{var}` interpolation
- 🔄 **REPL** - Interactive shell with history
- ⚡ **Fast** - Tree-walk interpreter written in Rust

## Language Basics

### Variables & Types

```rustx
x = 42              // Integer
pi = 3.14           // Float
name = "Alice"      // String
active = true       // Boolean
items = [1, 2, 3]   // Array
```

### Template Strings

```rustx
name = "Bob"
age = 25
print(`My name is {name} and I'm {age} years old`)
```

### Functions

```rustx
fn greet(name) {
    `Hello, {name}!`
}

fn add(a, b) => a + b  // Arrow function
```

### Control Flow

```rustx
// If expression
result = if age >= 18 { "Adult" } else { "Minor" }

// Loops
for i in range(5) {
    print(i)
}

while x < 10 {
    x = x + 1
}
```

## Built-in Functions

**Core:** `print`, `range`, `len`, `type`, `push`, `pop`  
**String:** `split`, `join`, `trim`, `upper`, `lower`  
**Math:** `abs`, `min`, `max`, `floor`, `ceil`, `round`

[See all built-in functions →](docs/built-in-functions.md)

## Rust Integration

```rust
use rustx_macros::rx;

fn main() {
    let result: i64 = rx! { "10 + 20 * 2" };
    println!("Result: {}", result);  // Result: 50
}
```

## Documentation

- [Getting Started](docs/getting-started.md)
- [Language Reference](docs/language-reference.md)
- [Built-in Functions](docs/built-in-functions.md)
- [Rust Integration](docs/rust-integration.md)
- [Examples](docs/examples-guide.md)

## Examples

Check out the `examples/` directory:

- `basic.rsx` - Variables, functions, arrays
- `loops.rsx` - For and while loops
- `recursion.rsx` - Recursive functions
- `template_strings.rsx` - Template string interpolation
- `string_math.rsx` - String and math functions

## Project Structure

```
RustX/
├── crates/
│   ├── core/      # Language core (lexer, parser, interpreter)
│   ├── macros/    # rx! and rsx! macros
│   └── cli/       # Command-line interface
├── examples/      # Example scripts
└── docs/          # Documentation
```

## Contributing

Contributions welcome! Please read our [contributing guidelines](CONTRIBUTING.md).

## License

MIT License - see [LICENSE](LICENSE) file for details.

---

**Made with ❤️ in Rust**
