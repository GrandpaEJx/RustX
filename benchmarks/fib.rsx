fn fib(n) {
    if n < 2 { return n }
    return fib(n-1) + fib(n-2)
}

print("Running RustX Fib(30)...")
start = time.now()
res = fib(30)
end = time.now()
print("Result:", res)
print("Time:", end - start)
