// Fibonacci function
fn fib(n) {
    if n <= 1 {
        n
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

// Calculate first 10 Fibonacci numbers
results = []
i = 0
while i < 10 {
    results = results  // Note: array append not yet implemented
    i = i + 1
}

// Factorial function
fn factorial(n) {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

factorial(5)
