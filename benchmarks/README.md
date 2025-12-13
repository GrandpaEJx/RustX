# RustX Benchmarks

Performance benchmarks for the RustX language interpreter.

## Running Benchmarks

### Quick Benchmark

```bash
./benchmarks/run_benchmarks.sh
```

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
```

## Benchmark Scripts

- `fibonacci.rsx` - Recursive function performance
- `string_ops.rsx` - String manipulation and method chaining
- `array_ops.rsx` - Array operations and built-in functions
- `loops.rsx` - Loop iteration performance
- `method_chaining.rsx` - Method chaining overhead

## Results

Results will vary based on your system. Typical results on a modern CPU:

| Benchmark     | Operations      | Time  | Ops/sec   |
| ------------- | --------------- | ----- | --------- |
| Fibonacci(30) | Recursive calls | ~50ms | ~20K/sec  |
| String ops    | 10K operations  | ~30ms | ~333K/sec |
| Array ops     | 10K operations  | ~25ms | ~400K/sec |
| Loops         | 100K iterations | ~40ms | ~2.5M/sec |

## Interpreting Results

RustX is a tree-walking interpreter, so performance is expected to be:

- **Faster than:** Pure Python, Ruby
- **Slower than:** Compiled languages (Rust, C, Go)
- **Similar to:** Lua, JavaScript (non-JIT)

The focus is on **simplicity** and **ease of integration** rather than raw speed.
