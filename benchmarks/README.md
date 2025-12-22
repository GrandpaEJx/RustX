# RustX Benchmarks

This directory contains performance comparisons between RustX (Interpreted) and optimized pure Rust.

## Results

<!-- RESULTS_START -->
| Benchmark | File | Time (Elap) | CPU % | RAM (Max) |
| :--- | :--- | :--- | :--- | :--- |
| Pure Rust | fib.rs | 5.86ms | 100% | 1.98 MB |
| RustX Interpreter | fib.rsx | 3.6009s | 99% | 6.66 MB |
| RustX JIT | fib_jit.rsx | 7.99ms | 66% | 6.59 MB |
| Pure Rust | loop.rs | 4.32ms | 86% | 2.04 MB |
| RustX Interpreter | loop.rsx | 536.94ms | 99% | 6.81 MB |
| RustX JIT | loop_jit.rsx | 54.73ms | 98% | 6.33 MB |

*Last updated: Mon Dec 22 22:08:09 +06 2025*
<!-- RESULTS_END -->

## Files

- `fib.rsx`: Recursive Fibonacci(30) in RustX.
- `fib.rs`: Recursive Fibonacci(30) in pure Rust.
- `loop.rsx`: Tight loop summation (1M iterations) in RustX.
- `loop.rs`: Tight loop summation (1M iterations) in pure Rust.
- `run.sh`: Script to compile, run, and update this README.
