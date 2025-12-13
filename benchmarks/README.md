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

## Web Server Performance (v0.3.0)

**Last Updated:** 2025-12-13 21:13:00

Benchmarks run on 8-core CPU using `wrk` (100 connections, 10s).

| Mode         | Requests/Sec | Optimization            |
| ------------ | ------------ | ----------------------- |
| **JIT**      | **158,547**  | `--release` + LTO       |
| **Compiler** | **150,932**  | `codegen-units=1` + LTO |

> Both modes now utilize `release` profile and LTO, achieving native-level performance.

## Latest Interpreter Results

**Last Updated:** 2025-12-13 08:50:39

| Benchmark         | Time    | Description              |
| ----------------- | ------- | ------------------------ |
| Fibonacci(20-30)  | 12391ms | Recursive function calls |
| String Operations | 62ms    | 30K string manipulations |
| Array Operations  | 93ms    | 11K array operations     |
| Loop Performance  | 208ms   | 100K+ loop iterations    |
| Method Chaining   | 74ms    | 30K method chain calls   |

## Interpreting Results

RustX supports both interpreted execution and JIT/AOT compilation.

- **Interpreter**: Good for scripting, similar to Lua/Python.
- **JIT/Compiler**: Excellent for high-performance apps (web servers), comparable to Go/Rust.

## System Information

- RustX Version: 0.3.0
- Optimization: Release + LTO enabled by default for JIT/Compiler modes.

Run `./benchmarks/run_benchmarks.sh` for interpreter benchmarks.
Run `./benchmarks/compare_modes.sh` for JIT vs Compiler web server benchmarks.
