#!/bin/bash
set -e
echo "Starting verification..."
for file in examples/*.rsx; do
    echo "Testing $file..."
    # Timeout 5s to avoid infinite loops, ignoring output
    timeout 5s cargo run --quiet --bin rustx_lang -- "$file" > /dev/null
    echo "Pass: $file"
done
echo "All examples passed!"
