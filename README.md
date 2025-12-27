# RustX

> A clean, minimal scripting language that seamlessly integrates with Rust.

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

## 📚 Documentation

- 🚀 **[Quick Start Guide](GUIDE.md)** - Learn RustX basics
- 📖 **[Quick Reference](QUICKREF.md)** - Syntax cheat sheet
- 💻 **[Installation](INSTALL.md)** - Setup instructions
- 🤝 **[Contributing](CONTRIBUTING.md)** - How to contribute
- 📝 **[Changelog](CHANGELOG.md)** - Version history

## Quick Start

### Option 1: Download Pre-built Binary (No Rust Required!)

```bash
# Linux/macOS - One-line installer
curl -sSL https://raw.githubusercontent.com/GrandpaEJx/RustX/main/install.sh | bash

# Or download manually from GitHub Releases
# https://github.com/GrandpaEJx/RustX/releases/latest
```

See [INSTALL.md](INSTALL.md) for detailed installation instructions.

### Option 2: Install via Cargo (Requires Rust)

```bash
cargo install --git https://github.com/GrandpaEJx/RustX rustx
```

### Run a Script

```bash
rustx examples/basic.rsx
```

### Compile to Binary (Requires Rust)

```bash
# Compile a script to a standalone executable
rustx build examples/basic.rsx --output my_app

# Run the compiled binary
./my_app
```

### Start REPL

```bash
rustx repl
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
- 🔌 **Standard Library** - web, json, time, http, os, fs, term modules
- 🎯 **Template Strings** - Backtick strings with `{var}` interpolation
- 🛠️ **Compiler** - Transpiles to Rust for native performance
- 🔄 **REPL** - Interactive shell with history
- ⚡ **Fast** - Tree-walk interpreter written in Rust
- 🎁 **Standalone** - No Rust required for interpreter mode

## Interpreter vs JIT

RustX runs in **two modes**:

| Mode | Speed | Requires Rust | Use Case |
|------|-------|---------------|----------|
| **Interpreter** | Fast startup | ❌ No | Most scripts, prototyping |
| **JIT Compiler** | ~6x faster than Node.js | ✅ Yes | Performance-critical code |

**Interpreter mode is the default** - just run `rustx script.rsx` and it works immediately!

JIT compilation is only needed for:
- `rust {}` blocks (embedded Rust code)
- `rust_import` statements (native dependencies)
- Maximum performance (see [benchmarks](benchmarks/LANG_COMPARISON.md))

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

### Module Imports (v0.5.0+)

**New!** Explicit imports for better performance:

```rustx
// Import only what you need
use json
use os

// Now use the modules
data = json.parse(`{"name": "Alice"}`)
path = os.env("HOME")
```

**Benefits:**
- 🚀 Faster startup (no overhead from unused modules)
- 📦 Cleaner transpiled code (9 lines vs 22 for hello world!)
- 📝 Explicit dependencies (like Python/JavaScript)

**Available modules:** `json`, `http`, `os`, `time`, `web`, `fs`, `term`

### Web Server Example

Build high-performance web servers with built-in modules:

```rustx
use web

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
