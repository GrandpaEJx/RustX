// Fibonacci function (recursive)
fn fib(n) {
    if n <= 1 {
        n
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

// Factorial function (recursive)
fn factorial(n) {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

// Test factorial
result = factorial(5)

// Test fibonacci (small number to avoid slowness)
fib_result = fib(10)

result
