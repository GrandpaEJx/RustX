#!/bin/bash
set -e

# Setup
README="benchmarks/LANG_COMPARISON.md"
TEMP_METRICS="benchmarks/metrics_comp.tmp"
RUSTX_BIN="target/release/rustx"
DATE=$(LC_ALL=C date)

# Formatting Function (Adaptive)
format_time() {
    NS=$1
    if [ "$NS" -lt 1000 ]; then
        echo "${NS}ns"
    elif [ "$NS" -lt 1000000 ]; then
        echo $(echo "scale=2; $NS / 1000" | bc | awk '{printf "%.2fus", $0}')
    elif [ "$NS" -lt 1000000000 ]; then
        echo $(echo "scale=2; $NS / 1000000" | bc | awk '{printf "%.2fms", $0}')
    else
        echo $(echo "scale=4; $NS / 1000000000" | bc | awk '{printf "%.4fs", $0}')
    fi
}

# Table Header
TABLE="| Language | Benchmark | Time | CPU % | RAM (Max) |\\n| :--- | :--- | :--- | :--- | :--- |\\n"

run_bench() {
    LANG="$1"
    TEST="$2"
    CMD="$3"
    
    echo "Running $LANG ($TEST)..."
    
    # 1. Capture Start Time (Nanoseconds)
    START_NS=$(mktime=1 date +%s%N 2>/dev/null || date +%s%N)
    
    # 2. Run command with time to capture RAM/CPU
    # -f "%P %M" -> CPU_PERCENT MAX_RSS_KB
    /usr/bin/time -f "%P %M" -o "$TEMP_METRICS" $CMD > /dev/null
    
    # 3. Capture End Time
    END_NS=$(mktime=1 date +%s%N 2>/dev/null || date +%s%N)
    
    # 4. Calculate Duration
    DURATION_NS=$((END_NS - START_NS))
    REAL_TIME=$(format_time $DURATION_NS)
    
    # 5. Parse RAM/CPU
    read CPU RAM_KB < "$TEMP_METRICS"
    
    # RAM bytes -> MB
    RAM_MB=$(echo "scale=2; $RAM_KB / 1024" | bc)
    if [ -z "$RAM_MB" ]; then RAM_MB="${RAM_KB} KB"; else RAM_MB="${RAM_MB} MB"; fi

    # Fix CPU % (sometimes comes as ?%)
    if [ "$CPU" == "?" ]; then CPU="N/A"; fi

    # Append row
    TABLE="${TABLE}| $LANG | $TEST | $REAL_TIME | $CPU | $RAM_MB |\\n"
}

echo "=== Language Comparison Benchmarks ==="

# --- FIBONACCI ---
run_bench "RustX JIT" "Fib(30)" "$RUSTX_BIN benchmarks/fib_jit.rsx"
run_bench "Python3" "Fib(30)" "python3 benchmarks/fib.py"
run_bench "Node.js" "Fib(30)" "node benchmarks/fib.js"

# --- LOOP ---
run_bench "RustX JIT" "Loop(1M)" "$RUSTX_BIN benchmarks/loop_jit.rsx"
run_bench "Python3" "Loop(1M)" "python3 benchmarks/loop.py"
run_bench "Node.js" "Loop(1M)" "node benchmarks/loop.js"

# Update README
TABLE="${TABLE}\\n*Last updated: $DATE*"
perl -i -0777 -pe "s/<!-- RESULTS_START -->.*<!-- RESULTS_END -->/<!-- RESULTS_START -->\\n$TABLE\\n<!-- RESULTS_END -->/s" "$README"

# Cleanup
rm -f "$TEMP_METRICS"

echo -e "\n✅ $README updated with detailed comparison metrics!"
