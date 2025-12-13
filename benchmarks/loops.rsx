// Loop performance benchmark

print("Loop performance benchmark...")

// While loop
iterations = 100000
counter = 0
total = 0

while counter < iterations {
    total = total + counter
    counter = counter + 1
}

print(`While loop: {iterations} iterations, sum = {total}`)

// For loop with range
total = 0
for i in range(iterations) {
    total = total + i
}

print(`For loop: {iterations} iterations, sum = {total}`)

// Nested loops
outer = 0
total = 0
while outer < 1000 {
    inner = 0
    while inner < 100 {
        total = total + 1
        inner = inner + 1
    }
    outer = outer + 1
}

print(`Nested loops: 1000 x 100 iterations, total = {total}`)

print("Loop benchmark complete!")
