#!/bin/bash
set -e

# Configuration
CLI_PATH="./target/release/rustx_lang"
SERVER_SCRIPT="examples/web_server.rsx"
COMPILED_BIN="benchmarks/server_bin"
REPORT_FILE="benchmarks/comparison_results.md"

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}Building RustX CLI (release)...${NC}"
cargo build --release

# Helper function for benchmarking
run_benchmark() {
    local mode=$1
    echo -e "${GREEN}Benchmarking $mode Mode...${NC}"
    
    # Wait for server to start (poll for port 8080)
    # Increased timeout for Release builds (can take > 3 mins)
    echo "Waiting for server to listen on port 8080..."
    for i in {1..180}; do # 180 * 2 = 360 seconds
        if nc -z localhost 8080; then
            echo "Server is up!"
            break
        fi
        sleep 2
    done
    
    if ! nc -z localhost 8080; then
        echo "Error: Server failed to start."
        return 1
    fi
    
    echo "Running wrk (GET /)..."
    RESULT=$(wrk -c100 -d5s -t4 http://localhost:8080/)
    echo "$RESULT"
    
    # Extract requests/sec
    RPS=$(echo "$RESULT" | grep "Requests/sec" | awk '{print $2}')
    echo -e "${BLUE}$mode RPS: $RPS${NC}"
    
    # Append to report
    echo "| $mode | $RPS |" >> "$REPORT_FILE"
}

# Clean previous report
echo "# JIT vs Compiler Benchmark Results" > "$REPORT_FILE"
echo "" >> "$REPORT_FILE"
echo "| Mode | Requests/Sec |" >> "$REPORT_FILE"
echo "|------|--------------|" >> "$REPORT_FILE"

# --- 1. Compiler Mode ---
echo -e "${BLUE}Compiling Web Server...${NC}"
$CLI_PATH build "$SERVER_SCRIPT" -o "$COMPILED_BIN"

echo "Starting Compiled Server..."
# Kill any existing
pkill -f "server_bin" || true
pkill -f "web_server.rsx" || true
fuser -k 8080/tcp || true

./"$COMPILED_BIN" > /dev/null 2>&1 &
PID=$!
run_benchmark "Compiler"
kill $PID || true
wait $PID 2>/dev/null || true

# --- 2. JIT Mode ---
echo -e "${BLUE}Running JIT Server...${NC}"
# Kill any existing
pkill -f "server_bin" || true
pkill -f "web_server.rsx" || true
fuser -k 8080/tcp || true

# JIT Mode takes longer to compile internally
echo "Starting JIT Server (this may take time)..."
$CLI_PATH "$SERVER_SCRIPT" > /dev/null 2>&1 &
PID=$!
run_benchmark "JIT"
kill $PID || true
wait $PID 2>/dev/null || true

# Display Results
echo ""
echo -e "${GREEN}Benchmark Complete!${NC}"
cat "$REPORT_FILE"
