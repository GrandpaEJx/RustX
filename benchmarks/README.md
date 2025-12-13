# RustX Benchmarks

Performance benchmarks for the RustX language interpreter.

## Running Benchmarks

### Quick Benchmark
```bash
./benchmarks/run_benchmarks.sh
```

This will automatically run all benchmarks and update this README with the results.

### Individual Benchmarks
```bash
# Fibonacci (recursion)
time rustx_lang benchmarks/fibonacci.rsx

# String operations
time rustx_lang benchmarks/string_ops.rsx

# Array operations
time rustx_lang benchmarks/array_ops.rsx

# Loop performance
time rustx_lang benchmarks/loops.rsx

# Method chaining
time rustx_lang benchmarks/method_chaining.rsx
```

## Benchmark Scripts

- `fibonacci.rsx` - Recursive function performance (fib 20-30)
- `string_ops.rsx` - String manipulation and method chaining (30K ops)
- `array_ops.rsx` - Array operations and built-in functions (11K ops)
- `loops.rsx` - Loop iteration performance (100K+ iterations)
- `method_chaining.rsx` - Method chaining overhead (30K ops)

## Latest Results

**Last Updated:** 2025-12-13 08:50:39

| Benchmark | Time | Description |
|-----------|------|-------------|
| Fibonacci(20-30) | 12391ms | Recursive function calls |
| String Operations | 62ms | 30K string manipulations |
| Array Operations | 93ms | 11K array operations |
| Loop Performance | 208ms | 100K+ loop iterations |
| Method Chaining | 74ms | 30K method chain calls |

## Interpreting Results

RustX is a tree-walking interpreter, so performance is expected to be:
- **Faster than:** Pure Python, Ruby
- **Slower than:** Compiled languages (Rust, C, Go)
- **Similar to:** Lua, JavaScript (non-JIT)

The focus is on **simplicity** and **ease of integration** rather than raw speed.

## Performance Tips

1. **Minimize function calls** - Function calls have overhead
2. **Use built-in functions** - They're implemented in Rust
3. **Avoid deep recursion** - Use loops when possible
4. **Cache results** - Store computed values in variables
5. **Method chaining is efficient** - No extra overhead vs separate calls

## System Information

Results will vary based on your system. These benchmarks were run on:
- CPU: Your system's CPU
- OS: Your operating system
- RustX Version: 0.2.0

Run `./benchmarks/run_benchmarks.sh` to get results for your system!
