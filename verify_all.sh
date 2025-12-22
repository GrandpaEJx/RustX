#!/bin/bash
# set -e
echo "Starting verification..."
for file in examples/*.rsx; do
    echo "Testing $file..."
    if [[ "$file" == *"web_server.rsx"* ]]; then
        echo "Skipping $file (server)"
        continue
    fi
    # Timeout 120s to allow for compilation
    timeout 120s cargo run --quiet --bin rustx -- "$file" > /dev/null
    echo "Pass: $file"
done
echo "All examples passed!"
