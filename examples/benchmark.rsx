
print("Running benchmark (Iterative)...")

fn fib_iter(n) {
    if n < 2 {
        return n
    }
    
    a = 0
    b = 1
    
    // Iterative approach to avoid recursion issues in current compiler version
    i = 2
    while i <= n {
        temp = (a + b) % 1000000
        a = b
        b = temp
        i = i + 1
    }
    return b
}

N = 50000 // Higher number for iterative
print(`Calculating Fibonacci of {N}...`)
result = fib_iter(N)
print("Calculation done.") 
// print(result) // Might be huge
