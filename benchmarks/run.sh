#!/bin/bash
set -e

# Setup
README="benchmarks/README.md"
TEMP_METRICS="benchmarks/metrics.tmp"
DATE=$(LC_ALL=C date)

echo "=== Compiling Pure Rust Benchmarks ==="
rustc -O benchmarks/fib.rs -o benchmarks/fib
rustc -O benchmarks/loop.rs -o benchmarks/loop

echo "=== Compiling RustX Release ==="
cargo build --release --quiet --bin rustx

RUSTX_BIN="target/release/rustx"

run_bench() {
    NAME="$1"
    FILE="$2"
    CMD="$3"
    
    echo "Running $NAME ($FILE)..."
    
    # 1. Capture Start Time (Nanoseconds)
    START_NS=$(date +%s%N)
    
    # 2. Run command with time to capture RAM/CPU only
    # -f "%P %M" -> CPU_PERCENT MAX_RSS_KB
    /usr/bin/time -f "%P %M" -o "$TEMP_METRICS" $CMD > /dev/null
    
    # 3. Capture End Time
    END_NS=$(date +%s%N)
    
    # 4. Calculate Duration
    DURATION_NS=$((END_NS - START_NS))
    
    # 5. Format Time (Adaptive)
    if [ "$DURATION_NS" -lt 1000 ]; then
        REAL_TIME="${DURATION_NS}ns"
    elif [ "$DURATION_NS" -lt 1000000 ]; then
        # < 1ms, show us
        REAL_TIME=$(echo "scale=2; $DURATION_NS / 1000" | bc | awk '{printf "%.2fus", $0}')
    elif [ "$DURATION_NS" -lt 1000000000 ]; then
        # < 1s, show ms
        REAL_TIME=$(echo "scale=2; $DURATION_NS / 1000000" | bc | awk '{printf "%.2fms", $0}')
    else
        # >= 1s, show s
        REAL_TIME=$(echo "scale=4; $DURATION_NS / 1000000000" | bc | awk '{printf "%.4fs", $0}')
    fi
    
    # 5. Parse RAM/CPU
    read CPU RAM_KB < "$TEMP_METRICS"
    
    # RAM bytes -> MB
    RAM_MB=$(echo "scale=2; $RAM_KB / 1024" | bc)
    if [ -z "$RAM_MB" ]; then RAM_MB="${RAM_KB} KB"; else RAM_MB="${RAM_MB} MB"; fi

    # Fix CPU % (sometimes comes as ?%)
    if [ "$CPU" == "?" ]; then CPU="N/A"; fi

    # Append row
    TABLE="${TABLE}| $NAME | $FILE | $REAL_TIME | $CPU | $RAM_MB |\\n"
}

# Add Header
TABLE="| Benchmark | File | Time (Elap) | CPU % | RAM (Max) |\\n| :--- | :--- | :--- | :--- | :--- |\\n"

echo -e "\n=== Execution ==="

# --- FIBONACCI ---
run_bench "Pure Rust" "fib.rs" "./benchmarks/fib"
run_bench "RustX Interpreter" "fib.rsx" "$RUSTX_BIN benchmarks/fib.rsx"
echo "Compiling and running JIT Fib (this may take a few minutes)..."
run_bench "RustX JIT" "fib_jit.rsx" "$RUSTX_BIN benchmarks/fib_jit.rsx"

# --- LOOP ---
run_bench "Pure Rust" "loop.rs" "./benchmarks/loop"
run_bench "RustX Interpreter" "loop.rsx" "$RUSTX_BIN benchmarks/loop.rsx"
echo "Compiling and running JIT Loop (this may take a few minutes)..."
run_bench "RustX JIT" "loop_jit.rsx" "$RUSTX_BIN benchmarks/loop_jit.rsx"

# Update README template to contain placeholder if missing (or we just rewrite the table section)
# We assume <!-- RESULTS_START --> and <!-- RESULTS_END --> exist.

TABLE="${TABLE}\\n*Last updated: $DATE*"

perl -i -0777 -pe "s/<!-- RESULTS_START -->.*<!-- RESULTS_END -->/<!-- RESULTS_START -->\\n$TABLE\\n<!-- RESULTS_END -->/s" "$README"

# Cleanup
rm -f "$TEMP_METRICS"

echo -e "\n✅ benchmarks/README.md updated with detailed metrics!"
