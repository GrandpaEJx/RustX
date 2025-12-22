# Language Comparison Benchmarks

This document compares RustX JIT performance against popular interpreted/JIT languages (Python 3, Node.js).

## Results

<!-- RESULTS_START -->
| Language | Benchmark | Time | CPU % | RAM (Max) |
| :--- | :--- | :--- | :--- | :--- |
| RustX JIT | Fib(30) | 11.77ms | 100% | 6.46 MB |
| Python3 | Fib(30) | 170.12ms | 98% | 9.37 MB |
| Node.js | Fib(30) | 73.36ms | 105% | 48.97 MB |
| RustX JIT | Loop(1M) | 13.59ms | 90% | 6.46 MB |
| Python3 | Loop(1M) | 240.32ms | 96% | 9.53 MB |
| Node.js | Loop(1M) | 45.65ms | 111% | 51.13 MB |

*Last updated: Mon Dec 22 22:26:45 +06 2025*
<!-- RESULTS_END -->

## Environment
- **CPU**: (Auto-detected)
- **OS**: Linux
- **RustX**: JIT Enabled (Release Build)

*Note: RustX JIT build times are excluded from "Hot" runs if cached.*
