# RustX

> A clean, minimal scripting language that seamlessly integrates with Rust.

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

## Quick Start

### Install from Crates.io

```bash
cargo install rustx-lang
```

### Or Build from Source

```bash
git clone https://github.com/GrandpaEJx/RustX.git
cd RustX
cargo build --release
```

### Run a Script

```bash
# If installed via cargo install
rustx_lang examples/basic.rsx

# If built from source
cargo run --bin rustx_lang -- examples/basic.rsx
```

### Compile to Binary <0.3.0>

```bash
# Compile a script to a standalone executable
rustx_lang build examples/basic.rsx --output my_app

# Run the compiled binary
./my_app
```

### Start REPL

```bash
# If installed via cargo install
rustx_lang repl

# If built from source
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
- 🌐 **Web Framework** - Build high-performance web servers (67k+ RPS)
- 🔌 **Standard Library** - web, json, time, http, os modules
- 🎯 **Template Strings** - Backtick strings with `{var}` interpolation
- 🛠️ **Compiler** - Transpiles to Rust for native performance
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

### Method Chaining

```rustx
// String methods
"hello world".upper()              // HELLO WORLD
"  trim me  ".trim().lower()       // trim me

// Array methods
[1, 2, 3].len()                    // 3

// Math methods
3.14.round()                       // 3
(-42).abs()                        // 42
```

### Functions

```rustx
fn greet(name) {
    `Hello, {name}!`
}

fn add(a, b) => a + b  // Arrow function
```

### Control Flow

````rustx
// If expression
result = if age >= 18 { "Adult" } else { "Minor" }

// Loops
for i in range(5) {
    print(i)
}

while x < 10 {
    x = x + 1
}

### Web Server with Standard Library <0.4.0>

Build high-performance web servers with built-in modules:

```rustx
rust {
    // Force JIT compilation
}

import web
import json

let app = web.app()

fn home(body, debug) {
    return web.json({
        "name": "RustX API",
        "version": "1.0.0",
        "status": "running"
    })
}

fn add(body, debug) {
    let data = json.parse(body)
    let result = data["a"] + data["b"]
    return web.json({"sum": result})
}

app.get("/", home)
app.post("/add", add)
app.listen(8080, false, 4)
```

**Performance:** 67k RPS (100 connections), 57k RPS (1000 connections)

### Crate Imports & Embedded Rust <0.3.0>

You can import Rust crates and write raw Rust code:

```rustx
use crate "rand" = "0.8"

rust {
    fn get_random() -> Result<Value, String> {
        let n: i64 = rand::random::<u8>() as i64;
        Ok(Value::Int(n))
    }
}

print("Random:", get_random())
````

## Built-in Functions & Standard Library

**Core:** `print`, `range`, `len`, `type`, `push`, `pop`  
**String:** `split`, `join`, `trim`, `upper`, `lower`  
**Math:** `abs`, `min`, `max`, `floor`, `ceil`, `round`

**Standard Library Modules:**

- **`web`** - Build web servers and APIs
- **`json`** - Parse and serialize JSON
- **`time`** - Timestamps and delays
- **`http`** - HTTP client requests
- **`os`** - Environment and CLI args

[See complete API reference →](docs/built-in-functions.md)

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
- `method_chaining.rsx` - Method chaining with dot operator
- `string_math.rsx` - String and math functions
- `rust_imports.rsx` - Importing crates and embedding Rust
- `web_server.rsx` - Running an Actix-web server

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

## FAQ

**Q: Is RustX faster than Python?**  
A: The interpreter is generally slower than CPython, but the **compiler** (which compiles to native Rust) can be significantly faster, especially for loop-heavy code.

**Q: Can I use crates.io libraries?**  
A: **Yes!** As of v0.3.0, you can use `use crate "name" = "version"` to import crates directly into your scripts. RustX detects this and JIT-compiles your script to a native Rust binary.

**Q: Does RustX support classes/structs?**  
A: Not yet. We support Maps and Functions for data structure and logic.

## Contributing

Contributions welcome! Please read our [contributing guidelines](CONTRIBUTING.md).

## License

MIT License - see [LICENSE](LICENSE) file for details.

---

**Made with ❤️ in Rust**
