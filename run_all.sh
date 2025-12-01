#!/bin/bash

echo "Running all tests and demos..."

for file in test/*.rsx demo/*.rsx; do
    echo "----------------------------------------"
    echo "Compiling $file..."
    if cargo run --quiet -- "$file" > /dev/null; then
        echo "OK"
    else
        echo "FAILED"
    fi
done
