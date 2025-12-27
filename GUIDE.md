# RustX Learning Guide

Welcome to RustX! This guide will help you learn RustX from basics to advanced features.

## Table of Contents

1. [Getting Started](#getting-started)
2. [Basic Syntax](#basic-syntax)
3. [Data Types](#data-types)
4. [Control Flow](#control-flow)
5. [Functions](#functions)
6. [Standard Library](#standard-library)
7. [Advanced Features](#advanced-features)
8. [Best Practices](#best-practices)

---

## Getting Started

### Installation

```bash
# Quick install (pre-built binary)
curl -sSL https://raw.githubusercontent.com/GrandpaEJx/RustX/main/install.sh | bash

# Or build from source
cargo install --path .
```

### Your First Program

```rust
// hello.rsx
print("Hello, World!")
```

Run it:
```bash
rustx hello.rsx
```

---

## Basic Syntax

### Variables

```rust
// Variables are dynamically typed
x = 42
name = "Alice"
pi = 3.14
is_active = true

// Reassignment is allowed
x = 100
x = "now a string"  // Type can change!
```

### Module Imports (v0.5.0+)

**Important:** For better performance, import only what you need!

```rust
// Import standard library modules
use json
use os
use http

// Now use them
data = json.parse(`{"name": "Alice"}`)
path = os.env("PATH")
response = http.get("https://api.github.com")
```

**Available modules:** `json`, `http`, `os`, `time`, `web`, `fs`, `term`

---

## Quick Examples

### JSON Parsing

```rust
use json

json_str = `{"name": "Alice", "age": 25}`
obj = json.parse(json_str)
print(obj.name)  // "Alice"
```

### HTTP Request

```rust
use http

response = http.get("https://api.github.com")
print(response)
```

### File Operations

```rust
use fs

content = fs.read("data.txt")
fs.write("output.txt", "Hello!")
```

### Web Server

```rust
use web

app = web.app()

app.get("/", fn(req) {
    return web.json({message: "Hello, API!"})
})

app.listen(3000)
```

---

## CLI Commands

```bash
# Run (interpreter - fast startup)
rustx script.rsx

# Build to binary (6x faster execution)
rustx script.rsx -o myapp

# Transpile to Rust source
rustx script.rsx --rs output.rs

# Check syntax
rustx check script.rsx

# Interactive REPL
rustx repl
```

---

## Performance

RustX is **fast**!

| Benchmark | Python | Node.js | RustX |
|-----------|--------|---------|-------|
| Fibonacci(30) | 170ms | 73ms | **11.77ms** |
| Loop (1M) | 240ms | 45ms | **13.59ms** |

**6x faster than Node.js** in JIT mode! 🚀

See [benchmarks/LANG_COMPARISON.md](benchmarks/LANG_COMPARISON.md) for details.

---

## More Resources

- 📖 **Full Guide**: See examples in [examples/](examples/) folder
- 🚀 **Installation**: [INSTALL.md](INSTALL.md)
- 📝 **Changelog**: [CHANGELOG.md](CHANGELOG.md)
- 🐛 **Issues**: [GitHub Issues](https://github.com/GrandpaEJx/RustX/issues)

---

## Example Scripts

- **Hello World**: [examples/hello_world.rsx](examples/hello_world.rsx)
- **Arrays**: [examples/array_functions.rsx](examples/array_functions.rsx)
- **Loops**: [examples/loops.rsx](examples/loops.rsx)
- **Web Server**: [examples/web_server.rsx](examples/web_server.rsx)
- **HTTP Client**: [examples/stdlib_test.rsx](examples/stdlib_test.rsx)
- **File System**: [examples/fs_test.rsx](examples/fs_test.rsx)

Happy coding! 🎉
