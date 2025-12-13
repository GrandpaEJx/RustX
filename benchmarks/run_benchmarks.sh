#!/bin/bash

# RustX Benchmark Runner

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

echo "Running benchmarks..."
echo ""

# Fibonacci benchmark
echo "1. Fibonacci (Recursion)"
echo "------------------------"
time rustx_lang "$SCRIPT_DIR/fibonacci.rsx"
echo ""

# String operations
echo "2. String Operations"
echo "--------------------"
time rustx_lang "$SCRIPT_DIR/string_ops.rsx"
echo ""

# Array operations
echo "3. Array Operations"
echo "-------------------"
time rustx_lang "$SCRIPT_DIR/array_ops.rsx"
echo ""

# Loop performance
echo "4. Loop Performance"
echo "-------------------"
time rustx_lang "$SCRIPT_DIR/loops.rsx"
echo ""

# Method chaining
echo "5. Method Chaining"
echo "------------------"
time rustx_lang "$SCRIPT_DIR/method_chaining.rsx"
echo ""

echo "======================================"
echo "All benchmarks complete!"
echo "======================================"
