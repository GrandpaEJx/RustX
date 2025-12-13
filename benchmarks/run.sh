#!/bin/bash
set -e

# Build the CLI in release mode first for maximum performance
echo "Building RustX CLI (release)..."
cargo build --release

CLI_PATH="./target/release/rustx_lang"
SERVER_SCRIPT="examples/web_server.rsx"

# Build the server script into a standalone binary
echo "Compiling Web Server (Release Mode)..."
$CLI_PATH build $SERVER_SCRIPT -o benchmarks/server_bin

echo "Starting Web Server..."
# Kill any existing server
pkill -f "server_bin" || true

# Run the compiled binary
./benchmarks/server_bin &
SERVER_PID=$!

echo "Waiting for server to start..."
sleep 5

echo "Running Benchmarks..."
echo "------------------------------------------------"
echo "Endpoint: GET /"
wrk -c100 -d10s -t4 http://localhost:8080/

echo "------------------------------------------------"
echo "Endpoint: POST /echo"
wrk -c100 -d10s -t4 -s benchmarks/post.lua http://localhost:8080/echo

echo "------------------------------------------------"
echo "Endpoint: POST /add (JSON Logic)"
wrk -c100 -d10s -t4 -s benchmarks/json.lua http://localhost:8080/add

# Cleanup
kill $SERVER_PID
echo "Benchmark Complete."
