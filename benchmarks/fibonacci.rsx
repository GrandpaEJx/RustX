// Fibonacci benchmark - tests recursion performance

fn fib(n) {
    if n <= 1 {
        n
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

print("Computing Fibonacci numbers...")

// Warm up
fib(10)

// Benchmark
start = 20
end = 30

for i in range(start, end + 1) {
    result = fib(i)
    print(`fib({i}) = {result}`)
}

print("Benchmark complete!")
