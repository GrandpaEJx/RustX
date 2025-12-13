#!/bin/bash

# RustX Benchmark Runner with Auto-Update

echo "======================================"
echo "RustX Performance Benchmarks"
echo "======================================"
echo ""

# Check if rustx_lang is available
if ! command -v rustx_lang &> /dev/null; then
    echo "Error: rustx_lang not found in PATH"
    echo "Please install with: cargo install rustx-lang"
    echo "Or run from project root with: cargo build --release"
    exit 1
fi

# Get the directory where this script is located
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
README_FILE="$SCRIPT_DIR/README.md"
RESULTS_FILE="$SCRIPT_DIR/.benchmark_results.tmp"

# Clear results file
> "$RESULTS_FILE"

echo "Running benchmarks..."
echo ""

# Function to run benchmark and capture time
run_benchmark() {
    local name="$1"
    local file="$2"
    
    echo "$name"
    echo "$(printf '%.0s-' {1..40})"
    
    # Run and capture time
    local start=$(date +%s%N)
    rustx_lang "$file"
    local end=$(date +%s%N)
    
    # Calculate time in milliseconds
    local duration=$(( (end - start) / 1000000 ))
    
    echo ""
    echo "Time: ${duration}ms"
    echo ""
    
    # Save result
    echo "$name|$duration" >> "$RESULTS_FILE"
}

# Run all benchmarks
run_benchmark "1. Fibonacci (Recursion)" "$SCRIPT_DIR/fibonacci.rsx"
run_benchmark "2. String Operations" "$SCRIPT_DIR/string_ops.rsx"
run_benchmark "3. Array Operations" "$SCRIPT_DIR/array_ops.rsx"
run_benchmark "4. Loop Performance" "$SCRIPT_DIR/loops.rsx"
run_benchmark "5. Method Chaining" "$SCRIPT_DIR/method_chaining.rsx"

echo "======================================"
echo "Updating README with results..."
echo "======================================"

# Read results
declare -A results
while IFS='|' read -r name time; do
    results["$name"]="$time"
done < "$RESULTS_FILE"

# Get current date
current_date=$(date "+%Y-%m-%d %H:%M:%S")

# Update README with results
cat > "$README_FILE" << 'EOF'
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

EOF

# Add results table
echo "**Last Updated:** $current_date" >> "$README_FILE"
echo "" >> "$README_FILE"
echo "| Benchmark | Time | Description |" >> "$README_FILE"
echo "|-----------|------|-------------|" >> "$README_FILE"
echo "| Fibonacci(20-30) | ${results["1. Fibonacci (Recursion)"]}ms | Recursive function calls |" >> "$README_FILE"
echo "| String Operations | ${results["2. String Operations"]}ms | 30K string manipulations |" >> "$README_FILE"
echo "| Array Operations | ${results["3. Array Operations"]}ms | 11K array operations |" >> "$README_FILE"
echo "| Loop Performance | ${results["4. Loop Performance"]}ms | 100K+ loop iterations |" >> "$README_FILE"
echo "| Method Chaining | ${results["5. Method Chaining"]}ms | 30K method chain calls |" >> "$README_FILE"

# Add interpretation section
cat >> "$README_FILE" << 'EOF'

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
EOF

# Clean up
rm "$RESULTS_FILE"

echo ""
echo "✅ README updated with benchmark results!"
echo "📊 Check benchmarks/README.md for details"
echo ""
