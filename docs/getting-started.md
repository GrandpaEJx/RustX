# Getting Started with RustX

## Installation

### Prerequisites

- Rust 1.70 or higher
- Cargo (comes with Rust)

### Install from Crates.io (Recommended)

The easiest way to install RustX:

```bash
cargo install rustx-lang
```

This will install the `rustx_lang` binary to your Cargo bin directory (usually `~/.cargo/bin`).

### Build from Source

If you want the latest development version:

```bash
git clone https://github.com/GrandpaEJx/RustX.git
cd RustX
cargo build --release
```

The compiled binary will be at `target/release/rustx_lang`.

### Verify Installation

```bash
rustx_lang --version
```

## Your First RustX Script

Create a file `hello.rsx`:

```rustx
name = "World"
print(`Hello, {name}!`)
```

Run it:

```bash
# If installed via cargo install
rustx_lang hello.rsx

# If built from source
cargo run --bin rustx_lang -- hello.rsx
```

Output:

```
Hello, World!
```

## Interactive REPL

Start the REPL (Read-Eval-Print Loop):

```bash
cargo run --bin rustx_lang repl
```

Try some commands:

```rustx
>>> x = 10
>>> y = 20
>>> x + y
30
>>> name = "Alice"
>>> `Hello, {name}!`
Hello, Alice!
>>> :exit
```

### REPL Commands

- `:help` - Show help message
- `:clear` - Clear screen
- `:vars` - Show all variables
- `:exit` or `Ctrl+D` - Exit REPL
- `Ctrl+C` - Cancel current input
- `Up/Down arrows` - Navigate command history

## CLI Options

### Run a Script

```bash
cargo run --bin rustx_lang -- script.rsx
```

### Debug Flags

**View AST (Abstract Syntax Tree):**

```bash
cargo run --bin rustx_lang -- --ast script.rsx
```

**View Tokens:**

```bash
cargo run --bin rustx_lang -- --tokens script.rsx
```

**Measure Execution Time:**

```bash
cargo run --bin rustx_lang -- --time script.rsx
```

**Verbose Output:**

```bash
cargo run --bin rustx_lang -- --verbose script.rsx
```

**Combine Flags:**

```bash
cargo run --bin rustx_lang -- --ast --time --verbose script.rsx
```

## Compiling to Native Binary

You can compile your RustX scripts into standalone, native executables using the `build` command.

```bash
# Basic compilation
cargo run --bin rustx_lang build script.rsx

# Specify output filename
cargo run --bin rustx_lang build script.rsx --output my_app
```

This transpiles your RustX code to Rust and compiles it with `cargo build --release`.
**Note:** You must have the `rustx_core` crate available for this to work.

## Next Steps

- [Language Reference](language-reference.md) - Learn the syntax
- [Built-in Functions](built-in-functions.md) - Explore available functions
- [Examples Guide](examples-guide.md) - See practical examples
- [Rust Integration](rust-integration.md) - Use RustX in Rust code

---

**Navigation:** [📚 Docs Home](README.md) | [⬅️ Main README](../README.md) | [➡️ Language Reference](language-reference.md)
