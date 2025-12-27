# Installation Guide

RustX can be installed in multiple ways depending on your needs.

## Option 1: Pre-built Binary (Recommended for Users)

**No Rust installation required!** Download a pre-built binary for your platform.

### Quick Install (Linux/macOS)

```bash
curl -sSL https://raw.githubusercontent.com/GrandpaEJx/RustX/main/install.sh | bash
```

### Manual Download

1. Visit the [Releases page](https://github.com/GrandpaEJx/RustX/releases/latest)
2. Download the binary for your platform:
   - `rustx-linux-x86_64` - Linux (x86_64)
   - `rustx-linux-aarch64` - Linux (ARM64)
   - `rustx-macos-x86_64` - macOS (Intel)
   - `rustx-macos-aarch64` - macOS (Apple Silicon)
   - `rustx-windows-x86_64.exe` - Windows

3. Make it executable (Linux/macOS):
   ```bash
   chmod +x rustx-*
   mv rustx-* ~/.local/bin/rustx
   ```

4. Add to PATH (if needed):
   ```bash
   export PATH="$HOME/.local/bin:$PATH"
   ```

---

## Option 2: Build from Source (For Developers)

Requires Rust toolchain (1.70+).

### Install via Cargo

```bash
cargo install --git https://github.com/GrandpaEJx/RustX rustx
```

### Build Manually

```bash
git clone https://github.com/GrandpaEJx/RustX.git
cd RustX
cargo build --release
./target/release/rustx --help
```

---

## Verify Installation

```bash
rustx --version
```

You should see:
```
rustx 0.5.0
```

---

## Quick Start

### Run Your First Script

Create `hello.rsx`:
```rust
print("Hello, RustX!")
```

Run it:
```bash
rustx hello.rsx
```

### Interactive REPL

```bash
rustx repl
```

---

## Interpreter vs JIT Modes

RustX runs in **two modes**:

| Mode | Performance | Requires Rust | Use Case |
|------|-------------|---------------|----------|
| **Interpreter** | Fast startup | ❌ No | Scripts, prototyping, most use cases |
| **JIT Compiler** | Near-native speed | ✅ Yes | High-performance loops, benchmarks |

### Using Interpreter (Default)
```bash
rustx script.rsx  # Works immediately, no Rust needed
```

### Using JIT Compiler
Requires [Rust](https://rustup.rs) to be installed.

```bash
rustx build script.rsx  # Compiles to native binary
./script                 # Run the compiled output
```

---

## Troubleshooting

### "rustx: command not found"

Your installation directory is not in PATH. Add it:

```bash
# For bash
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

# For zsh
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

### "Error: JIT compilation requires Rust toolchain"

This script uses features that need compilation (`rust {}` blocks or `rust_import`).

**Solution 1:** Install Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**Solution 2:** Modify the script to avoid JIT features (remove `rust {}` blocks)

---

## Uninstall

```bash
rm ~/.local/bin/rustx
```

---

## Next Steps

- [Language Guide](docs/LANGUAGE.md)
- [Standard Library](docs/STDLIB.md)
- [Examples](examples/)
