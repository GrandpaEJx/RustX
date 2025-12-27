# Contributing to RustX

Thank you for your interest in contributing to RustX! 🎉

## Getting Started

### Prerequisites

- Rust 1.70 or later
- Git

### Setting Up Development Environment

```bash
# Clone the repository
git clone https://github.com/GrandpaEJx/RustX.git
cd RustX

# Build the project
cargo build

# Run tests
cargo test

# Run examples
cargo run -- examples/basic.rsx
```

## Project Structure

```
RustX/
├── crates/
│   ├── core/          # Language core (lexer, parser, interpreter, transpiler)
│   ├── cli/           # Command-line interface
│   └── macros/        # Rust macro support
├── examples/          # Example .rsx scripts
├── tests/             # Test scripts
├── benchmarks/        # Performance benchmarks
└── docs/              # Documentation
```

## Development Workflow

### 1. Create a Branch

```bash
git checkout -b feature/your-feature-name
```

### 2. Make Changes

- Write clean, readable code
- Follow the existing code style
- Add tests for new features
- Update documentation as needed

### 3. Test Your Changes

```bash
# Run all tests
cargo test

# Test specific feature
cargo run -- examples/your_test.rsx

# Check formatting
cargo fmt --check

# Run clippy
cargo clippy
```

### 4. Commit

```bash
git add .
git commit -m "Brief description of changes"
```

**Commit message format:**
- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation changes
- `refactor:` Code refactoring
- `test:` Adding tests
- `chore:` Maintenance tasks

### 5. Push and Create PR

```bash
git push origin feature/your-feature-name
```

Then create a Pull Request on GitHub.

## Code Style Guidelines

### Rust Code

- Follow standard Rust conventions
- Use `cargo fmt` for formatting
- Address `cargo clippy` warnings
- Add comments for complex logic
- Keep functions small and focused

### RustX Scripts

- Use clear, descriptive variable names
- Add comments for non-obvious code
- Follow examples in `examples/` folder

## Adding New Features

### Adding a Stdlib Module

1. Create module in `crates/core/src/stdlib/your_module.rs`
2. Add to `crates/core/src/stdlib/mod.rs`
3. Update interpreter in `crates/core/src/interpreter/mod.rs`:
   - Add to `load_stdlib_module()` match statement
4. Update transpiler in `crates/core/src/compiler/transpiler.rs`:
   - Add to `generate_module_init()` match statement
5. Add examples in `examples/`
6. Update `GUIDE.md` documentation

### Adding Language Features

1. Update lexer if new tokens needed (`crates/core/src/lexer/`)
2. Update parser (`crates/core/src/parser/`)
3. Update AST (`crates/core/src/ast/`)
4. Implement in interpreter (`crates/core/src/interpreter/`)
5. Implement in transpiler (`crates/core/src/compiler/`)
6. Add tests
7. Update documentation

## Testing

### Running Tests

```bash
# All tests
cargo test

# Specific crate
cargo test -p rustx_core

# With output
cargo test -- --nocapture
```

### Adding Tests

Create test files in `tests/` directory:

```rust
// tests/test_feature.rsx
// Test your feature here
x = 42
print(x)
```

## Documentation

### Updating Documentation

When adding features, update:

1. `README.md` - Main documentation
2. `GUIDE.md` - Learning guide
3. `CHANGELOG.md` - Version history
4. Code comments - For complex logic
5. Examples - Add to `examples/`

### Documentation Style

- Use clear, simple language
- Provide code examples
- Explain the "why", not just the "what"
- Keep it concise

## Performance Considerations

RustX values **simplicity over premature optimization**.

When adding features:
- ✅ Keep code clean and readable
- ✅ Add optimizations only when needed
- ✅ Benchmark if performance-critical
- ❌ Don't sacrifice maintainability for minor gains

Run benchmarks:
```bash
cd benchmarks
./run.sh
./compare.sh
```

## Reporting Issues

### Bug Reports

Include:
- RustX version (`rustx --version`)
- Operating system
- Minimal reproduction case
- Expected vs actual behavior

### Feature Requests

Describe:
- Use case
- Proposed syntax/API
- Why it's needed
- Alternatives considered

## Code of Conduct

- Be respectful and welcoming
- Focus on constructive feedback
- Help newcomers
- Keep discussions on-topic

## Questions?

- 💬 Open a GitHub Discussion
- 🐛 File an issue for bugs
- 📖 Check existing documentation

Thank you for contributing to RustX! 🚀
