
fn factorial(n) {
    if n <= 1 {
        return 1
    }
    return n * factorial(n - 1)
}

print("Factorial of 5 is:")
print(factorial(5))

fn add(a, b) {
    a + b
}

result = add(10, 32)
print(`10 + 32 = {result}`)
